use crate::{lua::bevy::LuaTypeRegistration, providers::bevy_ecs::LuaEntity, ReflectReference};
use bevy_mod_scripting_lua::{
    prelude::{
        FromLua, FromLuaMulti, IntoLua, IntoLuaMulti, Lua, LuaError, LuaMultiValue, LuaValue,
    },
    tealr::{self, FunctionParam, KindOfType, Name, SingleType, TealMultiValue, ToTypename, Type},
};
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/// Newtype abstraction of usize to represent a lua integer indexing things.
/// Lua is 1 based, host is 0 based, and this type performs this conversion automatically via ToLua and FromLua traits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LuaIndex(usize);

impl ToTypename for LuaIndex {
    fn to_typename() -> tealr::Type {
        <usize as ToTypename>::to_typename()
    }
}

impl Deref for LuaIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoLua<'_> for LuaIndex {
    fn into_lua(self, lua: &Lua) -> Result<LuaValue, LuaError> {
        to_lua_idx(self.0).into_lua(lua)
    }
}

impl FromLua<'_> for LuaIndex {
    fn from_lua(value: LuaValue, lua: &Lua) -> Result<LuaIndex, LuaError> {
        Ok(LuaIndex(to_host_idx(usize::from_lua(value, lua)?)))
    }
}

/// Converts lua index to host index (Lua is 1 based, host is 0 based)
pub fn to_host_idx(lua_idx: usize) -> usize {
    lua_idx - 1
}

/// Converts host index to lua index (Lua is 1 based, host is 0 based)
pub fn to_lua_idx(host_idx: usize) -> usize {
    host_idx + 1
}

/// forwards the TypeName implementation of T, useful for internal 'fake' global instances
pub struct DummyTypeName<T> {
    _ph: PhantomData<T>,
}

impl<T> DummyTypeName<T> {
    pub fn new(
        _: &bevy_mod_scripting_lua::tealr::mlu::mlua::Lua,
    ) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<Self> {
        Ok(Self {
            _ph: PhantomData::<T>,
        })
    }
}

impl<'lua, T> bevy_mod_scripting_lua::tealr::mlu::mlua::IntoLua<'lua> for DummyTypeName<T> {
    fn into_lua(
        self,
        _: &'lua bevy_mod_scripting_lua::tealr::mlu::mlua::Lua,
    ) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<
        bevy_mod_scripting_lua::tealr::mlu::mlua::Value<'lua>,
    > {
        Ok(bevy_mod_scripting_lua::tealr::mlu::mlua::Value::Nil)
    }
}

impl<T: ToTypename> ToTypename for DummyTypeName<T> {
    fn to_typename() -> bevy_mod_scripting_lua::tealr::Type {
        T::to_typename()
    }
}

/// A utility type that allows us to accept any number of components as a parameter into a function.
#[derive(Clone)]
pub struct VariadicComponents(pub Vec<LuaTypeRegistration>);

impl IntoLuaMulti<'_> for VariadicComponents {
    fn into_lua_multi(self, lua: &Lua) -> Result<LuaMultiValue<'_>, LuaError> {
        let values = LuaMultiValue::from_vec(
            self.0
                .into_iter()
                .map(|v| v.into_lua(lua).unwrap())
                .collect(),
        );

        Ok(values)
    }
}

impl FromLuaMulti<'_> for VariadicComponents {
    fn from_lua_multi(value: LuaMultiValue<'_>, lua: &Lua) -> Result<VariadicComponents, LuaError> {
        Ok(VariadicComponents(
            value
                .into_vec()
                .into_iter()
                .map(|v| LuaTypeRegistration::from_lua(v, lua).unwrap())
                .collect(),
        ))
    }
}

impl TealMultiValue for VariadicComponents {
    fn get_types_as_params() -> Vec<FunctionParam> {
        vec![FunctionParam {
            // `...:T` will be a variadic type
            param_name: Some(Name("...".into())),
            ty: LuaTypeRegistration::to_typename(),
        }]
    }
}

