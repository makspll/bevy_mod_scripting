use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_infer::infer::{TyCtxtInferExt};
use rustc_middle::ty::{EarlyBinder, FnSig, PolyFnSig, Ty, TyCtxt, TypingEnv};
use rustc_trait_selection::infer::InferCtxtExt;

pub fn typing_env_function_arg_in_impl<'tcx>(tcx: TyCtxt<'tcx>, sig: EarlyBinder<PolyFnSig<'tcx>>, impl_block: DefId) -> (TypingEnv<'tcx>, FnSig<'tcx>){
    let env = TypingEnv::non_body_analysis(tcx, impl_block);
    (env, sig.skip_binder().skip_binder())
}

pub fn type_implements_trait<'tcx>(tcx: TyCtxt<'tcx>, ctxt: TypingEnv<'tcx>, self_ty: Ty<'tcx>, trait_did: DefId) -> Result<(), String>{
    let (infr_ctxt, param_env) = tcx.infer_ctxt()
        .build_with_typing_env(ctxt);


    let impls = infr_ctxt
        .type_implements_trait(trait_did, [self_ty], param_env).must_apply_modulo_regions();

    if impls {
        Ok(())
    } else {
        Err("error".to_owned())
    }
}

/// Returns all impls in the crate satisfying a condition, and their trait DefId
pub fn query_crate_trait_impls<'tcx>(tcx: TyCtxt<'tcx>, q: impl Fn(TyCtxt<'tcx>, &'tcx DefId, &'tcx LocalDefId) -> bool ) -> impl Iterator<Item=(&'tcx DefId, &'tcx LocalDefId)> {
    tcx.all_local_trait_impls(()).iter().flat_map(|(a,b)| b.iter().map(move |b| (a,b))).filter(move |(a, b)| q(tcx, a, b))
}

/// Returns all inherent type impls from the crate, and their type LocalDefId
pub fn query_crate_non_trait_impls<'tcx>(tcx: TyCtxt<'tcx>, q: impl Fn(TyCtxt<'tcx>, &'tcx LocalDefId, &'tcx DefId) -> bool) -> impl Iterator<Item=(&'tcx LocalDefId, &'tcx DefId)> {
    let iter = tcx.crate_inherent_impls(()).0;
    iter.inherent_impls.iter().flat_map(move |(t,impls)| impls.iter().map(move |i| (t,i))).filter(move |(l, i)| q(tcx, l, i))
}