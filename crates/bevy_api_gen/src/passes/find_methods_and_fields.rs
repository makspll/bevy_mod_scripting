use indexmap::IndexMap;
use log::{info, trace};
use rustc_ast::Attribute;
use rustc_hir::{
    def_id::{DefId, LOCAL_CRATE},
    Safety,
};
use rustc_infer::infer::TyCtxtInferExt;
use rustc_middle::ty::{
    AdtKind, AssocKind, FieldDef, FnSig, GenericArgs, Ty, TyCtxt, TyKind, TypingEnv,
};
use rustc_span::Symbol;
use rustc_trait_selection::infer::InferCtxtExt;

use crate::{
    Args, BevyCtxt, CachedTraits, FunctionContext, MetaLoader, ReflectType, ReflectionStrategy,
};

/// Finds all methods and fields which can be wrapped on a proxy, stores them in sorted order.
pub(crate) fn find_methods_and_fields(ctxt: &mut BevyCtxt<'_>, _args: &Args) -> bool {
    // we need to find all the methods and fields for which we want to generate lua bindings
    // we have to filter some out
    // go through all impls on the types (traits and non-traits) and pick signatures we're happy with

    // borrow checker fucky wucky
    let reflect_types = ctxt.reflect_types.keys().cloned().collect::<Vec<_>>();
    for def_id in reflect_types {
        let adt_def = ctxt.tcx.adt_def(def_id);

        match adt_def.adt_kind() {
            AdtKind::Enum => {
                let strats = adt_def.variants().iter().flat_map(|variant|  {
                    if has_reflect_ignore_attr(ctxt.tcx.get_attrs_unchecked(variant.def_id)) {
                        // TODO: is this the right approach? do we need to still include those variants? or do we just provide dummies
                        // or can we just skip those ?
                        info!("ignoring enum variant: {}::{} due to 'reflect(ignore)' attribute", ctxt.tcx.item_name(def_id), variant.name);
                        todo!();
                    }
                    let param_env = TypingEnv::non_body_analysis(ctxt.tcx, variant.def_id);
                    process_fields(ctxt.tcx, &ctxt.meta_loader, &ctxt.reflect_types, &ctxt.cached_traits, variant.fields.iter(), param_env)
                }).collect::<Vec<_>>();

                strats.iter().for_each(|(f_did, strat)| match strat {
                    ReflectionStrategy::Reflection => report_field_not_supported(ctxt.tcx, *f_did, def_id, None, "type is neither a proxy nor a type expressible as lua primitive"),
                    ReflectionStrategy::Filtered => report_field_not_supported(ctxt.tcx, *f_did, def_id, None, "field has a 'reflect(ignore)' attribute"),
                    _ => {}
                });

                let ty_ctxt = ctxt.reflect_types.get_mut(&def_id).unwrap();
                ty_ctxt.variant_data = Some(adt_def);
                ty_ctxt.set_field_reflection_strategies(strats.into_iter());

            },
            AdtKind::Struct => {
                let param_env = TypingEnv::non_body_analysis(ctxt.tcx, def_id);
                let fields = process_fields(ctxt.tcx, &ctxt.meta_loader, &ctxt.reflect_types,&ctxt.cached_traits, adt_def.all_fields(), param_env);
                fields.iter().for_each(|(f_did, strat)| match strat {
                    ReflectionStrategy::Reflection => report_field_not_supported(ctxt.tcx, *f_did, def_id, None, "type is neither a proxy nor a type expressible as lua primitive"),
                    ReflectionStrategy::Filtered => report_field_not_supported(ctxt.tcx, *f_did, def_id, None, "field has a 'reflect(ignore)' attribute"),
                    _ => {}
                });
                let ty_ctxt = ctxt.reflect_types.get_mut(&def_id).unwrap();
                assert!(ty_ctxt.variant_data.is_none(), "variant data already set!");
                ty_ctxt.variant_data = Some(adt_def);
                ty_ctxt.set_field_reflection_strategies(fields.into_iter());
            },
            t => panic!("Unexpected item type, all `Reflect` implementing items should be enums or structs. : {:?}", t)
        };

        // borrow checker fucky wucky pt2
        let trait_impls_for_ty = {
            let ty_ctxt = ctxt.reflect_types.get(&def_id).unwrap();
            ty_ctxt.trait_impls.as_ref()
                .expect("A type was not processed correctly in a previous pass, missing trait impl info")
                .values()
                .cloned()
                .collect::<Vec<_>>()
        };

        // should we not find functions set default value for future passes
        let ty_ctxt = ctxt.reflect_types.get_mut(&def_id).unwrap();
        assert!(
            ty_ctxt.valid_functions.is_none(),
            "valid functions already set!"
        );
        ty_ctxt.valid_functions = Some(Vec::default());

        // filter the list of all methods and select candidates applicable to proxy generation
        let mut all_impls = ctxt
            .tcx
            .inherent_impls(def_id)
            .iter()
            .chain(trait_impls_for_ty.iter().flatten())
            .collect::<Vec<_>>();

        // sort them to avoid unnecessary diffs, we can use hashes here as they are forever stable (touch wood)
        all_impls.sort_by_cached_key(|a| ctxt.tcx.def_path_hash(**a));

        for impl_did in all_impls {
            let functions = ctxt
                .tcx
                .associated_items(impl_did)
                .in_definition_order()
                .filter_map(|assoc_item| {
                    if assoc_item.kind != AssocKind::Fn {
                        return None;
                    }

                    let trait_did = ctxt
                        .tcx
                        .impl_trait_ref(*impl_did)
                        .map(|tr| tr.skip_binder().def_id);
                    let trait_name = trait_did
                        .map(|td| ctxt.tcx.item_name(td).to_ident_string())
                        .unwrap_or_else(|| "None".to_string());

                    let fn_name = assoc_item.name.to_ident_string();
                    let has_self = assoc_item.fn_has_self_parameter;
                    let fn_did = assoc_item.def_id;

                    trace!(
                        "Processing function: '{fn_name}' on type: `{}` on trait: `{trait_name}`",
                        ctxt.tcx.item_name(def_id)
                    );

                    let param_env = TypingEnv::non_body_analysis(ctxt.tcx, def_id);
                    let fn_sig = ctxt.tcx.fn_sig(fn_did).instantiate_identity();
                    let sig: FnSig = ctxt
                        .tcx
                        .normalize_erasing_late_bound_regions(param_env, fn_sig);

                    let function_generics = get_function_generics(ctxt.tcx, fn_did, *impl_did);

                    if !function_generics.is_empty() {
                        log::debug!(
                            "Skipping function: `{}` on type: `{}` as it has generics: {:?}",
                            assoc_item.name,
                            ctxt.tcx.item_name(def_id),
                            function_generics
                        );
                        return None;
                    }

                    if let Some(unstability) = ctxt.tcx.lookup_stability(fn_did) {
                        if unstability.is_unstable() {
                            log::debug!(
                                "Skipping unstable function: `{}` on type: `{}` feature: {:?}",
                                ctxt.tcx.item_name(fn_did),
                                ctxt.tcx.item_name(def_id),
                                unstability.feature.as_str()
                            );
                            return None;
                        }
                    };

                    let is_unsafe = sig.safety == Safety::Unsafe;

                    if trait_did.is_none() && !ctxt.tcx.visibility(fn_did).is_public() {
                        log::info!(
                            "Skipping non-public function: `{}` on type: `{}`",
                            fn_name,
                            ctxt.tcx.item_name(def_id)
                        );
                        return None;
                    }

                    let arg_names = ctxt.tcx.fn_arg_names(fn_did);

                    let mut reflection_strategies = Vec::with_capacity(sig.inputs().len());
                    for (idx, arg_ty) in sig.inputs().iter().enumerate() {
                        if type_is_supported_as_non_proxy_arg(
                            ctxt.tcx,
                            param_env,
                            &ctxt.cached_traits,
                            *arg_ty,
                        ) {
                            reflection_strategies.push(ReflectionStrategy::Primitive);
                        } else if type_is_supported_as_proxy_arg(
                            ctxt.tcx,
                            &ctxt.reflect_types,
                            &ctxt.meta_loader,
                            *arg_ty,
                        ) {
                            reflection_strategies.push(ReflectionStrategy::Proxy);
                        } else {
                            report_fn_arg_not_supported(
                                ctxt.tcx,
                                fn_did,
                                def_id,
                                *arg_ty,
                                &format!("argument \"{}\" not supported", arg_names[idx]),
                            );
                            return None;
                        }
                    }

                    if type_is_supported_as_non_proxy_return_val(
                        ctxt.tcx,
                        param_env,
                        &ctxt.cached_traits,
                        sig.output(),
                    ) {
                        reflection_strategies.push(ReflectionStrategy::Primitive);
                    } else if type_is_supported_as_proxy_return_val(
                        ctxt.tcx,
                        &ctxt.reflect_types,
                        &ctxt.meta_loader,
                        sig.output(),
                    ) {
                        reflection_strategies.push(ReflectionStrategy::Proxy);
                    } else {
                        report_fn_arg_not_supported(
                            ctxt.tcx,
                            fn_did,
                            def_id,
                            sig.output(),
                            "return value not supported",
                        );
                        return None;
                    }

                    Some(FunctionContext {
                        is_unsafe,
                        def_id: fn_did,
                        has_self,
                        trait_and_impl_did: trait_did.map(|td| (td, *impl_did)),
                        reflection_strategies,
                    })
                })
                .collect::<Vec<_>>();

            let ty_ctxt = ctxt.reflect_types.get_mut(&def_id).unwrap();
            // must exist since we set default above
            ty_ctxt.valid_functions.as_mut().unwrap().extend(functions);
        }
    }

    true
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

fn report_fn_arg_not_supported(tcx: TyCtxt, f_did: DefId, type_did: DefId, ty: Ty, reason: &str) {
    info!(
        "Ignoring function: `{}` on type: `{}` reason: `{}`, relevant type: `{}`",
        tcx.item_name(f_did),
        tcx.item_name(type_did),
        reason,
        ty
    );
}

fn report_field_not_supported(
    tcx: TyCtxt,
    f_did: DefId,
    type_did: DefId,
    variant_did: Option<DefId>,
    reason: &'static str,
) {
    let param_env = TypingEnv::non_body_analysis(tcx, type_did);
    let normalised_ty =
        tcx.normalize_erasing_regions(param_env, tcx.type_of(f_did).instantiate_identity());
    info!(
        "Ignoring field: `{}:{}` on type: `{}` in variant: `{}` as it is not supported: `{}`",
        tcx.item_name(f_did),
        normalised_ty,
        tcx.item_name(type_did),
        tcx.item_name(variant_did.unwrap_or(type_did)),
        reason
    );
}

/// Checks each field individually and returns reflection strategies
fn process_fields<'tcx, 'f, I: Iterator<Item = &'f FieldDef>>(
    tcx: TyCtxt<'tcx>,
    meta_loader: &MetaLoader,
    reflect_types: &IndexMap<DefId, ReflectType<'tcx>>,
    cached_traits: &CachedTraits,
    fields: I,
    param_env: TypingEnv<'tcx>,
) -> Vec<(DefId, ReflectionStrategy)> {
    fields
        .map(move |f| {
            if !f.vis.is_public() {
                return (f.did, crate::ReflectionStrategy::Filtered);
            }

            let field_ty = tcx.erase_regions(tcx.type_of(f.did).instantiate_identity());
            if type_is_supported_as_non_proxy_arg(tcx, param_env, cached_traits, field_ty)
                && type_is_supported_as_non_proxy_return_val(
                    tcx,
                    param_env,
                    cached_traits,
                    field_ty,
                )
            {
                (f.did, crate::ReflectionStrategy::Primitive)
            } else if type_is_supported_as_proxy_arg(tcx, reflect_types, meta_loader, field_ty)
                && type_is_supported_as_proxy_return_val(tcx, reflect_types, meta_loader, field_ty)
            {
                (f.did, crate::ReflectionStrategy::Proxy)
            } else if !has_reflect_ignore_attr(tcx.get_attrs_unchecked(f.did)) {
                (f.did, crate::ReflectionStrategy::Reflection)
            } else {
                (f.did, crate::ReflectionStrategy::Filtered)
            }
        })
        .collect::<Vec<_>>()
}

