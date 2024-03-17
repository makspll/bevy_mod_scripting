use log::debug;
use rustc_ast::{Attribute, VariantData};
use rustc_hir::{def_id::DefId, FieldDef};
use rustc_infer::infer::{InferCtxt, TyCtxtInferExt};
use rustc_middle::ty::{GenericArg, ParamEnv, Ty, TyCtxt};
use rustc_span::Symbol;
use rustc_trait_selection::infer::InferCtxtExt;

use crate::{BevyCtxt, FilteredEnumData, FilteredEnumVariant, FilteredVariant};

pub fn find_methods_and_fields(ctxt: &mut BevyCtxt<'_>) {
    let tcx = &ctxt.tcx;
    let all_types = &ctxt.reflect_types;
    let infer_ctxt = tcx.infer_ctxt().build();

    // we need to find all the methods and fields for which we want to generate lua bindings
    // we have to filter some out
    // go through all impls on the types (traits and non-traits) and pick signatures we're happy with
    for def_id in all_types {
        let item = tcx.hir().expect_item(def_id.expect_local());

        let variant = match item.kind {
            rustc_hir::ItemKind::Enum(variant_data, _) => {
                let mut variants = Vec::default();
                for variant in variant_data.variants {
                    if has_reflect_ignore_attr(tcx.hir().attrs(variant.hir_id)) {
                        // TODO: is this the right approach? do we need to still include those fields? or do we just provide dummies
                        // or can we just skip those ?
                        debug!("ignoring enum variant: {}::{} due to 'reflect(ignore)' attribute", tcx.item_name(*def_id), variant.ident);
                        variants.push(FilteredEnumVariant::Filtered);
                        continue;
                    }

                    for f in variant.data.fields() {
                        if field_can_be_lua_field(f,ctxt,&infer_ctxt) {
                            variants.push(FilteredEnumVariant::Variant(variant));
                        } else {
                            debug!("Skipping field: {}::{}.{} as it does not implement From and To Lua traits", tcx.item_name(*def_id), variant.ident, f.ident);
                            variants.push(FilteredEnumVariant::Filtered);
                        }
                    }
                }
                FilteredVariant::Enum(FilteredEnumData{
                    variants,
                })
            },
            rustc_hir::ItemKind::Struct(variant_data, _) => {
                FilteredVariant::Enum(FilteredEnumData { variants: vec![] })
                // if {}
                // Variant::Struct(variant_data)
            },
            t => panic!("Unexpected item type, all `Reflect` implementing items should be enums or structs. : {:?}", t)
        };
    }
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

/// Checks if the field satisfies the requirements to have it's own lua setters and getters.
fn field_can_be_lua_field<'tcx>(
    field: &FieldDef<'tcx>,
    ctxt: &BevyCtxt<'tcx>,
    infer_ctxt: &InferCtxt<'tcx>,
) -> bool {
    let ty = ctxt.tcx.type_of(field.def_id).skip_binder();
    let impls_from_lua = infer_ctxt
        .type_implements_trait(
            ctxt.cached_traits.mlua_from_lua.unwrap(),
            [ty],
            ParamEnv::reveal_all(),
        )
        .must_apply_considering_regions();

    let impls_to_lua = infer_ctxt
        .type_implements_trait(
            ctxt.cached_traits.mlua_to_lua.unwrap(),
            [ty],
            ParamEnv::reveal_all(),
        )
        .must_apply_considering_regions();

    impls_from_lua && impls_to_lua
}
