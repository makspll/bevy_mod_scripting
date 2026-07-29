use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_infer::{infer::TyCtxtInferExt, traits::ObligationCause};
use rustc_middle::ty::{EarlyBinder, FnSig, ParamEnv, PolyFnSig, Ty, TyCtxt, TypingEnv, TypingMode, Unnormalized};
use rustc_trait_selection::{error_reporting::InferCtxtErrorExt, traits::ObligationCtxt};

use crate::emitter::{CaptureState};

pub fn typing_env_function_arg_in_impl<'tcx>(  
    tcx: TyCtxt<'tcx>,  
    sig: EarlyBinder<'tcx, PolyFnSig<'tcx>>,  
    function: DefId,  
    impl_block: DefId,  
) -> (TypingEnv<'tcx>, FnSig<'tcx>) {  
    let impl_predicates = tcx  
        .predicates_of(impl_block)  
        .instantiate_identity(tcx)  
        .predicates  
        .into_iter()  
        .map(Unnormalized::skip_norm_wip);  
    let func_predicates = tcx  
        .predicates_of(function)  
        .instantiate_own_identity()  
        .map(|(a, _)| Unnormalized::skip_norm_wip(a));  
    let all_predicates = impl_predicates.chain(func_predicates);  
    let param_env = ParamEnv::new(tcx.mk_clauses_from_iter(all_predicates));  
    let env = TypingEnv::new(param_env, TypingMode::non_body_analysis());  
  
    // Discharge the EarlyBinder identically, then liberate the late-bound  
    // lifetimes on the fn sig using `function`'s DefId so no escaping  
    // bound vars remain in the extracted `FnSig`.  
    let fn_sig = sig.instantiate_identity();  
    let fn_sig = tcx.liberate_late_bound_regions(function, fn_sig.skip_norm_wip());  
  
    (env, fn_sig)  
}  

pub fn type_implements_trait<'tcx>(tcx: TyCtxt<'tcx>, ctxt: TypingEnv<'tcx>, self_ty: Ty<'tcx>, trait_did: DefId) -> Result<(), String>{

    let (_, messages, has_errors) = CaptureState::capture::<()>(tcx, || {  

    {
        let (infr_ctxt, param_env) = tcx.infer_ctxt()
            .build_with_typing_env(ctxt);

        let ocx = ObligationCtxt::new_with_diagnostics(&infr_ctxt);

        ocx.register_bound(
            ObligationCause::dummy(),
            param_env,
            self_ty,
            trait_did,
        );

        let errors = ocx.evaluate_obligations_error_on_ambiguity();

        let errcx = infr_ctxt.err_ctxt();

        // errcx.dcx().set_emitter(Box::new(BufferEmitter::new(shared_buffer.clone())));
        errcx.report_fulfillment_errors(errors);
    }
    });

    if has_errors  {
        Err(messages)
    } else {
        Ok(())
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