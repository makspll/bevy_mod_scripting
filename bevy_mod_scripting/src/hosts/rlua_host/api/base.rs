use std::{fmt,sync::Arc, cell::UnsafeCell, ops::Deref};
use bevy::reflect::*;
use anyhow::{anyhow,Result};
use rlua::{Value, Lua, Context};

use phf::{phf_map,Map};

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
        write!(f,"{:?}",self)
    }
}


impl LuaRef {
    pub fn get(&self) -> &dyn Reflect { 
        assert!(!self.0.is_null());
        unsafe{ &*self.0 }
    }

    pub fn get_mut(&mut self) -> &mut dyn Reflect {
        assert!(!self.0.is_null());
        unsafe{ &mut *self.0 }
    }

    pub fn path_ref(&self, path: &str) -> Result<Self> {
        let ref_mut = self.get();

        let re = ref_mut.path(path).map_err(|e| anyhow!("Cannot access field `{}`",path))?;
        Ok(Self(re as *const dyn Reflect as *mut dyn Reflect))
    }

    pub fn path_lua_val_ref(&self, path: Value) -> Result<Self>{
        let r = self.get().reflect_ref();

        match path {
            Value::Integer(idx) => {
                let idx = idx as usize - 1;
                match r {
                    ReflectRef::Tuple(v) => Ok(v.field(idx).unwrap()),
                    ReflectRef::TupleStruct(v) => Ok(v.field(idx).unwrap()),
                    ReflectRef::List(v) => Ok(v.get(idx).unwrap()),
                    ReflectRef::Map(v) => Ok(v.get(&(idx)).unwrap()),
                    _ => Err(anyhow!("Tried to index a primitive rust type {:#?}",self))
                }
            },
            Value::String(field) => {
                let path = field.to_str().unwrap();
                match r {
                    ReflectRef::Map(v) => Ok(v.get(&path.to_owned()).unwrap()),
                    ReflectRef::Struct(v) => Ok(v.field(path).unwrap()),
                    _ => Err(anyhow!("Tried to index a primitive rust type {:#?}", self))
                }

            },
            _ => Err(anyhow!("Cannot index a rust object with {:?}", path))
        }.map(|v| LuaRef(v as *const dyn Reflect as *mut dyn Reflect))
    }
}

unsafe impl Send for LuaRef {}


