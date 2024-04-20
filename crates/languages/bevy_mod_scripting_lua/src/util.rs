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
/// use bevy_mod_scripting::lua_path;
/// assert_eq!("scripts/build/my_script.lua",lua_path!("my_script"))
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
        impl<'lua> FromLua<'lua> for $ty {
            fn from_lua(
                value: ::tealr::mlu::mlua::Value<'lua>,
                lua: &::tealr::mlu::mlua::Lua,
            ) -> Result<Self, ::tealr::mlu::mlua::Error> {
                match value {
                    tealr::mlu::mlua::Value::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
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
