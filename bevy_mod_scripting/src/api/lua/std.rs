use ::std::borrow::Cow;
use std::iter;
use ::std::marker::PhantomData;
use ::std::mem::MaybeUninit;
use ::std::sync::{Weak,Arc};
use ::std::convert::AsRef;
use ::std::ops::{Deref,DerefMut};
use crate::{LuaProxyable,ScriptRef, ReflectedValue, impl_tealr_type, ReflectBase, ReflectPtr, ValueIndex, LuaWrapper, impl_user_data, FromLuaProxy, ApplyLua, ReflectPathElem};
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
macro_rules! impl_proxyable_by_copy(
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

                impl <'lua>FromLuaProxy<'lua> for $num_ty {
                    #[inline(always)]
                    fn from_lua_proxy(new_value: Value<'lua>, lua: &'lua Lua) -> tealr::mlu::mlua::Result<Self> {
                        Self::from_lua(new_value,lua)
                    }
                }
            )*
        }
    }  
);


impl_proxyable_by_copy!(bool);
impl_proxyable_by_copy!(f32,f64);
impl_proxyable_by_copy!(i8,i16,i32,i64,i128,isize);
impl_proxyable_by_copy!(u8,u16,u32,u64,u128,usize);

impl LuaProxyable for String {
    fn ref_to_lua<'lua>(self_: ScriptRef,lua: & 'lua Lua) -> mlua::Result<Value< 'lua> >  {
        self_.get_typed(|self_ : &String| self_.as_str().to_lua(lua))
    }

    fn apply_lua<'lua>(self_: &mut ScriptRef,lua: & 'lua Lua,new_val:Value< 'lua>) -> mlua::Result<()>  {
        self_.get_mut_typed(|self_| Ok(*self_ = Self::from_lua(new_val,lua)?))
    }
}

impl <'lua>FromLuaProxy<'lua> for String {
    fn from_lua_proxy(new_val : Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        Self::from_lua(new_val, lua)
    }
}

use std::fmt::Debug;
impl <T : LuaProxyable + Reflect + for <'a> Deserialize<'a> + serde::Serialize + Debug+ for<'a>FromLuaProxy<'a> + Clone>LuaProxyable for Option<T>{
    fn ref_to_lua< 'lua>(self_: ScriptRef,lua: & 'lua Lua) -> mlua::Result<Value< 'lua>>  {
        self_.get_typed(|s : &Option<T>| match  s {
            Some(_) => T::ref_to_lua(self_.sub_ref(ReflectPathElem::SubReflection{
                    label: "as_ref",
                    get: |ref_| 
                        ref_.downcast_ref::<Option<T>>()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                    get_mut: |ref_| 
                        ref_.downcast_mut::<Option<T>>()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                    })
                ,lua ),
            None => Ok(Value::Nil),
        })
    }

    fn apply_lua< 'lua>(self_: &mut ScriptRef,lua: & 'lua Lua,new_val:Value< 'lua>) -> mlua::Result<()>  {
        if let Value::Nil = new_val {
            self_.get_mut_typed(|s : &mut Option<T>| Ok(*s = None))
        } else {
            // we need to do this in two passes, first 
            // ensure that the target type is the 'some' variant to allow a sub reference
            match self_.get_mut_typed(|s : &mut Option<T>| {
                if s.is_none() {
                    *s = Some(T::from_lua_proxy(new_val.clone(),lua)?);
                    Ok::<_,mlua::Error>(true)
                } else {
                    Ok(false)
                }
            }){
                Ok(true) => return Ok(()),
                Ok(false) => {},
                Err(e) => return Err(e),
            }

            T::apply_lua(
                &mut self_.sub_ref(ReflectPathElem::SubReflection { 
                    label: "",
                    get: |ref_| 
                        ref_.downcast_ref::<Option<T>>()
                            .unwrap()
                            .as_ref()
                            .unwrap(),
                    get_mut: |ref_| 
                        ref_.downcast_mut::<Option<T>>()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                })
                ,lua, 
                new_val)
            
        }
    }
}

