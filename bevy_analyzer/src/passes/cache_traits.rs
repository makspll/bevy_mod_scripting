use log::debug;
use rustc_hir::def_id::LOCAL_CRATE;

use crate::{BevyCtxt, DEF_PATHS_FROM_LUA, DEF_PATHS_REFLECT, DEF_PATHS_TO_LUA};

/// Finds and caches relevant mlua traits, if they cannot be found throws an ICE
pub fn cache_traits(ctxt: &mut BevyCtxt<'_>) {
    let tcx = &ctxt.tcx;
    for trait_did in tcx.all_traits() {
        let def_path_str = tcx.def_path_str(trait_did);

        if DEF_PATHS_FROM_LUA.contains(&def_path_str.as_str()) {
            debug!("found from_lua trait def id: {trait_did:?}");
            ctxt.cached_traits.mlua_from_lua = Some(trait_did);
        } else if DEF_PATHS_TO_LUA.contains(&def_path_str.as_str()) {
            debug!("found to_lua trait def id: {trait_did:?}");
            ctxt.cached_traits.mlua_to_lua = Some(trait_did);
        } else if DEF_PATHS_REFLECT.contains(&def_path_str.as_str()) {
            debug!("found reflect trait def id: {trait_did:?}");
            ctxt.cached_traits.bevy_reflect_reflect = Some(trait_did);
        }
    }

    if !ctxt.cached_traits.has_all_mlua_traits() {
        panic!(
            "Could not find all mlua traits in crate: {}, did bootstrapping go wrong?",
            tcx.crate_name(LOCAL_CRATE)
        )
    }

    if !ctxt.cached_traits.has_all_bevy_traits() {
        panic!(
            "Could not find all reflect traits in crate: {}, did bootstrapping go wrong?",
            tcx.crate_name(LOCAL_CRATE)
        )
    }
}
