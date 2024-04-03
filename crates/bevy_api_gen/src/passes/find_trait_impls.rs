use std::collections::HashMap;

use log::trace;
use rustc_hir::def_id::DefId;
use rustc_infer::{
    infer::{BoundRegionConversionTime, DefineOpaqueTypes, InferCtxt, TyCtxtInferExt},
    traits::{Obligation, ObligationCause, PolyTraitObligation},
};
use rustc_middle::ty::{Binder, ClauseKind, ImplPolarity, PolyTraitPredicate, TraitPredicate, Ty};
use rustc_span::DUMMY_SP;
use rustc_trait_selection::{
    infer::InferCtxtExt,
    traits::{elaborate, ObligationCtxt},
};

use crate::{Args, BevyCtxt};

/// Finds the traits implemented by each reflect type
pub(crate) fn find_trait_impls(ctxt: &mut BevyCtxt<'_>, _args: &Args) -> bool {
    let tcx = &ctxt.tcx;

    ctxt.reflect_types.retain(|reflect_ty_did, _| {
        // first filter out those without GetTypeRegistration traits
        // TODO: this is working partially,
        // some types even though they clearly implement GetTypeRegistration are not being picked up
        // the impl is just straight up missing
        // let retaining = type_impl_of_trait(
        //     tcx,
        //     ctxt.cached_traits
        //         .bevy_reflect_get_type_registration
        //         .unwrap(),
        //     reflect_ty_did,
        // )
        // .is_some();

        // filter out types which have impls both ways
        let retaining = type_impl_of_trait(
            tcx,
            ctxt.cached_traits.mlua_from_lua_multi.unwrap(),
            reflect_ty_did,
        )
        .is_empty()
            || type_impl_of_trait(
                tcx,
                ctxt.cached_traits.mlua_into_lua_multi.unwrap(),
                reflect_ty_did,
            )
            .is_empty();

        if !retaining {
            trace!(
                "Type: `{:?}` implements both from and into lua, removing from reflect types",
                tcx.def_path_str(*reflect_ty_did)
            );
        }
        retaining
    });

    log::trace!(
        "Looking for impls of the traits: [{}]",
        ctxt.cached_traits
            .std_source_traits
            .values()
            .map(|d| tcx.def_path_str(*d))
            .collect::<Vec<_>>()
            .join(", ")
    );

    for (reflect_ty_did, type_ctxt) in ctxt.reflect_types.iter_mut() {
        let mut impls = Vec::default();

        for trait_did in ctxt.cached_traits.std_source_traits.values() {
            let matching_impls = type_impl_of_trait(tcx, *trait_did, reflect_ty_did);
            if !matching_impls.is_empty() {
                impls.push((*trait_did, matching_impls));
            }
        }

        assert!(type_ctxt.trait_impls.is_none(), "trait impls already set!");
        type_ctxt.trait_impls = Some(HashMap::from_iter(impls));
    }
    true
}

/// Checks if a type implements a trait, returns all implementations with the generic args required
fn type_impl_of_trait(
    tcx: &rustc_middle::ty::TyCtxt<'_>,
    trait_did: DefId,
    reflect_ty_did: &rustc_hir::def_id::DefId,
) -> Vec<DefId> {
    log::trace!(
        "Finding impl for trait: {:?} on type: {:?}",
        tcx.def_path_str(trait_did),
        tcx.def_path_str(*reflect_ty_did)
    );
    let mut out = Vec::default();

    tcx.for_each_relevant_impl(
        trait_did,
        tcx.type_of(reflect_ty_did).instantiate_identity(),
        |impl_did| {
            trace!(
                "Possible impl for trait: {:?} on type: {:?} found: {:?}",
                tcx.def_path_str(trait_did),
                tcx.def_path_str(reflect_ty_did),
                impl_did,
            );
            //TODO: false negatives coming from this inference

            let ty = tcx.type_of(reflect_ty_did).instantiate_identity();
            let infcx = tcx.infer_ctxt().build();
            let result = impl_matches(&infcx, ty, trait_did, impl_did);
            log::trace!("Result: {:#?}", result);
            if result {
                trace!(
                    "Type: `{}` implements trait: `{}`",
                    ty,
                    tcx.item_name(trait_did)
                );
                out.push(impl_did)
            } else {
                trace!(
                    "Type: `{}` does not implement trait: `{}`",
                    ty,
                    tcx.item_name(trait_did)
                );
            }
        },
    );
    out
}

/// this is the same logic as in rustc_trait_selection::...::recompute_applicable_impls, i.e. we need to go through all
/// impls that may match and perform full on matching on them
/// If this goes out of date with rustc, we can just copy the function here
fn impl_matches<'tcx>(
    infcx: &InferCtxt<'tcx>,
    ty: Ty<'tcx>,
    trait_def_id: DefId,
    impl_def_id: DefId,
) -> bool {
    let tcx = infcx.tcx;

    let impl_may_apply = |impl_def_id| {
        let ocx = ObligationCtxt::new(infcx);
        let param_env = tcx.param_env_reveal_all_normalized(impl_def_id);
        let impl_args = infcx.fresh_args_for_item(DUMMY_SP, impl_def_id);
        let impl_trait_ref = tcx
            .impl_trait_ref(impl_def_id)
            .unwrap()
            .instantiate(tcx, impl_args);
        let impl_trait_ref = ocx.normalize(&ObligationCause::dummy(), param_env, impl_trait_ref);
        let impl_trait_ref_ty = impl_trait_ref.self_ty();
        if let Err(_) = ocx.eq(&ObligationCause::dummy(), param_env, impl_trait_ref_ty, ty) {
            return false;
        }

        let impl_predicates = tcx.predicates_of(impl_def_id).instantiate(tcx, impl_args);
        ocx.register_obligations(impl_predicates.predicates.iter().map(|&predicate| {
            Obligation::new(tcx, ObligationCause::dummy(), param_env, predicate)
        }));

        ocx.select_where_possible().is_empty()
    };

    infcx.probe(|_| impl_may_apply(impl_def_id))
}
