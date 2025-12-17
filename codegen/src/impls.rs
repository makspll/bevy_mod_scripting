use rustc_infer::{
    infer::TyCtxtInferExt,
    traits::{Obligation, ObligationCause, PredicateObligation},
};
use rustc_middle::ty::{Ty, TyCtxt, TypingEnv};
use rustc_trait_selection::traits::ObligationCtxt;

use crate::{BmsTypes, candidate::GenerationExclusionNote};

pub(crate) trait BmsTraitChecker<'tcx> {
    /// Checks if the given type implements 'FromScript', assuming a monomorphised type.
    fn implements_from_script(
        &self,
        self_type: Ty<'tcx>,
        bms_types: &BmsTypes,
        tcx: TyCtxt<'tcx>,
    ) -> Result<(), GenerationExclusionNote>;

    /// Checks if the given type implements 'IntoScript', assuming a monomorphised type.
    fn implements_into_script(
        &self,
        self_type: Ty<'tcx>,
        bms_types: &BmsTypes,
        tcx: TyCtxt<'tcx>,
    ) -> Result<(), GenerationExclusionNote>;
}

fn typing_env_for_impl<'tcx>(_self_type: Ty<'tcx>, _tcx: TyCtxt<'tcx>) -> TypingEnv<'tcx> {
    // if let Some(adt) = self_type.ty_adt_def() {
    //     TypingEnv::non_body_analysis(tcx, adt.did())
    // } else {
    TypingEnv::fully_monomorphized()
    // }
}

impl<'tcx> BmsTraitChecker<'tcx> for Ty<'tcx> {
    fn implements_from_script(
        &self,
        self_type: Ty<'tcx>,
        bms_types: &BmsTypes,
        tcx: TyCtxt<'tcx>,
    ) -> Result<(), GenerationExclusionNote> {
        let typing_env = typing_env_for_impl(self_type, tcx);
        check_trait_impl(tcx, typing_env, self_type, bms_types.from_script)
    }

    fn implements_into_script(
        &self,
        self_type: Ty<'tcx>,
        bms_types: &BmsTypes,
        tcx: TyCtxt<'tcx>,
    ) -> Result<(), GenerationExclusionNote> {
        let typing_env = typing_env_for_impl(self_type, tcx);
        check_trait_impl(tcx, typing_env, self_type, bms_types.into_script)
    }
}

fn check_trait_impl<'tcx>(
    tcx: TyCtxt<'tcx>,
    typing_env: TypingEnv<'tcx>,
    self_ty: Ty<'tcx>,
    trait_def_id: rustc_hir::def_id::DefId,
) -> Result<(), GenerationExclusionNote> {
    let param_env = typing_env.param_env;

    let infcx = tcx.infer_ctxt().build(typing_env.typing_mode);

    let trait_ref = rustc_middle::ty::TraitRef::new(tcx, trait_def_id, [self_ty]);

    let obligation: PredicateObligation<'tcx> =
        Obligation::new(tcx, ObligationCause::dummy(), param_env, trait_ref);

    let ocx = ObligationCtxt::new_with_diagnostics(&infcx);
    ocx.register_obligation(obligation);

    let errors = ocx.evaluate_obligations_error_on_ambiguity();

    if errors.is_empty() {
        return Ok(());
    }

    let msg = format!(
        "type `{self_ty}` does NOT implement required trait, errors: {}. param env: {param_env:?}",
        errors
            .into_iter()
            .map(|e| format!("{e:?}"))
            .collect::<Vec<_>>()
            .join(",")
    );

    Err(GenerationExclusionNote::Reason(msg))
}
