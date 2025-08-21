use log::{trace, warn};
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_middle::ty::TyCtxt;
use rustc_span::Symbol;

use crate::{
    Args, BevyCtxt, DEF_PATHS_BMS_FROM_SCRIPT, DEF_PATHS_BMS_INTO_SCRIPT,
    DEF_PATHS_GET_TYPE_REGISTRATION, DEF_PATHS_REFLECT, STD_ONLY_TRAITS, STD_OR_CORE_TRAITS,
};

fn dump_traits(tcx: &TyCtxt) -> String {
    let mut buffer = String::new();
    for t in tcx.all_traits() {
        buffer.push_str(&tcx.def_path_str(t));
        buffer.push_str(", ");
    }
    buffer
}

/// Finds and caches relevant traits, if they cannot be found throws an ICE
pub(crate) fn cache_traits(ctxt: &mut BevyCtxt<'_>, _args: &Args) -> bool {
    let tcx = &ctxt.tcx;

    for trait_did in tcx.all_traits() {
        let def_path_str = tcx.def_path_str(trait_did);
        if DEF_PATHS_REFLECT.contains(&def_path_str.as_str()) {
            trace!("found Reflect trait def id: {trait_did:?}");
            ctxt.cached_traits.bevy_reflect_reflect = Some(trait_did);
        } else if DEF_PATHS_GET_TYPE_REGISTRATION.contains(&def_path_str.as_str()) {
            trace!("found GetTypeRegistration trait def id: {trait_did:?}");
            ctxt.cached_traits.bevy_reflect_get_type_registration = Some(trait_did);
        } else if STD_ONLY_TRAITS.contains(&def_path_str.as_str()) {
            trace!("found std trait def id: {trait_did:?}");
            ctxt.cached_traits
                .std_only_traits
                .insert(def_path_str.to_string(), trait_did);
        } else if let Some(full_name) = STD_OR_CORE_TRAITS.iter().find(|name| {
            (Some(**name) == def_path_str.strip_prefix("core::"))
                || (Some(**name) == def_path_str.strip_prefix("std::"))
        }) {
            trace!("found core trait def id: {trait_did:?}");
            ctxt.cached_traits
                .std_or_core_traits
                .insert(full_name.to_string(), trait_did);
        } else if DEF_PATHS_BMS_INTO_SCRIPT.contains(&def_path_str.as_str()) {
            trace!("found IntoScript trait def id: {trait_did:?}");
            ctxt.cached_traits.bms_into_script = Some(trait_did);
        } else if DEF_PATHS_BMS_FROM_SCRIPT.contains(&def_path_str.as_str()) {
            trace!("found FromScript trait def id: {trait_did:?}");
            ctxt.cached_traits.bms_from_script = Some(trait_did);
        }
    }

    let missing_bevy_traits = ctxt.cached_traits.missing_bevy_traits();
    if !missing_bevy_traits.is_empty() {
        panic!(
            "Could not find traits: [{}] in crate: {}, did bootstrapping go wrong? Available traits: {}",
            missing_bevy_traits.join(", "),
            tcx.crate_name(LOCAL_CRATE),
            dump_traits(tcx)
        );
    }

    let missing_bms_traits = ctxt.cached_traits.missing_bms_traits();
    if !missing_bms_traits.is_empty() {
        panic!(
            "Could not find traits: [{}] in crate: {}, did bootstrapping go wrong? Available traits: {}",
            missing_bms_traits.join(", "),
            tcx.crate_name(LOCAL_CRATE),
            dump_traits(tcx)
        );
    }

    let missing_std_traits = ctxt.cached_traits.missing_std_traits();
    if !missing_std_traits.is_empty() {
        warn!(
            "Could not find std traits: [{}] in crate: {}, this might lead to missing methods in the generated API.",
            missing_std_traits.join(", "),
            tcx.crate_name(LOCAL_CRATE),
        );
    }

    // some crates specifically do not have std in scope via `#![no_std]` which means we do not care about these traits
    let has_std = tcx
        .get_attrs_by_path(LOCAL_CRATE.as_def_id(), &[Symbol::intern("no_std")])
        .map(|_| ())
        .next()
        .is_none();

    log::trace!("has_std: {has_std}");

    // if has_std && !ctxt.cached_traits.has_all_std_source_traits() {
    //     log::debug!(
    //         "all traits: {}",
    //         tcx.all_traits()
    //             .map(|t| tcx.def_path_str(t).to_string())
    //             .collect::<Vec<_>>()
    //             .join(", ")
    //     );

    //     panic!(
    //         "Could not find traits: [{}] in crate: {}, did bootstrapping go wrong?",
    //         ctxt.cached_traits.missing_std_source_traits().join(", "),
    //         tcx.crate_name(LOCAL_CRATE)
    //     )
    // }

    true
}
