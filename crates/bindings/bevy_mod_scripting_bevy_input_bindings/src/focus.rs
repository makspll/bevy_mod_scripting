//! Focus: the [`InputFocus`] resource, which says which entity receives keyboard input.

// `script_bindings` expects these in scope
use bevy_ecs::{entity::Entity, reflect::AppTypeRegistry, world::World};
use bevy_input_focus::{FocusCause, InputFocus};
use bevy_mod_scripting_bindings::function::from::{M, R, V};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "input_focus_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl InputFocus {
    /// Gives the focus to the entity, so it receives keyboard input.
    fn set(mut _self: M<InputFocus>, entity: R<Entity>) {
        _self.set(*entity, FocusCause::Navigated);
    }

    /// Takes the focus away from every entity.
    fn clear(mut _self: M<InputFocus>) {
        _self.clear();
    }

    /// The entity which has the focus, or nil if no entity has it.
    fn get(_self: R<InputFocus>) -> Option<V<Entity>> {
        _self.get().map(V)
    }
}

/// Registers the functions of focus.
pub(crate) fn register(world: &mut World) {
    register_input_focus_functions(world);
}
