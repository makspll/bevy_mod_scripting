use itertools::Itertools;
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_middle::ty::AdtDef;

use crate::{
    Args, BevyCtxt,
    candidate::{Annotated, GenerationCandidate, GenerationExclusionNote},
};

/// Finds all reflect types which we can wrap in the crate as well as sorts the final list.
pub(crate) fn find_reflect_types(ctxt: &mut BevyCtxt<'_>, args: &Args) -> bool {
    let tcx = &ctxt.tcx;
    let ignored_types = match &args.cmd {
        crate::Command::Generate { ignored_types, .. } => ignored_types,
        crate::Command::ListTypes => &vec![],
        _ => return true,
    };

    let reflect_ref = ctxt.cached_traits.bevy_types.reflect;
    let reflect_impls = tcx.trait_impls_of(reflect_ref);
    // let sess = ctxt.tcx.sess;
    // let source_map = sess.source_map();
    // log::info!(
    //     "found impls: {:?}",
    //     reflect_impls.non_blanket_impls().iter().map(|impl_| sess
    //         .source_map()
    //         .span_to_location_info(ctxt.tcx.def_span(impl_.)))
    // );

    // blanket impls are implementations on generics directly, i.e. `impl From<T> for T`
    // non blanket impls may also contain generics but those will be contained within another type i.e. `impl Default for Vec<T>`
    // ignore anything with a generic, so blanket_impls are out for now
    // we also make sure to only work over types and impls directly in the local crate
    // log::info!(
    //     "reflect_crate: {:?}, {:?}",
    //     ctxt.cached_traits.bevy_types.reflect_crate,
    //     ctxt.tcx
    //         .def_path(ctxt.cached_traits.bevy_types.reflect_crate.as_def_id())
    // );
    // log::info!(
    //     "local_crate: {:?}, {:?}",
    //     ctxt.cached_traits.bevy_types.reflect_crate,
    //     ctxt.tcx.def_path(LOCAL_CRATE.as_def_id())
    // );
    let (reflect_adts_did, excluded_candidates): (Vec<_>, Vec<_>) =
        reflect_impls
            .non_blanket_impls()
            .iter()
            .flat_map(|(self_ty, impl_dids)| impl_dids.iter().zip(std::iter::repeat(self_ty)))
            // .inspect(|(impl_did, _)| {
            //     log::info!(
            //         "in crate: {LOCAL_CRATE:?}, found impl: \n{impl_did:?}\n{:?}\nis_local: {}\n",
            //         source_map.span_to_diagnostic_string(tcx.def_span(*impl_did)),
            //         impl_did.is_local()
            //     )
            // })
            // non local impls are not relevant, we only care about what a crate implements, we don't want to include in the excluded set either, as these aren't even considered
            .filter(|(impl_did, _)| impl_did.is_local())
            .map(|(impl_did, self_ty)| {
                let mut early_candidate = GenerationCandidate::<Option<AdtDef>>::default();

                let did = match self_ty.def() {
                    Some(ty) => ty,
                    None => {
                        return Err(early_candidate.with_note(GenerationExclusionNote::Reason(
                            format!("impl block {impl_did:?}, has no self type"),
                        )));
                    }
                };

                early_candidate.did = Some(did);

                let generics = tcx.generics_of(*impl_did);
                if generics.count() > 0 {
                    return Err(early_candidate.with_note(GenerationExclusionNote::Reason(
                        format!("impl block {impl_did:?}, has generics"),
                    )));
                }

                // only non parametrized simple types are allowed, i.e. "MyStruct" is allowed but "MyStruct<T>" isn't
                let short_form = format!(
                    "{}::{}",
                    ctxt.tcx.crate_name(LOCAL_CRATE),
                    ctxt.tcx.item_name(did)
                );

                if ignored_types.contains(&short_form)
                    || ignored_types.contains(&tcx.def_path_str(did))
                {
                    return Err(early_candidate.with_note(GenerationExclusionNote::Reason(
                        format!("type {short_form} explicitly excluded by user"),
                    )));
                };

                let adt_generics = tcx.generics_of(did);

                if adt_generics.count() > 0 {
                    return Err(early_candidate.with_note(GenerationExclusionNote::Reason(
                        format!("type has generics: {adt_generics:?}"),
                    )));
                }

                let visibility = tcx.visibility(did);
                if !visibility.is_public() {
                    return Err(early_candidate.with_note(GenerationExclusionNote::Reason(
                        format!("type has non-public visibility: {visibility:?}"),
                    )));
                }

                if ctxt
                    .path_finder
                    .find_import_paths_no_fallback(did)
                    .is_none()
                    && did.is_local()
                {
                    return Err(early_candidate.with_note(GenerationExclusionNote::Reason(
                        "type is local and has no public import paths".to_string(),
                    )));
                }

                let adt = ctxt.tcx.adt_def(did);

                Ok(early_candidate.promote(adt))
            })
            .partition_result();
    ctxt.reflect_types
        .extend(reflect_adts_did.into_iter().map(|a| (a.def.did(), a)));
    ctxt.excluded_reflect_types.extend(excluded_candidates);

    ctxt.reflect_types
        .sort_by_cached_key(|did, _| tcx.item_name(*did));

    log::debug!("Found: {} types", ctxt.reflect_types.len());
    log::debug!("Excluded: {} types", ctxt.excluded_reflect_types.len());

    true
}
