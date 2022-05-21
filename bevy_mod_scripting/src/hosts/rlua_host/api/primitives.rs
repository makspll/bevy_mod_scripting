use bevy::{reflect::Reflect, prelude::Handle, asset::Asset};
use num::{ToPrimitive,Num};
use anyhow::{anyhow,Result};
use rlua::{UserData, Value};

use crate::{CustomUserData, LuaFile};



/// Represents a Rust numeric type stored in lua
/// Necessary to retain precision and assign to various numeric rust via scripts
#[derive(Debug,Clone,Copy)]
pub enum LuaNumber {
    Usize(usize),
    Isize(isize),
    I64(i64),
    I32(i32),
    U32(u32),
    U16(u16),
    I16(i16),
    U8(u8),
    I8(i8),
    F32(f32),
    F64(f64)
}

impl ToPrimitive for LuaNumber {
    fn to_i64(&self) -> Option<i64> {
        match self {
            LuaNumber::Usize(v) => (*v).try_into().ok(),
            LuaNumber::Isize(v) => (*v).try_into().ok(),
            LuaNumber::I64(v) => (*v).try_into().ok(),
            LuaNumber::I32(v) => (*v).try_into().ok(),
            LuaNumber::U32(v) => (*v).try_into().ok(),
            LuaNumber::U16(v) => (*v).try_into().ok(),
            LuaNumber::I16(v) => (*v).try_into().ok(),
            LuaNumber::U8(v) => (*v).try_into().ok(),
            LuaNumber::I8(v) => (*v).try_into().ok(),
            LuaNumber::F32(v) => Some((*v) as i64),
            LuaNumber::F64(v) => Some((*v) as i64),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            LuaNumber::Usize(v) => (*v).try_into().ok(),
            LuaNumber::Isize(v) => (*v).try_into().ok(),
            LuaNumber::I64(v) => (*v).try_into().ok(),
            LuaNumber::I32(v) => (*v).try_into().ok(),
            LuaNumber::U32(v) => (*v).try_into().ok(),
            LuaNumber::U16(v) => (*v).try_into().ok(),
            LuaNumber::I16(v) => (*v).try_into().ok(),
            LuaNumber::U8(v) => (*v).try_into().ok(),
            LuaNumber::I8(v) => (*v).try_into().ok(),
            LuaNumber::F32(v) => Some((*v) as u64),
            LuaNumber::F64(v) => Some((*v) as u64),
        }
    }
}

impl LuaNumber {

    pub fn from_reflect(r : &dyn Reflect) -> Result<Self> {
        if let Some(v) = r.downcast_ref::<usize>(){
            Ok(Self::Usize(*v))
        } else if let Some(v) = r.downcast_ref::<isize>(){
            Ok(Self::Isize(*v))
        } else if let Some(v) = r.downcast_ref::<u32>(){
            Ok(Self::U32(*v))
        } else if let Some(v) = r.downcast_ref::<i32>(){
            Ok(Self::I32(*v))
        } else if let Some(v) = r.downcast_ref::<i64>(){
            Ok(Self::I64(*v))
        } else if let Some(v) = r.downcast_ref::<u16>(){
            Ok(Self::U16(*v))
        } else if let Some(v) = r.downcast_ref::<i16>(){
            Ok(Self::I16(*v))
        } else if let Some(v) = r.downcast_ref::<u8>(){
            Ok(Self::U8(*v))
        } else if let Some(v) = r.downcast_ref::<i8>(){
            Ok(Self::I8(*v))
        } else if let Some(v) = r.downcast_ref::<f32>(){
            Ok(Self::F32(*v))
        } else if let Some(v) = r.downcast_ref::<f64>(){
            Ok(Self::F64(*v))
        } else {
            Err(anyhow!("Wrong type"))
        }
    } 
    
    pub fn reflect_apply(self, r : &mut dyn Reflect) -> Result<()> {
        Ok(match self.as_type(r.type_name())? {
            LuaNumber::Usize(v) => r.apply(&v),
            LuaNumber::Isize(v) => r.apply(&v),
            LuaNumber::I64(v) => r.apply(&v),
            LuaNumber::I32(v) => r.apply(&v),
            LuaNumber::U32(v) => r.apply(&v),
            LuaNumber::U16(v) => r.apply(&v),
            LuaNumber::I16(v) => r.apply(&v),
            LuaNumber::U8(v) => r.apply(&v),
            LuaNumber::I8(v) => r.apply(&v),
            LuaNumber::F32(v) => r.apply(&v),
            LuaNumber::F64(v) => r.apply(&v),
        })
    }

