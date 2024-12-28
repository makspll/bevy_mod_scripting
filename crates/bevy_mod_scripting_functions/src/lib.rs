use ::bevy::prelude::*;
#[cfg(feature = "core_functions")]
pub mod bevy;
#[cfg(feature = "core_functions")]
pub mod core;

pub mod namespaced_register;

pub struct BevyFunctionsPlugin;

impl Plugin for BevyFunctionsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "core_functions")]
        app.add_plugins(core::CoreFunctionsPlugin);
    }
}
