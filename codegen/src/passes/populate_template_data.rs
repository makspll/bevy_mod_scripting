use std::{borrow::Cow, convert::identity, panic};

use log::{trace, warn};
use rustc_hir::def_id::{DefId, LOCAL_CRATE};
use rustc_middle::ty::{
    AdtDef, FieldDef, GenericArg, GenericParamDefKind, TraitRef, Ty, TyKind, TypingEnv,
};
use rustc_span::Symbol;

use crate::{
    Arg, Args, BevyCtxt, Field, Function, FunctionContext, Item, Output, ReflectType,
    TemplateContext, Variant,
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

        let mut functions = process_functions(ctxt, fn_ctxts);
        functions.sort_by(|a, b| {
            a.ident
                .cmp(&b.ident)
                .then(a.args.len().cmp(&b.args.len()))
                .then(
                    a.args
                        .iter()
                        .zip(b.args.iter())
                        .fold(std::cmp::Ordering::Equal, |acc, (a, b)| {
                            acc.then(a.ty.cmp(&b.ty))
                        }),
                )
        });

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

    ctxt.clear();

    ctxt.template_context = Some(TemplateContext { crate_name, items });

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
            ty: ty_to_string(ctxt, ctxt.tcx.type_of(field.did).skip_binder(), false),
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
                .fn_arg_idents(fn_ctxt.def_id)
                .iter()
                .zip(fn_sig.inputs())
                .enumerate()
                .map(|(idx, (ident, ty))| {
                    let normalized_ty = ctxt.tcx.normalize_erasing_regions(
                        TypingEnv::non_body_analysis(ctxt.tcx, fn_ctxt.def_id),
                        *ty,
                    );
                    Arg {
                        ident: ident.map(|s| s.to_string()).unwrap_or(format!("arg_{idx}")),
                        ty: ty_to_string(ctxt, normalized_ty, false),
                        proxy_ty: ty_to_string(ctxt, normalized_ty, true),
                        reflection_strategy: fn_ctxt.reflection_strategies[idx],
                    }
                })
                .collect();

            let out_ty = ctxt.tcx.normalize_erasing_regions(
                TypingEnv::non_body_analysis(ctxt.tcx, fn_ctxt.def_id),
                fn_sig.output(),
            );

            let output = Output {
                ty: ty_to_string(ctxt, out_ty, false),
                proxy_ty: ty_to_string(ctxt, out_ty, true),
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
                from_trait_path: fn_ctxt.trait_and_impl_did.map(|(_, impl_did)| {
                    let trait_ref = ctxt.tcx.impl_trait_ref(impl_did).unwrap().skip_binder();

                    trait_ref_to_string(ctxt, trait_ref)
                }),
            }
        })
        .collect()
}

/// extracts and normalizes docstrings in a given list of attributes
pub(crate) fn docstrings(attrs: &[rustc_hir::Attribute]) -> Vec<String> {
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
fn ty_to_string<'tcx>(ctxt: &BevyCtxt<'tcx>, ty: Ty<'tcx>, proxy_types: bool) -> String {
    // walk through the type and replace all paths with their standardised import paths
    TyPrinter::new(
        Box::new(|ty| {
            ty.ty_adt_def()
                .map(|def| {
                    let def_id = def.did();
                    let def_path_hash = ctxt.tcx.def_path_hash(def_id);
                    let meta_sources = [
                        &ctxt.tcx.crate_name(def_id.krate).to_ident_string(),
                        "bevy_reflect",
                    ];

                    trace!("Checking ADT: `{}`.", ctxt.tcx.item_name(def_id),);

                    ctxt.meta_loader
                        .one_of_meta_files_contains(&meta_sources, None, def_path_hash)
                })
                .is_some_and(identity)
        }),
        Box::new(|did| Cow::Owned(import_path(ctxt, did))),
        proxy_types,
    )
    .print(ty)
}

