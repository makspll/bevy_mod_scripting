use std::sync::Arc;

use bevy_mod_scripting_core::bindings::WorldGuard;
use bevy_mod_scripting_core::bindings::{WorldAccessGuard, WorldCallbackAccess};
use bevy_mod_scripting_core::error::InteropError;
use mlua::UserData;

#[derive(Clone, Debug, mlua::FromLua)]
pub struct LuaWorld(pub WorldCallbackAccess);

impl LuaWorld {
    pub fn world_callback_access(self) -> WorldCallbackAccess {
        self.0.clone()
    }
}

impl UserData for LuaWorld {}

impl From<&LuaWorld> for WorldCallbackAccess {
    fn from(value: &LuaWorld) -> Self {
        value.0.clone()
    }
}

pub trait GetWorld {
    fn get_world(&self) -> WorldGuard<'static>;
    fn try_get_world(&self) -> Result<Arc<WorldAccessGuard<'static>>, mlua::Error>;
}

impl GetWorld for mlua::Lua {
    fn try_get_world(&self) -> Result<Arc<WorldAccessGuard<'static>>, mlua::Error> {
        let access = self
            .app_data_ref::<WorldCallbackAccess>()
            .ok_or_else(InteropError::missing_world)?;

        let world = access.try_read()?;

        Ok(world)
    }

    fn get_world(&self) -> WorldGuard<'static> {
        self.try_get_world()
            .expect("global 'world' did not exist or was invalid. Cannot retrieve world")
    }
}
