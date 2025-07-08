use rustc_hir::def_id::LOCAL_CRATE;

use crate::{Args, BevyCtxt, Meta, ProxyMeta, META_VERSION};

/// Finds and caches relevant traits, if they cannot be found throws an ICE
pub(crate) fn write_meta(ctxt: &mut BevyCtxt<'_>, _args: &Args) -> bool {
    let tcx = &ctxt.tcx;

    let mut proxies = Vec::with_capacity(ctxt.reflect_types.len());
    for proxy in ctxt.reflect_types.keys() {
        let def_path_hash = tcx.def_path_hash(*proxy);
        proxies.push(ProxyMeta {
            ident: tcx.item_name(*proxy).to_ident_string(),
            stable_crate_id: def_path_hash.stable_crate_id().as_u64(),
            local_hash_id: def_path_hash.local_hash().as_u64(),
        });
    }
    let will_generate = !proxies.is_empty();
    let meta = Meta {
        crate_name: tcx.crate_name(LOCAL_CRATE).to_string(),
        proxies,
        will_generate,
        meta_version: META_VERSION.to_string(),
        timestamp: chrono::Local::now().naive_local(),
    };

    ctxt.meta_loader
        .write_meta(ctxt.tcx.crate_name(LOCAL_CRATE).as_str(), &meta);

    if !will_generate {
        return false;
    }

    true
}
