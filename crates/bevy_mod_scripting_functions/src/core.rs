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
    function::{from::Val, script_function::ScriptFunction},
    script_value::{FromScriptValue, IntoScriptValue, ScriptValue},
    ReflectReference, ReflectionPathExt, ScriptTypeRegistration, WorldAccessGuard,
    WorldCallbackAccess,
};
use error::InteropError;
use reflection_extensions::TypeIdExtensions;

use crate::namespaced_register::NamespaceBuilder;

pub struct CoreFunctionsPlugin;

pub trait RegisterScriptFunction {
    fn overwrite_script_function<M, N, F: ScriptFunction<M>>(&mut self, name: N, f: F) -> &mut Self
    where
        N: Into<Cow<'static, str>>;
}

impl<S: 'static> RegisterScriptFunction for NamespaceBuilder<'_, S> {
    fn overwrite_script_function<M, N, F: ScriptFunction<M>>(&mut self, name: N, f: F) -> &mut Self
    where
        N: Into<Cow<'static, str>>,
    {
        let dynamic_function = f.into_dynamic_function();
        self.overwrite(name, dynamic_function);
        self
    }
}

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

assert_impls_from_script!(Val<usize>);
assert_is_script_function!(|a: Val<usize>| Ok(2));

fn register_world_functions(reg: &mut FunctionRegistry) -> Result<(), FunctionRegistrationError> {
    NamespaceBuilder::<WorldCallbackAccess>::new(reg)
        // .overwrite_script_function("hello", |a: Val<usize>| 2)
        .overwrite(
            "test_vec",
            |s: WorldCallbackAccess, entities: Vec<Entity>| entities,
        )
        .overwrite("spawn", |s: WorldCallbackAccess| s.spawn())
        .overwrite(
            "get_type_by_name",
            |world: WorldCallbackAccess, type_name: String| world.get_type_by_name(type_name),
        )
        .overwrite(
            "get_component",
            |world: WorldCallbackAccess, entity: Entity, registration: ScriptTypeRegistration| {
                let s: ScriptValue = registration
                    .component_id()
                    .and_then(|id| world.get_component(entity, id).transpose())
                    .into();
                s
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
                    let script_val = <&dyn PartialReflect>::reference_into_script_value(r, world);
                    script_val.into()
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
                    let script_val = <&dyn PartialReflect>::reference_into_script_value(r, world);
                    script_val.into()
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
                    let r: ScriptValue = self_
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
                            .ok_or_else(|| InteropError::impossible_conversion(target_type_id))??;

                            r.try_apply(other.as_partial_reflect()).unwrap();
                            Ok::<_, InteropError>(())
                        })
                        .into();
                    return r;
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

                    let r: ScriptValue = self_
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
                            .ok_or_else(|| InteropError::impossible_conversion(target_type_id))??;

                            r.try_apply(other.as_partial_reflect()).unwrap();
                            Ok::<_, InteropError>(())
                        })
                        .into();
                    return r;
                }
                ScriptValue::Unit
            },
        );

    Ok(())
}
