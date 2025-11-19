use indexmap::IndexMap;
use log::trace;
use rustc_hir::{
    def::DefKind,
    def_id::{CrateNum, DefId},
};
use rustc_middle::ty::TyCtxt;

#[derive(Clone)]
pub(crate) enum ImportPathElement {
    /// renaming of the def id to a new name
    Rename(DefId, String),
    /// direct import of the def id by name
    Item(DefId),
    /// A crate root
    Crate(CrateNum),
}

impl ImportPathElement {
    pub fn def_id(&self) -> Option<DefId> {
        Some(match self {
            Self::Rename(did, _) => *did,
            Self::Item(did) => *did,
            _ => return None,
        })
    }
}

impl std::fmt::Debug for ImportPathElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportPathElement::Rename(did, name) => write!(f, "{did:?} as {name}"),
            ImportPathElement::Item(did) => write!(f, "{did:?}"),
            ImportPathElement::Crate(crate_num) => write!(f, "{crate_num:?}"),
        }
    }
}

/// A mechanism to figure out all the import paths for an item.
/// Because we do not need ALL the items in the crate, we start searching from the item itself and traverse up the tree.
/// Caches results for already found items.
pub(crate) struct ImportPathFinder<'tcx> {
    pub(crate) tcx: TyCtxt<'tcx>,
    pub(crate) cache: IndexMap<DefId, Vec<Vec<ImportPathElement>>>,
    pub(crate) include_private_paths: bool,
    pub(crate) import_path_processor: Option<Box<dyn Fn(&str) -> String>>,
}

impl<'tcx> ImportPathFinder<'tcx> {
    /// Creates a new ImportPathFinder with the provided TyCtxt
    pub(crate) fn new(
        tcx: TyCtxt<'tcx>,
        include_private_paths: bool,
        import_path_processor: Option<Box<dyn Fn(&str) -> String>>,
    ) -> Self {
        Self {
            tcx,
            cache: Default::default(),
            include_private_paths,
            import_path_processor,
        }
    }

    pub(crate) fn crawl_crate(&mut self, crate_num: CrateNum) {
        self.crawl_module(
            crate_num.as_def_id(),
            &[ImportPathElement::Crate(crate_num)],
        );
        // sort by length of path, shortest wins
        self.cache.iter_mut().for_each(|(_, paths)| {
            paths.sort_by_key(|a| a.len());
        });
    }

    fn crawl_module(&mut self, did: DefId, frontier: &[ImportPathElement]) {
        trace!("Crawling module {did:?}");

        // Get children of the module
        let children = if did.is_local() {
            self.tcx.module_children_local(did.expect_local())
        } else {
            self.tcx.module_children(did)
        };

        for child in children {
            // Skip if the child has no DefId
            let did = match child.res.opt_def_id() {
                Some(did) => did,
                None => {
                    trace!("Skipping child without did {:?}", child.ident);
                    continue;
                }
            };

            // Skip private items if we don't include them
            if !self.include_private_paths && !child.vis.is_public() {
                trace!("Skipping private child {:?}", child.ident);
                continue;
            }

            // skip non local items, i.e. don't go crawling serde
            if !did.is_local() {
                trace!("Skipping non-local child {:?}", child.ident);
                // continue;
            }

            let rename = (Some(child.ident.name) != self.tcx.opt_item_name(did))
                .then_some(child.ident.as_str());

            self.crawl_item(did, frontier, rename);
        }
    }

    fn crawl_item(&mut self, did: DefId, frontier: &[ImportPathElement], rename: Option<&str>) {
        trace!(
            "Crawling item: '{:?}', of kind: '{:?}'",
            did,
            self.tcx.def_kind(did)
        );

        match self.tcx.def_kind(did) {
            DefKind::Mod => {
                // Only recurse if this DefId is not already in the current path
                if frontier
                    .iter()
                    .any(|el| el.def_id().is_some_and(|id| id == did))
                {
                    trace!("Cycle detected for {did:?}, skipping recursion");
                    return;
                }
                let mut new_frontier = frontier.to_vec();
                new_frontier.push(
                    rename
                        .map(|rename| ImportPathElement::Rename(did, rename.to_string()))
                        .unwrap_or(ImportPathElement::Item(did)),
                );

                self.crawl_module(did, &new_frontier)
            }

            DefKind::Struct | DefKind::Union | DefKind::Enum | DefKind::Trait => {
                // Save the rename and the DefId
                let mut path_for_item = frontier.to_vec();

                path_for_item.push(ImportPathElement::Item(did));

                trace!("Saving import path for {did:?}: {path_for_item:?}");
                self.cache.entry(did).or_default().push(path_for_item);
            }

            _ => (),
        }
    }

    /// Like find_import_paths but won't always return at least one path
    pub(crate) fn find_import_paths_no_fallback(&self, def_id: DefId) -> Option<Vec<String>> {
        self.cache.get(&def_id).map(|v| {
            v.iter()
                .map(|elems| self.import_path_to_def_string(elems))
                .collect()
        })
    }

    /// Finds the import path for the item if there is one given the current settings
    /// or returns a fallback value based on the defID which might not be accurate
    pub(crate) fn find_import_paths(&self, def_id: DefId) -> Vec<String> {
        self.find_import_paths_no_fallback(def_id)
            .unwrap_or_else(|| {
                let path = self.tcx.def_path_str(def_id);
                if let Some(p) = &self.import_path_processor {
                    vec![(p)(&path)]
                } else {
                    vec![path]
                }
            })
    }

    pub(crate) fn import_path_to_def_string(&self, path: &[ImportPathElement]) -> String {
        let out = path
            .iter()
            .map(|elem| match elem {
                ImportPathElement::Rename(_, name) => name.to_owned(),
                ImportPathElement::Item(did) => self
                    .tcx
                    .opt_item_name(*did)
                    .expect("missing item name")
                    .to_ident_string(),
                ImportPathElement::Crate(crate_num) => {
                    self.tcx.crate_name(*crate_num).to_ident_string()
                }
            })
            .inspect(|e| {
                if e.is_empty() {
                    panic!("empty path elem: {e}")
                }
            })
            .collect::<Vec<_>>()
            .join("::");

        if let Some(processor) = &self.import_path_processor {
            (processor)(&out)
        } else {
            out
        }
    }
}
