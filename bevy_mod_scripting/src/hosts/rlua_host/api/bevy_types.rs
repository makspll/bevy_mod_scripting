use rlua::{UserData, MetaMethod,Value,Context};
use paste::paste;
use bevy::prelude::{Vec2,Reflect};
use std::{fmt::{Display,Formatter}, ops::Add};
use phf::{phf_map, Map};
use std::ops::DerefMut;

use crate::base::LuaRef;

macro_rules! make_lua_types {
    (   
        [
            $(
                $str:expr => $name:ty:($($inner:tt)*){
                    $(##[$q:tt] => $r:expr;)*
                    $(#[$e:tt] $g:expr => $f:expr;)*
                }
            ),*
        ]
    ) => {
        paste!{

            $(
                make_lua_struct!(
                    $name: ($($inner)*) {
                        $(##[$q] => $r;)*
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
                )*
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
                )*
            }; 

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
    $base:ty :($($inner:ty),*) {
        ##[from_base] => $from:expr;
        ##[apply_to_base] => $apply:expr;
        $(#[$e:tt] $g:expr => $f:expr;)*
    }

    ) => {
        paste!{
            #[derive(Debug,Clone)]
            pub struct [<Lua $base>] ($($inner),*);
            
            impl Display for [<Lua $base>] {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
                    write!(f,"{:#?}", self)
                }
            }

            impl [<Lua $base>] {
                pub fn base_to_self<'lua>(b: &$base) -> Self {
                    $from(b)
                }
                pub fn apply_self_to_base<'lua>(&mut self, b: &mut $base){
                    $apply(self,b);
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


make_lua_types!(
    [
        "glam::vec2::Vec2" => Vec2:(Vec2) {
            ##[from_base] => |b : &Vec2| {LuaVec2(*b)};
            ##[apply_to_base] => |s : &mut LuaVec2,b : &mut Vec2| {*b=s.0};
            #[meta_mut] MetaMethod::Add => |_,s : &mut LuaVec2,o : LuaVec2| { Ok(LuaVec2(s.0.add(o.0))) };
        }
    ]
);