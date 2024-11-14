use std::sync::Arc;

use bevy::ecs::{component::ComponentId, reflect::AppTypeRegistry, world::Mut};
use bevy::prelude::Entity;

use bevy_mod_scripting_core::{
    bindings::{
        ReflectReference, ScriptTypeRegistration, Unproxy, WorldAccessGuard, WorldCallbackAccess,
    },
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

use super::proxy::LuaReflectRefProxy;
use super::{
    providers::bevy_ecs::LuaEntity,
    proxy::{
        ErrorProxy, LuaIdentityProxy, LuaProxied, LuaReflectValProxy, LuaValProxy, TypenameProxy,
    },
    type_registration::LuaTypeRegistration,
};
use crate::{impl_userdata_from_lua, impl_userdata_with_tealdata};

pub struct Nil;

impl ToTypename for Nil {
    fn to_typename() -> Type {
        Type::new_single("nil", tealr::KindOfType::Builtin)
    }
}

#[derive(Clone, Debug)]
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

impl TealData for LuaWorld {
    fn add_methods<'lua, T: tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("_list_accesses", |_, this, ()| {
            let world = this.0.read().ok_or_else(|| {
                mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
            })?;
            let accesses = world
                .list_accesses()
                .into_iter()
                .map(|v| format!("Access to: {v:?}"))
                .collect::<Vec<_>>();
            Ok(accesses)
        });

        methods.add_method("spawn", |_, this, ()| {
            let world = this.0.read().ok_or_else(|| {
                mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
            })?;
            let entity: LuaReflectValProxy<Entity> = world
                .proxy_call((), |()| world.spawn())
                .map_err(mlua::Error::external)?;
            Ok(entity)
        });

        methods.add_method("get_type_by_name", |_, this, type_name: String| {
            let world = this.0.read().ok_or_else(|| {
                mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
            })?;
            let out: Option<LuaValProxy<ScriptTypeRegistration>> = world
                .proxy_call(type_name, |type_name| world.get_type_by_name(type_name))
                .map_err(mlua::Error::external)?;

            Ok(out)
        });

        methods.add_method(
            "add_default_component",
            |_,
             this,
             args: (
                LuaReflectValProxy<Entity>,
                LuaValProxy<ScriptTypeRegistration>,
            )| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: Result<(), ErrorProxy<ScriptError>> = world
                    .proxy_call(args, |(entity, registration)| {
                        world.add_default_component(entity, registration)
                    })
                    .map_err(mlua::Error::external)?;

                Ok(TypenameProxy::<_, Nil>::new(out))
            },
        );

        methods.add_method(
            "get_component",
            |_,
             this,
             args: (
                LuaReflectValProxy<Entity>,
                LuaValProxy<ScriptTypeRegistration>,
            )| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: Result<Option<LuaValProxy<ReflectReference>>, ErrorProxy<ScriptError>> =
                    world
                        .proxy_call(args, |(entity, component_id)| {
                            match component_id.component_id() {
                                Some(component_id) => world.get_component(entity, component_id),
                                None => Ok(None),
                            }
                        })
                        .map_err(mlua::Error::external)?;

                Ok(TypenameProxy::<
                    _,
                    Option<LuaReflectRefProxy<ReflectReference>>,
                >::new(out))
            },
        );

        methods.add_method(
            "has_component",
            |_,
             this,
             args: (
                LuaReflectValProxy<Entity>,
                LuaValProxy<ScriptTypeRegistration>,
            )| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: Result<bool, ErrorProxy<ScriptError>> = world
                    .proxy_call(args, |(entity, registration)| {
                        match registration.component_id() {
                            Some(component_id) => world.has_component(entity, component_id),
                            None => Ok(false),
                        }
                    })
                    .map_err(mlua::Error::external)?;

                Ok(TypenameProxy::<_, bool>::new(out))
            },
        );

        methods.add_method(
            "remove_component",
            |_,
             this,
             args: (
                LuaReflectValProxy<Entity>,
                LuaValProxy<ScriptTypeRegistration>,
            )| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: Result<(), ErrorProxy<ScriptError>> = world
                    .proxy_call(args, |(entity, registration)| {
                        world.remove_component(entity, registration)
                    })
                    .map_err(mlua::Error::external)?;

                Ok(TypenameProxy::<_, Nil>::new(out))
            },
        );

        methods.add_method(
            "get_resource",
            |_, this, registration: LuaValProxy<ScriptTypeRegistration>| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: Result<Option<LuaValProxy<ReflectReference>>, ErrorProxy<ScriptError>> =
                    world
                        .proxy_call(registration, |registration| {
                            match registration.resource_id {
                                Some(resource_id) => world.get_resource(resource_id),
                                None => Ok(None),
                            }
                        })
                        .map_err(mlua::Error::external)?;

                Ok(TypenameProxy::<_, LuaReflectRefProxy<ReflectReference>>::new(out))
            },
        );

        methods.add_method(
            "remove_resource",
            |_, this, registration: LuaValProxy<ScriptTypeRegistration>| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: Result<(), ErrorProxy<ScriptError>> = world
                    .proxy_call(registration, |registration| {
                        world.remove_resource(registration)
                    })
                    .map_err(mlua::Error::external)?;

                Ok(TypenameProxy::<_, Nil>::new(out))
            },
        );

        methods.add_method(
            "has_resource",
            |_, this, registration: LuaValProxy<ScriptTypeRegistration>| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: bool = world
                    .proxy_call(registration, |registration| {
                        match registration.resource_id {
                            Some(resource_id) => world.has_resource(resource_id),
                            None => false,
                        }
                    })
                    .map_err(mlua::Error::external)?;

                Ok(out)
            },
        );

        methods.add_method(
            "get_children",
            |_, this, entity: LuaReflectValProxy<Entity>| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: Result<Vec<LuaReflectValProxy<Entity>>, ErrorProxy<ScriptError>> = world
                    .proxy_call(entity, |entity| world.get_children(entity))
                    .map_err(mlua::Error::external)?;

                Ok(TypenameProxy::<_, Vec<LuaReflectValProxy<Entity>>>::new(
                    out,
                ))
            },
        );

        methods.add_method(
            "get_parent",
            |_, this, entity: LuaReflectValProxy<Entity>| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: Result<Option<LuaReflectValProxy<Entity>>, ErrorProxy<ScriptError>> =
                    world
                        .proxy_call(entity, |entity| world.get_parent(entity))
                        .map_err(mlua::Error::external)?;

                Ok(TypenameProxy::<_, Option<LuaReflectValProxy<Entity>>>::new(
                    out,
                ))
            },
        );

        methods.add_method(
            "push_children",
            |_, this, args: (LuaReflectValProxy<Entity>, Vec<LuaReflectValProxy<Entity>>)| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: Result<(), ErrorProxy<ScriptError>> = world
                    .proxy_call(args, |(parent, children)| {
                        world.push_children(parent, &children)
                    })
                    .map_err(mlua::Error::external)?;

                Ok(TypenameProxy::<_, Nil>::new(out))
            },
        );

        methods.add_method(
            "remove_children",
            |_, this, args: (LuaReflectValProxy<Entity>, Vec<LuaReflectValProxy<Entity>>)| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: Result<(), ErrorProxy<ScriptError>> = world
                    .proxy_call(args, |(parent, children)| {
                        world.remove_children(parent, &children)
                    })
                    .map_err(mlua::Error::external)?;

                Ok(TypenameProxy::<_, Nil>::new(out))
            },
        );

        methods.add_method(
            "insert_children",
            |_,
             this,
             args: (
                LuaReflectValProxy<Entity>,
                usize,
                Vec<LuaReflectValProxy<Entity>>,
            )| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: Result<(), ErrorProxy<ScriptError>> = world
                    .proxy_call(args, |(parent, index, children)| {
                        world.insert_children(parent, index, &children)
                    })
                    .map_err(mlua::Error::external)?;

                Ok(TypenameProxy::<_, Nil>::new(out))
            },
        );

        methods.add_method(
            "despawn_recursive",
            |_, this, entity: LuaReflectValProxy<Entity>| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: Result<(), ErrorProxy<ScriptError>> = world
                    .proxy_call(entity, |entity| world.despawn_recursive(entity))
                    .map_err(mlua::Error::external)?;

                Ok(TypenameProxy::<_, Nil>::new(out))
            },
        );

        methods.add_method("despawn", |_, this, entity: LuaReflectValProxy<Entity>| {
            let world = this.0.read().ok_or_else(|| {
                mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
            })?;
            let out: Result<(), ErrorProxy<ScriptError>> = world
                .proxy_call(entity, |entity| world.despawn(entity))
                .map_err(mlua::Error::external)?;

            Ok(TypenameProxy::<_, Nil>::new(out))
        });

        methods.add_method(
            "despawn_descendants",
            |_, this, entity: LuaReflectValProxy<Entity>| {
                let world = this.0.read().ok_or_else(|| {
                    mlua::Error::external(ScriptError::new_reflection_error("Stale world access"))
                })?;
                let out: Result<(), ErrorProxy<ScriptError>> = world
                    .proxy_call(entity, |entity| world.despawn_descendants(entity))
                    .map_err(mlua::Error::external)?;

                Ok(TypenameProxy::<_, Nil>::new(out))
            },
        );
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(_fields: &mut F) {}
}

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
