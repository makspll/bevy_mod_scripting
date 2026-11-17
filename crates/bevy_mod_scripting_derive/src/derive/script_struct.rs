use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, spanned::Spanned};

use crate::derive::{SharedArgs, parse_docstring};

#[derive(Default)]
struct Args {
    shared_args: SharedArgs,
    namespace: Option<syn::LitStr>,
}

impl Args {
    fn parse(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut args = Args::default();
        for attr in attrs {
            if attr.path().is_ident("script_struct") {
                attr.parse_nested_meta(|meta| {
                    if args.shared_args.apply_nested_meta(&meta)? {
                        return Ok(());
                    }
                    if meta.path.is_ident("namespace") {
                        args.namespace = Some(meta.value()?.parse()?);
                        return Ok(());
                    }
                    Err(meta.error("Unknown argument to script_struct"))
                })?;
            }
        }
        Ok(args)
    }
}

pub fn script_struct(input: TokenStream) -> TokenStream {
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
            "ScriptStruct can't be derived for generic types",
        )
        .to_compile_error();
    }
    let fields = match &derive_input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => {
            return syn::Error::new(
                derive_input.ident.span(),
                "ScriptStruct can only be derived for structs with named fields",
            )
            .to_compile_error();
        }
    };

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
                static INFO: #bms::ScriptStructInfo = #bms::ScriptStructInfo {
                    type_id: || ::std::any::TypeId::of::<#ident>(),
                    path: concat!(module_path!(), "::", stringify!(#ident)),
                    identifier: stringify!(#ident),
                    namespace: #namespace,
                    docs: #docs,
                    fields: &[#(
                        #bms::ScriptStructField {
                            name: #names,
                            type_info: <#types as #bms::TypedThrough>::through_type_info,
                            default: <#types as #bms::ArgMeta>::default_value,
                        }
                    ),*],
                };
                #bms::ThroughTypeInfo::Struct(&INFO)
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
