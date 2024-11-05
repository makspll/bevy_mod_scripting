use std::sync::Arc;

use bevy::ecs::{reflect::AppTypeRegistry, world::Mut};
use bevy_mod_scripting_core::{
    bindings::{Unproxy, WorldAccessGuard, WorldCallbackAccess},
    error::ScriptError,
};
use bevy_mod_scripting_derive::LuaProxy;
use tealr::{
    mlu::{
        mlua::{self, FromLua},
        FromToLua, TealData,
    },
    ToTypename, Type,
};

use crate::{impl_userdata_from_lua, impl_userdata_with_tealdata};

use super::{
    providers::bevy_ecs::LuaEntity,
    proxy::{LuaIdentityProxy, LuaProxied, LuaValProxy},
    type_registration::LuaTypeRegistration,
};

/// Lua UserData wrapper for [`bevy::ecs::world::World`]
#[derive(LuaProxy)]
#[proxy(
    bms_core_path = "bevy_mod_scripting_core",
    bms_lua_path = "crate",
    get_world_callback_access_fn = "self::LuaWorld::world_callback_access",
    proxy_as_type = "self::LuaWorld",
    remote = "bevy_mod_scripting_core::bindings::WorldAccessGuard<'_>",
    functions [
        r#"
            #[lua()]
            fn add_default_component(&self, entity: LuaEntity, registration: LuaTypeRegistration) -> Result<IdentityProxy<()>, ScriptError>;
        "#,
        // r#"
        //     #[lua()]
        //     fn get_type_by_name(&self, type_name: String) -> Result<LuaTypeRegistration, ScriptError>;
        // "#,
    ]
)]
pub struct LuaWorld(WorldCallbackAccess);

impl LuaWorld {
    pub fn world_callback_access(self) -> WorldCallbackAccess {
        self.0.clone()
    }
}

impl ToTypename for LuaWorld {
    fn to_typename() -> Type {
        Type::Userdata
    }
}

// impl_userdata_from_lua!(LuaWorld);

// impl LuaProxied for WorldCallbackAccess {
//     type Proxy = LuaWorld;
// }

// impl From<&LuaWorld> for WorldCallbackAccess {
//     fn from(value: &LuaWorld) -> Self {
//         value.0.clone()
//     }
// }

// impl TealData for LuaWorld {
//     fn add_methods<'lua, T: tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
//         methods.add_method("get_type_by_name", |_, world, type_name: String| {
//             Ok(world
//                 .0
//                 .get_type_by_name(type_name.as_str())
//                 .map(Into::<LuaTypeRegistration>::into))
//         });

//         methods.add_method(
//             "add_default_component",
//             |_, world, (entity, registration): (LuaEntity, LuaTypeRegistration)| {
//                 let entity = entity.0.with_reflect(world, type_registry, allocator, f)
//                 Ok(world
//                     .0
//                     .add_default_component(entity.0.with, registration.0.clone()))
//             },
//         )
//     }
// }

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
