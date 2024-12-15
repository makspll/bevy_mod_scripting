//! Contains functions defined by the [`bevy_mod_scripting_core`] crate
use bevy::{
    prelude::*,
    reflect::{
        func::{FunctionRegistrationError, FunctionRegistry, FunctionRegistryArc},
        ParsedPath,
    },
};
use bevy_mod_scripting_core::*;
use bindings::{
    script_val::{FromScriptValue, IntoScriptValue, ScriptValue},
    ReflectReference, ReflectionPathExt, ScriptTypeRegistration, WorldAccessGuard,
    WorldCallbackAccess,
};
use reflection_extensions::TypeIdExtensions;

use crate::namespaced_register::NamespaceBuilder;

pub struct CoreFunctionsPlugin;

impl Plugin for CoreFunctionsPlugin {
    fn build(&self, app: &mut App) {
        let function_registry = app
            .world_mut()
            .get_resource_or_init::<AppFunctionRegistry>();

        let mut function_registry = function_registry.write();

        // function_registry.register_with_name("spawn", || Entity::from_bits(2));
        register_world_functions(&mut function_registry)
            .expect("Failed to register world functions");
    }
}

fn register_world_functions(reg: &mut FunctionRegistry) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<WorldCallbackAccess>::new(reg)
        .overwrite("spawn", |s: WorldCallbackAccess| s.spawn())
        .overwrite(
            "get_type_by_name",
            |world: WorldCallbackAccess, type_name: String| world.get_type_by_name(type_name),
        )
        .overwrite(
            "get_component",
            |s: WorldCallbackAccess, entity: Entity, registration: ScriptTypeRegistration| {
                let c = s
                    .get_component(entity, registration.component_id().unwrap())
                    .unwrap();
                c.map(ScriptValue::Reference).unwrap_or(ScriptValue::Unit)
            },
        )
        .overwrite("exit", |s: WorldCallbackAccess| s.exit());

    NamespaceBuilder::<ReflectReference>::new(reg)
        .overwrite(
            "get",
            |world: WorldCallbackAccess, self_: ScriptValue, key: ScriptValue| {
                if let ScriptValue::Reference(mut r) = self_ {
                    let path: ParsedPath = key.try_into().unwrap();
                    r.index_path(path);
                    let world = world.read().expect("Stale world");
                    let script_val = r
                        .with_reflect(world.clone(), |r| r.into_script_value(world).unwrap())
                        .unwrap();
                    script_val
                } else {
                    ScriptValue::Unit
                }
            },
        )
        .overwrite(
            "get_1_indexed",
            |world: WorldCallbackAccess, self_: ScriptValue, key: ScriptValue| {
                if let ScriptValue::Reference(mut r) = self_ {
                    let mut path: ParsedPath = key.try_into().unwrap();
                    path.convert_to_0_indexed();
                    r.index_path(path);
                    let world = world.read().expect("Stale world");
                    let script_val = r
                        .with_reflect(world.clone(), |r| r.into_script_value(world).unwrap())
                        .unwrap();
                    script_val
                } else {
                    ScriptValue::Unit
                }
            },
        )
        .overwrite(
            "set",
            |world: WorldCallbackAccess,
             self_: ScriptValue,
             key: ScriptValue,
             value: ScriptValue| {
                if let ScriptValue::Reference(mut self_) = self_ {
                    let world = world.read().expect("stale world");
                    let path: ParsedPath = key.try_into().unwrap();

                    self_.index_path(path);
                    self_
                        .with_reflect_mut(world.clone(), |r| {
                            let target_type_id = r
                                .get_represented_type_info()
                                .map(|i| i.type_id())
                                .type_id_or_fake_id();
                            let other = <dyn PartialReflect>::from_script_value(
                                value,
                                world.clone(),
                                target_type_id,
                            )
                            .transpose()
                            .unwrap()
                            .unwrap();

                            r.try_apply(other.as_partial_reflect()).unwrap();
                            ScriptValue::Unit
                        })
                        .unwrap();
                }
                ScriptValue::Unit
            },
        )
        .overwrite(
            "set_1_indexed",
            |world: WorldCallbackAccess,
             self_: ScriptValue,
             key: ScriptValue,
             value: ScriptValue| {
                if let ScriptValue::Reference(mut self_) = self_ {
                    let world = world.read().expect("stale world");
                    let mut path: ParsedPath = key.try_into().unwrap();
                    path.convert_to_0_indexed();

                    self_.index_path(path);
                    self_
                        .with_reflect_mut(world.clone(), |r| {
                            let target_type_id = r
                                .get_represented_type_info()
                                .map(|i| i.type_id())
                                .type_id_or_fake_id();
                            let other = <dyn PartialReflect>::from_script_value(
                                value,
                                world.clone(),
                                target_type_id,
                            )
                            .transpose()
                            .unwrap()
                            .unwrap();

                            r.try_apply(other.as_partial_reflect()).unwrap();
                            ScriptValue::Unit
                        })
                        .unwrap();
                }
                ScriptValue::Unit
            },
        );

    Ok(())
}
