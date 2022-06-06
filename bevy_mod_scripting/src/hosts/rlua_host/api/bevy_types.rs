use bevy::reflect::TypeData;
use bevy::reflect::TypeRegistry;
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

macro_rules! make_lua_types {
    (   
        userdata: [
            $(
                $str:expr ;=> $name:ty:$(($($inner:tt)*))?{
                    $(##[glob] $glob_name:expr => $global_fn:expr;)*
                    $(#[$e:tt] $g:expr => $f:expr;)*

                }
            ),*
        ]
        non_assignable_ud: [
            $(
                $na_str:expr ;=> $na_name:ty:$(($($na_inner:tt)*))?{
                    $(##[glob] $na_glob_name:expr => $na_global_fn:expr;)*
                    $(#[$na_e:tt] $na_g:expr => $na_f:expr;)*
                }
            ),*     
        ]
        primitives: [
            $(
            $primitive_str:expr ;=> $primitive_base:ty : {
                #[from] $primitive_from:expr;
                #[to] $primitive_to:expr;
            }
            ),*
        ]
    ) => {
        paste!{

            // constructors/api 
            #[derive(Default)]
            pub struct LuaBevyAPI;

            impl APIProvider for LuaBevyAPI {
                type Ctx = Mutex<Lua>;
                fn attach_api(c: &mut <Self as APIProvider>::Ctx) {
                    c.lock()
                    .expect("Could not get lock on script context")
                    .context::<_, Result<(), ScriptError>>(|lua_ctx| {
                        let g = lua_ctx.globals();
                        $($(
                            g.set($glob_name, lua_ctx.create_function($global_fn)?)?;
                        )*)*
                        $($(
                            g.set($na_glob_name, lua_ctx.create_function($na_global_fn)?)?;
                        )*)*

                        Ok(())
                    }).unwrap();
                }
            }

            // structs

            $(
                make_lua_struct!(
                    $name: $( ($($inner)*) )? {
                        $(#[$e] $g => $f;)*
                    }
                );
            )*
            $(
                make_lua_struct!(
                    $na_name: $( ($($na_inner)*) )? {
                        $(#[$na_e] $na_g => $na_f;)*
                    }
                );
            )*

            pub static BEVY_TO_LUA: Map<&'static str,
                for<'l> fn(&LuaRef,Context<'l>) -> Value<'l>
            > = phf_map!{
                $(
                    $str => |r,c| {
                        let usr = c.create_userdata([<Lua $name>]::base_to_self(r)).unwrap();
                        Value::UserData(usr)
                    }
                ),*,
                $(
                    $primitive_str => $primitive_from
                ),*
            };

            pub static APPLY_LUA_TO_BEVY: Map<&'static str,
                for<'l> fn(&mut LuaRef, Context<'l>, Value<'l>) -> Result<(),Error>
            > = phf_map!{
                $(
                    $str => |r,c,n| {

                    if let Value::UserData(v) = n {
                        let mut v = v.borrow_mut::<[<Lua $name>]>()?;
                        [<Lua $name>]::apply_self_to_base(v.deref_mut(),r);
                        Ok(())
                    } else {
                        Err(Error::RuntimeError("Invalid type".to_owned()))
                    }
                    }
                ),*,
                $(
                    $primitive_str => $primitive_to
                ),*
            }; 

            #[cfg(test)]
            mod tests {
                use bevy::prelude::*;
                use bevy::math::*;

                $(
                    #[test]
                    fn [<test_ $name:snake >]() {
                        assert_eq!(std::any::type_name::<$name>(),$str);
                    }
                )*
            }

        }
    }
}


macro_rules! make_add_method {
    (   
        $methods:expr,
        #[meta] $mm:expr => $f:expr
    ) => {
        $methods.add_meta_method($mm,$f)
    };
    (   
        $methods:expr,
        #[meta_mut] 
        $mm:expr => $f:expr
    ) => {
        $methods.add_meta_method_mut($mm,$f)
    };
    (   
        $methods:expr,
        #[func] $mm:expr => $f:expr
    ) => {
        $methods.add_method($mm,$f)
    };
    (   
        $methods:expr,
        #[func_mut] $mm:expr => $f:expr
    ) => {
        $methods.add_method_mut($mm,$f)
    };
}

macro_rules! make_lua_struct {
    (
    $base:ty:{
        $(#[$e:tt] $g:expr => $f:expr;)*
    }

    ) => {
        paste!{
            #[derive(Debug,Clone)]
            pub enum [<Lua $base>] { 
                Owned($base,Arc<RwLock<()>>),
                Ref(LuaRef)
            }
            
            impl Drop for [<Lua $base>] {
                fn drop(&mut self) {
                    match self {
                        [<Lua $base>]::Owned(_,valid) => {
                            if valid.is_locked() {
                                panic!("Something is referencing {self:?} and it's about to go out of scope!");
                            }
                        },
                        [<Lua $base>]::Ref(_) => {},
                    }
                }
            }

            impl Display for [<Lua $base>] {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
                    write!(f,"{:#?}", self)
                }
            }

            impl [<Lua $base>] {

                pub fn new(b : $base) -> Self {
                    Self::Owned(b,Arc::new(RwLock::new(())))
                }

                pub fn new_ref(b : &LuaRef) -> Self {
                    Self::Ref(b.clone())
                }

                /// Perform an operation on the base type and optionally retrieve something by value
                /// may require a read lock on the world in case this is a reference
                pub fn val<G,F>(&self, accessor: F) -> G
                    where 
                    F: FnOnce(&$base) -> G
                {
                    match self {
                        [<Lua $base>]::Owned(ref v, ..) => accessor(v),
                        [<Lua $base>]::Ref(v) => {
                            v.get(|s,_| accessor(s.downcast_ref::<$base>().unwrap()))
                        },
                    }
                }

            pub fn val_mut<G,F>(&mut self, accessor: F) -> G
                where 
                F: FnOnce(&mut $base) -> G
            {
                match self {
                    [<Lua $base>]::Owned(ref mut v, ..) => accessor(v),
                    [<Lua $base>]::Ref(v) => {
                        v.get_mut(|s,_| accessor(s.downcast_mut::<$base>().unwrap()))
                    },
                }
            }

                /// Perform a binary operation on self and another base type and optionally retrieve something by value,
                /// may require a read lock on the world in case this is a reference
                pub fn bin<G,F>(&self,o: &[<Lua $base>], bin: F) -> G
                where 
                F: FnOnce(&$base,&$base) -> G
                {
                    match (self,o) {
                        ([<Lua $base>]::Owned(ref v, ..),[<Lua $base>]::Owned(ref o, ..)) => bin(v,o),
                        ([<Lua $base>]::Owned(ref v, ..),[<Lua $base>]::Ref(o)) => o.get(|o,_| bin(v,o.downcast_ref::<$base>().unwrap())),
                        ([<Lua $base>]::Ref(ref v),[<Lua $base>]::Owned(o, ..)) => v.get(|v,_| bin(v.downcast_ref::<$base>().unwrap(),o)),
                        ([<Lua $base>]::Ref(ref v),[<Lua $base>]::Ref(o)) => o.get(|o,_| v.get(|v,_| bin(v.downcast_ref::<$base>().unwrap(),o.downcast_ref::<$base>().unwrap()))),
                    }
                }

                

                /// Perform a binary operation on self and any other type and optionally retrieve something by value,
                /// may require a read lock on the world in case this is a reference
                pub fn binv<O,G,F>(&self,o: &O, binv: F) -> G
                where 
                F: FnOnce(&$base,&O) -> G
                {
                    match self {
                        [<Lua $base>]::Owned(ref v, ..) => binv(v,o),
                        [<Lua $base>]::Ref(ref v) => v.get(|v,_| binv(v.downcast_ref::<$base>().unwrap(),o)),
                    }
                }

                /// returns wrapped value by value, 
                /// may require a read lock on the world in case this is a reference
                pub fn inner(&self) -> $base
                {
                    match self {
                        [<Lua $base>]::Owned(ref v, ..) => *v,
                        [<Lua $base>]::Ref(v) => {
                            v.get(|s,_| *s.downcast_ref::<$base>().unwrap())
                        },
                    }
                }

                /// Converts a LuaRef to Self
                pub fn base_to_self(b: &LuaRef) -> Self {
                    [<Lua $base>]::Ref(b.clone())
                }

                /// Applies Self to a LuaRef.
                /// may require a write lock on the world
                pub fn apply_self_to_base(&self, b: &mut LuaRef){

                    match self {
                        [<Lua $base>]::Owned(ref v, ..) => {
                            // if we own the value, we are not borrowing from the world
                            // we're good to just apply, yeet
                            b.get_mut(|b,_| b.apply(v))
                        },
                        [<Lua $base>]::Ref(v) => {
                            // if we are a luaref, we have to be careful with borrows
                            b.apply_luaref(v)
                        }
                    }
                }

            }

            impl UserData for [<Lua $base>] {
                fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
                    // automatically generate 
                    methods.add_meta_method(MetaMethod::ToString, |_,s,()|{
                        Ok(format!("{}",s))
                    });


                    $(
                        make_add_method!(methods,#[$e] $g => $f);
                    )*
                }
            }
        }
    };
    // any old plain clone type with only string defaults
    (
        $base:ty:($($inner_tkns:tt)*) {
            $(#[$e:tt] $g:expr => $f:expr;)*
            $(#[struct_func] $($struct_func:tt)*;)*
        }
    ) => {
        paste!{
            #[derive(Debug,Clone)]
            pub struct [<Lua $base>] ($($inner_tkns)*);
            

            impl [<Lua $base>] {
                $(
                    $($struct_func)*
                )*
            }


            impl Display for [<Lua $base>] {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
                    write!(f,"{:#?}", self)
                }
            }

            impl UserData for [<Lua $base>] {
                fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
                    // automatically generate 
                    methods.add_meta_method(MetaMethod::ToString, |_,s,()|{
                        Ok(format!("{}",s))
                    });

                    $(
                        make_add_method!(methods,#[$e] $g => $f);
                    )*
                }
            }
        }
    };

}


macro_rules! make_it_all_baby {
    (
        vectors: [
            $(
                $vec_str:expr ;=> $vec_base:ty, $vec_num:ty $(:+:  $vec_float_inner:ty)? :{  
                    $($vec_inner:tt)* 
                } 
            ),*
        ]
        matrices: [
            $(
                $mat_str:expr ;=> $mat_base:ty, $mat_col:ty, $mat_num:ty :{  
                    $($mat_inner:tt)* 
                } 
            ),*
        ]

        quats : [
            $(
                $quat_str:expr ;=> $quat_base:ty, $quat_vec:ty ,$quat_num:ty :{  
                    $($quat_inner:tt)* 
                } 
            ),*
        ]

        primitives: [
            $(
                $primitive_inner:tt
            )*
        ]
        other: [$($o:tt)+]
        non_assignable: [$($na_inner:tt)*]
    ) => {
        paste!(
                make_lua_types!{
                    userdata: [   
                        // vectors
                        $(

                            $vec_str ;=> $vec_base : {
                                $($vec_inner)*

                                $(
                                    // $vec_base $vec_float_inner
                                    #[meta] MetaMethod::Pow => |_,s : &[<Lua $vec_base>], o : $vec_float_inner| { Ok([<Lua $vec_base>]::new(s.val(|s| s.powf(o)))) };
                                    #[meta] MetaMethod::Unm => |_,s : &[<Lua $vec_base>],()| { Ok(([<Lua $vec_base>]::new(s.inner().neg()))) };
                                    #[func] "abs" => |_,s : &[<Lua $vec_base>],()| { Ok([<Lua $vec_base>]::new(s.inner().abs())) };
                                    #[func] "signum" => |_,s : &[<Lua $vec_base>],()| { Ok([<Lua $vec_base>]::new(s.inner().signum())) };
                                )?
                                #[func] "dot" => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok(s.inner().dot(o.inner())) };
                                #[func] "min_element" => |_,s : &[<Lua $vec_base>],()| { Ok(s.inner().min_element()) };
                                #[func] "max_element" => |_,s : &[<Lua $vec_base>],()| { Ok(s.inner().max_element()) };
                                #[func] "min" => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.inner().min(o.inner()))) };
                                #[func] "max" => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.inner().max(o.inner()))) };
                                #[func] "clamp" => |_,s : &[<Lua $vec_base>],(o,max) : ([<Lua $vec_base>],[<Lua $vec_base>])| { Ok([<Lua $vec_base>]::new(s.inner().clamp(o.inner(),max.inner()))) };

                                #[meta] MetaMethod::Add => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.inner().add(o.inner()))) };
                                #[meta] MetaMethod::Sub => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.inner().sub(o.inner()))) };
                                #[meta] MetaMethod::Mul => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.inner().mul(o.inner()))) };
                                #[meta] MetaMethod::Div => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.inner().div(o.inner()))) };
                                #[meta] MetaMethod::Mod => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.inner().rem(o.inner()))) };
                                #[meta] MetaMethod::Eq => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok((s.bin(&o,|s,o| s.eq(o)))) };
                                #[meta_mut] MetaMethod::Index => |_,s : &mut [<Lua $vec_base>],idx : String| { 
                                    let idx = match idx.as_str() {
                                        "x" => 0,
                                        "y" => 1,
                                        "z" => 2,
                                        "w" => 3,
                                        _ => Err(Error::RuntimeError(format!("Cannot index {} with {:#?}",stringify!($vec_base),idx)))?
                                    };
                                    Ok(s.val(|s| s[idx]))
                                };
                                #[meta_mut] MetaMethod::NewIndex => |_,s : &mut [<Lua $vec_base>],(idx,val) : (Value,$vec_num)| { // (Value,$vec_num) 
                                    match idx {
                                        Value::Integer(v) => Ok(s.val_mut(|s| s[v as usize] = val)),
                                        Value::String(ref v) => {
                                            let idx = match v.to_str()? {
                                                "x" => 0,
                                                "y" => 1,
                                                "z" => 2,
                                                "w" => 3,
                                                _ => Err(Error::RuntimeError(format!("Cannot index {} with {:#?}",stringify!($vec_base),idx)))?
                                            };
                                            // if our wrapper holds a reference it means it is an immediate indexing into
                                            // the original value, i.e. some_struct.our_vec[idx] = value
                                            Ok(s.val_mut(|s| s[idx] = val))

                                        },
                                        _ => Err(Error::RuntimeError(format!("Cannot index {} with {:#?}",stringify!($vec_base),idx)))
                                        
                                    }
                                };

                            },
                        )*
                        // matrices 
                        $(

                            $mat_str ;=> $mat_base : {
                                $($mat_inner)*

                                #[func_mut] "col" => |_,s,idx : usize| {
                                    match s {
                                        [<Lua $mat_base>]::Owned(ref mut v, ref valid) => {
                                            Ok([<Lua $mat_col>]::Ref(LuaRef{
                                                root: LuaRefBase::LuaOwned{valid: Arc::downgrade((valid))},
                                                r: ReflectPtr::Mut(v.col_mut(idx)),
                                                path: None
                                            }))
                                        },
                                        [<Lua $mat_base>]::Ref(ref mut r) => {
                                            // if this is rust owned,
                                            // we need to make sure to clear the reflection path
                                            // since it's not relevant anymore
                                            r.get_mut(|s,r| {
                                                Ok([<Lua $mat_col>]::Ref(LuaRef{
                                                    root: r.root.clone(),
                                                    r: ReflectPtr::Mut(s.downcast_mut::<$mat_base>().unwrap().col_mut(idx)),
                                                    path: None
                                                })) 
                                            })
                                        },
                                    }

                                };
                                #[func] "transpose" => |_,s,()| Ok([<Lua $mat_base>]::new(s.val(|s| s.transpose())));
                                #[func] "determinant" => |_,s,()| Ok(s.val(|s| s.determinant()));
                                #[func] "inverse" => |_,s,()| Ok([<Lua $mat_base>]::new(s.val(|s| s.inverse())));
                                #[func] "is_nan" => |_,s,()| Ok(s.val(|s| s.is_nan()));
                                #[func] "is_finite" => |_,s,()| Ok(s.val(|s| s.is_finite()));

                                #[meta] MetaMethod::Unm => |_,s,()| Ok([<Lua $mat_base>]::new(s.val(|s| s.neg())));
                                #[meta] MetaMethod::Sub => |_,s,o : [<Lua $mat_base>]| Ok([<Lua $mat_base>]::new(s.bin(&o,|s,o|s.sub(*o))));
                                #[meta] MetaMethod::Add => |_,s,o : [<Lua $mat_base>]| Ok([<Lua $mat_base>]::new(s.bin(&o,|s,o|s.add(*o))));

                                #[meta] MetaMethod::Mul => |c,s,v: Value| {
                                    match &v {
                                        Value::UserData(u) => {
                                            if let Ok(v) = u.borrow::<[<Lua $mat_base>]>(){
                                                return c.create_userdata([<Lua $mat_base>]::new(s.bin(&v,|s,o|s.mul(*o)))).map(Value::UserData)
                                            } else if let Ok(v) = u.borrow::<[<Lua $mat_col>]>() {
                                                return c.create_userdata([<Lua $mat_col>]::new(s.binv(&v,|s,o|s.mul(o.inner())))).map(Value::UserData)
                                            }
                                        },
                                        _ => {}
                                    }

                                    c.coerce_number(v)?
                                        .and_then(|v| Some([<Lua $mat_base>]::new(s.binv(&v,|s,o| s.mul(*o as $mat_num)))))
                                        .and_then(|v| c.create_userdata(v).ok())
                                        .map(Value::UserData)
                                        .ok_or_else(|| Error::RuntimeError(format!("Can only multiply matrix by number or vector")))
                                };
                          
                            },
                        )*

                        // quats
                        $(
                            $quat_str ;=> $quat_base : {
                                $($quat_inner)*
                                #[func] "to_axis_angle" => |_,s,()| {
                                    let (v,f) = s.val(|v| v.to_axis_angle());
                                    Ok(([<Lua $quat_vec>]::new(v),f))
                                };

                                #[func] "to_scaled_axis" => |_,s,()| Ok([<Lua $quat_vec>]::new(s.val(|v| v.to_scaled_axis())));
                                #[func] "xyz" => |_,s,()| Ok([<Lua $quat_vec>]::new(s.val(|v| v.xyz())));
                                #[func] "conjugate" => |_,s,()| Ok([<Lua $quat_base>]::new(s.val(|v| v.conjugate())));
                                #[func] "inverse" => |_,s,()| Ok([<Lua $quat_base>]::new(s.val(|v| v.inverse())));
                                #[func] "length" => |_,s,()| Ok(s.val(|v| v.length()));
                                #[func] "length_squared" => |_,s,()| Ok(s.val(|v| v.length_squared()));
                                #[func] "length_recip" => |_,s,()| Ok(s.val(|v| v.length_recip()));
                                #[func] "normalize" => |_,s,()| Ok([<Lua $quat_base>]::new(s.val(|v| v.normalize())));
                                #[func] "is_finite" => |_,s,()| Ok(s.val(|v| v.is_finite()));
                                #[func] "is_nan" => |_,s,()| Ok(s.val(|v| v.is_nan()));
                                #[func] "is_normalized" => |_,s,()| Ok(s.val(|v| v.is_normalized()));
                                #[func] "is_near_identity" => |_,s,()| Ok(s.val(|v| v.is_near_identity()));

                                #[func] "to_euler" => |_,s,e : LuaEulerRot| Ok(s.val(|v| v.to_euler(e.0)));

                                #[func] "dot" => |_,s,o : [<Lua $quat_base>]| Ok(s.bin(&o,|s,o| s.dot(*o)));
                                #[func] "angle_between" => |_,s,o : [<Lua $quat_base>]| Ok(s.bin(&o,|s,o| s.angle_between(*o)));
                                #[func] "abs_diff_eq" => |_,s,(o,diff) : ([<Lua $quat_base>],$quat_num)| Ok(s.bin(&o,|s,o| s.abs_diff_eq(*o,diff)));
                                #[func] "lerp" => |_,s,(o,f) : ([<Lua $quat_base>],$quat_num)| Ok([<Lua $quat_base>]::new(s.bin(&o,|s,o| s.lerp(*o,f))));
                                #[func] "slerp" => |_,s,(o,f) : ([<Lua $quat_base>],$quat_num)| Ok([<Lua $quat_base>]::new(s.bin(&o,|s,o| s.slerp(*o,f))));
                                #[meta] MetaMethod::Mul => |c,s,o : Value| {
                                    if let Value::UserData(ref o) = o {
                                        if let Ok(o) = o.borrow::<[<Lua $quat_vec>]>(){
                                            return c.create_userdata([<Lua $quat_vec>]::new(s.binv(&o,|s,o| s.mul(o.inner())))).map(Value::UserData)
                                        } else if let Ok(o) = o.borrow::<[<Lua $quat_base>]>(){
                                            return c.create_userdata([<Lua $quat_base>]::new(s.bin(&o,|s,o| s.mul(*o)))).map(Value::UserData)
                                        }
                                    } 
                                    c.coerce_number(o)?
                                        .and_then(|o| c.create_userdata([<Lua $quat_base>]::new(s.binv(&o,|s,_| s.mul(o as $quat_num)))).ok())
                                        .map(Value::UserData)
                                        .ok_or_else(|| Error::RuntimeError("Can only multiply Quat by vec3, quat or a number".to_owned()))
                                };
                                #[meta] MetaMethod::Add => |_,s,o : [<Lua $quat_base>]| Ok([<Lua $quat_base>]::new(s.bin(&o,|s,v| s.add(*v))));
                                #[meta] MetaMethod::Sub => |_,s,o : [<Lua $quat_base>]| Ok([<Lua $quat_base>]::new(s.bin(&o,|s,v| s.sub(*v))));
                                #[meta] MetaMethod::Unm => |_,s,()| Ok([<Lua $quat_base>]::new(s.val(|s| s.neg())));
                            },
                        )*

                        // vanilla/others
                        $($o)+

                    ]
                    non_assignable_ud:[
                        $($na_inner)*
                    ]
                    primitives: [
                        // primitives
                        $($primitive_inner)*
                    ]
            }
        );
            
    }
}

