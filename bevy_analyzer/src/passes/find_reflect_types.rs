use log::debug;
use rustc_hir::def_id::LOCAL_CRATE;

use crate::BevyCtxt;

pub fn find_reflect_types(ctxt: &mut BevyCtxt<'_>) {
    let tcx = &ctxt.tcx;

    for (trait_def_id, impls) in tcx.all_local_trait_impls(()) {
        // we want to find the canonical `Reflect` trait's implemenations across crates, so let's check all impls and choose those
        // whose def_path is equal to what we know the Reflect trait's

        let def_path_str = tcx.def_path_str(trait_def_id);

        if def_path_str != "bevy_reflect::Reflect" {
            continue;
        }

        debug!(
            "Found Reflect impls in crate: {}",
            tcx.crate_name(LOCAL_CRATE)
        );

        let reflect_trait_impls = tcx.trait_impls_of(trait_def_id);

        for (ty, items) in reflect_trait_impls.non_blanket_impls() {
            debug!(
                "On type: {}",
                ty.def()
                    .map(|id| tcx.item_name(id).to_ident_string())
                    .unwrap_or(format!("{ty:?}*"))
            );
        }

        // if trait_def_path.to
        // let trait_name = tcx.item_name(*trait_def_id);
        // if trait_name.to_ident_string() == "Reflect" {
        //     debug!("Reflect found with defid: {}", trait_def_id.index)
        // }
    }
}
