use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn typed_through(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    }: DeriveInput = syn::parse2(input).unwrap();

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let turbofish = type_generics.as_turbofish();
    quote! {
        impl #impl_generics ::bevy_mod_scripting::core::docgen::typed_through::TypedThrough for #ident #type_generics #where_clause {
            fn through_type_info() -> ::bevy_mod_scripting::core::docgen::typed_through::ThroughTypeInfo {
                ::bevy_mod_scripting::core::docgen::typed_through::ThroughTypeInfo::TypeInfo(#ident #turbofish ::type_info())
            }
        }
    }
}
