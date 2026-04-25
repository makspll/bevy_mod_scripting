mod arg_meta;
mod debug_with_type_info;
mod from_script;
mod get_type_dependencies;
mod into_script;
mod script_bindings;
mod script_globals;
mod typed_through;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote_spanned};
use syn::{Ident, ImplItemFn, ItemImpl};

pub use self::{
    arg_meta::arg_meta, debug_with_type_info::debug_with_type_info, from_script::from_script,
    get_type_dependencies::get_type_dependencies, into_script::into_script,
    script_bindings::script_bindings, script_globals::script_globals, typed_through::typed_through,
};

#[allow(dead_code)]
pub(crate) struct SharedArgs {
    /// If set the path to override bms bindings root path
    pub bms_bindings_path: syn::Path,
    /// If set the path to override bms core root path
    pub bms_core_path: syn::Path,
    /// If set the path to override bms display root path
    pub bms_display_path: syn::Path,
}

impl Default for SharedArgs {
    fn default() -> Self {
        let bms_path = syn::Path::from(syn::Ident::new("bevy_mod_scripting", Span::call_site()));

        let mut bms_bindings_path = bms_path.clone();
        bms_bindings_path.segments.push(syn::PathSegment {
            ident: syn::Ident::new("bindings", Span::call_site()),
            arguments: syn::PathArguments::None,
        });
        let mut bms_core_path = bms_path.clone();
        bms_core_path.segments.push(syn::PathSegment {
            ident: syn::Ident::new("core", Span::call_site()),
            arguments: syn::PathArguments::None,
        });
        let mut bms_display_path = bms_path.clone();
        bms_display_path.segments.push(syn::PathSegment {
            ident: syn::Ident::new("display", Span::call_site()),
            arguments: syn::PathArguments::None,
        });
        Self {
            bms_bindings_path,
            bms_core_path,
            bms_display_path,
        }
    }
}

impl SharedArgs {
    const BMS_BINDINGS_PATH: &'static str = "bms_bindings_path";
    const BMS_CORE_PATH: &'static str = "bms_core_path";
    const BMS_DISPLAY_PATH: &'static str = "bms_display_path";
    pub fn apply_nested_meta(&mut self, meta: &syn::meta::ParseNestedMeta) -> syn::Result<bool> {
        if meta.path.is_ident(Self::BMS_BINDINGS_PATH) {
            let value = meta.value()?;
            let lit: syn::LitStr = value.parse()?;
            self.bms_bindings_path = syn::parse_str(&lit.value())?;
            return Ok(true);
        }

        if meta.path.is_ident(Self::BMS_CORE_PATH) {
            let value = meta.value()?;
            let lit: syn::LitStr = value.parse()?;
            self.bms_core_path = syn::parse_str(&lit.value())?;
            return Ok(true);
        }

        if meta.path.is_ident(Self::BMS_DISPLAY_PATH) {
            let value = meta.value()?;
            let lit: syn::LitStr = value.parse()?;
            self.bms_display_path = syn::parse_str(&lit.value())?;
            return Ok(true);
        }

        Ok(false)
    }
}

pub(crate) fn impl_fn_to_namespace_builder_registration(fun: &ImplItemFn) -> TokenStream {
    process_impl_fn(
        fun,
        Ident::new("register_documented", Span::call_site()),
        true,
    )
}

pub(crate) fn impl_fn_to_global_registry_registration(fun: &ImplItemFn) -> TokenStream {
    process_impl_fn(
        fun,
        Ident::new("register_documented", Span::call_site()),
        false,
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
) -> TokenStream {
    let args = &fun.sig.inputs;
    let fun_span = fun.sig.ident.span();

    let args_names = match include_arg_names {
        true => {
            let args = args.iter().map(|arg| match arg {
                syn::FnArg::Receiver(_) => syn::LitStr::new("self", Span::call_site()),
                syn::FnArg::Typed(pat_type) => {
                    syn::LitStr::new(&stringify_pat_type(&pat_type.pat), Span::call_site())
                }
            });

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
    quote_spanned! {fun_span=>
        .#generated_name(
            #fun_name,
            |#args| {
                let output: #out_type = {#body};
                output
            },
            #docstring,
            #args_names
        )
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
