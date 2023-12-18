use std::error::Error;

use indexmap::IndexMap;
use rustdoc_types::{GenericArg, GenericBound, Generics, Id, ItemEnum, Path, Type};
use wildmatch::WildMatch;

use crate::{CrateId, CrawledImportData, ImportPath};

/// Holds the data from multiple crates combined into one queryable structure
#[derive(Debug)]
pub struct CrateUnion<'a> {
    pub types: Vec<Item<'a>>,
    pub impls: Vec<Impl<'a>>,
    pub traits: IndexMap<(CrateId<'a>, Id), Trait<'a>>,
}

impl<'a> CrateUnion<'a> {
    fn keep_item(import_path: &ImportPath, filters: &[WildMatch], include: bool) -> bool {
        let filter_response = filters.iter().map(|f| f.matches(&import_path.to_string()));
        if include {
            // dont filter or exclude on empties
            filter_response.fold(true, |a, b| a || b)
        } else {
            !filter_response.fold(false, |a, b| a && b)
        }
    }

    pub fn new(data: CrawledImportData<'a>, filters: &[WildMatch], excludes: &[WildMatch]) -> Self {
        let traits = data
            .get_public_traits()
            .map(|(crate_, trait_id)| {
                let trait_ =
                    if let ItemEnum::Trait(trait_) = &crate_.0.index.get(trait_id).unwrap().inner {
                        trait_
                    } else {
                        unreachable!()
                    };
                (
                    (*crate_, trait_id.clone()),
                    Self::convert_trait(trait_id, *crate_, trait_, &data),
                )
            })
            .collect::<IndexMap<_, _>>();

        let impls = data
            .get_public_impls()
            .map(|(impl_id, trait_id)| {
                if let ItemEnum::Impl(impl_) =
                    &impl_id.0.index.get(&impl_id.1).as_ref().unwrap().inner
                {
                    (impl_id, impl_, trait_id)
                } else {
                    unreachable!()
                }
            })
            .map(|(id, impl_, trait_id)| Self::convert_impl(id, impl_, trait_id, &data))
            .collect::<Vec<_>>();

        let mut impls_for_traits = IndexMap::<(CrateId, Id), Vec<&Impl>>::default();
        impls.iter().fold(&mut impls_for_traits, |acc, impl_| {
            if let Some(trait_) = &impl_.trait_ {
                acc.entry(trait_.clone()).or_default().push(impl_);
            }
            acc
        });

        let types = data
            .get_public_types()
            .filter_map(|(crate_, id)| {
                let import_path = data.get_item_path(&(*crate_, id.clone())).unwrap();

                let filtered = !Self::keep_item(import_path, filters, true);
                let excluded = !Self::keep_item(import_path, excludes, false);
                if filtered || excluded {
                    return None;
                }
                log::info!("Processing type: `{}`", import_path);

                Self::convert_type(crate_, id, &data, &impls_for_traits)
            })
            .collect();

        Self {
            types,
            impls,
            traits,
        }
    }

