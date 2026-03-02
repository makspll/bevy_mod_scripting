mod debug_with_type_info;
mod get_type_dependencies;
mod into_script;
mod script_bindings;
mod script_globals;
mod typed_through;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote_spanned};
use syn::{FnArg, Ident, ImplItemFn, ItemImpl, LitStr, Path, Type};

pub use self::{
    debug_with_type_info::debug_with_type_info, get_type_dependencies::get_type_dependencies,
    into_script::into_script, script_bindings::script_bindings, script_globals::script_globals,
    typed_through::typed_through,
};

pub(crate) fn impl_fn_to_namespace_builder_registration(
    fun: &ImplItemFn,
    bevy_bindings_path: &Path,
) -> TokenStream {
    process_impl_fn(
        fun,
        Ident::new("register_documented", Span::call_site()),
        true,
        true,
        bevy_bindings_path,
    )
}

pub(crate) fn impl_fn_to_global_registry_registration(
    fun: &ImplItemFn,
    bevy_bindings_path: &Path,
) -> TokenStream {
    process_impl_fn(
        fun,
        Ident::new("register_documented", Span::call_site()),
        false,
        false,
        bevy_bindings_path,
    )
}

/// checks if the impl contains at least one public function
pub(crate) fn is_public_impl(fun: &ItemImpl) -> bool {
    for i in &fun.items {
        match i {
            syn::ImplItem::Fn(impl_item_fn) => {
                if matches!(impl_item_fn.vis, syn::Visibility::Public(..)) {
                    return true;
                }
            }
            _ => continue,
        }
    }

    false
}

/// Converts an impl block function into a function registration, i.e. a closure which will be used to register this function, as well as
/// the target function reference and other metadata
fn process_impl_fn(
    fun: &ImplItemFn,
    generated_name: Ident,
    include_arg_names: bool,
    include_context_arg: bool,
    bevy_bindings_path: &Path,
) -> TokenStream {
    let args = &fun.sig.inputs;
    let fun_span = fun.sig.ident.span();

    let args_names = match include_arg_names {
        true => {
            let args = args.iter().map(arg_name);

            quote_spanned!(fun_span=>
                &[#(#args),*]
            )
        }
        false => Default::default(),
    };

    let body = &fun.block;
    let docstring = parse_docstring(fun.attrs.iter())
        .map(|s| syn::LitStr::new(&s, Span::call_site()))
        .unwrap_or(syn::LitStr::new("", Span::call_site()));
    let fun_name = syn::LitStr::new(&fun.sig.ident.to_string(), Span::call_site());
    let out_type = match &fun.sig.output {
        syn::ReturnType::Default => quote_spanned! {fun_span=>
            ()
        },
        syn::ReturnType::Type(_, ty) => quote_spanned! {fun_span=>
            #ty
        },
    };

    let is_func_call_context =
        |arg| arg_ty_ident(arg).is_some_and(|id| id == "FunctionCallContext");

    let call_context_arg_name = args
        .iter()
        .find(|a| is_func_call_context(a))
        .map(arg_ident)
        .unwrap_or(Ident::new("_caller_context", Span::call_site()));

    let (arg_names, arg_tys) =
        collect_fn_args_tuple(args.into_iter().filter(|a| !is_func_call_context(a)));

    let context_arg = if include_context_arg {
        quote_spanned! {fun_span=>
            #call_context_arg_name: #bevy_bindings_path::FunctionCallContext,
        }
    } else {
        Default::default()
    };

    quote_spanned! {fun_span=>
        .#generated_name(
            #fun_name,
            #[allow(unused_parens)]
            |#context_arg (#(#arg_names),*): (#(#arg_tys),*)| {
                let output: #out_type = {#body};
                output
            },
            #docstring,
            #args_names
        )
    }
}

pub(crate) fn collect_fn_args_tuple<'a>(
    args: impl IntoIterator<Item = &'a FnArg>,
) -> (Vec<Ident>, Vec<&'a Type>) {
    let mut names = Vec::<Ident>::default();
    let mut tys = Vec::<&'a Type>::default();
    for a in args {
        names.push(arg_ident(a));
        tys.push(arg_ty(a));
    }
    (names, tys)
}

pub(crate) fn arg_name(arg: &FnArg) -> LitStr {
    match arg {
        syn::FnArg::Receiver(_) => LitStr::new("self", Span::call_site()),
        syn::FnArg::Typed(pat_type) => {
            LitStr::new(&stringify_pat_type(&pat_type.pat), Span::call_site())
        }
    }
}

pub(crate) fn arg_ident(arg: &FnArg) -> Ident {
    match arg {
        syn::FnArg::Receiver(_) => Ident::new("self", Span::call_site()),
        syn::FnArg::Typed(pat_type) => {
            Ident::new(&stringify_pat_type(&pat_type.pat), Span::call_site())
        }
    }
}

pub(crate) fn arg_ty(arg: &FnArg) -> &Type {
    match arg {
        syn::FnArg::Receiver(r) => &r.ty,
        syn::FnArg::Typed(pat_type) => &pat_type.ty,
    }
}

pub(crate) fn arg_ty_ident(arg: &FnArg) -> Option<&Ident> {
    match arg_ty(arg) {
        Type::Path(type_path) => type_path.path.get_ident(),
        _ => None,
    }
}

pub(crate) fn stringify_pat_type(pat_type: &syn::Pat) -> String {
    match pat_type {
        syn::Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
        syn::Pat::Type(pat_type) => stringify_pat_type(&pat_type.pat),

        p => p.to_token_stream().to_string(),
    }
}

/// Ideally we'd be doing something like rustdoc: https://github.com/rust-lang/rust/blob/124cc92199ffa924f6b4c7cc819a85b65e0c3984/compiler/rustc_resolve/src/rustdoc.rs#L102
/// but that is too much complexity, stripping the first space should be good enough for now.
pub(crate) fn parse_docstring<'a>(
    attrs: impl Iterator<Item = &'a syn::Attribute>,
) -> Option<String> {
    let docs = attrs.filter_map(|attr| {
        if attr.path().is_ident("doc")
            && let syn::Meta::NameValue(meta_name_value) = &attr.meta
            && let syn::Expr::Lit(expr_lit) = &meta_name_value.value
            && let syn::Lit::Str(lit_str) = &expr_lit.lit
        {
            if lit_str.value().len() > 1 {
                return Some(lit_str.value()[1..].to_string());
            } else {
                return Some(lit_str.value());
            }
        };

        None
    });

    // join with newline
    let docs = docs.collect::<Vec<_>>();
    if docs.is_empty() {
        return None;
    }

    Some(docs.join("\n"))
}
