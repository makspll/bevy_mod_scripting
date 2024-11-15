use std::ops::{Deref, DerefMut};

use tealr::{
    mlu::mlua::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti},
    ToTypename,
};

/// generates path to the given script depending on build configuration.
/// (optimized builds don't have the teal compiler available)
///
/// Current configuration will provide "scripts/*.tl" paths
/// ```rust
/// use bevy_mod_scripting_lua::lua_path;
/// assert_eq!("scripts/my_script.tl",lua_path!("my_script"))
/// ```
#[cfg(all(feature = "teal", debug_assertions))]
#[macro_export]
macro_rules! lua_path {
    ($v:literal) => {
        concat!("scripts/", $v, ".tl")
    };
}

/// generates path to the given script depending on build configuration.
/// (optimized builds don't have the teal compiler available)
///
/// Current configuration will provide "scripts/build/*.lua" paths
/// ```rust
/// use bevy_mod_scripting::lua_path;
/// assert_eq!("scripts/build/my_script.lua",lua_path!("my_script"))
/// ```
#[cfg(all(not(debug_assertions), feature = "teal"))]
#[macro_export]
macro_rules! lua_path {
    ($v:literal) => {
        concat!("scripts/build/", $v, ".lua")
    };
}

/// generates path to the given script depending on build configuration.
/// (optimized builds don't have the teal compiler available)
///
/// Current configuration will provide "/scripts/*.lua" paths
/// ```rust
/// use bevy_mod_scripting_lua::lua_path;
/// assert_eq!("scripts/my_script.lua",lua_path!("my_script"))
/// ```
#[cfg(not(feature = "teal"))]
#[macro_export]
macro_rules! lua_path {
    ($v:literal) => {
        concat!("scripts/", $v, ".lua")
    };
}

#[cfg(feature = "lua51")]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_lua51 {
    ( $( $tok:tt )* ) => { $( $tok )* }
}

#[cfg(not(feature = "lua51"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_lua51 {
    ( $( $tok:tt )* ) => {};
}

#[cfg(feature = "lua52")]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_lua52 {
    ( $( $tok:tt )* ) => { $( $tok )* }
}

#[cfg(not(feature = "lua52"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_lua52 {
    ( $( $tok:tt )* ) => {};
}

#[cfg(feature = "lua53")]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_lua53 {
    ( $( $tok:tt )* ) => { $( $tok )* }
}

#[cfg(not(feature = "lua53"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_lua53 {
    ( $( $tok:tt )* ) => {};
}

#[cfg(feature = "lua54")]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_lua54 {
    ( $( $tok:tt )* ) => { $( $tok )* }
}

#[cfg(not(feature = "lua54"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_lua54 {
    ( $( $tok:tt )* ) => {};
}

#[cfg(feature = "luajit")]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_luajit {
    ( $( $tok:tt )* ) => { $( $tok )* }
}

#[cfg(not(feature = "luajit"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_luajit {
    ( $( $tok:tt )* ) => {};
}

#[cfg(feature = "luajit52")]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_luajit52 {
    ( $( $tok:tt )* ) => { $( $tok )* }
}

#[cfg(not(feature = "luajit52"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_luajit52 {
    ( $( $tok:tt )* ) => {};
}

#[cfg(any(
    feature = "lua52",
    feature = "lua53",
    feature = "lua54",
    feature = "luajit52"
))]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_any_lua52_lua53_lua54_luajit52 {
    ( $( $tok:tt )* ) => { $( $tok )* }
}

#[cfg(not(any(
    feature = "lua52",
    feature = "lua53",
    feature = "lua54",
    feature = "luajit52"
)))]
#[doc(hidden)]
#[macro_export]
macro_rules! __cfg_feature_any_lua52_lua53_lua54_luajit52 {
    ( $( $tok:tt )* ) => {};
}

#[macro_export]
macro_rules! impl_userdata_from_lua {
    ($ty:ident) => {
        impl<'lua> ::tealr::mlu::mlua::FromLua<'lua> for $ty {
            fn from_lua(
                value: ::tealr::mlu::mlua::Value<'lua>,
                _lua: &::tealr::mlu::mlua::Lua,
            ) -> Result<Self, ::tealr::mlu::mlua::Error> {
                match value {
                    tealr::mlu::mlua::Value::UserData(ud) => {
                        // for types which deref to something else we need to be explicit
                        let self_ref: std::cell::Ref<Self> = ud.borrow::<Self>()?;
                        let self_ref: &Self = std::ops::Deref::deref(&self_ref);
                        Ok(self_ref.clone())
                    }
                    _ => {
                        return Err(::tealr::mlu::mlua::Error::FromLuaConversionError {
                            from: value.type_name(),
                            to: stringify!($ty),
                            message: None,
                        })
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_userdata_with_tealdata {
    ($ty:ident) => {
        impl ::tealr::mlu::mlua::UserData for $ty
        where
            Self: ::tealr::mlu::TealData,
        {
            fn add_methods<'lua, T: ::tealr::mlu::mlua::UserDataMethods<'lua, Self>>(
                methods: &mut T,
            ) {
                let mut wrapper = tealr::mlu::UserDataWrapper::from_user_data_methods(methods);
                <Self as ::tealr::mlu::TealData>::add_methods(&mut wrapper);
            }

            fn add_fields<'lua, T: ::tealr::mlu::mlua::UserDataFields<'lua, Self>>(fields: &mut T) {
                let mut wrapper = tealr::mlu::UserDataWrapper::from_user_data_fields(fields);
                <Self as ::tealr::mlu::TealData>::add_fields(&mut wrapper);
            }
        }
    };
}

/// Variadic newtype with [`ToTypename`] implemantation
pub struct Variadic<T>(pub(crate) tealr::mlu::mlua::Variadic<T>);

impl<T> Variadic<T> {
    pub fn new<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Variadic(tealr::mlu::mlua::Variadic::from_iter(iter.into_iter()))
    }
}

impl<T> Deref for Variadic<T> {
    type Target = tealr::mlu::mlua::Variadic<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Variadic<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ToTypename> ToTypename for Variadic<T> {
    fn to_typename() -> tealr::Type {
        let single_type = T::to_typename();
        let collection_type = <Vec<T>>::to_typename();
        tealr::Type::Or(vec![single_type, collection_type])
    }
}

impl<'lua, T: FromLua<'lua>> FromLuaMulti<'lua> for Variadic<T> {
    fn from_lua_multi(
        values: tealr::mlu::mlua::MultiValue<'lua>,
        lua: &'lua tealr::mlu::mlua::Lua,
    ) -> tealr::mlu::mlua::Result<Self> {
        Ok(Variadic(tealr::mlu::mlua::Variadic::from_lua_multi(
            values, lua,
        )?))
    }
}

impl<'lua, T: IntoLua<'lua>> IntoLuaMulti<'lua> for Variadic<T> {
    fn into_lua_multi(
        self,
        lua: &'lua tealr::mlu::mlua::Lua,
    ) -> tealr::mlu::mlua::Result<tealr::mlu::mlua::MultiValue<'lua>> {
        self.0.into_lua_multi(lua)
    }
}
