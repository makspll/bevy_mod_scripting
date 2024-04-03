use std::fmt::Write;

use log::trace;
use rustc_ast::Attribute;
use rustc_hir::def_id::{DefId, LOCAL_CRATE};
use rustc_middle::ty::{FieldDef, ParamTy, Ty, TyKind, TypeFoldable};
use rustc_span::Symbol;

use crate::{
    Arg, Args, BevyCtxt, Field, Function, FunctionContext, ImportPathFinder, Item, Output,
    ReflectType, TemplateContext, Variant,
};
/// Converts the BevyCtxt into simpler data that can be used in templates directly,
/// Clears the BevyCtxt by clearing data structures after it uses them.
pub(crate) fn populate_template_data(ctxt: &mut BevyCtxt<'_>, args: &Args) -> bool {
    let tcx = &ctxt.tcx;

    let mut items: Vec<_> = Vec::with_capacity(ctxt.reflect_types.len());

    let clone_diagnostic = tcx.get_diagnostic_item(Symbol::intern("Clone")).unwrap();
    let debug_diagnostic = tcx.get_diagnostic_item(Symbol::intern("Debug")).unwrap();
    let display_diagnostic = tcx.get_diagnostic_item(Symbol::intern("Display")).unwrap();

    for (reflect_ty_did, ty_ctxt) in ctxt.reflect_types.drain(..).collect::<Vec<_>>().into_iter() {
        let fn_ctxts = ty_ctxt
            .valid_functions
            .as_ref()
            .expect("Missing function context for a type, were all the passes run correctly?");

        let has_static_methods = fn_ctxts.iter().any(|fn_ctxt| !fn_ctxt.has_self);

        let functions = process_functions(ctxt, fn_ctxts);
        let variant = ty_ctxt.variant_data.as_ref().unwrap();

        let is_tuple_struct = variant.is_struct()
            && variant
                .all_fields()
                .next()
                .is_some_and(|f| f.name.as_str().chars().all(|c| c.is_numeric()));

        let variants = variant
            .variants()
            .iter()
            .map(|variant| Variant {
                docstrings: docstrings(ctxt.tcx.get_attrs_unchecked(variant.def_id)),
                name: variant.name.to_ident_string().into(),
                fields: process_fields(ctxt, variant.fields.iter(), &ty_ctxt),
            })
            .collect::<Vec<_>>();
        let trait_impls = ty_ctxt.trait_impls.as_ref().unwrap();
        let item = Item {
            ident: tcx.item_name(reflect_ty_did).to_ident_string(),
            import_path: import_path(ctxt, reflect_ty_did),
            has_static_methods,
            functions,
            is_enum: variants.len() > 1,
            variants,
            is_tuple_struct,
            docstrings: docstrings(tcx.get_attrs_unchecked(reflect_ty_did)),
            impls_clone: trait_impls.contains_key(&clone_diagnostic),
            impls_debug: trait_impls.contains_key(&debug_diagnostic),
            impls_display: trait_impls.contains_key(&display_diagnostic),
        };

        items.push(item);
    }

    let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();
    let dep_names = tcx
        .crates(())
        .iter()
        .map(|d| tcx.crate_name(*d).to_ident_string())
        .collect::<Vec<_>>();

    let dependencies = ctxt
        .meta_loader
        .workspace_meta
        .crates
        .iter()
        .filter(|c| {
            dep_names.contains(c)
                && ctxt
                    .meta_loader
                    .meta_for(c)
                    .unwrap_or_else(|| panic!("Expected meta for dependency: {c}"))
                    .will_generate
        })
        .cloned()
        .collect();

    ctxt.clear();

    ctxt.template_context = Some(TemplateContext {
        crate_name,
        items,
        dependencies,
    });

    if let crate::Command::Generate {
        template_data_only, ..
    } = args.cmd
        && template_data_only
    {
        println!(
            "{}",
            serde_json::to_string_pretty(&ctxt.template_context).unwrap()
        );
        return false;
    }

    trace!("Populated template context:");
    trace!(
        "{}",
        serde_json::to_string_pretty(&ctxt.template_context).unwrap()
    );

    true
}

pub(crate) fn process_fields<'f, I: Iterator<Item = &'f FieldDef>>(
    ctxt: &BevyCtxt,
    fields: I,
    ty_ctxt: &ReflectType,
) -> Vec<Field> {
    fields
        .map(|field| Field {
            docstrings: docstrings(ctxt.tcx.get_attrs_unchecked(field.did)),
            ident: field.name.to_ident_string(),
            ty: ty_to_string(ctxt, ctxt.tcx.type_of(field.did).skip_binder()),
            reflection_strategy: *ty_ctxt
                .get_field_reflection_strat(field.did)
                .unwrap_or_else(|| panic!("{ty_ctxt:#?}")),
        })
        .collect()
}

