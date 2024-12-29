//! Contains functions defined by the [`bevy_mod_scripting_core`] crate
use std::borrow::Cow;

use bevy::{
    prelude::*,
    reflect::{
        func::{FunctionRegistrationError, FunctionRegistry, FunctionRegistryArc},
        ParsedPath,
    },
};
use bevy_mod_scripting_core::*;
use bindings::{
    function::{
        from::{Mut, Ref, Val},
        from_ref::FromScriptRef,
        into_ref::IntoScriptRef,
        script_function::{GetFunctionTypeDependencies, ScriptFunction},
    },
    pretty_print::DisplayWithWorld,
    script_value::ScriptValue,
    ReflectReference, ReflectionPathExt, ScriptQueryBuilder, ScriptQueryResult,
    ScriptTypeRegistration, WorldAccessGuard, WorldCallbackAccess,
};
use error::InteropError;
use reflection_extensions::TypeIdExtensions;

use crate::{bevy_bindings::LuaBevyScriptingPlugin, namespaced_register::NamespaceBuilder};

pub trait RegisterScriptFunction {
    fn overwrite_script_function<M, N, F>(&mut self, name: N, f: F) -> &mut Self
    where
        N: Into<Cow<'static, str>>,
        F: ScriptFunction<'static, M> + GetFunctionTypeDependencies<M>;
}

impl<S: 'static> RegisterScriptFunction for NamespaceBuilder<'_, S> {
    fn overwrite_script_function<M, N, F>(&mut self, name: N, f: F) -> &mut Self
    where
        N: Into<Cow<'static, str>>,
        F: ScriptFunction<'static, M> + GetFunctionTypeDependencies<M>,
    {
        {
            let registry = self.world.get_resource_or_init::<AppTypeRegistry>();
            let mut registry = registry.write();
            F::register_type_dependencies(&mut registry);
        }
        let dynamic_function = f.into_dynamic_function();
        self.overwrite(name, dynamic_function);
        self
    }
}

pub fn register_bevy_bindings(app: &mut App) {
    app.add_plugins(LuaBevyScriptingPlugin);
}

