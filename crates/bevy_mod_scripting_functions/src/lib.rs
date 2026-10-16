#![allow(missing_docs)]

pub mod constructors;
pub mod core;
pub use constructors::ConstructorsPlugin;
pub use core::*;

use bevy_app::{App, Plugin};
use bevy_ecs::hierarchy::{ChildOf, Children};

/// A plugin that registers the core scripting functions.
#[derive(Default)]
pub struct ScriptFunctionsPlugin;

impl Plugin for ScriptFunctionsPlugin {
    fn build(&self, app: &mut App) {
        register_bevy_bindings(app);
        register_core_functions(app);

        // TODO: if bevy ever does this itself we should remove this
        let world_mut = app.world_mut();
        world_mut.register_component::<ChildOf>();
        world_mut.register_component::<Children>();
    }

    fn finish(&self, app: &mut App) {
        // BMS' own types live in the `bms` namespace
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &[
                "bevy_mod_scripting_asset",
                "bevy_mod_scripting_bindings",
                "bevy_mod_scripting_bindings_domain",
                "bevy_mod_scripting_core",
                "bevy_mod_scripting_display",
                "bevy_mod_scripting_functions",
                "bevy_mod_scripting_script",
                "bevy_mod_scripting_world",
                "bevy_system_reflection",
            ],
            "bms",
        );
    }
}
