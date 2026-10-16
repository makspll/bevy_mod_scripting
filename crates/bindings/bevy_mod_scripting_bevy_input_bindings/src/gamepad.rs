//! Gamepads: the [`Gamepad`] component, and the `world` function which finds them.

// `script_bindings` expects these in scope
use bevy_ecs::{entity::Entity, reflect::AppTypeRegistry, world::World};
use bevy_input::gamepad::{Gamepad, GamepadAxis, GamepadButton};
use bevy_math::Vec2;
use bevy_mod_scripting_bindings::{
    InteropError, ScriptQueryBuilder, WorldExtensions,
    function::{
        from::{R, V},
        script_function::FunctionCallContext,
    },
};
use bevy_mod_scripting_derive::script_bindings;

use crate::component_registration;

#[script_bindings(
    remote,
    name = "gamepad_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Gamepad {
    /// Whether the button is held down.
    fn pressed(_self: R<Gamepad>, button: R<GamepadButton>) -> bool {
        _self.pressed(*button)
    }

    /// Whether the button was pressed during the last update.
    fn just_pressed(_self: R<Gamepad>, button: R<GamepadButton>) -> bool {
        _self.just_pressed(*button)
    }

    /// Whether the button was released during the last update.
    fn just_released(_self: R<Gamepad>, button: R<GamepadButton>) -> bool {
        _self.just_released(*button)
    }

    /// The value of the button or axis, from `0` to `1` for buttons and from `-1` to `1` for axes, or nil if the gamepad has no such input.
    fn get(_self: R<Gamepad>, axis: R<GamepadAxis>) -> Option<f32> {
        _self.get(*axis)
    }

    /// The position of the left stick, from `-1` to `1` on each axis.
    fn left_stick(_self: R<Gamepad>) -> V<Vec2> {
        V(_self.left_stick())
    }

    /// The position of the right stick, from `-1` to `1` on each axis.
    fn right_stick(_self: R<Gamepad>) -> V<Vec2> {
        V(_self.right_stick())
    }

    /// The direction of the d-pad, from `-1` to `1` on each axis.
    fn dpad(_self: R<Gamepad>) -> V<Vec2> {
        V(_self.dpad())
    }
}

#[script_bindings(
    remote,
    name = "world_input_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    unregistered
)]
impl World {
    /// The entities of the connected gamepads.
    fn gamepads(ctxt: FunctionCallContext) -> Result<Vec<V<Entity>>, InteropError> {
        let world = ctxt.world()?;
        let Some(gamepad) = component_registration::<Gamepad>(&world) else {
            return Ok(Vec::new());
        };
        let mut query = ScriptQueryBuilder::new();
        query.with_component(gamepad);
        let mut gamepads = Vec::new();
        for result in world.query(query)? {
            gamepads.push(V(result.entity));
        }
        Ok(gamepads)
    }
}

/// Registers the functions of gamepads.
pub(crate) fn register(world: &mut World) {
    register_gamepad_functions(world);
    register_world_input_functions(world);
}
