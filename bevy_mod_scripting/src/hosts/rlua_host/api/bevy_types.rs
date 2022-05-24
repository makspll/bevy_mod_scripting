use rlua::{UserData, MetaMethod,Value,Context};
use paste::paste;
use bevy::prelude::*;
use bevy::math::*;
use std::{fmt::{Display,Formatter}, ops::*};
use phf::{phf_map, Map};
use std::ops::DerefMut;
use num::ToPrimitive;
use crate::base::LuaRef;

macro_rules! make_lua_types {
    (   
        [
            $(
                $str:expr ;=> $name:ty:$(($($inner:tt)*))?{
                    $(#[$e:tt] $g:expr => $f:expr;)*
                }
            ),*
        ]
        [
            $(
            $primitive_str:expr ;=> $primitive_base:ty : {
                #[from] $primitive_from:expr;
                #[to] $primitive_to:expr;
            }
            ),*
        ]
    ) => {
        paste!{

            $(
                make_lua_struct!(
                    $name: $( ($($inner)*) )? {
                        $(#[$e] $g => $f;)*
                    }
                );
            )*

            pub static BEVY_TO_LUA: Map<&'static str,
                for<'l> fn(&dyn Reflect, ctx: Context<'l>) -> Value<'l>
            > = phf_map!{
                $(
                    $str => |r,c| {
                        let usr = c.create_userdata([<Lua $name>]::base_to_self(r.downcast_ref::<$name>().unwrap())).unwrap();
                        Value::UserData(usr)
                    }
                ),*,
                $(
                    $primitive_str => $primitive_from
                ),*
            };

            pub static APPLY_LUA_TO_BEVY: Map<&'static str,
                for<'l> fn(&mut dyn Reflect, ctx: Context<'l>, new_val: Value<'l>) -> Result<(),rlua::Error>
            > = phf_map!{
                $(
                    $str => |r,c,n| {

                        if let Value::UserData(v) = n {
                            let mut v = v.borrow_mut::<[<Lua $name>]>()?;
                            [<Lua $name>]::apply_self_to_base(v.deref_mut(),r.downcast_mut::<$name>().unwrap());
                            Ok(())
                        } else {
                            Err(rlua::Error::RuntimeError("Invalid type".to_owned()))
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
    // Reference type
    (
        $base:ty:(LuaRef) {
            $(#[$e:tt] $g:expr => $f:expr;)*
        }

    ) => {
        paste!{
            #[derive(Debug,Clone)]
            pub struct [<Lua $base>] (LuaRef);
            
            impl Display for [<Lua $base>] {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
                    write!(f,"{:#?}", self)
                }
            }

            impl [<Lua $base>] {
                pub fn base_to_self<'lua>(b: &$base) -> Self {
                    Self(LuaRef(b as *const dyn Reflect as *mut dyn Reflect))
                }
                pub fn apply_self_to_base<'lua>(&mut self, b: &mut $base){
                    b.apply(self.0.get());
                }
            }

            impl UserData for [<Lua $base>] {
                fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
                    // automatically generate 
                    methods.add_meta_method(MetaMethod::ToString, |_,s,()|{
                        Ok(format!("{}",s))
                    });

                    methods.add_meta_method(MetaMethod::Index, |ctx, val, field: String| {
                        let r = val.0.path_ref(&field).unwrap();
                        Ok(r.convert_to_lua(ctx).unwrap())
                    });

                    methods.add_meta_method_mut(
                        MetaMethod::NewIndex,
                        |ctx, val, (field, new_val): (Value, Value)| {
                            val.0
                                .path_lua_val_ref(field)
                                .unwrap()
                                .apply_lua(ctx, new_val)
                                .unwrap();
                            Ok(())
                        },
                    );

                    $(
                        make_add_method!(methods,#[$e] $g => $f);
                    )*
                }
            }

            
        }
    };
    // Value type
    (
    $base:ty :{
        $(#[$e:tt] $g:expr => $f:expr;)*
    }

    ) => {
        paste!{
            #[derive(Debug,Clone)]
            pub struct [<Lua $base>] { 
                val:$base, 
                vref:Option<LuaRef> 
            }
            
            

            impl Display for [<Lua $base>] {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
                    write!(f,"{:#?}", self)
                }
            }

            impl [<Lua $base>] {
                pub fn base_to_self<'lua>(b: &$base) -> Self {
                    [<Lua $base>] {
                        val:*b,
                        vref: Some(LuaRef(b as *const dyn Reflect as *mut dyn Reflect))
                    }
                }
                pub fn apply_self_to_base<'lua>(&mut self, b: &mut $base){
                    *b = self.val;
                }

                pub fn new(b : $base) -> Self {
                    [<Lua $base>] {
                        val:b,
                        vref: None
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
        primitives: [
            $(
                $primitive_inner:tt
            )*
        ]
        other: [$($o:tt)+]
    ) => {
        paste!(
                make_lua_types!{
                    [   

                        // vectors
                        $(

                            $vec_str ;=> $vec_base : {
                                $(
                                    // $vec_base $vec_float_inner
                                    #[meta] MetaMethod::Pow => |_,s : &[<Lua $vec_base>], o : $vec_float_inner| { Ok([<Lua $vec_base>]::new(s.val.powf(o))) };
                                    #[meta] MetaMethod::Unm => |_,s : &[<Lua $vec_base>],()| { Ok(([<Lua $vec_base>]::new(s.val.neg()))) };
                                    #[func] "abs" => |_,s : &[<Lua $vec_base>],()| { Ok([<Lua $vec_base>]::new(s.val.abs())) };
                                    #[func] "signum" => |_,s : &[<Lua $vec_base>],()| { Ok([<Lua $vec_base>]::new(s.val.signum())) };
                                )?
                                #[func] "dot" => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok(s.val.dot(o.val)) };
                                #[func] "min_element" => |_,s : &[<Lua $vec_base>],()| { Ok(s.val.min_element()) };
                                #[func] "max_element" => |_,s : &[<Lua $vec_base>],()| { Ok(s.val.max_element()) };
                                #[func] "min" => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.val.min(o.val))) };
                                #[func] "max" => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.val.max(o.val))) };
                                #[func] "clamp" => |_,s : &[<Lua $vec_base>],(o,max) : ([<Lua $vec_base>],[<Lua $vec_base>])| { Ok([<Lua $vec_base>]::new(s.val.clamp(o.val,max.val))) };



                                #[meta] MetaMethod::Add => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.val.add(o.val))) };
                                #[meta] MetaMethod::Sub => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.val.sub(o.val))) };
                                #[meta] MetaMethod::Mul => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.val.mul(o.val))) };
                                #[meta] MetaMethod::Div => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.val.div(o.val))) };
                                #[meta] MetaMethod::Mod => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok([<Lua $vec_base>]::new(s.val.rem(o.val))) };
                                #[meta] MetaMethod::Eq => |_,s : &[<Lua $vec_base>],o : [<Lua $vec_base>]| { Ok((s.val.eq(&o.val))) };
                                #[meta] MetaMethod::Index => |_,s : &[<Lua $vec_base>],idx : Value| { 
                                    match idx {
                                        Value::Integer(v) => Ok(s.val[v as usize]),
                                        Value::String(ref v) => match v.to_str()? {
                                            "x" => Ok(s.val[0]),
                                            "y" => Ok(s.val[1]),
                                            "z" => Ok(s.val[2]),
                                            "w" => Ok(s.val[3]),
                                            _ => Err(rlua::Error::RuntimeError(format!("Cannot index {} with {:#?}",stringify!($vec_base),idx)))
                                        },
                                        _ => Err(rlua::Error::RuntimeError(format!("Cannot index {} with {:#?}",stringify!($vec_base),idx)))
                                    }
                                };
                                #[meta_mut] MetaMethod::NewIndex => |_,s : &mut [<Lua $vec_base>],(idx,val) : (Value,$vec_num)| { 
                                    match idx {
                                        Value::Integer(v) => Ok(s.val[v as usize] = val),
                                        Value::String(ref v) => {
                                            let idx = match v.to_str()? {
                                                "x" => 0,
                                                "y" => 1,
                                                "z" => 2,
                                                "w" => 3,
                                                _ => Err(rlua::Error::RuntimeError(format!("Cannot index {} with {:#?}",stringify!($vec_base),idx)))?
                                            };
                                            
                                            // if our wrapper holds a reference it means it is an immediate indexing into
                                            // the original value, i.e. some_struct.our_vec[idx] = value
                                            Ok(match &mut s.vref {
                                                Some(r) => r.get_mut().downcast_mut::<$vec_base>().unwrap()[idx] = val,
                                                None => s.val[idx] = val,
                                            })

                                        },
                                        _ => Err(rlua::Error::RuntimeError(format!("Cannot index {} with {:#?}",stringify!($vec_base),idx)))
                                    }
                                };

                                $($vec_inner)*
                            },
                        )*
                        // vanilla/others
                        $($o)+

                    ]
                    [
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
            #[func] "perp_dot" => |_,s : &LuaVec2,o : LuaVec2| { Ok(s.val.perp_dot(o.val)) };
        },
        "glam::vec3::Vec3" ;=> Vec3,f32 :+: f32: {},
        "glam::vec4::Vec4" ;=> Vec4,f32 :+: f32: {},

        // f64
        "glam::vec2::DVec2" ;=> DVec2 ,f64 :+: f64 :{
            #[func] "perp_dot" => |_,s : &LuaDVec2,o : LuaDVec2| { Ok(s.val.perp_dot(o.val)) };
        },
        "glam::vec3::DVec3" ;=> DVec3,f64 :+: f64: {},
        "glam::vec4::DVec4" ;=> DVec4,f64 :+: f64: {},

        // u32
        "glam::vec2::UVec2" ;=> UVec2,u32: {},
        "glam::vec3::UVec3" ;=> UVec3, u32: {},
        "glam::vec4::UVec4" ;=> UVec4, u32: {},

        // i32
        "glam::vec2::IVec2" ;=> IVec2,i32: {},
        "glam::vec3::IVec3" ;=> IVec3,i32: {},
        "glam::vec4::IVec4" ;=> IVec4,i32: {}

    ]
    primitives: [
        "usize" ;=> usize : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<usize>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(rlua::Error::RuntimeError("u".to_owned()))?.to_usize().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "isize" ;=> isize : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<isize>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(rlua::Error::RuntimeError("a".to_owned()))?.to_isize().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "i128" ;=> i128 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<i128>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(rlua::Error::RuntimeError("b".to_owned()))?.to_i128().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "i64" ;=> i64 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<i64>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(rlua::Error::RuntimeError("c".to_owned()))?.to_i64().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "i32" ;=> i32 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<i32>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(rlua::Error::RuntimeError("d".to_owned()))?.to_i32().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "i16" ;=> i16 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<i16>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(rlua::Error::RuntimeError("e".to_owned()))?.to_i16().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "i8" ;=> i8 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<i8>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(rlua::Error::RuntimeError("f".to_owned()))?.to_i8().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "u128" ;=> u128 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<u128>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(rlua::Error::RuntimeError("g".to_owned()))?.to_u128().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "u64" ;=> u64 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<u64>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(rlua::Error::RuntimeError("h".to_owned()))?.to_u64().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "u32" ;=> u32 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<u32>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(rlua::Error::RuntimeError("j".to_owned()))?.to_u32().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "u16" ;=> u16 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<u16>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(rlua::Error::RuntimeError("i".to_owned()))?.to_u16().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "u8" ;=> u8 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<u8>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(rlua::Error::RuntimeError("k".to_owned()))?.to_u8().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "f32" ;=> f32 : {
            #[from] |r,_| Value::Number( r.downcast_ref::<f32>().unwrap().to_f64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_number(v)?.ok_or(rlua::Error::RuntimeError("l".to_owned()))?.to_f32().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "f64" ;=> f64 : {
            #[from] |r,_| Value::Number( r.downcast_ref::<f64>().unwrap().to_f64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_number(v)?.ok_or(rlua::Error::RuntimeError("m".to_owned()))?.to_f64().ok_or(rlua::Error::RuntimeError("".to_owned()))?));
        },
        "string" ;=> String : {
            #[from] |r,c| Value::String( c.create_string(r.downcast_ref::<String>().unwrap()).unwrap());
            #[to] |r,c,v : Value| c.coerce_string(v)?.ok_or(rlua::Error::RuntimeError("n".to_owned())).and_then(|s| Ok(r.apply(&s.to_str()?.to_owned())));
        }
    ]
    other: [
        "i120" ;=> i128 : {}
    ]
}
