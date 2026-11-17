//! `#[derive(ScriptData)]`: plain data passed to and from scripts by value, see the derive's documentation.

use proc_macro2::TokenStream;
use syn::{DeriveInput, spanned::Spanned};

use crate::derive::SharedArgs;

mod data_enum;
mod data_struct;

/// The arguments of `#[script_data(..)]`
#[derive(Default)]
struct Args {
    shared_args: SharedArgs,
    /// The script namespace a struct is documented in
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
        syn::Data::Enum(data) => match &args.namespace {
            // an enum is documented as the union of its variants' types, it isn't a type of its own
            Some(namespace) => Err(syn::Error::new(
                namespace.span(),
                "`namespace` only applies to structs, an enum is documented as the types of its variants",
            )),
            None => data_enum::expand(&derive_input, data, &args),
        },
        _ => Err(syn::Error::new(
            derive_input.ident.span(),
            "ScriptData is derived for structs with named fields, or enums whose variants hold one value each",
        )),
    };
    result.unwrap_or_else(|error| error.to_compile_error())
}
