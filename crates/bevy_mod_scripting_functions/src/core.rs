//! Contains functions defined by the [`bevy_mod_scripting_core`] crate

use bevy::{
    prelude::*,
    reflect::{func::FunctionRegistrationError, ParsedPath},
};
use bevy_mod_scripting_core::*;
use bindings::{
    function::{
        from::{Ref, Val},
        from_ref::FromScriptRef,
        into_ref::IntoScriptRef,
        namespace::NamespaceBuilder,
        script_function::{CallerContext, ScriptFunctionMut},
    },
    pretty_print::DisplayWithWorld,
    script_value::ScriptValue,
    ReflectReference, ReflectionPathExt, ScriptComponentRegistration, ScriptQueryBuilder,
    ScriptQueryResult, ScriptResourceRegistration, ScriptTypeRegistration, WorldCallbackAccess,
};
use error::InteropError;
use reflection_extensions::{PartialReflectExt, TypeIdExtensions};

pub fn register_bevy_bindings(app: &mut App) {
    #[cfg(feature = "bevy_bindings")]
    app.add_plugins(crate::bevy_bindings::LuaBevyScriptingPlugin);
}

pub fn register_world_functions(reg: &mut World) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<World>::new_unregistered(reg)
        .register(
            "get_type_by_name",
            |world: WorldCallbackAccess, type_name: String| {
                let world = world.try_read()?;
                let val = world.get_type_by_name(type_name);

                Ok(match val {
                    Some(registration) => {
                        let allocator = world.allocator();

                        let registration = match world.get_resource_type(registration) {
                            Ok(res) => {
                                let mut allocator = allocator.write();
                                return Ok(Some(ReflectReference::new_allocated(
                                    res,
                                    &mut allocator,
                                )));
                            }
                            Err(registration) => registration,
                        };

                        let registration = match world.get_component_type(registration) {
                            Ok(comp) => {
                                let mut allocator = allocator.write();
                                return Ok(Some(ReflectReference::new_allocated(
                                    comp,
                                    &mut allocator,
                                )));
                            }
                            Err(registration) => registration,
                        };

                        let mut allocator = allocator.write();
                        Some(ReflectReference::new_allocated(
                            registration,
                            &mut allocator,
                        ))
                    }
                    None => None,
                })
            },
        )
        .register(
            "get_component",
            |world: WorldCallbackAccess,
             entity: Val<Entity>,
             registration: Val<ScriptComponentRegistration>| {
                world.get_component(*entity, registration.component_id())
            },
        )
        .register(
            "has_component",
            |world: WorldCallbackAccess,
             entity: Val<Entity>,
             registration: Val<ScriptComponentRegistration>| {
                world.has_component(*entity, registration.component_id())
            },
        )
        .register(
            "remove_component",
            |world: WorldCallbackAccess, e: Val<Entity>, r: Val<ScriptComponentRegistration>| {
                world.remove_component(*e, r.clone())
            },
        )
        .register(
            "get_resource",
            |world: WorldCallbackAccess, registration: Val<ScriptResourceRegistration>| {
                world.get_resource(registration.resource_id())
            },
        )
        .register(
            "has_resource",
            |world: WorldCallbackAccess, registration: Val<ScriptResourceRegistration>| {
                world.has_resource(registration.resource_id())
            },
        )
        .register(
            "remove_resource",
            |s: WorldCallbackAccess, r: Val<ScriptResourceRegistration>| {
                s.remove_resource(r.into_inner())
            },
        )
        .register(
            "add_default_component",
            |w: WorldCallbackAccess, e: Val<Entity>, r: Val<ScriptComponentRegistration>| {
                w.add_default_component(*e, r.clone())
            },
        )
        .register(
            "insert_component",
            |w: WorldCallbackAccess,
             e: Val<Entity>,
             r: Val<ScriptComponentRegistration>,
             v: ReflectReference| { w.insert_component(*e, r.into_inner(), v) },
        )
        .register("spawn", |s: WorldCallbackAccess| Ok(Val(s.spawn()?)))
        .register(
            "insert_children",
            |caller_context: CallerContext,
             w: WorldCallbackAccess,
             e: Val<Entity>,
             index: usize,
             c: Vec<Val<Entity>>| {
                let index = if caller_context.convert_to_0_indexed {
                    index - 1
                } else {
                    index
                };
                w.insert_children(*e, index, &c.into_iter().map(|v| *v).collect::<Vec<_>>())
            },
        )
        .register(
            "push_children",
            |w: WorldCallbackAccess, e: Val<Entity>, c: Vec<Val<Entity>>| {
                w.push_children(*e, &c.into_iter().map(|v| *v).collect::<Vec<_>>())
            },
        )
        .register("get_children", |w: WorldCallbackAccess, e: Val<Entity>| {
            let children = w.get_children(*e)?;
            Ok(children.into_iter().map(Val).collect::<Vec<_>>())
        })
        .register("get_parent", |w: WorldCallbackAccess, e: Val<Entity>| {
            let parent = w.get_parent(*e)?;
            Ok(parent.map(Val))
        })
        .register("despawn", |s: WorldCallbackAccess, e: Val<Entity>| {
            s.despawn(*e)
        })
        .register(
            "despawn_descendants",
            |s: WorldCallbackAccess, e: Val<Entity>| s.despawn_descendants(*e),
        )
        .register(
            "despawn_recursive",
            |s: WorldCallbackAccess, e: Val<Entity>| s.despawn_recursive(*e),
        )
        .register("has_entity", |s: WorldCallbackAccess, e: Val<Entity>| {
            s.has_entity(*e)
        })
        .register("query", || {
            let query_builder = ScriptQueryBuilder::default();
            Ok(Val(query_builder))
        })
        .register("exit", |s: WorldCallbackAccess| s.exit());
    Ok(())
}

