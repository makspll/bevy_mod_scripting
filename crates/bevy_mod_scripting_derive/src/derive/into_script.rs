use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn into_script(input: TokenStream) -> TokenStream {
    let (ident, generics) = match syn::parse2(input) {
        Ok(DeriveInput {
            ident, generics, ..
        }) => (ident, generics),
        Err(err) => return err.to_compile_error(),
    };

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::bevy_mod_scripting::core::bindings::function::into::IntoScript for #ident #type_generics #where_clause {
            fn into_script(self, world: ::bevy_mod_scripting::core::bindings::WorldGuard) -> Result<::bevy_mod_scripting::core::bindings::script_value::ScriptValue, ::bevy_mod_scripting::core::error::InteropError> {
                ::bevy_mod_scripting::core::bindings::function::into::IntoScript::into_script(
                    ::bevy_mod_scripting::core::bindings::function::from::Val(self),
                    world,
                )
            }
        }
    }
}
