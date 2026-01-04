use std::io::Write;

use indexmap::IndexMap;
use itertools::Itertools;
use log::trace;
use rustc_hir::{
    StableSince,
    def_id::{DefId, LOCAL_CRATE},
};
use rustc_infer::{infer::TyCtxtInferExt, traits::{Obligation, ObligationCause}};
use rustc_middle::ty::{self, AdtDef, AssocKind, FnSig, GenericArgs, Ty, TyCtxt, TyKind, TypingEnv};
use rustc_span::Symbol;
use rustc_trait_selection::{infer::InferCtxtExt, traits::ObligationCtxt};

use crate::{
    candidate::{
        Annotated, AnnotationContextCollector, FieldCandidate, FunctionArgCandidate,
        FunctionCandidate, FunctionCandidateKind, GenerationCandidate, GenerationExclusionNote,
        VariantCandidate,
    }, impls::BmsTraitChecker, ty_convert::{wrap_non_primtiives}, Args, BevyCtxt, CachedItems, MetaLoader, ReflectionStrategy
};

/// Finds all methods and fields which can be wrapped on a proxy, stores them in sorted order.
pub(crate) fn find_methods_and_fields(ctxt: &mut BevyCtxt<'_>, args: &Args) -> bool {
    // we need to find all the methods and fields for which we want to generate lua bindings
    // we have to filter some out
    // go through all impls on the types (traits and non-traits) and pick signatures we're happy with

    // borrow checker fucky wucky
    // let mut reflect_types = std::mem::take(&mut ctxt.reflect_types);
    let all_def_ids = ctxt.reflect_types.keys().cloned().collect::<Vec<_>>();
    for def_id in all_def_ids {
        let (variants, excluded_variants) = generate_variants(ctxt, def_id);

        // filter the list of all methods and select candidates applicable to proxy generation
        let mut all_impls = {
            let trait_impls = &ctxt.reflect_types[&def_id].trait_impls;

            ctxt.tcx
                .inherent_impls(def_id)
                .iter()
                .chain(trait_impls.iter().flat_map(|(_, impl_did)| impl_did))
                .cloned()
                .collect::<Vec<_>>()
        };

        // sort them to avoid unnecessary diffs, we can use hashes here as they are forever stable (touch wood)
        all_impls.sort_by_cached_key(|a| ctxt.tcx.def_path_hash(*a));

        for impl_did in all_impls {
            let (functions, excluded_functions) = generate_functions(ctxt, args, def_id, impl_did);

            let candidate = &mut ctxt.reflect_types[&def_id];
            candidate.functions.extend(functions);
            candidate.excluded_functions.extend(excluded_functions);
        }
        let candidate = &mut ctxt.reflect_types[&def_id];
        candidate.variants.extend(variants);
        candidate.excluded_variants.extend(excluded_variants);
    }

    if args.cmd.is_list_types() {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        writeln!(
            handle,
            "included generation candidates in crate: {}",
            ctxt.tcx.crate_name(LOCAL_CRATE)
        )
        .unwrap();
        for (did, candidate) in ctxt.reflect_types.iter() {
            writeln!(handle, "{:?}", ctxt.tcx.def_path_str(did)).unwrap();
            writeln!(handle, "Exclusions: ").unwrap();
            let mut annotator = AnnotationContextCollector::new(ctxt.tcx);
            annotator.annotate(candidate);
            let built = annotator.build();
            for (ctxt, annotation) in built {
                writeln!(handle, "{ctxt} : {annotation}").unwrap();
            }
        }
        writeln!(
            handle,
            "excluded generation candidates in crate: {}",
            ctxt.tcx.crate_name(LOCAL_CRATE)
        )
        .unwrap();
        for candidate in ctxt.excluded_reflect_types.iter() {
            if let Some(did) = candidate.did {
                writeln!(handle, "{:?}", ctxt.tcx.def_path_str(did)).unwrap();
            }
            writeln!(handle, "Exclusions: ").unwrap();
            let mut annotator = AnnotationContextCollector::new(ctxt.tcx);
            annotator.annotate(candidate);
            let built = annotator.build();
            for (ctxt, annotation) in built {
                writeln!(handle, "{ctxt} : {annotation}").unwrap();
            }
        }
        return false;
    }

    true
}