    fn convert_type(
        crate_: &CrateId<'a>,
        id: &Id,
        data: &CrawledImportData<'a>,
        impls_for_traits: &IndexMap<(CrateId<'a>, Id), Vec<&Impl<'a>>>,
    ) -> Option<Item<'a>> {
        let item = &crate_.index[id];
        let (generics, fields) = match &item.inner {
            ItemEnum::Union(_) => return None,
            ItemEnum::Struct(struct_) => (
                &struct_.generics,
                match &struct_.kind {
                    rustdoc_types::StructKind::Unit => Default::default(),
                    rustdoc_types::StructKind::Tuple(fields) => fields.clone(),
                    rustdoc_types::StructKind::Plain { fields, .. } => {
                        fields.iter().cloned().map(Option::Some).collect()
                    }
                },
            ),
            ItemEnum::Enum(enum_) => (&enum_.generics, Default::default()),
            _ => unreachable!(),
        };

        if !generics.params.is_empty() {
            return None;
        }

        let fields = fields
            .into_iter()
            .enumerate()
            .filter_map(|(idx, field_id)| {
                let field = &crate_.index[&field_id?];
                log::info!("field: {:?}", field);
                match &field.inner {
                    ItemEnum::StructField(type_) => {
                        let mut converted_type =
                            Self::type_to_intermediate_type(*crate_, type_, data)
                                .with_bubbled_up_unsupported();

                        converted_type.make_paths_absolute(data, impls_for_traits);

                        Some((
                            field.name.clone().unwrap_or(idx.to_string()),
                            converted_type.try_into().unwrap(),
                        ))
                    }
                    _ => unreachable!(),
                }
            })
            .collect();

        Some(Item {
            fields,
            // TODO: docstrings
            docstrings: vec![],
            type_: UniversalType::Path {
                // TODO: generics support
                id: (*crate_, id.clone()),
                generics: vec![],
            },
        })
    }

    fn convert_impl(
        id: &(CrateId<'a>, Id),
        impl_: &rustdoc_types::Impl,
        trait_id: &Option<(CrateId<'a>, Id)>,
        data: &CrawledImportData<'a>,
    ) -> Impl<'a> {
        let mut functions = Vec::default();
        let mut associated_types = Vec::default();
        log::info!("Processing impl : `{:?}`", id);
        for i in &impl_.items {
            let item = id.0.index.get(i).unwrap();

            // skip private functions
            log::info!("item: {:?}", item);
            if !(matches!(item.inner, ItemEnum::Function(_))
                || item.visibility != rustdoc_types::Visibility::Public)
            {
                log::info!("Skipping");
                continue;
            }

            match &item.inner {
                ItemEnum::Function(f) => functions.push(f),
                ItemEnum::AssocConst { .. } => (), //TODO: support assoc consts and types
                ItemEnum::AssocType { default, .. } => {
                    // the rest of the fields should be empty in impls assoc_types TODO: assert this
                    associated_types.push((
                        item.name.clone().unwrap(),
                        Self::type_to_intermediate_type(id.0, default.as_ref().unwrap(), data),
                    ));
                }
                _ => unreachable!(),
            }
        }

        let for_ = Self::type_to_intermediate_type(id.0, &impl_.for_, data);

        let functions = functions
            .iter()
            .filter_map(|f| {
                // TODO: support generics (either by specifying which types to cover, or somehow supporting them dynamically in lua)
                if !f.generics.params.is_empty() || !f.generics.where_predicates.is_empty() {
                    return None;
                }
                let args = f.decl.inputs.iter().map(|(name, type_)| {
                    (
                        name.to_owned(),
                        Self::type_to_intermediate_type(id.0, type_, data),
                    )
                });
                let output = f
                    .decl
                    .output
                    .as_ref()
                    .map(|type_| Self::type_to_intermediate_type(id.0, type_, data))
                    .unwrap_or(IntermediateType::Unit);

                if trait_id.is_some() {
                    Some(Function::Intermediate {
                        args: args.collect(),
                        output,
                        // TODO: docstrings
                        docstrings: vec![],
                    })
                } else {
                    if !impl_.generics.params.is_empty()
                        || !impl_.generics.where_predicates.is_empty()
                    {
                        return None;
                    }
                    let subs = &[("Self".to_owned(), for_.clone())];
                    Some(Function::Universal {
                        args: args
                            .map(|(a, b)| (a, b.substitute(subs).unwrap().try_into().unwrap()))
                            .collect(),
                        output: output.substitute(subs).unwrap().try_into().unwrap(),
                        // TODO: docstrings
                        docstrings: vec![],
                    })
                }
            })
            .collect();

        let blanket_type = impl_
            .blanket_impl
            .as_ref()
            .map(|blanket_type| Self::type_to_intermediate_type(id.0, blanket_type, data));

        let generics = Self::convert_generics(id.0, &impl_.generics, data);
        Impl {
            id: id.clone(),
            trait_: trait_id.clone(),
            for_,
            blanket_type,
            functions,
            generics,
            associated_types,
        }
    }

    fn convert_trait(
        trait_id: &Id,
        def_crate: CrateId<'a>,
        trait_: &rustdoc_types::Trait,
        data: &CrawledImportData<'a>,
    ) -> Trait<'a> {
        let generics = Self::convert_generics(def_crate, &trait_.generics, data);
        // TODO: use trait_.bounds
        Trait {
            type_: IntermediateType::Path {
                id: (def_crate, trait_id.clone()),
                generics: generics.bounds.keys().cloned().collect(),
            },
            generics,
        }
    }
    /// Converts generics to universal form with bounds
    fn convert_generics(
        found_in_crate: CrateId<'a>,
        generics: &Generics,
        data: &CrawledImportData<'a>,
    ) -> GenericBounds<'a> {
        let mut bounds = IndexMap::<_, Vec<_>>::default();
        for p in &generics.params {
            let name = &p.name;
            match &p.kind {
                rustdoc_types::GenericParamDefKind::Type {
                    bounds: generic_bounds,
                    default,
                    synthetic,
                } => bounds
                    .entry(IntermediateType::Generic(name.to_owned()))
                    .or_default()
                    .extend(generic_bounds.iter().filter_map(|b| {
                        if let GenericBound::TraitBound { trait_, .. } = b {
                            let trait_ =
                                Self::path_to_intermediate_type(found_in_crate, trait_, &data);
                            Some(trait_)
                        } else {
                            None
                        }
                    })),
                _ => (),
            }
        }

