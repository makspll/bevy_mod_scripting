//! [`TextLayout`](bevy_text::TextLayout): how a block of text is aligned and wrapped.

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;
use bevy_text::{Justify, TextLayout};

#[script_bindings(
    remote,
    name = "text_layout_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl TextLayout {
    /// A text layout that aligns lines with the given justification and wraps at word boundaries.
    fn new_with_justify(justify: R<Justify>) -> V<TextLayout> {
        V(TextLayout::justify(*justify))
    }

    /// A text layout that never wraps soft line breaks, only explicit newlines start a new line.
    fn new_with_no_wrap() -> V<TextLayout> {
        V(TextLayout::no_wrap())
    }
}

/// Registers the [`TextLayout`] functions.
pub(crate) fn register(world: &mut World) {
    register_text_layout_functions(world);
}
