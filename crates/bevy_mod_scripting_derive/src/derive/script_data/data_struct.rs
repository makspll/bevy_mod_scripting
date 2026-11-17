//! A struct with named fields, passed as a map of its fields.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, punctuated::Punctuated, token::Comma};

use super::Args;
use crate::derive::parse_docstring;

/// The conversions and documentation of the struct, through `StructFields` / `StructMap`.
pub(super) fn expand(
    derive_input: &DeriveInput,
    fields: &Punctuated<Field, Comma>,
    args: &Args,
) -> TokenStream {
    let bms = &args.shared_args.bms_bindings_path;
    let ident = &derive_input.ident;
    let field_idents = fields
        .iter()
        .map(|field| field.ident.as_ref())
        .collect::<Vec<_>>();
    let names = field_idents
        .iter()
        .map(|ident| ident.map(|i| i.to_string()).unwrap_or_default())
        .collect::<Vec<_>>();
    let types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();
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
        impl #bms::FromScript for #ident {
            type This<'w> = Self;

            fn from_script(
                value: #bms::ScriptValue,
                scope: &#bms::CallScope<'_>,
            ) -> Result<Self, #bms::InteropError> {
                let mut fields = #bms::StructFields::new(stringify!(#ident), value)?;
                let value = Self { #( #field_idents: fields.take::<#types>(#names, scope)?, )* };
                fields.finish(&[#( #names ),*])?;
                Ok(value)
            }

            fn could_convert(value: &#bms::ScriptValue) -> bool {
                #bms::StructFields::could_convert(value)
            }
        }

        #[automatically_derived]
        impl #bms::IntoScript for #ident {
            fn into_script(
                self,
                world: #bms::WorldGuard,
            ) -> Result<#bms::ScriptValue, #bms::InteropError> {
                let mut map = #bms::StructMap::default();
                #( map.push(#names, self.#field_idents, world.clone())?; )*
                Ok(map.into_value())
            }
        }

        #[automatically_derived]
        impl #bms::TypedThrough for #ident {
            fn through_type_info() -> #bms::ThroughTypeInfo {
                static INFO: #bms::ScriptDataInfo = #bms::ScriptDataInfo {
                    type_id: || ::std::any::TypeId::of::<#ident>(),
                    path: concat!(module_path!(), "::", stringify!(#ident)),
                    identifier: stringify!(#ident),
                    namespace: #namespace,
                    docs: #docs,
                    fields: &[#(
                        #bms::ScriptDataField {
                            name: #names,
                            type_info: <#types as #bms::TypedThrough>::through_type_info,
                            default: <#types as #bms::ArgMeta>::default_value,
                        }
                    ),*],
                };
                #bms::ThroughTypeInfo::Data(&INFO)
            }
        }

        #[automatically_derived]
        impl #bms::ArgMeta for #ident {}

        #[automatically_derived]
        impl #bms::GetTypeDependencies for #ident {
            // it isn't reflected, containers of it are registered as containers of the map it converts from
            type Underlying = #bms::ScriptValue;

            fn register_type_dependencies(registry: &mut #bms::__derive::TypeRegistry) {
                #( <#types as #bms::GetTypeDependencies>::register_type_dependencies(registry); )*
            }
        }
    }
}
