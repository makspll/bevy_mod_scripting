//! Core globals exposed by the BMS framework

use crate::function::from::V;
use crate::docgen::into_through_type_info;
use ::{
    bevy_app::Plugin,
    bevy_asset::Handle,
    bevy_ecs::{entity::Entity, reflect::AppTypeRegistry, world::World},
    bevy_reflect::TypeRegistration,
};
use bevy_app::App;
use bevy_log::warn;
use bevy_mod_scripting_asset::ScriptAsset;

use super::{AppScriptGlobalsRegistry, AppScriptTypeCatalogue};

/// A plugin introducing core globals for the BMS framework.
///
/// By default all types added to the type registry are present as globals, you can customize this behavior
/// by providing a filter function
pub struct CoreScriptGlobalsPlugin {
    /// the filter function used to determine which types are registered as globals
    /// When `true` for the given type registration, the type will be registered as a global.
    pub filter: fn(&TypeRegistration) -> bool,

    /// Whether to register static references to types
    /// By default static type references such as `Vec3` or `Mat3` are accessible directly from the global namespace.
    pub register_static_references: bool,
}

impl Default for CoreScriptGlobalsPlugin {
    fn default() -> Self {
        Self {
            filter: |_| true,
            register_static_references: true,
        }
    }
}

impl Plugin for CoreScriptGlobalsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppScriptGlobalsRegistry>();
        app.insert_resource(AppScriptTypeCatalogue::new(self.filter));
    }
    fn finish(&self, app: &mut App) {
        // profiling::function_scope!("app finish");

        if self.register_static_references {
            register_static_core_globals(app.world_mut(), self.filter);
        }
    }
}
const MSG_DUPLICATE_GLOBAL: &str = "This can cause confusing issues for script users, use `CoreScriptGlobalsPlugin.filter` to filter out uneeded duplicate types.";

#[profiling::function]
fn register_static_core_globals(world: &mut World, filter: fn(&TypeRegistration) -> bool) {
    let global_registry = world
        .get_resource_or_init::<AppScriptGlobalsRegistry>()
        .clone();
    let type_registry = world.get_resource_or_init::<AppTypeRegistry>().clone();
    let mut global_registry = global_registry.write();
    let type_registry = type_registry.read();

    // find all reflectable types without generics
    for registration in type_registry.iter().filter(|r| filter(r)) {
        if !registration.type_info().generics().is_empty() {
            continue;
        }

        if let Some(global_name) = registration.type_info().type_path_table().ident() {
            let documentation = "A reference to the type, allowing you to call static methods.";
            let type_info = registration.type_info();
            if global_registry
                .register_static_documented_dynamic(
                    registration.type_id(),
                    into_through_type_info(type_info),
                    global_name.into(),
                    documentation.into(),
                )
                .is_some()
            {
                warn!(
                    "Duplicate global registration for type: '{}'. {MSG_DUPLICATE_GLOBAL}",
                    type_info.type_path_table().short_path()
                )
            };
        }
    }

    // register basic globals
    if global_registry
        .register_dummy::<World>("world", "The current ECS world.")
        .is_some()
    {
        warn!("existing `world` global was replaced by the core `world` dummy type.")
    };

    if global_registry
        .register_dummy::<Entity>("entity", "The entity this script is attached to if any.")
        .is_some()
    {
        warn!("existing `entity` global was replaced by the core `entity` dummy type.")
    }

    if global_registry.register_dummy_typed::<V<Handle<ScriptAsset>>>("script_asset", "the asset handle for this script. If the asset is ever unloaded, the handle will be less useful.").is_some() {
        warn!("existing `script_asset` global was replaced by the core `script_asset` dummy type.")
    };
}


#[cfg(test)]
mod test {
    use super::*;
    use ::{bevy_app::App, bevy_reflect::Reflect};

    #[test]
    fn test_register_globals() {
        let mut app = App::new();

        // register a type
        #[derive(Debug, Clone, Reflect)]
        struct TestType;
        app.register_type::<TestType>();
        let plugin = CoreScriptGlobalsPlugin::default();
        plugin.build(&mut app);
        plugin.finish(&mut app);
        let globals = app
            .world()
            .get_resource::<AppScriptGlobalsRegistry>()
            .unwrap()
            .read();
        assert!(globals.get("TestType").is_some());

        // now do the same but with a filter
        let mut app = App::new();
        let plugin = CoreScriptGlobalsPlugin {
            filter: |_| false,
            register_static_references: true,
        };
        plugin.build(&mut app);
        plugin.finish(&mut app);

        let globals = app
            .world()
            .get_resource::<AppScriptGlobalsRegistry>()
            .unwrap()
            .read();

        // check that the type is not registered
        assert!(globals.is_empty());
    }
}
