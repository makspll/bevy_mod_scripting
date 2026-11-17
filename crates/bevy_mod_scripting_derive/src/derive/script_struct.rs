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

/// Whether the type is written `Option<..>`, which makes the field optional.
fn is_option(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(path) if path.qself.is_none()
        && path.path.segments.last().is_some_and(|segment| segment.ident == "Option"))
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
    let optional = types.iter().map(|ty| is_option(ty)).collect::<Vec<_>>();
    let all_names = names.join(", ");
    let vars = field_idents
        .iter()
        .map(|ident| {
            quote::format_ident!("field_{}", ident.map(|i| i.to_string()).unwrap_or_default())
        })
        .collect::<Vec<_>>();
    let unwrapped = vars
        .iter()
        .zip(&optional)
        .zip(&names)
        .map(|((var, optional), name)| {
            if *optional {
                quote! { #var.flatten() }
            } else {
                quote! {
                    #var.ok_or_else(|| #bms::InteropError::string(format!(
                        "missing field `{}` of `{}`", #name, stringify!(#ident)
                    )))?
                }
            }
        });
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
                let map = match value {
                    #bms::ScriptValue::Map(map) => map,
                    // an empty table can't tell it isn't a list
                    #bms::ScriptValue::List(list) if list.is_empty() => Default::default(),
                    value => {
                        return Err(#bms::InteropError::value_mismatch(
                            ::std::any::TypeId::of::<Self>(),
                            value,
                        ));
                    }
                };
                #( let mut #vars: Option<#types> = None; )*
                for (key, value) in map {
                    match key.as_str() {
                        #(
                            #names => {
                                #vars = Some(
                                    <#types as #bms::FromScript>::from_script(value, scope).map_err(|e| {
                                        #bms::InteropError::string(format!(
                                            "field `{}` of `{}`: {e}", #names, stringify!(#ident)
                                        ))
                                    })?,
                                );
                            }
                        )*
                        other => {
                            return Err(#bms::InteropError::string(format!(
                                "unknown field `{other}` of `{}`, the fields are: {}",
                                stringify!(#ident),
                                #all_names
                            )));
                        }
                    }
                }
                Ok(Self { #( #field_idents: #unwrapped ),* })
            }

            fn could_convert(value: &#bms::ScriptValue) -> bool {
                match value {
                    #bms::ScriptValue::Map(_) => true,
                    #bms::ScriptValue::List(list) => list.is_empty(),
                    _ => false,
                }
            }
        }

        #[automatically_derived]
        impl #bms::IntoScript for #ident {
            fn into_script(
                self,
                world: #bms::WorldGuard,
            ) -> Result<#bms::ScriptValue, #bms::InteropError> {
                let mut entries = Vec::new();
                #(
                    let value = #bms::IntoScript::into_script(self.#field_idents, world.clone())?;
                    // left out like a missing optional field
                    if !matches!(value, #bms::ScriptValue::Unit) {
                        entries.push((#names.to_owned(), value));
                    }
                )*
                Ok(#bms::ScriptValue::Map(entries.into_iter().collect()))
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
                            optional: #optional,
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
