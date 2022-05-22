use anyhow::{anyhow, Result};
use bevy::reflect::*;
use rlua::{Context, Value};
use std::{fmt, any::TypeId};

use phf::{phf_map, Map};

use crate::{primitives::LuaNumber, LuaWorld, ReflectCustomUserData, PrintReflect, util::PrintableReflect, APPLY_LUA_TO_BEVY, BEVY_TO_LUA};

/// A rust type representation in lua
#[derive(Clone)]
pub struct LuaRef(pub(crate) *mut (dyn Reflect + 'static));

impl fmt::Debug for LuaRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().print(f)
    }
}

impl fmt::Display for LuaRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl LuaRef {
    pub fn get(&self) -> &dyn Reflect {
        assert!(!self.0.is_null());
        unsafe { &*self.0 }
    }

    pub fn get_mut(&mut self) -> &mut dyn Reflect {
        assert!(!self.0.is_null());
        unsafe { &mut *self.0 }
    }

    pub fn path_ref(&self, path: &str) -> Result<Self> {
        let ref_mut = self.get();

        let re = ref_mut
            .path(path)
            .map_err(|_e| anyhow!("Cannot access field `{}`", path))?;
        Ok(Self(re as *const dyn Reflect as *mut dyn Reflect))
    }

    pub fn path_lua_val_ref(&self, path: Value) -> Result<Self> {
        let r = self.get().reflect_ref();

        match path {
            Value::Integer(idx) => {
                let idx = idx as usize - 1;
                match r {
                    ReflectRef::Tuple(v) => Ok(v.field(idx).unwrap()),
                    ReflectRef::TupleStruct(v) => Ok(v.field(idx).unwrap()),
                    ReflectRef::List(v) => Ok(v.get(idx).unwrap()),
                    ReflectRef::Map(v) => Ok(v.get(&(idx)).unwrap()),
                    _ => Err(anyhow!("Tried to index a primitive rust type {:#?}", self)),
                }
            }
            Value::String(field) => {
                let path = field.to_str().unwrap();
                match r {
                    ReflectRef::Map(v) => Ok(v.get(&path.to_owned()).unwrap()),
                    ReflectRef::Struct(v) => Ok(v.field(path).unwrap()),
                    _ => Err(anyhow!("Tried to index a primitive rust type {:#?}", self)),
                }
            }
            _ => Err(anyhow!("Cannot index a rust object with {:?}", path)),
        }
        .map(|v| LuaRef(v as *const dyn Reflect as *mut dyn Reflect))
    }

    pub fn convert_to_lua<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        
        if let Some(f) = BEVY_TO_LUA.get(self.get().type_name()){
            Ok(f(self.get(),ctx))
        } else if let Some(c) = REFLECT_TO_LUA_CONVERSIONS.get(self.get().type_name()) {
            Ok(c(self.get(), ctx))
        } else {
            let w = unsafe { &mut *(ctx.globals().get::<_, LuaWorld>("world").unwrap()).0 };
            let typedata = w.resource::<TypeRegistry>();

            let g = typedata.read();
            if let Some(v) = g.get_type_data::<ReflectCustomUserData>(self.get().type_id()) {
                Ok(v.get(self.get()).unwrap().ref_to_lua(ctx).unwrap())
            } else {
                Ok(Value::UserData(ctx.create_userdata(self).unwrap()))
            }
        } 
        // another case for an enumeration of 
    }

    pub fn apply_lua<'lua>(&mut self, ctx: Context<'lua>, v: Value<'lua>) -> Result<()> {
        let w = unsafe { &mut *(ctx.globals().get::<_, LuaWorld>("world").unwrap()).0 };
        let typedata = w.resource::<TypeRegistry>();
        let g = typedata.read();

        if let Some(f) = APPLY_LUA_TO_BEVY.get(self.get().type_name()) {
            f(self.get_mut(),ctx,v);
            Ok(())
        } else if let Some(ud) = g.get_type_data::<ReflectCustomUserData>(self.get().type_id()) {
            ud.get_mut(self.get_mut())
                .unwrap()
                .apply_lua(ctx, v)
                .unwrap();
            Ok(())
        } else {
            match v {
                Value::Boolean(b) => LuaNumber::Usize(b as usize).reflect_apply(self.get_mut()),
                Value::Integer(i) => LuaNumber::I64(i).reflect_apply(self.get_mut()),
                Value::Number(n) => LuaNumber::F64(n).reflect_apply(self.get_mut()),
                Value::String(v) => {
                    self.get_mut().apply(&v.to_str().unwrap().to_owned());
                    Ok(())
                }
                Value::UserData(v) => {
                    // can only set a field to another field or primitive
                    if v.is::<LuaRef>() {
                        let b = v.borrow_mut::<LuaRef>().unwrap();
                        self.get_mut().apply(b.get());
                        Ok(())
                    } else {
                        return Err(anyhow!(""));
                    }
                }
                _ => return Err(anyhow!("Type not supported")),
            }
        }
    }
}

unsafe impl Send for LuaRef {}

/// Jump table for numeric conversions to lua
pub static REFLECT_TO_LUA_CONVERSIONS: Map<
    &'static str,
    for<'l> fn(&dyn Reflect, ctx: Context<'l>) -> Value<'l>,
> = phf_map! {
        "usize" => |r,_| Value::Integer( *r.downcast_ref::<usize>().unwrap() as i64) ,
        "isize" => |r,_| Value::Integer(*r.downcast_ref::<isize>().unwrap() as i64) ,
        "i64" => |r,_| Value::Integer(*r.downcast_ref::<i64>().unwrap() as i64) ,
        "i32" => |r,_| Value::Integer(*r.downcast_ref::<i32>().unwrap() as i64) ,
        "u32" => |r,_| Value::Integer(*r.downcast_ref::<u32>().unwrap() as i64) ,
        "u16" => |r,_| Value::Integer(*r.downcast_ref::<u16>().unwrap() as i64) ,
        "i16" => |r,_| Value::Integer(*r.downcast_ref::<i16>().unwrap() as i64) ,
        "u8" => |r,_| Value::Integer(*r.downcast_ref::<u8>().unwrap() as i64) ,
        "i8" => |r,_| Value::Integer(*r.downcast_ref::<i8>().unwrap() as i64) ,
        "f32" => |r,_| Value::Number(*r.downcast_ref::<f32>().unwrap() as f64) ,
        "f64" => |r,_| Value::Number(*r.downcast_ref::<f64>().unwrap() as f64)
};