pub fn register_reflect_reference_functions(
    reg: &mut World,
) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<ReflectReference>::new(reg)
        .register(
            "display_ref",
            |w: WorldCallbackAccess, s: ReflectReference| {
                let world = w.try_read()?;
                Ok(s.display_with_world(world))
            },
        )
        .register("display_value", |w: WorldCallbackAccess, s: ReflectReference| {
            let world = w.try_read()?;
            Ok(s.display_value_with_world(world))
        })
        .register(
            "get",
            |caller_context: CallerContext,
             world: WorldCallbackAccess,
             mut self_: ReflectReference,
             key: ScriptValue| {
                let mut path: ParsedPath = key.try_into()?;
                if caller_context.convert_to_0_indexed {
                    path.convert_to_0_indexed();
                }
                self_.index_path(path);
                let world = world.try_read()?;
                ReflectReference::into_script_ref(self_, world)
            },
        )
        .register(
            "set",
            |caller_context: CallerContext,
             world: WorldCallbackAccess,
             self_: ScriptValue,
             key: ScriptValue,
             value: ScriptValue| {
                if let ScriptValue::Reference(mut self_) = self_ {
                    let world = world.try_read()?;
                    let mut path: ParsedPath = key.try_into()?;
                    if caller_context.convert_to_0_indexed {
                        path.convert_to_0_indexed();
                    }
                    self_.index_path(path);
                    let r: ScriptValue = self_
                        .with_reflect_mut(world.clone(), |r| {
                            let target_type_id = r
                                .get_represented_type_info()
                                .map(|i| i.type_id())
                                .or_fake_id();
                            let other = <Box<dyn PartialReflect>>::from_script_ref(
                                target_type_id,
                                value,
                                world.clone(),
                            )?;
                            r.try_apply(other.as_partial_reflect()).map_err(|e| InteropError::external_error(Box::new(e)))?;
                            Ok::<_, InteropError>(())
                        })
                        .into();
                    return Ok(r);
                }
                Ok(ScriptValue::Unit)
            },
        )
        .register(
            "push",
            |w: WorldCallbackAccess, s: ReflectReference, v: ScriptValue| {
                let world = w.try_read()?;
                let target_type_id = s.element_type_id(world.clone())?.ok_or_else(|| {
                    InteropError::unsupported_operation(
                        s.tail_type_id(world.clone()).unwrap_or_default(),
                        Some(Box::new(v.clone())),
                        "Could not get element type id. Are you trying to insert elements into a type that's not a list?".to_owned(),
                    )
                })?;
                let other = <Box<dyn PartialReflect>>::from_script_ref(target_type_id, v, world.clone())?;
                s.with_reflect_mut(world, |s| s.try_push_boxed(other))?
            },
        )
        .register("pop", |w: WorldCallbackAccess, s: ReflectReference| {
            let world = w.try_read()?;
            let o = s.with_reflect_mut(world.clone(), |s| s.try_pop_boxed())??;
            let reference = {
                let allocator = world.allocator();
                let mut allocator = allocator.write();
                ReflectReference::new_allocated_boxed_parial_reflect(o, &mut allocator)?
            };

            ReflectReference::into_script_ref(reference, world)
        })
        .register("insert", |caller_context: CallerContext, w: WorldCallbackAccess, s: ReflectReference, k: ScriptValue, v: ScriptValue| {
            let world = w.try_read()?;
            let key_type_id = s.key_type_id(world.clone())?.ok_or_else(|| {
                InteropError::unsupported_operation(
                    s.tail_type_id(world.clone()).unwrap_or_default(),
                    Some(Box::new(k.clone())),
                    "Could not get key type id. Are you trying to insert elements into a type that's not a map?".to_owned(),
                )
            })?;

            let mut key = <Box<dyn PartialReflect>>::from_script_ref(key_type_id, k, world.clone())?;

            if caller_context.convert_to_0_indexed {
                key.convert_to_0_indexed_key();
            }

            let value_type_id = s.element_type_id(world.clone())?.ok_or_else(|| {
                InteropError::unsupported_operation(
                    s.tail_type_id(world.clone()).unwrap_or_default(),
                    Some(Box::new(v.clone())),
                    "Could not get element type id. Are you trying to insert elements into a type that's not a map?".to_owned(),
                )
            })?;

            let value = <Box<dyn PartialReflect>>::from_script_ref(value_type_id, v, world.clone())?;

            s.with_reflect_mut(world, |s| s.try_insert_boxed(key, value))?
        })
        .register("clear", |w: WorldCallbackAccess, s: ReflectReference| {
            let world = w.try_read()?;
            s.with_reflect_mut(world, |s| s.try_clear())?
        })
        .register("len", |w: WorldCallbackAccess, s: ReflectReference| {
            let world = w.try_read()?;
            s.len(world)
        })
        .register("remove", |caller_context: CallerContext, w: WorldCallbackAccess, s: ReflectReference, k: ScriptValue| {
            let world = w.try_read()?;
            let key_type_id = s.key_type_id(world.clone())?.ok_or_else(|| {
                InteropError::unsupported_operation(
                    s.tail_type_id(world.clone()).unwrap_or_default(),
                    Some(Box::new(k.clone())),
                    "Could not get key type id. Are you trying to remove elements from a type that's not a map?".to_owned(),
                )
            })?;

            let mut key = <Box<dyn PartialReflect>>::from_script_ref(key_type_id, k, world.clone())?;

            if caller_context.convert_to_0_indexed {
                key.convert_to_0_indexed_key();
            }

            let removed = s.with_reflect_mut(world.clone(), |s| s.try_remove_boxed(key))??;
            match removed {
                Some(removed) => {
                    let reference = {
                        let allocator = world.allocator();
                        let mut allocator = allocator.write();
                        ReflectReference::new_allocated_boxed_parial_reflect(removed, &mut allocator)?
                    };
                    ReflectReference::into_script_ref(reference, world)
                }
                None => Ok(ScriptValue::Unit),
            }
        })
        .register("iter", |w: WorldCallbackAccess, s: ReflectReference| {
            let world = w.try_read()?;
            let mut len = s.len(world.clone())?.unwrap_or_default();
            let mut infinite_iter = s.into_iter_infinite();
            let iter_function = move || {
                if len == 0 {
                    return Ok(ScriptValue::Unit);
                }

                let (next_ref, _) = infinite_iter.next_ref();

                let converted = ReflectReference::into_script_ref(next_ref, world.clone());
                // println!("idx: {idx:?}, converted: {converted:?}");
                len -= 1;
                // we stop once the reflection path is invalid
                converted
            };

            Ok(iter_function.into_dynamic_script_function_mut())
        });

    Ok(())
}

