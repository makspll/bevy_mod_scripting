use ::std::borrow::Cow;
use ::std::marker::PhantomData;
use ::std::mem::MaybeUninit;
use ::std::sync::{Weak,Arc};
use ::std::convert::AsRef;
use ::std::ops::{Deref,DerefMut};
use crate::{ScriptRef, ReflectedValue, impl_tealr_type, ReflectBase, ReflectPtr, ValueIndex, LuaWrapper};
use ::bevy::ecs::system::Command;
use ::bevy::hierarchy::BuildWorldChildren;
use ::bevy::reflect::{ReflectRef, FromReflect};
use ::bevy::reflect::erased_serde::Serialize;
use ::bevy::{reflect::{reflect_trait, Reflect, TypeRegistry, TypeRegistration, DynamicStruct, DynamicTupleStruct, DynamicTuple, DynamicList, DynamicArray, DynamicMap}, prelude::{World, ReflectComponent, ReflectDefault, ReflectResource}, hierarchy::{Children, Parent, DespawnChildrenRecursive, DespawnRecursive}};

use parking_lot::RwLock;
use serde::Deserialize;
use tealr::TypeName;
use tealr::mlu::mlua::MetaMethod;
use tealr::mlu::{mlua::{Lua, Value,self, UserData, ToLua,FromLua}, TealData, TealDataMethods};

use self::bevy::LuaWorld;

pub mod std;
pub mod bevy;

impl ValueIndex<Value<'_>> for ScriptRef {
    type Output=Result<Self,mlua::Error>;

    fn index(&self, index: Value<'_>) -> Self::Output {
        match index {
            Value::Integer(idx) => {
                Ok(self.index(idx  as usize))
            }
            Value::String(field) => {
                let str_ = field.to_str()?.to_string();
                // TODO: hopefully possible to use a &'_ str here
                // but this requires Reflect implementation for &str 
                Ok(<Self as ValueIndex<Cow<'static,str>>>::index(self,str_.into()))
            }
            _ => return Err(mlua::Error::RuntimeError(format!("Cannot index a rust object with {:?}", index))),
        }
    }
}

/// For internal use only.
/// 
/// Mainly necessary for separation of concerns on the [`ScriptRef`] type, but might have other uses potentially.
/// 
/// This is not the same as [`LuaProxyable`], internally this in fact will use [`LuaProxyable`] so treating it like so will cause inifnite loops.
pub(crate) trait ApplyLua {
    /// set the proxied object with the given lua value
    fn apply_lua<'lua>(&mut self, ctx: &'lua Lua, v: Value<'lua>) -> mlua::Result<()>;
}
impl ApplyLua for ScriptRef{
    /// Applies the given lua value to the proxied reflect type. Semantically equivalent to `Reflect::apply`
    fn apply_lua<'lua>(&mut self, ctx: &'lua Lua, v: Value<'lua>) -> Result<(),mlua::Error> {
        let luaworld = ctx.globals()
                        .get::<_, LuaWorld>("world")
                        .unwrap();

        // remove typedata from the world to be able to manipulate world 
        let proxyable = {
            let world = luaworld.upgrade().unwrap();
            let world = &world.read();
            let type_registry = world.resource::<TypeRegistry>().read();
            type_registry.get_type_data::<ReflectLuaProxyable>(self.get(|s| s.type_id())?).cloned()
        };

        if let Some(ud) = proxyable{
            return ud.apply_lua(self,ctx, v)
        } else if let Value::UserData(v) = &v{
            if v.is::<ReflectedValue>() {
                let b = v.take::<ReflectedValue>().unwrap();
                self.apply(&b.into());
                return Ok(())
            }
        }

        Err(mlua::Error::RuntimeError(self.get(|s| 
            format!("Attempted to assign `{}` = {v:?}. Did you forget to call `app.register_foreign_lua_type::<{}>`?",
                self.path.to_string(),
                s.type_name()
            ))?)
        )

    }
}

impl <'lua>ToLua<'lua> for ScriptRef {
    /// Converts the LuaRef to the most convenient representation
    /// checking conversions in this order:
    /// - A primitive or bevy type which has a reflect interface is converted to a custom UserData exposing its API to lua conveniently
    /// - A type implementing CustomUserData is converted with its `ref_to_lua` method
    /// - Finally the method is represented as a `ReflectedValue` which exposes the Reflect interface 
    fn to_lua(self, ctx: &'lua Lua) -> mlua::Result<Value<'lua>> {
        let luaworld = ctx.globals()
            .get::<_, LuaWorld>("world")
            .unwrap();

        let world = luaworld.upgrade().unwrap();
        let world = &mut world.read();

        let typedata = world.resource::<TypeRegistry>();
        let g = typedata.read();

        let type_id = self.get(|s| s.type_id())?;
        if let Some(v) = g.get_type_data::<ReflectLuaProxyable>(type_id) {
            Ok(v.ref_to_lua(self,ctx)?)
        } else {
            ReflectedValue{ref_: self.clone()}.to_lua(ctx)
        }
    }
}

impl TypeName for ScriptRef {
    /// We represent LuaRef types as `any` wildcards, since they can convert to literally anything 
    fn get_type_parts() -> Cow<'static, [tealr::NamePart]> {
        Cow::Borrowed(&[tealr::NamePart::Type(tealr::TealType {
            name: Cow::Borrowed("any"),
            generics: None,
            type_kind: tealr::KindOfType::Builtin,
        })])     
    }
}

