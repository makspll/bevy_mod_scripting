use ::std::borrow::Cow;
use ::std::marker::PhantomData;
use ::std::mem::MaybeUninit;
use ::std::sync::{Weak,Arc};
use ::std::convert::AsRef;
use ::std::ops::{Deref,DerefMut};
use crate::{LuaProxyable,ScriptRef, ReflectedValue, impl_tealr_type, ScriptRefBase, ReflectPtr, ValueIndex, IdentitySubReflect, SubReflect, LuaWrapper};
use bevy::ecs::system::Command;
use bevy::hierarchy::BuildWorldChildren;
use bevy::reflect::{ReflectRef, FromReflect};
use bevy::reflect::erased_serde::Serialize;
use bevy::{reflect::{reflect_trait, Reflect, TypeRegistry, TypeRegistration, DynamicStruct, DynamicTupleStruct, DynamicTuple, DynamicList, DynamicArray, DynamicMap}, prelude::{World, ReflectComponent, ReflectDefault, ReflectResource}, hierarchy::{Children, Parent, DespawnChildrenRecursive, DespawnRecursive}};

use parking_lot::RwLock;
use serde::Deserialize;
use tealr::TypeName;
use tealr::mlu::mlua::MetaMethod;
use tealr::mlu::{mlua::{Lua, Value,self, UserData, ToLua,FromLua}, TealData, TealDataMethods};


use paste::paste;


/// Implements custom user data for simple copy types which implement to and from lua
macro_rules! impl_copy_custom_user_data(
    ( $($num_ty:ty),*) => {
        paste! {
            $(
                impl LuaProxyable for $num_ty {
                    fn ref_to_lua< 'lua>(self_: crate::ScriptRef,lua: & 'lua tealr::mlu::mlua::Lua) -> tealr::mlu::mlua::Result<tealr::mlu::mlua::Value< 'lua> >  {
                        self_.get_typed(|self_ : &Self| self_.to_lua(lua))
                    }
                
                    fn apply_lua< 'lua>(self_: &mut crate::ScriptRef,lua: & 'lua tealr::mlu::mlua::Lua,new_val:tealr::mlu::mlua::Value< 'lua>) -> tealr::mlu::mlua::Result<()>  {
                        self_.set_val(Self::from_lua(new_val,lua)?);
                        Ok(())
                    }
                }
            )*
        }
    }  
);


impl_copy_custom_user_data!(bool);
impl_copy_custom_user_data!(f32,f64);
impl_copy_custom_user_data!(i8,i16,i32,i64,i128,isize);
impl_copy_custom_user_data!(u8,u16,u32,u64,u128,usize);

impl LuaProxyable for String {
    fn ref_to_lua<'lua>(self_: ScriptRef,lua: & 'lua Lua) -> mlua::Result<Value< 'lua> >  {
        self_.get_typed(|self_ : &String| self_.as_str().to_lua(lua))
    }

    fn apply_lua<'lua>(self_: &mut ScriptRef,lua: & 'lua Lua,new_val:Value< 'lua>) -> mlua::Result<()>  {
        self_.get_mut_typed(|self_,_| Ok(*self_ = Self::from_lua(new_val,lua)?))
    }
}

impl <T : LuaProxyable + Reflect + for <'a> Deserialize<'a> + serde::Serialize + Default + Clone>LuaProxyable for Option<T>{
    fn ref_to_lua< 'lua>(self_: ScriptRef,lua: & 'lua Lua) -> mlua::Result<Value< 'lua>>  {
        self_.get_typed(|s : &Option<T>| match  s {
            Some(v) => T::ref_to_lua(self_.sub_ref(
                    |ref_| 
                        ref_.downcast_ref::<Option<T>>()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                    |ref_| 
                        ref_.downcast_mut::<Option<T>>()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                    )
                ,lua ),
            None => Ok(Value::Nil),
        })
    }

    fn apply_lua< 'lua>(self_: &mut ScriptRef,lua: & 'lua Lua,new_val:Value< 'lua>) -> mlua::Result<()>  {
        if let Value::Nil = new_val {
            self_.get_mut_typed(|s : &mut Option<T>,_| Ok(*s = None))
        } else {
            // we need to do this in two passes, first 
            // ensure that the target type is the 'some' variant to allow a sub reference
            self_.get_mut_typed(|s : &mut Option<T>,_| {
                if s.is_none() {
                    *s = Some(T::default());
                }
            });

            T::apply_lua(
                &mut self_.sub_ref(
                    |ref_| 
                        ref_.downcast_ref::<Option<T>>()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                    |ref_| 
                        ref_.downcast_mut::<Option<T>>()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                    )
                ,lua, 
                new_val)
            
        }
    }
}

type LuaVec<T> = LuaWrapper<Vec<T>>;

impl <T : FromReflect>TypeName for LuaVec<T> {
    fn get_type_parts() -> Cow<'static, [tealr::NamePart]> {
        todo!()
    }
}

impl <T : FromReflect>TealData for LuaVec<T> {
    fn add_methods<'lua, M: TealDataMethods<'lua, Self>>(methods: &mut M) {

    }
}