/// A utility enum that allows us to return an entity and any number of components from a function.
#[derive(Clone)]
pub enum VariadicQueryResult {
    Some(LuaEntity, Vec<ReflectReference>),
    None,
}

impl IntoLuaMulti<'_> for VariadicQueryResult {
    fn into_lua_multi(self, lua: &Lua) -> Result<LuaMultiValue<'_>, LuaError> {
        match self {
            VariadicQueryResult::Some(entity, vec) => {
                let mut values = LuaMultiValue::from_vec(
                    vec.into_iter()
                        .map(|v| v.into_lua(lua))
                        .collect::<Result<Vec<_>, LuaError>>()?,
                );

                values.push_front(entity.into_lua(lua)?);
                Ok(values)
            }
            VariadicQueryResult::None => Ok(().into_lua_multi(lua)?),
        }
    }
}

impl TealMultiValue for VariadicQueryResult {
    fn get_types_as_params() -> Vec<FunctionParam> {
        vec![
            FunctionParam {
                param_name: None,
                ty: LuaEntity::to_typename(),
            },
            FunctionParam {
                param_name: None,
                ty: Type::Single(SingleType {
                    kind: KindOfType::External,
                    // tealr doesn't have a way to add variadic return types, so it's inserted into the type name instead
                    name: Name(format!("{}...", stringify!(ReflectedValue)).into()),
                }),
            },
        ]
    }
}

#[macro_export]
macro_rules! impl_from_lua_with_clone {
    ($v:ty) => {
        impl<'lua> bevy_mod_scripting_lua::tealr::mlu::mlua::FromLua<'lua> for $v {
            #[inline]
            fn from_lua(
                value: bevy_mod_scripting_lua::tealr::mlu::mlua::Value<'lua>,
                _: &'lua bevy_mod_scripting_lua::tealr::mlu::mlua::Lua,
            ) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<$v> {
                match value {
                    bevy_mod_scripting_lua::tealr::mlu::mlua::Value::UserData(ud) => {
                        Ok(ud.borrow::<$v>()?.clone())
                    }
                    _ => Err(
                        bevy_mod_scripting_lua::tealr::mlu::mlua::Error::FromLuaConversionError {
                            from: value.type_name(),
                            to: "userdata",
                            message: None,
                        },
                    ),
                }
            }
        }
    };
}

/// Implements :tealr::TypeName, tealr::TypeBody and mlua::Userdata based on non-generic single token type name implementing TealData
#[macro_export]
macro_rules! impl_tealr_type {
    ($v:ty) => {
        impl bevy_mod_scripting_lua::tealr::ToTypename for $v {
            fn to_typename() -> bevy_mod_scripting_lua::tealr::Type {
                bevy_mod_scripting_lua::tealr::Type::new_single(stringify!($v), bevy_mod_scripting_lua::tealr::KindOfType::External)
            }
        }

        impl bevy_mod_scripting_lua::tealr::mlu::mlua::UserData for $v {
            fn add_fields<'lua, F: bevy_mod_scripting_lua::tealr::mlu::mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
                let mut wrapper = ::bevy_mod_scripting_lua::tealr::mlu::UserDataWrapper::from_user_data_fields(fields);
                <Self as ::bevy_mod_scripting_lua::tealr::mlu::TealData>::add_fields(&mut wrapper)
            }

            fn add_methods<'lua, M: bevy_mod_scripting_lua::tealr::mlu::mlua::prelude::LuaUserDataMethods<'lua, Self>>(
                methods: &mut M,
            ) {
                let mut x = ::bevy_mod_scripting_lua::tealr::mlu::UserDataWrapper::from_user_data_methods(methods);
                <Self as ::bevy_mod_scripting_lua::tealr::mlu::TealData>::add_methods(&mut x);
            }
        }

        impl bevy_mod_scripting_lua::tealr::TypeBody for $v {
            fn get_type_body() -> bevy_mod_scripting_lua::tealr::TypeGenerator {
                let mut gen = ::bevy_mod_scripting_lua::tealr::RecordGenerator::new::<Self>(false);
                gen.is_user_data = true;
                <Self as ::bevy_mod_scripting_lua::tealr::mlu::TealData>::add_fields(&mut gen);
                <Self as ::bevy_mod_scripting_lua::tealr::mlu::TealData>::add_methods(&mut gen);
                <_ as ::std::convert::From<_>>::from(gen)
            }
        }
    };
}