    pub fn add(&self, o: LuaNumber) -> Result<LuaNumber> {
        match (self,o){
            (LuaNumber::Usize(l),LuaNumber::Usize(r)) => l.checked_add(r).map(LuaNumber::Usize),
            (LuaNumber::Isize(l),LuaNumber::Isize(r)) => l.checked_add(r).map(LuaNumber::Isize),
            (LuaNumber::I64(l),LuaNumber::I64(r))         => l.checked_add(r).map(LuaNumber::I64),
            (LuaNumber::I32(l),LuaNumber::I32(r))         => l.checked_add(r).map(LuaNumber::I32),
            (LuaNumber::U32(l),LuaNumber::U32(r))         => l.checked_add(r).map(LuaNumber::U32),
            (LuaNumber::U16(l),LuaNumber::U16(r))         => l.checked_add(r).map(LuaNumber::U16),
            (LuaNumber::I16(l),LuaNumber::I16(r))         => l.checked_add(r).map(LuaNumber::I16),
            (LuaNumber::U8(l),LuaNumber::U8(r))             => l.checked_add(r).map(LuaNumber::U8),
            (LuaNumber::I8(l),LuaNumber::I8(r))             => l.checked_add(r).map(LuaNumber::I8),
            (LuaNumber::F32(l),LuaNumber::F32(r))         => Some(LuaNumber::F32(l + r)),
            (LuaNumber::F64(l),LuaNumber::F64(r))         => Some(LuaNumber::F64(l + r)),
            _ => None 
        }.ok_or(anyhow!("Cannot perform addition, numbers are not of the same type. For {:#?} and {:#?}",self,o))
    }

    pub fn sub(&self, o: LuaNumber) -> Result<LuaNumber> {
        match (self,o){
            (LuaNumber::Usize(l),LuaNumber::Usize(r)) => l.checked_sub(r).map(LuaNumber::Usize),
            (LuaNumber::Isize(l),LuaNumber::Isize(r)) => l.checked_sub(r).map(LuaNumber::Isize),
            (LuaNumber::I64(l),LuaNumber::I64(r))         => l.checked_sub(r).map(LuaNumber::I64),
            (LuaNumber::I32(l),LuaNumber::I32(r))         => l.checked_sub(r).map(LuaNumber::I32),
            (LuaNumber::U32(l),LuaNumber::U32(r))         => l.checked_sub(r).map(LuaNumber::U32),
            (LuaNumber::U16(l),LuaNumber::U16(r))         => l.checked_sub(r).map(LuaNumber::U16),
            (LuaNumber::I16(l),LuaNumber::I16(r))         => l.checked_sub(r).map(LuaNumber::I16),
            (LuaNumber::U8(l),LuaNumber::U8(r))             => l.checked_sub(r).map(LuaNumber::U8),
            (LuaNumber::I8(l),LuaNumber::I8(r))             => l.checked_sub(r).map(LuaNumber::I8),
            (LuaNumber::F32(l),LuaNumber::F32(r))         => Some(LuaNumber::F32(l + r)),
            (LuaNumber::F64(l),LuaNumber::F64(r))         => Some(LuaNumber::F64(l + r)),
            _ => None 
        }.ok_or(anyhow!("Cannot perform subtraction, numbers are not of the same type. For {:#?} and {:#?}",self,o))    }

