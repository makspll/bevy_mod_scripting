//! Conversions from the other color spaces to [`Color`].

// `script_bindings` expects these in scope
use bevy_color::{Color, Hsla, Hsva, Hwba, Laba, Lcha, Oklaba, Oklcha, Xyza};
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;

/// Adds a `to_color` function to each color space type, each in its own binding block.
// rustfmt keeps indenting the attribute inside the macro further on every run
#[rustfmt::skip]
macro_rules! colorspace_functions {
    ($($ty:ident => $name:literal, $new_doc:literal),* $(,)?) => {
        $(
            #[script_bindings(
                remote,
                name = $name,
                bms_bindings_path = "bevy_mod_scripting_bindings",
                core
            )]
            impl $ty {
                #[doc = $new_doc]
                fn new(a: f32, b: f32, c: f32, alpha: f32) -> V<$ty> {
                    V(<$ty>::new(a, b, c, alpha))
                }

                /// The color as a [`Color`](bevy_color::Color), in the same color space.
                fn to_color(_self: R<$ty>) -> V<Color> {
                    V(Color::$ty(*_self))
                }
            }
        )*
    };
}

colorspace_functions!(
    Hsla => "hsla_functions", "Creates a color from hue (degrees), saturation, lightness and alpha.",
    Hsva => "hsva_functions", "Creates a color from hue (degrees), saturation, value and alpha.",
    Hwba => "hwba_functions", "Creates a color from hue (degrees), whiteness, blackness and alpha.",
    Laba => "laba_functions", "Creates a color from lightness, a, b and alpha.",
    Lcha => "lcha_functions", "Creates a color from lightness, chroma, hue (degrees) and alpha.",
    Oklaba => "oklaba_functions", "Creates a color from lightness, a, b and alpha.",
    Oklcha => "oklcha_functions", "Creates a color from lightness, chroma, hue (degrees) and alpha.",
    Xyza => "xyza_functions", "Creates a color from x, y, z and alpha.",
);

/// Registers the color space functions.
pub(crate) fn register(world: &mut World) {
    register_hsla_functions(world);
    register_hsva_functions(world);
    register_hwba_functions(world);
    register_laba_functions(world);
    register_lcha_functions(world);
    register_oklaba_functions(world);
    register_oklcha_functions(world);
    register_xyza_functions(world);
}
