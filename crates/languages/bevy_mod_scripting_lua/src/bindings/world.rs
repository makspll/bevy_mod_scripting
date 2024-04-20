use bevy_mod_scripting_core::bindings::WorldCallbackAccess;
use tealr::{
    mlu::{mlua::FromLua, FromToLua, TealData},
    ToTypename, Type,
};

use crate::{impl_userdata_from_lua, impl_userdata_with_tealdata};

use super::proxy::LuaProxied;

/// Lua UserData wrapper for [`bevy::ecs::world::World`]
#[derive(Clone)]
pub struct LuaWorld(pub WorldCallbackAccess);

impl_userdata_from_lua!(LuaWorld);
impl_userdata_with_tealdata!(LuaWorld);

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
        // methods.add_function("spawn_entity", |_, _, _| Ok(()));
    }
}

impl ToTypename for LuaWorld {
    fn to_typename() -> Type {
        Type::new_single("World", tealr::KindOfType::External)
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use bevy::ecs::world::World;
    use bevy_mod_scripting_core::{
        bindings::WorldAccessGuard,
        proxy::{Unproxy, ValProxy},
    };
    use tealr::mlu::mlua::Lua;

    use super::*;
    use crate::bindings::proxy::LuaNonReflectProxy;
    use tealr::mlu::mlua::IntoLua;

    #[test]
    fn test_world_from_to_lua() {
        let mut world = World::new();
        let world_access_guard = Arc::new(WorldAccessGuard::new(&mut world));
        let callback_access =
            unsafe { WorldCallbackAccess::new(Arc::downgrade(&world_access_guard)) };
        let proxy =
            LuaNonReflectProxy::<WorldCallbackAccess>(ValProxy::new(LuaWorld(callback_access)));

        let lua = Lua::new();
        let lua_val = proxy.into_lua(&lua).unwrap();
        let mut val = LuaNonReflectProxy::<WorldCallbackAccess>::from_lua(lua_val, &lua).unwrap();

        let val = val.unproxy().unwrap();
    }
}
