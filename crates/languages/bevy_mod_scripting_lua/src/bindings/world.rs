use std::sync::Arc;

use bevy::ecs::{reflect::AppTypeRegistry, world::Mut};
use bevy_mod_scripting_core::{
    bindings::{Unproxy, WorldAccessGuard, WorldCallbackAccess},
    error::ScriptError,
};
use tealr::{
    mlu::{
        mlua::{self, FromLua},
        FromToLua, TealData,
    },
    ToTypename, Type,
};

use crate::{impl_userdata_from_lua, impl_userdata_with_tealdata};

use super::proxy::{LuaProxied, LuaValProxy};

/// Lua UserData wrapper for [`bevy::ecs::world::World`]
#[derive(Clone, tealr::mlu::UserData, tealr::ToTypename)]
pub struct LuaWorld(pub WorldCallbackAccess);

impl_userdata_from_lua!(LuaWorld);

impl LuaProxied for WorldCallbackAccess {
    type Proxy = LuaWorld;
}

impl From<&LuaWorld> for WorldCallbackAccess {
    fn from(value: &LuaWorld) -> Self {
        value.0.clone()
    }
}

impl TealData for LuaWorld {
    fn add_methods<'lua, T: tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
        // methods.add_method("get_type_by_name", |_, world, type_name: String| {
        //     let world = world
        //         .0
        //         .read()
        //         .ok_or_else(|| mlua::Error::external(ReflectionError::StaleWorldAccess))?;

        //     world.with_resource(|world, registry: Mut<AppTypeRegistry>| {
        //         let registry = registry.read();
        //         Ok(registry.get_with_short_type_path(&type_name).map(Arc::new))
        //     })
        // });
    }
}

pub trait GetWorld {
    fn get_world(&self) -> Result<Arc<WorldAccessGuard<'static>>, mlua::Error>;
}

impl GetWorld for mlua::Lua {
    fn get_world(&self) -> Result<Arc<WorldAccessGuard<'static>>, mlua::Error> {
        self.globals()
            .get::<_, LuaValProxy<WorldCallbackAccess>>("world")?
            .unproxy()
            .and_then(|guard| {
                guard
                    .read()
                    .ok_or_else(|| ScriptError::new_reflection_error("Stale world access"))
            })
            .map_err(mlua::Error::external)
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use bevy::ecs::world::World;
    use bevy_mod_scripting_core::{
        bindings::WorldAccessGuard,
        bindings::{Unproxy, ValProxy},
    };
    use tealr::mlu::mlua::Lua;

    use super::*;
    use crate::bindings::proxy::LuaValProxy;
    use tealr::mlu::mlua::IntoLua;

    #[test]
    fn test_world_from_to_lua() {
        let mut world = World::new();
        let world_access_guard = Arc::new(WorldAccessGuard::new(&mut world));
        let callback_access =
            unsafe { WorldCallbackAccess::new(Arc::downgrade(&world_access_guard)) };
        let proxy = LuaValProxy::<WorldCallbackAccess>(ValProxy::new(LuaWorld(callback_access)));

        let lua = Lua::new();
        let lua_val = proxy.into_lua(&lua).unwrap();
        let mut val = LuaValProxy::<WorldCallbackAccess>::from_lua(lua_val, &lua).unwrap();

        let _val = val.unproxy().unwrap();
    }
}
