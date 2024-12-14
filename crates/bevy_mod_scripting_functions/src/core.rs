//! Contains functions defined by the [`bevy_mod_scripting_core`] crate
use bevy::{
    prelude::*,
    reflect::func::{FunctionRegistrationError, FunctionRegistry, FunctionRegistryArc},
};
use bevy_mod_scripting_core::*;
use bindings::{
    script_val::ScriptValue, ScriptTypeRegistration, WorldAccessGuard, WorldCallbackAccess,
};

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
        .register_function("spawn", |s: WorldCallbackAccess| s.spawn())?
        .register_function(
            "get_type_by_name",
            |world: WorldCallbackAccess, type_name: String| world.get_type_by_name(type_name),
        )?
        .register_function(
            "get_component",
            |s: WorldCallbackAccess, entity: Entity, registration: ScriptTypeRegistration| {
                let c = s
                    .get_component(entity, registration.component_id().unwrap())
                    .unwrap();
                c.map(ScriptValue::Reference).unwrap_or(ScriptValue::Unit)
            },
        )?
        .register_function("exit", |s: WorldCallbackAccess| s.exit())?;

    Ok(())
}
