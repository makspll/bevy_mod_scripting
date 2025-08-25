#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_mod_scripting_core::bindings::{
    ReflectReference,
    function::{
        from::{Mut, Ref, Val},
        namespace::NamespaceBuilder,
    },
};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyColorScriptingPlugin;
pub(crate) fn register_color_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_color::prelude::Color,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_color::prelude::Color>| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = <::bevy_color::prelude::Color as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_color::prelude::Color>,
                other: Ref<::bevy_color::prelude::Color>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_color::prelude::Color as ::core::cmp::PartialEq<
                            ::bevy_color::prelude::Color,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "hsl",
            |hue: f32, saturation: f32, lightness: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::hsl(
                                hue,
                                saturation,
                                lightness,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Hsla`] color with an alpha of 1.0.\n # Arguments\n * `hue` - Hue channel. [0.0, 360.0]\n * `saturation` - Saturation channel. [0.0, 1.0]\n * `lightness` - Lightness channel. [0.0, 1.0]",
            &["hue", "saturation", "lightness"],
        )
        .register_documented(
            "hsla",
            |hue: f32, saturation: f32, lightness: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::hsla(
                                hue,
                                saturation,
                                lightness,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Hsla`] color.\n # Arguments\n * `hue` - Hue channel. [0.0, 360.0]\n * `saturation` - Saturation channel. [0.0, 1.0]\n * `lightness` - Lightness channel. [0.0, 1.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["hue", "saturation", "lightness", "alpha"],
        )
        .register_documented(
            "hsv",
            |hue: f32, saturation: f32, value: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::hsv(
                                hue,
                                saturation,
                                value,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Hsva`] color with an alpha of 1.0.\n # Arguments\n * `hue` - Hue channel. [0.0, 360.0]\n * `saturation` - Saturation channel. [0.0, 1.0]\n * `value` - Value channel. [0.0, 1.0]",
            &["hue", "saturation", "value"],
        )
        .register_documented(
            "hsva",
            |hue: f32, saturation: f32, value: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::hsva(
                                hue,
                                saturation,
                                value,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Hsva`] color.\n # Arguments\n * `hue` - Hue channel. [0.0, 360.0]\n * `saturation` - Saturation channel. [0.0, 1.0]\n * `value` - Value channel. [0.0, 1.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["hue", "saturation", "value", "alpha"],
        )
        .register_documented(
            "hwb",
            |hue: f32, whiteness: f32, blackness: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::hwb(
                                hue,
                                whiteness,
                                blackness,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Hwba`] color with an alpha of 1.0.\n # Arguments\n * `hue` - Hue channel. [0.0, 360.0]\n * `whiteness` - Whiteness channel. [0.0, 1.0]\n * `blackness` - Blackness channel. [0.0, 1.0]",
            &["hue", "whiteness", "blackness"],
        )
        .register_documented(
            "hwba",
            |hue: f32, whiteness: f32, blackness: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::hwba(
                                hue,
                                whiteness,
                                blackness,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Hwba`] color.\n # Arguments\n * `hue` - Hue channel. [0.0, 360.0]\n * `whiteness` - Whiteness channel. [0.0, 1.0]\n * `blackness` - Blackness channel. [0.0, 1.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["hue", "whiteness", "blackness", "alpha"],
        )
        .register_documented(
            "lab",
            |lightness: f32, a: f32, b: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::lab(
                                lightness,
                                a,
                                b,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Laba`] color with an alpha of 1.0.\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.5]\n * `a` - a axis. [-1.5, 1.5]\n * `b` - b axis. [-1.5, 1.5]",
            &["lightness", "a", "b"],
        )
        .register_documented(
            "laba",
            |lightness: f32, a: f32, b: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::laba(
                                lightness,
                                a,
                                b,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Laba`] color.\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.5]\n * `a` - a axis. [-1.5, 1.5]\n * `b` - b axis. [-1.5, 1.5]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["lightness", "a", "b", "alpha"],
        )
        .register_documented(
            "lch",
            |lightness: f32, chroma: f32, hue: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::lch(
                                lightness,
                                chroma,
                                hue,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Lcha`] color with an alpha of 1.0.\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.5]\n * `chroma` - Chroma channel. [0.0, 1.5]\n * `hue` - Hue channel. [0.0, 360.0]",
            &["lightness", "chroma", "hue"],
        )
        .register_documented(
            "lcha",
            |lightness: f32, chroma: f32, hue: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::lcha(
                                lightness,
                                chroma,
                                hue,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Lcha`] color.\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.5]\n * `chroma` - Chroma channel. [0.0, 1.5]\n * `hue` - Hue channel. [0.0, 360.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["lightness", "chroma", "hue", "alpha"],
        )
        .register_documented(
            "linear_rgb",
            |red: f32, green: f32, blue: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::linear_rgb(
                                red,
                                green,
                                blue,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`LinearRgba`] color with an alpha of 1.0.\n # Arguments\n * `red` - Red channel. [0.0, 1.0]\n * `green` - Green channel. [0.0, 1.0]\n * `blue` - Blue channel. [0.0, 1.0]",
            &["red", "green", "blue"],
        )
        .register_documented(
            "linear_rgba",
            |red: f32, green: f32, blue: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::linear_rgba(
                                red,
                                green,
                                blue,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`LinearRgba`] color.\n # Arguments\n * `red` - Red channel. [0.0, 1.0]\n * `green` - Green channel. [0.0, 1.0]\n * `blue` - Blue channel. [0.0, 1.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["red", "green", "blue", "alpha"],
        )
        .register_documented(
            "oklab",
            |lightness: f32, a: f32, b: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::oklab(
                                lightness,
                                a,
                                b,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Oklaba`] color with an alpha of 1.0.\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.0]\n * `a` - Green-red channel. [-1.0, 1.0]\n * `b` - Blue-yellow channel. [-1.0, 1.0]",
            &["lightness", "a", "b"],
        )
        .register_documented(
            "oklaba",
            |lightness: f32, a: f32, b: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::oklaba(
                                lightness,
                                a,
                                b,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Oklaba`] color.\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.0]\n * `a` - Green-red channel. [-1.0, 1.0]\n * `b` - Blue-yellow channel. [-1.0, 1.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["lightness", "a", "b", "alpha"],
        )
        .register_documented(
            "oklch",
            |lightness: f32, chroma: f32, hue: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::oklch(
                                lightness,
                                chroma,
                                hue,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Oklcha`] color with an alpha of 1.0.\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.0]\n * `chroma` - Chroma channel. [0.0, 1.0]\n * `hue` - Hue channel. [0.0, 360.0]",
            &["lightness", "chroma", "hue"],
        )
        .register_documented(
            "oklcha",
            |lightness: f32, chroma: f32, hue: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::oklcha(
                                lightness,
                                chroma,
                                hue,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Oklcha`] color.\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.0]\n * `chroma` - Chroma channel. [0.0, 1.0]\n * `hue` - Hue channel. [0.0, 360.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["lightness", "chroma", "hue", "alpha"],
        )
        .register_documented(
            "srgb",
            |red: f32, green: f32, blue: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::srgb(
                                red,
                                green,
                                blue,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Srgba`] color with an alpha of 1.0.\n # Arguments\n * `red` - Red channel. [0.0, 1.0]\n * `green` - Green channel. [0.0, 1.0]\n * `blue` - Blue channel. [0.0, 1.0]",
            &["red", "green", "blue"],
        )
        .register_documented(
            "srgb_from_array",
            |array: [f32; 3]| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::srgb_from_array(
                                array,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Reads an array of floats to creates a new [`Color`] object storing a [`Srgba`] color with an alpha of 1.0.\n # Arguments\n * `array` - Red, Green and Blue channels. Each channel is in the range [0.0, 1.0]",
            &["array"],
        )
        .register_documented(
            "srgb_u8",
            |red: u8, green: u8, blue: u8| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::srgb_u8(
                                red,
                                green,
                                blue,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Srgba`] color from [`u8`] values with an alpha of 1.0.\n # Arguments\n * `red` - Red channel. [0, 255]\n * `green` - Green channel. [0, 255]\n * `blue` - Blue channel. [0, 255]",
            &["red", "green", "blue"],
        )
        .register_documented(
            "srgba",
            |red: f32, green: f32, blue: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::srgba(
                                red,
                                green,
                                blue,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Srgba`] color.\n # Arguments\n * `red` - Red channel. [0.0, 1.0]\n * `green` - Green channel. [0.0, 1.0]\n * `blue` - Blue channel. [0.0, 1.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["red", "green", "blue", "alpha"],
        )
        .register_documented(
            "srgba_u8",
            |red: u8, green: u8, blue: u8, alpha: u8| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::srgba_u8(
                                red,
                                green,
                                blue,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Srgba`] color from [`u8`] values.\n # Arguments\n * `red` - Red channel. [0, 255]\n * `green` - Green channel. [0, 255]\n * `blue` - Blue channel. [0, 255]\n * `alpha` - Alpha channel. [0, 255]",
            &["red", "green", "blue", "alpha"],
        )
        .register_documented(
            "to_linear",
            |_self: Ref<::bevy_color::prelude::Color>| {
                let output: Val<::bevy_color::prelude::LinearRgba> = {
                    {
                        let output: Val<::bevy_color::prelude::LinearRgba> = ::bevy_color::prelude::Color::to_linear(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return the color as a linear RGBA color.",
            &["_self"],
        )
        .register_documented(
            "to_srgba",
            |_self: Ref<::bevy_color::prelude::Color>| {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = ::bevy_color::prelude::Color::to_srgba(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return the color as an SRGBA color.",
            &["_self"],
        )
        .register_documented(
            "xyz",
            |x: f32, y: f32, z: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::xyz(
                                x,
                                y,
                                z,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Xyza`] color with an alpha of 1.0.\n # Arguments\n * `x` - x-axis. [0.0, 1.0]\n * `y` - y-axis. [0.0, 1.0]\n * `z` - z-axis. [0.0, 1.0]",
            &["x", "y", "z"],
        )
        .register_documented(
            "xyza",
            |x: f32, y: f32, z: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Color> = {
                    {
                        let output: Val<::bevy_color::prelude::Color> = ::bevy_color::prelude::Color::xyza(
                                x,
                                y,
                                z,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Color`] object storing a [`Xyza`] color.\n # Arguments\n * `x` - x-axis. [0.0, 1.0]\n * `y` - y-axis. [0.0, 1.0]\n * `z` - z-axis. [0.0, 1.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["x", "y", "z", "alpha"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_color::prelude::Color,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_srgba_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_color::prelude::Srgba,
    >::new(world)
        .register_documented(
            "add",
            |
                _self: Val<::bevy_color::prelude::Srgba>,
                rhs: Val<::bevy_color::prelude::Srgba>|
            {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = <::bevy_color::prelude::Srgba as ::core::ops::Add<
                            ::bevy_color::prelude::Srgba,
                        >>::add(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_color::prelude::Srgba>| {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = <::bevy_color::prelude::Srgba as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "div",
            |_self: Val<::bevy_color::prelude::Srgba>, rhs: f32| {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = <::bevy_color::prelude::Srgba as ::core::ops::Div<
                            f32,
                        >>::div(_self.into_inner(), rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_color::prelude::Srgba>,
                other: Ref<::bevy_color::prelude::Srgba>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_color::prelude::Srgba as ::core::cmp::PartialEq<
                            ::bevy_color::prelude::Srgba,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "gamma_function",
            |value: f32| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_color::prelude::Srgba::gamma_function(
                                value,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Converts a non-linear sRGB value to a linear one via [gamma correction](https://en.wikipedia.org/wiki/Gamma_correction).",
            &["value"],
        )
        .register_documented(
            "gamma_function_inverse",
            |value: f32| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_color::prelude::Srgba::gamma_function_inverse(
                                value,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Converts a linear sRGB value to a non-linear one via [gamma correction](https://en.wikipedia.org/wiki/Gamma_correction).",
            &["value"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_color::prelude::Srgba>, rhs: f32| {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = <::bevy_color::prelude::Srgba as ::core::ops::Mul<
                            f32,
                        >>::mul(_self.into_inner(), rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "neg",
            |_self: Val<::bevy_color::prelude::Srgba>| {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = <::bevy_color::prelude::Srgba as ::core::ops::Neg>::neg(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "new",
            |red: f32, green: f32, blue: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = ::bevy_color::prelude::Srgba::new(
                                red,
                                green,
                                blue,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Srgba`] color from components.\n # Arguments\n * `red` - Red channel. [0.0, 1.0]\n * `green` - Green channel. [0.0, 1.0]\n * `blue` - Blue channel. [0.0, 1.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["red", "green", "blue", "alpha"],
        )
        .register_documented(
            "rgb",
            |red: f32, green: f32, blue: f32| {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = ::bevy_color::prelude::Srgba::rgb(
                                red,
                                green,
                                blue,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Srgba`] color from (r, g, b) components, with the default alpha (1.0).\n # Arguments\n * `red` - Red channel. [0.0, 1.0]\n * `green` - Green channel. [0.0, 1.0]\n * `blue` - Blue channel. [0.0, 1.0]",
            &["red", "green", "blue"],
        )
        .register_documented(
            "rgb_u8",
            |r: u8, g: u8, b: u8| {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = ::bevy_color::prelude::Srgba::rgb_u8(
                                r,
                                g,
                                b,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " New `Srgba` from sRGB colorspace.\n # Arguments\n * `r` - Red channel. [0, 255]\n * `g` - Green channel. [0, 255]\n * `b` - Blue channel. [0, 255]\n See also [`Srgba::new`], [`Srgba::rgba_u8`], [`Srgba::hex`].",
            &["r", "g", "b"],
        )
        .register_documented(
            "rgba_u8",
            |r: u8, g: u8, b: u8, a: u8| {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = ::bevy_color::prelude::Srgba::rgba_u8(
                                r,
                                g,
                                b,
                                a,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " New `Srgba` from sRGB colorspace.\n # Arguments\n * `r` - Red channel. [0, 255]\n * `g` - Green channel. [0, 255]\n * `b` - Blue channel. [0, 255]\n * `a` - Alpha channel. [0, 255]\n See also [`Srgba::new`], [`Srgba::rgb_u8`], [`Srgba::hex`].",
            &["r", "g", "b", "a"],
        )
        .register_documented(
            "sub",
            |
                _self: Val<::bevy_color::prelude::Srgba>,
                rhs: Val<::bevy_color::prelude::Srgba>|
            {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = <::bevy_color::prelude::Srgba as ::core::ops::Sub<
                            ::bevy_color::prelude::Srgba,
                        >>::sub(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "to_hex",
            |_self: Ref<::bevy_color::prelude::Srgba>| {
                let output: ::std::string::String = {
                    {
                        let output: ::std::string::String = ::bevy_color::prelude::Srgba::to_hex(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Convert this color to CSS-style hexadecimal notation.",
            &["_self"],
        )
        .register_documented(
            "with_blue",
            |_self: Val<::bevy_color::prelude::Srgba>, blue: f32| {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = ::bevy_color::prelude::Srgba::with_blue(
                                _self.into_inner(),
                                blue,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the blue channel set to the given value.",
            &["_self", "blue"],
        )
        .register_documented(
            "with_green",
            |_self: Val<::bevy_color::prelude::Srgba>, green: f32| {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = ::bevy_color::prelude::Srgba::with_green(
                                _self.into_inner(),
                                green,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the green channel set to the given value.",
            &["_self", "green"],
        )
        .register_documented(
            "with_red",
            |_self: Val<::bevy_color::prelude::Srgba>, red: f32| {
                let output: Val<::bevy_color::prelude::Srgba> = {
                    {
                        let output: Val<::bevy_color::prelude::Srgba> = ::bevy_color::prelude::Srgba::with_red(
                                _self.into_inner(),
                                red,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the red channel set to the given value.",
            &["_self", "red"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_color::prelude::Srgba,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_linear_rgba_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_color::prelude::LinearRgba,
    >::new(world)
        .register_documented(
            "add",
            |
                _self: Val<::bevy_color::prelude::LinearRgba>,
                rhs: Val<::bevy_color::prelude::LinearRgba>|
            {
                let output: Val<::bevy_color::prelude::LinearRgba> = {
                    {
                        let output: Val<::bevy_color::prelude::LinearRgba> = <::bevy_color::prelude::LinearRgba as ::core::ops::Add<
                            ::bevy_color::prelude::LinearRgba,
                        >>::add(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "as_u32",
            |_self: Ref<::bevy_color::prelude::LinearRgba>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_color::prelude::LinearRgba::as_u32(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Converts this color to a u32.\n Maps the RGBA channels in RGBA order to a little-endian byte array (GPUs are little-endian).\n `A` will be the most significant byte and `R` the least significant.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_color::prelude::LinearRgba>| {
                let output: Val<::bevy_color::prelude::LinearRgba> = {
                    {
                        let output: Val<::bevy_color::prelude::LinearRgba> = <::bevy_color::prelude::LinearRgba as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "div",
            |_self: Val<::bevy_color::prelude::LinearRgba>, rhs: f32| {
                let output: Val<::bevy_color::prelude::LinearRgba> = {
                    {
                        let output: Val<::bevy_color::prelude::LinearRgba> = <::bevy_color::prelude::LinearRgba as ::core::ops::Div<
                            f32,
                        >>::div(_self.into_inner(), rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_color::prelude::LinearRgba>,
                other: Ref<::bevy_color::prelude::LinearRgba>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_color::prelude::LinearRgba as ::core::cmp::PartialEq<
                            ::bevy_color::prelude::LinearRgba,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_color::prelude::LinearRgba>, rhs: f32| {
                let output: Val<::bevy_color::prelude::LinearRgba> = {
                    {
                        let output: Val<::bevy_color::prelude::LinearRgba> = <::bevy_color::prelude::LinearRgba as ::core::ops::Mul<
                            f32,
                        >>::mul(_self.into_inner(), rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "neg",
            |_self: Val<::bevy_color::prelude::LinearRgba>| {
                let output: Val<::bevy_color::prelude::LinearRgba> = {
                    {
                        let output: Val<::bevy_color::prelude::LinearRgba> = <::bevy_color::prelude::LinearRgba as ::core::ops::Neg>::neg(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "new",
            |red: f32, green: f32, blue: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::LinearRgba> = {
                    {
                        let output: Val<::bevy_color::prelude::LinearRgba> = ::bevy_color::prelude::LinearRgba::new(
                                red,
                                green,
                                blue,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`LinearRgba`] color from components.",
            &["red", "green", "blue", "alpha"],
        )
        .register_documented(
            "rgb",
            |red: f32, green: f32, blue: f32| {
                let output: Val<::bevy_color::prelude::LinearRgba> = {
                    {
                        let output: Val<::bevy_color::prelude::LinearRgba> = ::bevy_color::prelude::LinearRgba::rgb(
                                red,
                                green,
                                blue,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`LinearRgba`] color from (r, g, b) components, with the default alpha (1.0).\n # Arguments\n * `red` - Red channel. [0.0, 1.0]\n * `green` - Green channel. [0.0, 1.0]\n * `blue` - Blue channel. [0.0, 1.0]",
            &["red", "green", "blue"],
        )
        .register_documented(
            "sub",
            |
                _self: Val<::bevy_color::prelude::LinearRgba>,
                rhs: Val<::bevy_color::prelude::LinearRgba>|
            {
                let output: Val<::bevy_color::prelude::LinearRgba> = {
                    {
                        let output: Val<::bevy_color::prelude::LinearRgba> = <::bevy_color::prelude::LinearRgba as ::core::ops::Sub<
                            ::bevy_color::prelude::LinearRgba,
                        >>::sub(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "with_blue",
            |_self: Val<::bevy_color::prelude::LinearRgba>, blue: f32| {
                let output: Val<::bevy_color::prelude::LinearRgba> = {
                    {
                        let output: Val<::bevy_color::prelude::LinearRgba> = ::bevy_color::prelude::LinearRgba::with_blue(
                                _self.into_inner(),
                                blue,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the blue channel set to the given value.",
            &["_self", "blue"],
        )
        .register_documented(
            "with_green",
            |_self: Val<::bevy_color::prelude::LinearRgba>, green: f32| {
                let output: Val<::bevy_color::prelude::LinearRgba> = {
                    {
                        let output: Val<::bevy_color::prelude::LinearRgba> = ::bevy_color::prelude::LinearRgba::with_green(
                                _self.into_inner(),
                                green,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the green channel set to the given value.",
            &["_self", "green"],
        )
        .register_documented(
            "with_red",
            |_self: Val<::bevy_color::prelude::LinearRgba>, red: f32| {
                let output: Val<::bevy_color::prelude::LinearRgba> = {
                    {
                        let output: Val<::bevy_color::prelude::LinearRgba> = ::bevy_color::prelude::LinearRgba::with_red(
                                _self.into_inner(),
                                red,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the red channel set to the given value.",
            &["_self", "red"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_color::prelude::LinearRgba,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_hsla_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_color::prelude::Hsla,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_color::prelude::Hsla>| {
                let output: Val<::bevy_color::prelude::Hsla> = {
                    {
                        let output: Val<::bevy_color::prelude::Hsla> = <::bevy_color::prelude::Hsla as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_color::prelude::Hsla>,
                other: Ref<::bevy_color::prelude::Hsla>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_color::prelude::Hsla as ::core::cmp::PartialEq<
                            ::bevy_color::prelude::Hsla,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "hsl",
            |hue: f32, saturation: f32, lightness: f32| {
                let output: Val<::bevy_color::prelude::Hsla> = {
                    {
                        let output: Val<::bevy_color::prelude::Hsla> = ::bevy_color::prelude::Hsla::hsl(
                                hue,
                                saturation,
                                lightness,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Hsla`] color from (h, s, l) components, with the default alpha (1.0).\n # Arguments\n * `hue` - Hue channel. [0.0, 360.0]\n * `saturation` - Saturation channel. [0.0, 1.0]\n * `lightness` - Lightness channel. [0.0, 1.0]",
            &["hue", "saturation", "lightness"],
        )
        .register_documented(
            "new",
            |hue: f32, saturation: f32, lightness: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Hsla> = {
                    {
                        let output: Val<::bevy_color::prelude::Hsla> = ::bevy_color::prelude::Hsla::new(
                                hue,
                                saturation,
                                lightness,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Hsla`] color from components.\n # Arguments\n * `hue` - Hue channel. [0.0, 360.0]\n * `saturation` - Saturation channel. [0.0, 1.0]\n * `lightness` - Lightness channel. [0.0, 1.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["hue", "saturation", "lightness", "alpha"],
        )
        .register_documented(
            "sequential_dispersed",
            |index: u32| {
                let output: Val<::bevy_color::prelude::Hsla> = {
                    {
                        let output: Val<::bevy_color::prelude::Hsla> = ::bevy_color::prelude::Hsla::sequential_dispersed(
                                index,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Generate a deterministic but [quasi-randomly distributed](https://en.wikipedia.org/wiki/Low-discrepancy_sequence)\n color from a provided `index`.\n This can be helpful for generating debug colors.\n # Examples\n ```rust\n # use bevy_color::Hsla;\n // Unique color for an entity\n # let entity_index = 123;\n // let entity_index = entity.index();\n let color = Hsla::sequential_dispersed(entity_index);\n // Palette with 5 distinct hues\n let palette = (0..5).map(Hsla::sequential_dispersed).collect::<Vec<_>>();\n ```",
            &["index"],
        )
        .register_documented(
            "with_lightness",
            |_self: Val<::bevy_color::prelude::Hsla>, lightness: f32| {
                let output: Val<::bevy_color::prelude::Hsla> = {
                    {
                        let output: Val<::bevy_color::prelude::Hsla> = ::bevy_color::prelude::Hsla::with_lightness(
                                _self.into_inner(),
                                lightness,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the lightness channel set to the given value.",
            &["_self", "lightness"],
        )
        .register_documented(
            "with_saturation",
            |_self: Val<::bevy_color::prelude::Hsla>, saturation: f32| {
                let output: Val<::bevy_color::prelude::Hsla> = {
                    {
                        let output: Val<::bevy_color::prelude::Hsla> = ::bevy_color::prelude::Hsla::with_saturation(
                                _self.into_inner(),
                                saturation,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the saturation channel set to the given value.",
            &["_self", "saturation"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_color::prelude::Hsla,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_hsva_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_color::prelude::Hsva,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_color::prelude::Hsva>| {
                let output: Val<::bevy_color::prelude::Hsva> = {
                    {
                        let output: Val<::bevy_color::prelude::Hsva> = <::bevy_color::prelude::Hsva as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_color::prelude::Hsva>,
                other: Ref<::bevy_color::prelude::Hsva>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_color::prelude::Hsva as ::core::cmp::PartialEq<
                            ::bevy_color::prelude::Hsva,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "hsv",
            |hue: f32, saturation: f32, value: f32| {
                let output: Val<::bevy_color::prelude::Hsva> = {
                    {
                        let output: Val<::bevy_color::prelude::Hsva> = ::bevy_color::prelude::Hsva::hsv(
                                hue,
                                saturation,
                                value,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Hsva`] color from (h, s, v) components, with the default alpha (1.0).\n # Arguments\n * `hue` - Hue channel. [0.0, 360.0]\n * `saturation` - Saturation channel. [0.0, 1.0]\n * `value` - Value channel. [0.0, 1.0]",
            &["hue", "saturation", "value"],
        )
        .register_documented(
            "new",
            |hue: f32, saturation: f32, value: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Hsva> = {
                    {
                        let output: Val<::bevy_color::prelude::Hsva> = ::bevy_color::prelude::Hsva::new(
                                hue,
                                saturation,
                                value,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Hsva`] color from components.\n # Arguments\n * `hue` - Hue channel. [0.0, 360.0]\n * `saturation` - Saturation channel. [0.0, 1.0]\n * `value` - Value channel. [0.0, 1.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["hue", "saturation", "value", "alpha"],
        )
        .register_documented(
            "with_saturation",
            |_self: Val<::bevy_color::prelude::Hsva>, saturation: f32| {
                let output: Val<::bevy_color::prelude::Hsva> = {
                    {
                        let output: Val<::bevy_color::prelude::Hsva> = ::bevy_color::prelude::Hsva::with_saturation(
                                _self.into_inner(),
                                saturation,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the saturation channel set to the given value.",
            &["_self", "saturation"],
        )
        .register_documented(
            "with_value",
            |_self: Val<::bevy_color::prelude::Hsva>, value: f32| {
                let output: Val<::bevy_color::prelude::Hsva> = {
                    {
                        let output: Val<::bevy_color::prelude::Hsva> = ::bevy_color::prelude::Hsva::with_value(
                                _self.into_inner(),
                                value,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the value channel set to the given value.",
            &["_self", "value"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_color::prelude::Hsva,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_hwba_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_color::prelude::Hwba,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_color::prelude::Hwba>| {
                let output: Val<::bevy_color::prelude::Hwba> = {
                    {
                        let output: Val<::bevy_color::prelude::Hwba> = <::bevy_color::prelude::Hwba as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_color::prelude::Hwba>,
                other: Ref<::bevy_color::prelude::Hwba>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_color::prelude::Hwba as ::core::cmp::PartialEq<
                            ::bevy_color::prelude::Hwba,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "hwb",
            |hue: f32, whiteness: f32, blackness: f32| {
                let output: Val<::bevy_color::prelude::Hwba> = {
                    {
                        let output: Val<::bevy_color::prelude::Hwba> = ::bevy_color::prelude::Hwba::hwb(
                                hue,
                                whiteness,
                                blackness,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Hwba`] color from (h, s, l) components, with the default alpha (1.0).\n # Arguments\n * `hue` - Hue channel. [0.0, 360.0]\n * `whiteness` - Whiteness channel. [0.0, 1.0]\n * `blackness` - Blackness channel. [0.0, 1.0]",
            &["hue", "whiteness", "blackness"],
        )
        .register_documented(
            "new",
            |hue: f32, whiteness: f32, blackness: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Hwba> = {
                    {
                        let output: Val<::bevy_color::prelude::Hwba> = ::bevy_color::prelude::Hwba::new(
                                hue,
                                whiteness,
                                blackness,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Hwba`] color from components.\n # Arguments\n * `hue` - Hue channel. [0.0, 360.0]\n * `whiteness` - Whiteness channel. [0.0, 1.0]\n * `blackness` - Blackness channel. [0.0, 1.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["hue", "whiteness", "blackness", "alpha"],
        )
        .register_documented(
            "with_blackness",
            |_self: Val<::bevy_color::prelude::Hwba>, blackness: f32| {
                let output: Val<::bevy_color::prelude::Hwba> = {
                    {
                        let output: Val<::bevy_color::prelude::Hwba> = ::bevy_color::prelude::Hwba::with_blackness(
                                _self.into_inner(),
                                blackness,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the blackness channel set to the given value.",
            &["_self", "blackness"],
        )
        .register_documented(
            "with_whiteness",
            |_self: Val<::bevy_color::prelude::Hwba>, whiteness: f32| {
                let output: Val<::bevy_color::prelude::Hwba> = {
                    {
                        let output: Val<::bevy_color::prelude::Hwba> = ::bevy_color::prelude::Hwba::with_whiteness(
                                _self.into_inner(),
                                whiteness,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the whiteness channel set to the given value.",
            &["_self", "whiteness"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_color::prelude::Hwba,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_laba_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_color::prelude::Laba,
    >::new(world)
        .register_documented(
            "add",
            |
                _self: Val<::bevy_color::prelude::Laba>,
                rhs: Val<::bevy_color::prelude::Laba>|
            {
                let output: Val<::bevy_color::prelude::Laba> = {
                    {
                        let output: Val<::bevy_color::prelude::Laba> = <::bevy_color::prelude::Laba as ::core::ops::Add<
                            ::bevy_color::prelude::Laba,
                        >>::add(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_color::prelude::Laba>| {
                let output: Val<::bevy_color::prelude::Laba> = {
                    {
                        let output: Val<::bevy_color::prelude::Laba> = <::bevy_color::prelude::Laba as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "div",
            |_self: Val<::bevy_color::prelude::Laba>, rhs: f32| {
                let output: Val<::bevy_color::prelude::Laba> = {
                    {
                        let output: Val<::bevy_color::prelude::Laba> = <::bevy_color::prelude::Laba as ::core::ops::Div<
                            f32,
                        >>::div(_self.into_inner(), rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_color::prelude::Laba>,
                other: Ref<::bevy_color::prelude::Laba>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_color::prelude::Laba as ::core::cmp::PartialEq<
                            ::bevy_color::prelude::Laba,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "lab",
            |lightness: f32, a: f32, b: f32| {
                let output: Val<::bevy_color::prelude::Laba> = {
                    {
                        let output: Val<::bevy_color::prelude::Laba> = ::bevy_color::prelude::Laba::lab(
                                lightness,
                                a,
                                b,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Laba`] color from (l, a, b) components, with the default alpha (1.0).\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.5]\n * `a` - a axis. [-1.5, 1.5]\n * `b` - b axis. [-1.5, 1.5]",
            &["lightness", "a", "b"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_color::prelude::Laba>, rhs: f32| {
                let output: Val<::bevy_color::prelude::Laba> = {
                    {
                        let output: Val<::bevy_color::prelude::Laba> = <::bevy_color::prelude::Laba as ::core::ops::Mul<
                            f32,
                        >>::mul(_self.into_inner(), rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "neg",
            |_self: Val<::bevy_color::prelude::Laba>| {
                let output: Val<::bevy_color::prelude::Laba> = {
                    {
                        let output: Val<::bevy_color::prelude::Laba> = <::bevy_color::prelude::Laba as ::core::ops::Neg>::neg(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "new",
            |lightness: f32, a: f32, b: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Laba> = {
                    {
                        let output: Val<::bevy_color::prelude::Laba> = ::bevy_color::prelude::Laba::new(
                                lightness,
                                a,
                                b,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Laba`] color from components.\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.5]\n * `a` - a axis. [-1.5, 1.5]\n * `b` - b axis. [-1.5, 1.5]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["lightness", "a", "b", "alpha"],
        )
        .register_documented(
            "sub",
            |
                _self: Val<::bevy_color::prelude::Laba>,
                rhs: Val<::bevy_color::prelude::Laba>|
            {
                let output: Val<::bevy_color::prelude::Laba> = {
                    {
                        let output: Val<::bevy_color::prelude::Laba> = <::bevy_color::prelude::Laba as ::core::ops::Sub<
                            ::bevy_color::prelude::Laba,
                        >>::sub(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "with_lightness",
            |_self: Val<::bevy_color::prelude::Laba>, lightness: f32| {
                let output: Val<::bevy_color::prelude::Laba> = {
                    {
                        let output: Val<::bevy_color::prelude::Laba> = ::bevy_color::prelude::Laba::with_lightness(
                                _self.into_inner(),
                                lightness,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the lightness channel set to the given value.",
            &["_self", "lightness"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_color::prelude::Laba,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_lcha_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_color::prelude::Lcha,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_color::prelude::Lcha>| {
                let output: Val<::bevy_color::prelude::Lcha> = {
                    {
                        let output: Val<::bevy_color::prelude::Lcha> = <::bevy_color::prelude::Lcha as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_color::prelude::Lcha>,
                other: Ref<::bevy_color::prelude::Lcha>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_color::prelude::Lcha as ::core::cmp::PartialEq<
                            ::bevy_color::prelude::Lcha,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "lch",
            |lightness: f32, chroma: f32, hue: f32| {
                let output: Val<::bevy_color::prelude::Lcha> = {
                    {
                        let output: Val<::bevy_color::prelude::Lcha> = ::bevy_color::prelude::Lcha::lch(
                                lightness,
                                chroma,
                                hue,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Lcha`] color from (h, s, l) components, with the default alpha (1.0).\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.5]\n * `chroma` - Chroma channel. [0.0, 1.5]\n * `hue` - Hue channel. [0.0, 360.0]",
            &["lightness", "chroma", "hue"],
        )
        .register_documented(
            "new",
            |lightness: f32, chroma: f32, hue: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Lcha> = {
                    {
                        let output: Val<::bevy_color::prelude::Lcha> = ::bevy_color::prelude::Lcha::new(
                                lightness,
                                chroma,
                                hue,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Lcha`] color from components.\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.5]\n * `chroma` - Chroma channel. [0.0, 1.5]\n * `hue` - Hue channel. [0.0, 360.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["lightness", "chroma", "hue", "alpha"],
        )
        .register_documented(
            "sequential_dispersed",
            |index: u32| {
                let output: Val<::bevy_color::prelude::Lcha> = {
                    {
                        let output: Val<::bevy_color::prelude::Lcha> = ::bevy_color::prelude::Lcha::sequential_dispersed(
                                index,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Generate a deterministic but [quasi-randomly distributed](https://en.wikipedia.org/wiki/Low-discrepancy_sequence)\n color from a provided `index`.\n This can be helpful for generating debug colors.\n # Examples\n ```rust\n # use bevy_color::Lcha;\n // Unique color for an entity\n # let entity_index = 123;\n // let entity_index = entity.index();\n let color = Lcha::sequential_dispersed(entity_index);\n // Palette with 5 distinct hues\n let palette = (0..5).map(Lcha::sequential_dispersed).collect::<Vec<_>>();\n ```",
            &["index"],
        )
        .register_documented(
            "with_chroma",
            |_self: Val<::bevy_color::prelude::Lcha>, chroma: f32| {
                let output: Val<::bevy_color::prelude::Lcha> = {
                    {
                        let output: Val<::bevy_color::prelude::Lcha> = ::bevy_color::prelude::Lcha::with_chroma(
                                _self.into_inner(),
                                chroma,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the chroma channel set to the given value.",
            &["_self", "chroma"],
        )
        .register_documented(
            "with_lightness",
            |_self: Val<::bevy_color::prelude::Lcha>, lightness: f32| {
                let output: Val<::bevy_color::prelude::Lcha> = {
                    {
                        let output: Val<::bevy_color::prelude::Lcha> = ::bevy_color::prelude::Lcha::with_lightness(
                                _self.into_inner(),
                                lightness,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the lightness channel set to the given value.",
            &["_self", "lightness"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_color::prelude::Lcha,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_oklaba_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_color::prelude::Oklaba,
    >::new(world)
        .register_documented(
            "add",
            |
                _self: Val<::bevy_color::prelude::Oklaba>,
                rhs: Val<::bevy_color::prelude::Oklaba>|
            {
                let output: Val<::bevy_color::prelude::Oklaba> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklaba> = <::bevy_color::prelude::Oklaba as ::core::ops::Add<
                            ::bevy_color::prelude::Oklaba,
                        >>::add(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_color::prelude::Oklaba>| {
                let output: Val<::bevy_color::prelude::Oklaba> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklaba> = <::bevy_color::prelude::Oklaba as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "div",
            |_self: Val<::bevy_color::prelude::Oklaba>, rhs: f32| {
                let output: Val<::bevy_color::prelude::Oklaba> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklaba> = <::bevy_color::prelude::Oklaba as ::core::ops::Div<
                            f32,
                        >>::div(_self.into_inner(), rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_color::prelude::Oklaba>,
                other: Ref<::bevy_color::prelude::Oklaba>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_color::prelude::Oklaba as ::core::cmp::PartialEq<
                            ::bevy_color::prelude::Oklaba,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "lab",
            |lightness: f32, a: f32, b: f32| {
                let output: Val<::bevy_color::prelude::Oklaba> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklaba> = ::bevy_color::prelude::Oklaba::lab(
                                lightness,
                                a,
                                b,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Oklaba`] color from (l, a, b) components, with the default alpha (1.0).\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.0]\n * `a` - Green-red channel. [-1.0, 1.0]\n * `b` - Blue-yellow channel. [-1.0, 1.0]",
            &["lightness", "a", "b"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_color::prelude::Oklaba>, rhs: f32| {
                let output: Val<::bevy_color::prelude::Oklaba> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklaba> = <::bevy_color::prelude::Oklaba as ::core::ops::Mul<
                            f32,
                        >>::mul(_self.into_inner(), rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "neg",
            |_self: Val<::bevy_color::prelude::Oklaba>| {
                let output: Val<::bevy_color::prelude::Oklaba> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklaba> = <::bevy_color::prelude::Oklaba as ::core::ops::Neg>::neg(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "new",
            |lightness: f32, a: f32, b: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Oklaba> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklaba> = ::bevy_color::prelude::Oklaba::new(
                                lightness,
                                a,
                                b,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Oklaba`] color from components.\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.0]\n * `a` - Green-red channel. [-1.0, 1.0]\n * `b` - Blue-yellow channel. [-1.0, 1.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["lightness", "a", "b", "alpha"],
        )
        .register_documented(
            "sub",
            |
                _self: Val<::bevy_color::prelude::Oklaba>,
                rhs: Val<::bevy_color::prelude::Oklaba>|
            {
                let output: Val<::bevy_color::prelude::Oklaba> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklaba> = <::bevy_color::prelude::Oklaba as ::core::ops::Sub<
                            ::bevy_color::prelude::Oklaba,
                        >>::sub(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "with_a",
            |_self: Val<::bevy_color::prelude::Oklaba>, a: f32| {
                let output: Val<::bevy_color::prelude::Oklaba> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklaba> = ::bevy_color::prelude::Oklaba::with_a(
                                _self.into_inner(),
                                a,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the 'a' channel set to the given value.",
            &["_self", "a"],
        )
        .register_documented(
            "with_b",
            |_self: Val<::bevy_color::prelude::Oklaba>, b: f32| {
                let output: Val<::bevy_color::prelude::Oklaba> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklaba> = ::bevy_color::prelude::Oklaba::with_b(
                                _self.into_inner(),
                                b,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the 'b' channel set to the given value.",
            &["_self", "b"],
        )
        .register_documented(
            "with_lightness",
            |_self: Val<::bevy_color::prelude::Oklaba>, lightness: f32| {
                let output: Val<::bevy_color::prelude::Oklaba> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklaba> = ::bevy_color::prelude::Oklaba::with_lightness(
                                _self.into_inner(),
                                lightness,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the 'lightness' channel set to the given value.",
            &["_self", "lightness"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_color::prelude::Oklaba,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_oklcha_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_color::prelude::Oklcha,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_color::prelude::Oklcha>| {
                let output: Val<::bevy_color::prelude::Oklcha> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklcha> = <::bevy_color::prelude::Oklcha as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_color::prelude::Oklcha>,
                other: Ref<::bevy_color::prelude::Oklcha>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_color::prelude::Oklcha as ::core::cmp::PartialEq<
                            ::bevy_color::prelude::Oklcha,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "lch",
            |lightness: f32, chroma: f32, hue: f32| {
                let output: Val<::bevy_color::prelude::Oklcha> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklcha> = ::bevy_color::prelude::Oklcha::lch(
                                lightness,
                                chroma,
                                hue,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Oklcha`] color from (l, c, h) components, with the default alpha (1.0).\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.0]\n * `chroma` - Chroma channel. [0.0, 1.0]\n * `hue` - Hue channel. [0.0, 360.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["lightness", "chroma", "hue"],
        )
        .register_documented(
            "new",
            |lightness: f32, chroma: f32, hue: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Oklcha> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklcha> = ::bevy_color::prelude::Oklcha::new(
                                lightness,
                                chroma,
                                hue,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Oklcha`] color from components.\n # Arguments\n * `lightness` - Lightness channel. [0.0, 1.0]\n * `chroma` - Chroma channel. [0.0, 1.0]\n * `hue` - Hue channel. [0.0, 360.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["lightness", "chroma", "hue", "alpha"],
        )
        .register_documented(
            "sequential_dispersed",
            |index: u32| {
                let output: Val<::bevy_color::prelude::Oklcha> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklcha> = ::bevy_color::prelude::Oklcha::sequential_dispersed(
                                index,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Generate a deterministic but [quasi-randomly distributed](https://en.wikipedia.org/wiki/Low-discrepancy_sequence)\n color from a provided `index`.\n This can be helpful for generating debug colors.\n # Examples\n ```rust\n # use bevy_color::Oklcha;\n // Unique color for an entity\n # let entity_index = 123;\n // let entity_index = entity.index();\n let color = Oklcha::sequential_dispersed(entity_index);\n // Palette with 5 distinct hues\n let palette = (0..5).map(Oklcha::sequential_dispersed).collect::<Vec<_>>();\n ```",
            &["index"],
        )
        .register_documented(
            "with_chroma",
            |_self: Val<::bevy_color::prelude::Oklcha>, chroma: f32| {
                let output: Val<::bevy_color::prelude::Oklcha> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklcha> = ::bevy_color::prelude::Oklcha::with_chroma(
                                _self.into_inner(),
                                chroma,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the 'chroma' channel set to the given value.",
            &["_self", "chroma"],
        )
        .register_documented(
            "with_lightness",
            |_self: Val<::bevy_color::prelude::Oklcha>, lightness: f32| {
                let output: Val<::bevy_color::prelude::Oklcha> = {
                    {
                        let output: Val<::bevy_color::prelude::Oklcha> = ::bevy_color::prelude::Oklcha::with_lightness(
                                _self.into_inner(),
                                lightness,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the 'lightness' channel set to the given value.",
            &["_self", "lightness"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_color::prelude::Oklcha,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_xyza_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_color::prelude::Xyza,
    >::new(world)
        .register_documented(
            "add",
            |
                _self: Val<::bevy_color::prelude::Xyza>,
                rhs: Val<::bevy_color::prelude::Xyza>|
            {
                let output: Val<::bevy_color::prelude::Xyza> = {
                    {
                        let output: Val<::bevy_color::prelude::Xyza> = <::bevy_color::prelude::Xyza as ::core::ops::Add<
                            ::bevy_color::prelude::Xyza,
                        >>::add(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_color::prelude::Xyza>| {
                let output: Val<::bevy_color::prelude::Xyza> = {
                    {
                        let output: Val<::bevy_color::prelude::Xyza> = <::bevy_color::prelude::Xyza as ::core::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "div",
            |_self: Val<::bevy_color::prelude::Xyza>, rhs: f32| {
                let output: Val<::bevy_color::prelude::Xyza> = {
                    {
                        let output: Val<::bevy_color::prelude::Xyza> = <::bevy_color::prelude::Xyza as ::core::ops::Div<
                            f32,
                        >>::div(_self.into_inner(), rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_color::prelude::Xyza>,
                other: Ref<::bevy_color::prelude::Xyza>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_color::prelude::Xyza as ::core::cmp::PartialEq<
                            ::bevy_color::prelude::Xyza,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_color::prelude::Xyza>, rhs: f32| {
                let output: Val<::bevy_color::prelude::Xyza> = {
                    {
                        let output: Val<::bevy_color::prelude::Xyza> = <::bevy_color::prelude::Xyza as ::core::ops::Mul<
                            f32,
                        >>::mul(_self.into_inner(), rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "neg",
            |_self: Val<::bevy_color::prelude::Xyza>| {
                let output: Val<::bevy_color::prelude::Xyza> = {
                    {
                        let output: Val<::bevy_color::prelude::Xyza> = <::bevy_color::prelude::Xyza as ::core::ops::Neg>::neg(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "new",
            |x: f32, y: f32, z: f32, alpha: f32| {
                let output: Val<::bevy_color::prelude::Xyza> = {
                    {
                        let output: Val<::bevy_color::prelude::Xyza> = ::bevy_color::prelude::Xyza::new(
                                x,
                                y,
                                z,
                                alpha,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Xyza`] color from components.\n # Arguments\n * `x` - x-axis. [0.0, 1.0]\n * `y` - y-axis. [0.0, 1.0]\n * `z` - z-axis. [0.0, 1.0]\n * `alpha` - Alpha channel. [0.0, 1.0]",
            &["x", "y", "z", "alpha"],
        )
        .register_documented(
            "sub",
            |
                _self: Val<::bevy_color::prelude::Xyza>,
                rhs: Val<::bevy_color::prelude::Xyza>|
            {
                let output: Val<::bevy_color::prelude::Xyza> = {
                    {
                        let output: Val<::bevy_color::prelude::Xyza> = <::bevy_color::prelude::Xyza as ::core::ops::Sub<
                            ::bevy_color::prelude::Xyza,
                        >>::sub(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "with_x",
            |_self: Val<::bevy_color::prelude::Xyza>, x: f32| {
                let output: Val<::bevy_color::prelude::Xyza> = {
                    {
                        let output: Val<::bevy_color::prelude::Xyza> = ::bevy_color::prelude::Xyza::with_x(
                                _self.into_inner(),
                                x,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the 'x' channel set to the given value.",
            &["_self", "x"],
        )
        .register_documented(
            "with_y",
            |_self: Val<::bevy_color::prelude::Xyza>, y: f32| {
                let output: Val<::bevy_color::prelude::Xyza> = {
                    {
                        let output: Val<::bevy_color::prelude::Xyza> = ::bevy_color::prelude::Xyza::with_y(
                                _self.into_inner(),
                                y,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the 'y' channel set to the given value.",
            &["_self", "y"],
        )
        .register_documented(
            "with_z",
            |_self: Val<::bevy_color::prelude::Xyza>, z: f32| {
                let output: Val<::bevy_color::prelude::Xyza> = {
                    {
                        let output: Val<::bevy_color::prelude::Xyza> = ::bevy_color::prelude::Xyza::with_z(
                                _self.into_inner(),
                                z,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Return a copy of this color with the 'z' channel set to the given value.",
            &["_self", "z"],
        )
        .register_documented(
            "xyz",
            |x: f32, y: f32, z: f32| {
                let output: Val<::bevy_color::prelude::Xyza> = {
                    {
                        let output: Val<::bevy_color::prelude::Xyza> = ::bevy_color::prelude::Xyza::xyz(
                                x,
                                y,
                                z,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a new [`Xyza`] color from (x, y, z) components, with the default alpha (1.0).\n # Arguments\n * `x` - x-axis. [0.0, 1.0]\n * `y` - y-axis. [0.0, 1.0]\n * `z` - z-axis. [0.0, 1.0]",
            &["x", "y", "z"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_color::prelude::Xyza,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyColorScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_color_functions(&mut world);
        register_srgba_functions(&mut world);
        register_linear_rgba_functions(&mut world);
        register_hsla_functions(&mut world);
        register_hsva_functions(&mut world);
        register_hwba_functions(&mut world);
        register_laba_functions(&mut world);
        register_lcha_functions(&mut world);
        register_oklaba_functions(&mut world);
        register_oklcha_functions(&mut world);
        register_xyza_functions(&mut world);
    }
}
