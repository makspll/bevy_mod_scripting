use std::sync::Arc;

use bevy::ecs::{reflect::AppTypeRegistry, world::Mut};
use bevy::prelude::Entity;
use bevy_mod_scripting_core::{
    bindings::{ScriptTypeRegistration, Unproxy, WorldAccessGuard, WorldCallbackAccess},
    error::ScriptError,
};
use bevy_mod_scripting_derive::LuaProxy;
use tealr::mlu::mlua::IntoLua;
use tealr::{
    mlu::{
        mlua::{self, FromLua},
        FromToLua, TealData,
    },
    ToTypename, Type,
};

use super::{
    providers::bevy_ecs::LuaEntity,
    proxy::{LuaIdentityProxy, LuaProxied, LuaReflectValProxy, LuaValProxy},
    type_registration::LuaTypeRegistration,
};
use crate::{impl_userdata_from_lua, impl_userdata_with_tealdata};

/// Lua UserData wrapper for [`bevy::ecs::world::World`]
#[derive(LuaProxy, Clone)]
#[proxy(
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    get_world_callback_access_fn = "self::LuaWorld::world_callback_access",
    proxy_as_type = "self::LuaWorld",
    remote = "bevy_mod_scripting_core::bindings::WorldAccessGuard<'_>",
    functions [
        r#"
            #[lua()]
            fn add_default_component(&self, entity: LuaReflectValProxy<Entity>, registration: LuaValProxy<ScriptTypeRegistration>) -> Result<LuaIdentityProxy<()>, ScriptError>;
        "#,
        // r#"
        //     #[lua()]
        //     fn get_type_by_name(&self, type_name: String) -> Result<LuaTypeRegistration, ScriptError>;
        // "#,
    ]
)]
pub struct LuaWorld(pub WorldCallbackAccess);

impl LuaWorld {
    pub fn world_callback_access(self) -> WorldCallbackAccess {
        self.0.clone()
    }
}

impl ToTypename for LuaWorld {
    fn to_typename() -> Type {
        Type::new_single("LuaWorld", tealr::KindOfType::External)
    }
}

impl LuaProxied for WorldCallbackAccess {
    type Proxy = LuaWorld;
}

impl_userdata_with_tealdata!(LuaWorld);

// impl LuaProxied for WorldCallbackAccess {
//     type Proxy = LuaWorld;
// }

impl From<&LuaWorld> for WorldCallbackAccess {
    fn from(value: &LuaWorld) -> Self {
        value.0.clone()
    }
}

pub trait GetWorld {
    fn get_world(&self) -> Result<Arc<WorldAccessGuard<'static>>, mlua::Error>;
}

impl GetWorld for mlua::Lua {
    fn get_world(&self) -> Result<Arc<WorldAccessGuard<'static>>, mlua::Error> {
        self.globals()
            .get::<_, LuaValProxy<bevy_mod_scripting_core::bindings::WorldCallbackAccess>>("world")?
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
        let callback_access = unsafe {
            bevy_mod_scripting_core::bindings::WorldCallbackAccess::new(Arc::downgrade(
                &world_access_guard,
            ))
        };
        let proxy = LuaValProxy::<bevy_mod_scripting_core::bindings::WorldCallbackAccess>(
            ValProxy::new(LuaWorld(callback_access)),
        );

        let lua = Lua::new();
        let lua_val = proxy.into_lua(&lua).unwrap();
        let mut val =
            LuaValProxy::<bevy_mod_scripting_core::bindings::WorldCallbackAccess>::from_lua(
                lua_val, &lua,
            )
            .unwrap();

        let _val = val.unproxy().unwrap();
    }
}