        GenericBounds { bounds }
    }

    fn type_to_intermediate_type(
        found_in_crate: CrateId<'a>,
        type_: &Type,
        data: &CrawledImportData<'a>,
    ) -> IntermediateType<'a> {
        match type_ {
            Type::ResolvedPath(p) => Self::path_to_intermediate_type(found_in_crate, p, data),
            Type::QualifiedPath {
                name,
                //TODO generics
                args,
                self_type,
                trait_,
            } => {
                let base = Self::type_to_intermediate_type(found_in_crate, self_type, data).into();
                let trait_ =
                    Self::path_to_intermediate_type(found_in_crate, trait_.as_ref().unwrap(), data)
                        .into();
                let name = name.to_owned();
                IntermediateType::RelativePath { base, trait_, name }
            }
            Type::Generic(g) => IntermediateType::Generic(g.to_owned()),
            Type::Primitive(p) => IntermediateType::Primitive(p.to_owned()),
            Type::Tuple(t) => {
                let types = t
                    .iter()
                    .map(|t| Self::type_to_intermediate_type(found_in_crate, t, data))
                    .collect();
                IntermediateType::Tuple(types)
            }
            Type::Slice(s) => IntermediateType::Slice(
                Self::type_to_intermediate_type(found_in_crate, s.as_ref(), data).into(),
            ),
            Type::Array { type_, len } => IntermediateType::Array(
                Self::type_to_intermediate_type(found_in_crate, type_, data).into(),
                len.to_owned(),
            ),
            Type::BorrowedRef {
                lifetime,
                mutable,
                type_,
            } => IntermediateType::Reference {
                inner: Self::type_to_intermediate_type(found_in_crate, type_, data).into(),
                mutable: *mutable,
            },
            Type::ImplTrait(_) => IntermediateType::Unsupported(UnsupportedType::ImplTrait),
            Type::Infer => IntermediateType::Unsupported(UnsupportedType::Infer),
            Type::RawPointer { mutable, type_ } => {
                IntermediateType::Unsupported(UnsupportedType::RawPointer)
            }
            Type::FunctionPointer(_) => {
                IntermediateType::Unsupported(UnsupportedType::FunctionPointer)
            }
            Type::DynTrait(_) => IntermediateType::Unsupported(UnsupportedType::DynTrait),
        }
    }

    fn path_to_intermediate_type(
        found_in_crate: CrateId<'a>,
        path: &Path,
        data: &CrawledImportData<'a>,
    ) -> IntermediateType<'a> {
        // figure out global id
        let id = match data.find_item(&(found_in_crate, path.id.clone())) {
            Some(v) => v,
            None => return IntermediateType::Unsupported(UnsupportedType::MissingPath),
        };

        // process generics recursively
        let generics = if let Some(g) = path.args.as_deref() {
            match g {
                rustdoc_types::GenericArgs::AngleBracketed { args, bindings } => args
                    .iter()
                    .map(|arg| match arg {
                        GenericArg::Type(t) => {
                            Self::type_to_intermediate_type(found_in_crate, t, data)
                        }
                        GenericArg::Lifetime(_) => {
                            IntermediateType::Unsupported(UnsupportedType::Lifetime)
                        }
                        GenericArg::Const(_) => {
                            IntermediateType::Unsupported(UnsupportedType::Const)
                        }
                        GenericArg::Infer => IntermediateType::Unsupported(UnsupportedType::Infer),
                    })
                    .collect(),
                rustdoc_types::GenericArgs::Parenthesized { .. } => {
                    vec![IntermediateType::Unsupported(
                        UnsupportedType::ParenthesisedGeneric,
                    )]
                }
            }
        } else {
            Default::default()
        };

        IntermediateType::Path { id, generics }
    }
}

