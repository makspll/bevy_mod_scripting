use ::bevy::prelude::*;
#[cfg(feature = "bevy_bindings")]
pub mod bevy_bindings;
pub mod core;

pub mod namespaced_register;

pub use core::*;
pub use namespaced_register::*;

pub struct ScriptFunctionsPlugin;

impl Plugin for ScriptFunctionsPlugin {
    fn build(&self, app: &mut App) {
        register_bevy_bindings(app);
        register_core_functions(app);
    }
}