/// Jump table for numeric conversions to lua
pub static REFLECT_TO_LUA_CONVERSIONS: Map<&'static str, for<'l> fn(&dyn Reflect, ctx: &Context<'l>)->Value<'l> > = phf_map! {
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


/// Jump table for conversions from lua
/// conversions are placed in destination directly if they are successful
// pub static LUA_TO_REFLECT_CONVERSIONS: Map<&'static str, for <'a> fn(Value<'a>, *const dyn Reflect)-> Result<()>> = phf_map! {
//     "usize" => |v,d| unsafe{  if let Value::Integer(v) = v {*(d as *mut usize) = v as usize; Ok(())} else if let Value::Number(v) = v {*(d as *mut usize) = v as usize;Ok(())} else {Err(anyhow!("Cannot convert {:#?} to numeric type",v))} },
//     "isize" => |v,d| unsafe{  if let Value::Integer(v) = v {*(d as *mut isize) = v as isize; Ok(())} else if let Value::Number(v) = v {*(d as *mut isize) = v as isize;Ok(())} else {Err(anyhow!("Cannot convert {:#?} to numeric type",v))} },
//     "i64" => |v,d| unsafe{  if let Value::Integer(v) = v {*(d as *mut i64) = v as i64; Ok(())} else if let Value::Number(v) = v {*(d as *mut i64) = v as i64;Ok(())} else {Err(anyhow!("Cannot convert {:#?} to numeric type",v))} },
//     "i32" => |v,d| unsafe{ if let Value::Integer(v) = v {*(d as *mut i32) = v as i32; Ok(())} else if let Value::Number(v) = v {*(d as *mut i32) = v as i32;Ok(())} else {Err(anyhow!("Cannot convert {:#?} to numeric type",v))} },
//     "u32" => |v,d| unsafe{  if let Value::Integer(v) = v {*(d as *mut u32) = v as u32; Ok(())} else if let Value::Number(v) = v {*(d as *mut u32) = v as u32;Ok(())} else {Err(anyhow!("Cannot convert {:#?} to numeric type",v))} },
//     "u16" => |v,d| unsafe{  if let Value::Integer(v) = v {*(d as *mut u16) = v as u16; Ok(())} else if let Value::Number(v) = v {*(d as *mut u16) = v as u16;Ok(())} else {Err(anyhow!("Cannot convert {:#?} to numeric type",v))} },   
//     "i16" => |v,d| unsafe{  if let Value::Integer(v) = v {*(d as *mut i16) = v as i16; Ok(())} else if let Value::Number(v) = v {*(d as *mut i16) = v as i16;Ok(())} else {Err(anyhow!("Cannot convert {:#?} to numeric type",v))} },
//     "u8" => |v,d| unsafe{  if let Value::Integer(v) = v {*(d as *mut u8) = v as u8; Ok(())} else if let Value::Number(v) = v {*(d as *mut u8) = v as u8;Ok(())} else {Err(anyhow!("Cannot convert {:#?} to numeric type",v))} },
//     "i8" => |v,d| unsafe{  if let Value::Integer(v) = v {*(d as *mut i8) = v as i8; Ok(())} else if let Value::Number(v) = v {*(d as *mut i8) = v as i8;Ok(())} else {Err(anyhow!("Cannot convert {:#?} to numeric type",v))} },
//     "f32" => |v,d| unsafe{  if let Value::Integer(v) = v {*(d as *mut f32) = v as f32; Ok(())} else if let Value::Number(v) = v {*(d as *mut f32) = v as f32;Ok(())} else {Err(anyhow!("Cannot convert {:#?} to numeric type",v))} },
//     "f64" => |v,d| unsafe{  if let Value::Integer(v) = v {*(d as *mut f64) = v as f64; Ok(())} else if let Value::Number(v) = v {*(d as *mut f64) = v as f64;Ok(())} else {Err(anyhow!("Cannot convert {:#?} to numeric type",v))}},
//     "bool" => |v,d| unsafe{  if let Value::Boolean(v) = v {*(d as *mut bool) = v;Ok(())} else {Err(anyhow!("Cannot convert {:#?} to bool type",v))}}
// };



pub struct PrintableReflect<'a>(pub &'a dyn Reflect);

impl fmt::Debug for PrintableReflect<'_>{
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

impl <T : Reflect + ?Sized> PrintReflect for &T {
    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.reflect_ref() {
            ReflectRef::Struct(s) => {
                (0..s.field_len()).fold(f.debug_struct(s.type_name()), |mut b, i| {
                    b.field(
                        s.name_at(i).unwrap(),
                        &PrintableReflect(s.field_at(i).unwrap()));
                    b
                }).finish()
            },
            ReflectRef::Map(m) => {
                m.iter().fold(f.debug_map(), |mut b, (k,v)| {
                    b.entry(
                        &PrintableReflect(k),
                        &PrintableReflect(v));
                    b
                }).finish()
            },
            ReflectRef::TupleStruct(ts) => {
                ts.iter_fields().fold(f.debug_tuple(ts.type_name()),|mut b, i|{
                    b.field(&PrintableReflect(i));
                    b
                }).finish()
            },
            ReflectRef::Tuple(t) => {
                t.iter_fields().fold(f.debug_tuple(""),|mut b, i|{
                    b.field(&PrintableReflect(i));
                    b
                }).finish()
            },
            ReflectRef::List(l) => {
                l.iter().fold(f.debug_list(), |mut b, i|{
                    b.entry(&PrintableReflect(i));
                    b
                }).finish()
            },
            ReflectRef::Array(a) => {
                a.iter().fold(f.debug_list(), |mut b, i|{
                    b.entry(&PrintableReflect(i));
                    b
                }).finish()
            },
            ReflectRef::Value(v) => {
                impl_downcast_print_cases!(v,f,
                    usize,
                    isize,
                    u128,
                    i128,
                    u64,
                    i64,
                    u32,
                    i32,
                    u16,
                    i16,
                    u8,
                    i8,
                    f32,
                    f64,
                    String);
                Ok(())
            },
            
        }

    }
} 
