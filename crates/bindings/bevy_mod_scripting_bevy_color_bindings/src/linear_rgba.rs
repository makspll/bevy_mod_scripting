//! [`LinearRgba`] colors: red, green, blue and alpha in linear light, as used for rendering and light intensity.

// `script_bindings` expects these in scope
use bevy_color::{Color, LinearRgba};
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "linear_rgba_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "color",
    core
)]
impl LinearRgba {
    /// A color from linear red, green, blue and alpha, each from `0` to `1`.
    ///
    /// ```multi-lang-example,lua
    /// local LinearRgba = import_namespace("color").LinearRgba
    /// local c = LinearRgba.new(1.0, 0.0, 0.0, 1.0)
    /// assert(c.red == 1.0)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let LinearRgba = import_namespace("color").LinearRgba;
    /// let c = LinearRgba.new(1.0, 0.0, 0.0, 1.0);
    /// assert!(c.red == 1.0);
    /// ```
    fn new(red: f32, green: f32, blue: f32, alpha: f32) -> V<LinearRgba> {
        V(LinearRgba::new(red, green, blue, alpha))
    }

    /// A fully opaque color from linear red, green and blue, each from `0` to `1`.
    fn rgb(red: f32, green: f32, blue: f32) -> V<LinearRgba> {
        V(LinearRgba::rgb(red, green, blue))
    }

    /// The color as a [`Color`](bevy_color::Color).
    fn to_color(_self: R<LinearRgba>) -> V<Color> {
        V(Color::LinearRgba(*_self))
    }

    /// A copy with red, green and blue multiplied by `factor`, keeping alpha. Useful for making emissive colors brighter.
    fn mul(_self: R<LinearRgba>, factor: f32) -> V<LinearRgba> {
        V(LinearRgba::new(
            _self.red * factor,
            _self.green * factor,
            _self.blue * factor,
            _self.alpha,
        ))
    }

    /// Whether both colors have the same red, green, blue and alpha.
    fn eq(_self: R<LinearRgba>, other: R<LinearRgba>) -> bool {
        *_self == *other
    }
}

/// Registers the [`LinearRgba`] functions.
pub(crate) fn register(world: &mut World) {
    register_linear_rgba_functions(world);
}