fn generate_functions<'tcx>(
    ctxt: &mut BevyCtxt<'tcx>,
    args: &Args,
    def_id: DefId,
    impl_did: DefId,
) -> (Vec<FunctionCandidate<'tcx>>, Vec<FunctionCandidate<'tcx>>) {
    let (functions, excluded_functions): (Vec<_>, Vec<_>) = ctxt
        .tcx
        .associated_items(impl_did)
        .in_definition_order()
        .filter_map(|assoc_item| {
            if !matches!(assoc_item.kind, AssocKind::Fn { .. }) {
                return None;
            }

            let (fn_name, has_self) = match assoc_item.kind {
                AssocKind::Fn { has_self, name } => (name, has_self),
                _ => return None,
            };

            let trait_did = ctxt
                .tcx
                .impl_opt_trait_ref(impl_did)
                .map(|tr| tr.skip_binder().def_id);
            let param_env = TypingEnv::non_body_analysis(ctxt.tcx, def_id);
            let fn_did = assoc_item.def_id;
            let fn_sig = ctxt.tcx.fn_sig(fn_did).instantiate_identity();
            let sig: FnSig = ctxt
                .tcx
                .normalize_erasing_late_bound_regions(param_env, fn_sig);
            Some(FunctionCandidate {
                fn_name,
                did: fn_did,
                visibility: assoc_item.visibility(ctxt.tcx),
                sig,
                has_self,
                is_unsafe: sig.safety.is_unsafe(),
                kind: trait_did
                    .map(|trait_did| FunctionCandidateKind::TraitImplMethod {trait_did,impl_did})
                    .unwrap_or(FunctionCandidateKind::Method { impl_did }),
                notes: vec![],
                arguments: vec![],
                ret: FunctionArgCandidate::new(String::from("Return value")),
            })
        })
        .map(|mut fn_candidate| {

            if !fn_candidate.visibility.is_public() {
                return Err(fn_candidate.with_note(GenerationExclusionNote::Reason(String::from("function is not public"))))
            }

            let function_generics =
                get_function_generics(ctxt.tcx, fn_candidate.did, fn_candidate.kind.impl_did());

            if !function_generics.is_empty() {
                return Err(fn_candidate.with_note(GenerationExclusionNote::Reason(format!(
                    "function has generics: {function_generics:?}"
                ))));
            }

            let stability = ctxt.tcx.lookup_stability(fn_candidate.did);
            let is_stable_for_target = stability
                .map(|stability| match stability.stable_since() {
                    Some(StableSince::Version(rustc_version)) => {
                        !args.rustc_version_is_greater_than_mrsv_target(rustc_version)
                    }
                    _ => false,
                })
                .unwrap_or(true);

            if !is_stable_for_target {
                return Err(fn_candidate.with_note(GenerationExclusionNote::Reason(format!(
                    "function is not stable for the target: {:?} or unstable. Item stability: {stability:?}",
                    args.mrsv_target()
                ))));
            }

            if fn_candidate.is_unsafe {

                return Err(fn_candidate.with_note(GenerationExclusionNote::Reason("Function is unsafe".to_string())))
            }

            if matches!(fn_candidate.kind, FunctionCandidateKind::TraitImplMethod { .. }) && !ctxt.tcx.visibility(fn_candidate.did).is_public() {
                return Err(fn_candidate.with_note(GenerationExclusionNote::Reason("Function is not public and does not come from a trait impl".to_string())));
            }


            let arg_names = ctxt.tcx.fn_arg_idents(fn_candidate.did);
            fn_candidate.arguments = fn_candidate.sig.inputs().iter().zip(arg_names).enumerate().map(|(index, (arg_ty, ident))| {
                let candidate_input = FunctionArgCandidate::new(ident.map(|id| id.to_string()).unwrap_or(index.to_string()));
                
                let alternative_type = wrap_non_primtiives(*arg_ty, ctxt.tcx, &ctxt.cached_traits.bms_types);                
                let supported_as_primitive = alternative_type.implements_from_script(
                    alternative_type,
                    &ctxt.cached_traits.bms_types,
                    ctxt.tcx
                );

                match supported_as_primitive  {
                    Ok(_) => candidate_input.with_reflection_strategy(ReflectionStrategy::Primitive),
                    Err(e1) => candidate_input
                        .with_note(GenerationExclusionNote::Reason("argument is neither a primitive type implementing FromScript or a reflectable type".to_string()))
                        .with_note(e1)
                }
            }).collect::<Vec<_>>();

            let alternative_type = wrap_non_primtiives(fn_candidate.sig.output(), ctxt.tcx, &ctxt.cached_traits.bms_types);
            let supported_as_primitive = alternative_type.implements_into_script(
                    alternative_type,
                    &ctxt.cached_traits.bms_types,
                    ctxt.tcx
                );
            fn_candidate.ret = match supported_as_primitive {
                Ok(_) => fn_candidate.ret.with_reflection_strategy(ReflectionStrategy::Primitive),
                Err(e1) => fn_candidate.ret
                    .with_note(GenerationExclusionNote::Reason("return type is neither a primitive type implementing IntoScript or a reflectable type".to_string()))
                    .with_note(e1)
            };


            if fn_candidate.applying_notes().next().is_some() {
                // notes inside will suffice here
                return Err(fn_candidate);
            }

            Ok(fn_candidate)
        })
        .partition_result();
    (functions, excluded_functions)
}

