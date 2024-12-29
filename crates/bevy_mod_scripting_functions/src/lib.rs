use ::bevy::prelude::*;
pub mod bevy_bindings;
pub mod core;

pub mod namespaced_register;

pub use core::*;
pub use namespaced_register::*;

pub struct ScriptFunctionsPlugin;

impl Plugin for ScriptFunctionsPlugin {
    fn build(&self, app: &mut App) {
        register_bevy_bindings(app);
        let world = app.world_mut();

        register_world_functions(world).expect("Failed to register world functions");

        register_reflect_reference_functions(world)
            .expect("Failed to register reflect reference functions");

        register_script_type_registration_functions(world)
            .expect("Failed to register script type registration functions");

        register_script_query_builder_functions(world)
            .expect("Failed to register script query builder functions");

        register_script_query_result_functions(world)
            .expect("Failed to register script query result functions");
    }
}