pub fn register_world_functions(reg: &mut World) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<WorldCallbackAccess>::new(reg)
        .overwrite_script_function("spawn", |s: WorldCallbackAccess| Ok(Val(s.spawn()?)))
        .overwrite_script_function(
            "get_type_by_name",
            |world: WorldCallbackAccess, type_name: String| {
                let val = world.get_type_by_name(type_name)?;
                Ok(val.map(Val))
            },
        )
        .overwrite_script_function(
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
        .overwrite_script_function(
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
        .overwrite_script_function(
            "remove_component",
            |s: WorldCallbackAccess, e: Val<Entity>, r: Val<ScriptTypeRegistration>| {
                s.remove_component(*e, r.clone())
            },
        )
        .overwrite_script_function(
            "get_resource",
            |world: WorldCallbackAccess, registration: Val<ScriptTypeRegistration>| {
                match registration.resource_id() {
                    Some(id) => Ok(world.get_resource(id)?),
                    None => Ok(None),
                }
            },
        )
        .overwrite_script_function(
            "has_resource",
            |s: WorldCallbackAccess, registration: Val<ScriptTypeRegistration>| match registration
                .resource_id()
            {
                Some(id) => s.has_resource(id),
                None => Ok(false),
            },
        )
        .overwrite_script_function(
            "remove_resource",
            |s: WorldCallbackAccess, r: Val<ScriptTypeRegistration>| s.remove_resource(r.clone()),
        )
        .overwrite_script_function(
            "add_default_component",
            |w: WorldCallbackAccess, e: Val<Entity>, r: Val<ScriptTypeRegistration>| {
                w.add_default_component(*e, r.clone())
            },
        )
        .overwrite_script_function(
            "insert_children",
            |w: WorldCallbackAccess, e: Val<Entity>, index: usize, c: Vec<Val<Entity>>| {
                w.insert_children(*e, index, &c.into_iter().map(|v| *v).collect::<Vec<_>>())
            },
        )
        .overwrite_script_function(
            "push_children",
            |w: WorldCallbackAccess, e: Val<Entity>, c: Vec<Val<Entity>>| {
                w.push_children(*e, &c.into_iter().map(|v| *v).collect::<Vec<_>>())
            },
        )
        .overwrite_script_function("get_children", |w: WorldCallbackAccess, e: Val<Entity>| {
            let children = w.get_children(*e)?;
            Ok(children.into_iter().map(Val).collect::<Vec<_>>())
        })
        .overwrite_script_function("get_parent", |w: WorldCallbackAccess, e: Val<Entity>| {
            let parent = w.get_parent(*e)?;
            Ok(parent.map(Val))
        })
        .overwrite_script_function("despawn", |s: WorldCallbackAccess, e: Val<Entity>| {
            s.despawn(*e)
        })
        .overwrite_script_function(
            "despawn_descendants",
            |s: WorldCallbackAccess, e: Val<Entity>| s.despawn_descendants(*e),
        )
        .overwrite_script_function(
            "despawn_recursive",
            |s: WorldCallbackAccess, e: Val<Entity>| s.despawn_recursive(*e),
        )
        .overwrite_script_function("has_entity", |s: WorldCallbackAccess, e: Val<Entity>| {
            s.has_entity(*e)
        })
        .overwrite_script_function(
            "query",
            |s: WorldCallbackAccess, components: Vec<Val<ScriptTypeRegistration>>| {
                let mut query_builder = ScriptQueryBuilder::default();
                query_builder.components(components.into_iter().map(|v| v.into_inner()).collect());
                Ok(Val(query_builder))
            },
        )
        .overwrite_script_function("exit", |s: WorldCallbackAccess| s.exit());
    Ok(())
}

pub fn register_reflect_reference_functions(
    reg: &mut World,
) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<ReflectReference>::new(reg)
        .overwrite_script_function(
            "display_ref",
            |w: WorldCallbackAccess, s: ReflectReference| {
                let world = w.try_read().expect("Stale world");
                s.display_with_world(world)
            },
        )
        .overwrite_script_function(
            "get",
            |world: WorldCallbackAccess, mut self_: ReflectReference, key: ScriptValue| {
                let path: ParsedPath = key.try_into()?;
                self_.index_path(path);
                let world = world.try_read().expect("Stale world");
                ReflectReference::into_script_ref(self_, world)
            },
        )
        .overwrite_script_function(
            "get_1_indexed",
            |world: WorldCallbackAccess, mut self_: ReflectReference, key: ScriptValue| {
                let mut path: ParsedPath = key.try_into()?;
                path.convert_to_0_indexed();
                self_.index_path(path);
                let world = world.try_read().expect("Stale world");
                ReflectReference::into_script_ref(self_, world)
            },
        )
        .overwrite_script_function(
            "set",
            |world: WorldCallbackAccess,
             self_: ScriptValue,
             key: ScriptValue,
             value: ScriptValue| {
                if let ScriptValue::Reference(mut self_) = self_ {
                    let world = world.try_read().expect("stale world");
                    let path: ParsedPath = key.try_into().unwrap();

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
        .overwrite_script_function(
            "set_1_indexed",
            |world: WorldCallbackAccess,
             mut self_: ReflectReference,
             key: ScriptValue,
             value: ScriptValue| {
                let world = world.try_read().expect("stale world");
                let mut path: ParsedPath = key.try_into()?;
                path.convert_to_0_indexed();
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
                Ok(r)
            },
        );

    Ok(())
}

pub fn register_script_type_registration_functions(
    registry: &mut World,
) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<ScriptTypeRegistration>::new(registry)
        .overwrite_script_function("type_name", |s: Ref<ScriptTypeRegistration>| s.type_name())
        .overwrite_script_function("short_name", |s: Ref<ScriptTypeRegistration>| {
            s.short_name()
        })
        .overwrite_script_function("is_resource", |s: Ref<ScriptTypeRegistration>| {
            s.resource_id().is_some()
        })
        .overwrite_script_function("is_component", |s: Ref<ScriptTypeRegistration>| {
            s.component_id().is_some()
        });
    Ok(())
}

pub fn register_script_query_builder_functions(
    registry: &mut World,
) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<ScriptQueryBuilder>::new(registry)
        .overwrite_script_function(
            "with",
            |s: Val<ScriptQueryBuilder>, with: Val<ScriptTypeRegistration>| {
                let mut builder = s.into_inner();
                builder.with(vec![with.into_inner()]);
                Val(builder)
            },
        )
        .overwrite_script_function(
            "without",
            |s: Val<ScriptQueryBuilder>, without: Val<ScriptTypeRegistration>| {
                let mut builder = s.into_inner();
                builder.without(vec![without.into_inner()]);
                Val(builder)
            },
        )
        .overwrite_script_function(
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
        .overwrite_script_function("entity", |s: Ref<ScriptQueryResult>| Val::new(s.entity))
        .overwrite_script_function("components", |s: Ref<ScriptQueryResult>| {
            let components = s.components.to_vec();
            Val::new(components)
        });
    Ok(())
}
