//! Core globals exposed by the BMS framework

use bevy::{app::Plugin, ecs::reflect::AppTypeRegistry};

use super::AppScriptGlobalsRegistry;

/// A plugin introducing core globals for the BMS framework
pub struct CoreScriptGlobalsPlugin;

impl Plugin for CoreScriptGlobalsPlugin {
    fn build(&self, _app: &mut bevy::app::App) {}
    fn finish(&self, app: &mut bevy::app::App) {
        let global_registry = app
            .world_mut()
            .get_resource_or_init::<AppScriptGlobalsRegistry>()
            .clone();
        let type_registry = app
            .world_mut()
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
                global_registry.register_static_documented_dynamic(
                    registration.type_id(),
                    None,
                    global_name.into(),
                    documentation.into(),
                )
            }
        }
    }
}
