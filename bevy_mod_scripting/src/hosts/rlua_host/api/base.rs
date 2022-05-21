use anyhow::{anyhow, Result};
use bevy::{prelude::info, reflect::*};
use rlua::{Context, Lua, ToLua, Value};
use std::{cell::UnsafeCell, fmt, ops::Deref, sync::Arc};

use phf::{phf_map, Map};

use crate::{
    primitives::LuaNumber, CustomUserData, LuaCustomUserData, LuaWorld, ReflectCustomUserData,
};

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
        write!(f, "{:?}", self)
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
            .map_err(|e| anyhow!("Cannot access field `{}`", path))?;
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
        if let Some(c) = REFLECT_TO_LUA_CONVERSIONS.get(self.get().type_name()) {
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
    }

    pub fn apply_lua<'lua>(&mut self, ctx: Context<'lua>, v: Value<'lua>) -> Result<()> {
        let w = unsafe { &mut *(ctx.globals().get::<_, LuaWorld>("world").unwrap()).0 };
        let typedata = w.resource::<TypeRegistry>();
        let g = typedata.read();

        if let Some(ud) = g.get_type_data::<ReflectCustomUserData>(self.get().type_id()) {
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

pub struct PrintableReflect<'a>(pub &'a dyn Reflect);

impl fmt::Debug for PrintableReflect<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.print(f)
    }
}

pub trait PrintReflect {
    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

macro_rules! impl_downcast_print_cases {
    ($v:ident,$fmt:ident,$f:ty,$($x:ty),*) => {
        {

            if let Some(i) = $v.downcast_ref::<$f>(){
                write!($fmt,"({:#?}){}",$v.type_name(),i)?;
            }
            $(
            else if let Some(i) = $v.downcast_ref::<$x>() {
                write!($fmt,"({:#?}){}",$v.type_name(),i)?;
            }
            )*
            else {
                write!($fmt,"({:#?})",$v.type_name())?;
            }
        }
    };
}

impl<T: Reflect + ?Sized> PrintReflect for &T {
    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.reflect_ref() {
            ReflectRef::Struct(s) => (0..s.field_len())
                .fold(f.debug_struct(s.type_name()), |mut b, i| {
                    b.field(
                        s.name_at(i).unwrap(),
                        &PrintableReflect(s.field_at(i).unwrap()),
                    );
                    b
                })
                .finish(),
            ReflectRef::Map(m) => m
                .iter()
                .fold(f.debug_map(), |mut b, (k, v)| {
                    b.entry(&PrintableReflect(k), &PrintableReflect(v));
                    b
                })
                .finish(),
            ReflectRef::TupleStruct(ts) => ts
                .iter_fields()
                .fold(f.debug_tuple(ts.type_name()), |mut b, i| {
                    b.field(&PrintableReflect(i));
                    b
                })
                .finish(),
            ReflectRef::Tuple(t) => t
                .iter_fields()
                .fold(f.debug_tuple(""), |mut b, i| {
                    b.field(&PrintableReflect(i));
                    b
                })
                .finish(),
            ReflectRef::List(l) => l
                .iter()
                .fold(f.debug_list(), |mut b, i| {
                    b.entry(&PrintableReflect(i));
                    b
                })
                .finish(),
            ReflectRef::Array(a) => a
                .iter()
                .fold(f.debug_list(), |mut b, i| {
                    b.entry(&PrintableReflect(i));
                    b
                })
                .finish(),
            ReflectRef::Value(v) => {
                impl_downcast_print_cases!(
                    v, f, usize, isize, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8, f32, f64,
                    String
                );
                Ok(())
            }
        }
    }
}

pub fn reflect_to_lua<'s, 'lua>(
    v: &'s dyn Reflect,
    ctx: rlua::Context<'lua>,
) -> Result<Value<'lua>> {
    match v.reflect_ref() {
        ReflectRef::List(l) => {
            let i = l
                .iter()
                .map(|v| reflect_to_lua(v, ctx))
                .collect::<Result<Vec<Value<'lua>>>>()?
                .into_iter()
                .enumerate();

            ctx.create_table_from(i)
                .map(|v| Value::Table(v))
                .map_err(|e| anyhow!(e.to_string()))
        }
        ReflectRef::Value(v) => {
            if let Some(c) = REFLECT_TO_LUA_CONVERSIONS.get(v.type_name()) {
                return Ok(c(v, ctx));
            } else if let Some(v) = v.downcast_ref::<String>() {
                return ctx
                    .create_string(v)
                    .map(|v| Value::String(v))
                    .map_err(|e| anyhow!("{}", e));
            } else {
                return Err(anyhow!(
                    "This type cannot be converted to a lua value: {:#?}",
                    PrintableReflect(v)
                ));
            }
        }
        ReflectRef::Struct(s) => Ok(Value::Nil),
        ReflectRef::TupleStruct(ts) => todo!(),
        ReflectRef::Tuple(t) => todo!(),
        ReflectRef::Map(m) => todo!(),
        ReflectRef::Array(_) => todo!(),
    }
}
