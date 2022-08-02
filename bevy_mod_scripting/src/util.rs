use std::marker::PhantomData;
use tealr::{mlu::mlua::ToLua, TypeName};

/// forwards the TypeName implementation of T, useful for internal 'fake' global instances
pub struct DummyTypeName<T> {
    _ph: PhantomData<T>,
}

impl<T> DummyTypeName<T> {
    pub fn new(_: &tealr::mlu::mlua::Lua) -> tealr::mlu::mlua::Result<Self> {
        Ok(Self {
            _ph: PhantomData::<T>,
        })
    }
}

impl<'lua, T> tealr::mlu::mlua::ToLua<'lua> for DummyTypeName<T> {
    fn to_lua(
        self,
        _: &'lua tealr::mlu::mlua::Lua,
    ) -> tealr::mlu::mlua::Result<tealr::mlu::mlua::Value<'lua>> {
        Ok(tealr::mlu::mlua::Value::Nil)
    }
}

impl<T: TypeName> tealr::TypeName for DummyTypeName<T> {
    fn get_type_parts() -> std::borrow::Cow<'static, [tealr::NamePart]> {
        T::get_type_parts()
    }
}

/// generates path to the given script depending on build configuration.
/// (optimized builds don't have the teal compiler available)
///
/// Current configuration will provide ".tl" paths
/// ```rust
/// use bevy_mod_scripting::lua_path;
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
/// Current configuration will provide ".lua" paths
/// ```rust
/// use bevy_mod_scripting::lua_path;
/// assert_eq!("scripts/build/my_script.lua",lua_path!("my_script"))
/// ```
#[cfg(all(feature = "teal", not(debug_assertions)))]
#[macro_export]
macro_rules! lua_path {
    ($v:literal) => {
        concat!("scripts/build/", $v, ".lua")
    };
}

/// Implements tealr::TypeName,tealr::TypeBody and mlua::Userdata based on non-generic single token type name implementing TealData
#[macro_export]
macro_rules! impl_tealr_type {
    ($v:ty) => {
        impl tealr::TypeName for $v {
            fn get_type_parts() -> ::std::borrow::Cow<'static, [tealr::NamePart]> {
                ::std::borrow::Cow::Borrowed(&[tealr::NamePart::Type(tealr::TealType {
                    name: ::std::borrow::Cow::Borrowed(stringify!($v)),
                    generics: None,
                    type_kind: tealr::KindOfType::External,
                })])
            }
        }

        impl mlua::UserData for $v {
            fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
                let mut wrapper = ::tealr::mlu::UserDataWrapper::from_user_data_fields(fields);
                <Self as ::tealr::mlu::TealData>::add_fields(&mut wrapper)
            }

            fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(
                methods: &mut M,
            ) {
                let mut x = ::tealr::mlu::UserDataWrapper::from_user_data_methods(methods);
                <Self as ::tealr::mlu::TealData>::add_methods(&mut x);
            }
        }

        impl tealr::TypeBody for $v {
            fn get_type_body() -> tealr::TypeGenerator {
                let mut gen = ::tealr::RecordGenerator::new::<Self>(false);
                gen.is_user_data = true;
                <Self as ::tealr::mlu::TealData>::add_fields(&mut gen);
                <Self as ::tealr::mlu::TealData>::add_methods(&mut gen);
                <_ as ::std::convert::From<_>>::from(gen)
            }
        }
    };
}

