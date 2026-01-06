use itertools::Itertools;
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_middle::ty::AdtDef;

use crate::{
    Args, BevyCtxt, DEF_PATHS_REFLECT,
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

    for trait_did in tcx.all_local_trait_impls(()).keys() {
        // we want to find the canonical `Reflect` trait's implemenations across crates, so let's check all impls and choose those
        // whose def_path is equal to what we know the Reflect trait's is

        let def_path_str = tcx.def_path_str(trait_did);

        if !DEF_PATHS_REFLECT.contains(&def_path_str.as_str()) {
            continue;
        }

        // this returns non-local impls as well
        let reflect_trait_impls = tcx.trait_impls_of(trait_did);

        // blanket impls are implementations on generics directly, i.e. `impl From<T> for T`
        // non blanket impls may also contain generics but those will be contained within another type i.e. `impl Default for Vec<T>`
        // ignore anything with a generic, so blanket_impls are out for now
        // we also make sure to only work over types and impls directly in the local crate
        let (reflect_adts_did, excluded_candidates): (Vec<_>, Vec<_>) = reflect_trait_impls
            .non_blanket_impls()
            .iter()
            .flat_map(|(self_ty, impl_dids)| impl_dids.iter().zip(std::iter::repeat(self_ty)))
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

                if !impl_did.is_local() {
                    return Err(early_candidate.with_note(GenerationExclusionNote::Reason(
                        format!("impl block {impl_did:?}, is not local"),
                    )));
                }

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
    }

    ctxt.reflect_types
        .sort_by_cached_key(|did, _| tcx.item_name(*did));

    log::info!("Found: {} types", ctxt.reflect_types.len());
    log::info!("Excluded: {} types", ctxt.excluded_reflect_types.len());

    true
}
