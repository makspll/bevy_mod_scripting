use log::debug;
use rustc_infer::infer::TyCtxtInferExt;
use rustc_trait_selection::infer::InferCtxtExt;

use crate::{Args, BevyCtxt};

/// Finds the traits implemented by each reflect type
pub(crate) fn find_trait_impls(ctxt: &mut BevyCtxt<'_>, _args: &Args) -> bool {
    let tcx = &ctxt.tcx;

    for (reflect_ty_did, type_ctxt) in ctxt.reflect_types.iter_mut() {
        let mut impls = Vec::default();

        for trait_def_did in ctxt.cached_traits.fn_source_traits.values() {
            // debug!("{}", ctxt.tcx.def_path_str(t));
            tcx.for_each_relevant_impl(
                *trait_def_did,
                tcx.type_of(reflect_ty_did).instantiate_identity(),
                |impl_did| {
                    debug!(
                        "Possible impl for trait: {:?} on type: {:?} found: {:?}",
                        tcx.def_path_str(*trait_def_did),
                        tcx.def_path_str(reflect_ty_did),
                        impl_did
                    );
                    let ty = tcx.type_of(reflect_ty_did).instantiate_identity();
                    let param_env = tcx.param_env(impl_did);
                    let applies = tcx
                        .infer_ctxt()
                        .build()
                        .type_implements_trait(*trait_def_did, [ty], param_env)
                        .must_apply_modulo_regions();
                    if applies {
                        debug!("Applies with: {param_env:?}, type: {ty}",);
                        impls.push(impl_did);
                    }
                },
            )
        }

        assert!(type_ctxt.trait_impls.is_none(), "trait impls already set!");
        type_ctxt.trait_impls = Some(impls);
    }
    true
}
