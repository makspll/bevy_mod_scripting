//! [`Text2d`](bevy_sprite::Text2d): text shown in the 2d world.

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{M, R, V};
use bevy_mod_scripting_derive::script_bindings;
use bevy_sprite::Text2d;

#[script_bindings(
    remote,
    name = "text2d_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Text2d {
    /// Creates a new 2d text component holding `text`.
    fn new(text: String) -> V<Text2d> {
        V(Text2d::new(text))
    }

    /// The text as a string.
    fn as_str(_self: R<Text2d>) -> String {
        _self.as_str().to_owned()
    }

    /// Replaces the text.
    fn set(mut _self: M<Text2d>, text: String) {
        _self.0.0 = text;
    }
}

/// Registers the [`Text2d`] functions.
pub(crate) fn register(world: &mut World) {
    register_text2d_functions(world);
}