impl_tealr_type!(ReflectedValue);
impl TealData for ReflectedValue {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_, val, ()| {
            val.ref_.get(|s| 
                Ok(format!("{:#?}", &s))
            )?
        });

        methods.add_meta_method_mut(MetaMethod::Index, |_, val, field: Value| {
            let r = val.ref_.index(field).unwrap();
            Ok(r)
        });

        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |ctx, val, (field, new_val): (Value, Value)| {
                val.ref_.index(field).unwrap().apply_lua(ctx, new_val).unwrap();
                Ok(())
            },
        );

        methods.add_meta_method(MetaMethod::Len, |_, val, ()| {
            val.ref_.get(|s| {
                let r = s.reflect_ref();
                if let ReflectRef::List(v) = r {
                    Ok(v.len())
                } else if let ReflectRef::Map(v) = r {
                    Ok(v.len())
                } else if let ReflectRef::Tuple(v) = r {
                    Ok(v.field_len())
                } else {
                    panic!("No length on this type");
                }
            })?
        });

    }

}
/// A higher level trait for allowing types to be interpreted as custom lua proxy types (or just normal types, this interface is flexible).
/// Types implementing this trait can have [`ReflectLuaProxyable`] type data registrations inserted into the reflection API.
/// 
/// Types registered via the reflection API this way can be accessed from Lua via [`ScriptRef`] objects (via field access).
pub trait LuaProxyable : {
    /// a version of [`mlua::ToLua::to_lua`] which does not consume the object.
    /// 
    /// Note: The self reference is sourced from the given ScriptRef, attempting to get another mutable reference from the ScriptRef might
    /// cause a runtime error to prevent breaking of aliasing rules
    fn ref_to_lua<'lua>(self_ : ScriptRef, lua: &'lua Lua) -> mlua::Result<Value<'lua>>;

    /// similar to [`Reflect::apply`]
    /// 
    /// Note: 
    /// The self reference is sourced from the given ScriptRef, attempting to get another reference from the ScriptRef might
    /// cause a runtime error to prevent breaking of aliasing rules
    fn apply_lua<'lua>(self_ : &mut ScriptRef, lua: &'lua Lua, new_val: Value<'lua>) -> mlua::Result<()>;
}


/// Exactly alike to [`mlua::ToLua`] 
pub trait FromLuaProxy<'lua> : Sized{
    fn from_lua_proxy(new_val : Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self>;
}

/// Exactly alike to [`mlua::FromLua`]
pub trait ToLuaProxy<'lua>  {
    fn to_lua_proxy(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>>;
}

/// A struct providing type data for the `LuaProxyable` trait.
/// 
/// This allows casting static methods from the `LuaProxyable trait`.
#[derive(Clone)]
pub struct ReflectLuaProxyable {
    ref_to_lua: for<'lua> fn(ref_ : ScriptRef, lua: &'lua Lua) -> mlua::Result<Value<'lua>>,
    apply_lua: for<'lua> fn(ref_ : &mut ScriptRef, lua: &'lua Lua, new_val: Value<'lua>) -> mlua::Result<()>,
}

impl ReflectLuaProxyable {
    pub fn ref_to_lua<'lua>(
        &self,
        ref_ : ScriptRef, lua: &'lua Lua
    ) -> mlua::Result<Value<'lua>> {
        (self.ref_to_lua)(ref_,lua)
    }

    pub fn apply_lua<'lua>(
        &self,
        ref_ : &mut ScriptRef, lua: &'lua Lua, new_val: Value<'lua>
    ) -> mlua::Result<()> {
        (self.apply_lua)(ref_, lua, new_val)
    }
}


impl<T: LuaProxyable + ::bevy::reflect::Reflect> ::bevy::reflect::FromType<T> for ReflectLuaProxyable {
    fn from_type() -> Self {
        Self {
            ref_to_lua: T::ref_to_lua,
            apply_lua: T::apply_lua
        }
    }
}

/// A dummy trait used to combat rust's orphan rules 
/// 
/// In the future when trait specialization is a thing, this might be a companion trait
/// to `RefLuaType` which allows non Clone types to be used
pub trait ValueLuaType {}

impl<T: Clone + UserData + Send + ValueLuaType + Reflect + 'static> LuaProxyable for T {
    fn ref_to_lua<'lua>(self_ : ScriptRef, lua: &'lua Lua) -> mlua::Result<Value<'lua>>{
        self_.get_typed(|s: &Self| s.clone().to_lua(lua))?
    }

    fn apply_lua<'lua>(self_ : &mut ScriptRef, _: &'lua Lua, new_val: Value<'lua>) -> mlua::Result<()>{
        if let Value::UserData(v) = new_val {
            let o = v.borrow::<T>()?;

            self_.get_mut_typed(|s| {
                *s = o.clone()
            });
            
            Ok(())
        } else {
            Err(mlua::Error::RuntimeError(
                "Error in assigning to custom user data".to_owned(),
            ))
        }
    }
}

impl<'lua,T: Clone + UserData + Send + ValueLuaType + Reflect + 'static> FromLuaProxy<'lua> for T {
    fn from_lua_proxy(new_val : Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        T::from_lua(new_val,lua)
    }
}


impl<'lua,T: Clone + UserData + Send + ValueLuaType + Reflect + 'static> ToLuaProxy<'lua> for T {
    fn to_lua_proxy(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        self.to_lua(lua)
    }
}


