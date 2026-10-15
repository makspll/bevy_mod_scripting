//! [`TextSpan`](bevy_text::TextSpan): a piece of text inside a text block, with its own font.

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{M, R, V};
use bevy_mod_scripting_derive::script_bindings;
use bevy_text::TextSpan;

#[script_bindings(
    remote,
    name = "text_span_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl TextSpan {
    /// Creates a new text span holding `text`.
    fn new(text: String) -> V<TextSpan> {
        V(TextSpan::new(text))
    }

    /// The text as a string.
    fn as_str(_self: R<TextSpan>) -> String {
        _self.as_str().to_owned()
    }

    /// Replaces the text.
    fn set(mut _self: M<TextSpan>, text: String) {
        _self.0.0 = text;
    }
}

/// Registers the [`TextSpan`] functions.
pub(crate) fn register(world: &mut World) {
    register_text_span_functions(world);
}
