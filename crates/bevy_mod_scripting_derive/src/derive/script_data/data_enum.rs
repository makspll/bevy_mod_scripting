//! An enum whose variants hold one value each, passed as the value of any of them.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, DeriveInput, spanned::Spanned};

use super::{Args, typed_through};

/// The conversions and documentation of the enum, a name for a nested `Union` of its variants' types.
pub(super) fn expand(
    derive_input: &DeriveInput,
    data: &DataEnum,
    args: &Args,
) -> syn::Result<TokenStream> {
    let mut variants = Vec::new();
    let mut types = Vec::new();
    for variant in &data.variants {
        match &variant.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                variants.push(&variant.ident);
                types.push(&fields.unnamed[0].ty);
            }
            _ => {
                return Err(syn::Error::new(
                    variant.span(),
                    "each variant of a ScriptData enum holds one value, i.e. `Number(f64)`",
                ));
            }
        }
    }
    if variants.is_empty() {
        return Err(syn::Error::new(
            derive_input.ident.span(),
            "a ScriptData enum needs variants",
        ));
    }

    let bms = &args.shared_args.bms_bindings_path;
    let ident = &derive_input.ident;
    let last = variants.len() - 1;

    // the enum converts as `Union<T1, Union<T2, T3>>`, and is documented as a named union of its variants' types
    let typed_through = typed_through(
        derive_input,
        args,
        quote! {
            #bms::ScriptDataShape::Union(&[#( <#types as #bms::TypedThrough>::through_type_info ),*])
        },
    );
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

    Ok(quote! {
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

        #typed_through

        #[automatically_derived]
        impl #bms::ArgMeta for #ident {}

        #[automatically_derived]
        impl #bms::GetTypeDependencies for #ident {
            // it isn't reflected, containers of it are registered as containers of the values it converts from
            type Underlying = #bms::ScriptValue;

            fn register_type_dependencies(registry: &mut #bms::__derive::TypeRegistry) {
                #bms::__derive::register_data_dependencies(::std::any::TypeId::of::<Self>(), || {
                    #( <#types as #bms::GetTypeDependencies>::register_type_dependencies(registry); )*
                });
            }
        }
    })
}