/// Checks if the given attributes contain among them a reflect ignore attribute
fn has_reflect_ignore_attr(attrs: &[Attribute]) -> bool {
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
    reflect_types: &IndexMap<DefId, ReflectType<'tcx>>,
    meta_loader: &MetaLoader,
    ty: Ty,
) -> bool {
    log::trace!("Checking type is supported as proxy arg: '{}'", ty);
    type_is_adt_and_reflectable(tcx, reflect_types, meta_loader, ty.peel_refs())
}

/// Returns true if this type can be used in return position by checking if it's a top level proxy arg without references
fn type_is_supported_as_proxy_return_val<'tcx>(
    tcx: TyCtxt<'tcx>,
    reflect_types: &IndexMap<DefId, ReflectType<'tcx>>,
    meta_loader: &MetaLoader,
    ty: Ty,
) -> bool {
    log::trace!("Checking type is supported as proxy return val: '{}'", ty);
    type_is_adt_and_reflectable(tcx, reflect_types, meta_loader, ty)
}

/// Check if the type is an ADT and is reflectable (i.e. a proxy is being generated for it in SOME crate that we know about from the meta files)
fn type_is_adt_and_reflectable<'tcx>(
    tcx: TyCtxt<'tcx>,
    reflect_types: &IndexMap<DefId, ReflectType<'tcx>>,
    meta_loader: &MetaLoader,
    ty: Ty,
) -> bool {
    ty.ty_adt_def().is_some_and(|adt_def| {
        let did = adt_def.did();

        // even though our meta might already be written at this point, we use this as a quick out
        if reflect_types.contains_key(&did) {
            // local types are easy to check
            return true;
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

        if contains_hash {
            log::info!(
                "Meta for type: `{}` with hash: `{:?}`, contained in the meta file",
                tcx.item_name(did),
                tcx.def_path_hash(did),
            );
        } else {
            log::info!(
                "Meta for type: `{}` with hash: `{:?}`, was not found in meta files for {crate_name} or in bevy_reflect, meaning it will not generate a proxy.",
                tcx.item_name(did),
                tcx.def_path_hash(did),
            );
        }

        contains_hash
    })
}

/// Checks if the type can be used directly as a lua function argument, by checking if it implements the FromLua trait
fn type_is_supported_as_non_proxy_arg<'tcx>(
    tcx: TyCtxt<'tcx>,
    param_env: TypingEnv<'tcx>,
    cached_traits: &CachedTraits,
    ty: Ty<'tcx>,
) -> bool {
    trace!("Checking type is supported as non proxy arg: '{ty:?}' with param_env: '{param_env:?}'");
    impls_trait(tcx, param_env, ty, cached_traits.bms_from_script.unwrap())
}

/// Checks if the type can be used directly as a lua function output
fn type_is_supported_as_non_proxy_return_val<'tcx>(
    tcx: TyCtxt<'tcx>,
    param_env: TypingEnv<'tcx>,
    cached_traits: &CachedTraits,
    ty: Ty<'tcx>,
) -> bool {
    trace!("Checkign type is supported as non proxy return val: '{ty:?}' with param_env: '{param_env:?}'");
    if let TyKind::Ref(region, _, _) = ty.kind() {
        if region.get_name().is_none_or(|rn| rn.as_str() != "'static") {
            return false;
        }
    }

    impls_trait(tcx, param_env, ty, cached_traits.bms_into_script.unwrap())
}

pub(crate) fn impls_trait<'tcx>(
    tcx: TyCtxt<'tcx>,
    param_env: TypingEnv<'tcx>,
    ty: Ty<'tcx>,
    trait_did: DefId,
) -> bool {
    tcx.infer_ctxt()
        .build(param_env.typing_mode)
        .type_implements_trait(trait_did, [ty], param_env.param_env)
        .must_apply_modulo_regions()
}
