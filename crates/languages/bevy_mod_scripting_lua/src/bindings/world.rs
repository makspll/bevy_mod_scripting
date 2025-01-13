// use std::sync::Arc;

// use bevy_mod_scripting_core::bindings::WorldCallbackAccess;
// use mlua::UserData;

// #[derive(Clone, Debug, mlua::FromLua)]
// pub struct LuaWorld(pub WorldCallbackAccess);

// impl LuaWorld {
//     pub fn world_callback_access(self) -> WorldCallbackAccess {
//         self.0.clone()
//     }
// }

// impl UserData for LuaWorld {}

// impl From<&LuaWorld> for WorldCallbackAccess {
//     fn from(value: &LuaWorld) -> Self {
//         value.0.clone()
//     }
// }