/// like create_tealr_union but translates to `any` in the lua declaration file,
/// a fill in to allow multiple userdata types
#[macro_export]
macro_rules! impl_tealr_any_union {
    ($visibility:vis $(Derives($($derives:ident), +))? enum $type_name:ident = $($sub_types:ident) | +) => {
        #[derive(Clone,$($($derives ,)*)*)]
        #[allow(non_camel_case_types)]
        $visibility enum $type_name {
            $($sub_types($sub_types) ,)*
        }
        impl<'lua> ::tealr::mlu::mlua::ToLua<'lua> for $type_name {
            fn to_lua(self, lua: &'lua ::tealr::mlu::mlua::Lua) -> ::std::result::Result<::tealr::mlu::mlua::Value<'lua>, ::tealr::mlu::mlua::Error> {
                match self {
                    $($type_name::$sub_types(x) => x.to_lua(lua),)*
                }
            }
        }
        impl<'lua> ::tealr::mlu::mlua::FromLua<'lua> for $type_name {
            fn from_lua(value: ::tealr::mlu::mlua::Value<'lua>, lua: &'lua ::tealr::mlu::mlua::Lua) -> ::std::result::Result<Self, ::tealr::mlu::mlua::Error> {
                $(match $sub_types::from_lua(value.clone(),lua) {
                    Ok(x) => return Ok($type_name::$sub_types(x)),
                    Err(::tealr::mlu::mlua::Error::FromLuaConversionError{from:_,to:_,message:_}) => {}
                    Err(x) => return Err(x)
                };)*
                Err(::tealr::mlu::mlua::Error::FromLuaConversionError{
                    to: stringify!( $($sub_types)|* ),
                    from: value.type_name(),
                    message: None
                })
            }
        }
        impl ::tealr::TypeName for $type_name {
            fn get_type_parts() -> ::std::borrow::Cow<'static,[::tealr::NamePart]> {
                ::std::borrow::Cow::Borrowed(&[::tealr::NamePart::Type(::tealr::TealType {
                    name: ::std::borrow::Cow::Borrowed("any"),
                    generics: None,
                    type_kind: ::tealr::KindOfType::Builtin,
                })])
            }

            fn get_type_kind() -> ::tealr::KindOfType {
                ::tealr::KindOfType::Builtin
            }
        }
    };
}

#[macro_export]
macro_rules! impl_tealr_generic{
    {
        $vis:vis struct $name:ident
    } => {
        #[derive(Default,Clone,Debug)]
        $vis struct $name;

        impl $crate::ValueLuaType for $name {}

        impl ::tealr::mlu::TealData for $name {

        }

        impl ::bevy::reflect::Reflect for $name {
            fn type_name(&self) -> &str {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn get_type_info(&self) -> &'static bevy::reflect::TypeInfo {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn as_any(&self) -> &dyn std::any::Any {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn as_reflect(&self) -> &dyn ::bevy::reflect::Reflect {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn as_reflect_mut(&mut self) -> &mut dyn ::bevy::reflect::Reflect {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn apply(&mut self, _: &dyn ::bevy::reflect::Reflect) {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn set(&mut self, _: Box<dyn ::bevy::reflect::Reflect>) -> Result<(), Box<dyn ::bevy::reflect::Reflect>> {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn clone_value(&self) -> Box<dyn ::bevy::reflect::Reflect> {
                panic!("This should never be called, I am a dummy implementation");
            }
        }

        impl ::bevy::reflect::FromReflect for $name {
            fn from_reflect(_: &dyn ::bevy::reflect::Reflect) -> Option<Self> {
                panic!("This should never be called, I am a dummy implementation");
            }
        }

        $crate::impl_tealr_type!($name);
    }
}

// /// Implements UserData for type which implements TealData, can handle generics after the type name:
// /// ```rust,ignore
// /// impl_user_data!(MyType<'a,T : Debug>);
// /// ```
// macro_rules! impl_user_data {
//     ($v:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt $(<'a>)? )* )? ),+ >)? ) => {
//         impl $(< $( $lt $( : $clt $(+ $dlt $(<'a>)?)* )? ),+ >)? ::tealr::mlu::mlua::UserData for $v $(< $( $lt ),+ >)?  {
//             fn add_methods<'lua, M: ::tealr::mlu::mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
//                 let mut x = ::tealr::mlu::UserDataWrapper::from_user_data_methods(methods);
//                 <Self as ::tealr::mlu::TealData>::add_methods(&mut x);
//             }
//             fn add_fields<'lua, F: ::tealr::mlu::mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
//                 let mut wrapper = ::tealr::mlu::UserDataWrapper::from_user_data_fields(fields);
//                 <Self as ::tealr::mlu::TealData>::add_fields(&mut wrapper)
//             }
//         }

//     }
// }

// pub(crate) use impl_user_data;
