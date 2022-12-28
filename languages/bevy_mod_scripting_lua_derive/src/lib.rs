use bevy_mod_scripting_common::{implementor::WrapperImplementor, newtype::Newtype};
use implementor::LuaImplementor;
// use impls::{impl_enum, impl_struct};
use proc_macro::TokenStream;
use syn::parse_macro_input;

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

// #[proc_macro_derive(LuaProxy, attributes(lua, scripting))]
// pub fn lua_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let data: Result<ProxyData, _> = (&input).try_into();

//     TokenStream::from(match data {
//         Ok(data) => match data {
//             ProxyData::Struct(data)
//             | ProxyData::TupleStruct(data)
//             | ProxyData::UnitStruct(data) => impl_struct(data),
//             ProxyData::Enum(data) => impl_enum(data),
//         },
//         Err(error) => error.to_compile_error(),
//     })
// }