pub(crate) fn process_functions(ctxt: &BevyCtxt, fns: &[FunctionContext]) -> Vec<Function> {
    fns.iter()
        .map(|fn_ctxt| {
            let fn_sig = ctxt.tcx.fn_sig(fn_ctxt.def_id).skip_binder().skip_binder();
            let args = ctxt
                .tcx
                .fn_arg_names(fn_ctxt.def_id)
                .iter()
                .zip(fn_sig.inputs())
                .enumerate()
                .map(|(idx, (ident, ty))| {
                    let (ident, ty) = if fn_ctxt.has_self && idx == 0 {
                        // self argument, we want to map to something like `&self` instead of `&Component`
                        // we do that by renaming every adt inside to "self"
                        // this is a bit hacky but it works, might not work if we decide to support generics in the future
                        // TODO: fix to work with generics
                        let ty = ty.fold_with(&mut rustc_middle::ty::fold::BottomUpFolder {
                            tcx: ctxt.tcx,
                            ty_op: |ty| {
                                if ty.is_adt() {
                                    ctxt.tcx.mk_ty_from_kind(TyKind::Param(ParamTy::new(
                                        0,
                                        Symbol::intern("self"),
                                    )))
                                } else {
                                    ty
                                }
                            },
                            lt_op: |lt| lt,
                            ct_op: |ct| ct,
                        });
                        (None, ty)
                    } else {
                        (ident.to_string().into(), *ty)
                    };
                    // remove projections like `<Struct as Trait>::AssocType`
                    let ty = ty_to_string(
                        ctxt,
                        ctxt.tcx
                            .normalize_erasing_regions(ctxt.tcx.param_env(fn_ctxt.def_id), ty),
                    );
                    Arg {
                        ident,
                        ty,
                        reflection_strategy: fn_ctxt.reflection_strategies[idx],
                    }
                })
                .collect();

            let ty = ty_to_string(
                ctxt,
                ctxt.tcx
                    .normalize_erasing_regions(ctxt.tcx.param_env(fn_ctxt.def_id), fn_sig.output()),
            );

            let output = Output {
                ty,
                reflection_strategy: *fn_ctxt.reflection_strategies.last().unwrap(),
            };

            let is_unsafe = fn_ctxt.is_unsafe;

            Function {
                is_unsafe,
                ident: ctxt.tcx.item_name(fn_ctxt.def_id).to_ident_string(),
                args,
                output,
                has_self: fn_ctxt.has_self,
                docstrings: docstrings(ctxt.tcx.get_attrs_unchecked(fn_ctxt.def_id)),
                from_trait_path: fn_ctxt
                    .trait_did
                    .map(|trait_did| import_path(ctxt, trait_did)),
            }
        })
        .collect()
}

/// extracts and normalizes docstrings in a given list of attributes
pub(crate) fn docstrings(attrs: &[Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter_map(|attr| attr.doc_str())
        .flat_map(|symbol| {
            symbol
                .as_str()
                .lines()
                .map(|str_| str_.to_owned())
                .collect::<Vec<_>>()
        })
        .collect()
}

// TODO: this is probably too simplistic, and might yield non public paths
pub(crate) fn import_path(ctxt: &BevyCtxt, def_id: DefId) -> String {
    ctxt.path_finder
        .find_import_paths(def_id)
        .first()
        .unwrap()
        .to_owned()
}

/// Normalizes type import paths in types before printing them
fn ty_to_string<'tcx>(ctxt: &BevyCtxt<'tcx>, ty: Ty<'tcx>) -> String {
    // walk through the type and replace all paths with their standardised import paths
    TyPrinter::new().print(&ctxt.path_finder, ty)
}

struct TyPrinter {
    buffer: String,
}

impl TyPrinter {
    pub fn new() -> Self {
        TyPrinter {
            buffer: String::new(),
        }
    }
    pub fn print(mut self, path_finder: &ImportPathFinder, ty: Ty<'_>) -> String {
        self.build_str(path_finder, ty);
        self.buffer
    }

    fn build_str(&mut self, path_finder: &ImportPathFinder, ty: Ty<'_>) {
        match ty.kind() {
            TyKind::Adt(adt_def, args) => {
                let did = adt_def.did();
                let import_path = path_finder
                    .find_import_paths(did)
                    .first()
                    .unwrap()
                    .to_owned();
                self.buffer.push_str(&import_path);
                if args.len() > 0 {
                    self.buffer.push('<');
                    for (idx, a) in args.iter().enumerate() {
                        match a.as_type() {
                            Some(ty) => self.build_str(path_finder, ty),
                            None => _ = self.buffer.write_str(&a.to_string()),
                        }
                        if idx != args.len() - 1 {
                            self.buffer.push_str(", ");
                        }
                    }
                    self.buffer.push('>');
                }
            }
            _ => self.buffer.push_str(&ty.to_string()),
        }
    }
}
