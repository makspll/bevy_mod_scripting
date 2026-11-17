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
    let last = variants.len() - 1;

    // the enum is a name for `Union<T1, Union<T2, T3>>`, which converts and documents it
    let last_type = types[last];
    let repr = types[..last].iter().rev().fold(
        quote! { #last_type },
        |inner, ty| quote! { #bms::Union<#ty, #inner> },
    );
    // `Union<T1, Union<T2, T3>>` to the variant holding its value
    let from_repr = (0..last).rev().fold(
        {
            let variant = variants[last];
            quote! { Self::#variant(rest) }
        },
        |inner, i| {
            let variant = variants[i];
            quote! {
                match rest.into_left() {
                    Ok(value) => Self::#variant(value),
                    Err(rest) => #inner,
                }
            }
        },
    );
    // the variant to `Union<T1, Union<T2, T3>>`, the left of as many rights as variants before it
    let into_repr = variants.iter().enumerate().map(|(i, variant)| {
        let innermost = match i == last {
            true => quote! { value },
            false => quote! { #bms::Union::new_left(value) },
        };
        let wrapped = (0..i).fold(
            innermost,
            |inner, _| quote! { #bms::Union::new_right(#inner) },
        );
        quote! { Self::#variant(value) => #wrapped }
    });

    quote! {
        #[automatically_derived]
        impl #bms::FromScript for #ident {
            type This<'w> = Self;

            fn from_script(
                value: #bms::ScriptValue,
                scope: &#bms::CallScope<'_>,
            ) -> Result<Self, #bms::InteropError> {
                let rest = <#repr as #bms::FromScript>::from_script(value, scope)?;
                Ok(#from_repr)
            }

            fn could_convert(value: &#bms::ScriptValue) -> bool {
                <#repr as #bms::FromScript>::could_convert(value)
            }
        }

        #[automatically_derived]
        impl #bms::IntoScript for #ident {
            fn into_script(
                self,
                world: #bms::WorldGuard,
            ) -> Result<#bms::ScriptValue, #bms::InteropError> {
                let repr: #repr = match self { #( #into_repr, )* };
                #bms::IntoScript::into_script(repr, world)
            }
        }

        #[automatically_derived]
        impl #bms::TypedThrough for #ident {
            fn through_type_info() -> #bms::ThroughTypeInfo {
                <#repr as #bms::TypedThrough>::through_type_info()
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