    pub fn mul(&self, o: LuaNumber) -> Result<LuaNumber> {
        match (self,o){
            (LuaNumber::Usize(l),LuaNumber::Usize(r)) => l.checked_mul(r).map(LuaNumber::Usize),
            (LuaNumber::Isize(l),LuaNumber::Isize(r)) => l.checked_mul(r).map(LuaNumber::Isize),
            (LuaNumber::I64(l),LuaNumber::I64(r))         => l.checked_mul(r).map(LuaNumber::I64),
            (LuaNumber::I32(l),LuaNumber::I32(r))         => l.checked_mul(r).map(LuaNumber::I32),
            (LuaNumber::U32(l),LuaNumber::U32(r))         => l.checked_mul(r).map(LuaNumber::U32),
            (LuaNumber::U16(l),LuaNumber::U16(r))         => l.checked_mul(r).map(LuaNumber::U16),
            (LuaNumber::I16(l),LuaNumber::I16(r))         => l.checked_mul(r).map(LuaNumber::I16),
            (LuaNumber::U8(l),LuaNumber::U8(r))             => l.checked_mul(r).map(LuaNumber::U8),
            (LuaNumber::I8(l),LuaNumber::I8(r))             => l.checked_mul(r).map(LuaNumber::I8),
            (LuaNumber::F32(l),LuaNumber::F32(r))         => Some(LuaNumber::F32(l + r)),
            (LuaNumber::F64(l),LuaNumber::F64(r))         => Some(LuaNumber::F64(l + r)),
            _ => None 
        }.ok_or(anyhow!("Cannot perform multiplication, numbers are not of the same type. For {:#?} and {:#?}",self,o))    
    }

    pub fn div(&self, o: LuaNumber) -> Result<LuaNumber> {
        match (self,o){
            (LuaNumber::Usize(l),LuaNumber::Usize(r)) => l.checked_div(r).map(LuaNumber::Usize),
            (LuaNumber::Isize(l),LuaNumber::Isize(r)) => l.checked_div(r).map(LuaNumber::Isize),
            (LuaNumber::I64(l),LuaNumber::I64(r))         => l.checked_div(r).map(LuaNumber::I64),
            (LuaNumber::I32(l),LuaNumber::I32(r))         => l.checked_div(r).map(LuaNumber::I32),
            (LuaNumber::U32(l),LuaNumber::U32(r))         => l.checked_div(r).map(LuaNumber::U32),
            (LuaNumber::U16(l),LuaNumber::U16(r))         => l.checked_div(r).map(LuaNumber::U16),
            (LuaNumber::I16(l),LuaNumber::I16(r))         => l.checked_div(r).map(LuaNumber::I16),
            (LuaNumber::U8(l),LuaNumber::U8(r))             => l.checked_div(r).map(LuaNumber::U8),
            (LuaNumber::I8(l),LuaNumber::I8(r))             => l.checked_div(r).map(LuaNumber::I8),
            (LuaNumber::F32(l),LuaNumber::F32(r))         => Some(LuaNumber::F32(l + r)),
            (LuaNumber::F64(l),LuaNumber::F64(r))         => Some(LuaNumber::F64(l + r)),
            _ => None 
        }.ok_or(anyhow!("Cannot perform division, numbers are not of the same type. For {:#?} and {:#?}",self,o))    }

    pub fn rem(&self, o: LuaNumber) -> Result<LuaNumber> {
        match (self,o){
            (LuaNumber::Usize(l),LuaNumber::Usize(r)) => l.checked_rem(r).map(LuaNumber::Usize),
            (LuaNumber::Isize(l),LuaNumber::Isize(r)) => l.checked_rem(r).map(LuaNumber::Isize),
            (LuaNumber::I64(l),LuaNumber::I64(r))         => l.checked_rem(r).map(LuaNumber::I64),
            (LuaNumber::I32(l),LuaNumber::I32(r))         => l.checked_rem(r).map(LuaNumber::I32),
            (LuaNumber::U32(l),LuaNumber::U32(r))         => l.checked_rem(r).map(LuaNumber::U32),
            (LuaNumber::U16(l),LuaNumber::U16(r))         => l.checked_rem(r).map(LuaNumber::U16),
            (LuaNumber::I16(l),LuaNumber::I16(r))         => l.checked_rem(r).map(LuaNumber::I16),
            (LuaNumber::U8(l),LuaNumber::U8(r))             => l.checked_rem(r).map(LuaNumber::U8),
            (LuaNumber::I8(l),LuaNumber::I8(r))             => l.checked_rem(r).map(LuaNumber::I8),
            (LuaNumber::F32(l),LuaNumber::F32(r))         => Some(LuaNumber::F32(l + r)),
            (LuaNumber::F64(l),LuaNumber::F64(r))         => Some(LuaNumber::F64(l + r)),
            _ => None 
        }.ok_or(anyhow!("Cannot perform modulo, numbers are not of the same type. For {:#?} and {:#?}",self,o))    }

