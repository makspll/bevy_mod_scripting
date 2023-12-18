use gtrie::Trie;
use indexmap::{IndexMap, IndexSet};
use rustdoc_types::{
    Crate, Enum, GenericArg, GenericArgs, Id, Impl, Import, Item, ItemEnum, Module, Struct, Trait,
    Type, Visibility,
};
use serde_derive::Serialize;
use std::borrow::Cow::{Borrowed, Owned};
use std::collections::hash_map::RandomState;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::iter::{once, repeat};
use std::{borrow::Cow, ops::Deref};
use syn::ItemTrait;

use crate::{args, UniversalType};

pub fn print_item_variant(variant: &ItemEnum) -> &'static str {
    match variant {
        ItemEnum::Module(_) => "Module",
        ItemEnum::ExternCrate { .. } => "ExternCrate",
        ItemEnum::Import(_) => "Import",
        ItemEnum::Union(_) => "Union",
        ItemEnum::Struct(_) => "Struct",
        ItemEnum::StructField(_) => "StructField",
        ItemEnum::Enum(_) => "Enum",
        ItemEnum::Variant(_) => "Variant",
        ItemEnum::Function(_) => "Function",
        ItemEnum::Trait(_) => "Trait",
        ItemEnum::TraitAlias(_) => "TraitAlias",
        ItemEnum::Impl(_) => "Impl",
        ItemEnum::TypeAlias(_) => "TypeAlias",
        ItemEnum::OpaqueTy(_) => "OpaqueTy",
        ItemEnum::Constant(_) => "Constant",
        ItemEnum::Static(_) => "Static",
        ItemEnum::ForeignType => "ForeignType",
        ItemEnum::Macro(_) => "Macro",
        ItemEnum::ProcMacro(_) => "ProcMacro",
        ItemEnum::Primitive(_) => "Primitive",
        ItemEnum::AssocConst { .. } => "AssocConst",
        ItemEnum::AssocType { .. } => "AssocType",
    }
}

#[derive(Clone, Serialize, Debug, Eq, PartialOrd, Ord)]
pub struct ImportPath {
    pub components: Vec<String>,
    pub is_public: bool,
}

impl PartialEq for ImportPath {
    fn eq(&self, other: &Self) -> bool {
        self.components == other.components
    }
}

impl ImportPath {
    pub fn new_public(value: Vec<String>) -> Self {
        Self {
            components: value,
            is_public: true,
        }
    }
}

impl Display for ImportPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.components.join("::").as_str())
    }
}

impl ImportPath {
    pub fn replace_prefix(&self, prefix: &str, replacement: &str) -> Self {
        let mut components = self.components.clone();
        if let Some(first) = components.first_mut() {
            if let Some(stripped) = first.strip_prefix(prefix) {
                *first = replacement.to_owned() + stripped;
            }
        }
        Self {
            components,
            is_public: self.is_public,
        }
    }
}

/// An Id which uniquely identifies a crate
#[derive(Clone, Eq, Copy)]
pub struct CrateId<'a>(pub &'a Crate, pub &'a str);
impl<'a> CrateId<'a> {
    pub fn crate_name(self) -> &'a str {
        self.1
    }
}

impl Debug for CrateId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CrateId").field(&self.1).finish()
    }
}

impl Deref for CrateId<'_> {
    type Target = Crate;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Hash for CrateId<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}

impl PartialEq for CrateId<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