/// A struct/enum/union
#[derive(Debug)]
pub struct Item<'a> {
    pub type_: UniversalType<'a>,
    pub fields: Vec<(String, UniversalType<'a>)>,
    pub docstrings: Vec<String>,
}

#[derive(Debug)]
pub struct Impl<'a> {
    pub id: (CrateId<'a>, Id),
    pub trait_: Option<(CrateId<'a>, Id)>,
    pub for_: IntermediateType<'a>,
    pub blanket_type: Option<IntermediateType<'a>>,
    pub functions: Vec<Function<'a>>,
    pub generics: GenericBounds<'a>,
    pub associated_types: Vec<(String, IntermediateType<'a>)>,
}

#[derive(Default, Debug)]
pub struct GenericBounds<'a> {
    pub bounds: IndexMap<IntermediateType<'a>, Vec<IntermediateType<'a>>>,
}

#[derive(Debug)]
pub struct Trait<'a> {
    pub type_: IntermediateType<'a>,
    // TODO: better generics format
    pub generics: GenericBounds<'a>,
}

/// A function or method
#[derive(Debug)]
pub enum Function<'a> {
    /// A function that is part of a trait, it might contain generics and relative paths
    Intermediate {
        args: Vec<(String, IntermediateType<'a>)>,
        output: IntermediateType<'a>,
        docstrings: Vec<String>,
    },
    /// A fully resolved function, all generics and relative paths are resolved
    /// these are found naturally in impl blocks for structs/enums/unions, but can also be resolved from trait functions given context.
    Universal {
        args: Vec<(String, UniversalType<'a>)>,
        output: UniversalType<'a>,
        docstrings: Vec<String>,
    },
}

/// A type that is valid in rustdoc, but not parsed fully by us, "Don't care" values about the existence of which we might want to know.
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum UnsupportedType {
    /// I.e. `Fn(usize) -> String`
    ParenthesisedGeneric,
    /// I.e. `'a` in '&'a T'
    Lifetime,
    /// I.e. `const FOO: usize = 5`
    Const,
    /// I.e. `_`
    Infer,
    /// I.e. `impl Trait`
    ImplTrait,
    /// I.e. `*const T` or `*mut T`
    RawPointer,
    /// I.e. `extern "ABI" fn`
    FunctionPointer,
    /// I.e. `dyn Trait`
    DynTrait,
    /// Unresolvable cross-crate path, most likely due to missing private items in the crawl or missing crates
    MissingPath,
    Multiple(Vec<UnsupportedType>),
}

impl FromIterator<UnsupportedType> for Option<UnsupportedType> {
    fn from_iter<T: IntoIterator<Item = UnsupportedType>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let inner = iter.collect::<Vec<_>>();
        if inner.len() > 1 {
            Some(UnsupportedType::Multiple(inner))
        } else if inner.is_empty() {
            None
        } else {
            Some(inner[0].clone())
        }
    }
}