/// Converts a specific trait instantiation (in the context of an impl) into a string taking into account correctly the
/// import transformations and generics
/// TODO: this doesn't explicitly print out associated types, because I don't think it's necessary yet and annoying to do (idk how to do it)
fn trait_ref_to_string<'tcx>(ctxt: &BevyCtxt<'tcx>, trait_ref: TraitRef<'tcx>) -> String {
    let generics_def = ctxt.tcx.generics_of(trait_ref.def_id);
    let generic_args = trait_ref
        .args
        .iter()
        .enumerate()
        .skip(if generics_def.has_self { 1 } else { 0 })
        .map(|(idx, a)| (a, generics_def.param_at(idx, ctxt.tcx)))
        // filter out non const | type generics and the compiler generated ones
        .filter(|(_, arg_def)| match arg_def.kind {
            GenericParamDefKind::Lifetime => false,
            GenericParamDefKind::Const { synthetic, .. } => !synthetic,
            _ => true,
        })
        .map(|(arg, arg_def)| {
            log::trace!("Printing for trait: `{trait_ref}` arg: `{arg}`, with def: `{arg_def:#?}`");

            let arg_ty = if let Some(ty) = arg.as_type() {
                ty
            } else if arg.as_const().is_some() {
                arg.as_type().unwrap()
            } else {
                unreachable!("should be filtered")
            };

            ty_to_string(ctxt, arg_ty, false)
        })
        .collect::<Vec<_>>();

    let trait_path = import_path(ctxt, trait_ref.def_id);

    if generic_args.is_empty() {
        trait_path
    } else {
        format!("{trait_path}::<{}>", generic_args.join(", "))
    }
}

#[derive(Clone, Copy)]
pub(crate) enum ProxyType {
    Ref,
    RefMut,
    Val,
}

impl ProxyType {
    pub fn to_ident_str(self) -> &'static str {
        match self {
            ProxyType::Ref => "Ref",
            ProxyType::RefMut => "Mut",
            ProxyType::Val => "Val",
        }
    }
}
/// Pretty prints types fully using the given import path finder or ADT's
struct TyPrinter<'a> {
    buffer: String,
    path_finder: Box<dyn Fn(DefId) -> Cow<'static, str> + 'a>,
    is_proxied_check: Box<dyn Fn(Ty<'_>) -> bool + 'a>,
    /// If true will wrap types in appropriate proxies instead of directly pringting the type
    proxy_types: bool,
}

