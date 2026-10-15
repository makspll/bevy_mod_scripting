//! Easing curves: [`EaseFunction`](bevy_curve::EaseFunction).

// `script_bindings` expects these in scope
use bevy_curve::{Curve, EaseFunction};
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::R;
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "ease_function_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl EaseFunction {
    /// The eased value at `t`, clamped to between 0 and 1.
    fn sample(_self: R<EaseFunction>, t: f32) -> f32 {
        _self.sample_clamped(t)
    }
}

/// Registers the functions of easing curves.
pub(crate) fn register(world: &mut World) {
    register_ease_function_functions(world);
}