pub struct CrawledImportData<'a> {
    /// contains import paths public + private of every item in impls and types
    /// as well as the traits implemented by impls, may contain MORE than that, but these should be ignored
    /// paths are sorted, with public and shorter paths appearing earlier
    paths: IndexMap<(CrateId<'a>, Id), Vec<ImportPath>>,
    /// a trie of all paths, used to resolve external paths to internal paths and match up items from different modules
    inverse_paths: Trie<char, (CrateId<'a>, Id)>,
    /// a set of all impls in the crate, possibly for external traits for which paths will be present,
    /// optionally mapping to the id of the trait they implement
    impls: IndexMap<(CrateId<'a>, Id), Option<(CrateId<'a>, Id)>>,
    /// Mapping from definitions of traits to ALL their implementations, including in other crates
    inverse_impls: IndexMap<(CrateId<'a>, Id), Vec<(CrateId<'a>, Id)>>,
    /// a set of all structs/enums in the crate, always in their definition crates
    types: IndexSet<(CrateId<'a>, Id)>,

    all_crates: Vec<CrateId<'a>>,
}

impl<'a> CrawledImportData<'a> {
    /// Given path, find the item in one of the crates we crawled if it exists, and return the ID
    pub fn get_item_by_path(&self, path: &[String]) -> Option<(CrateId<'a>, Id)> {
        self.inverse_paths.get_value(path.join("::").chars())
    }

    /// Given the id of an item and the crate it's found, identify the definition id of that item (might be the same)
    /// The crate id must be the one the item was found in, i.e. it must either be in the index or paths field of the crate.
    ///
    /// Note: for std lib, private items are not documented so this won't work for those.
    /// If None is returned then either a crate was not crawled, you're looking in the wrong crate, or you didn't crawl the crate of definition with --document-private-items
    pub fn find_item(&self, id: &(CrateId<'a>, Id)) -> Option<(CrateId<'a>, Id)> {
        if id.0.index.contains_key(&id.1) {
            Some(id.clone())
        } else {
            id.0.paths
                .get(&id.1)
                .and_then(|p| self.get_item_by_path(&p.path))
        }
    }

    /// Returns all the types in all crates (structs, enums, type aliases etc)
    pub fn get_types(&self) -> impl Iterator<Item = &(CrateId<'a>, Id)> {
        self.types.iter()
    }

    /// Returns all the public types in all crates (structs, enums, type aliases etc)
    pub fn get_public_types(&self) -> impl Iterator<Item = &(CrateId<'a>, Id)> {
        self.get_types()
            .filter(|id| self.get_public_item_path(id).is_some())
    }

    /// Returns all the traits in all crates
    pub fn get_traits(&self) -> impl Iterator<Item = &(CrateId<'a>, Id)> {
        self.inverse_impls.keys()
    }

    pub fn get_impls_for_trait(
        &self,
        trait_id: &(CrateId<'a>, Id),
    ) -> Option<&[(CrateId<'a>, Id)]> {
        self.inverse_impls.get(trait_id).map(|v| v.as_slice())
    }

    /// Returns all the impls in all crates, impls for traits will only appear if the traits themselves are public
    pub fn get_public_impls(
        &self,
    ) -> impl Iterator<Item = (&(CrateId<'a>, Id), &Option<(CrateId<'a>, Id)>)> {
        self.impls.iter()
    }

    /// Returns all the public traits in all crates
    pub fn get_public_traits(&self) -> impl Iterator<Item = &(CrateId<'a>, Id)> {
        self.get_traits()
            .filter(|id| self.get_public_item_path(id).is_some())
    }

    /// searches for the given item in the given crate and returns the shortest import path (might not be public)
    pub fn get_item_path(&self, id: &(CrateId<'a>, Id)) -> Option<&ImportPath> {
        self.paths.get(id).and_then(|paths| paths.first())
    }

    /// searches for the given item in the given crate and returns the shortest import path which is public
    pub fn get_public_item_path(&self, id: &(CrateId<'a>, Id)) -> Option<&ImportPath> {
        self.get_item_path(id).filter(|p| p.is_public)
    }

    /// Searches for the given trait in the given crate, and if not found, searches the resolved external traits list from that crate.
    ///
    /// If the given item exists and is a trate it's returned, otherwise None
    pub fn find_trait(&self, trait_id: &(CrateId<'a>, Id)) -> Option<(CrateId<'a>, Id)> {
        if trait_id.0.index.contains_key(&trait_id.1) {
            Some(trait_id.clone())
        } else {
            trait_id
                .0
                .paths
                .get(&trait_id.1)
                .and_then(|p| self.get_item_by_path(&p.path))
        }
    }

    /// Searches for the given trait in both the given crate and resolved external traits list and returns the shortest path found (might not be public)
    pub fn find_trait_path(&self, trait_id: &(CrateId<'a>, Id)) -> Option<&ImportPath> {
        log::trace!(
            "Searching for trait path for trait: `{:?}` in crate: `{}`",
            &trait_id.1,
            trait_id.0.crate_name()
        );
        let out = self.get_item_path(trait_id).or_else(|| {
            log::trace!("Trait not in the given crate, searching external traits");
            trait_id
                .0
                .paths
                .get(&trait_id.1)
                .and_then(|p| self.get_item_by_path(&p.path))
                .map(|i| self.get_item_path(&i).unwrap())
        });
        log::trace!("Path found?: {:?}", out);
        out
    }

    pub fn find_public_trait_path(&self, trait_id: &(CrateId<'a>, Id)) -> Option<&ImportPath> {
        self.find_trait_path(trait_id).filter(|p| p.is_public)
    }
}

/// Used to hold data for processing of import paths in rustdoc json output
/// Future proofed as it does not use the .paths data
#[derive(Default)]
pub struct ImportPathCrawler {
    data: IndexMap<String, CrateCrawlerData>,
}

#[derive(Default)]
pub(crate) struct CrateCrawlerData {
    /// The possible import paths for each leaf item
    pub(crate) paths: IndexMap<Id, Vec<Vec<Id>>>,
    /// All the impls in the crate
    pub(crate) impls: IndexSet<Id>,
    //// All the types in the crate
    pub(crate) types: IndexSet<Id>,
    /// Mapping from implementations to the traits they're implementing
    pub(crate) impls_to_traits: IndexMap<Id, Id>,
    // /// Traits which were found with an external path which are being referenced
    // /// in the impls, if not matched up to items from other crates, need to be removed together with impls which reference those
    // pub(crate) external_traits: IndexMap<Id, Vec<String>>,
}

impl ImportPathCrawler {
    pub fn new() -> Self {
        Default::default()
    }

    /// Finalizes the crawler, once all items in every crate needed are crawled, calling this
    /// ensures that no items without a public import path are found in this struct and resolves cross-crate links
    ///
    /// crates must contain all of the crawled crates or the function will panic
    pub fn finalize(self, crates: &[Crate]) -> CrawledImportData {
        let crate_data_iter = self.data.iter().map(|(crate_id, data)| {
            (
                crates
                    .iter()
                    .find_map(|crate_| {
                        let ref_crate_id = crate_name(crate_);
                        if ref_crate_id == crate_id {
                            Some(CrateId(crate_, ref_crate_id))
                        } else {
                            None
                        }
                    })
                    .unwrap(),
                data,
            )
        });

        // before we process impls, we need to be able to resolve external items,
        // finalize all our paths and build a prefix tree from import paths to ID's so that we can resolve
        // external imports (which we only know in String form) to ID's + Crate ID's from the crate's we've crawled
        let mut paths = IndexMap::default();
        crate_data_iter
            .clone()
            .for_each(|(crate_id, data)| Self::finalize_paths(crate_id, data, &mut paths));
        paths.iter_mut().for_each(|(_, paths)| {
            // put shortest paths first, public ahead of private
            // we create a mapping from [1,inf) -> [1000, inf) then take away 1000 for public paths to shift this to [0,inf)
            // if we encounter longer paths, well somebody is doing something wrong but it ain't us
            paths.sort_by_key(|p| (p.components.len() * 1000) - (p.is_public as usize * 1000))
        });
        let mut inverse_paths = gtrie::Trie::<char, (CrateId, Id)>::new();

        paths.iter().for_each(|(id, import_paths)| {
            for p in import_paths {
                // use full import syntax with ::'s so that My::Trait::Struct does not equal MyTrait::Struct
                inverse_paths.insert(p.to_string().chars(), id.clone())
            }
        });

        // with the tree built we can process the rest
        let mut impls = IndexMap::default();
        let mut types = IndexSet::default();
        crate_data_iter.clone().for_each(|(crate_id, data)| {
            Self::finalize_impls(crate_id, data, &mut impls, &inverse_paths);
            Self::finalize_types(crate_id, data, &mut types);
        });

        let inverse_impls = impls
            .iter()
            .filter_map(|(impl_id, trait_id)| Some((trait_id.clone()?, impl_id.clone())))
            .fold(
                IndexMap::default(),
                |mut inverse_impls: IndexMap<(CrateId<'_>, Id), Vec<(CrateId<'_>, Id)>>,
                 (impl_id, trait_id)| {
                    inverse_impls
                        .entry(trait_id)
                        .or_default()
                        .push(impl_id.clone());
                    inverse_impls
                },
            );

        CrawledImportData {
            paths,
            impls,
            types,
            inverse_paths,
            inverse_impls,
            all_crates: crate_data_iter.map(|(id, _)| id).collect(),
        }
    }

    /// Finalize the crawl data by converting search traces to import paths
    fn finalize_paths<'a>(
        crate_id: CrateId<'a>,
        data: &CrateCrawlerData,
        paths: &mut IndexMap<(CrateId<'a>, Id), Vec<ImportPath>>,
    ) {
        let new_paths = data
            .paths
            .iter()
            .map(|(item_id, paths)| {
                // first we need to convert the paths into actual import paths
                // we need to also calculate visibility
                let import_paths = paths
                    .iter()
                    .map(|p| Self::search_trace_to_import_path(p, &crate_id))
                    .collect::<Vec<_>>();
                ((crate_id, item_id.clone()), import_paths)
            })
            .collect::<IndexMap<_, _>>();
        log::trace!("Paths for crate: {crate_id:?}: {new_paths:#?} ");
        paths.extend(new_paths);
    }

    /// finalizes impls given the globally (across crawled crates) known import paths to ids mapping,
    ///
    /// Returns the external items which were matched up to something in the crate
    fn finalize_impls<'a>(
        crate_id: CrateId<'a>,
        data: &CrateCrawlerData,
        impls: &mut IndexMap<(CrateId<'a>, Id), Option<(CrateId<'a>, Id)>>,
        crawled_id_map: &gtrie::Trie<char, (CrateId<'a>, Id)>,
    ) {
        let filtered_impls = data.impls.iter().filter_map(|impl_id| {
            let trait_id = if let Some(trait_id) = data.impls_to_traits.get(impl_id) {
                if crate_id.0.index.contains_key(trait_id) {
                    Some((crate_id, trait_id.clone()))
                } else {
                    let path = crate_id.0.paths.get(trait_id).unwrap().path.join("::");
                    Some(crawled_id_map.get_value(path.chars())?) // if it has a trait but can't find it filter it out
                }
            } else {
                None
            };

            // is_external implies it's resolved externally or return None
            Some(((crate_id, impl_id.clone()), trait_id))
        });

        impls.extend(filtered_impls);
    }

    fn finalize_types<'a>(
        crate_id: CrateId<'a>,
        data: &CrateCrawlerData,
        types: &mut IndexSet<(CrateId<'a>, Id)>,
    ) {
        types.extend(std::iter::repeat(crate_id).zip(data.types.iter().cloned()))
    }

    fn search_trace_to_import_path(trace: &[Id], crate_: &Crate) -> ImportPath {
        log::trace!("Converting trace: {trace:?} to import path");
        // paths look like this right now: crate c -> pub module a -> pub use B as C -> struct B = c::a::C
        // imports can rename modules and types
        // IF you can import a struct/enum/trait and it's public, it's public
        let items = trace
            .iter()
            .map(|id| Self::get_item(id, crate_))
            .map(|item| {
                let name: &str;
                let mut is_import = false;
                let is_public = matches!(item.visibility, Visibility::Public);
                let mut is_glob_import = false;
                match &item.inner {
                    ItemEnum::Import(Import {
                        name: imported_name,
                        glob,
                        ..
                    }) => {
                        name = imported_name;
                        is_import = true;
                        is_glob_import = *glob;
                    }
                    _ => name = item.name.as_ref().expect("Expected named item"),
                };
                (name, is_import, is_glob_import, is_public)
            })
            .collect::<Vec<_>>()
            .into_iter();
        let items_prev = once(None).chain(items.clone().map(Option::Some));
        items_prev.zip(items).fold(
            ImportPath::new_public(vec![]),
            |mut import_path, (previous, (current_name, is_import, is_glob, mut is_public))| {
                let mut name = current_name;

                let mut dont_emit_name = is_import;
                if let Some((prev_name, prev_is_import, prev_is_glob, prev_is_public)) = previous {
                    if prev_is_import {
                        is_public |= prev_is_public;
                        name = prev_name;
                    }
                    // glob imports implies this is a module, which we don't want to be part of the path
                    if prev_is_glob {
                        dont_emit_name = true;
                    }
                };

                if !dont_emit_name {
                    import_path.components.push(name.to_owned());
                }

                if !is_public {
                    // any part of the import path being private makes the whole thing private
                    import_path.is_public = false;
                }
                log::trace!(
                    "item - name: `{name}`, is_public: `{is_public}`, is_import: `{is_import}`, is_glob: `{is_glob}` import path is now: `{import_path}`, public?: {}", import_path.is_public
                );
                import_path
            },
        )
    }

    /// get item and panic if not found
    fn get_item<'b>(id: &Id, crate_: &'b Crate) -> &'b Item {
        crate_.index
            .get(id)
            .unwrap_or_else(|| panic!("Expected to find item with id: `{id:?}` but it was not present in the given crate index"))
    }

    fn try_get_item<'b>(id: &Id, crate_: &'b Crate) -> Option<&'b Item> {
        crate_.index.get(id)
    }

    pub fn crawl_crate(&mut self, crate_: &Crate) {
        log::trace!(
            "Crawling crate: `{}` with id: `{:?}`",
            crate_name(crate_),
            crate_.root
        );
        let crate_name = crate_name(crate_);
        self.data.insert(crate_name.to_owned(), Default::default());
        self.crawl_item(crate_.root.clone(), Borrowed(&[]), crate_, crate_name)
    }

    /// Perform depth-first search starting from this item, keep track of the generated path
    /// so that we don't have to re-lookup crate sources later, avoid recursion by keeping track of
    /// visited id's on the way.
    #[allow(clippy::too_many_arguments)]
    fn crawl_item(&mut self, id: Id, mut path: Cow<[Id]>, crate_: &Crate, crate_name: &str) {
        let item = match Self::try_get_item(&id, crate_) {
            Some(v) => v,
            None => {
                log::trace!("Could not find item in index. skipping.");
                return;
            }
        };

        log::trace!(
            "Found matching item in index: `{:?}`, item is a/an: `{}`",
            item.id,
            print_item_variant(&item.inner)
        );

        let children;

        // do we save the current path + emit if any for this item
        let mut register_path_for_item = false;

        match &item.inner {
            ItemEnum::Module(mod_) => {
                children = mod_.items.to_owned();
            }
            ItemEnum::Import(import) => {
                children = import
                    .id
                    .as_ref()
                    .map(|id| vec![id.clone()])
                    .unwrap_or_default();
            }
            ItemEnum::Trait(Trait {
                implementations, ..
            }) => {
                children = implementations.clone();
                register_path_for_item = true;
            }
            ItemEnum::Enum(Enum { impls, .. }) | ItemEnum::Struct(Struct { impls, .. }) => {
                self.data[crate_name].types.insert(id.clone());
                children = impls.to_owned();
                register_path_for_item = true;
            }
            ItemEnum::TypeAlias(ty) => {
                children = vec![];
                register_path_for_item = true;
            }
            ItemEnum::Impl(Impl { trait_, for_, .. }) => {
                // keep track of impls
                if let Some(trait_) = trait_ {
                    if !crate_.index.contains_key(&trait_.id) {
                        if !crate_.paths.contains_key(&trait_.id) {
                            log::trace!("Impl is for an external trait: `{:?}` which does not exist in the index or external paths, excluding the impl as cannot find import path for trait", trait_.id);
                            return;
                        }
                        // if the trait is external, we won't encounter it on the normal path,
                        // we have to add the import path from here, this might add traits which are
                        // not visible from the public API
                        // log::trace!("Impl is for external trait: `{:?}`, we won't find it in the index, saving import path from .paths", trait_.id);
                        // self.data[crate_name]
                        //     .external_traits
                        //     .insert(trait_.id.clone(), crate_.paths[&trait_.id].path.to_owned());
                    }

                    // store impl to trait mapping
                    if let Some(existing) = self.data[crate_name]
                        .impls_to_traits
                        .insert(id.clone(), trait_.id.clone())
                    {
                        if existing != trait_.id {
                            panic!(
                            "Impl already has a trait mapping: impl: `{id:?}`, for trait: `{existing:?}` but tried replacing with: `{:?}`",trait_.id
                        );
                        }
                    }
                };
                self.data[crate_name].impls.insert(id.clone());
                // impls don't have import paths, so simply return
                log::trace!("Item is an Impl: `{id:?}` for trait: `{:?}` for type: {for_:?}, so skipping but keeping track of Id", trait_.as_ref().map(|t| &t.id) );
                return;
            }
            // we are not interested in these
            _ => {
                log::trace!("Item is being skipped as it is not any of the types we're interested in: `{:?}`, name: {:?}",id, item.name);
                return;
            }
        };
        log::trace!("Item name: `{:?}`, children: `{:?}`", item.name, children);

        path.to_mut().push(id.clone());
        // keep track of this item now
        if register_path_for_item {
            self.store_path(&id, &path, crate_name)
        }

        for (idx, child) in children.into_iter().enumerate() {
            log::trace!("Child no: {idx} from parent id: {id:?}");
            if path.contains(&child) {
                log::trace!("Skipping child as it's already in the path.");
                continue;
            }
            self.crawl_item(child, path.clone(), crate_, crate_name);
        }
    }

    /// Either ignore the given path for this item or write it if it's new or shorter than the existing record
    fn store_path(&mut self, id: &Id, path: &[Id], crate_name: &str) {
        log::trace!("Storing path for id: `{id:?}` in crate: `{crate_name}`: {path:?}");
        // if a path was already found, use the shorter of the two
        self.data[crate_name]
            .paths
            .entry(id.to_owned())
            .or_default()
            .push(path.to_vec());
    }
}

/// Get the name of this crate
pub fn crate_name(crate_: &Crate) -> &str {
    crate_
        .index
        .get(&crate_.root)
        .as_ref()
        .unwrap()
        .name
        .as_ref()
        .unwrap()
}

/// out of the given crates figure out which one the given item belongs to if any
pub fn lookup_item_crate_source<'a>(id: &Id, crates: &'a [Crate]) -> Option<&'a Crate> {
    crates.iter().find(|crate_| crate_.index.contains_key(id))
}
