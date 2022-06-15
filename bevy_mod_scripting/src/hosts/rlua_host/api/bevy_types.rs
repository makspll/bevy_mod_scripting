use bevy::reflect::TypeData;
use bevy::reflect::TypeRegistry;
use bevy_mod_scripting_derive::{impl_lua_newtypes,replace};
use rlua::{UserData, MetaMethod,Value,Context,Error,Lua};
use paste::paste;
use bevy::prelude::*;
use bevy::math::*;
use std::sync::Weak;
use std::{fmt,fmt::{Debug,Display,Formatter}, ops::*,sync::Mutex};
use phf::{phf_map, Map};
use std::ops::DerefMut;
use num::ToPrimitive;
use crate::LuaFile;
use crate::LuaRefBase;
use crate::PrintableReflect;
use crate::ReflectPtr;
use crate::Script;
use crate::ScriptCollection;
use crate::LuaRef;
use crate::APIProvider;
use crate::ScriptError;
use std::sync::{Arc};
use parking_lot::{RwLock};
pub trait LuaWrappable : Reflect + Clone {}

impl <T : Reflect + Clone> LuaWrappable for T {}


#[derive(Debug,Clone)]
pub enum LuaWrapper<T : LuaWrappable> { 
    Owned(T,Arc<RwLock<()>>),
    Ref(LuaRef)
}

impl <T : LuaWrappable>Drop for LuaWrapper<T> {
    fn drop(&mut self) {
        match self {
            Self::Owned(_,valid) => {
                if valid.is_locked() {
                    panic!("Something is referencing a lua value and it's about to go out of scope!");
                }
            },
            Self::Ref(_) => {},
        }
    }
}

impl <T : LuaWrappable + Display> Display for LuaWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f,"{}", self)
    }
}

impl <T : LuaWrappable>LuaWrapper<T> {

    pub fn new(b : T) -> Self {
        Self::Owned(b,Arc::new(RwLock::new(())))
    }

    pub fn new_ref(b : &LuaRef) -> Self {
        Self::Ref(b.clone())
    }

    /// Perform an operation on the base type and optionally retrieve something by value
    /// may require a read lock on the world in case this is a reference
    pub fn val<G,F>(&self, accessor: F) -> G
        where 
        F: FnOnce(&T) -> G
    {
        match self {
            Self::Owned(ref v, valid) => {
                // we lock here in case the accessor has a luaref holding reference to us
                let lock = valid.read();
                let o = accessor(v);
                drop(lock);

                o
            },
            Self::Ref(v) => {
                v.get(|s,_| accessor(s.downcast_ref::<T>().unwrap()))
            },
        }
    }

    pub fn val_mut<G,F>(&mut self, accessor: F) -> G
        where 
        F: FnOnce(&mut T) -> G
    {
        match self {
            Self::Owned(ref mut v, valid) => {
                let lock = valid.read();
                let o = accessor(v);
                drop(lock);

                o
            },
            Self::Ref(v) => {
                v.get_mut(|s,_| accessor(s.downcast_mut::<T>().unwrap()))
            },
        }
    }


    

    /// Perform a binary operation on self and any other type and optionally retrieve something by value,
    /// may require a read lock on the world in case this is a reference
    pub fn bin<O,G,F>(&self,o: &O, binv: F) -> G
    where 
    F: FnOnce(&T,&O) -> G
    {
        match self {
            Self::Owned(ref v, valid) => {
                let lock = valid.read();
                let o = binv(v,o);
                drop(lock);
                o
            },
            Self::Ref(ref v) => v.get(|v,_| binv(v.downcast_ref::<T>().unwrap(),o)),
        }
    }

    /// returns wrapped value by value, 
    /// may require a read lock on the world in case this is a reference
    pub fn inner(&self) -> T
    {
        match self {
            Self::Owned(ref v, ..) => v.clone(),//no need to lock here
            Self::Ref(v) => {
                v.get(|s,_| s.downcast_ref::<T>().unwrap().clone())
            },
        }
    }

    /// Converts a LuaRef to Self
    pub fn base_to_self(b: &LuaRef) -> Self {
        Self::Ref(b.clone())
    }

    /// Applies Self to a LuaRef.
    /// may require a write lock on the world
    pub fn apply_self_to_base(&self, b: &mut LuaRef){

        match self {
            Self::Owned(ref v, ..) => {
                // if we own the value, we are not borrowing from the world
                // we're good to just apply, yeet
                b.get_mut(|b,_| b.apply(v))
            },
            Self::Ref(v) => {
                // if we are a luaref, we have to be careful with borrows
                b.apply_luaref(v)
            }
        }
    }
}




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

impl Debug for LuaComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LuaComponent")
            .field("comp", &self.comp)
            .finish()
    }
}

