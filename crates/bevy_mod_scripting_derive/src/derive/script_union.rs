use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, spanned::Spanned};

use crate::derive::SharedArgs;

#[derive(Default)]
struct Args {
    shared_args: SharedArgs,
}

impl Args {
    fn parse(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut args = Args::default();
        for attr in attrs {
            if attr.path().is_ident("script_union") {
                attr.parse_nested_meta(|meta| {
                    if args.shared_args.apply_nested_meta(&meta)? {
                        return Ok(());
                    }
                    Err(meta.error("Unknown argument to script_union"))
                })?;
            }
        }
        Ok(args)
    }
}

pub fn script_union(input: TokenStream) -> TokenStream {
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
            "ScriptUnion can't be derived for generic types",
        )
        .to_compile_error();
    }
    let syn::Data::Enum(data) = &derive_input.data else {
        return syn::Error::new(
            derive_input.ident.span(),
            "ScriptUnion can only be derived for enums",
        )
        .to_compile_error();
    };
    let mut variants = Vec::new();
    let mut types = Vec::new();
    for variant in &data.variants {
        match &variant.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                variants.push(&variant.ident);
                types.push(&fields.unnamed[0].ty);
            }
            _ => {
                return syn::Error::new(
                    variant.span(),
                    "each variant of a ScriptUnion holds one value, i.e. `Number(f64)`",
                )
                .to_compile_error();
            }
        }
    }
    if variants.is_empty() {
        return syn::Error::new(derive_input.ident.span(), "a ScriptUnion needs variants")
            .to_compile_error();
    }

    let bms = &args.shared_args.bms_bindings_path;
    let ident = &derive_input.ident;
    let indices = 0..variants.len();

    quote! {
        #[automatically_derived]
        impl #bms::FromScript for #ident {
            type This<'w> = Self;

            /// The first variant whose type converts the value, only trying the types which could.
            fn from_script(
                value: #bms::ScriptValue,
                scope: &#bms::CallScope<'_>,
            ) -> Result<Self, #bms::InteropError> {
                let could = [#( <#types as #bms::FromScript>::could_convert(&value) ),*];
                // the value is only copied for types tried before the last one which could convert it
                let last = could.iter().rposition(|could| *could);
                #(
                    if could[#indices] {
                        if last == Some(#indices) {
                            return <#types as #bms::FromScript>::from_script(value, scope).map(Self::#variants);
                        }
                        if let Ok(converted) = <#types as #bms::FromScript>::from_script(value.clone(), scope) {
                            return Ok(Self::#variants(converted));
                        }
                    }
                )*
                Err(#bms::InteropError::value_mismatch(::std::any::TypeId::of::<Self>(), value))
            }

            fn could_convert(value: &#bms::ScriptValue) -> bool {
                #( <#types as #bms::FromScript>::could_convert(value) )||*
            }
        }

        #[automatically_derived]
        impl #bms::IntoScript for #ident {
            fn into_script(
                self,
                world: #bms::WorldGuard,
            ) -> Result<#bms::ScriptValue, #bms::InteropError> {
                match self {
                    #( Self::#variants(value) => #bms::IntoScript::into_script(value, world), )*
                }
            }
        }

        #[automatically_derived]
        impl #bms::TypedThrough for #ident {
            fn through_type_info() -> #bms::ThroughTypeInfo {
                #bms::ThroughTypeInfo::Union(vec![#( <#types as #bms::TypedThrough>::through_type_info() ),*])
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
