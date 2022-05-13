use std::sync::{RwLock,Weak};

use bevy::prelude::*;
use rlua::UserData;


/// A lua representation of an entity reference
pub struct LuaEntityUserData(Entity);

impl UserData for LuaEntityUserData {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("id", |_,e,()| {
            Ok(e.0.id())
        });

        methods.add_method("generation", |_,e,()| {
            Ok(e.0.generation())
        });

        methods.add_method("to_bits", |_,e,()| {
            Ok(e.0.to_bits())
        });
    }
}

/// A lua representation of a world reference
/// 
pub struct LuaWorldUserData(Weak<RwLock<World>>);

// impl UserData for LuaWorldUserData {
//     fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {

//         methods.add_meta_function(meta, function)
//     }
// }
