
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]



use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{Ref, Mut, Val},
        namespace::NamespaceBuilder,
    },
};
use bevy_ecs::prelude::*;
use bevy_app::{App, Plugin};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyTextScriptingPlugin;
pub(crate) fn register_font_hinting_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontHinting,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_text::FontHinting>| {
                let output: Val<::bevy_text::FontHinting> = {
                    {
                        let output: Val<::bevy_text::FontHinting> = <::bevy_text::FontHinting as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::FontHinting>, other: Ref<::bevy_text::FontHinting>| {
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
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::FontHinting,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_font_weight_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontWeight,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_text::FontWeight>| {
                let output: () = {
                    {
                        let output: () = <::bevy_text::FontWeight as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            "clamp",
            |_self: Val<::bevy_text::FontWeight>| {
                let output: Val<::bevy_text::FontWeight> = {
                    {
                        let output: Val<::bevy_text::FontWeight> = ::bevy_text::FontWeight::clamp(
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
            |_self: Ref<::bevy_text::FontWeight>| {
                let output: Val<::bevy_text::FontWeight> = {
                    {
                        let output: Val<::bevy_text::FontWeight> = <::bevy_text::FontWeight as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::FontWeight>, other: Ref<::bevy_text::FontWeight>| {
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
pub(crate) fn register_justify_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::Justify,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_text::Justify>| {
                let output: () = {
                    {
                        let output: () = <::bevy_text::Justify as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            "clone",
            |_self: Ref<::bevy_text::Justify>| {
                let output: Val<::bevy_text::Justify> = {
                    {
                        let output: Val<::bevy_text::Justify> = <::bevy_text::Justify as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::Justify>, other: Ref<::bevy_text::Justify>| {
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
        .register_type_data::<
            ::bevy_text::Justify,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_line_break_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::LineBreak,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_text::LineBreak>| {
                let output: () = {
                    {
                        let output: () = <::bevy_text::LineBreak as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            "clone",
            |_self: Ref<::bevy_text::LineBreak>| {
                let output: Val<::bevy_text::LineBreak> = {
                    {
                        let output: Val<::bevy_text::LineBreak> = <::bevy_text::LineBreak as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::LineBreak>, other: Ref<::bevy_text::LineBreak>| {
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
        .register_type_data::<
            ::bevy_text::LineBreak,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_strikethrough_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::Strikethrough,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_text::Strikethrough>| {
                let output: Val<::bevy_text::Strikethrough> = {
                    {
                        let output: Val<::bevy_text::Strikethrough> = <::bevy_text::Strikethrough as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::StrikethroughColor>| {
                let output: Val<::bevy_text::StrikethroughColor> = {
                    {
                        let output: Val<::bevy_text::StrikethroughColor> = <::bevy_text::StrikethroughColor as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_text::StrikethroughColor>,
                other: Ref<::bevy_text::StrikethroughColor>|
            {
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
            |_self: Ref<::bevy_text::TextColor>| {
                let output: Val<::bevy_text::TextColor> = {
                    {
                        let output: Val<::bevy_text::TextColor> = <::bevy_text::TextColor as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::TextColor>, other: Ref<::bevy_text::TextColor>| {
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
        .register_type_data::<
            ::bevy_text::TextColor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_font_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextFont,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_text::TextFont>| {
                let output: Val<::bevy_text::TextFont> = {
                    {
                        let output: Val<::bevy_text::TextFont> = <::bevy_text::TextFont as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::TextFont>, other: Ref<::bevy_text::TextFont>| {
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
            "from_font_size",
            |font_size: f32| {
                let output: Val<::bevy_text::TextFont> = {
                    {
                        let output: Val<::bevy_text::TextFont> = ::bevy_text::TextFont::from_font_size(
                                font_size,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a new [`TextFont`] with the specified font size.",
            &["font_size"],
        )
        .register_documented(
            "with_font_size",
            |_self: Val<::bevy_text::TextFont>, font_size: f32| {
                let output: Val<::bevy_text::TextFont> = {
                    {
                        let output: Val<::bevy_text::TextFont> = ::bevy_text::TextFont::with_font_size(
                                _self.into_inner(),
                                font_size,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns this [`TextFont`] with the specified font size.",
            &["_self", "font_size"],
        )
        .register_documented(
            "with_font_smoothing",
            |
                _self: Val<::bevy_text::TextFont>,
                font_smoothing: Val<::bevy_text::FontSmoothing>|
            {
                let output: Val<::bevy_text::TextFont> = {
                    {
                        let output: Val<::bevy_text::TextFont> = ::bevy_text::TextFont::with_font_smoothing(
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
        .register_type_data::<
            ::bevy_text::TextFont,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_layout_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextLayout,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_text::TextLayout>| {
                let output: Val<::bevy_text::TextLayout> = {
                    {
                        let output: Val<::bevy_text::TextLayout> = <::bevy_text::TextLayout as ::std::clone::Clone>::clone(
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
            |justify: Val<::bevy_text::Justify>, linebreak: Val<::bevy_text::LineBreak>| {
                let output: Val<::bevy_text::TextLayout> = {
                    {
                        let output: Val<::bevy_text::TextLayout> = ::bevy_text::TextLayout::new(
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
            "new_with_justify",
            |justify: Val<::bevy_text::Justify>| {
                let output: Val<::bevy_text::TextLayout> = {
                    {
                        let output: Val<::bevy_text::TextLayout> = ::bevy_text::TextLayout::new_with_justify(
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
            "new_with_linebreak",
            |linebreak: Val<::bevy_text::LineBreak>| {
                let output: Val<::bevy_text::TextLayout> = {
                    {
                        let output: Val<::bevy_text::TextLayout> = ::bevy_text::TextLayout::new_with_linebreak(
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
            "new_with_no_wrap",
            || {
                let output: Val<::bevy_text::TextLayout> = {
                    {
                        let output: Val<::bevy_text::TextLayout> = ::bevy_text::TextLayout::new_with_no_wrap()
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
            |_self: Val<::bevy_text::TextLayout>, justify: Val<::bevy_text::Justify>| {
                let output: Val<::bevy_text::TextLayout> = {
                    {
                        let output: Val<::bevy_text::TextLayout> = ::bevy_text::TextLayout::with_justify(
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
            |
                _self: Val<::bevy_text::TextLayout>,
                linebreak: Val<::bevy_text::LineBreak>|
            {
                let output: Val<::bevy_text::TextLayout> = {
                    {
                        let output: Val<::bevy_text::TextLayout> = ::bevy_text::TextLayout::with_linebreak(
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
            |_self: Val<::bevy_text::TextLayout>| {
                let output: Val<::bevy_text::TextLayout> = {
                    {
                        let output: Val<::bevy_text::TextLayout> = ::bevy_text::TextLayout::with_no_wrap(
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
            |_self: Ref<::bevy_text::TextSpan>| {
                let output: Val<::bevy_text::TextSpan> = {
                    {
                        let output: Val<::bevy_text::TextSpan> = <::bevy_text::TextSpan as ::std::clone::Clone>::clone(
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
            ::bevy_text::TextSpan,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_underline_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::Underline,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_text::Underline>| {
                let output: Val<::bevy_text::Underline> = {
                    {
                        let output: Val<::bevy_text::Underline> = <::bevy_text::Underline as ::std::clone::Clone>::clone(
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
            ::bevy_text::Underline,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_underline_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::UnderlineColor,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_text::UnderlineColor>| {
                let output: Val<::bevy_text::UnderlineColor> = {
                    {
                        let output: Val<::bevy_text::UnderlineColor> = <::bevy_text::UnderlineColor as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_text::UnderlineColor>,
                other: Ref<::bevy_text::UnderlineColor>|
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
            |_self: Ref<::bevy_text::TextBounds>| {
                let output: Val<::bevy_text::TextBounds> = {
                    {
                        let output: Val<::bevy_text::TextBounds> = <::bevy_text::TextBounds as ::std::clone::Clone>::clone(
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
                let output: Val<::bevy_text::TextBounds> = {
                    {
                        let output: Val<::bevy_text::TextBounds> = ::bevy_text::TextBounds::new(
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
                let output: Val<::bevy_text::TextBounds> = {
                    {
                        let output: Val<::bevy_text::TextBounds> = ::bevy_text::TextBounds::new_horizontal(
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
                let output: Val<::bevy_text::TextBounds> = {
                    {
                        let output: Val<::bevy_text::TextBounds> = ::bevy_text::TextBounds::new_vertical(
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
pub(crate) fn register_font_smoothing_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontSmoothing,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_text::FontSmoothing>| {
                let output: () = {
                    {
                        let output: () = <::bevy_text::FontSmoothing as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            "clone",
            |_self: Ref<::bevy_text::FontSmoothing>| {
                let output: Val<::bevy_text::FontSmoothing> = {
                    {
                        let output: Val<::bevy_text::FontSmoothing> = <::bevy_text::FontSmoothing as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_text::FontSmoothing>,
                other: Ref<::bevy_text::FontSmoothing>|
            {
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
pub(crate) fn register_glyph_atlas_info_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::GlyphAtlasInfo,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_text::GlyphAtlasInfo>| {
                let output: Val<::bevy_text::GlyphAtlasInfo> = {
                    {
                        let output: Val<::bevy_text::GlyphAtlasInfo> = <::bevy_text::GlyphAtlasInfo as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::GlyphAtlasLocation>| {
                let output: Val<::bevy_text::GlyphAtlasLocation> = {
                    {
                        let output: Val<::bevy_text::GlyphAtlasLocation> = <::bevy_text::GlyphAtlasLocation as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::PositionedGlyph>| {
                let output: Val<::bevy_text::PositionedGlyph> = {
                    {
                        let output: Val<::bevy_text::PositionedGlyph> = <::bevy_text::PositionedGlyph as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::ComputedTextBlock>| {
                let output: Val<::bevy_text::ComputedTextBlock> = {
                    {
                        let output: Val<::bevy_text::ComputedTextBlock> = <::bevy_text::ComputedTextBlock as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::ComputedTextBlock>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_text::ComputedTextBlock::needs_rerender(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Indicates if the text needs to be refreshed in [`TextLayoutInfo`].\n Updated automatically by [`detect_text_needs_rerender`] and cleared\n by [`TextPipeline`](crate::TextPipeline) methods.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_text::ComputedTextBlock,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_line_height_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::LineHeight,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_text::LineHeight>| {
                let output: Val<::bevy_text::LineHeight> = {
                    {
                        let output: Val<::bevy_text::LineHeight> = <::bevy_text::LineHeight as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::LineHeight>, other: Ref<::bevy_text::LineHeight>| {
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
pub(crate) fn register_text_entity_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextEntity,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_text::TextEntity>| {
                let output: Val<::bevy_text::TextEntity> = {
                    {
                        let output: Val<::bevy_text::TextEntity> = <::bevy_text::TextEntity as ::std::clone::Clone>::clone(
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
pub(crate) fn register_text_layout_info_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextLayoutInfo,
    >::new(world)
        .register_documented(
            "clear",
            |mut _self: Mut<::bevy_text::TextLayoutInfo>| {
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
            |_self: Ref<::bevy_text::TextLayoutInfo>| {
                let output: Val<::bevy_text::TextLayoutInfo> = {
                    {
                        let output: Val<::bevy_text::TextLayoutInfo> = <::bevy_text::TextLayoutInfo as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::RunGeometry>| {
                let output: Val<::bevy_text::RunGeometry> = {
                    {
                        let output: Val<::bevy_text::RunGeometry> = <::bevy_text::RunGeometry as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_text::RunGeometry>| {
                let output: Val<::bevy_math::Vec2> = {
                    {
                        let output: Val<::bevy_math::Vec2> = ::bevy_text::RunGeometry::strikethrough_position(
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
            |_self: Ref<::bevy_text::RunGeometry>| {
                let output: Val<::bevy_math::Vec2> = {
                    {
                        let output: Val<::bevy_math::Vec2> = ::bevy_text::RunGeometry::strikethrough_size(
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
            |_self: Ref<::bevy_text::RunGeometry>| {
                let output: Val<::bevy_math::Vec2> = {
                    {
                        let output: Val<::bevy_math::Vec2> = ::bevy_text::RunGeometry::underline_position(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the center of the underline in the text layout.",
            &["_self"],
        )
        .register_documented(
            "underline_size",
            |_self: Ref<::bevy_text::RunGeometry>| {
                let output: Val<::bevy_math::Vec2> = {
                    {
                        let output: Val<::bevy_math::Vec2> = ::bevy_text::RunGeometry::underline_size(
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
pub(crate) fn register_font_features_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontFeatures,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_text::FontFeatures>| {
                let output: Val<::bevy_text::FontFeatures> = {
                    {
                        let output: Val<::bevy_text::FontFeatures> = <::bevy_text::FontFeatures as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_text::FontFeatures>,
                other: Ref<::bevy_text::FontFeatures>|
            {
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
pub(crate) fn register_font_feature_tag_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::FontFeatureTag,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_text::FontFeatureTag>| {
                let output: () = {
                    {
                        let output: () = <::bevy_text::FontFeatureTag as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            "clone",
            |_self: Ref<::bevy_text::FontFeatureTag>| {
                let output: Val<::bevy_text::FontFeatureTag> = {
                    {
                        let output: Val<::bevy_text::FontFeatureTag> = <::bevy_text::FontFeatureTag as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_text::FontFeatureTag>,
                other: Ref<::bevy_text::FontFeatureTag>|
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
pub(crate) fn register_text_background_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_text::TextBackgroundColor,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_text::TextBackgroundColor>| {
                let output: Val<::bevy_text::TextBackgroundColor> = {
                    {
                        let output: Val<::bevy_text::TextBackgroundColor> = <::bevy_text::TextBackgroundColor as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_text::TextBackgroundColor>,
                other: Ref<::bevy_text::TextBackgroundColor>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_text::TextBackgroundColor as ::std::cmp::PartialEq<
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
impl Plugin for BevyTextScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_font_hinting_functions(&mut world);
        register_font_weight_functions(&mut world);
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
        register_font_smoothing_functions(&mut world);
        register_glyph_atlas_info_functions(&mut world);
        register_glyph_atlas_location_functions(&mut world);
        register_positioned_glyph_functions(&mut world);
        register_computed_text_block_functions(&mut world);
        register_line_height_functions(&mut world);
        register_text_entity_functions(&mut world);
        register_text_layout_info_functions(&mut world);
        register_run_geometry_functions(&mut world);
        register_font_features_functions(&mut world);
        register_font_feature_tag_functions(&mut world);
        register_text_background_color_functions(&mut world);
    }
}