    pub fn pow(&self, o: LuaNumber) -> Result<LuaNumber> {
        match (self,o){
            (LuaNumber::Usize(l),LuaNumber::Usize(r)) => Ok(LuaNumber::Usize(l.pow((r) as u32))),
            (LuaNumber::Isize(l),LuaNumber::Isize(r)) => Ok(LuaNumber::Isize(l.pow((r) as u32))),
            (LuaNumber::I64(l),LuaNumber::I64(r))         => Ok(LuaNumber::I64(l.pow((r) as u32))),
            (LuaNumber::I32(l),LuaNumber::I32(r))         => Ok(LuaNumber::I32(l.pow((r) as u32))),
            (LuaNumber::U32(l),LuaNumber::U32(r))         => Ok(LuaNumber::U32(l.pow((r) as u32))),
            (LuaNumber::U16(l),LuaNumber::U16(r))         => Ok(LuaNumber::U16(l.pow((r) as u32))),
            (LuaNumber::I16(l),LuaNumber::I16(r))         => Ok(LuaNumber::I16(l.pow((r) as u32))),
            (LuaNumber::U8(l),LuaNumber::U8(r))             => Ok(LuaNumber::U8(l.pow((r) as u32))),
            (LuaNumber::I8(l),LuaNumber::I8(r))             => Ok(LuaNumber::I8(l.pow((r) as u32))),
            (LuaNumber::F32(l),LuaNumber::F32(r))         => Ok(LuaNumber::F32(l.powf(r))),
            (LuaNumber::F64(l),LuaNumber::F64(r))         => Ok(LuaNumber::F64(l.powf(r))),
            _ => return Err(anyhow!("Cannot perform exponentiation, numbers are not of the same type. For {:#?} and {:#?}",self,o))
        }
    }

    pub fn neg(&self) -> Result<LuaNumber> {
        match self{
            LuaNumber::Isize(l) => Ok(LuaNumber::Isize(-l)),
            LuaNumber::I64(l)   => Ok(LuaNumber::I64(-l)),
            LuaNumber::I32(l)   => Ok(LuaNumber::I32(-l)),
            LuaNumber::I16(l)   => Ok(LuaNumber::I16(-l)),
            LuaNumber::I8(l)    => Ok(LuaNumber::I8(-l)),
            LuaNumber::F32(l)   => Ok(LuaNumber::F32(-l)),
            LuaNumber::F64(l)   => Ok(LuaNumber::F64(-l)),
            _ => return Err(anyhow!("Cannot perform negation, number is not signed. For {:#?}",self))
        }
    }

    fn cast<V: ToPrimitive>(v: V, target : &str) -> Option<LuaNumber>{
        match target {
            "usize" => v.to_usize().map(LuaNumber::Usize),
            "isize" => v.to_isize().map(LuaNumber::Isize),
            "i64" => v.to_i64().map(LuaNumber::I64),
            "i32" => v.to_i32().map(LuaNumber::I32),
            "u32" => v.to_u32().map(LuaNumber::U32),
            "u16" => v.to_u16().map(LuaNumber::U16),
            "i16" => v.to_i16().map(LuaNumber::I16),
            "u8" => v.to_u8().map(LuaNumber::U8),
            "i8" => v.to_i8().map(LuaNumber::I8),
            "f32" => v.to_f32().map(LuaNumber::F32),
            "f64" => v.to_f64().map(LuaNumber::F64),
            _ => None
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            LuaNumber::Usize(_) => "usize",
            LuaNumber::Isize(_) => "isize",
            LuaNumber::I64(_) => "i64",
            LuaNumber::I32(_) => "i32",
            LuaNumber::U32(_) => "u32",
            LuaNumber::U16(_) => "u16",
            LuaNumber::I16(_) => "i16",
            LuaNumber::U8(_) => "u8",
            LuaNumber::I8(_) => "i8",
            LuaNumber::F32(_) => "f32",
            LuaNumber::F64(_) => "f64",
        }
    }

