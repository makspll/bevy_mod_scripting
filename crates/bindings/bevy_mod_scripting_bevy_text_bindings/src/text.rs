//! [`Text`](bevy_ui::widget::Text): text shown in the UI.

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{M, R, V};
use bevy_mod_scripting_derive::script_bindings;
use bevy_ui::widget::Text;

#[script_bindings(
    remote,
    name = "text_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "text",
    core
)]
impl Text {
    /// Creates a new text component holding `text`.
    ///
    /// ```multi-lang-example,lua
    /// local Text = import_namespace("text").Text
    /// local text = Text.new("hi")
    /// assert(text:as_str() == "hi")
    /// ```
    ///
    /// ```multi-lang-example,rhai
    /// let Text = import_namespace.call("text").Text;
    /// let text = Text.new_.call("hi");
    /// assert(text.as_str.call() == "hi");
    /// ```
    fn new(text: String) -> V<Text> {
        V(Text::new(text))
    }

    /// The text as a string.
    fn as_str(_self: R<Text>) -> String {
        _self.as_str().to_owned()
    }

    /// Replaces the text.
    fn set(mut _self: M<Text>, text: String) {
        _self.0.0 = text;
    }
}

/// Registers the [`Text`] functions.
pub(crate) fn register(world: &mut World) {
    register_text_functions(world);
}
