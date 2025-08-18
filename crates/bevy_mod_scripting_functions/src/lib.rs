#![allow(missing_docs)]

use bevy::prelude::*;
pub mod bevy_bindings;
pub mod core;
pub use core::*;

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
}