    pub fn as_type(&self, typ : &str) -> Result<LuaNumber>{
        match self {
            LuaNumber::Usize(v) => LuaNumber::cast(*v,typ),
            LuaNumber::Isize(v) => LuaNumber::cast(*v,typ),
            LuaNumber::I64(v) => LuaNumber::cast(*v,typ),
            LuaNumber::I32(v) => LuaNumber::cast(*v,typ),
            LuaNumber::U32(v) => LuaNumber::cast(*v,typ),
            LuaNumber::U16(v) => LuaNumber::cast(*v,typ),
            LuaNumber::I16(v) => LuaNumber::cast(*v,typ),
            LuaNumber::U8(v) => LuaNumber::cast(*v,typ),
            LuaNumber::I8(v) => LuaNumber::cast(*v,typ),
            LuaNumber::F32(v) => LuaNumber::cast(*v,typ),
            LuaNumber::F64(v) => LuaNumber::cast(*v,typ),
        }.ok_or(anyhow!("Could not cast {:#?} to {:#?}", self, typ))
    }

    pub fn from_lua(v : Value, expected_type: &'static str) -> Result<Self> {
        let lua_val = match v {
            Value::Nil => LuaNumber::Usize(0),
            Value::Boolean(v) => LuaNumber::Usize(v as usize),
            Value::Integer(v) => LuaNumber::I64(v),
            Value::Number(v) => LuaNumber::F64(v),
            Value::UserData(d) => *d.borrow::<LuaNumber>()?,
            _ => return Err(anyhow!(""))
        };

        if lua_val.type_name() != expected_type {
            lua_val.as_type(expected_type)
        } else {
            Ok(lua_val)
        }
    }
}

impl UserData for LuaNumber {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {

        methods.add_meta_method(rlua::MetaMethod::Add, |_,v,o : Value|{
            Ok(v.add(LuaNumber::from_lua(o,v.type_name()).unwrap()).unwrap())
        });
        methods.add_meta_method(rlua::MetaMethod::Sub, |_,v,o : Value|{
            Ok(v.sub(LuaNumber::from_lua(o,v.type_name()).unwrap()).unwrap())
        });
        methods.add_meta_method(rlua::MetaMethod::Mul, |_,v,o : Value|{
            Ok(v.mul(LuaNumber::from_lua(o,v.type_name()).unwrap()).unwrap())
        });
        methods.add_meta_method(rlua::MetaMethod::Div, |_,v,o : Value|{
            Ok(v.div(LuaNumber::from_lua(o,v.type_name()).unwrap()).unwrap())
        });
        methods.add_meta_method(rlua::MetaMethod::Mod, |_,v,o : Value|{
            Ok(v.rem(LuaNumber::from_lua(o,v.type_name()).unwrap()).unwrap())
        });
        methods.add_meta_method(rlua::MetaMethod::Pow, |_,v,o : Value|{
            Ok(v.pow(LuaNumber::from_lua(o,v.type_name()).unwrap()).unwrap())
        });

        methods.add_meta_method(rlua::MetaMethod::ToString, |_,v,()|{
            Ok(format!("{:?}",v))
        });

        methods.add_method_mut("as",|_,v,typ : String|{
            Ok(v.as_type(&typ).unwrap())
        })
    }
}


#[cfg(test)]
pub mod tests {
    use super::LuaNumber;

    fn assert_eq_lua(a: LuaNumber,b : LuaNumber){
        match (a,b) {
            (LuaNumber::Usize(l),LuaNumber::Usize(r)) => assert_eq!(l,r),
            (LuaNumber::Isize(l),LuaNumber::Isize(r)) =>  assert_eq!(l,r),
            (LuaNumber::I64(l),LuaNumber::I64(r))         =>  assert_eq!(l,r),
            (LuaNumber::I32(l),LuaNumber::I32(r))         =>  assert_eq!(l,r),
            (LuaNumber::U32(l),LuaNumber::U32(r))         =>  assert_eq!(l,r),
            (LuaNumber::U16(l),LuaNumber::U16(r))         =>  assert_eq!(l,r),
            (LuaNumber::I16(l),LuaNumber::I16(r))         =>  assert_eq!(l,r),
            (LuaNumber::U8(l),LuaNumber::U8(r))             => assert_eq!(l,r),
            (LuaNumber::I8(l),LuaNumber::I8(r))             => assert_eq!(l,r),
            (LuaNumber::F32(l),LuaNumber::F32(r))         =>  assert_eq!(l,r),
            (LuaNumber::F64(l),LuaNumber::F64(r))         =>  assert_eq!(l,r),
            _ => panic!("Values had different types {:?} {:?}",a,b), 
        }    
    }