impl<'a> TyPrinter<'a> {
    pub fn new(
        is_proxied_check: Box<dyn Fn(Ty<'_>) -> bool + 'a>,
        path_finder: Box<dyn Fn(DefId) -> Cow<'static, str> + 'a>,
        proxy_types: bool,
    ) -> Self {
        TyPrinter {
            buffer: String::new(),
            is_proxied_check,
            proxy_types,
            path_finder,
        }
    }

    pub fn print(mut self, ty: Ty<'_>) -> String {
        self.print_ty(ty);
        self.buffer
    }

    fn print_args<'tcx, I: Iterator<Item = GenericArg<'tcx>>>(&mut self, mut args: I) {
        let mut next = args.next();
        if next.is_some() {
            self.buffer.push('<');
            while let Some(arg) = next {
                let ty = if let Some(ty) = arg.as_type() {
                    ty
                } else if arg.as_const().is_some() {
                    arg.as_type().unwrap()
                } else {
                    next = args.next();
                    continue;
                };
                self.print_ty(ty);

                next = args.next();
                if next.is_some() {
                    self.buffer.push_str(", ");
                }
            }

            self.buffer.push('>');
        }
    }

    fn print_adt<'tcx, I: Iterator<Item = GenericArg<'tcx>>>(&mut self, ty: AdtDef<'tcx>, args: I) {
        let did = ty.did();
        let import_path = (self.path_finder)(did);
        self.buffer.push_str(&import_path);
        // filter out std::alloc::Global, as these are unstable to directly reference
        // idk how to do this more generally
        self.print_args(args.filter(|a| {
            a.as_type()
                .is_none_or(|t| t.to_string() != "std::alloc::Global")
        }));
    }

    fn print_ty(&mut self, ty: Ty<'_>) {
        match ty.kind() {
            TyKind::Bool => self.print_literal("bool"),
            TyKind::Char => self.print_literal("char"),
            TyKind::Str => self.print_literal("str"),
            TyKind::Int(ty) => self.print_literal(ty.name_str()),
            TyKind::Uint(ty) => self.print_literal(ty.name_str()),
            TyKind::Float(ty) => self.print_literal(ty.name_str()),
            TyKind::Adt(adt_ty, args) => {
                if self.proxy_types {
                    self.print_proxied_ty(ty, ProxyType::Val);
                } else {
                    self.print_adt(*adt_ty, args.iter());
                }
            }
            TyKind::Array(ty, const_) => {
                self.buffer.push('[');
                self.print_ty(*ty);
                self.buffer.push(';');
                // shortcut, we won't encounter ADT's here just use native printer
                self.buffer.push_str(&const_.to_string());
                self.buffer.push(']');
            }
            TyKind::Slice(ty) => {
                self.buffer.push('[');
                self.print_ty(*ty);
                self.buffer.push(']');
            }
            TyKind::RawPtr(ptr, mutability) => {
                self.buffer.push('*');
                if mutability.is_mut() {
                    self.buffer.push_str("mut ");
                }
                self.print_ty(*ptr);
            }
            TyKind::Ref(_, ty, mut_) => {
                if self.proxy_types {
                    let proxy_type = if mut_.is_mut() {
                        ProxyType::RefMut
                    } else {
                        ProxyType::Ref
                    };
                    self.print_proxied_ty(*ty, proxy_type);
                } else {
                    self.buffer.push('&');
                    if mut_.is_mut() {
                        self.buffer.push_str("mut ");
                    }
                    self.print_ty(*ty);
                }
            }
            TyKind::Tuple(tys) => {
                self.buffer.push('(');
                for (idx, ty) in tys.iter().enumerate() {
                    self.print_ty(ty);
                    if idx != tys.len() - 1 {
                        self.buffer.push(',');
                    }
                }
                self.buffer.push(')');
            }
            TyKind::Alias(_, ty) => {
                self.buffer.push_str(&(self.path_finder)(ty.def_id));
                self.print_args(ty.args.iter());
            }
            // self is one I think
            TyKind::Param(param) => self.print_literal(param.name.as_str()),
            _ => {
                warn!(
                    "Type outside the scope of the TyPrinter being printed: pretty=`{}` kind=`{:?}`",
                    ty,
                    ty.kind()
                );
                self.buffer.push_str(&ty.to_string())
            }
        }
    }

    /// prints a type but without making further proxies at this level
    /// i.e. a &T will be printed as RefProxy<T> instead of RefProxy<ValProxy<T>> since the T will not be printed via print_ty but directly here
    /// But only for ADT's, other types are printed as normal
    fn print_proxied_ty(&mut self, ty: Ty<'_>, proxy_type: ProxyType) {
        match ty.kind() {
            TyKind::Ref(_, ty, _) => {
                panic!("Nested references should not be proxied, got: `{ty}`");
            }
            TyKind::Adt(adt_ty, args) => {
                if (self.is_proxied_check)(ty) {
                    self.print_literal_surround_content(
                        proxy_type.to_ident_str(),
                        '<',
                        '>',
                        |self_| {
                            self_.print_adt(*adt_ty, args.iter());
                        },
                    );
                } else {
                    self.print_adt(*adt_ty, args.iter())
                }
            }
            _ => self.print_ty(ty),
        }
    }

    fn print_literal_surround_content<F: FnOnce(&mut Self)>(
        &mut self,
        literal: &str,
        start: char,
        end: char,
        f: F,
    ) {
        self.buffer.push_str(literal);
        self.buffer.push(start);
        f(self);
        self.buffer.push(end);
    }

    fn print_literal(&mut self, literal: &str) {
        self.buffer.push_str(literal);
    }
}
