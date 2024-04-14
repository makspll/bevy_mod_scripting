use bevy_mod_scripting_core::bindings::WorldCallbackAccess;

pub mod traits;

/// Lua UserData wrapper for [`bevy::ecs::world::World`]
pub struct LuaWorld(pub WorldCallbackAccess);