/// like create_bevy_mod_scripting_lua::tealr_union but translates to `any` in the lua declaration file,
/// a fill in to allow multiple userdata types
#[macro_export]
macro_rules! impl_tealr_any_union {
    ($visibility:vis $(Derives($($derives:ident), +))? enum $type_name:ident = $($sub_types_ident:ident: $sub_types_type:ty) | +) => {
        #[derive($($($derives ,)*)*)]
        #[allow(non_camel_case_types)]
        $visibility enum $type_name {
            $($sub_types_ident($sub_types_type) ,)*
        }
        impl<'lua> ::bevy_mod_scripting_lua::tealr::mlu::mlua::IntoLua<'lua> for $type_name {
            fn into_lua(self, lua: &'lua ::bevy_mod_scripting_lua::tealr::mlu::mlua::Lua) -> ::std::result::Result<::bevy_mod_scripting_lua::tealr::mlu::mlua::Value<'lua>, ::bevy_mod_scripting_lua::tealr::mlu::mlua::Error> {
                match self {
                    $($type_name::$sub_types_ident(x) => x.into_lua(lua),)*
                }
            }
        }
        impl<'lua> ::bevy_mod_scripting_lua::tealr::mlu::mlua::FromLua<'lua> for $type_name {
            fn from_lua(value: ::bevy_mod_scripting_lua::tealr::mlu::mlua::Value<'lua>, lua: &'lua ::bevy_mod_scripting_lua::tealr::mlu::mlua::Lua) -> ::std::result::Result<Self, ::bevy_mod_scripting_lua::tealr::mlu::mlua::Error> {
                $(match $sub_types_ident::from_lua(value.clone(),lua) {
                    Ok(x) => return Ok($type_name::$sub_types_ident(x)),
                    Err(::bevy_mod_scripting_lua::tealr::mlu::mlua::Error::FromLuaConversionError{from:_,to:_,message:_}) => {}
                    Err(x) => return Err(x)
                };)*
                Err(::bevy_mod_scripting_lua::tealr::mlu::mlua::Error::FromLuaConversionError{
                    to: stringify!( $($sub_types_ident)|* ),
                    from: value.type_name(),
                    message: None
                })
            }
        }
        impl ::bevy_mod_scripting_lua::tealr::ToTypename for $type_name {
            fn to_typename() -> bevy_mod_scripting_lua::tealr::Type {
                    bevy_mod_scripting_lua::tealr::Type::new_single("any", bevy_mod_scripting_lua::tealr::KindOfType::Builtin)
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

        impl $crate::lua::ValueLuaType for $name {}

        impl ::bevy_mod_scripting_lua::tealr::mlu::TealData for $name {

        }

        impl ::bevy::reflect::Typed for $name {
            fn type_info() -> &'static ::bevy::reflect::TypeInfo {
                panic!("This should never be called, I am a dummy implementation")
            }
        }

        impl ::bevy::reflect::TypePath for $name {
            fn short_type_path() -> &'static str{
                panic!("This should never be called, I am a dummy implementation")
            }

            fn type_path() -> &'static str{
                panic!("This should never be called, I am a dummy implementation")
            }
        }


        impl ::bevy::reflect::PartialReflect for $name {
            fn get_represented_type_info(&self) -> std::option::Option<&'static bevy::reflect::TypeInfo> {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn into_partial_reflect(self: Box<Self>) -> Box<dyn ::bevy::reflect::PartialReflect> {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn as_partial_reflect(&self) -> &dyn ::bevy::reflect::PartialReflect {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn as_partial_reflect_mut(&mut self) -> &mut dyn ::bevy::reflect::PartialReflect {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn try_into_reflect(self: Box<Self>) -> std::result::Result<std::boxed::Box<(dyn bevy::prelude::Reflect + 'static)>, std::boxed::Box<(dyn bevy::prelude::PartialReflect + 'static)>> {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn try_as_reflect(&self) -> std::option::Option<&(dyn bevy::prelude::Reflect + 'static)> {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn try_as_reflect_mut(&mut self) -> std::option::Option<&mut (dyn bevy::prelude::Reflect + 'static)> {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn try_apply(&mut self, _value: &dyn ::bevy::prelude::PartialReflect) -> std::result::Result<(), ::bevy::reflect::ApplyError> {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn reflect_ref(&self) -> ::bevy::reflect::ReflectRef {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn reflect_mut(&mut self) -> ::bevy::reflect::ReflectMut {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn reflect_owned(self: Box<Self>) -> ::bevy::reflect::ReflectOwned {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn clone_value(&self) -> Box<dyn ::bevy::prelude::PartialReflect + 'static> {
                panic!("This should never be called, I am a dummy implementation");
            }
        }


        impl ::bevy::reflect::Reflect for $name {

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

            fn set(&mut self, _: Box<dyn ::bevy::reflect::Reflect>) -> Result<(), Box<dyn ::bevy::reflect::Reflect>> {
                panic!("This should never be called, I am a dummy implementation");
            }

            fn into_reflect(self: Box<Self>) -> Box<dyn ::bevy::reflect::Reflect> {
                panic!("This should never be called, I am a dummy implementation");
            }
        }

        impl ::bevy::reflect::FromReflect for $name {
            fn from_reflect(_: &(dyn bevy::prelude::PartialReflect + 'static)) -> std::option::Option<Self> {
                panic!("This should never be called, I am a dummy implementation");
            }

        }

        impl ::bevy::reflect::GetTypeRegistration for $name {
            fn get_type_registration() -> bevy::reflect::TypeRegistration {
                panic!("This should never be called, I am a dummy implementation");
            }
        }

        $crate::impl_tealr_type!($name);
        $crate::impl_from_lua_with_clone!($name);
    }
}

// /// Implements UserData for type which implements TealData, can handle generics after the type name:
// /// ```rust,ignore
// /// impl_user_data!(MyType<'a,T : Debug>);
// /// ```
// macro_rules! impl_user_data {
//     ($v:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt $(<'a>)? )* )? ),+ >)? ) => {
//         impl $(< $( $lt $( : $clt $(+ $dlt $(<'a>)?)* )? ),+ >)? ::bevy_mod_scripting_lua::tealr::mlu::mlua::UserData for $v $(< $( $lt ),+ >)?  {
//             fn add_methods<'lua, M: ::bevy_mod_scripting_lua::tealr::mlu::mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
//                 let mut x = ::bevy_mod_scripting_lua::tealr::mlu::UserDataWrapper::from_user_data_methods(methods);
//                 <Self as ::bevy_mod_scripting_lua::tealr::mlu::TealData>::add_methods(&mut x);
//             }
//             fn add_fields<'lua, F: ::bevy_mod_scripting_lua::tealr::mlu::mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
//                 let mut wrapper = ::bevy_mod_scripting_lua::tealr::mlu::UserDataWrapper::from_user_data_fields(fields);
//                 <Self as ::bevy_mod_scripting_lua::tealr::mlu::TealData>::add_fields(&mut wrapper)
//             }
//         }

//     }
// }

// pub(crate) use impl_user_data;
