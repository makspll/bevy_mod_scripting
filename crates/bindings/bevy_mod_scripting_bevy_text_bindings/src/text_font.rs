//! [`TextFont`](bevy_text::TextFont): the font face and size of a piece of text.

// `script_bindings` expects these in scope
use bevy_asset::Handle;
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;
use bevy_text::{Font, TextFont};

#[script_bindings(
    remote,
    name = "text_font_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "text",
    core
)]
impl TextFont {
    /// Creates a text font with the default font face and the given size in logical pixels.
    ///
    /// ```multi-lang-example,lua
    /// local TextFont = import_namespace("text").TextFont
    /// local font = TextFont.from_font_size(20.0)
    /// assert(font:with_font_size(30.0) ~= nil)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let TextFont = import_namespace("text").TextFont;
    /// let font = TextFont.from_font_size(20.0);
    /// assert!(font.with_font_size(30.0) != ());
    /// ```
    fn from_font_size(font_size: f32) -> V<TextFont> {
        V(TextFont::from_font_size(font_size))
    }

    /// A copy of the text font with its size set to `font_size` in logical pixels.
    fn with_font_size(_self: R<TextFont>, font_size: f32) -> V<TextFont> {
        V((*_self).clone().with_font_size(font_size))
    }

    /// A copy of the text font that uses the font face `font`.
    fn with_font(_self: R<TextFont>, font: R<Handle<Font>>) -> V<TextFont> {
        V((*_self).clone().with_font((*font).clone()))
    }
}

/// Registers the [`TextFont`] functions.
pub(crate) fn register(world: &mut World) {
    register_text_font_functions(world);
}