impl <'lua,T : for<'a>FromLuaProxy<'a>>FromLuaProxy<'lua> for Option<T> {
    fn from_lua_proxy(new_val : Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        if let Value::Nil = new_val {
            Ok(None)
        } else {
            T::from_lua_proxy(new_val, lua).map(Option::Some)
        }
    }
}

/// A reference to a rust vec, does not need an owned variant since 
/// lua can natively represent lists of things
pub struct LuaVec<T>{
    ref_: ScriptRef,
    _ph: PhantomData<T>
}

impl <T>LuaVec<T> {
    pub fn new_ref(ref_ : ScriptRef) -> Self{
        Self {
            ref_,
            _ph: PhantomData,
        }
    }
}

impl <T : FromReflect>TypeName for LuaVec<T> {
    fn get_type_parts() -> Cow<'static, [tealr::NamePart]> {
        Default::default()
    }
}

impl_user_data!(LuaVec<T : FromReflect + LuaProxyable>);
impl <T : FromReflect + LuaProxyable>TealData for LuaVec<T> {
    fn add_methods<'lua, M: TealDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |_,s,index : usize|{
           Ok(s.ref_.index(index)?)
        });

        methods.add_meta_method(MetaMethod::NewIndex, |ctx,s,(index,value) : (usize, Value)|{
            Ok(s.ref_.index(index)?.apply_lua(ctx, value)?)
         });
    }
}

impl <T : FromReflect + LuaProxyable + for<'a>FromLuaProxy<'a> + Debug>LuaProxyable for Vec<T> {
    fn ref_to_lua<'lua>(self_ : ScriptRef, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        LuaVec::<T>::new_ref(self_).to_lua(lua)
    }

    fn apply_lua<'lua>(self_ : &mut ScriptRef, lua: &'lua Lua, new_val: Value<'lua>) -> mlua::Result<()> {
        // general idea TODO: in case two script refs are identical we don't need to go through this method at all

        match &new_val {
            Value::UserData(ud) => {
                let lua_vec = ud.borrow::<LuaVec<T>>()?;
                self_.apply(&lua_vec.ref_);
            },
            Value::Table(table) => {    

                let target_len = self_.get_typed(|s : &Vec<T>| s.len());
                // there is also another case to consider, Vec has a lua representation available as well (table)
                // if we receive one of those, we should also apply it
                for entry in table.clone().pairs::<usize,Value>() {
                    let (lua_idx,v) = entry?;
                    let idx = lua_idx - 1;
                    if lua_idx > target_len {
                        // here we don't need to do anything special just use LuaProxyable impl
                        T::apply_lua(&mut self_.index(idx)?, lua, v)?;
                    } else {
                        // here we don't have anything to apply this to
                        // use FromLua impl
                        self_.get_mut_typed(|s : &mut Vec<T>| 
                            Ok::<_,mlua::Error>(s[idx] = T::from_lua_proxy(v, lua)?)
                        )?;
                    }
                }
            }
            _ => return Err(mlua::Error::FromLuaConversionError { 
                from: new_val.type_name(), 
                to: "userdata or table", 
                message: Some("LuaVec can only be assigned with itself or a table".to_owned()) 
            })
        }

        Ok(())
    }
} 

impl <'lua,T : for<'a>FromLuaProxy<'a> + Clone + FromReflect + LuaProxyable>FromLuaProxy<'lua> for Vec<T> {
    fn from_lua_proxy(new_val : Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        match &new_val {
            Value::UserData(ud) => {
                let lua_vec = ud.borrow::<LuaVec<T>>()?;
                Ok(lua_vec.ref_.get_typed(|s : &Vec<T>| s.clone()))
            },
            Value::Table(table) => {    

                let mut out = Vec::default(); 
                // there is also another case to consider, Vec has a lua representation available as well (table)
                // if we receive one of those, we should clone it one by one
                for entry in table.clone().pairs::<usize,Value>() {
                    let (_,v) = entry?;

                    out.push(T::from_lua_proxy(v, lua)?);
                }

                Ok(out)
            }
            _ => return Err(mlua::Error::FromLuaConversionError { 
                from: new_val.type_name(), 
                to: "userdata or table", 
                message: Some("LuaVec can only be assigned with itself or a table".to_owned()) 
            })
        }
    }
}