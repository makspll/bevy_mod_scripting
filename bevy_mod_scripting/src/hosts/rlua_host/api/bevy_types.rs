#![allow(unused_variables,unused_parens)]
use bevy::reflect::TypeData;
use bevy::reflect::TypeRegistry;
use bevy::prelude::*;
use bevy::math::*;
use std::sync::Weak;
use std::{fmt,fmt::{Debug}, ops::*,sync::Mutex};
use phf::{phf_map, Map};
use crate::LuaFile;
use crate::LuaRefBase;
use crate::PrintableReflect;
use crate::ReflectPtr;
use crate::Script;
use crate::ScriptCollection;
use crate::LuaRef;
use crate::{APIProvider};
use crate::ScriptError;
use std::sync::{Arc};
use parking_lot::{RwLock};
use crate::util::impl_tealr_type;
use num_traits::cast::ToPrimitive;

use bevy_mod_scripting_derive::{impl_lua_newtypes,replace};
use tealr::{mlu::{mlua,TealDataMethods,TealData,mlua::{prelude::*,Error,MetaMethod,Value}}};


pub fn get_type_data<T: TypeData + ToOwned<Owned = T>>(w: &mut World, name: &str) -> Result<T,Error> {
    let registry: &TypeRegistry = w.get_resource().unwrap();

    let registry = registry.read();

    let reg = registry
        .get_with_short_name(&name)
        .or(registry.get_with_name(&name))
        .ok_or_else(|| Error::RuntimeError(format!(
            "Invalid component name {name}"
        )))
        .unwrap();

    let refl: T = reg
        .data::<T>()
        .ok_or_else(|| Error::RuntimeError(format!(
            "Invalid component name {name}"
        )))
        .unwrap()
        .to_owned();

    Ok(refl)
}


#[derive(Clone)]
pub struct LuaComponent {
    comp: LuaRef,
}

impl_tealr_type!(LuaComponent);


impl Debug for LuaComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LuaComponent")
            .field("comp", &self.comp)
            .finish()
    }
}

impl TealData for LuaComponent {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_, val, _a: Value| {
            val.comp.get(|s,_| {
                Ok(format!("{:#?}", PrintableReflect(s)))
            })
        });

        methods.add_meta_method_mut(MetaMethod::Index, |ctx, val, field: String| {
            let r = val.comp
                .path_ref(&field)
                .map_err(|_| Error::RuntimeError(format!("The field {field} does not exist on {val:?}")))?;

            Ok(r.convert_to_lua(ctx).unwrap())
        });

        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |ctx, val, (field, new_val): (Value, Value)| {
                val.comp
                    .path_ref_lua(field)?
                    .apply_lua(ctx, new_val).unwrap();
                
                
                Ok(())
            },
        );
    }
}

pub struct LuaResource {
    res: LuaRef,
}
impl_tealr_type!(LuaResource);

impl TealData for LuaResource {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(_methods: &mut T) {}
}