/// Partially converted rustdoc type, still has relative paths and generics but all references contain a crateID
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum IntermediateType<'a> {
    /// A built-in primitive which has no "definition" as such
    /// e.g. `u32`
    Primitive(String),
    /// Anything that can be imported,
    /// structs, unions enums and traits
    /// e.g. `std::collections::HashMap<K,V>` or `std::ops::Add<usize>`
    Path {
        /// The globally (across a set of crates being queried) unique id of the crate this type/trait is defined in
        id: (CrateId<'a>, Id),
        /// The generics of the type, no lifetimes are included, purely types or named generics
        generics: Vec<IntermediateType<'a>>,
    },
    /// A path based off of a type, e.g. `<T as Add<usize>>::Output`
    /// <base as trait_>::name
    RelativePath {
        base: Box<IntermediateType<'a>>,
        trait_: Box<IntermediateType<'a>>,
        name: String,
    },
    /// String identifier of a named generic variable,
    /// e.g. `T`
    Generic(String),
    /// the unit type `()`
    Unit,
    /// A tuple of types,
    /// e.g. `(T, U)`
    Tuple(Vec<IntermediateType<'a>>),
    /// A slice of a type
    /// e.g. `[T]`
    Slice(Box<IntermediateType<'a>>),
    /// A fixed size array of a type
    /// e.g. `[T; 5]`
    Array(Box<IntermediateType<'a>>, String),
    /// A reference to a type
    /// e.g. `&T`
    Reference {
        inner: Box<IntermediateType<'a>>,
        mutable: bool,
    },
    Unsupported(UnsupportedType),
}

impl<'a> From<UniversalType<'a>> for IntermediateType<'a> {
    fn from(value: UniversalType<'a>) -> Self {
        match value {
            UniversalType::Primitive(p) => IntermediateType::Primitive(p),
            UniversalType::Path { id, generics } => IntermediateType::Path {
                id,
                generics: generics.into_iter().map(Into::into).collect(),
            },
            UniversalType::Unit => IntermediateType::Unit,
            UniversalType::Tuple(t) => {
                IntermediateType::Tuple(t.into_iter().map(Into::into).collect())
            }
            UniversalType::Slice(s) => IntermediateType::Slice(Box::new((*s).into())),
            UniversalType::Array(i, l) => IntermediateType::Array(Box::new((*i).into()), l),
            UniversalType::Reference { inner, mutable } => IntermediateType::Reference {
                inner: Box::new((*inner).into()),
                mutable,
            },
            UniversalType::Unsupported(u) => IntermediateType::Unsupported(u),
        }
    }
}

impl<'a> IntermediateType<'a>
where
    String: Eq,
{
    // /// flattens relative paths without generics, if generics are present nothing happens
    pub fn make_paths_absolute(
        &mut self,
        data: &CrawledImportData<'a>,
        impls_for_traits: &IndexMap<(CrateId<'a>, Id), Vec<&Impl<'a>>>,
    ) -> &mut Self {
        // resolve all paths but cant' resolve unresolved generics at the top
        while !self.has_generics() && self.has_relative_paths() {
            self._make_paths_absolute(data, impls_for_traits);
        }

        self
    }

    /// helper for make_paths_absolute, assumes no generics present, and that every trait has at least a single impl
    fn _make_paths_absolute(
        &mut self,
        data: &CrawledImportData<'a>,
        impls_for_traits: &IndexMap<(CrateId<'a>, Id), Vec<&Impl<'a>>>,
    ) {
        match self {
            IntermediateType::Path { generics, .. } => generics
                .iter_mut()
                .for_each(|g| g._make_paths_absolute(data, impls_for_traits)),

            IntermediateType::RelativePath { base, trait_, name } => {
                // first find the trait and the implementations for that trait, identify if the base type implements the trait (including with generics)
                // if it does resolve the relative path using the associated types on the impl
                if let IntermediateType::Path { id, generics } = trait_.as_ref() {
                    let impls = impls_for_traits.get(id).unwrap();
                    // assumes no generics so this should never fail
                    let base: UniversalType = base.as_ref().clone().try_into().unwrap();
                    // this should NEVER match more than one impl, if it does we have a problem
                    // and also should match at least one impl, otherwise we have traits with no impls which should not be here
                    let (impl_, subs) = impls
                        .iter()
                        .find_map(|i| Some((i, base.matches_for_type(i)?)))
                        .unwrap();
                    // we know the impl block matches the base type with the given subsitution, we need to now check if there are any bounds on the types in the impl block and if so
                    // check if the substituted types satisfy those.
                    // TODO: check bounds

                    // now we need to resolve the relative path using the associated types on the impl
                    // the impl may contain generics so we need to substitute those in with the subsitution we found earlier
                    let replacement = impl_
                        .associated_types
                        .iter()
                        .find_map(|(n, ty)| (name == n).then_some(ty))
                        .unwrap_or_else(|| panic!("Missing associated type: `{}` when trying to make type with base: `{:?}` absolute, when looking through impls: `{:?}`",
                            name, base, impl_));

                    *self = replacement.substitute(&subs).unwrap_or_else(|_| {
                        panic!(
                            "Substitution failed when replacing: {:?}, with substitution: {:?}",
                            replacement, subs
                        )
                    })
                } else {
                    unreachable!()
                };
            }
            IntermediateType::Slice(t) => t._make_paths_absolute(data, impls_for_traits),
            IntermediateType::Array(t, _) => t._make_paths_absolute(data, impls_for_traits),
            IntermediateType::Reference { inner, .. } => {
                inner._make_paths_absolute(data, impls_for_traits)
            }
            _ => (),
        }
    }

    pub fn has_generics(&self) -> bool {
        match self {
            IntermediateType::Generic(_) => true,
            IntermediateType::Path { generics, .. } => generics.iter().any(|i| i.has_generics()),
            IntermediateType::RelativePath { base, trait_, .. } => {
                base.has_generics() || trait_.has_generics()
            }
            IntermediateType::Slice(t) => t.has_generics(),
            IntermediateType::Array(t, _) => t.has_generics(),
            IntermediateType::Reference { inner, .. } => inner.has_generics(),
            _ => false,
        }
    }

    pub fn has_relative_paths(&self) -> bool {
        match self {
            IntermediateType::Path { generics, .. } => {
                generics.iter().any(|i| i.has_relative_paths())
            }
            IntermediateType::RelativePath { .. } => true,
            IntermediateType::Slice(t) => t.has_relative_paths(),
            IntermediateType::Array(t, _) => t.has_relative_paths(),
            IntermediateType::Reference { inner, .. } => inner.has_relative_paths(),
            _ => false,
        }
    }

    pub fn with_bubbled_up_unsupported(mut self) -> Self {
        self.bubble_up_unsupported();
        self
    }

    pub fn bubble_up_unsupported(&mut self) -> &Self {
        if let Some(u) = self.find_unsupported() {
            *self = IntermediateType::Unsupported(u);
        }
        self
    }

    pub fn find_unsupported(&self) -> Option<UnsupportedType> {
        match self {
            IntermediateType::Path { generics: t, .. } | IntermediateType::Tuple(t) => {
                t.iter().filter_map(|t| t.find_unsupported()).collect()
            }
            IntermediateType::RelativePath { base, trait_, name } => base
                .find_unsupported()
                .into_iter()
                .chain(trait_.find_unsupported())
                .collect(),
            IntermediateType::Slice(t)
            | IntermediateType::Array(t, _)
            | IntermediateType::Reference { inner: t, .. } => t.find_unsupported(),
            IntermediateType::Unsupported(u) => Some(u.clone()),
            _ => None,
        }
    }

    /// substitute generics in the given type with the given substitutions, throws error if substitution is not found for a generic
    pub fn substitute<T: Into<IntermediateType<'a>> + Clone>(
        &self,
        substitutions: &[(String, T)],
    ) -> Result<IntermediateType<'a>, Box<dyn Error>> {
        Ok(match self {
            IntermediateType::Slice(i) => {
                IntermediateType::Slice(i.substitute(substitutions)?.into())
            }
            IntermediateType::Path { generics, id } => IntermediateType::Path {
                generics: generics
                    .iter()
                    .map(|g| g.substitute(substitutions))
                    .collect::<Result<_, _>>()?,
                id: id.clone(),
            },
            IntermediateType::RelativePath { base, trait_, name } => {
                IntermediateType::RelativePath {
                    base: base.substitute(substitutions)?.into(),
                    trait_: trait_.substitute(substitutions)?.into(),
                    name: name.clone(),
                }
            }
            IntermediateType::Reference { inner, mutable } => IntermediateType::Reference {
                inner: inner.substitute(substitutions)?.into(),
                mutable: *mutable,
            },
            IntermediateType::Generic(g) => substitutions
                .iter()
                .find_map(|(l, r)| (l == g).then_some((r.clone()).into()))
                .expect("Missing generic in substitution")
                .clone(),

            IntermediateType::Tuple(t) => IntermediateType::Tuple(
                t.iter()
                    .map(|v| v.substitute(substitutions))
                    .collect::<Result<_, _>>()?,
            ),
            IntermediateType::Array(t, len) => {
                IntermediateType::Array(t.substitute(substitutions)?.into(), len.clone())
            }
            _ => self.clone(),
        })
    }
    /// one-way unification algorithm, returns the substitutions if the types unify, otherwise None.
    /// The substitutions can be applied to this type to make it equal to the other type.
    /// Will panic on encountering relative paths
    pub fn match_(
        &self,
        other: &UniversalType<'a>,
        substitutions: &mut Vec<(String, UniversalType<'a>)>,
    ) -> bool {
        match (self, other) {
            (IntermediateType::Unit, UniversalType::Unit) => return true,
            (IntermediateType::Primitive(l), UniversalType::Primitive(r)) => return l == r,
            (
                IntermediateType::Path {
                    id: l_i,
                    generics: l_g,
                },
                UniversalType::Path { id, generics },
            ) => {
                if l_i != id || l_g.len() != generics.len() {
                    return false;
                }

                for (l, r) in l_g.iter().zip(generics.iter()) {
                    if !l.match_(r, substitutions) {
                        return false;
                    }
                }
            }
            (IntermediateType::Tuple(l), UniversalType::Tuple(r)) => {
                if l.len() != r.len() {
                    return false;
                }

                for (l, r) in l.iter().zip(r.iter()) {
                    if !l.match_(r, substitutions) {
                        return false;
                    }
                }
            }
            (IntermediateType::Slice(l), UniversalType::Slice(r)) => {
                if !l.match_(r, substitutions) {
                    return false;
                }
            }
            (IntermediateType::Array(t_l, l_l), UniversalType::Array(t, l)) => {
                if l_l != l || !t_l.match_(t, substitutions) {
                    return false;
                }
            }
            (
                IntermediateType::Reference {
                    inner: l_i,
                    mutable: l_m,
                },
                UniversalType::Reference { inner, mutable },
            ) => {
                if l_m != mutable || !l_i.match_(inner, substitutions) {
                    return false;
                }
            }
            (IntermediateType::Generic(g), r) => {
                if let Some(existing_sub) = substitutions
                    .iter()
                    .find_map(|(l, r)| (l == g).then_some(r))
                {
                    // in case a substitution already exists, the check if the new substitution is equal,
                    // otherwise we fail early.
                    if existing_sub != r {
                        return false;
                    }
                };
                substitutions.push((g.to_owned(), r.to_owned()));
            }
            _ => return false,
        };

        true
    }
}

/// A fully converted rustdoc type, all paths are fully resolved and so are the generics
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UniversalType<'a> {
    /// A built-in primitive which has no "definition" as such
    /// e.g. `u32`
    Primitive(String),
    /// Anything that can be imported,
    /// structs, unions enums and traits
    /// e.g. `std::collections::HashMap<K,V>` or `std::ops::Add<usize>`
    Path {
        /// The globally (across a set of crates being queried) unique id of the crate this type/trait is defined in
        id: (CrateId<'a>, Id),
        /// resolved generics
        generics: Vec<UniversalType<'a>>,
    },
    /// the unit type `()`
    Unit,
    /// A tuple of types,
    /// e.g. `(T, U)`
    Tuple(Vec<UniversalType<'a>>),
    /// A slice of a type
    /// e.g. `[T]`
    Slice(Box<UniversalType<'a>>),
    /// A fixed size array of a type
    /// e.g. `[T; 5]`
    Array(Box<UniversalType<'a>>, String),
    /// A reference to a type
    /// e.g. `&T`
    Reference {
        inner: Box<UniversalType<'a>>,
        mutable: bool,
    },
    Unsupported(UnsupportedType),
}

impl<'a> TryFrom<IntermediateType<'a>> for UniversalType<'a> {
    type Error = Box<dyn Error>;

    fn try_from(value: IntermediateType<'a>) -> Result<Self, Self::Error> {
        Ok(match value {
            IntermediateType::Primitive(p) => UniversalType::Primitive(p),
            IntermediateType::Path { id, generics } => UniversalType::Path {
                id,
                generics: generics
                    .into_iter()
                    .map(UniversalType::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
            },
            t @ IntermediateType::RelativePath { .. } => {
                Err(format!("Relative path not resolved: {:?}", t))?
            }
            g @ IntermediateType::Generic(_) => Err(format!("generic not resolved: {:?}", g))?,
            IntermediateType::Unit => UniversalType::Unit,
            IntermediateType::Tuple(t) => UniversalType::Tuple(
                t.into_iter()
                    .map(UniversalType::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            IntermediateType::Slice(s) => UniversalType::Slice(UniversalType::try_from(*s)?.into()),
            IntermediateType::Array(a, s) => {
                UniversalType::Array(UniversalType::try_from(*a)?.into(), s)
            }
            IntermediateType::Reference { inner, mutable } => UniversalType::Reference {
                inner: UniversalType::try_from(*inner)?.into(),
                mutable,
            },
            IntermediateType::Unsupported(u) => UniversalType::Unsupported(u),
        })
    }
}

impl<'a> UniversalType<'a> {
    /// Checks if the given type can be unified with the type the impl block is for and returns the substitution required to do so.
    ///
    /// If the type cannot be unified with the impl block type, None is returned.
    /// If no substitutions are required, an empty vec is returned.
    pub fn matches_for_type(&self, impl_: &Impl<'a>) -> Option<Vec<(String, UniversalType<'a>)>> {
        let mut substitutions = Vec::default();
        impl_
            .for_
            .match_(self, &mut substitutions)
            .then_some(substitutions)
    }
}
