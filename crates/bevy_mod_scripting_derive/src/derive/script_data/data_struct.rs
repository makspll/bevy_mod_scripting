//! A struct with named fields, passed as a map of its fields.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, punctuated::Punctuated, token::Comma};

use super::{Args, typed_through};

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
    // the converted fields, named so they can't shadow the locals of the conversion
    let locals = names
        .iter()
        .map(|name| quote::format_ident!("__field_{}", name))
        .collect::<Vec<_>>();
    let typed_through = typed_through(
        derive_input,
        args,
        quote! {
            #bms::ScriptDataShape::Fields(&[#(
                #bms::ScriptDataField {
                    name: #names,
                    type_info: <#types as #bms::TypedThrough>::through_type_info,
                    default: <#types as #bms::ArgMeta>::default_value,
                }
            ),*])
        },
    );

    quote! {
        #[automatically_derived]
        impl #bms::FromScript for #ident {
            type This<'w> = Self;

            fn from_script(
                value: #bms::ScriptValue,
                scope: &#bms::CallScope<'_>,
            ) -> Result<Self, #bms::InteropError> {
                const NAMES: &[&str] = &[#( #names ),*];
                let given = #bms::StructFields::given(value)?;
                // the fields given, converted in one pass, the others are their defaults
                #( let mut #locals: Option<#types> = None; )*
                for (name, value) in given {
                    match name.as_str() {
                        #( #names => #locals = Some(
                            #bms::StructFields::convert::<#types>(stringify!(#ident), #names, value, scope)?
                        ), )*
                        unknown => {
                            return Err(#bms::StructFields::unknown(stringify!(#ident), unknown, NAMES));
                        }
                    }
                }
                Ok(Self { #( #field_idents: match #locals {
                    Some(value) => value,
                    None => #bms::StructFields::missing::<#types>(stringify!(#ident), #names, scope)?,
                }, )* })
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

        #typed_through

        #[automatically_derived]
        impl #bms::ArgMeta for #ident {}

        #[automatically_derived]
        impl #bms::GetTypeDependencies for #ident {
            // it isn't reflected, containers of it are registered as containers of the map it converts from
            type Underlying = #bms::ScriptValue;

            fn register_type_dependencies(registry: &mut #bms::__derive::TypeRegistry) {
                #bms::__derive::register_data_dependencies(::std::any::TypeId::of::<Self>(), || {
                    #( <#types as #bms::GetTypeDependencies>::register_type_dependencies(registry); )*
                });
            }
        }
    }
}
