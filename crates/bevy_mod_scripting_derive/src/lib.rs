//! Derive macros for BMS

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote_spanned, ToTokens};
use syn::{spanned::Spanned, ImplItemFn, ItemImpl};

/// Derive macro for generating script bindings from an impl block.
///
/// Does not support generics.
///
/// Arguments:
/// - `name`: the name to use to suffix the generated function, i.e. `test_fn` will generate `register_test_fn. Defaults to `functions`
/// - `remote`: If true the original impl block will be ignored, and only the function registrations will be generated
/// - `bms_core_path`: If set the path to override bms imports, normally only used internally
/// - `unregistered`: If set, will use `new_unregistered` instead of `new` for the namespace builder
#[proc_macro_attribute]
pub fn script_bindings(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(args as Args);

    let impl_block = syn::parse_macro_input!(input as ItemImpl);
    let impl_span = impl_block.span();
    // let (impl_generics, ty_generics, where_clause) = impl_block.generics.split_for_impl();

    let type_ident_with_generics = &impl_block.self_ty;
    let mut function_registrations = Vec::with_capacity(impl_block.items.len());
    for i in &impl_block.items {
        match i {
            syn::ImplItem::Fn(impl_item_fn) => {
                let fun = process_impl_fn(impl_item_fn);
                function_registrations.push(fun);
            }
            _ => continue,
        }
    }

    let impl_block = match args.remote {
        true => TokenStream::default(),
        false => quote_spanned! {impl_span=>
            #impl_block
        },
    };

    let bms_core_path = &args.bms_core_path;

    let function_name = format_ident!("register_{}", args.name);
    let builder_function_name = if args.unregistered {
        format_ident!("new_unregistered")
    } else {
        format_ident!("new")
    };

    let out = quote_spanned! {impl_span=>
        fn #function_name(world: &mut bevy::ecs::world::World) {
            #bms_core_path::bindings::function::namespace::NamespaceBuilder::<#type_ident_with_generics>::#builder_function_name(world)
                #(#function_registrations)*;
        }

        #impl_block
    };

    out.into()
}

struct Args {
    /// The name to use to suffix the generated function, i.e. `test_fn` will generate `register_test_fn
    pub name: syn::Ident,
    /// If true the original impl block will be ignored, and only the function registrations will be generated
    pub remote: bool,
    /// If set the path to override bms imports
    pub bms_core_path: syn::Path,
    /// If true will use `new_unregistered` instead of `new` for the namespace builder
    pub unregistered: bool,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse separated key-value pairs
        let pairs =
            syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated(input)?;

        let mut name = syn::Ident::new("functions", Span::call_site());
        let mut remote = false;
        let mut unregistered = false;
        let mut bms_core_path =
            syn::Path::from(syn::Ident::new("bevy_mod_scripting", Span::call_site()));
        bms_core_path.segments.push(syn::PathSegment {
            ident: syn::Ident::new("core", Span::call_site()),
            arguments: syn::PathArguments::None,
        });
        let mut unknown_spans = Vec::default();
        for pair in pairs {
            match &pair {
                syn::Meta::Path(path) => {
                    if path.is_ident("remote") {
                        remote = true;
                        continue;
                    } else if path.is_ident("unregistered") {
                        unregistered = true;
                        continue;
                    }
                }
                syn::Meta::NameValue(name_value) => {
                    if name_value.path.is_ident("bms_core_path") {
                        if let syn::Expr::Lit(path) = &name_value.value {
                            if let syn::Lit::Str(lit_str) = &path.lit {
                                bms_core_path = syn::parse_str(&lit_str.value())?;
                                continue;
                            }
                        }
                    } else if name_value.path.is_ident("name") {
                        if let syn::Expr::Lit(path) = &name_value.value {
                            if let syn::Lit::Str(lit_str) = &path.lit {
                                name = syn::parse_str(&lit_str.value())?;
                                continue;
                            }
                        }
                    }
                }
                _ => {
                    unknown_spans.push((pair.span(), "Unsupported meta kind for script_bindings"));
                    continue;
                }
            }

            unknown_spans.push((pair.span(), "Unknown argument to script_bindings"));
        }

        if !unknown_spans.is_empty() {
            return Err(syn::Error::new(unknown_spans[0].0, unknown_spans[0].1));
        }

        Ok(Self {
            remote,
            bms_core_path,
            name,
            unregistered,
        })
    }
}

fn stringify_pat_type(pat_type: &syn::Pat) -> String {
    match pat_type {
        syn::Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
        syn::Pat::Type(pat_type) => stringify_pat_type(&pat_type.pat),

        p => p.to_token_stream().to_string(),
    }
}

/// Converts an impl block function into a function registration, i.e. a closure which will be used to register this function, as well as
/// the target function reference and other metadata
fn process_impl_fn(fun: &ImplItemFn) -> TokenStream {
    let args = &fun.sig.inputs;
    let args_names = args.iter().map(|arg| match arg {
        syn::FnArg::Receiver(_) => syn::LitStr::new("self", Span::call_site()),
        syn::FnArg::Typed(pat_type) => {
            syn::LitStr::new(&stringify_pat_type(&pat_type.pat), Span::call_site())
        }
    });
    let body = &fun.block;
    let docstring = parse_docstring(fun.attrs.iter())
        .map(|s| syn::LitStr::new(&s, Span::call_site()))
        .unwrap_or(syn::LitStr::new("", Span::call_site()));
    let fun_name = syn::LitStr::new(&fun.sig.ident.to_string(), Span::call_site());
    let fun_span = fun.sig.ident.span();
    quote_spanned! {fun_span=>
        .register_documented(
            #fun_name,
            |#args| #body,
            #docstring,
            &[#(#args_names),*]
        )
    }
}

/// Ideally we'd be doing something like rustdoc: https://github.com/rust-lang/rust/blob/124cc92199ffa924f6b4c7cc819a85b65e0c3984/compiler/rustc_resolve/src/rustdoc.rs#L102
/// but that is too much complexity, stripping the first space should be good enough for now.
fn parse_docstring<'a>(attrs: impl Iterator<Item = &'a syn::Attribute>) -> Option<String> {
    let docs = attrs.filter_map(|attr| {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta_name_value) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta_name_value.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        if lit_str.value().len() > 1 {
                            return Some(lit_str.value()[1..].to_string());
                        } else {
                            return Some(lit_str.value());
                        }
                    }
                }
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
