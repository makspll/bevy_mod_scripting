//! Core globals exposed by the BMS framework

use std::{collections::HashMap, sync::Arc};

use bevy::{app::Plugin, ecs::reflect::AppTypeRegistry};
use bevy_mod_scripting_derive::script_globals;

use crate::{bindings::{function::from::{Union, Val}, ScriptComponentRegistration, ScriptResourceRegistration, ScriptTypeRegistration, WorldGuard}, docgen::into_through_type_info, error::InteropError};

use super::AppScriptGlobalsRegistry;

/// A plugin introducing core globals for the BMS framework
pub struct CoreScriptGlobalsPlugin;

impl Plugin for CoreScriptGlobalsPlugin {
    fn build(&self, _app: &mut bevy::app::App) {}
    fn finish(&self, app: &mut bevy::app::App) {
        register_static_core_globals(app.world_mut());
        register_core_globals(app.world_mut());
    }
}

fn register_static_core_globals(world: &mut bevy::ecs::world::World) {
    let global_registry = world
            .get_resource_or_init::<AppScriptGlobalsRegistry>()
            .clone();
        let type_registry = world
            .get_resource_or_init::<AppTypeRegistry>()
            .clone();
        let mut global_registry = global_registry.write();
        let type_registry = type_registry.read();

        // find all reflectable types without generics
        for registration in type_registry.iter() {
            if !registration.type_info().generics().is_empty() {
                continue;
            }

            if let Some(global_name) = registration.type_info().type_path_table().ident() {
                let documentation = "A reference to the type, allowing you to call static methods.";
                let type_info = registration.type_info();
                global_registry.register_static_documented_dynamic(
                    registration.type_id(),
                    into_through_type_info(type_info),
                    global_name.into(),
                    documentation.into(),
                );
            }
        }
}

#[script_globals(
    bms_core_path = "crate",
    name = "core_globals",
)]
impl CoreGlobals {
    /// A cache of types normally available through the `world.get_type_by_name` function.
    /// 
    /// You can use this to avoid having to store type references.
    fn types(guard: WorldGuard) -> Result<HashMap<String, Union<Val<ScriptTypeRegistration>, Union<Val<ScriptComponentRegistration>, Val<ScriptResourceRegistration>>>>, InteropError> {
        let type_registry = guard.type_registry();
        let type_registry = type_registry.read();
        let mut type_cache = HashMap::<String, _>::default();
        for registration in type_registry.iter(){
            if let Some(ident) = registration.type_info().type_path_table().ident() {
                let registration = ScriptTypeRegistration::new(Arc::new(registration.clone()));
                let registration = guard.clone().get_type_registration(registration)?;
                let registration = registration.map_both(Val::from, |u| u.map_both(Val::from, Val::from));
                type_cache.insert(ident.to_string(), registration);
            }
        }
        
        Ok(type_cache)
    }
}