impl_lua_newtypes!{
    ( // test imports
        use bevy::math::*;
        use bevy::prelude::*;
        use bevy_mod_scripting_derive::replace;
    )
    [     // wrappers
        
    // ----------------------------------------------------------------------------- //
    // --------------------------- PRIMITIVE ASSIGNMENTS --------------------------- //
    // ----------------------------------------------------------------------------- //

    {
            usize : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<usize>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_usize().ok_or_else(||Error::RuntimeError("Value not compatibile with usize".to_owned()))?)));
            }
    },
    {
            isize : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<isize>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_isize().ok_or_else(||Error::RuntimeError("Value not compatibile with isize".to_owned()))?)));
            }
    },
    {
            i128 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i128>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i128().ok_or_else(||Error::RuntimeError("Value not compatibile with i128".to_owned()))?)));
            }
    },
    {
            i64 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i64>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i64().ok_or_else(||Error::RuntimeError("Value not compatibile with i64".to_owned()))?)));
            }
    },
    {
            i32 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i32>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i32().ok_or_else(||Error::RuntimeError("Value not compatibile with i32".to_owned()))?)));
            }
    },
    {
            i16 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i16>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i16().ok_or_else(||Error::RuntimeError("Value not compatibile with i16".to_owned()))?)));
            }
    },
    {
            i8 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i8>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i8().ok_or_else(||Error::RuntimeError("Value not compatibile with i8".to_owned()))?)));
            }
    },
    {
            u128 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u128>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u128().ok_or_else(||Error::RuntimeError("Value not compatibile with u128".to_owned()))?)));
            }
    },
    {
            u64 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u64>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u64().ok_or_else(||Error::RuntimeError("Value not compatibile with u64".to_owned()))?)));
            }
    },
    {
            u32 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u32>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u32().ok_or_else(||Error::RuntimeError("Value not compatibile with u32".to_owned()))?)));
            }
    },
    {
            u16 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u16>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u16().ok_or_else(||Error::RuntimeError("Value not compatibile with u16".to_owned()))?)));
            }
    },
    {
            u8 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u8>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u8().ok_or_else(||Error::RuntimeError("Value not compatibile with u8".to_owned()))?)));
            }
    },
    {
            f32 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Number(s.downcast_ref::<f32>().unwrap().to_f64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_number(v)?.ok_or_else(||Error::RuntimeError("Not a number".to_owned()))?.to_f32().ok_or_else(||Error::RuntimeError("Value not compatibile with f32".to_owned()))?)));
            }
    },
    {
            f64 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Number(s.downcast_ref::<f64>().unwrap().to_f64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_number(v)?.ok_or_else(||Error::RuntimeError("Not a number".to_owned()))?.to_f64().ok_or_else(||Error::RuntimeError("Value not compatibile with f64".to_owned()))?)));
            }
    },
    {
            alloc::string::String : Primitive
            impl {
            "to" => |r,c| r.get(|s,_| Value::String(c.create_string(s.downcast_ref::<String>().unwrap()).unwrap()));
            "from" =>   |r,c,v : Value| c.coerce_string(v)?.ok_or_else(||Error::RuntimeError("Not a string".to_owned())).and_then(|string| r.get_mut(|s,_| Ok(s.apply(&string.to_str()?.to_owned()))));                             //      
            }
    },
    // ----------------------------------------------------------------- //
    // --------------------------- BEVY MATH --------------------------- //
    // ----------------------------------------------------------------- //

    // --------------------------- Vectors ----------------------------- //

    {
        glam::vec2::Vec2 : Full 
        : 
            DebugToString
            + MathBinOp(Add:LuaWrapper[
                &self(Rhs:LuaVec2)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Sub:LuaWrapper[
                &self(Rhs:LuaVec2)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Div:LuaWrapper[
                &self(Rhs:LuaVec2)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Mul:LuaWrapper[
                &self(Rhs:LuaVec2)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Mod:LuaWrapper[
                &self(Rhs:LuaVec2)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                abs(),
                signum(),
                round(),
                floor(),
                ceil(),
                fract(),
                exp(),
                recip(),
                clamp(LuaVec2,LuaVec2),
                clamp_length(f32,f32),
                clamp_length_max(f32),
                clamp_length_min(f32),
                lerp(LuaVec2,f32),
                abs_diff_eq(LuaVec2,f32)->bool,
                normalize(),
                normalize_or_zero(),
                perp(),
                is_normalized() -> bool,
                is_finite() -> bool,
                is_nan() -> bool,
                length() -> f32,
                length_squared() -> f32,
                length_recip() -> f32,
                min_element() -> f32,
                max_element() -> f32,
                angle_between(LuaVec2) -> f32,
                project_onto(LuaVec2),
                reject_from(LuaVec2),
                project_onto_normalized(LuaVec2),
                reject_from_normalized(LuaVec2),
                perp_dot(LuaVec2) -> f32,
                dot(LuaVec2) -> f32,
                distance(LuaVec2) -> f32,
                distance_squared(LuaVec2) -> f32,
                min(LuaVec2),
                max(LuaVec2),
            )
        

        impl {
            static "new" => |_,(x,y) : (f32,f32)| Ok(LuaVec2::new(Vec2::new(x,y)));
            (MetaMethod::Index) (s=LuaVec2)=> {|_,s,idx: usize| {Ok(s.inner()[idx-1])}};
            mut (MetaMethod::NewIndex) (n=f32) => {|_,s,(idx,val): (usize,($n))| {Ok(s.val_mut(|s| s[idx-1] = val))}};
            (MetaMethod::Pow) (s=LuaVec2,a=f32) => {|_,s : &($s), o : ($a)| { Ok(($s)::new(s.inner().powf(o))) }};
        }
    },
    {
        glam::vec3::Vec3 : Full 
        : 
            DebugToString
            + MathBinOp(Add:LuaWrapper[
                &self(Rhs:LuaVec3)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Sub:LuaWrapper[
                &self(Rhs:LuaVec3)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Div:LuaWrapper[
                &self(Rhs:LuaVec3)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Mul:LuaWrapper[
                &self(Rhs:LuaVec3)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Mod:LuaWrapper[
                &self(Rhs:LuaVec3)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                // vec2 
                abs(),
                signum(),
                round(),
                floor(),
                ceil(),
                fract(),
                exp(),
                recip(),
                clamp(LuaVec3,LuaVec3),
                clamp_length(f32,f32),
                clamp_length_max(f32),
                clamp_length_min(f32),
                lerp(LuaVec3,f32),
                abs_diff_eq(LuaVec3,f32)->bool,
                normalize(),
                normalize_or_zero(),
                is_normalized() -> bool,
                is_finite() -> bool,
                is_nan() -> bool,
                length() -> f32,
                length_squared() -> f32,
                length_recip() -> f32,
                min_element() -> f32,
                max_element() -> f32,
                angle_between(LuaVec3) -> f32,
                project_onto(LuaVec3),
                reject_from(LuaVec3),
                project_onto_normalized(LuaVec3),
                reject_from_normalized(LuaVec3),
                dot(LuaVec3) -> f32,
                distance(LuaVec3) -> f32,
                distance_squared(LuaVec3) -> f32,
                min(LuaVec3),
                max(LuaVec3),
                // vec3 
                cross(LuaVec3),
                any_orthogonal_vector(),
                any_orthonormal_vector(),
            )
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaVec3),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=f32),
                LuaVec2 -> (MetaMethod::Pow) (s=LuaVec3,a=f32),
            )
    
        impl {
            static "new" => |_,(x,y,z) : (f32,f32,f32)| Ok(LuaVec3::new(Vec3::new(x,y,z)));
            "any_orthonormal_pair" (s=LuaVec3) => {|_,s : &($s),()| { 
                let (a,b) = s.inner().any_orthonormal_pair();
                Ok((($s)::new(a),($s)::new(b))) }
            };
        }
    },
    {
        glam::vec4::Vec4 : Full 
        : 
            DebugToString
            + MathBinOp(Add:LuaWrapper[
                &self(Rhs:LuaVec4)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Sub:LuaWrapper[
                &self(Rhs:LuaVec4)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Div:LuaWrapper[
                &self(Rhs:LuaVec4)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Mul:LuaWrapper[
                &self(Rhs:LuaVec4)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Mod:LuaWrapper[
                &self(Rhs:LuaVec4)],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                // vec2 
                abs(),
                signum(),
                round(),
                floor(),
                ceil(),
                fract(),
                exp(),
                recip(),
                clamp(LuaVec4,LuaVec4),
                clamp_length(f32,f32),
                clamp_length_max(f32),
                clamp_length_min(f32),
                lerp(LuaVec4,f32),
                abs_diff_eq(LuaVec4,f32)->bool,
                normalize(),
                normalize_or_zero(),
                is_normalized() -> bool,
                is_finite() -> bool,
                is_nan() -> bool,
                length() -> f32,
                length_squared() -> f32,
                length_recip() -> f32,
                min_element() -> f32,
                max_element() -> f32,
                project_onto(LuaVec4),
                reject_from(LuaVec4),
                project_onto_normalized(LuaVec4),
                reject_from_normalized(LuaVec4),
                dot(LuaVec4) -> f32,
                distance(LuaVec4) -> f32,
                distance_squared(LuaVec4) -> f32,
                min(LuaVec4),
                max(LuaVec4),
            )
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaVec4),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=f32),
                LuaVec2 -> (MetaMethod::Pow) (s=LuaVec4,a=f32),
            )
    
        impl {
            static "new" => |_,(x,y,z,w) : (f32,f32,f32,f32)| Ok(LuaVec4::new(Vec4::new(x,y,z,w)));
        }
    },
    {
        glam::vec2::DVec2 : Full 
        : 
            DebugToString
            + MathBinOp(Add:LuaWrapper[
                &self(Rhs:LuaDVec2)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Sub:LuaWrapper[
                &self(Rhs:LuaDVec2)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Div:LuaWrapper[
                &self(Rhs:LuaDVec2)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Mul:LuaWrapper[
                &self(Rhs:LuaDVec2)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Mod:LuaWrapper[
                &self(Rhs:LuaDVec2)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                abs(),
                signum(),
                round(),
                floor(),
                ceil(),
                fract(),
                exp(),
                recip(),
                clamp(LuaDVec2,LuaDVec2),
                clamp_length(f64,f64),
                clamp_length_max(f64),
                clamp_length_min(f64),
                lerp(LuaDVec2,f64),
                abs_diff_eq(LuaDVec2,f64)->bool,
                normalize(),
                normalize_or_zero(),
                perp(),
                is_normalized() -> bool,
                is_finite() -> bool,
                is_nan() -> bool,
                length() -> f64,
                length_squared() -> f64,
                length_recip() -> f64,
                min_element() -> f64,
                max_element() -> f64,
                angle_between(LuaDVec2) -> f64,
                project_onto(LuaDVec2),
                reject_from(LuaDVec2),
                project_onto_normalized(LuaDVec2),
                reject_from_normalized(LuaDVec2),
                perp_dot(LuaDVec2) -> f64,
                dot(LuaDVec2) -> f64,
                distance(LuaDVec2) -> f64,
                distance_squared(LuaDVec2) -> f64,
                min(LuaDVec2),
                max(LuaDVec2),
            )
        
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaDVec2),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=f64),
                LuaVec2 -> (MetaMethod::Pow) (s=LuaDVec2,a=f64),
            )
    
        impl {
            static "new" => |_,(x,y) : (f64,f64)| Ok(LuaDVec2::new(DVec2::new(x,y)));
        }
    },
    {
        glam::vec3::DVec3 : Full 
        : 
            DebugToString
            + MathBinOp(Add:LuaWrapper[
                &self(Rhs:LuaDVec3)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Sub:LuaWrapper[
                &self(Rhs:LuaDVec3)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Div:LuaWrapper[
                &self(Rhs:LuaDVec3)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Mul:LuaWrapper[
                &self(Rhs:LuaDVec3)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Mod:LuaWrapper[
                &self(Rhs:LuaDVec3)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                // vec2 
                abs(),
                signum(),
                round(),
                floor(),
                ceil(),
                fract(),
                exp(),
                recip(),
                clamp(LuaDVec3,LuaDVec3),
                clamp_length(f64,f64),
                clamp_length_max(f64),
                clamp_length_min(f64),
                lerp(LuaDVec3,f64),
                abs_diff_eq(LuaDVec3,f64)->bool,
                normalize(),
                normalize_or_zero(),
                is_normalized() -> bool,
                is_finite() -> bool,
                is_nan() -> bool,
                length() -> f64,
                length_squared() -> f64,
                length_recip() -> f64,
                min_element() -> f64,
                max_element() -> f64,
                angle_between(LuaDVec3) -> f64,
                project_onto(LuaDVec3),
                reject_from(LuaDVec3),
                project_onto_normalized(LuaDVec3),
                reject_from_normalized(LuaDVec3),
                dot(LuaDVec3) -> f64,
                distance(LuaDVec3) -> f64,
                distance_squared(LuaDVec3) -> f64,
                min(LuaDVec3),
                max(LuaDVec3),
                // vec3 
                cross(LuaDVec3),
                any_orthogonal_vector(),
                any_orthonormal_vector(),
            )
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaDVec3),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=f64),
                LuaVec2 -> (MetaMethod::Pow) (s=LuaDVec3,a=f64),
                LuaVec3 -> "any_orthonormal_pair" (s=LuaDVec3),
            )
        impl {
            static "new" => |_,(x,y,z) : (f64,f64,f64)| Ok(LuaDVec3::new(DVec3::new(x,y,z)));
        }
    },
    {
        glam::vec4::DVec4 : Full 
        : 
            DebugToString
            + MathBinOp(Add:LuaWrapper[
                &self(Rhs:LuaDVec4)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Sub:LuaWrapper[
                &self(Rhs:LuaDVec4)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Div:LuaWrapper[
                &self(Rhs:LuaDVec4)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Mul:LuaWrapper[
                &self(Rhs:LuaDVec4)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Mod:LuaWrapper[
                &self(Rhs:LuaDVec4)],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                // vec2 
                abs(),
                signum(),
                round(),
                floor(),
                ceil(),
                fract(),
                exp(),
                recip(),
                clamp(LuaDVec4,LuaDVec4),
                clamp_length(f64,f64),
                clamp_length_max(f64),
                clamp_length_min(f64),
                lerp(LuaDVec4,f64),
                abs_diff_eq(LuaDVec4,f64)->bool,
                normalize(),
                normalize_or_zero(),
                is_normalized() -> bool,
                is_finite() -> bool,
                is_nan() -> bool,
                length() -> f64,
                length_squared() -> f64,
                length_recip() -> f64,
                min_element() -> f64,
                max_element() -> f64,
                project_onto(LuaDVec4),
                reject_from(LuaDVec4),
                project_onto_normalized(LuaDVec4),
                reject_from_normalized(LuaDVec4),
                dot(LuaDVec4) -> f64,
                distance(LuaDVec4) -> f64,
                distance_squared(LuaDVec4) -> f64,
                min(LuaDVec4),
                max(LuaDVec4),
            )
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaDVec4),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=f64),
                LuaVec2 -> (MetaMethod::Pow) (s=LuaDVec4,a=f64),
            )
        impl {
            static "new" => |_,(x,y,z,w) : (f64,f64,f64,f64)| Ok(LuaDVec4::new(DVec4::new(x,y,z,w)));
        }
    },
    {
        glam::vec2::IVec2 : Full 
        : 
            DebugToString
            + MathBinOp(Add:LuaWrapper[
                &self(Rhs:LuaIVec2)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathBinOp(Sub:LuaWrapper[
                &self(Rhs:LuaIVec2)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathBinOp(Div:LuaWrapper[
                &self(Rhs:LuaIVec2)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathBinOp(Mul:LuaWrapper[
                &self(Rhs:LuaIVec2)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathBinOp(Mod:LuaWrapper[
                &self(Rhs:LuaIVec2)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                // vec2 
                abs(),
                signum(),
                clamp(LuaIVec2,LuaIVec2),
                min_element() -> i32,
                max_element() -> i32,
                dot(LuaIVec2) -> i32,
                min(LuaIVec2),
                max(LuaIVec2),
            )
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaIVec2),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=i32),
            )
    
        impl {
            static "new" => |_,(x,y) : (i32,i32)| Ok(LuaIVec2::new(IVec2::new(x,y)));
        }
    },
    {
        glam::vec3::IVec3 : Full 
        : 
            DebugToString
            + MathBinOp(Add:LuaWrapper[
                &self(Rhs:LuaIVec3)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathBinOp(Sub:LuaWrapper[
                &self(Rhs:LuaIVec3)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathBinOp(Div:LuaWrapper[
                &self(Rhs:LuaIVec3)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathBinOp(Mul:LuaWrapper[
                &self(Rhs:LuaIVec3)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathBinOp(Mod:LuaWrapper[
                &self(Rhs:LuaIVec3)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                // vec2 
                abs(),
                signum(),
                clamp(LuaIVec3,LuaIVec3),
                min_element() -> i32,
                max_element() -> i32,
                dot(LuaIVec3) -> i32,
                min(LuaIVec3),
                max(LuaIVec3),
                cross(LuaIVec3),
            )
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaIVec3),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=i32),
            )
    
        impl {
            static "new" => |_,(x,y,z) : (i32,i32,i32)| Ok(LuaIVec3::new(IVec3::new(x,y,z)));
        }
    },
    {
        glam::vec4::IVec4 : Full 
        : 
            DebugToString
            + MathBinOp(Add:LuaWrapper[
                &self(Rhs:LuaIVec4)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathBinOp(Sub:LuaWrapper[
                &self(Rhs:LuaIVec4)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathBinOp(Div:LuaWrapper[
                &self(Rhs:LuaIVec4)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathBinOp(Mul:LuaWrapper[
                &self(Rhs:LuaIVec4)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathBinOp(Mod:LuaWrapper[
                &self(Rhs:LuaIVec4)],
                Number[&self(Rhs:i32),(Lhs:i32)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                // vec2 
                abs(),
                signum(),
                clamp(LuaIVec4,LuaIVec4),
                min_element() -> i32,
                max_element() -> i32,
                dot(LuaIVec4) -> i32,
                min(LuaIVec4),
                max(LuaIVec4),
            )
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaIVec4),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=i32),
            )
    
        impl {
            static "new" => |_,(x,y,z,w) : (i32,i32,i32,i32)| Ok(LuaIVec4::new(IVec4::new(x,y,z,w)));
        }
    },
    {
        glam::vec2::UVec2 : Full 
        : 
            DebugToString
            + MathBinOp(Add:LuaWrapper[
                &self(Rhs:LuaUVec2)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + MathBinOp(Sub:LuaWrapper[
                &self(Rhs:LuaUVec2)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + MathBinOp(Div:LuaWrapper[
                &self(Rhs:LuaUVec2)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + MathBinOp(Mul:LuaWrapper[
                &self(Rhs:LuaUVec2)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + MathBinOp(Mod:LuaWrapper[
                &self(Rhs:LuaUVec2)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + AutoMethods(
                // vec2 
                clamp(LuaUVec2,LuaUVec2),
                min_element() -> i32,
                max_element() -> i32,
                dot(LuaUVec2) -> i32,
                min(LuaUVec2),
                max(LuaUVec2),
            )
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaUVec2),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=u32),
            )
    
        impl {
            static "new" => |_,(x,y) : (u32,u32)| Ok(LuaUVec2::new(UVec2::new(x,y)));
        }
    },
    {
        glam::vec3::UVec3 : Full 
        : 
            DebugToString
            + MathBinOp(Add:LuaWrapper[
                &self(Rhs:LuaUVec3)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + MathBinOp(Sub:LuaWrapper[
                &self(Rhs:LuaUVec3)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + MathBinOp(Div:LuaWrapper[
                &self(Rhs:LuaUVec3)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + MathBinOp(Mul:LuaWrapper[
                &self(Rhs:LuaUVec3)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + MathBinOp(Mod:LuaWrapper[
                &self(Rhs:LuaUVec3)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + AutoMethods(
                // vec2 
                clamp(LuaUVec3,LuaUVec3),
                min_element() -> i32,
                max_element() -> i32,
                dot(LuaUVec3) -> i32,
                min(LuaUVec3),
                max(LuaUVec3),
                cross(LuaUVec3)
            )
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaUVec3),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=u32),
            )
        impl {
            static "new" => |_,(x,y,z) : (u32,u32,u32)| Ok(LuaUVec3::new(UVec3::new(x,y,z)));
        }
    },
    {
        glam::vec4::UVec4 : Full 
        : 
            DebugToString
            + MathBinOp(Add:LuaWrapper[
                &self(Rhs:LuaUVec4)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + MathBinOp(Sub:LuaWrapper[
                &self(Rhs:LuaUVec4)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + MathBinOp(Div:LuaWrapper[
                &self(Rhs:LuaUVec4)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + MathBinOp(Mul:LuaWrapper[
                &self(Rhs:LuaUVec4)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + MathBinOp(Mod:LuaWrapper[
                &self(Rhs:LuaUVec4)],
                Number[&self(Rhs:u32),(Lhs:u32)])
            + AutoMethods(
                // vec2 
                clamp(LuaUVec4,LuaUVec4),
                min_element() -> i32,
                max_element() -> i32,
                dot(LuaUVec4) -> i32,
                min(LuaUVec4),
                max(LuaUVec4),
            )
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaUVec4),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=u32),
            )
    
        impl {
            static "new" => |_,(x,y,z,w) : (u32,u32,u32,u32)| Ok(LuaUVec4::new(UVec4::new(x,y,z,w)));
        }
    },
    // --------------------------- Matrices --------------------------- //
    {
        glam::mat3::Mat3: Full 
        : DebugToString
            + MathBinOp(Mul:
                LuaWrapper[&self(Rhs:LuaMat3),
                    &self(Rhs:LuaVec3->LuaWrapper(LuaVec3))],
                Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Sub:
                LuaWrapper[&self(Rhs:LuaMat3)])
            + MathBinOp(Add:
                LuaWrapper[&self(Rhs:LuaMat3)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                transpose(),
                determinant() -> f32,
                inverse(),
                is_nan() -> bool,
                is_finite() -> bool,
                is_nan() -> bool,
                transform_point2(LuaVec2) -> LuaVec2,
                transform_vector2(LuaVec2) -> LuaVec2,
            )
        impl{       
            static "new" => |_,(x,y,z) : (LuaVec3,LuaVec3,LuaVec3)| Ok(LuaMat3::new(Mat3::from_cols(x.inner(),y.inner(),z.inner())));
              
            mut (MetaMethod::Index) (s=LuaMat3,b=Mat3,v=LuaVec3) => {|_,s,idx : usize| {
                match s {
                    ($s)::Owned(ref mut v, ref valid) => {
                        Ok(($v)::Ref(LuaRef{
                            root: LuaRefBase::LuaOwned{valid: Arc::downgrade((valid))},
                            r: ReflectPtr::Mut(v.col_mut(idx-1)),
                            path: None
                        }))
                    },
                    ($s)::Ref(ref mut r) => {
                        r.get_mut(|s,r| {
                            Ok(($v)::Ref(LuaRef{
                                root: r.root.clone(),
                                r: ReflectPtr::Mut(s.downcast_mut::<($b)>().unwrap().col_mut(idx-1)),
                                path: None
                            })) 
                        })
                    }
                }
            }};
        }
    },
    {
        glam::mat4::Mat4: Full 
        : DebugToString
            + MathBinOp(Mul:
                LuaWrapper[&self(Rhs:LuaMat4),
                    &self(Rhs:LuaVec4->LuaWrapper(LuaVec4))],                
                    Number[&self(Rhs:f32),(Lhs:f32)])
            + MathBinOp(Sub:
                LuaWrapper[&self(Rhs:LuaMat4)])
            + MathBinOp(Add:
                LuaWrapper[&self(Rhs:LuaMat4)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                transpose(),
                determinant() -> f32,
                inverse(),
                is_nan() -> bool,
                is_finite() -> bool,
                is_nan() -> bool,
                transform_point3(LuaVec3) -> LuaVec3,
                transform_vector3(LuaVec3) -> LuaVec3,
                project_point3(LuaVec3) -> LuaVec3,
            )
            + Copy(
                LuaMat3 -> mut (MetaMethod::Index) (s=LuaMat4,b=Mat4,v=LuaVec4),
            )
        impl {
            static "new" => |_,(x,y,z,w) : (LuaVec4,LuaVec4,LuaVec4,LuaVec4)| Ok(LuaMat4::new(Mat4::from_cols(x.inner(),y.inner(),z.inner(),w.inner())));
       }
    },
    {
        glam::mat3::DMat3: Full 
        : DebugToString
            + MathBinOp(Mul:
                LuaWrapper[&self(Rhs:LuaDMat3),
                    &self(Rhs:LuaDVec3->LuaWrapper(LuaDVec3))],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Sub:
                LuaWrapper[&self(Rhs:LuaDMat3)])
            + MathBinOp(Add:
                LuaWrapper[&self(Rhs:LuaDMat3)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                transpose(),
                determinant() -> f64,
                inverse(),
                is_nan() -> bool,
                is_finite() -> bool,
                is_nan() -> bool,
                transform_point2(LuaDVec2) -> LuaDVec2,
                transform_vector2(LuaDVec2) -> LuaDVec2,
            )
            + Copy(
                LuaMat3 -> mut (MetaMethod::Index) (s=LuaDMat3,b=DMat3,v=LuaDVec3),
            )
    },
    {
        glam::mat4::DMat4: Full 
        : DebugToString
            + MathBinOp(Mul:
                LuaWrapper[&self(Rhs:LuaDMat4),
                    &self(Rhs:LuaDVec4->LuaWrapper(LuaDVec4))],
                Number[&self(Rhs:f64),(Lhs:f64)])
            + MathBinOp(Sub:
                LuaWrapper[&self(Rhs:LuaDMat4)])
            + MathBinOp(Add:
                LuaWrapper[&self(Rhs:LuaDMat4)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                transpose(),
                determinant() -> f64,
                inverse(),
                is_nan() -> bool,
                is_finite() -> bool,
                is_nan() -> bool,
                transform_point3(LuaDVec3) -> LuaDVec3,
                transform_vector3(LuaDVec3) -> LuaDVec3,
                project_point3(LuaDVec3) -> LuaDVec3,
            )
            + Copy(
                LuaMat3 -> mut (MetaMethod::Index) (s=LuaDMat4,b=DMat4,v=LuaDVec4),
            )
        impl {
            static "new" => |_,(x,y,z,w) : (LuaDVec4,LuaDVec4,LuaDVec4,LuaDVec4)| Ok(LuaDMat4 ::new(DMat4::from_cols(x.inner(),y.inner(),z.inner(),w.inner())));
        }
    },
    // --------------------------- Quats --------------------------- //
    {
        glam::quat::Quat : Full 
        : DebugToString
            + MathBinOp(Add:
                LuaWrapper[&self(Rhs:LuaQuat)])
            + MathBinOp(Sub:
                LuaWrapper[&self(Rhs:LuaQuat)])
            + MathBinOp(Mul:
                LuaWrapper[&self(Rhs:LuaQuat),
                &self(Rhs:LuaVec3->LuaWrapper(LuaVec3))],
                Number[&self(Rhs:f32)])
            + MathBinOp(Div:
                Number[&self(Rhs:f32)])
            + MathUnaryOp(Unm:)
            + AutoMethods(
                to_scaled_axis() -> LuaVec3,
                xyz() -> LuaVec3,
                conjugate(),
                inverse(),
                length() -> f32,
                length_squared() -> f32,
                length_recip() -> f32,
                normalize(),
                is_finite() -> bool,
                is_nan() -> bool,
                is_normalized() -> bool,
                is_near_identity() -> bool,
                dot(LuaQuat) -> f32,
                angle_between(LuaQuat) -> f32,
                abs_diff_eq(LuaQuat,f32) -> bool,
                lerp(LuaQuat,f32),
                slerp(LuaQuat,f32)
            )

        impl {
            static "new" => |_,(x,y,z,w) : (f32,f32,f32,f32)| Ok(LuaQuat::new(Quat::from_xyzw(x,y,z,w)));

            "to_axis_angle" (v=LuaVec3) => {|_,s,()| {
                                                let (v,f) = s.inner().to_axis_angle();
                                                let o = (($v)::new(v),f);
                                                Ok(o)
                                            }};
            "to_euler" (s=LuaQuat) => {|_,s,e : LuaEulerRot| Ok(s.inner().to_euler(*e))};
        }
    },
    {
        glam::quat::DQuat : Full 
        : DebugToString
        + MathBinOp(Add:
            LuaWrapper[&self(Rhs:LuaDQuat)])
        + MathBinOp(Sub:
            LuaWrapper[&self(Rhs:LuaDQuat)])
        + MathBinOp(Mul:
            LuaWrapper[&self(Rhs:LuaDQuat),
            &self(Rhs:LuaDVec3->LuaWrapper(LuaDVec3))],
            Number[&self(Rhs:f64)])
        + MathBinOp(Div:
            Number[&self(Rhs:f64)])
        + MathUnaryOp(Unm:)
        + AutoMethods(
            to_scaled_axis() -> LuaDVec3,
            xyz() -> LuaDVec3,
            conjugate(),
            inverse(),
            length() -> f64,
            length_squared() -> f64,
            length_recip() -> f64,
            normalize(),
            is_finite() -> bool,
            is_nan() -> bool,
            is_normalized() -> bool,
            is_near_identity() -> bool,
            dot(LuaDQuat) -> f64,
            angle_between(LuaDQuat) -> f64,
            abs_diff_eq(LuaDQuat,f64) -> bool,
            lerp(LuaDQuat,f64),
            slerp(LuaDQuat,f64)
        )
        + Copy(
            LuaQuat -> "to_axis_angle" (v=LuaDVec3),
            LuaQuat -> "to_euler" (s=LuaDQuat),
        )
        impl {
            static "new" => |_,(x,y,z,w) : (f64,f64,f64,f64)| Ok(LuaDQuat::new(DQuat::from_xyzw(x,y,z,w)));
        }
    },
    {
        bevy_ecs::entity::Entity: Full :
            AutoMethods(
                id() -> u32,
                generation() -> u32,
                to_bits() -> u64,
            )
    },
    {
        bevy_ecs::world::World: NonAssignable(Weak<RwLock<World>>)

        impl {
            "add_component" =>  |_, world, (entity, comp_name): (LuaEntity, String)| {
                 // grab this entity before acquiring a lock in case it's a reference
                 let entity = entity.inner();
                 let w = world.upgrade().unwrap();
                 let w = &mut w.write();
                 let refl: ReflectComponent = get_type_data(w, &comp_name)
                     .map_err(|_| Error::RuntimeError(format!("Not a component {}",comp_name)))?;
                 let def = get_type_data::<ReflectDefault>(w, &comp_name)
                     .map_err(|_| Error::RuntimeError(format!("Component does not derive ReflectDefault and cannot be instantiated: {}",comp_name)))?;
                 let s = def.default();
                 refl.add_component(w, entity, s.as_ref());
                 let id = w.components().get_id(s.type_id()).unwrap();

                 Ok(LuaComponent {
                     comp: LuaRef{
                         root: LuaRefBase::Component{ 
                             comp: refl.clone(), 
                             id,
                             entity: entity,
                             world: world.as_ref().clone()
                         }, 
                         path: Some("".to_string()), 
                         r: ReflectPtr::Const(refl.reflect_component(w,entity).unwrap())
                     }    
                 })
            };

            "add_component" =>  |_, world, (entity, comp_name): (LuaEntity, String)| {
                // grab this entity before acquiring a lock in case it's a reference
                let entity = entity.inner();

                let w = world.upgrade().unwrap();
                let w = &mut w.write();

                let refl: ReflectComponent = get_type_data(w, &comp_name)
                    .map_err(|_| Error::RuntimeError(format!("Not a component {}",comp_name)))?;
                let def = get_type_data::<ReflectDefault>(w, &comp_name)
                    .map_err(|_| Error::RuntimeError(format!("Component does not derive Default and cannot be instantiated: {}",comp_name)))?;

                let s = def.default();
                let id = w.components().get_id(s.type_id()).unwrap();

                refl.add_component(w, entity, s.as_ref());


                Ok(LuaComponent {
                    comp: LuaRef{
                        root: LuaRefBase::Component{ 
                            comp: refl.clone(), 
                            id,
                            entity: entity,
                            world: world.as_ref().clone()
                        }, 
                        path: Some("".to_string()), 
                        r: ReflectPtr::Const(refl.reflect_component(w,entity).unwrap())
                    }    
                })
            };

            "get_component" => |_, world, (entity, comp_name) : (LuaEntity,String)| {

                // grab this entity before acquiring a lock in case it's a reference
                let entity = entity.inner();

                let w = world.upgrade().unwrap();
                let w = &mut w.write();

                let refl: ReflectComponent = get_type_data(w, &comp_name)
                    .map_err(|_| Error::RuntimeError(format!("Not a component {}",comp_name)))?;

                let dyn_comp = refl
                    .reflect_component(&w, entity)
                    .ok_or_else(|| Error::RuntimeError(format!("Could not find {comp_name} on {:?}",entity),
                    ))?;

                let id = w.components().get_id(dyn_comp.type_id()).unwrap();

                Ok(
                    LuaComponent {
                        comp: LuaRef{
                            root: LuaRefBase::Component{ 
                                comp: refl, 
                                id,
                                entity: entity,
                                world: world.as_ref().clone()
                            }, 
                            path: Some("".to_string()), 
                            r: ReflectPtr::Const(dyn_comp)
                        }    
                    }  
                )
            };

            "new_script_entity" => |_, world, name: String| {
                let w = world.upgrade().unwrap();
                let w = &mut w.write();

                w.resource_scope(|w, r: Mut<AssetServer>| {
                    let handle = r.load::<LuaFile, _>(&name);
                    Ok(LuaEntity::new(
                        w.spawn()
                            .insert(ScriptCollection::<crate::LuaFile> {
                                scripts: vec![Script::<LuaFile>::new(name, handle)],
                            })
                            .id(),
                    ))
                })
            };

            "spawn" => |_, world, ()| {
                let w = world.upgrade().unwrap();
                let w = &mut w.write();                
                
                Ok(LuaEntity::new(w.spawn().id()))
            };
        }
    },
    {
        glam::euler::EulerRot: NonAssignable(EulerRot) 
    }
    ]

}





#[cfg(test)]

mod test {
    use crate::{langs::mlu::{mlua,mlua::prelude::*},LuaEntity, LuaEvent, Recipients, LuaComponent, LuaRef, LuaRefBase, get_type_data, ReflectPtr};
    use bevy::{prelude::*,reflect::TypeRegistryArc};
    use std::{any::Any,sync::Arc};
    use parking_lot::RwLock;

    #[derive(Clone)]
    struct TestArg(LuaEntity);

    impl <'lua>ToLua<'lua> for TestArg {
        fn to_lua(self, ctx: &'lua Lua) -> Result<LuaValue<'lua>, mlua::Error> { 
            self.0.to_lua(ctx) 
        }
    }

    #[derive(Component,Reflect,Default)]
    #[reflect(Component)]
    struct TestComponent{
        mat3: Mat3,
    }

    #[test]
    #[should_panic]
    fn miri_test_components(){
        let world_arc = Arc::new(RwLock::new(World::new()));

        let mut component_ref1;
        let mut component_ref2;

        {
            let world = &mut world_arc.write();

            world.init_resource::<TypeRegistryArc>();
            let registry = world.resource_mut::<TypeRegistryArc>();
            registry.write().register::<TestComponent>();

            let tst_comp = TestComponent{
                mat3: Mat3::from_cols(Vec3::new(1.0,2.0,3.0),
                                    Vec3::new(4.0,5.0,6.0),
                                    Vec3::new(7.0,8.0,9.0))
            };

            let entity = world.spawn()
                            .insert(tst_comp)
                            .id();

            let refl: ReflectComponent = get_type_data(world, "TestComponent").unwrap();
            let refl_ref = refl.reflect_component(world,entity).unwrap();
            let ptr : ReflectPtr = ReflectPtr::Const(refl_ref);
            let id = world.components().get_id(refl_ref.type_id()).unwrap();

            component_ref1 = LuaRef{
                r: ptr,
                root: LuaRefBase::Component{ 
                    comp: refl, 
                    id,
                    entity,
                    world: Arc::downgrade(&world_arc),
                }, 
                path: Some("".to_string()), 
            };
            component_ref2 = component_ref1.clone();
        }

        component_ref1.get(|r1,_| {
            component_ref2.get(|r2,_|{
                let _ = r1.downcast_ref::<TestComponent>().unwrap().mat3 + r2.downcast_ref::<TestComponent>().unwrap().mat3;
            })
        });

        component_ref1.get_mut(|r1,_| {
            let _ = r1.downcast_ref::<TestComponent>().unwrap().mat3 * 2.0;
        });

        component_ref2.get_mut(|r2,_|{
            let _ = r2.downcast_ref::<TestComponent>().unwrap().mat3 * 2.0;
        });

        // invalid should panic here
        component_ref1.get_mut(|r1,_| {
            component_ref2.get(|r2,_|{
                *r1.downcast_mut::<TestComponent>().unwrap().mat3 = *r2.downcast_ref::<TestComponent>().unwrap().mat3;
            })
        });    
    }

    #[test]
    #[should_panic]
    fn miri_test_owned(){
       
        let mut mat = Mat3::from_cols(Vec3::new(1.0,2.0,3.0),
                                Vec3::new(4.0,5.0,6.0),
                                Vec3::new(7.0,8.0,9.0));
        
        let ptr : ReflectPtr = ReflectPtr::Mut(mat.col_mut(0));
        let valid = Arc::new(RwLock::new(()));

        let mut ref1 = LuaRef{
            r: ptr,
            root: LuaRefBase::LuaOwned{valid:Arc::downgrade(&valid)},
            path: None, 
        };
        let mut ref2 = ref1.clone();

        ref1.get(|r1,_| {
            ref2.get(|r2,_|{
                let _ = *r1.downcast_ref::<Vec3>().unwrap() + *r2.downcast_ref::<Vec3>().unwrap();
            })
        });

        ref1.get_mut(|r1,_| {
            let _ = *r1.downcast_ref::<Vec3>().unwrap() * 2.0;
        });

        ref2.get_mut(|r2,_|{
            let _ = *r2.downcast_ref::<Vec3>().unwrap() * 2.0;
        });

        drop(valid);
        drop(mat);

        // should panic since original value dropped
        ref1.get_mut(|r1,_| r1.downcast_mut::<Vec3>().unwrap()[1] = 2.0);
    }

}