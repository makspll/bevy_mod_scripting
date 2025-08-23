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
}

impl std::fmt::Debug for ImportPathElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportPathElement::Rename(did, name) => write!(f, "{did:?} as {name}"),
            ImportPathElement::Item(did) => write!(f, "{did:?}"),
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
        self.crawl_module(crate_num.as_def_id(), &[])
    }

    fn crawl_module(&mut self, did: DefId, frontier: &[ImportPathElement]) {
        trace!("Crawling module {did:?}");

        let mut new_frontier = frontier.to_vec();
        new_frontier.push(ImportPathElement::Item(did));
        // do not allow modification or weird things happen
        let new_frontier = &new_frontier;

        let children = if did.is_local() {
            self.tcx.module_children_local(did.expect_local())
        } else {
            self.tcx.module_children(did)
        };

        for child in children {
            let rename = child.ident.to_string();

            if !self.include_private_paths && !child.vis.is_public() {
                trace!("Skipping private child {rename:?}");
                continue;
            }

            let did = if let Some(did) = child.res.opt_def_id() {
                did
            } else {
                continue;
            };

            trace!(
                "Crawling item: '{:?}', of kind: '{:?}'",
                did,
                self.tcx.def_kind(did)
            );

            match self.tcx.def_kind(did) {
                DefKind::Mod => self.crawl_module(did, new_frontier),
                DefKind::Struct | DefKind::Union | DefKind::Enum | DefKind::Trait => {
                    // save the rename and the def id

                    let mut new_frontier = new_frontier.clone();
                    new_frontier.push(ImportPathElement::Rename(did, rename));

                    trace!("saving import path for {did:?}: {new_frontier:?}");
                    self.cache.entry(did).or_default().push(new_frontier);
                }
                _ => continue,
            }
        }
    }

    pub(crate) fn find_import_paths(&self, def_id: DefId) -> Vec<String> {
        self.cache
            .get(&def_id)
            .map(|v| {
                v.iter()
                    .map(|elems| self.import_path_to_def_string(elems))
                    .collect()
            })
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
                ImportPathElement::Item(did) => self.tcx.item_name(*did).to_ident_string(),
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
