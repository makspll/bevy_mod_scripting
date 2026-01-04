use log::{debug, trace};
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_span::Symbol;

use crate::{
    Args, BevyCtxt, BevyTypes, BmsTypes, DEF_PATHS_BMS_FROM_SCRIPT, DEF_PATHS_BMS_INTO_SCRIPT,
    DEF_PATHS_BMS_MUT_WRAPPER, DEF_PATHS_BMS_REF_WRAPPER, DEF_PATHS_BMS_VAL_WRAPPER,
    DEF_PATHS_GET_TYPE_REGISTRATION, DEF_PATHS_REFLECT, STD_ONLY_TRAITS, STD_OR_CORE_TRAITS,
};
// tODO: verify there is only one Reflect trait impl
// fn dump_items(ctxt: &BevyCtxt) -> String {
//     let mut buffer = String::new();
//     buffer.push_str("traits:\n");
//     for t in ctxt.tcx.all_traits_including_private() {
//         buffer.push_str(&ctxt.tcx.def_path_str(t));
//         buffer.push_str(", ");
//     }
//     buffer.push_str("bms_bindings items:\n");
//     for item in ctxt
//         .path_finder
//         .crawled_items_in_crate(ctxt.cached_traits.bms_types.bindings_crate)
//     {
//         buffer.push_str(&ctxt.tcx.def_path_str(item));
//         buffer.push_str(", ");
//     }

//     buffer
// }

pub(crate) fn cache_bms_items(ctxt: &mut BevyCtxt<'_>) -> BmsTypes {
    let bindings_crate = *ctxt
        .tcx
        .crates(())
        .iter()
        .find(|c| ctxt.tcx.crate_name(**c).as_str() == "bevy_mod_scripting_bindings")
        .expect("bevy_mod_scripting_bindings crate not linked");
    let mut val_wrapper = None;
    let mut ref_wrapper = None;
    let mut mut_wrapper = None;
    let mut into_script = None;
    let mut from_script = None;
    ctxt.path_finder.ensure_crate_crawled(bindings_crate);
    for item in ctxt.path_finder.crawled_items_in_crate(bindings_crate) {
        let item_name = ctxt.tcx.def_path_str(item);
        let item_name = item_name.as_str();
        if DEF_PATHS_BMS_VAL_WRAPPER.contains(&item_name) {
            val_wrapper = Some(item)
        } else if DEF_PATHS_BMS_REF_WRAPPER.contains(&item_name) {
            ref_wrapper = Some(item)
        } else if DEF_PATHS_BMS_MUT_WRAPPER.contains(&item_name) {
            mut_wrapper = Some(item)
        } else if DEF_PATHS_BMS_FROM_SCRIPT.contains(&item_name) {
            from_script = Some(item)
        } else if DEF_PATHS_BMS_INTO_SCRIPT.contains(&item_name) {
            into_script = Some(item)
        }
    }

    BmsTypes {
        bindings_crate,
        into_script: into_script.expect("could not find IntoScript trait from bms linked"),
        from_script: from_script.expect("could not find FromScript trait from bms linked"),
        ref_wrapper: ref_wrapper.expect("could not find Ref<T> type from bms linked"),
        val_wrapper: val_wrapper.expect("could not find Val<T> type from bms linked"),
        mut_wrapper: mut_wrapper.expect("could not find Mut<T> type from bms linked"),
    }
}

pub(crate) fn cache_bevy_items(ctxt: &mut BevyCtxt<'_>) -> Result<BevyTypes, String> {
    let reflect_crates = ctxt
        .tcx
        .crates(())
        .iter()
        .chain(std::iter::once(&LOCAL_CRATE))
        .filter(|c| ctxt.tcx.crate_name(**c).as_str() == "bevy_reflect")
        .collect::<Vec<_>>();

    // if reflect_crates.len() != 1 {
    //     return Err(format!(
    //         "Either found none or multiple bevy_reflect crates: {:?}",
    //         reflect_crates
    //             .iter()
    //             .map(|c| ctxt.tcx.def_path_str(c.as_def_id()))
    //             .collect::<Vec<_>>()
    //             .join(",")
    //     ));
    // }
    let reflect_crate = **reflect_crates.last().unwrap();

    ctxt.path_finder.ensure_crate_crawled(reflect_crate);
    let mut reflect = None;
    let mut get_type_registration = None;
    for item in ctxt.path_finder.crawled_items_in_crate(reflect_crate) {
        let item_name = ctxt.tcx.def_path_str(item);
        let item_name = item_name.as_str();
        if DEF_PATHS_REFLECT.contains(&item_name) {
            reflect = Some(item)
        } else if DEF_PATHS_GET_TYPE_REGISTRATION.contains(&item_name) {
            get_type_registration = Some(item)
        }
    }

    Ok(BevyTypes {
        reflect_crate,
        reflect: reflect.ok_or("could not find Reflect from bevy_reflect linked")?,
        get_type_registration: get_type_registration
            .ok_or("could not find GetTypeRegistration from bevy_reflect linked")?,
    })
}

/// Finds and caches relevant traits, if they cannot be found throws an ICE
pub(crate) fn cache_items(ctxt: &mut BevyCtxt<'_>, _args: &Args) -> bool {
    ctxt.cached_traits.bms_types = cache_bms_items(ctxt);
    ctxt.cached_traits.bevy_types = match cache_bevy_items(ctxt) {
        Ok(t) => t,
        Err(err) => {
            log::warn!(
                "could not cache bevy types in crate {}. {err}. Will not generate bindings.",
                ctxt.tcx.crate_name(LOCAL_CRATE)
            );
            return false;
        }
    };

    let tcx = &ctxt.tcx;

    for trait_did in tcx.all_traits_including_private() {
        let def_path_str = tcx.def_path_str(trait_did);
        if STD_ONLY_TRAITS.contains(&def_path_str.as_str()) {
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
        }
    }

    let missing_std_traits = ctxt.cached_traits.missing_std_traits();
    if !missing_std_traits.is_empty() {
        debug!(
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
