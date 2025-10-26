use rustc_hir::def_id::LOCAL_CRATE;

use crate::{Args, BevyCtxt};

/// Finds and caches import paths
pub(crate) fn crawl_paths(ctxt: &mut BevyCtxt<'_>, _args: &Args) -> bool {
    ctxt.path_finder.crawl_crate(LOCAL_CRATE);

    // for c in ctxt.tcx.crates(()) {
    //     ctxt.path_finder.crawl_crate(*c);
    // }
    true
}