pub fn register_script_type_registration_functions(
    registry: &mut World,
) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<ScriptTypeRegistration>::new(registry)
        .register("type_name", |s: Ref<ScriptTypeRegistration>| s.type_name())
        .register("short_name", |s: Ref<ScriptTypeRegistration>| {
            s.short_name()
        });

    NamespaceBuilder::<ScriptComponentRegistration>::new(registry)
        .register("type_name", |s: Ref<ScriptComponentRegistration>| {
            s.type_registration().type_name()
        })
        .register("short_name", |s: Ref<ScriptComponentRegistration>| {
            s.type_registration().short_name()
        });

    NamespaceBuilder::<ScriptResourceRegistration>::new(registry)
        .register("type_name", |s: Ref<ScriptResourceRegistration>| {
            s.type_registration().type_name()
        })
        .register("short_name", |s: Ref<ScriptResourceRegistration>| {
            s.type_registration().short_name()
        });

    Ok(())
}

pub fn register_script_query_builder_functions(
    registry: &mut World,
) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<ScriptQueryBuilder>::new(registry)
        .register(
            "component",
            |s: Val<ScriptQueryBuilder>, components: Val<ScriptComponentRegistration>| {
                let mut builder = s.into_inner();
                builder.component(components.into_inner());
                Val(builder)
            },
        )
        .register(
            "with",
            |s: Val<ScriptQueryBuilder>, with: Val<ScriptComponentRegistration>| {
                let mut builder = s.into_inner();
                builder.with_component(with.into_inner());
                Val(builder)
            },
        )
        .register(
            "without",
            |s: Val<ScriptQueryBuilder>, without: Val<ScriptComponentRegistration>| {
                let mut builder = s.into_inner();
                builder.without_component(without.into_inner());
                Val(builder)
            },
        )
        .register(
            "build",
            |world: WorldCallbackAccess, s: Val<ScriptQueryBuilder>| {
                let builder = s.into_inner();
                let result = world.query(builder)?;
                let result = result.into_iter().map(Val).collect::<Vec<_>>();
                Ok(result)
            },
        );
    Ok(())
}