    #[test]
    fn test_normal_ops(){
        assert_eq_lua(LuaNumber::Usize(0).add(LuaNumber::Usize(1)).unwrap(), LuaNumber::Usize(1));
        assert_eq_lua(LuaNumber::U32(2).sub(LuaNumber::U32(1)).unwrap(), LuaNumber::U32(1));
        assert_eq_lua(LuaNumber::U8(2).mul(LuaNumber::U8(1)).unwrap(), LuaNumber::U8(2));
        assert_eq_lua(LuaNumber::I8(4).div(LuaNumber::I8(2)).unwrap(), LuaNumber::I8(2));
        assert_eq_lua(LuaNumber::I64(3).rem(LuaNumber::I64(2)).unwrap(), LuaNumber::I64(1));
        assert_eq_lua(LuaNumber::U16(3).pow(LuaNumber::U16(2)).unwrap(), LuaNumber::U16(9));
        assert_eq_lua(LuaNumber::Isize(3).neg().unwrap(), LuaNumber::Isize(-3));
    }

    #[test]
    fn test_valid_casts(){
        assert_eq_lua(LuaNumber::Usize(0).as_type("isize").unwrap(),LuaNumber::Isize(0));
        assert_eq_lua(LuaNumber::Isize(0).as_type("usize").unwrap(),LuaNumber::Usize(0));

        assert_eq_lua(LuaNumber::U32(255).as_type("u8").unwrap(),LuaNumber::U8(255));
        assert_eq_lua(LuaNumber::U8(255).as_type("u32").unwrap(),LuaNumber::U32(255));

        assert_eq_lua(LuaNumber::F32(64.0).as_type("u8").unwrap(),LuaNumber::U8(64));
        assert_eq_lua(LuaNumber::U8(64).as_type("f32").unwrap(),LuaNumber::F32(64.0));

        assert_eq_lua(LuaNumber::F32(64.0).as_type("f64").unwrap(),LuaNumber::F64(64.0));
        assert_eq_lua(LuaNumber::F64(64.0).as_type("f32").unwrap(),LuaNumber::F32(64.0));
    }


    #[test]
    fn test_usize_overflows(){
        LuaNumber::Usize(usize::MAX).add(LuaNumber::Usize(1)).err();
        LuaNumber::Usize(usize::MIN).sub(LuaNumber::Usize(1)).err();
    }

    #[test]
    fn test_isize_overflows(){
        LuaNumber::Isize(isize::MAX).add(LuaNumber::Isize(1)).err();
        LuaNumber::Isize(isize::MIN).sub(LuaNumber::Isize(1)).err();
    }

    #[test]
    fn test_u32_overflows(){
        LuaNumber::U32(u32::MAX).add(LuaNumber::U32(1)).err();
        LuaNumber::U32(u32::MIN).sub(LuaNumber::U32(1)).err();
    }

    #[test]
    fn test_u16_overflows(){
        LuaNumber::U16(u16::MAX).add(LuaNumber::U16(1)).err();
        LuaNumber::U16(u16::MIN).sub(LuaNumber::U16(1)).err();
    }

    #[test]
    fn test_u8_overflows(){
        LuaNumber::U8(u8::MAX).add(LuaNumber::U8(1)).err();
        LuaNumber::U8(u8::MIN).sub(LuaNumber::U8(1)).err();
    }

    #[test]
    fn test_i32_overflows(){
        LuaNumber::I32(i32::MAX).add(LuaNumber::I32(1)).err();
        LuaNumber::I32(i32::MIN).sub(LuaNumber::I32(1)).err();
    }

    #[test]
    fn test_i64_overflows(){
        LuaNumber::I64(i64::MAX).add(LuaNumber::I64(1)).err();
        LuaNumber::I64(i64::MIN).sub(LuaNumber::I64(1)).err();
    }

    #[test]
    fn test_i16_overflows(){
        LuaNumber::I16(i16::MAX).add(LuaNumber::I16(1)).err();
        LuaNumber::I16(i16::MIN).sub(LuaNumber::I16(1)).err();
    }

    #[test]
    fn test_i8_overflows(){
        LuaNumber::I8(i8::MAX).add(LuaNumber::I8(1)).err();
        LuaNumber::I8(i8::MIN).sub(LuaNumber::I8(1)).err();
    }
}