#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
extern crate std;

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{M, R, V},
        namespace::NamespaceBuilder,
    },
};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyTextScriptingPlugin;
pub(crate) fn register_font_hinting_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontHinting,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::FontHinting>| {
                let output: V<::bevy_text::FontHinting> = {
                    {
                        let output: V<::bevy_text::FontHinting> = <::bevy_text::FontHinting as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::FontHinting>, other: R<::bevy_text::FontHinting>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::FontHinting as ::std::cmp::PartialEq<
                            ::bevy_text::FontHinting,
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
            "is_enabled",
            |_self: V<::bevy_text::FontHinting>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_text::FontHinting::is_enabled(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if font hinting is enabled.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::FontHinting,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_font_size_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontSize,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::FontSize>| {
                let output: V<::bevy_text::FontSize> = {
                    {
                        let output: V<::bevy_text::FontSize> = <::bevy_text::FontSize as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::FontSize>, other: R<::bevy_text::FontSize>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::FontSize as ::std::cmp::PartialEq<
                            ::bevy_text::FontSize,
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
            "eval",
            |
                _self: V<::bevy_text::FontSize>,
                logical_viewport_size: V<::bevy_math::Vec2>,
                rem_size: f32|
            {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_text::FontSize::eval(
                                _self.into_inner(),
                                logical_viewport_size.into_inner(),
                                rem_size,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Evaluate the font size to a value in logical pixels",
            &["_self", "logical_viewport_size", "rem_size"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_text::FontSize, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_font_smoothing_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontSmoothing,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::FontSmoothing>| {
                let output: V<::bevy_text::FontSmoothing> = {
                    {
                        let output: V<::bevy_text::FontSmoothing> = <::bevy_text::FontSmoothing as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::FontSmoothing>, other: R<::bevy_text::FontSmoothing>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::FontSmoothing as ::std::cmp::PartialEq<
                            ::bevy_text::FontSmoothing,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::FontSmoothing,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_font_source_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontSource,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::FontSource>| {
                let output: V<::bevy_text::FontSource> = {
                    {
                        let output: V<::bevy_text::FontSource> = <::bevy_text::FontSource as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::FontSource>, other: R<::bevy_text::FontSource>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::FontSource as ::std::cmp::PartialEq<
                            ::bevy_text::FontSource,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::FontSource,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_font_style_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontStyle,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::FontStyle>| {
                let output: V<::bevy_text::FontStyle> = {
                    {
                        let output: V<::bevy_text::FontStyle> = <::bevy_text::FontStyle as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::FontStyle>, other: R<::bevy_text::FontStyle>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::FontStyle as ::std::cmp::PartialEq<
                            ::bevy_text::FontStyle,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_text::FontStyle, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_font_weight_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontWeight,
    >::new(world)
        .register_documented(
            "clamp",
            |_self: V<::bevy_text::FontWeight>| {
                let output: V<::bevy_text::FontWeight> = {
                    {
                        let output: V<::bevy_text::FontWeight> = ::bevy_text::FontWeight::clamp(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Clamp the weight value to between 1 and 1000.\n Values of 0 are mapped to `Weight::DEFAULT`.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_text::FontWeight>| {
                let output: V<::bevy_text::FontWeight> = {
                    {
                        let output: V<::bevy_text::FontWeight> = <::bevy_text::FontWeight as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::FontWeight>, other: R<::bevy_text::FontWeight>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::FontWeight as ::std::cmp::PartialEq<
                            ::bevy_text::FontWeight,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::FontWeight,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_font_width_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontWidth,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::FontWidth>| {
                let output: V<::bevy_text::FontWidth> = {
                    {
                        let output: V<::bevy_text::FontWidth> = <::bevy_text::FontWidth as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::FontWidth>, other: R<::bevy_text::FontWidth>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::FontWidth as ::std::cmp::PartialEq<
                            ::bevy_text::FontWidth,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_text::FontWidth, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_justify_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::Justify,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::Justify>| {
                let output: V<::bevy_text::Justify> = {
                    {
                        let output: V<::bevy_text::Justify> = <::bevy_text::Justify as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::Justify>, other: R<::bevy_text::Justify>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::Justify as ::std::cmp::PartialEq<
                            ::bevy_text::Justify,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_text::Justify, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_line_break_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::LineBreak,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::LineBreak>| {
                let output: V<::bevy_text::LineBreak> = {
                    {
                        let output: V<::bevy_text::LineBreak> = <::bevy_text::LineBreak as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::LineBreak>, other: R<::bevy_text::LineBreak>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::LineBreak as ::std::cmp::PartialEq<
                            ::bevy_text::LineBreak,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_text::LineBreak, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_strikethrough_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::Strikethrough,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::Strikethrough>| {
                let output: V<::bevy_text::Strikethrough> = {
                    {
                        let output: V<::bevy_text::Strikethrough> = <::bevy_text::Strikethrough as ::std::clone::Clone>::clone(
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
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::Strikethrough,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_strikethrough_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::StrikethroughColor,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_text::StrikethroughColor>| {
            let output: V<::bevy_text::StrikethroughColor> = {
                {
                    let output: V<::bevy_text::StrikethroughColor> =
                        <::bevy_text::StrikethroughColor as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_text::StrikethroughColor>, other: R<::bevy_text::StrikethroughColor>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_text::StrikethroughColor as ::std::cmp::PartialEq<
                        ::bevy_text::StrikethroughColor,
                    >>::eq(&_self, &other)
                    .into();
                    output
                }
            };
            output
        },
        "",
        &["_self", "other"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::StrikethroughColor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextColor,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::TextColor>| {
                let output: V<::bevy_text::TextColor> = {
                    {
                        let output: V<::bevy_text::TextColor> = <::bevy_text::TextColor as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::TextColor>, other: R<::bevy_text::TextColor>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::TextColor as ::std::cmp::PartialEq<
                            ::bevy_text::TextColor,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_text::TextColor, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_text_font_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextFont,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::TextFont>| {
                let output: V<::bevy_text::TextFont> = {
                    {
                        let output: V<::bevy_text::TextFont> = <::bevy_text::TextFont as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::TextFont>, other: R<::bevy_text::TextFont>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::TextFont as ::std::cmp::PartialEq<
                            ::bevy_text::TextFont,
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
            "with_font_smoothing",
            |
                _self: V<::bevy_text::TextFont>,
                font_smoothing: V<::bevy_text::FontSmoothing>|
            {
                let output: V<::bevy_text::TextFont> = {
                    {
                        let output: V<::bevy_text::TextFont> = ::bevy_text::TextFont::with_font_smoothing(
                                _self.into_inner(),
                                font_smoothing.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns this [`TextFont`] with the specified [`FontSmoothing`].",
            &["_self", "font_smoothing"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_text::TextFont, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_text_layout_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextLayout,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::TextLayout>| {
                let output: V<::bevy_text::TextLayout> = {
                    {
                        let output: V<::bevy_text::TextLayout> = <::bevy_text::TextLayout as ::std::clone::Clone>::clone(
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
            "justify",
            |justify: V<::bevy_text::Justify>| {
                let output: V<::bevy_text::TextLayout> = {
                    {
                        let output: V<::bevy_text::TextLayout> = ::bevy_text::TextLayout::justify(
                                justify.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Makes a new [`TextLayout`] with the specified [`Justify`].",
            &["justify"],
        )
        .register_documented(
            "linebreak",
            |linebreak: V<::bevy_text::LineBreak>| {
                let output: V<::bevy_text::TextLayout> = {
                    {
                        let output: V<::bevy_text::TextLayout> = ::bevy_text::TextLayout::linebreak(
                                linebreak.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Makes a new [`TextLayout`] with the specified [`LineBreak`].",
            &["linebreak"],
        )
        .register_documented(
            "new",
            |justify: V<::bevy_text::Justify>, linebreak: V<::bevy_text::LineBreak>| {
                let output: V<::bevy_text::TextLayout> = {
                    {
                        let output: V<::bevy_text::TextLayout> = ::bevy_text::TextLayout::new(
                                justify.into_inner(),
                                linebreak.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Makes a new [`TextLayout`].",
            &["justify", "linebreak"],
        )
        .register_documented(
            "no_wrap",
            || {
                let output: V<::bevy_text::TextLayout> = {
                    {
                        let output: V<::bevy_text::TextLayout> = ::bevy_text::TextLayout::no_wrap()
                            .into();
                        output
                    }
                };
                output
            },
            " Makes a new [`TextLayout`] with soft wrapping disabled.\n Hard wrapping, where text contains an explicit linebreak such as the escape sequence `\\n`, will still occur.",
            &[],
        )
        .register_documented(
            "with_justify",
            |_self: V<::bevy_text::TextLayout>, justify: V<::bevy_text::Justify>| {
                let output: V<::bevy_text::TextLayout> = {
                    {
                        let output: V<::bevy_text::TextLayout> = ::bevy_text::TextLayout::with_justify(
                                _self.into_inner(),
                                justify.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns this [`TextLayout`] with the specified [`Justify`].",
            &["_self", "justify"],
        )
        .register_documented(
            "with_linebreak",
            |_self: V<::bevy_text::TextLayout>, linebreak: V<::bevy_text::LineBreak>| {
                let output: V<::bevy_text::TextLayout> = {
                    {
                        let output: V<::bevy_text::TextLayout> = ::bevy_text::TextLayout::with_linebreak(
                                _self.into_inner(),
                                linebreak.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns this [`TextLayout`] with the specified [`LineBreak`].",
            &["_self", "linebreak"],
        )
        .register_documented(
            "with_no_wrap",
            |_self: V<::bevy_text::TextLayout>| {
                let output: V<::bevy_text::TextLayout> = {
                    {
                        let output: V<::bevy_text::TextLayout> = ::bevy_text::TextLayout::with_no_wrap(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns this [`TextLayout`] with soft wrapping disabled.\n Hard wrapping, where text contains an explicit linebreak such as the escape sequence `\\n`, will still occur.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::TextLayout,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_span_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextSpan,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::TextSpan>| {
                let output: V<::bevy_text::TextSpan> = {
                    {
                        let output: V<::bevy_text::TextSpan> = <::bevy_text::TextSpan as ::std::clone::Clone>::clone(
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
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_text::TextSpan, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_underline_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::Underline,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::Underline>| {
                let output: V<::bevy_text::Underline> = {
                    {
                        let output: V<::bevy_text::Underline> = <::bevy_text::Underline as ::std::clone::Clone>::clone(
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
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_text::Underline, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_underline_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::UnderlineColor,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::UnderlineColor>| {
                let output: V<::bevy_text::UnderlineColor> = {
                    {
                        let output: V<::bevy_text::UnderlineColor> = <::bevy_text::UnderlineColor as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_text::UnderlineColor>,
                other: R<::bevy_text::UnderlineColor>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::UnderlineColor as ::std::cmp::PartialEq<
                            ::bevy_text::UnderlineColor,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::UnderlineColor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_bounds_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextBounds,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::TextBounds>| {
                let output: V<::bevy_text::TextBounds> = {
                    {
                        let output: V<::bevy_text::TextBounds> = <::bevy_text::TextBounds as ::std::clone::Clone>::clone(
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
            "new",
            |width: f32, height: f32| {
                let output: V<::bevy_text::TextBounds> = {
                    {
                        let output: V<::bevy_text::TextBounds> = ::bevy_text::TextBounds::new(
                                width,
                                height,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new `TextBounds`, bounded with the specified width and height values.",
            &["width", "height"],
        )
        .register_documented(
            "new_horizontal",
            |width: f32| {
                let output: V<::bevy_text::TextBounds> = {
                    {
                        let output: V<::bevy_text::TextBounds> = ::bevy_text::TextBounds::new_horizontal(
                                width,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new `TextBounds`, bounded with the specified width value and unbounded on height.",
            &["width"],
        )
        .register_documented(
            "new_vertical",
            |height: f32| {
                let output: V<::bevy_text::TextBounds> = {
                    {
                        let output: V<::bevy_text::TextBounds> = ::bevy_text::TextBounds::new_vertical(
                                height,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new `TextBounds`, bounded with the specified height value and unbounded on width.",
            &["height"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::TextBounds,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_edit_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextEdit,
    >::new(world)
        .register_documented(
            "clear_ime_compose",
            || {
                let output: V<::bevy_text::TextEdit> = {
                    {
                        let output: V<::bevy_text::TextEdit> = ::bevy_text::TextEdit::clear_ime_compose()
                            .into();
                        output
                    }
                };
                output
            },
            " Convenience constructor for a [`TextEdit::ImeSetCompose`] that clears the preedit.",
            &[],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_text::TextEdit>| {
                let output: V<::bevy_text::TextEdit> = {
                    {
                        let output: V<::bevy_text::TextEdit> = <::bevy_text::TextEdit as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::TextEdit>, other: R<::bevy_text::TextEdit>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::TextEdit as ::std::cmp::PartialEq<
                            ::bevy_text::TextEdit,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_text::TextEdit, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_line_height_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::LineHeight,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::LineHeight>| {
                let output: V<::bevy_text::LineHeight> = {
                    {
                        let output: V<::bevy_text::LineHeight> = <::bevy_text::LineHeight as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::LineHeight>, other: R<::bevy_text::LineHeight>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::LineHeight as ::std::cmp::PartialEq<
                            ::bevy_text::LineHeight,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::LineHeight,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_glyph_atlas_info_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::GlyphAtlasInfo,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::GlyphAtlasInfo>| {
                let output: V<::bevy_text::GlyphAtlasInfo> = {
                    {
                        let output: V<::bevy_text::GlyphAtlasInfo> = <::bevy_text::GlyphAtlasInfo as ::std::clone::Clone>::clone(
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
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::GlyphAtlasInfo,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_glyph_atlas_location_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::GlyphAtlasLocation,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_text::GlyphAtlasLocation>| {
            let output: V<::bevy_text::GlyphAtlasLocation> = {
                {
                    let output: V<::bevy_text::GlyphAtlasLocation> =
                        <::bevy_text::GlyphAtlasLocation as ::std::clone::Clone>::clone(&_self)
                            .into();
                    output
                }
            };
            output
        },
        "",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::GlyphAtlasLocation,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_positioned_glyph_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::PositionedGlyph,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_text::PositionedGlyph>| {
            let output: V<::bevy_text::PositionedGlyph> = {
                {
                    let output: V<::bevy_text::PositionedGlyph> =
                        <::bevy_text::PositionedGlyph as ::std::clone::Clone>::clone(&_self).into();
                    output
                }
            };
            output
        },
        "",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::PositionedGlyph,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_computed_text_block_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::ComputedTextBlock,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::ComputedTextBlock>| {
                let output: V<::bevy_text::ComputedTextBlock> = {
                    {
                        let output: V<::bevy_text::ComputedTextBlock> = <::bevy_text::ComputedTextBlock as ::std::clone::Clone>::clone(
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
            "needs_rerender",
            |
                _self: R<::bevy_text::ComputedTextBlock>,
                is_viewport_size_changed: bool,
                is_rem_size_changed: bool|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_text::ComputedTextBlock::needs_rerender(
                                &_self,
                                is_viewport_size_changed,
                                is_rem_size_changed,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Indicates if the text needs to be refreshed in [`TextLayoutInfo`].\n Updated automatically by [`detect_text_needs_rerender`] and cleared\n by [`TextPipeline`](crate::TextPipeline) methods.",
            &["_self", "is_viewport_size_changed", "is_rem_size_changed"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::ComputedTextBlock,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_letter_spacing_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::LetterSpacing,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::LetterSpacing>| {
                let output: V<::bevy_text::LetterSpacing> = {
                    {
                        let output: V<::bevy_text::LetterSpacing> = <::bevy_text::LetterSpacing as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::LetterSpacing>, other: R<::bevy_text::LetterSpacing>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::LetterSpacing as ::std::cmp::PartialEq<
                            ::bevy_text::LetterSpacing,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::LetterSpacing,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_entity_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextEntity,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::TextEntity>| {
                let output: V<::bevy_text::TextEntity> = {
                    {
                        let output: V<::bevy_text::TextEntity> = <::bevy_text::TextEntity as ::std::clone::Clone>::clone(
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
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::TextEntity,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_font_features_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontFeatures,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::FontFeatures>| {
                let output: V<::bevy_text::FontFeatures> = {
                    {
                        let output: V<::bevy_text::FontFeatures> = <::bevy_text::FontFeatures as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::FontFeatures>, other: R<::bevy_text::FontFeatures>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::FontFeatures as ::std::cmp::PartialEq<
                            ::bevy_text::FontFeatures,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::FontFeatures,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_font_variations_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontVariations,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::FontVariations>| {
                let output: V<::bevy_text::FontVariations> = {
                    {
                        let output: V<::bevy_text::FontVariations> = <::bevy_text::FontVariations as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_text::FontVariations>,
                other: R<::bevy_text::FontVariations>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::FontVariations as ::std::cmp::PartialEq<
                            ::bevy_text::FontVariations,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::FontVariations,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_layout_info_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextLayoutInfo,
    >::new(world)
        .register_documented(
            "clear",
            |mut _self: M<::bevy_text::TextLayoutInfo>| {
                let output: () = {
                    {
                        let output: () = ::bevy_text::TextLayoutInfo::clear(&mut _self)
                            .into();
                        output
                    }
                };
                output
            },
            " Clear the layout, retaining capacity",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_text::TextLayoutInfo>| {
                let output: V<::bevy_text::TextLayoutInfo> = {
                    {
                        let output: V<::bevy_text::TextLayoutInfo> = <::bevy_text::TextLayoutInfo as ::std::clone::Clone>::clone(
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
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::TextLayoutInfo,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_run_geometry_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::RunGeometry,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::RunGeometry>| {
                let output: V<::bevy_text::RunGeometry> = {
                    {
                        let output: V<::bevy_text::RunGeometry> = <::bevy_text::RunGeometry as ::std::clone::Clone>::clone(
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
            "strikethrough_position",
            |_self: R<::bevy_text::RunGeometry>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_text::RunGeometry::strikethrough_position(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the center of the strikethrough in the text layout.",
            &["_self"],
        )
        .register_documented(
            "strikethrough_size",
            |_self: R<::bevy_text::RunGeometry>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_text::RunGeometry::strikethrough_size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the size of the strikethrough.",
            &["_self"],
        )
        .register_documented(
            "underline_position",
            |_self: R<::bevy_text::RunGeometry>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_text::RunGeometry::underline_position(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the center of the underline in the text layout.",
            &["_self"],
        )
        .register_documented(
            "underline_size",
            |_self: R<::bevy_text::RunGeometry>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_text::RunGeometry::underline_size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the size of the underline.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::RunGeometry,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_font_feature_tag_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontFeatureTag,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::FontFeatureTag>| {
                let output: V<::bevy_text::FontFeatureTag> = {
                    {
                        let output: V<::bevy_text::FontFeatureTag> = <::bevy_text::FontFeatureTag as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_text::FontFeatureTag>,
                other: R<::bevy_text::FontFeatureTag>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::FontFeatureTag as ::std::cmp::PartialEq<
                            ::bevy_text::FontFeatureTag,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::FontFeatureTag,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_font_variation_tag_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontVariationTag,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_text::FontVariationTag>| {
            let output: V<::bevy_text::FontVariationTag> = {
                {
                    let output: V<::bevy_text::FontVariationTag> =
                        <::bevy_text::FontVariationTag as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_text::FontVariationTag>, other: R<::bevy_text::FontVariationTag>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_text::FontVariationTag as ::std::cmp::PartialEq<
                        ::bevy_text::FontVariationTag,
                    >>::eq(&_self, &other)
                    .into();
                    output
                }
            };
            output
        },
        "",
        &["_self", "other"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::FontVariationTag,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_background_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextBackgroundColor,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_text::TextBackgroundColor>| {
            let output: V<::bevy_text::TextBackgroundColor> = {
                {
                    let output: V<::bevy_text::TextBackgroundColor> =
                        <::bevy_text::TextBackgroundColor as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_text::TextBackgroundColor>, other: R<::bevy_text::TextBackgroundColor>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_text::TextBackgroundColor as ::std::cmp::PartialEq<
                            ::bevy_text::TextBackgroundColor,
                        >>::eq(&_self, &other)
                        .into();
                    output
                }
            };
            output
        },
        "",
        &["_self", "other"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::TextBackgroundColor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_preedit_cursor_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::PreeditCursor,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_text::PreeditCursor>| {
                let output: V<::bevy_text::PreeditCursor> = {
                    {
                        let output: V<::bevy_text::PreeditCursor> = <::bevy_text::PreeditCursor as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_text::PreeditCursor>, other: R<::bevy_text::PreeditCursor>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::PreeditCursor as ::std::cmp::PartialEq<
                            ::bevy_text::PreeditCursor,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::PreeditCursor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyTextScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_font_hinting_functions(&mut world);
        register_font_size_functions(&mut world);
        register_font_smoothing_functions(&mut world);
        register_font_source_functions(&mut world);
        register_font_style_functions(&mut world);
        register_font_weight_functions(&mut world);
        register_font_width_functions(&mut world);
        register_justify_functions(&mut world);
        register_line_break_functions(&mut world);
        register_strikethrough_functions(&mut world);
        register_strikethrough_color_functions(&mut world);
        register_text_color_functions(&mut world);
        register_text_font_functions(&mut world);
        register_text_layout_functions(&mut world);
        register_text_span_functions(&mut world);
        register_underline_functions(&mut world);
        register_underline_color_functions(&mut world);
        register_text_bounds_functions(&mut world);
        register_text_edit_functions(&mut world);
        register_line_height_functions(&mut world);
        register_glyph_atlas_info_functions(&mut world);
        register_glyph_atlas_location_functions(&mut world);
        register_positioned_glyph_functions(&mut world);
        register_computed_text_block_functions(&mut world);
        register_letter_spacing_functions(&mut world);
        register_text_entity_functions(&mut world);
        register_font_features_functions(&mut world);
        register_font_variations_functions(&mut world);
        register_text_layout_info_functions(&mut world);
        register_run_geometry_functions(&mut world);
        register_font_feature_tag_functions(&mut world);
        register_font_variation_tag_functions(&mut world);
        register_text_background_color_functions(&mut world);
        register_preedit_cursor_functions(&mut world);
    }
}