impl UserData for LuaComponent {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
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

impl UserData for LuaResource {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(_methods: &mut T) {}
}


// pub type LuaDQuat = crate::LuaWrapper<DQuat>;
// impl rlua::UserData for LuaDQuat {
//     fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
//         methods.add_meta_function(rlua::MetaMethod::Sub, |ctx, (lhs, rhs): (Value, Value)| {
//             let (ud, op_on_rhs): (&rlua::AnyUserData, bool) = match (&lhs, &rhs) {
//                 (Value::UserData(v), _) => (v, true),
//                 (_, Value::UserData(v)) => (v, false),
//                 _ => panic!(""),
//             };
//             let ud: &LuaDQuat = &ud.borrow::<LuaDQuat>().unwrap();
//             if op_on_rhs {
//                 let op = &rhs;
//                 if let Value::UserData(op) = op {
//                     if let Ok(op) = op.borrow::<LuaDQuat>() {
//                         let op = &op;
//                         return ud
//                             .bin(op, |ud, op| op.val(|op| Ok(LuaDQuat::new((ud).sub(*op)))));
//                     };
//                 };
//             } else {
//                 let op = &lhs;
//             };
//             Err(rlua::Error::RuntimeError(
//                 "Attempted to perform invalid arithmetic with userdata".to_owned(),
//             ))
//         });
//         methods.add_meta_function(rlua::MetaMethod::Mul, |ctx, (lhs, rhs): (Value, Value)| {
//             let (ud, op_on_rhs): (&rlua::AnyUserData, bool) = match (&lhs, &rhs) {
//                 (Value::UserData(v), _) => (v, true),
//                 (_, Value::UserData(v)) => (v, false),
//                 _ => panic!(""),
//             };
//             let ud: &LuaDQuat = &ud.borrow::<LuaDQuat>().unwrap();
//             if op_on_rhs {
//                 let op = &rhs;
//                 if let Ok(Some(op)) = ctx.coerce_number(op.clone()) {
//                     let op = &(op as f64);
//                     return ud.bin(op, |ud, op| Ok(LuaDQuat::new((ud).mul(*op))));
//                 };
//             } else {
//                 let op = &lhs;
//             };
//             Err(rlua::Error::RuntimeError(
//                 "Attempted to perform invalid arithmetic with userdata".to_owned(),
//             ))
//         });
//     }
// }

impl_lua_newtypes!{
    ( // test imports
        use bevy::math::*;
        use bevy::prelude::*;
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
            String : Primitive
            impl {
            "to" => |r,c| r.get(|s,_| Value::String(c.create_string(s.downcast_ref::<String>().unwrap()).unwrap()));
            "from" =>   |r,c,v : Value| c.coerce_string(v)?.ok_or_else(||Error::RuntimeError("Not a string".to_owned())).and_then(|string| r.get_mut(|s,_| Ok(s.apply(&string.to_str()?.to_owned()))));                             //      
            }
    },
    // ----------------------------------------------------------------- //
    // --------------------------- BEVY MATH --------------------------- //
    // ----------------------------------------------------------------- //

    // --------------------------- Vectors --------------------------- //

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
        

        impl {
            static "vec2" => |_,(x,y) : (f32,f32)| Ok(LuaVec2::new(Vec2::new(x,y)));

            (MetaMethod::Index) (s=LuaVec2)=> {|_,s,idx: usize| {Ok(s.val(|s| s[idx-1]))}};
            mut (MetaMethod::NewIndex) (n=f32) => {|_,s,(idx,val): (usize,($n))| {Ok(s.val_mut(|s| s[idx-1] = val))}};

            (MetaMethod::Pow) (s=LuaVec2,a=f32) => {|_,s : &($s), o : ($a)| { Ok(($s)::new(s.val(|s| s.powf(o)))) }};
            "abs" (s=LuaVec2,a=f32) => {|_,s : &($s),()| { Ok(($s)::new(s.inner().abs())) }};
            "signum" (s=LuaVec2,a=f32) => {|_,s : &($s),()| { Ok(($s)::new(s.inner().signum())) }};
            "round" (s=LuaVec2,a=f32) => {|_,s : &($s),()| { Ok(($s)::new(s.inner().round())) }};
            "floor" (s=LuaVec2,a=f32) => {|_,s : &($s),()| { Ok(($s)::new(s.inner().floor())) }};
            "ceil" (s=LuaVec2,a=f32) => {|_,s : &($s),()| { Ok(($s)::new(s.inner().ceil())) }};
            "fract" (s=LuaVec2,a=f32) => {|_,s : &($s),()| { Ok(($s)::new(s.inner().fract())) }};
            "exp" (s=LuaVec2,a=f32) => {|_,s : &($s),()| { Ok(($s)::new(s.inner().exp())) }};
            "recip" (s=LuaVec2,a=f32) => {|_,s : &($s),()| { Ok(($s)::new(s.inner().recip())) }};
            "clamp" (s=LuaVec2,a=f32) => {|_,s : &($s),(min,max) :(($s),($s))| { Ok(($s)::new(s.inner().clamp(min.inner(),max.inner()))) }};
            "clamp_length" (s=LuaVec2,a=f32) => {|_,s : &($s),(min,max) :(($a),($a))| { Ok(($s)::new(s.inner().clamp_length(min,max))) }};
            "clamp_length_max" (s=LuaVec2,a=f32) => {|_,s : &($s),(max) :($a)| { Ok(($s)::new(s.inner().clamp_length_max(max))) }};
            "clamp_length_min" (s=LuaVec2,a=f32) => {|_,s : &($s),(max) :($a)| { Ok(($s)::new(s.inner().clamp_length_min(max))) }};
            "lerp" (s=LuaVec2,a=f32) => {|_,s : &($s),(o,f) :(($s),($a))| { Ok(($s)::new(s.inner().lerp(o.inner(),f))) }};
            "abs_diff_eq" (s=LuaVec2,a=f32) => {|_,s : &($s),(o,max_diff) :(($s),($a))| { Ok(s.inner().abs_diff_eq(o.inner(),max_diff)) }};
            "normalize" (s=LuaVec2) => {|_,s : &($s),()| { Ok(($s)::new(s.inner().normalize())) }};
            "normalize_or_zero" (s=LuaVec2) => {|_,s : &($s),()| { Ok(($s)::new(s.inner().normalize_or_zero())) }};
            "perp" (s=LuaVec2) => {|_,s : &($s),()| { Ok(($s)::new(s.inner().perp())) }};
            "is_normalized" (s=LuaVec2) => {|_,s : &($s),()| { Ok(s.inner().is_normalized()) }};
            "is_finite" (s=LuaVec2) => {|_,s : &($s),()| { Ok(s.inner().is_finite()) }};
            "is_nan" (s=LuaVec2) => {|_,s : &($s),()| { Ok(s.inner().is_nan()) }};
            "length" (s=LuaVec2) => {|_,s : &($s),()| { Ok(s.inner().length()) }};
            "length_squared" (s=LuaVec2) => {|_,s : &($s),()| { Ok(s.inner().length_squared()) }};
            "length_recip" (s=LuaVec2) => {|_,s : &($s),()| { Ok(s.inner().length_recip()) }};
            "min_element" (s=LuaVec2) => {|_,s : &($s),()| { Ok(s.inner().min_element()) }};
            "max_element" (s=LuaVec2) => {|_,s : &($s),()| { Ok(s.inner().max_element()) }};
            "angle_between" (s=LuaVec2) => {|_,s : &($s),o : ($s)| { Ok(s.inner().angle_between(o.inner())) }};
            "project_onto" (s=LuaVec2) => {|_,s : &($s),o : ($s)| { Ok(($s)::new(s.inner().project_onto(o.inner()))) }};
            "reject_from" (s=LuaVec2) => {|_,s : &($s),o : ($s)| { Ok(($s)::new(s.inner().reject_from(o.inner()))) }};
            "project_onto_normalized" (s=LuaVec2) => {|_,s : &($s),o : ($s)| { Ok(($s)::new(s.inner().project_onto_normalized(o.inner()))) }};
            "reject_from_normalized" (s=LuaVec2) => {|_,s : &($s),o : ($s)| { Ok(($s)::new(s.inner().reject_from_normalized(o.inner()))) }};
            "perp_dot" (s=LuaVec2) => {|_,s : &($s),o : ($s)| { Ok(s.inner().perp_dot(o.inner())) }};
            "dot" (s=LuaVec2) => {|_,s : &($s),o : ($s)| { Ok(s.inner().dot(o.inner())) }};
            "distance" (s=LuaVec2) => {|_,s : &($s),o : ($s)| { Ok(s.inner().distance(o.inner())) }};
            "distance_squared" (s=LuaVec2) => {|_,s : &($s),o : ($s)| { Ok(s.inner().distance_squared(o.inner())) }};
            "min" (s=LuaVec2) => {|_,s : &($s),o : ($s)| { Ok(($s)::new(s.inner().min(o.inner()))) }};
            "max" (s=LuaVec2) => {|_,s : &($s),o : ($s)| { Ok(($s)::new(s.inner().max(o.inner()))) }};
            
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
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaVec3),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=f32),
                LuaVec2 -> (MetaMethod::Pow) (s=LuaVec3,a=f32),
                LuaVec2 -> "abs" (s=LuaVec3,a=f32),
                LuaVec2 -> "signum" (s=LuaVec3,a=f32),
                LuaVec2 -> "round" (s=LuaVec3,a=f32),
                LuaVec2 -> "floor" (s=LuaVec3,a=f32),
                LuaVec2 -> "ceil" (s=LuaVec3,a=f32),
                LuaVec2 -> "fract" (s=LuaVec3,a=f32),
                LuaVec2 -> "exp" (s=LuaVec3,a=f32),
                LuaVec2 -> "recip" (s=LuaVec3,a=f32),
                LuaVec2 -> "clamp" (s=LuaVec3,a=f32),
                LuaVec2 -> "clamp_length" (s=LuaVec3,a=f32),
                LuaVec2 -> "clamp_length_max" (s=LuaVec3,a=f32),
                LuaVec2 -> "clamp_length_min" (s=LuaVec3,a=f32),
                LuaVec2 -> "lerp" (s=LuaVec3,a=f32),
                LuaVec2 -> "abs_diff_eq" (s=LuaVec3,a=f32),
                LuaVec2 -> "normalize" (s=LuaVec3),
                LuaVec2 -> "normalize_or_zero" (s=LuaVec3),
                LuaVec2 -> "is_normalized" (s=LuaVec3),
                LuaVec2 -> "is_finite" (s=LuaVec3),
                LuaVec2 -> "is_nan" (s=LuaVec3),
                LuaVec2 -> "length" (s=LuaVec3),
                LuaVec2 -> "length_squared" (s=LuaVec3),
                LuaVec2 -> "length_recip" (s=LuaVec3),
                LuaVec2 -> "min_element" (s=LuaVec3),
                LuaVec2 -> "max_element" (s=LuaVec3),
                LuaVec2 -> "angle_between" (s=LuaVec3),
                LuaVec2 -> "project_onto" (s=LuaVec3),
                LuaVec2 -> "reject_from" (s=LuaVec3),
                LuaVec2 -> "project_onto_normalized" (s=LuaVec3),
                LuaVec2 -> "reject_from_normalized" (s=LuaVec3),
                LuaVec2 -> "dot" (s=LuaVec3),
                LuaVec2 -> "distance" (s=LuaVec3),
                LuaVec2 -> "distance_squared" (s=LuaVec3),
                LuaVec2 -> "min" (s=LuaVec3),
                LuaVec2 -> "max" (s=LuaVec3),
            )
    
        impl {
            static "vec3" => |_,(x,y,z) : (f32,f32,f32)| Ok(LuaVec3::new(Vec3::new(x,y,z)));
            "cross" (s=LuaVec3) => {|_,s : &($s),o : ($s)| { Ok(($s)::new(s.inner().cross(o.inner()))) }};
            "any_orthogonal_vector" (s=LuaVec3) => {|_,s : &($s),()| { Ok(($s)::new(s.inner().any_orthogonal_vector())) }};
            "any_orthonormal_vector" (s=LuaVec3) => {|_,s : &($s),()| { Ok(($s)::new(s.inner().any_orthonormal_vector())) }};
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
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaVec4),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=f32),
                LuaVec2 -> (MetaMethod::Pow) (s=LuaVec4,a=f32),
                LuaVec2 -> "abs" (s=LuaVec4,a=f32),
                LuaVec2 -> "signum" (s=LuaVec4,a=f32),
                LuaVec2 -> "round" (s=LuaVec4,a=f32),
                LuaVec2 -> "floor" (s=LuaVec4,a=f32),
                LuaVec2 -> "ceil" (s=LuaVec4,a=f32),
                LuaVec2 -> "fract" (s=LuaVec4,a=f32),
                LuaVec2 -> "exp" (s=LuaVec4,a=f32),
                LuaVec2 -> "recip" (s=LuaVec4,a=f32),
                LuaVec2 -> "clamp" (s=LuaVec4,a=f32),
                LuaVec2 -> "clamp_length" (s=LuaVec4,a=f32),
                LuaVec2 -> "clamp_length_max" (s=LuaVec4,a=f32),
                LuaVec2 -> "clamp_length_min" (s=LuaVec4,a=f32),
                LuaVec2 -> "lerp" (s=LuaVec4,a=f32),
                LuaVec2 -> "abs_diff_eq" (s=LuaVec4,a=f32),
                LuaVec2 -> "normalize" (s=LuaVec4),
                LuaVec2 -> "normalize_or_zero" (s=LuaVec4),
                LuaVec2 -> "is_normalized" (s=LuaVec4),
                LuaVec2 -> "is_finite" (s=LuaVec4),
                LuaVec2 -> "is_nan" (s=LuaVec4),
                LuaVec2 -> "length" (s=LuaVec4),
                LuaVec2 -> "length_squared" (s=LuaVec4),
                LuaVec2 -> "length_recip" (s=LuaVec4),
                LuaVec2 -> "min_element" (s=LuaVec4),
                LuaVec2 -> "max_element" (s=LuaVec4),
                LuaVec2 -> "project_onto" (s=LuaVec4),
                LuaVec2 -> "reject_from" (s=LuaVec4),
                LuaVec2 -> "project_onto_normalized" (s=LuaVec4),
                LuaVec2 -> "reject_from_normalized" (s=LuaVec4),
                LuaVec2 -> "dot" (s=LuaVec4),
                LuaVec2 -> "distance" (s=LuaVec4),
                LuaVec2 -> "distance_squared" (s=LuaVec4),
                LuaVec2 -> "min" (s=LuaVec4),
                LuaVec2 -> "max" (s=LuaVec4),
            )
    
        impl {
            static "vec4" => |_,(x,y,z,w) : (f32,f32,f32,f32)| Ok(LuaVec4::new(Vec4::new(x,y,z,w)));
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
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaDVec2),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=f64),
                LuaVec2 -> (MetaMethod::Pow) (s=LuaDVec2,a=f64),
                LuaVec2 -> "abs" (s=LuaDVec2,a=f64),
                LuaVec2 -> "signum" (s=LuaDVec2,a=f64),
                LuaVec2 -> "round" (s=LuaDVec2,a=f64),
                LuaVec2 -> "floor" (s=LuaDVec2,a=f64),
                LuaVec2 -> "ceil" (s=LuaDVec2,a=f64),
                LuaVec2 -> "fract" (s=LuaDVec2,a=f64),
                LuaVec2 -> "exp" (s=LuaDVec2,a=f64),
                LuaVec2 -> "recip" (s=LuaDVec2,a=f64),
                LuaVec2 -> "clamp" (s=LuaDVec2,a=f64),
                LuaVec2 -> "clamp_length" (s=LuaDVec2,a=f64),
                LuaVec2 -> "clamp_length_max" (s=LuaDVec2,a=f64),
                LuaVec2 -> "clamp_length_min" (s=LuaDVec2,a=f64),
                LuaVec2 -> "lerp" (s=LuaDVec2,a=f64),
                LuaVec2 -> "abs_diff_eq" (s=LuaDVec2,a=f64),
                LuaVec2 -> "normalize" (s=LuaDVec2),
                LuaVec2 -> "normalize_or_zero" (s=LuaDVec2),
                LuaVec2 -> "is_normalized" (s=LuaDVec2),
                LuaVec2 -> "is_finite" (s=LuaDVec2),
                LuaVec2 -> "is_nan" (s=LuaDVec2),
                LuaVec2 -> "length" (s=LuaDVec2),
                LuaVec2 -> "length_squared" (s=LuaDVec2),
                LuaVec2 -> "length_recip" (s=LuaDVec2),
                LuaVec2 -> "min_element" (s=LuaDVec2),
                LuaVec2 -> "max_element" (s=LuaDVec2),
                LuaVec2 -> "angle_between" (s=LuaDVec2),
                LuaVec2 -> "project_onto" (s=LuaDVec2),
                LuaVec2 -> "reject_from" (s=LuaDVec2),
                LuaVec2 -> "project_onto_normalized" (s=LuaDVec2),
                LuaVec2 -> "reject_from_normalized" (s=LuaDVec2),
                LuaVec2 -> "dot" (s=LuaDVec2),
                LuaVec2 -> "distance" (s=LuaDVec2),
                LuaVec2 -> "distance_squared" (s=LuaDVec2),
                LuaVec2 -> "min" (s=LuaDVec2),
                LuaVec2 -> "max" (s=LuaDVec2),
                LuaVec2 -> "perp" (s=LuaDVec2),
                LuaVec2 -> "perp_dot" (s=LuaDVec2),
            )
    
        impl {
            static "DVec2" => |_,(x,y) : (f64,f64)| Ok(LuaDVec2::new(DVec2::new(x,y)));
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
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaDVec3),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=f64),
                LuaVec2 -> (MetaMethod::Pow) (s=LuaDVec3,a=f64),
                LuaVec2 -> "abs" (s=LuaDVec3,a=f64),
                LuaVec2 -> "signum" (s=LuaDVec3,a=f64),
                LuaVec2 -> "round" (s=LuaDVec3,a=f64),
                LuaVec2 -> "floor" (s=LuaDVec3,a=f64),
                LuaVec2 -> "ceil" (s=LuaDVec3,a=f64),
                LuaVec2 -> "fract" (s=LuaDVec3,a=f64),
                LuaVec2 -> "exp" (s=LuaDVec3,a=f64),
                LuaVec2 -> "recip" (s=LuaDVec3,a=f64),
                LuaVec2 -> "clamp" (s=LuaDVec3,a=f64),
                LuaVec2 -> "clamp_length" (s=LuaDVec3,a=f64),
                LuaVec2 -> "clamp_length_max" (s=LuaDVec3,a=f64),
                LuaVec2 -> "clamp_length_min" (s=LuaDVec3,a=f64),
                LuaVec2 -> "lerp" (s=LuaDVec3,a=f64),
                LuaVec2 -> "abs_diff_eq" (s=LuaDVec3,a=f64),
                LuaVec2 -> "normalize" (s=LuaDVec3),
                LuaVec2 -> "normalize_or_zero" (s=LuaDVec3),
                LuaVec2 -> "is_normalized" (s=LuaDVec3),
                LuaVec2 -> "is_finite" (s=LuaDVec3),
                LuaVec2 -> "is_nan" (s=LuaDVec3),
                LuaVec2 -> "length" (s=LuaDVec3),
                LuaVec2 -> "length_squared" (s=LuaDVec3),
                LuaVec2 -> "length_recip" (s=LuaDVec3),
                LuaVec2 -> "min_element" (s=LuaDVec3),
                LuaVec2 -> "max_element" (s=LuaDVec3),
                LuaVec2 -> "angle_between" (s=LuaDVec3),
                LuaVec2 -> "project_onto" (s=LuaDVec3),
                LuaVec2 -> "reject_from" (s=LuaDVec3),
                LuaVec2 -> "project_onto_normalized" (s=LuaDVec3),
                LuaVec2 -> "reject_from_normalized" (s=LuaDVec3),
                LuaVec2 -> "dot" (s=LuaDVec3),
                LuaVec2 -> "distance" (s=LuaDVec3),
                LuaVec2 -> "distance_squared" (s=LuaDVec3),
                LuaVec2 -> "min" (s=LuaDVec3),
                LuaVec2 -> "max" (s=LuaDVec3),
                LuaVec3 -> "cross" (s=LuaDVec3),
                LuaVec3 -> "any_orthogonal_vector" (s=LuaDVec3), 
                LuaVec3 -> "any_orthonormal_vector" (s=LuaDVec3), 
                LuaVec3 -> "any_orthonormal_pair" (s=LuaDVec3),
            )
        impl {
            static "dvec3" => |_,(x,y,z) : (f64,f64,f64)| Ok(LuaDVec3::new(DVec3::new(x,y,z)));
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
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaDVec4),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=f64),
                LuaVec2 -> (MetaMethod::Pow) (s=LuaDVec4,a=f64),
                LuaVec2 -> "abs" (s=LuaDVec4,a=f64),
                LuaVec2 -> "signum" (s=LuaDVec4,a=f64),
                LuaVec2 -> "round" (s=LuaDVec4,a=f64),
                LuaVec2 -> "floor" (s=LuaDVec4,a=f64),
                LuaVec2 -> "ceil" (s=LuaDVec4,a=f64),
                LuaVec2 -> "fract" (s=LuaDVec4,a=f64),
                LuaVec2 -> "exp" (s=LuaDVec4,a=f64),
                LuaVec2 -> "recip" (s=LuaDVec4,a=f64),
                LuaVec2 -> "clamp" (s=LuaDVec4,a=f64),
                LuaVec2 -> "clamp_length" (s=LuaDVec4,a=f64),
                LuaVec2 -> "clamp_length_max" (s=LuaDVec4,a=f64),
                LuaVec2 -> "clamp_length_min" (s=LuaDVec4,a=f64),
                LuaVec2 -> "lerp" (s=LuaDVec4,a=f64),
                LuaVec2 -> "abs_diff_eq" (s=LuaDVec4,a=f64),
                LuaVec2 -> "normalize" (s=LuaDVec4),
                LuaVec2 -> "normalize_or_zero" (s=LuaDVec4),
                LuaVec2 -> "is_normalized" (s=LuaDVec4),
                LuaVec2 -> "is_finite" (s=LuaDVec4),
                LuaVec2 -> "is_nan" (s=LuaDVec4),
                LuaVec2 -> "length" (s=LuaDVec4),
                LuaVec2 -> "length_squared" (s=LuaDVec4),
                LuaVec2 -> "length_recip" (s=LuaDVec4),
                LuaVec2 -> "min_element" (s=LuaDVec4),
                LuaVec2 -> "max_element" (s=LuaDVec4),
                LuaVec2 -> "project_onto" (s=LuaDVec4),
                LuaVec2 -> "reject_from" (s=LuaDVec4),
                LuaVec2 -> "project_onto_normalized" (s=LuaDVec4),
                LuaVec2 -> "reject_from_normalized" (s=LuaDVec4),
                LuaVec2 -> "dot" (s=LuaDVec4),
                LuaVec2 -> "distance" (s=LuaDVec4),
                LuaVec2 -> "distance_squared" (s=LuaDVec4),
                LuaVec2 -> "min" (s=LuaDVec4),
                LuaVec2 -> "max" (s=LuaDVec4),
            )
        impl {
            static "dvec4" => |_,(x,y,z,w) : (f64,f64,f64,f64)| Ok(LuaDVec4::new(DVec4::new(x,y,z,w)));
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
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaIVec2),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=i32),
                LuaVec2 -> "abs" (s=LuaIVec2,a=i32),
                LuaVec2 -> "signum" (s=LuaIVec2,a=i32),
                LuaVec2 -> "clamp" (s=LuaIVec2,a=i32),
                LuaVec2 -> "min_element" (s=LuaIVec2),
                LuaVec2 -> "max_element" (s=LuaIVec2),
                LuaVec2 -> "dot" (s=LuaIVec2),
                LuaVec2 -> "min" (s=LuaIVec2),
                LuaVec2 -> "max" (s=LuaIVec2),
                LuaVec2 -> "perp" (s=LuaIVec2),
                LuaVec2 -> "perp_dot" (s=LuaIVec2),
            )
    
        impl {
            static "ivec2" => |_,(x,y) : (i32,i32)| Ok(LuaIVec2::new(IVec2::new(x,y)));
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
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaIVec3),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=i32),
                LuaVec2 -> "abs" (s=LuaIVec3,a=i32),
                LuaVec2 -> "signum" (s=LuaIVec3,a=i32),
                LuaVec2 -> "clamp" (s=LuaIVec3,a=i32),
                LuaVec2 -> "min_element" (s=LuaIVec3),
                LuaVec2 -> "max_element" (s=LuaIVec3),
                LuaVec2 -> "dot" (s=LuaIVec3),
                LuaVec2 -> "min" (s=LuaIVec3),
                LuaVec2 -> "max" (s=LuaIVec3),
                LuaVec3 -> "cross" (s=LuaIVec3),
            )
    
        impl {
            static "ivec3" => |_,(x,y,z) : (i32,i32,i32)| Ok(LuaIVec3::new(IVec3::new(x,y,z)));
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
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaIVec4),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=i32),
                LuaVec2 -> "abs" (s=LuaIVec4,a=i32),
                LuaVec2 -> "signum" (s=LuaIVec4,a=i32),
                LuaVec2 -> "clamp" (s=LuaIVec4,a=i32),
                LuaVec2 -> "min_element" (s=LuaIVec4),
                LuaVec2 -> "max_element" (s=LuaIVec4),
                LuaVec2 -> "dot" (s=LuaIVec4),
                LuaVec2 -> "min" (s=LuaIVec4),
                LuaVec2 -> "max" (s=LuaIVec4),
            )
    
        impl {
            static "ivec4" => |_,(x,y,z,w) : (i32,i32,i32,i32)| Ok(LuaIVec4::new(IVec4::new(x,y,z,w)));
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
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaUVec2),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=u32),
                LuaVec2 -> "clamp" (s=LuaUVec2,a=u32),
                LuaVec2 -> "min_element" (s=LuaUVec2),
                LuaVec2 -> "max_element" (s=LuaUVec2),
                LuaVec2 -> "dot" (s=LuaUVec2),
                LuaVec2 -> "min" (s=LuaUVec2),
                LuaVec2 -> "max" (s=LuaUVec2),
            )
    
        impl {
            static "uvec2" => |_,(x,y) : (u32,u32)| Ok(LuaUVec2::new(UVec2::new(x,y)));
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
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaUVec3),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=u32),
                LuaVec2 -> "clamp" (s=LuaUVec3,a=u32),
                LuaVec2 -> "min_element" (s=LuaUVec3),
                LuaVec2 -> "max_element" (s=LuaUVec3),
                LuaVec2 -> "dot" (s=LuaUVec3),
                LuaVec2 -> "min" (s=LuaUVec3),
                LuaVec2 -> "max" (s=LuaUVec3),
                LuaVec3 -> "cross" (s=LuaUVec3),
            )
        impl {
            static "uvec3" => |_,(x,y,z) : (u32,u32,u32)| Ok(LuaUVec3::new(UVec3::new(x,y,z)));
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
            // + MathUnaryOp(Unm:)
            + Copy(
                LuaVec2 -> (MetaMethod::Index) (s=LuaUVec4),
                LuaVec2 -> mut (MetaMethod::NewIndex) (n=u32),
                LuaVec2 -> "clamp" (s=LuaUVec4,a=u32),
                LuaVec2 -> "min_element" (s=LuaUVec4),
                LuaVec2 -> "max_element" (s=LuaUVec4),
                LuaVec2 -> "dot" (s=LuaUVec4),
                LuaVec2 -> "min" (s=LuaUVec4),
                LuaVec2 -> "max" (s=LuaUVec4),
            )
    
        impl {
            static "uvec4" => |_,(x,y,z,w) : (u32,u32,u32,u32)| Ok(LuaUVec4::new(UVec4::new(x,y,z,w)));
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

        impl{       
            static "mat3" => |_,(x,y,z) : (LuaVec3,LuaVec3,LuaVec3)| Ok(LuaMat3::new(Mat3::from_cols(x.inner(),y.inner(),z.inner())));
              
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

            "transpose" (s=LuaMat3) => {|_,s,()| Ok(($s)::new(s.val(|s| s.transpose())))};
            "determinant" (s=LuaMat3) => {|_,s,()| Ok(s.val(|s| s.determinant()))};
            "inverse" (s=LuaMat3) => {|_,s,()| Ok(($s)::new(s.val(|s| s.inverse())))};
            "is_nan"(s=LuaMat3) => {|_,s,()| Ok(s.val(|s| s.is_nan()))};
            "is_finite" (s=LuaMat3) => {|_,s,()| Ok(s.val(|s| s.is_finite()))};
            "transform_point2" => |_,s,o:LuaVec2| Ok(s.val(|s| LuaVec2::new(s.transform_point2(o.inner()))));
            "transform_vector2" => |_,s,o:LuaVec2| Ok(s.val(|s| LuaVec2::new(s.transform_vector2(o.inner()))));
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
            + Copy(
                LuaMat3 -> mut (MetaMethod::Index) (s=LuaMat4,b=Mat4,v=LuaVec4),
                LuaMat3 -> "transpose" (s=LuaMat4),
                LuaMat3 -> "determinant" (s=LuaMat4),
                LuaMat3 -> "inverse" (s=LuaMat4),
                LuaMat3 -> "is_nan"(s=LuaMat4),
                LuaMat3 -> "is_finite" (s=LuaMat4),
            )
        impl {
            static "mat4" => |_,(x,y,z,w) : (LuaVec4,LuaVec4,LuaVec4,LuaVec4)| Ok(LuaMat4::new(Mat4::from_cols(x.inner(),y.inner(),z.inner(),w.inner())));
            "transform_point3" (v=LuaVec3) => {|_,s,o:($v)| Ok(s.val(|s| ($v)::new(s.transform_point3(o.inner()))))};
            "project_point3" (v=LuaVec3) => {|_,s,o:($v)| Ok(s.val(|s| ($v)::new(s.project_point3(o.inner()))))};
            "transform_vector3" (v=LuaVec3) => {|_,s,o:($v)| Ok(s.val(|s| ($v)::new(s.transform_vector3(o.inner()))))};
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
            + Copy(
                LuaMat3 -> mut (MetaMethod::Index) (s=LuaDMat3,b=DMat3,v=LuaDVec3),
                LuaMat3 -> "transpose" (s=LuaDMat3),
                LuaMat3 -> "determinant" (s=LuaDMat3),
                LuaMat3 -> "inverse" (s=LuaDMat3),
                LuaMat3 -> "is_nan"(s=LuaDMat3),
                LuaMat3 -> "is_finite" (s=LuaDMat3),
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
            + Copy(
                LuaMat3 -> mut (MetaMethod::Index) (s=LuaDMat4,b=DMat4,v=LuaDVec4),
                LuaMat3 -> "transpose" (s=LuaDMat4),
                LuaMat3 -> "determinant" (s=LuaDMat4),
                LuaMat3 -> "inverse" (s=LuaDMat4),
                LuaMat3 -> "is_nan"(s=LuaDMat4),
                LuaMat3 -> "is_finite" (s=LuaDMat4),
                LuaMat4 -> "transform_point3" (v=LuaDVec3),
                LuaMat4 -> "project_point3" (v=LuaDVec3),
                LuaMat4 -> "transform_vector3" (v=LuaDVec3),
            )
        impl {
            static "dmat4" => |_,(x,y,z,w) : (LuaDVec4,LuaDVec4,LuaDVec4,LuaDVec4)| Ok(LuaDMat4 ::new(DMat4::from_cols(x.inner(),y.inner(),z.inner(),w.inner())));
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
            + MathBinOp(Mul:
                Number[&self(Rhs:f32)])
            + MathUnaryOp(Unm:)

        impl {
            static "quat" => |_,(x,y,z,w) : (f32,f32,f32,f32)| Ok(LuaQuat::new(Quat::from_xyzw(x,y,z,w)));

            "to_axis_angle" (v=LuaVec3) => {|_,s,()| {
                                                let (v,f) = s.val(|v| v.to_axis_angle());
                                                let o = (($v)::new(v),f);
                                                Ok(o)
                                            }};
            "to_scaled_axis" (v=LuaVec3) => {|_,s,()| Ok(($v)::new(s.val(|v| v.to_scaled_axis())))};
            "xyz" (v=LuaVec3) => {|_,s,()| Ok(($v)::new(s.val(|v| v.xyz())))};
            "conjugate" (s=LuaQuat) => {|_,s,()| Ok(($s)::new(s.val(|v| v.conjugate())))};
            "inverse" (s=LuaQuat) => {|_,s,()| Ok(($s)::new(s.val(|v| v.inverse())))};
            "length" (s=LuaQuat) => {|_,s,()| Ok(s.val(|v| v.length()))};
            "length_squared" (s=LuaQuat) => {|_,s,()| Ok(s.val(|v| v.length_squared()))};
            "length_recip" (s=LuaQuat) => {|_,s,()| Ok(s.val(|v| v.length_recip()))};
            "normalize" (s=LuaQuat) => {|_,s,()| Ok(($s)::new(s.val(|v| v.normalize())))};
            "is_finite" (s=LuaQuat) => {|_,s,()| Ok(s.val(|v| v.is_finite()))};
            "is_nan" (s=LuaQuat) => {|_,s,()| Ok(s.val(|v| v.is_nan()))};
            "is_normalized" (s=LuaQuat) => {|_,s,()| Ok(s.val(|v| v.is_normalized()))};
            "is_near_identity" (s=LuaQuat) => {|_,s,()| Ok(s.val(|v| v.is_near_identity()))};
            "to_euler" (s=LuaQuat) => {|_,s,e : LuaEulerRot| Ok(s.val(|v| v.to_euler(e.rot)))};
            "dot" (s=LuaQuat) => {|_,s,o : ($s)| Ok(s.bin(&o,|s,o| s.dot(o.inner())))};
            "angle_between"(s=LuaQuat) => {|_,s,o : ($s)| Ok(s.bin(&o,|s,o| s.angle_between(o.inner())))};
            "abs_diff_eq" (s=LuaQuat,n=f32) => {|_,s,(o,diff) : (($s),($n))| Ok(s.bin(&o,|s,o| s.abs_diff_eq(o.inner(),diff)))};
            "lerp" (s=LuaQuat,n=f32) => {|_,s,(o,f) : (($s),($n))| Ok(($s)::new(s.bin(&o,|s,o| s.lerp(o.inner(),f))))};
            "slerp"(s=LuaQuat,n=f32) => {|_,s,(o,f) : (($s),($n))| Ok(($s)::new(s.bin(&o,|s,o| s.slerp(o.inner(),f))))};
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
            LuaWrapper[&self(Rhs:LuaDQuat)])
        + MathBinOp(Mul:
            LuaWrapper[&self(Rhs:LuaDQuat),
            &self(Rhs:LuaDVec3->LuaWrapper(LuaDVec3))],
            Number[&self(Rhs:f64)])
        + MathBinOp(Div:
            Number[&self(Rhs:f64)])
        + MathBinOp(Mul:
            Number[&self(Rhs:f64)])
        + MathUnaryOp(Unm:)
        
        + Copy(
            LuaQuat -> "to_axis_angle" (v=LuaDVec3),
            LuaQuat -> "to_scaled_axis" (v=LuaDVec3),
            LuaQuat -> "xyz" (v=LuaDVec3),
            LuaQuat -> "conjugate" (s=LuaDQuat),
            LuaQuat -> "inverse" (s=LuaDQuat),
            LuaQuat -> "length" (s=LuaDQuat),
            LuaQuat -> "length_squared" (s=LuaDQuat),
            LuaQuat -> "length_recip" (s=LuaDQuat),
            LuaQuat -> "normalize" (s=LuaDQuat),
            LuaQuat -> "is_finite" (s=LuaDQuat),
            LuaQuat -> "is_nan" (s=LuaDQuat),
            LuaQuat -> "is_normalized" (s=LuaDQuat),
            LuaQuat -> "is_near_identity" (s=LuaDQuat),
            LuaQuat -> "to_euler" (s=LuaDQuat),
            LuaQuat -> "dot" (s=LuaDQuat),
            LuaQuat -> "angle_between"(s=LuaDQuat),
            LuaQuat -> "abs_diff_eq" (s=LuaDQuat,n=f64),
            LuaQuat -> "lerp" (s=LuaDQuat,n=f64),
            LuaQuat -> "slerp"(s=LuaDQuat,n=f64),
        )
        impl {
            static "dquat" => |_,(x,y,z,w) : (f64,f64,f64,f64)| Ok(LuaDQuat::new(DQuat::from_xyzw(x,y,z,w)));
        }
    },
    {
        bevy_ecs::entity::Entity: Full 
        impl {
            "id" => |_,s : &LuaEntity, ()| Ok(s.val(|v| v.id()));
            "generation" => |_,s : &LuaEntity, ()| Ok(s.val(|v| v.generation()));
            "bits" => |_,s : &LuaEntity, ()| Ok(s.val(|v| v.to_bits()));
        }
    },
    {
        bevy_ecs::world::World: NonAssignable{pub world: Weak<RwLock<World>>}

        impl {
            "add_component" =>  |_, world, (entity, comp_name): (LuaEntity, String)| {
                 // grab this entity before acquiring a lock in case it's a reference
                 let entity = entity.inner();
                 let w = world.world.upgrade().unwrap();
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
                             world: world.world.clone()
                         }, 
                         path: Some("".to_string()), 
                         r: ReflectPtr::Const(refl.reflect_component(w,entity).unwrap())
                     }    
                 })
            };

            "add_component" =>  |_, world, (entity, comp_name): (LuaEntity, String)| {
                // grab this entity before acquiring a lock in case it's a reference
                let entity = entity.inner();

                let w = world.world.upgrade().unwrap();
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
                            world: world.world.clone()
                        }, 
                        path: Some("".to_string()), 
                        r: ReflectPtr::Const(refl.reflect_component(w,entity).unwrap())
                    }    
                })
            };

            "get_component" => |_, world, (entity, comp_name) : (LuaEntity,String)| {

                // grab this entity before acquiring a lock in case it's a reference
                let entity = entity.inner();

                let w = world.world.upgrade().unwrap();
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
                                world: world.world.clone()
                            }, 
                            path: Some("".to_string()), 
                            r: ReflectPtr::Const(dyn_comp)
                        }    
                    }  
                )
            };

            "new_script_entity" => |_, world, name: String| {
                let w = world.world.upgrade().unwrap();
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
                let w = world.world.upgrade().unwrap();
                let w = &mut w.write();                
                
                Ok(LuaEntity::new(w.spawn().id()))
            };
        }
    },
    {
        glam::euler::EulerRot: NonAssignable{pub rot: EulerRot} 
    }
    ]

}