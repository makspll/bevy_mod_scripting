//! Core globals exposed by the BMS framework

use crate::function::from::V;
use ::{
    bevy_app::Plugin,
    bevy_asset::Handle,
    bevy_ecs::{entity::Entity, world::World},
    bevy_reflect::TypeRegistration,
};
use bevy_app::App;
use bevy_log::warn;
use bevy_mod_scripting_asset::ScriptAsset;

use super::{AppScriptGlobalsRegistry, AppScriptTypeCatalogue};

/// A plugin introducing core globals for the BMS framework.
///
/// Types are not globals: scripts reach them through their namespace instead, i.e. `import_namespace("math").Vec3`,
/// which avoids types with the same name shadowing each other.
pub struct CoreScriptGlobalsPlugin {
    /// The filter function used to determine which types scripts can look up by name.
    /// When `true` for the given type registration, the type is part of the script type catalogue.
    pub filter: fn(&TypeRegistration) -> bool,
}

impl Default for CoreScriptGlobalsPlugin {
    fn default() -> Self {
        Self { filter: |_| true }
    }
}

impl Plugin for CoreScriptGlobalsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppScriptGlobalsRegistry>();
        app.insert_resource(AppScriptTypeCatalogue::new(self.filter));
    }
    fn finish(&self, app: &mut App) {
        // profiling::function_scope!("app finish");

        register_core_globals(app.world_mut());
    }
}

#[profiling::function]
fn register_core_globals(world: &mut World) {
    let global_registry = world
        .get_resource_or_init::<AppScriptGlobalsRegistry>()
        .clone();
    let mut global_registry = global_registry.write();

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
    fn types_are_not_registered_as_globals() {
        let mut app = App::new();

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

        // types are reached through their namespace
        assert!(globals.get("TestType").is_none());
        assert!(globals.is_empty());
    }
}
