//! `#[derive(ScriptData)]`: plain data passed to and from scripts by value, see the derive's documentation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, spanned::Spanned};

use crate::derive::{SharedArgs, parse_docstring};

mod data_enum;
mod data_struct;

/// The arguments of `#[script_data(..)]`
#[derive(Default)]
struct Args {
    shared_args: SharedArgs,
    /// The script namespace the type is documented in
    namespace: Option<syn::LitStr>,
}

impl Args {
    fn parse(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut args = Args::default();
        for attr in attrs {
            if attr.path().is_ident("script_data") {
                attr.parse_nested_meta(|meta| {
                    if args.shared_args.apply_nested_meta(&meta)? {
                        return Ok(());
                    }
                    if meta.path.is_ident("namespace") {
                        args.namespace = Some(meta.value()?.parse()?);
                        return Ok(());
                    }
                    Err(meta.error("Unknown argument to script_data"))
                })?;
            }
        }
        Ok(args)
    }
}

/// The `TypedThrough` of a data type: a named, documented [`ScriptDataInfo`] whose parts, given by `shape`, are
/// described lazily, so a type holding itself is described without recursing.
fn typed_through(derive_input: &DeriveInput, args: &Args, shape: TokenStream) -> TokenStream {
    let bms = &args.shared_args.bms_bindings_path;
    let ident = &derive_input.ident;
    let namespace = match &args.namespace {
        Some(namespace) => quote! { Some(#namespace) },
        None => quote! { None },
    };
    let docs = match parse_docstring(derive_input.attrs.iter()) {
        Some(docs) => quote! { Some(#docs) },
        None => quote! { None },
    };
    quote! {
        #[automatically_derived]
        impl #bms::TypedThrough for #ident {
            fn through_type_info() -> #bms::ThroughTypeInfo {
                static INFO: #bms::ScriptDataInfo = #bms::ScriptDataInfo {
                    type_id: || ::std::any::TypeId::of::<#ident>(),
                    path: concat!(module_path!(), "::", stringify!(#ident)),
                    identifier: stringify!(#ident),
                    namespace: #namespace,
                    docs: #docs,
                    shape: #shape,
                };
                #bms::ThroughTypeInfo::Data(&INFO)
            }
        }
    }
}

/// A struct with named fields is a map of its fields, an enum whose variants hold one value each is any of those values.
pub fn script_data(input: TokenStream) -> TokenStream {
    let derive_input = match syn::parse2::<DeriveInput>(input) {
        Ok(derive_input) => derive_input,
        Err(err) => return err.to_compile_error(),
    };
    let args = match Args::parse(&derive_input.attrs) {
        Ok(args) => args,
        Err(error) => return error.to_compile_error(),
    };
    if !derive_input.generics.params.is_empty() {
        return syn::Error::new(
            derive_input.generics.span(),
            "ScriptData can't be derived for generic types",
        )
        .to_compile_error();
    }
    let result = match &derive_input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => Ok(data_struct::expand(&derive_input, &fields.named, &args)),
        syn::Data::Enum(data) => data_enum::expand(&derive_input, data, &args),
        _ => Err(syn::Error::new(
            derive_input.ident.span(),
            "ScriptData is derived for structs with named fields, or enums whose variants hold one value each",
        )),
    };
    let bms = &args.shared_args.bms_bindings_path;
    let ident = &derive_input.ident;
    match result {
        Ok(expanded) => quote! {
            #expanded

            // conflicts with the implementation for every reflected type, so a reflected type can't derive this
            #[automatically_derived]
            impl #bms::__derive::ReflectedTypesCannotDeriveScriptData for #ident {}
        },
        Err(error) => error.to_compile_error(),
    }
}
