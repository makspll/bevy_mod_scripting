use bevy_mod_scripting_common::{
    implementor::WrapperImplementor, input::ProxyMeta, newtype::Newtype,
};
use implementor::LuaImplementor;
// use impls::{impl_enum, impl_struct};
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

pub(crate) mod derive_flags;
pub(crate) mod implementor;
pub(crate) mod impls;
pub(crate) mod lua_method;

#[proc_macro]
pub fn impl_lua_newtype(tokens: TokenStream) -> TokenStream {
    let newtype = parse_macro_input!(tokens as Newtype);
    let mut implementor = LuaImplementor::default();

    implementor
        .generate(newtype)
        .map_err(|e| e.to_compile_error())
        .unwrap_or_else(core::convert::identity)
        .into()
}

#[proc_macro]
pub fn impl_lua_proxy(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let meta: Result<ProxyMeta, syn::Error> = derive_input.try_into();

    Default::default()
}
