use log::trace;
use rustc_hir::def_id::LOCAL_CRATE;

use crate::{
    Args, BevyCtxt, DEF_PATHS_FROM_LUA, DEF_PATHS_REFLECT, DEF_PATHS_TO_LUA, FN_SOURCE_TRAITS,
};

/// Finds and caches relevant traits, if they cannot be found throws an ICE
pub(crate) fn cache_traits(ctxt: &mut BevyCtxt<'_>, _args: &Args) -> bool {
    let tcx = &ctxt.tcx;
    for trait_did in tcx.all_traits() {
        let def_path_str = tcx.def_path_str(trait_did);

        if DEF_PATHS_FROM_LUA.contains(&def_path_str.as_str()) {
            trace!("found FromLuaMulti trait def id: {trait_did:?}");
            ctxt.cached_traits.mlua_from_lua_multi = Some(trait_did);
        } else if DEF_PATHS_TO_LUA.contains(&def_path_str.as_str()) {
            trace!("found ToLuaMulti trait def id: {trait_did:?}");
            ctxt.cached_traits.mlua_to_lua_multi = Some(trait_did);
        } else if DEF_PATHS_REFLECT.contains(&def_path_str.as_str()) {
            trace!("found Reflect trait def id: {trait_did:?}");
            ctxt.cached_traits.bevy_reflect_reflect = Some(trait_did);
        } else if FN_SOURCE_TRAITS.contains(&def_path_str.as_str()) {
            trace!("found misc trait def id: {trait_did:?}");
            ctxt.cached_traits
                .fn_source_traits
                .insert(def_path_str.to_string(), trait_did);
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

    if !ctxt.cached_traits.has_all_fn_source_traits() {
        panic!(
            "Could not find all fn source traits in crate: {}, did bootstrapping go wrong?",
            tcx.crate_name(LOCAL_CRATE)
        )
    }

    true
}
