//! Touch: the [`Touches`] resource of the fingers on the screen, and a single [`Touch`].

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_input::touch::{Touch, Touches};
use bevy_math::Vec2;
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "touches_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Touches {
    /// Whether a finger has touched the screen during the last update.
    fn any_just_pressed(_self: R<Touches>) -> bool {
        _self.any_just_pressed()
    }

    /// The position of the first finger on the screen, or nil if no finger is on it.
    fn first_pressed_position(_self: R<Touches>) -> Option<V<Vec2>> {
        _self.first_pressed_position().map(V)
    }

    /// Every finger on the screen.
    fn iter(_self: R<Touches>) -> Vec<V<Touch>> {
        _self.iter().cloned().map(V).collect()
    }
}

#[script_bindings(
    remote,
    name = "touch_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Touch {
    /// The current position of the finger, in logical pixels.
    fn position(_self: R<Touch>) -> V<Vec2> {
        V(_self.position())
    }

    /// The position where the finger first touched the screen, in logical pixels.
    fn start_position(_self: R<Touch>) -> V<Vec2> {
        V(_self.start_position())
    }

    /// How far the finger moved since the last update, in logical pixels.
    fn delta(_self: R<Touch>) -> V<Vec2> {
        V(_self.delta())
    }

    /// The id of the finger, which stays the same while the finger is on the screen.
    fn id(_self: R<Touch>) -> u64 {
        _self.id()
    }
}

/// Registers the functions of touches.
pub(crate) fn register(world: &mut World) {
    register_touches_functions(world);
    register_touch_functions(world);
}
