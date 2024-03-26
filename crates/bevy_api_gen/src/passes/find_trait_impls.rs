use std::collections::HashMap;

use log::trace;
use rustc_hir::def_id::DefId;
use rustc_infer::infer::TyCtxtInferExt;
use rustc_trait_selection::infer::InferCtxtExt;

use crate::{Args, BevyCtxt};

/// Finds the traits implemented by each reflect type
pub(crate) fn find_trait_impls(ctxt: &mut BevyCtxt<'_>, _args: &Args) -> bool {
    let tcx = &ctxt.tcx;

    // first filter out those without GetTypeRegistration traits
    ctxt.reflect_types.retain(|reflect_ty_did, _| {
        type_impl_of_trait(
            tcx,
            ctxt.cached_traits
                .bevy_reflect_get_type_registration
                .unwrap(),
            reflect_ty_did,
        )
        .is_some()
    });

    for (reflect_ty_did, type_ctxt) in ctxt.reflect_types.iter_mut() {
        let mut impls = Vec::default();

        for trait_did in ctxt.cached_traits.fn_source_traits.values() {
            let impl_ = type_impl_of_trait(tcx, *trait_did, reflect_ty_did);
            if let Some(impl_did) = impl_ {
                impls.push((*trait_did, impl_did));
            }
        }

        assert!(type_ctxt.trait_impls.is_none(), "trait impls already set!");
        type_ctxt.trait_impls = Some(HashMap::from_iter(impls));
    }
    true
}

fn type_impl_of_trait(
    tcx: &rustc_middle::ty::TyCtxt<'_>,
    trait_did: DefId,
    reflect_ty_did: &rustc_hir::def_id::DefId,
) -> Option<DefId> {
    let mut out = None;
    tcx.for_each_relevant_impl(
        trait_did,
        tcx.type_of(reflect_ty_did).instantiate_identity(),
        |impl_did| {
            trace!(
                "Possible impl for trait: {:?} on type: {:?} found: {:?}",
                tcx.def_path_str(trait_did),
                tcx.def_path_str(reflect_ty_did),
                impl_did
            );
            let ty = tcx.type_of(reflect_ty_did).instantiate_identity();
            let param_env = tcx.param_env(impl_did);
            let applies = tcx
                .infer_ctxt()
                .build()
                .type_implements_trait(trait_did, [ty], param_env)
                .must_apply_modulo_regions();
            if applies {
                trace!("Applies with: {param_env:?}, type: {ty}",);
                out = Some(impl_did);
            }
        },
    );
    out
}