// the string paths are neccessary since phf cannot handle macro inputs  
// maybe in a switch to TypeId this will workout automatically
// and there won't be a need for these auto-tests
make_it_all_baby!{
    vectors: [
        "glam::vec2::Vec2" ;=> Vec2,f32 :+: f32 : {
            ##[glob] "vec2" => |_,(x,y): (f32,f32)| {Ok(LuaVec2::new(Vec2::new(x,y)))};
            #[func] "perp_dot" => |_,s : &LuaVec2,o : LuaVec2| { Ok(s.bin(&o,|s,o| s.perp_dot(*o))) };
        },
        "glam::vec3::Vec3" ;=> Vec3,f32 :+: f32: {
            ##[glob] "vec3" => |_,(x,y,z): (f32,f32,f32)| {Ok(LuaVec3::new(Vec3::new(x,y,z)))};
        },
        "glam::vec4::Vec4" ;=> Vec4,f32 :+: f32: {
            ##[glob] "vec4" => |_,(x,y,z,w): (f32,f32,f32,f32)| {Ok(LuaVec4::new(Vec4::new(x,y,z,w)))};
        },

        // f64
        "glam::vec2::DVec2" ;=> DVec2 ,f64 :+: f64 :{
            ##[glob] "dvec2" => |_,(x,y): (f64,f64)| {Ok(LuaDVec2::new(DVec2::new(x,y)))};
            #[func] "perp_dot" => |_,s : &LuaDVec2,o : LuaDVec2| { Ok(s.bin(&o,|s,o| s.perp_dot(*o))) };
        },
        "glam::vec3::DVec3" ;=> DVec3,f64 :+: f64: {
            ##[glob] "dvec3" => |_,(x,y,z): (f64,f64,f64)| {Ok(LuaDVec3::new(DVec3::new(x,y,z)))};
        },
        "glam::vec4::DVec4" ;=> DVec4,f64 :+: f64: {
            ##[glob] "dvec4" => |_,(x,y,z,w): (f64,f64,f64,f64)| {Ok(LuaDVec4::new(DVec4::new(x,y,z,w)))};
        },

        // u32
        "glam::vec2::UVec2" ;=> UVec2,u32: {
            ##[glob] "uvec2" => |_,(x,y): (u32,u32)| {Ok(LuaUVec2::new(UVec2::new(x,y)))};
        },
        "glam::vec3::UVec3" ;=> UVec3, u32: {
            ##[glob] "uvec3" => |_,(x,y,z): (u32,u32,u32)| {Ok(LuaUVec3::new(UVec3::new(x,y,z)))};
        },
        "glam::vec4::UVec4" ;=> UVec4, u32: {
            ##[glob] "uvec4" => |_,(x,y,z,w): (u32,u32,u32,u32)| {Ok(LuaUVec4::new(UVec4::new(x,y,z,w)))};
        },

        // i32
        "glam::vec2::IVec2" ;=> IVec2,i32: {
            ##[glob] "ivec2" => |_,(x,y): (i32,i32)| {Ok(LuaIVec2::new(IVec2::new(x,y)))};
        },
        "glam::vec3::IVec3" ;=> IVec3,i32: {
            ##[glob] "uvec3" => |_,(x,y,z): (i32,i32,i32)| {Ok(LuaIVec3::new(IVec3::new(x,y,z)))};
        },
        "glam::vec4::IVec4" ;=> IVec4,i32: {
            ##[glob] "ivec4" => |_,(x,y,z,w): (i32,i32,i32,i32)| {Ok(LuaIVec4::new(IVec4::new(x,y,z,w)))};
        }

    ]

    matrices: [
        "glam::mat3::Mat3" ;=> Mat3,Vec3,f32: {
            ##[glob] "mat3" => |_,(x,y,z): (LuaVec3,LuaVec3,LuaVec3)| {Ok(LuaMat3::new(Mat3::from_cols(x.inner(),y.inner(),z.inner())))};
        },
        "glam::mat4::Mat4" ;=> Mat4,Vec4,f32: {
            ##[glob] "mat4" => |_,(x,y,z,w): (LuaVec4,LuaVec4,LuaVec4,LuaVec4)| {Ok(LuaMat4::new(Mat4::from_cols(x.inner(),y.inner(),z.inner(),w.inner())))};
        },
        "glam::mat3::DMat3" ;=> DMat3,DVec3,f64: {
            ##[glob] "dmat3" => |_,(x,y,z): (LuaDVec3,LuaDVec3,LuaDVec3)| {Ok(LuaDMat3::new(DMat3::from_cols(x.inner(),y.inner(),z.inner())))};
        },
        "glam::mat4::DMat4" ;=> DMat4,DVec4,f64: {
            ##[glob] "dmat4" => |_,(x,y,z,w): (LuaDVec4,LuaDVec4,LuaDVec4,LuaDVec4)| {Ok(LuaDMat4::new(DMat4::from_cols(x.inner(),y.inner(),z.inner(),w.inner())))};
        }
    ]

    quats: [
        "glam::quat::Quat" ;=> Quat,Vec3,f32 : {
            ##[glob] "quat" => |_,(x,y,z,w) : (f32,f32,f32,f32)| Ok(LuaQuat::new(Quat::from_xyzw(x,y,z,w)));
        },
        "glam::quat::DQuat" ;=> DQuat,DVec3,f64 : {
            ##[glob] "dquat" => |_,(x,y,z,w) : (f64,f64,f64,f64)| Ok(LuaDQuat::new(DQuat::from_xyzw(x,y,z,w)));
        }
    ]

    primitives: [
        "usize" ;=> usize : {
            #[from] |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<usize>().unwrap().to_i64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_usize().ok_or_else(||Error::RuntimeError("Value not compatibile with usize".to_owned()))?)));
        },
        "isize" ;=> isize : {
            #[from] |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<isize>().unwrap().to_i64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_isize().ok_or_else(||Error::RuntimeError("Value not compatibile with isize".to_owned()))?)));
        },
        "i128" ;=> i128 : {
            #[from] |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i128>().unwrap().to_i64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i128().ok_or_else(||Error::RuntimeError("Value not compatibile with i128".to_owned()))?)));
        },
        "i64" ;=> i64 : {
            #[from] |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i64>().unwrap().to_i64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i64().ok_or_else(||Error::RuntimeError("Value not compatibile with i64".to_owned()))?)));
        },
        "i32" ;=> i32 : {
            #[from] |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i32>().unwrap().to_i64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i32().ok_or_else(||Error::RuntimeError("Value not compatibile with i32".to_owned()))?)));
        },
        "i16" ;=> i16 : {
            #[from] |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i16>().unwrap().to_i64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i16().ok_or_else(||Error::RuntimeError("Value not compatibile with i16".to_owned()))?)));
        },
        "i8" ;=> i8 : {
            #[from] |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i8>().unwrap().to_i64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i8().ok_or_else(||Error::RuntimeError("Value not compatibile with i8".to_owned()))?)));
        },
        "u128" ;=> u128 : {
            #[from] |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u128>().unwrap().to_i64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u128().ok_or_else(||Error::RuntimeError("Value not compatibile with u128".to_owned()))?)));
        },
        "u64" ;=> u64 : {
            #[from] |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u64>().unwrap().to_i64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u64().ok_or_else(||Error::RuntimeError("Value not compatibile with u64".to_owned()))?)));
        },
        "u32" ;=> u32 : {
            #[from] |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u32>().unwrap().to_i64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u32().ok_or_else(||Error::RuntimeError("Value not compatibile with u32".to_owned()))?)));
        },
        "u16" ;=> u16 : {
            #[from] |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u16>().unwrap().to_i64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u16().ok_or_else(||Error::RuntimeError("Value not compatibile with u16".to_owned()))?)));
        },
        "u8" ;=> u8 : {
            #[from] |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u8>().unwrap().to_i64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u8().ok_or_else(||Error::RuntimeError("Value not compatibile with u8".to_owned()))?)));
        },
        "f32" ;=> f32 : {
            #[from] |r,_| r.get(|s,_| Value::Number(s.downcast_ref::<f32>().unwrap().to_f64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_number(v)?.ok_or_else(||Error::RuntimeError("Not a number".to_owned()))?.to_f32().ok_or_else(||Error::RuntimeError("Value not compatibile with f32".to_owned()))?)));
        },
        "f64" ;=> f64 : {
            #[from] |r,_| r.get(|s,_| Value::Number(s.downcast_ref::<f64>().unwrap().to_f64().unwrap()));
            #[to] |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_number(v)?.ok_or_else(||Error::RuntimeError("Not a number".to_owned()))?.to_f64().ok_or_else(||Error::RuntimeError("Value not compatibile with f64".to_owned()))?)));
        },
        "string" ;=> String : {
            #[from] |r,c| r.get(|s,_| Value::String(c.create_string(s.downcast_ref::<String>().unwrap()).unwrap()));
            #[to] |r,c,v : Value| c.coerce_string(v)?.ok_or_else(||Error::RuntimeError("Not a string".to_owned())).and_then(|string| r.get_mut(|s,_| Ok(s.apply(&string.to_str()?.to_owned()))));
        }
    ]
    other: [
        "bevy_ecs::entity::Entity" ;=> Entity : {
            #[func] "id" => |_,s : &LuaEntity, ()| Ok(s.val(|v| v.id()));
            #[func] "generation" => |_,s : &LuaEntity, ()| Ok(s.val(|v| v.generation()));
            #[func] "bits" => |_,s : &LuaEntity, ()| Ok(s.val(|v| v.to_bits()));
        }
    ]
    // things which cannot be reflected from/assigned to, since they do not support reflection/
    // hence can only be created via lua globals or passed explicitly to the script
    non_assignable: [
        "glam::EulerRot" ;=> EulerRot :(pub EulerRot) {
            ##[glob] "euler_rot" => |_,v : String| Ok(LuaEulerRot(match v.as_str() {
                "ZYX" => EulerRot::ZYX,
                "ZXY" => EulerRot::ZXY,
                "YXZ" => EulerRot::YXZ,
                "YZX" => EulerRot::YZX,
                "XYZ" => EulerRot::XYZ,
                "XZY" => EulerRot::ZXY,
                _ => return Err(Error::RuntimeError("Invalid euler rotation".to_owned()))
            }));
        },
        "bevy_ecs::world::World" ;=> World: (pub Weak<RwLock<World>>) {
            #[func] "add_component" =>  |_, world, (entity, comp_name): (LuaEntity, String)| {

                // grab this entity before acquiring a lock in case it's a reference
                let entity = entity.inner();

                let w = world.0.upgrade().unwrap();
                let w = &mut w.write();

                let refl: ReflectComponent = get_type_data(w, &comp_name)
                    .map_err(|_| Error::RuntimeError(format!("Not a component {}",comp_name)))?;
                let def = get_type_data::<ReflectDefault>(w, &comp_name)
                    .map_err(|_| Error::RuntimeError(format!("Component does not derive Default and cannot be instantiated: {}",comp_name)))?;

                let s = def.default();
                refl.add_component(w, entity, s.as_ref());


                Ok(LuaComponent {
                    comp: LuaRef{
                        root: LuaRefBase::Component{ 
                            comp: refl.clone(), 
                            entity: entity,
                            world: world.0.clone()
                        }, 
                        path: Some("".to_string()), 
                        r: ReflectPtr::Const(refl.reflect_component(w,entity).unwrap())
                    }    
                })
            };

            #[func_mut] "get_component" => |_, world, (entity, comp_name) : (LuaEntity,String)| {

                // grab this entity before acquiring a lock in case it's a reference
                let entity = entity.inner();

                let w = world.0.upgrade().unwrap();
                let w = &mut w.write();

                let refl: ReflectComponent = get_type_data(w, &comp_name)
                    .map_err(|_| Error::RuntimeError(format!("Not a component {}",comp_name)))?;

                let dyn_comp = refl
                    .reflect_component(&w, entity)
                    .ok_or_else(|| Error::RuntimeError(format!("Could not find {comp_name} on {:?}",entity),
                    ))?;

                Ok(
                    LuaComponent {
                        comp: LuaRef{
                            root: LuaRefBase::Component{ 
                                comp: refl, 
                                entity: entity,
                                world: world.0.clone()
                            }, 
                            path: Some("".to_string()), 
                            r: ReflectPtr::Const(dyn_comp)
                        }    
                    }  
                )
            };

            #[func] "new_script_entity" => |_, world, name: String| {
                let w = world.0.upgrade().unwrap();
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

            #[func] "spawn" => |_, world, ()| {
                let w = world.0.upgrade().unwrap();
                let w = &mut w.write();                
                
                Ok(LuaEntity::new(w.spawn().id()))
            };

        }
    ]
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