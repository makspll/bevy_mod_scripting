use bevy::reflect::TypeData;
use bevy::reflect::TypeRegistry;
use rlua::{UserData, MetaMethod,Value,Context,Error,Lua};
use paste::paste;
use bevy::prelude::*;
use bevy::math::*;
use std::{fmt,fmt::{Debug,Display,Formatter}, ops::*,sync::Mutex};
use phf::{phf_map, Map};
use std::ops::DerefMut;
use num::ToPrimitive;
use crate::LuaFile;
use crate::PrintableReflect;
use crate::Script;
use crate::ScriptCollection;
use crate::LuaRef;
use crate::APIProvider;
use crate::ScriptError;

macro_rules! make_lua_types {
    (   
        [
            $(
                $str:expr ;=> $name:ty:$(($($inner:tt)*))?{
                    $(##[glob] $glob_name:expr => $global_fn:expr;)*
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

            // constructors/api 
            #[derive(Default)]
            pub struct LuaBevyAPI;

            impl APIProvider for LuaBevyAPI {
                type Ctx = Mutex<Lua>;
                fn attach_api(c: &mut <Self as APIProvider>::Ctx) {
                    c.lock()
                    .expect("Could not get lock on script context")
                    .context::<_, Result<(), ScriptError>>(|lua_ctx| {
                        $($(
                            lua_ctx.globals()
                                .set($glob_name, lua_ctx.create_function($global_fn)?)?;
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
                for<'l> fn(&mut dyn Reflect, ctx: Context<'l>, new_val: Value<'l>) -> Result<(),Error>
            > = phf_map!{
                $(
                    $str => |r,c,n| {

                        if let Value::UserData(v) = n {
                            let mut v = v.borrow_mut::<[<Lua $name>]>()?;
                            [<Lua $name>]::apply_self_to_base(v.deref_mut(),r.downcast_mut::<$name>().unwrap());
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
    // bare pointer type 
    (
        $base:ty:(*mut $_:ty) {
            $(#[$e:tt] $g:expr => $f:expr;)*
        }
    ) => {
        paste!{
            #[derive(Debug,Clone)]
            pub struct [<Lua $base>] (pub *mut $base);
            
            unsafe impl Send for [<Lua $base>]{}

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

    // reflect type
    (
        $base:ty:(LuaRef) {
            $(#[$e:tt] $g:expr => $f:expr;)*
        }

    ) => {
        paste!{
            #[derive(Debug,Clone)]
            pub struct [<Lua $base>] (pub LuaRef);
            
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

                    methods.add_meta_method(MetaMethod::Index, |ctx, val, field: Value| {
                        let r = val.0.path_lua_val_ref(&field).unwrap();
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
                pub val: $base, 
                pub vref: Option<LuaRef> 
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
        non_assignable: [
            $(
                $na_str:expr ;=> $na_base:ty : (*mut $_:ty) {  
                    $($na_inner:tt)* 
                } 
            ),*
        ]
    ) => {
        paste!(
                // non assignable
                $(make_lua_struct!{
                    $na_base : (*mut $_){  
                        $($na_inner)* 
                    }              
                })*

                make_lua_types!{
                    [   

                        // vectors
                        $(

                            $vec_str ;=> $vec_base : {

                                $($vec_inner)*

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
                                            _ => Err(Error::RuntimeError(format!("Cannot index {} with {:#?}",stringify!($vec_base),idx)))
                                        },
                                        _ => Err(Error::RuntimeError(format!("Cannot index {} with {:#?}",stringify!($vec_base),idx)))
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
                                                _ => Err(Error::RuntimeError(format!("Cannot index {} with {:#?}",stringify!($vec_base),idx)))?
                                            };
                                            
                                            // if our wrapper holds a reference it means it is an immediate indexing into
                                            // the original value, i.e. some_struct.our_vec[idx] = value
                                            Ok(match &mut s.vref {
                                                Some(r) => r.get_mut().downcast_mut::<$vec_base>().unwrap()[idx] = val,
                                                None => s.val[idx] = val,
                                            })

                                        },
                                        _ => Err(Error::RuntimeError(format!("Cannot index {} with {:#?}",stringify!($vec_base),idx)))
                                    }
                                };

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
            ##[glob] "vec2" => |_,(x,y): (f32,f32)| {Ok(LuaVec2::new(Vec2::new(x,y)))};
            #[func] "perp_dot" => |_,s : &LuaVec2,o : LuaVec2| { Ok(s.val.perp_dot(o.val)) };
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
            #[func] "perp_dot" => |_,s : &LuaDVec2,o : LuaDVec2| { Ok(s.val.perp_dot(o.val)) };
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
    primitives: [
        "usize" ;=> usize : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<usize>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(Error::RuntimeError("Not an integer".to_owned()))?.to_usize().ok_or(Error::RuntimeError("Value not compatibile with usize".to_owned()))?));
        },
        "isize" ;=> isize : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<isize>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(Error::RuntimeError("Not an integer".to_owned()))?.to_isize().ok_or(Error::RuntimeError("Value not compatibile with isize".to_owned()))?));
        },
        "i128" ;=> i128 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<i128>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(Error::RuntimeError("Not an integer".to_owned()))?.to_i128().ok_or(Error::RuntimeError("Value not compatibile with i128".to_owned()))?));
        },
        "i64" ;=> i64 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<i64>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(Error::RuntimeError("Not an integer".to_owned()))?.to_i64().ok_or(Error::RuntimeError("Value not compatibile with i64".to_owned()))?));
        },
        "i32" ;=> i32 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<i32>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(Error::RuntimeError("Not an integer".to_owned()))?.to_i32().ok_or(Error::RuntimeError("Value not compatibile with i32".to_owned()))?));
        },
        "i16" ;=> i16 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<i16>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(Error::RuntimeError("Not an integer".to_owned()))?.to_i16().ok_or(Error::RuntimeError("Value not compatibile with i16".to_owned()))?));
        },
        "i8" ;=> i8 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<i8>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(Error::RuntimeError("Not an integer".to_owned()))?.to_i8().ok_or(Error::RuntimeError("Value not compatibile with i8".to_owned()))?));
        },
        "u128" ;=> u128 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<u128>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(Error::RuntimeError("Not an integer".to_owned()))?.to_u128().ok_or(Error::RuntimeError("Value not compatibile with u128".to_owned()))?));
        },
        "u64" ;=> u64 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<u64>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(Error::RuntimeError("Not an integer".to_owned()))?.to_u64().ok_or(Error::RuntimeError("Value not compatibile with u64".to_owned()))?));
        },
        "u32" ;=> u32 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<u32>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(Error::RuntimeError("Not an integer".to_owned()))?.to_u32().ok_or(Error::RuntimeError("Value not compatibile with u32".to_owned()))?));
        },
        "u16" ;=> u16 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<u16>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(Error::RuntimeError("Not an integer".to_owned()))?.to_u16().ok_or(Error::RuntimeError("Value not compatibile with u16".to_owned()))?));
        },
        "u8" ;=> u8 : {
            #[from] |r,_| Value::Integer( r.downcast_ref::<u8>().unwrap().to_i64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_integer(v)?.ok_or(Error::RuntimeError("Not an integer".to_owned()))?.to_u8().ok_or(Error::RuntimeError("Value not compatibile with u8".to_owned()))?));
        },
        "f32" ;=> f32 : {
            #[from] |r,_| Value::Number( r.downcast_ref::<f32>().unwrap().to_f64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_number(v)?.ok_or(Error::RuntimeError("Not a number".to_owned()))?.to_f32().ok_or(Error::RuntimeError("Value not compatibile with f32".to_owned()))?));
        },
        "f64" ;=> f64 : {
            #[from] |r,_| Value::Number( r.downcast_ref::<f64>().unwrap().to_f64().unwrap());
            #[to] |r,c,v : Value| Ok(r.apply(&c.coerce_number(v)?.ok_or(Error::RuntimeError("Not a number".to_owned()))?.to_f64().ok_or(Error::RuntimeError("Value not compatibile with f64".to_owned()))?));
        },
        "string" ;=> String : {
            #[from] |r,c| Value::String( c.create_string(r.downcast_ref::<String>().unwrap()).unwrap());
            #[to] |r,c,v : Value| c.coerce_string(v)?.ok_or(Error::RuntimeError("Not a string".to_owned())).and_then(|s| Ok(r.apply(&s.to_str()?.to_owned())));
        }
    ]
    other: [
        "bevy_ecs::entity::Entity" ;=> Entity : {
            #[func] "id" => |_,s : &LuaEntity, ()| Ok(s.val.id());
            #[func] "generation" => |_,s : &LuaEntity, ()| Ok(s.val.generation());
            #[func] "bits" => |_,s : &LuaEntity, ()| Ok(s.val.to_bits());
        }
    ]
    non_assignable: [
        "bevy_ecs::world::World" ;=> World: (*mut World) {
            #[func] "add_component" =>  |_, w, (entity, comp_name): (LuaEntity, String)| {
                let w = unsafe { &mut *w.0 };

                let refl: ReflectComponent = get_type_data(w, &comp_name)
                    .map_err(|_| Error::RuntimeError(format!("Not a component {}",comp_name)))?;
                let def = get_type_data::<ReflectDefault>(w, &comp_name)
                    .map_err(|_| Error::RuntimeError(format!("Component does not derive Default and cannot be instantiated: {}",comp_name)))?;

                let s = def.default();
                refl.add_component(w, entity.val, s.as_ref());

                Ok(LuaComponent {
                    comp: LuaRef(
                        refl.reflect_component(w, entity.val).expect("Could not reflect freshly added component") as *const dyn Reflect
                            as *mut dyn Reflect,
                    )                
                })
            };

            #[func] "get_component" => |_, w, (entity, comp_name) : (LuaEntity,String)| {
                let w = unsafe { &mut *w.0 };

                let refl: ReflectComponent = get_type_data(w, &comp_name)
                    .map_err(|_| Error::RuntimeError(format!("Not a component {}",comp_name)))?;

                let dyn_comp = refl
                    .reflect_component(w, entity.val)
                    .ok_or(Error::RuntimeError(format!("Could not find {comp_name} on {:?}",entity.val),
                    ))?;

                Ok(LuaComponent {
                    comp: LuaRef(dyn_comp as *const dyn Reflect as *mut dyn Reflect),
                })
            };

            #[func] "new_script_entity" => |_, w, name: String| {
                let w = unsafe { &mut *w.0 };
    
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

            #[func] "spawn" => |_, w, ()| {
                let w = unsafe { &mut *w.0 };
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
        .ok_or(Error::RuntimeError(format!(
            "Invalid component name {name}"
        )))
        .unwrap();

    let refl: T = reg
        .data::<T>()
        .ok_or(Error::RuntimeError(format!(
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
            Ok(format!("{:#?}", PrintableReflect(val.comp.get())))
        });

        methods.add_meta_method(MetaMethod::Index, |ctx, val, field: String| {
            let r = val.comp
                .path_ref(&field)
                .map_err(|_| Error::RuntimeError(format!("The field {field} does not exist on {val:?}")))?;
                
            Ok(r.convert_to_lua(ctx).unwrap())
        });

        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |ctx, val, (field, new_val): (Value, Value)| {
                val.comp
                    .path_lua_val_ref(field)
                    .unwrap()
                    .apply_lua(ctx, new_val)
                    .unwrap();
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