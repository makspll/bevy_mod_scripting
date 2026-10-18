//! [`Srgba`] colors: gamma corrected red, green, blue and alpha, as used by most image formats and CSS.

// `script_bindings` expects these in scope
use bevy_color::{Alpha, Color, Srgba};
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::{
    InteropError,
    function::from::{R, V},
};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "srgba_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "color",
    core
)]
impl Srgba {
    /// A color from red, green, blue and alpha, each from `0` to `1`.
    ///
    /// ```multi-lang-example,lua
    /// local Srgba = import_namespace("color").Srgba
    /// local c = Srgba.new(1.0, 0.0, 0.0, 0.5)
    /// assert(c.alpha == 0.5)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let Srgba = import_namespace("color").Srgba;
    /// let c = Srgba.new(1.0, 0.0, 0.0, 0.5);
    /// assert!(c.alpha == 0.5);
    /// ```
    fn new(red: f32, green: f32, blue: f32, alpha: f32) -> V<Srgba> {
        V(Srgba::new(red, green, blue, alpha))
    }

    /// A fully opaque color from red, green and blue, each from `0` to `1`.
    fn rgb(red: f32, green: f32, blue: f32) -> V<Srgba> {
        V(Srgba::rgb(red, green, blue))
    }

    /// A color from a hex string such as `"#ff8800"` or `"#ff880080"`, or an error if the string is not valid hex.
    fn hex(hex: String) -> Result<V<Srgba>, InteropError> {
        Srgba::hex(&hex)
            .map(V)
            .map_err(|e| InteropError::string(format!("invalid hex color `{hex}`: {e}")))
    }

    /// The color as an uppercase hex string such as `"#FF8800"`, with alpha appended unless fully opaque.
    fn to_hex(_self: R<Srgba>) -> String {
        _self.to_hex()
    }

    /// A copy of the color with its alpha set to `alpha`.
    fn with_alpha(_self: R<Srgba>, alpha: f32) -> V<Srgba> {
        V(_self.with_alpha(alpha))
    }

    /// The color as a [`Color`](bevy_color::Color).
    fn to_color(_self: R<Srgba>) -> V<Color> {
        V(Color::Srgba(*_self))
    }

    /// Whether both colors have the same red, green, blue and alpha.
    fn eq(_self: R<Srgba>, other: R<Srgba>) -> bool {
        *_self == *other
    }
}

/// Registers the [`Srgba`] functions.
pub(crate) fn register(world: &mut World) {
    register_srgba_functions(world);
}
