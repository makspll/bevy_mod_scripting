//! Contains functions defined by the [`bevy_mod_scripting_core`] crate
use std::borrow::Cow;

use bevy::{
    prelude::*,
    reflect::{
        func::{FunctionRegistrationError, FunctionRegistry, FunctionRegistryArc}, GetTypeRegistration, ParsedPath
    },
};
use bevy_mod_scripting_core::*;
use bindings::{
    function::{
        from::{Mut, Ref, Val},
        from_ref::FromScriptRef,
        into_ref::IntoScriptRef,
        script_function::{CallerContext, GetFunctionTypeDependencies, ScriptFunction, ScriptFunctionMut},
    },
    pretty_print::DisplayWithWorld,
    script_value::ScriptValue,
    ReflectReference, ReflectionPathExt, ScriptQueryBuilder, ScriptQueryResult,
    ScriptTypeRegistration, WorldAccessGuard, WorldCallbackAccess,
};
use error::{InteropError, InteropErrorInner};
use reflection_extensions::{PartialReflectExt, TypeIdExtensions};

use crate::{bevy_bindings::LuaBevyScriptingPlugin, namespaced_register::NamespaceBuilder};


pub fn register_bevy_bindings(app: &mut App) {
    app.add_plugins(LuaBevyScriptingPlugin);
}

pub fn register_world_functions(reg: &mut World) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<World>::new(reg)
        .register("spawn", |s: WorldCallbackAccess| Ok(Val(s.spawn()?)))
        .register(
            "get_type_by_name",
            |world: WorldCallbackAccess, type_name: String| {
                let val = world.get_type_by_name(type_name)?;
                Ok(val.map(Val))
            },
        )
        .register(
            "get_component",
            |world: WorldCallbackAccess,
             entity: Val<Entity>,
             registration: Val<ScriptTypeRegistration>| {
                registration
                    .component_id()
                    .and_then(|id| world.get_component(*entity, id).transpose())
                    .transpose()
            },
        )
        .register(
            "has_component",
            |s: WorldCallbackAccess,
             entity: Val<Entity>,
             registration: Val<ScriptTypeRegistration>| {
                match registration.component_id() {
                    Some(id) => s.has_component(*entity, id),
                    None => Ok(false),
                }
            },
        )
        .register(
            "remove_component",
            |s: WorldCallbackAccess, e: Val<Entity>, r: Val<ScriptTypeRegistration>| {
                s.remove_component(*e, r.clone())
            },
        )
        .register(
            "get_resource",
            |world: WorldCallbackAccess, registration: Val<ScriptTypeRegistration>| {
                match registration.resource_id() {
                    Some(id) => Ok(world.get_resource(id)?),
                    None => Ok(None),
                }
            },
        )
        .register(
            "has_resource",
            |s: WorldCallbackAccess, registration: Val<ScriptTypeRegistration>| match registration
                .resource_id()
            {
                Some(id) => s.has_resource(id),
                None => Ok(false),
            },
        )
        .register(
            "remove_resource",
            |s: WorldCallbackAccess, r: Val<ScriptTypeRegistration>| s.remove_resource(r.clone()),
        )
        .register(
            "add_default_component",
            |w: WorldCallbackAccess, e: Val<Entity>, r: Val<ScriptTypeRegistration>| {
                w.add_default_component(*e, r.clone())
            },
        )
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
        .register(
            "query",
            |s: WorldCallbackAccess, components: Vec<Val<ScriptTypeRegistration>>| {
                let mut query_builder = ScriptQueryBuilder::default();
                query_builder.components(components.into_iter().map(|v| v.into_inner()).collect());
                Ok(Val(query_builder))
            },
        )
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
                let world = w.try_read().expect("Stale world");
                s.display_with_world(world)
            },
        )
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
                let world = world.try_read().expect("Stale world");
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
                    let world = world.try_read().expect("stale world");
                    let mut path: ParsedPath = key.try_into().unwrap();
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
                            r.try_apply(other.as_partial_reflect()).unwrap();
                            Ok::<_, InteropError>(())
                        })
                        .into();
                    return r;
                }
                ScriptValue::Unit
            },
        )
        .register(
            "push",
            |w: WorldCallbackAccess, s: ReflectReference, v: ScriptValue| {
                let world = w.try_read().expect("stale world");
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
            let world = w.try_read().expect("stale world");
            let o = s.with_reflect_mut(world.clone(), |s| s.try_pop_boxed())??;
            let reference = { 
                let allocator = world.allocator();
                let mut allocator = allocator.write();
                ReflectReference::new_allocated_boxed(o, &mut allocator)
            };

            ReflectReference::into_script_ref(reference, world)
        })
        .register("insert", |caller_context: CallerContext, w: WorldCallbackAccess, s: ReflectReference, k: ScriptValue, v: ScriptValue| {
            let world = w.try_read().expect("stale world");
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
            let world = w.try_read().expect("stale world");
            s.with_reflect_mut(world, |s| s.try_clear())?
        })
        .register("len", |w: WorldCallbackAccess, s: ReflectReference| {
            let world = w.try_read().expect("stale world");
            s.len(world)
        })
        .register("remove", |caller_context: CallerContext, w: WorldCallbackAccess, s: ReflectReference, k: ScriptValue| {
            let world = w.try_read().expect("stale world");
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

            removed.map(|some| {
                let reference = {
                    let allocator = world.allocator();
                    let mut allocator = allocator.write();
                    ReflectReference::new_allocated_boxed(some, &mut allocator)
                };
                ReflectReference::into_script_ref(reference, world)
            }).transpose()
        })
        .register("iter", |w: WorldCallbackAccess, s: ReflectReference| {
            let world = w.try_read().expect("stale world");
            let mut len = s.len(world.clone())?.unwrap_or_default();
            let mut infinite_iter = s.into_iter_infinite();
            let iter_function = move || {
                if len == 0 {
                    return Ok(ScriptValue::Unit);
                }

                let (next_ref, idx) = infinite_iter.next_ref();

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


trait Test: GetTypeRegistration {}

// impl Test for smol_str::SmolStr {}

pub fn register_script_type_registration_functions(
    registry: &mut World,
) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<ScriptTypeRegistration>::new(registry)
        .register("type_name", |s: Ref<ScriptTypeRegistration>| s.type_name())
        .register("short_name", |s: Ref<ScriptTypeRegistration>| {
            s.short_name()
        })
        .register("is_resource", |s: Ref<ScriptTypeRegistration>| {
            s.resource_id().is_some()
        })
        .register("is_component", |s: Ref<ScriptTypeRegistration>| {
            s.component_id().is_some()
        });
    Ok(())
}

pub fn register_script_query_builder_functions(
    registry: &mut World,
) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<ScriptQueryBuilder>::new(registry)
        .register(
            "with",
            |s: Val<ScriptQueryBuilder>, with: Val<ScriptTypeRegistration>| {
                let mut builder = s.into_inner();
                builder.with(vec![with.into_inner()]);
                Val(builder)
            },
        )
        .register(
            "without",
            |s: Val<ScriptQueryBuilder>, without: Val<ScriptTypeRegistration>| {
                let mut builder = s.into_inner();
                builder.without(vec![without.into_inner()]);
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
            let components = s.components.to_vec();
            Val::new(components)
        });
    Ok(())
}