fn generate_variants<'tcx>(
    ctxt: &mut BevyCtxt<'tcx>,
    did: DefId,
) -> (Vec<VariantCandidate<'tcx>>, Vec<VariantCandidate<'tcx>>) {
    let candidate = &ctxt.reflect_types[&did];
    let (mut variants, excluded_variants): (Vec<_>, Vec<_>) = candidate
        .def
        .variants()
        .iter()
        .map(|variant| {
            let variant_candidate = VariantCandidate::new(variant);
            if has_reflect_ignore_attr(ctxt.tcx.get_all_attrs(variant.def_id)) {
                return Err(variant_candidate.with_note(GenerationExclusionNote::Reason(
                    "variant has 'reflect(ignore)' attribute".to_string(),
                )));
            }

            Ok(variant_candidate)
        })
        .partition_result();

    for variant in &mut variants {
        let param_env = TypingEnv::non_body_analysis(ctxt.tcx, variant.def.def_id);

        let (fields, excluded_fields): (Vec<_>, Vec<_>) = variant
            .def
            .fields
            .iter()
            .map(|field| {
                let candidate = FieldCandidate::new(field);
                let visibility = field.vis;
                if !visibility.is_public() {
                    return Err(candidate.with_note(GenerationExclusionNote::Reason(format!(
                        "field is not public {:?}",
                        field.did
                    ))));
                }

                if has_reflect_ignore_attr(ctxt.tcx.get_all_attrs(field.did)) {
                    return Err(candidate.with_note(GenerationExclusionNote::Reason(
                        "field has 'reflect(ignore)' attribute".to_string(),
                    )));
                }

                let field_ty = ctxt.tcx.erase_and_anonymize_regions(
                    ctxt.tcx.type_of(field.did).instantiate_identity(),
                );

                Ok(
                    if type_is_supported_as_non_proxy_arg(
                        ctxt.tcx,
                        param_env,
                        &ctxt.cached_traits,
                        field_ty,
                    )
                    .is_ok()
                        && type_is_supported_as_non_proxy_return_val(
                            ctxt.tcx,
                            param_env,
                            &ctxt.cached_traits,
                            field_ty,
                        )
                        .is_ok()
                    {
                        candidate.with_reflection_strategy(ReflectionStrategy::Primitive)
                    } else if type_is_supported_as_proxy_arg(
                        ctxt.tcx,
                        &ctxt.reflect_types,
                        &ctxt.meta_loader,
                        field_ty,
                    )
                    .is_ok()
                        && type_is_supported_as_proxy_return_val(
                            ctxt.tcx,
                            &ctxt.reflect_types,
                            &ctxt.meta_loader,
                            field_ty,
                        )
                        .is_ok()
                    {
                        candidate.with_reflection_strategy(ReflectionStrategy::Proxy)
                    } else {
                        candidate.with_reflection_strategy(ReflectionStrategy::Reflection)
                    },
                )
            })
            .partition_result();

        variant.fields.extend(fields);
        variant.excluded_fields.extend(excluded_fields);
    }
    (variants, excluded_variants)
}

fn get_function_generics(tcx: TyCtxt, fn_did: DefId, impl_did: DefId) -> Vec<Ty> {
    // the early binder for this fn_sig will contain the generics on the function
    // we can't use it to iterate them though, for that we need to get the generics via the identity mapping
    // we want to first instantiate the function with any args in the impl, as those don't affect the standalone function signature

    let identity_args = GenericArgs::identity_for_item(tcx, fn_did)
        .types()
        .collect::<Vec<_>>();
    let identity_args_impl = GenericArgs::identity_for_item(tcx, impl_did)
        .types()
        .collect::<Vec<_>>();
    identity_args
        .into_iter()
        .filter(|arg| !identity_args_impl.contains(arg))
        .collect::<Vec<_>>()
}