pub fn register_script_query_result_functions(
    world: &mut World,
) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<ScriptQueryResult>::new(world)
        .register("entity", |s: Ref<ScriptQueryResult>| Val::new(s.entity))
        .register("components", |s: Ref<ScriptQueryResult>| {
            s.components.to_vec()
        });
    Ok(())
}

pub fn register_core_functions(app: &mut App) {
    let world = app.world_mut();
    // we don't exclude from compilation here,
    // since these are much smaller and still useful if not included initially
    // perhaps people might want to include some but not all of these

    #[cfg(feature = "core_functions")]
    if let Err(e) = register_world_functions(world) {
        bevy::log::error!("Failed to register script world functions: {:?}", e);
    }

    #[cfg(feature = "core_functions")]
    if let Err(e) = register_reflect_reference_functions(world) {
        bevy::log::error!("Failed to register reflect reference functions: {:?}", e);
    }

    #[cfg(feature = "core_functions")]
    if let Err(e) = register_script_type_registration_functions(world) {
        bevy::log::error!(
            "Failed to register script type registration functions: {:?}",
            e
        );
    }

    #[cfg(feature = "core_functions")]
    if let Err(e) = register_script_query_builder_functions(world) {
        bevy::log::error!("Failed to register script query builder functions: {:?}", e);
    }

    #[cfg(feature = "core_functions")]
    if let Err(e) = register_script_query_result_functions(world) {
        bevy::log::error!("Failed to register script query result functions: {:?}", e);
    }
}
