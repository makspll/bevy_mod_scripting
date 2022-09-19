use bevy_mod_scripting_common::{implementor::WrapperImplementor, newtype::Newtype};
use implementor::LuaImplementor;
use proc_macro::TokenStream;
use syn::parse_macro_input;

pub(crate) mod derive_flags;
pub(crate) mod implementor;
pub(crate) mod rhai_method;

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