/// Checks if the given attributes contain among them a reflect ignore attribute
fn has_reflect_ignore_attr(attrs: &[rustc_hir::Attribute]) -> bool {
    attrs.iter().any(|a| {
        a.path_matches(&[Symbol::intern("reflect")])
            && a.value_str()
                .map(|s| s.as_str().contains("ignore"))
                .unwrap_or(false)
    })
}

/// Returns true if this type can be used in argument position by checking if it's a top level proxy arg
fn type_is_supported_as_proxy_arg<'tcx>(
    tcx: TyCtxt<'tcx>,
    reflect_types: &IndexMap<DefId, GenerationCandidate<'tcx, AdtDef<'tcx>>>,
    meta_loader: &MetaLoader,
    ty: Ty,
) -> Result<(), GenerationExclusionNote> {
    log::trace!("Checking type is supported as proxy arg: '{ty}'");

    // nested references are not allowed for now
    type_is_adt_and_reflectable(tcx, reflect_types, meta_loader, peel_refs_up_to_once(ty))
}

fn peel_refs_up_to_once(ty: Ty) -> Ty {
    if let TyKind::Ref(_, inner, _) = ty.kind() {
        return *inner;
    }
    ty
}

/// Returns true if this type can be used in return position by checking if it's a top level proxy arg without references
fn type_is_supported_as_proxy_return_val<'tcx>(
    tcx: TyCtxt<'tcx>,
    reflect_types: &IndexMap<DefId, GenerationCandidate<'tcx, AdtDef<'tcx>>>,
    meta_loader: &MetaLoader,
    ty: Ty,
) -> Result<(), GenerationExclusionNote> {
    log::trace!("Checking type is supported as proxy return val: '{ty}'");
    type_is_adt_and_reflectable(tcx, reflect_types, meta_loader, ty)
}

/// Check if the type is an ADT and is reflectable (i.e. a proxy is being generated for it in SOME crate that we know about from the meta files)
fn type_is_adt_and_reflectable<'tcx>(
    tcx: TyCtxt<'tcx>,
    reflect_types: &IndexMap<DefId, GenerationCandidate<'tcx, AdtDef<'tcx>>>,
    meta_loader: &MetaLoader,
    ty: Ty,
) -> Result<(), GenerationExclusionNote> {
    let adt_def = match ty.ty_adt_def() {
        Some(v) => v,
        None => {
            return Err(GenerationExclusionNote::Reason(format!(
                "Type is not an ADT: {ty}",
            )));
        }
    };

    let did = adt_def.did();

    // even though our meta might already be written at this point, we use this as a quick out
    if reflect_types.contains_key(&did) {
        // local types are easy to check
        return Ok(());
    }

    // for other crates, reach for meta data
    // we know a reflect impl can ONLY exist in one of two places due to orphan rules:
    // 1) the bevy_reflect crate
    // 2) the crate that defines the type
    // so search for these metas!
    let crate_name = tcx.crate_name(did.krate).to_ident_string();

    let contains_hash = meta_loader.one_of_meta_files_contains(
        &[&crate_name, "bevy_reflect"],
        Some(&tcx.crate_name(LOCAL_CRATE).to_ident_string()),
        tcx.def_path_hash(did),
    );

    if !contains_hash {
        return Err(GenerationExclusionNote::Reason(format!(
            "Meta for type: `{}` with hash: `{:?}`, was not found in meta files for {crate_name} or in bevy_reflect, meaning it will not generate a proxy.",
            tcx.item_name(did),
            tcx.def_path_hash(did),
        )));
    }

    Ok(())
}

/// Checks if the type can be used directly as a lua function argument, by checking if it implements the FromLua trait
fn type_is_supported_as_non_proxy_arg<'tcx>(
    tcx: TyCtxt<'tcx>,
    param_env: TypingEnv<'tcx>,
    cached_traits: &CachedItems,
    ty: Ty<'tcx>,
) -> Result<(), GenerationExclusionNote> {
    trace!("Checking type is supported as non proxy arg: '{ty:?}' with param_env: '{param_env:?}'");
    impls_trait(tcx, param_env, ty, cached_traits.bms_types.from_script)
}

/// Checks if the type can be used directly as a lua function output
fn type_is_supported_as_non_proxy_return_val<'tcx>(
    tcx: TyCtxt<'tcx>,
    param_env: TypingEnv<'tcx>,
    cached_traits: &CachedItems,
    ty: Ty<'tcx>,
) -> Result<(), GenerationExclusionNote> {
    trace!(
        "Checkign type is supported as non proxy return val: '{ty:?}' with param_env: '{param_env:?}'"
    );
    if let TyKind::Ref(region, _, _) = ty.kind()
        && region
            .get_name(tcx)
            .is_none_or(|rn| rn.as_str() != "'static")
    {
        return Err(GenerationExclusionNote::Reason(format!(
            "{ty} cannot be used as a return value. References are not supported as return values.",
        )));
    }

    impls_trait(tcx, param_env, ty, cached_traits.bms_types.into_script)
}

pub(crate) fn impls_trait<'tcx>(
    tcx: TyCtxt<'tcx>,
    param_env: TypingEnv<'tcx>,
    ty: Ty<'tcx>,
    trait_did: DefId,
) -> Result<(), GenerationExclusionNote> {


    if tcx
        .infer_ctxt()
        .build(param_env.typing_mode)
        .type_implements_trait(trait_did, [ty], param_env.param_env)
        .must_apply_modulo_regions()
    {
        Ok(())
    } else {
        let failure = debug_trait_impl_failure(tcx, param_env, trait_did, [ty].into_iter());
        Err(GenerationExclusionNote::Reason(format!(
            "type {ty}, does not implement the trait {}. {}",
            tcx.item_name(trait_did),
            failure.unwrap_err()
        )))
    }
}

pub fn debug_trait_impl_failure<'tcx>(
    tcx: TyCtxt<'tcx>,
    param_env: TypingEnv<'tcx>,
    trait_def: DefId,
    trait_params: impl Iterator<Item = Ty<'tcx>>,
) -> Result<(), String> {
    // let trait_generics = tcx.generics_of(trait_def);

    let params = trait_params.collect::<Vec<_>>();

    // if params.len() != trait_generics.count() {
    // panic!("Expected generics for trait {:?}", trait_generics);
    // }

    // 1) Build a trait ref for trait_def<ty_to_check>
    //    Use tcx.mk_trait_ref if present, otherwise create the TraitRef directly.
    // let trait_args = tcx.mk_args(&[GenericArg::from(ty_to_check)]);
    let trait_ref = ty::TraitRef::new(tcx, trait_def, params);

    // 2) Convert to a predicate (poly trait predicate)
    // Some rustc versions allow: let pred = ty::Binder::dummy(trait_ref).to_predicate(tcx);
    // If `to_predicate` is not found on Binder in your toolchain, use:
    // let poly_trait_pred = ty::Binder::dummy(trait_ref);
    // let predicate: Predicate<'tcx> = trait_ref.upcast(tcx); // <- if this errors, see notes below
        // let obligation = traits::Obligation {
        //     cause: traits::ObligationCause::dummy(),
        //     param_env,
        //     recursion_depth: 0,
        //     predicate: trait_ref.upcast(self.tcx),
        // };
    // 3) Build an obligation
    let cause = ObligationCause::dummy();
    let obligation = Obligation::new(tcx, cause, param_env.param_env, trait_ref);

    // 4) Create an infer context and selection machinery
    // The build(...) call needs a `typing_mode` argument in some versions.
    // Use your existing typing_mode (you should have it alongside param_env).
    let infcx = tcx.infer_ctxt().build(param_env.typing_mode);

    // Create an obligation context (fulfillment/selection helper)
    // In some versions this is `ObligationCtxt::new(&infcx)`, in others `ObligationCtxt::new_infer(&infcx)`.
    let ocx = ObligationCtxt::new(&infcx);

    // Register the obligation and try to select/fulfill.
    ocx.register_obligation(obligation);

    // This will attempt all obligation selection and return any errors (empty vec == ok)
    // API name may be `select_all_or_error`, `select_all_or_error(&mut infcx)`, or
    // `select_all_or_error(&infcx)` depending on version. Try the no-arg one first.
    let errors = ocx.evaluate_obligations_error_on_ambiguity();
    if errors.is_empty() {
        return Ok(());
    }

    // If there are errors, gather text + nested obligations for debugging
    let mut msgs = Vec::new();
    for err in errors {
        // FulfillmentError (or FulfillmentErrorCode) pretty-printers vary; we stringify:
        msgs.push(format!("fulfillment error: {:#?}", err));
    }

    // Also dump any nested obligations from the obligation stack if possible:
    // (some APIs expose ocx.fulfillment_cx or ocx.infcx; if not available, skip)
    
    // This is version-dependent; if not present, ignore.
    // msgs.push(format!("fulfillment_cx: {:#?}", ocx.infcx.));


    Err(msgs.join("\n"))
}