#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{Mut, Ref, Val},
        namespace::NamespaceBuilder,
    },
};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyUiScriptingPlugin;
pub(crate) fn register_display_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::Display,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::Display>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::Display as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::Display>| {
                let output: Val<::bevy_ui::prelude::Display> = {
                    {
                        let output: Val<::bevy_ui::prelude::Display> = <::bevy_ui::prelude::Display as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::Display>,
                other: Ref<::bevy_ui::prelude::Display>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::Display as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::Display,
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
            ::bevy_ui::prelude::Display,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_picking_camera_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::picking_backend::UiPickingCamera,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::picking_backend::UiPickingCamera>| {
                let output: Val<::bevy_ui::picking_backend::UiPickingCamera> = {
                    {
                        let output: Val<::bevy_ui::picking_backend::UiPickingCamera> = <::bevy_ui::picking_backend::UiPickingCamera as ::std::clone::Clone>::clone(
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
            ::bevy_ui::picking_backend::UiPickingCamera,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_picking_settings_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::picking_backend::UiPickingSettings,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::picking_backend::UiPickingSettings,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::widget::Text,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::widget::Text>| {
                let output: Val<::bevy_ui::widget::Text> = {
                    {
                        let output: Val<::bevy_ui::widget::Text> = <::bevy_ui::widget::Text as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::widget::Text>, other: Ref<::bevy_ui::widget::Text>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::widget::Text as ::std::cmp::PartialEq<
                            ::bevy_ui::widget::Text,
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
            ::bevy_ui::widget::Text,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_shadow_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::widget::TextShadow,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::widget::TextShadow>| {
            let output: Val<::bevy_ui::widget::TextShadow> = {
                {
                    let output: Val<::bevy_ui::widget::TextShadow> =
                        <::bevy_ui::widget::TextShadow as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_ui::widget::TextShadow>, other: Ref<::bevy_ui::widget::TextShadow>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_ui::widget::TextShadow as ::std::cmp::PartialEq<
                        ::bevy_ui::widget::TextShadow,
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
            ::bevy_ui::widget::TextShadow,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_button_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::widget::Button,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::widget::Button>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::widget::Button as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::widget::Button>| {
                let output: Val<::bevy_ui::widget::Button> = {
                    {
                        let output: Val<::bevy_ui::widget::Button> = <::bevy_ui::widget::Button as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::widget::Button>,
                other: Ref<::bevy_ui::widget::Button>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::widget::Button as ::std::cmp::PartialEq<
                            ::bevy_ui::widget::Button,
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
            ::bevy_ui::widget::Button,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_image_node_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::widget::ImageNode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::widget::ImageNode>| {
                let output: Val<::bevy_ui::widget::ImageNode> = {
                    {
                        let output: Val<::bevy_ui::widget::ImageNode> = <::bevy_ui::widget::ImageNode as ::std::clone::Clone>::clone(
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
            "solid_color",
            |color: Val<::bevy_color::Color>| {
                let output: Val<::bevy_ui::widget::ImageNode> = {
                    {
                        let output: Val<::bevy_ui::widget::ImageNode> = ::bevy_ui::widget::ImageNode::solid_color(
                                color.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a solid color [`ImageNode`].\n This is primarily useful for debugging / mocking the extents of your image.",
            &["color"],
        )
        .register_documented(
            "with_color",
            |_self: Val<::bevy_ui::widget::ImageNode>, color: Val<::bevy_color::Color>| {
                let output: Val<::bevy_ui::widget::ImageNode> = {
                    {
                        let output: Val<::bevy_ui::widget::ImageNode> = ::bevy_ui::widget::ImageNode::with_color(
                                _self.into_inner(),
                                color.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Set the color tint",
            &["_self", "color"],
        )
        .register_documented(
            "with_flip_x",
            |_self: Val<::bevy_ui::widget::ImageNode>| {
                let output: Val<::bevy_ui::widget::ImageNode> = {
                    {
                        let output: Val<::bevy_ui::widget::ImageNode> = ::bevy_ui::widget::ImageNode::with_flip_x(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Flip the image along its x-axis",
            &["_self"],
        )
        .register_documented(
            "with_flip_y",
            |_self: Val<::bevy_ui::widget::ImageNode>| {
                let output: Val<::bevy_ui::widget::ImageNode> = {
                    {
                        let output: Val<::bevy_ui::widget::ImageNode> = ::bevy_ui::widget::ImageNode::with_flip_y(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Flip the image along its y-axis",
            &["_self"],
        )
        .register_documented(
            "with_mode",
            |
                _self: Val<::bevy_ui::widget::ImageNode>,
                mode: Val<::bevy_ui::widget::NodeImageMode>|
            {
                let output: Val<::bevy_ui::widget::ImageNode> = {
                    {
                        let output: Val<::bevy_ui::widget::ImageNode> = ::bevy_ui::widget::ImageNode::with_mode(
                                _self.into_inner(),
                                mode.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "mode"],
        )
        .register_documented(
            "with_rect",
            |_self: Val<::bevy_ui::widget::ImageNode>, rect: Val<::bevy_math::Rect>| {
                let output: Val<::bevy_ui::widget::ImageNode> = {
                    {
                        let output: Val<::bevy_ui::widget::ImageNode> = ::bevy_ui::widget::ImageNode::with_rect(
                                _self.into_inner(),
                                rect.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rect"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::widget::ImageNode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_label_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::widget::Label,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::widget::Label>| {
                let output: Val<::bevy_ui::widget::Label> = {
                    {
                        let output: Val<::bevy_ui::widget::Label> = <::bevy_ui::widget::Label as ::std::clone::Clone>::clone(
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
            ::bevy_ui::widget::Label,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_node_image_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::widget::NodeImageMode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::widget::NodeImageMode>| {
                let output: Val<::bevy_ui::widget::NodeImageMode> = {
                    {
                        let output: Val<::bevy_ui::widget::NodeImageMode> = <::bevy_ui::widget::NodeImageMode as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::widget::NodeImageMode>,
                other: Ref<::bevy_ui::widget::NodeImageMode>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::widget::NodeImageMode as ::std::cmp::PartialEq<
                            ::bevy_ui::widget::NodeImageMode,
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
            "uses_slices",
            |_self: Ref<::bevy_ui::widget::NodeImageMode>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ui::widget::NodeImageMode::uses_slices(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if this mode uses slices internally ([`NodeImageMode::Sliced`] or [`NodeImageMode::Tiled`])",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::widget::NodeImageMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_viewport_node_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::widget::ViewportNode,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::widget::ViewportNode>| {
            let output: Val<::bevy_ui::widget::ViewportNode> = {
                {
                    let output: Val<::bevy_ui::widget::ViewportNode> =
                        <::bevy_ui::widget::ViewportNode as ::std::clone::Clone>::clone(&_self)
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
        |camera: Val<::bevy_ecs::entity::Entity>| {
            let output: Val<::bevy_ui::widget::ViewportNode> = {
                {
                    let output: Val<::bevy_ui::widget::ViewportNode> =
                        ::bevy_ui::widget::ViewportNode::new(camera.into_inner()).into();
                    output
                }
            };
            output
        },
        " Creates a new [`ViewportNode`] with a given `camera`.",
        &["camera"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::widget::ViewportNode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_interaction_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::Interaction,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::Interaction>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::Interaction as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::Interaction>| {
                let output: Val<::bevy_ui::prelude::Interaction> = {
                    {
                        let output: Val<::bevy_ui::prelude::Interaction> = <::bevy_ui::prelude::Interaction as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::Interaction>,
                other: Ref<::bevy_ui::prelude::Interaction>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::Interaction as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::Interaction,
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
            ::bevy_ui::prelude::Interaction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_scale_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::UiScale,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::prelude::UiScale,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_computed_ui_target_camera_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::ComputedUiTargetCamera,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::ComputedUiTargetCamera>| {
            let output: Val<::bevy_ui::prelude::ComputedUiTargetCamera> = {
                {
                    let output: Val<::bevy_ui::prelude::ComputedUiTargetCamera> =
                        <::bevy_ui::prelude::ComputedUiTargetCamera as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::prelude::ComputedUiTargetCamera>,
         other: Ref<::bevy_ui::prelude::ComputedUiTargetCamera>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::prelude::ComputedUiTargetCamera as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::ComputedUiTargetCamera,
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
            ::bevy_ui::prelude::ComputedUiTargetCamera,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_computed_ui_render_target_info_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::ComputedUiRenderTargetInfo,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::prelude::ComputedUiRenderTargetInfo>| {
                let output: Val<::bevy_ui::prelude::ComputedUiRenderTargetInfo> = {
                    {
                        let output: Val<
                            ::bevy_ui::prelude::ComputedUiRenderTargetInfo,
                        > = <::bevy_ui::prelude::ComputedUiRenderTargetInfo as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::ComputedUiRenderTargetInfo>,
                other: Ref<::bevy_ui::prelude::ComputedUiRenderTargetInfo>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::ComputedUiRenderTargetInfo as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::ComputedUiRenderTargetInfo,
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
            "scale_factor",
            |_self: Ref<::bevy_ui::prelude::ComputedUiRenderTargetInfo>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui::prelude::ComputedUiRenderTargetInfo::scale_factor(
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
            ::bevy_ui::prelude::ComputedUiRenderTargetInfo,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_content_size_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::measurement::ContentSize,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::measurement::ContentSize,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_global_transform_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::ui_transform::UiGlobalTransform,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::ui_transform::UiGlobalTransform>| {
            let output: Val<::bevy_ui::ui_transform::UiGlobalTransform> = {
                {
                    let output: Val<::bevy_ui::ui_transform::UiGlobalTransform> =
                        <::bevy_ui::ui_transform::UiGlobalTransform as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::ui_transform::UiGlobalTransform>,
         other: Ref<::bevy_ui::ui_transform::UiGlobalTransform>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::ui_transform::UiGlobalTransform as ::std::cmp::PartialEq<
                            ::bevy_ui::ui_transform::UiGlobalTransform,
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
            ::bevy_ui::ui_transform::UiGlobalTransform,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_calculated_clip_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::CalculatedClip,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::CalculatedClip>| {
            let output: Val<::bevy_ui::prelude::CalculatedClip> = {
                {
                    let output: Val<::bevy_ui::prelude::CalculatedClip> =
                        <::bevy_ui::prelude::CalculatedClip as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui::prelude::CalculatedClip,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_node_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::Node,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::prelude::Node>| {
                let output: Val<::bevy_ui::prelude::Node> = {
                    {
                        let output: Val<::bevy_ui::prelude::Node> = <::bevy_ui::prelude::Node as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::prelude::Node>, other: Ref<::bevy_ui::prelude::Node>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::Node as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::Node,
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
            ::bevy_ui::prelude::Node,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_overflow_axis_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::OverflowAxis,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::OverflowAxis>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::OverflowAxis as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::OverflowAxis>| {
                let output: Val<::bevy_ui::prelude::OverflowAxis> = {
                    {
                        let output: Val<::bevy_ui::prelude::OverflowAxis> = <::bevy_ui::prelude::OverflowAxis as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::OverflowAxis>,
                other: Ref<::bevy_ui::prelude::OverflowAxis>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::OverflowAxis as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::OverflowAxis,
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
            "is_visible",
            |_self: Ref<::bevy_ui::prelude::OverflowAxis>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ui::prelude::OverflowAxis::is_visible(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Overflow is visible on this axis",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::prelude::OverflowAxis,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_target_camera_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::UiTargetCamera,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::UiTargetCamera>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::UiTargetCamera as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::UiTargetCamera>| {
                let output: Val<::bevy_ui::prelude::UiTargetCamera> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiTargetCamera> = <::bevy_ui::prelude::UiTargetCamera as ::std::clone::Clone>::clone(
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
            "entity",
            |_self: Ref<::bevy_ui::prelude::UiTargetCamera>| {
                let output: Val<::bevy_ecs::entity::Entity> = {
                    {
                        let output: Val<::bevy_ecs::entity::Entity> = ::bevy_ui::prelude::UiTargetCamera::entity(
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
                _self: Ref<::bevy_ui::prelude::UiTargetCamera>,
                other: Ref<::bevy_ui::prelude::UiTargetCamera>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::UiTargetCamera as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::UiTargetCamera,
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
            ::bevy_ui::prelude::UiTargetCamera,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_computed_node_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::ComputedNode,
    >::new(world)
        .register_documented(
            "border",
            |_self: Ref<::bevy_ui::prelude::ComputedNode>| {
                let output: Val<::bevy_sprite::BorderRect> = {
                    {
                        let output: Val<::bevy_sprite::BorderRect> = ::bevy_ui::prelude::ComputedNode::border(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the thickness of the node's border on each edge in physical pixels.\n Automatically calculated by [`ui_layout_system`](`super::layout::ui_layout_system`).",
            &["_self"],
        )
        .register_documented(
            "border_radius",
            |_self: Ref<::bevy_ui::prelude::ComputedNode>| {
                let output: Val<::bevy_ui::prelude::ResolvedBorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::ResolvedBorderRadius> = ::bevy_ui::prelude::ComputedNode::border_radius(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the border radius for each of the node's corners in physical pixels.\n Automatically calculated by [`ui_layout_system`](`super::layout::ui_layout_system`).",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::prelude::ComputedNode>| {
                let output: Val<::bevy_ui::prelude::ComputedNode> = {
                    {
                        let output: Val<::bevy_ui::prelude::ComputedNode> = <::bevy_ui::prelude::ComputedNode as ::std::clone::Clone>::clone(
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
            "content_inset",
            |_self: Ref<::bevy_ui::prelude::ComputedNode>| {
                let output: Val<::bevy_sprite::BorderRect> = {
                    {
                        let output: Val<::bevy_sprite::BorderRect> = ::bevy_ui::prelude::ComputedNode::content_inset(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the combined inset on each edge including both padding and border thickness in physical pixels.",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_ui::prelude::ComputedNode>,
                other: Ref<::bevy_ui::prelude::ComputedNode>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::ComputedNode as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::ComputedNode,
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
            "inner_radius",
            |_self: Ref<::bevy_ui::prelude::ComputedNode>| {
                let output: Val<::bevy_ui::prelude::ResolvedBorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::ResolvedBorderRadius> = ::bevy_ui::prelude::ComputedNode::inner_radius(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the inner border radius for each of the node's corners in physical pixels.",
            &["_self"],
        )
        .register_documented(
            "inverse_scale_factor",
            |_self: Ref<::bevy_ui::prelude::ComputedNode>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui::prelude::ComputedNode::inverse_scale_factor(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the inverse of the scale factor for this node.\n To convert from physical coordinates to logical coordinates multiply by this value.",
            &["_self"],
        )
        .register_documented(
            "is_empty",
            |_self: Ref<::bevy_ui::prelude::ComputedNode>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ui::prelude::ComputedNode::is_empty(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Check if the node is empty.\n A node is considered empty if it has a zero or negative extent along either of its axes.",
            &["_self"],
        )
        .register_documented(
            "outline_offset",
            |_self: Ref<::bevy_ui::prelude::ComputedNode>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui::prelude::ComputedNode::outline_offset(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the amount of space between the outline and the edge of the node in physical pixels.\n Automatically calculated by [`ui_layout_system`](`super::layout::ui_layout_system`).",
            &["_self"],
        )
        .register_documented(
            "outline_radius",
            |_self: Ref<::bevy_ui::prelude::ComputedNode>| {
                let output: Val<::bevy_ui::prelude::ResolvedBorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::ResolvedBorderRadius> = ::bevy_ui::prelude::ComputedNode::outline_radius(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the border radius for each corner of the outline\n An outline's border radius is derived from the node's border-radius\n so that the outline wraps the border equally at all points.\n Automatically calculated by [`ui_layout_system`](`super::layout::ui_layout_system`).",
            &["_self"],
        )
        .register_documented(
            "outline_width",
            |_self: Ref<::bevy_ui::prelude::ComputedNode>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui::prelude::ComputedNode::outline_width(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the thickness of the UI node's outline in physical pixels.\n If this value is negative or `0.` then no outline will be rendered.\n Automatically calculated by [`ui_layout_system`](`super::layout::ui_layout_system`).",
            &["_self"],
        )
        .register_documented(
            "padding",
            |_self: Ref<::bevy_ui::prelude::ComputedNode>| {
                let output: Val<::bevy_sprite::BorderRect> = {
                    {
                        let output: Val<::bevy_sprite::BorderRect> = ::bevy_ui::prelude::ComputedNode::padding(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the thickness of the node's padding on each edge in physical pixels.\n Automatically calculated by [`ui_layout_system`](`super::layout::ui_layout_system`).",
            &["_self"],
        )
        .register_documented(
            "resolve_clip_rect",
            |
                _self: Ref<::bevy_ui::prelude::ComputedNode>,
                overflow: Val<::bevy_ui::prelude::Overflow>,
                overflow_clip_margin: Val<::bevy_ui::prelude::OverflowClipMargin>|
            {
                let output: Val<::bevy_math::Rect> = {
                    {
                        let output: Val<::bevy_math::Rect> = ::bevy_ui::prelude::ComputedNode::resolve_clip_rect(
                                &_self,
                                overflow.into_inner(),
                                overflow_clip_margin.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Resolve the node's clipping rect in local space",
            &["_self", "overflow", "overflow_clip_margin"],
        )
        .register_documented(
            "stack_index",
            |_self: Ref<::bevy_ui::prelude::ComputedNode>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_ui::prelude::ComputedNode::stack_index(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The order of the node in the UI layout.\n Nodes with a higher stack index are drawn on top of and receive interactions before nodes with lower stack indices.\n Automatically calculated in [`UiSystems::Stack`](super::UiSystems::Stack).",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::prelude::ComputedNode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_overflow_clip_box_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::OverflowClipBox,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::OverflowClipBox>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::OverflowClipBox as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::OverflowClipBox>| {
                let output: Val<::bevy_ui::prelude::OverflowClipBox> = {
                    {
                        let output: Val<::bevy_ui::prelude::OverflowClipBox> = <::bevy_ui::prelude::OverflowClipBox as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::OverflowClipBox>,
                other: Ref<::bevy_ui::prelude::OverflowClipBox>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::OverflowClipBox as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::OverflowClipBox,
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
            ::bevy_ui::prelude::OverflowClipBox,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_focus_policy_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::FocusPolicy,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::FocusPolicy>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::FocusPolicy as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::FocusPolicy>| {
                let output: Val<::bevy_ui::FocusPolicy> = {
                    {
                        let output: Val<::bevy_ui::FocusPolicy> = <::bevy_ui::FocusPolicy as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::FocusPolicy>, other: Ref<::bevy_ui::FocusPolicy>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::FocusPolicy as ::std::cmp::PartialEq<
                            ::bevy_ui::FocusPolicy,
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
        .register_type_data::<::bevy_ui::FocusPolicy, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_image_node_size_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::widget::ImageNodeSize,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::widget::ImageNodeSize>| {
            let output: Val<::bevy_ui::widget::ImageNodeSize> = {
                {
                    let output: Val<::bevy_ui::widget::ImageNodeSize> =
                        <::bevy_ui::widget::ImageNodeSize as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui::widget::ImageNodeSize,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_node_flags_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::widget::TextNodeFlags,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::widget::TextNodeFlags>| {
            let output: Val<::bevy_ui::widget::TextNodeFlags> = {
                {
                    let output: Val<::bevy_ui::widget::TextNodeFlags> =
                        <::bevy_ui::widget::TextNodeFlags as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui::widget::TextNodeFlags,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_position_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::UiPosition,
    >::new(world)
    .register_documented(
        "at",
        |_self: Val<::bevy_ui::prelude::UiPosition>,
         x: Val<::bevy_ui::prelude::Val>,
         y: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::at(
                            _self.into_inner(),
                            x.into_inner(),
                            y.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Creates a position from self with the given `x` and `y` coordinates",
        &["_self", "x", "y"],
    )
    .register_documented(
        "at_percent",
        |_self: Val<::bevy_ui::prelude::UiPosition>, x: f32, y: f32| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::at_percent(_self.into_inner(), x, y).into();
                    output
                }
            };
            output
        },
        " Creates a percentage position from self with the given `x` and `y` coordinates",
        &["_self", "x", "y"],
    )
    .register_documented(
        "at_px",
        |_self: Val<::bevy_ui::prelude::UiPosition>, x: f32, y: f32| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::at_px(_self.into_inner(), x, y).into();
                    output
                }
            };
            output
        },
        " Creates a position in logical pixels from self with the given `x` and `y` coordinates",
        &["_self", "x", "y"],
    )
    .register_documented(
        "at_x",
        |_self: Val<::bevy_ui::prelude::UiPosition>, x: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::at_x(_self.into_inner(), x.into_inner())
                            .into();
                    output
                }
            };
            output
        },
        " Creates a position from self with the given `x` coordinate",
        &["_self", "x"],
    )
    .register_documented(
        "at_y",
        |_self: Val<::bevy_ui::prelude::UiPosition>, y: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::at_y(_self.into_inner(), y.into_inner())
                            .into();
                    output
                }
            };
            output
        },
        " Creates a position from self with the given `y` coordinate",
        &["_self", "y"],
    )
    .register_documented(
        "bottom",
        |x: Val<::bevy_ui::prelude::Val>, y: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::bottom(x.into_inner(), y.into_inner())
                            .into();
                    output
                }
            };
            output
        },
        " Position relative to the bottom edge",
        &["x", "y"],
    )
    .register_documented(
        "bottom_left",
        |x: Val<::bevy_ui::prelude::Val>, y: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::bottom_left(x.into_inner(), y.into_inner())
                            .into();
                    output
                }
            };
            output
        },
        " Position relative to the bottom-left corner",
        &["x", "y"],
    )
    .register_documented(
        "bottom_right",
        |x: Val<::bevy_ui::prelude::Val>, y: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::bottom_right(
                            x.into_inner(),
                            y.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Position relative to the bottom-right corner",
        &["x", "y"],
    )
    .register_documented(
        "center",
        |x: Val<::bevy_ui::prelude::Val>, y: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::center(x.into_inner(), y.into_inner())
                            .into();
                    output
                }
            };
            output
        },
        " Position relative to the center",
        &["x", "y"],
    )
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::UiPosition>| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        <::bevy_ui::prelude::UiPosition as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_ui::prelude::UiPosition>, other: Ref<::bevy_ui::prelude::UiPosition>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_ui::prelude::UiPosition as ::std::cmp::PartialEq<
                        ::bevy_ui::prelude::UiPosition,
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
        "left",
        |x: Val<::bevy_ui::prelude::Val>, y: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::left(x.into_inner(), y.into_inner()).into();
                    output
                }
            };
            output
        },
        " Position relative to the left edge",
        &["x", "y"],
    )
    .register_documented(
        "right",
        |x: Val<::bevy_ui::prelude::Val>, y: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::right(x.into_inner(), y.into_inner())
                            .into();
                    output
                }
            };
            output
        },
        " Position relative to the right edge",
        &["x", "y"],
    )
    .register_documented(
        "top",
        |x: Val<::bevy_ui::prelude::Val>, y: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::top(x.into_inner(), y.into_inner()).into();
                    output
                }
            };
            output
        },
        " Position relative to the top edge",
        &["x", "y"],
    )
    .register_documented(
        "top_left",
        |x: Val<::bevy_ui::prelude::Val>, y: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::top_left(x.into_inner(), y.into_inner())
                            .into();
                    output
                }
            };
            output
        },
        " Position relative to the top-left corner",
        &["x", "y"],
    )
    .register_documented(
        "top_right",
        |x: Val<::bevy_ui::prelude::Val>, y: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::prelude::UiPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::UiPosition> =
                        ::bevy_ui::prelude::UiPosition::top_right(x.into_inner(), y.into_inner())
                            .into();
                    output
                }
            };
            output
        },
        " Position relative to the top-right corner",
        &["x", "y"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::prelude::UiPosition,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_val_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::Val,
    >::new(world)
        .register_documented(
            "all",
            |_self: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::Val::all(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a [`UiRect`] with all its fields equal to this value.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = Val::Px(1.).all();\n assert_eq!(ui_rect.left, Val::Px(1.));\n assert_eq!(ui_rect.right, Val::Px(1.));\n assert_eq!(ui_rect.top, Val::Px(1.));\n assert_eq!(ui_rect.bottom, Val::Px(1.));\n ```",
            &["_self"],
        )
        .register_documented(
            "bottom",
            |_self: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::Val::bottom(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a [`UiRect`] with its `bottom` equal to this value,\n and all other fields set to `Val::ZERO`.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = Val::Px(1.).bottom();\n assert_eq!(ui_rect.left, Val::ZERO);\n assert_eq!(ui_rect.right, Val::ZERO);\n assert_eq!(ui_rect.top, Val::ZERO);\n assert_eq!(ui_rect.bottom, Val::Px(1.));\n ```",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::Val> = {
                    {
                        let output: Val<::bevy_ui::prelude::Val> = <::bevy_ui::prelude::Val as ::std::clone::Clone>::clone(
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
            |_self: Val<::bevy_ui::prelude::Val>, rhs: f32| {
                let output: Val<::bevy_ui::prelude::Val> = {
                    {
                        let output: Val<::bevy_ui::prelude::Val> = <::bevy_ui::prelude::Val as ::std::ops::Div<
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
            |_self: Ref<::bevy_ui::prelude::Val>, other: Ref<::bevy_ui::prelude::Val>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::Val as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::Val,
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
            "horizontal",
            |_self: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::Val::horizontal(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a [`UiRect`] with all its `left` and `right` equal to this value,\n and its `top` and `bottom` set to `Val::ZERO`.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = Val::Px(1.).horizontal();\n assert_eq!(ui_rect.left, Val::Px(1.));\n assert_eq!(ui_rect.right, Val::Px(1.));\n assert_eq!(ui_rect.top, Val::ZERO);\n assert_eq!(ui_rect.bottom, Val::ZERO);\n ```",
            &["_self"],
        )
        .register_documented(
            "left",
            |_self: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::Val::left(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a [`UiRect`] with its `left` equal to this value,\n and all other fields set to `Val::ZERO`.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = Val::Px(1.).left();\n assert_eq!(ui_rect.left, Val::Px(1.));\n assert_eq!(ui_rect.right, Val::ZERO);\n assert_eq!(ui_rect.top, Val::ZERO);\n assert_eq!(ui_rect.bottom, Val::ZERO);\n ```",
            &["_self"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_ui::prelude::Val>, rhs: f32| {
                let output: Val<::bevy_ui::prelude::Val> = {
                    {
                        let output: Val<::bevy_ui::prelude::Val> = <::bevy_ui::prelude::Val as ::std::ops::Mul<
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
            |_self: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::Val> = {
                    {
                        let output: Val<::bevy_ui::prelude::Val> = <::bevy_ui::prelude::Val as ::std::ops::Neg>::neg(
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
            "right",
            |_self: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::Val::right(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a [`UiRect`] with its `right` equal to this value,\n and all other fields set to `Val::ZERO`.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = Val::Px(1.).right();\n assert_eq!(ui_rect.left, Val::ZERO);\n assert_eq!(ui_rect.right, Val::Px(1.));\n assert_eq!(ui_rect.top, Val::ZERO);\n assert_eq!(ui_rect.bottom, Val::ZERO);\n ```",
            &["_self"],
        )
        .register_documented(
            "top",
            |_self: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::Val::top(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a [`UiRect`] with its `top` equal to this value,\n and all other fields set to `Val::ZERO`.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = Val::Px(1.).top();\n assert_eq!(ui_rect.left, Val::ZERO);\n assert_eq!(ui_rect.right, Val::ZERO);\n assert_eq!(ui_rect.top, Val::Px(1.));\n assert_eq!(ui_rect.bottom, Val::ZERO);\n ```",
            &["_self"],
        )
        .register_documented(
            "vertical",
            |_self: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::Val::vertical(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a [`UiRect`] with all its `top` and `bottom` equal to this value,\n and its `left` and `right` set to `Val::ZERO`.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = Val::Px(1.).vertical();\n assert_eq!(ui_rect.left, Val::ZERO);\n assert_eq!(ui_rect.right, Val::ZERO);\n assert_eq!(ui_rect.top, Val::Px(1.));\n assert_eq!(ui_rect.bottom, Val::Px(1.));\n ```",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::prelude::Val,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_color_stop_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::gradients::ColorStop,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::gradients::ColorStop>| {
            let output: Val<::bevy_ui::gradients::ColorStop> = {
                {
                    let output: Val<::bevy_ui::gradients::ColorStop> =
                        <::bevy_ui::gradients::ColorStop as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_ui::gradients::ColorStop>,
         other: Ref<::bevy_ui::gradients::ColorStop>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_ui::gradients::ColorStop as ::std::cmp::PartialEq<
                        ::bevy_ui::gradients::ColorStop,
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
        "with_hint",
        |_self: Val<::bevy_ui::gradients::ColorStop>, hint: f32| {
            let output: Val<::bevy_ui::gradients::ColorStop> = {
                {
                    let output: Val<::bevy_ui::gradients::ColorStop> =
                        ::bevy_ui::gradients::ColorStop::with_hint(_self.into_inner(), hint).into();
                    output
                }
            };
            output
        },
        "",
        &["_self", "hint"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::gradients::ColorStop,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_angular_color_stop_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::gradients::AngularColorStop,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::gradients::AngularColorStop>| {
            let output: Val<::bevy_ui::gradients::AngularColorStop> = {
                {
                    let output: Val<::bevy_ui::gradients::AngularColorStop> =
                        <::bevy_ui::gradients::AngularColorStop as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::gradients::AngularColorStop>,
         other: Ref<::bevy_ui::gradients::AngularColorStop>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::gradients::AngularColorStop as ::std::cmp::PartialEq<
                            ::bevy_ui::gradients::AngularColorStop,
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
        "with_hint",
        |_self: Val<::bevy_ui::gradients::AngularColorStop>, hint: f32| {
            let output: Val<::bevy_ui::gradients::AngularColorStop> = {
                {
                    let output: Val<::bevy_ui::gradients::AngularColorStop> =
                        ::bevy_ui::gradients::AngularColorStop::with_hint(_self.into_inner(), hint)
                            .into();
                    output
                }
            };
            output
        },
        "",
        &["_self", "hint"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::gradients::AngularColorStop,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_linear_gradient_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::gradients::LinearGradient,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::gradients::LinearGradient>| {
            let output: Val<::bevy_ui::gradients::LinearGradient> = {
                {
                    let output: Val<::bevy_ui::gradients::LinearGradient> =
                        <::bevy_ui::gradients::LinearGradient as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::gradients::LinearGradient>,
         other: Ref<::bevy_ui::gradients::LinearGradient>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::gradients::LinearGradient as ::std::cmp::PartialEq<
                            ::bevy_ui::gradients::LinearGradient,
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
        "in_color_space",
        |_self: Val<::bevy_ui::gradients::LinearGradient>,
         color_space: Val<::bevy_ui::gradients::InterpolationColorSpace>| {
            let output: Val<::bevy_ui::gradients::LinearGradient> = {
                {
                    let output: Val<::bevy_ui::gradients::LinearGradient> =
                        ::bevy_ui::gradients::LinearGradient::in_color_space(
                            _self.into_inner(),
                            color_space.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        "",
        &["_self", "color_space"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::gradients::LinearGradient,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_interpolation_color_space_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::gradients::InterpolationColorSpace,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::gradients::InterpolationColorSpace>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::gradients::InterpolationColorSpace as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::gradients::InterpolationColorSpace>| {
                let output: Val<::bevy_ui::gradients::InterpolationColorSpace> = {
                    {
                        let output: Val<::bevy_ui::gradients::InterpolationColorSpace> = <::bevy_ui::gradients::InterpolationColorSpace as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::gradients::InterpolationColorSpace>,
                other: Ref<::bevy_ui::gradients::InterpolationColorSpace>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::gradients::InterpolationColorSpace as ::std::cmp::PartialEq<
                            ::bevy_ui::gradients::InterpolationColorSpace,
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
            ::bevy_ui::gradients::InterpolationColorSpace,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_radial_gradient_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::gradients::RadialGradient,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::gradients::RadialGradient>| {
            let output: Val<::bevy_ui::gradients::RadialGradient> = {
                {
                    let output: Val<::bevy_ui::gradients::RadialGradient> =
                        <::bevy_ui::gradients::RadialGradient as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::gradients::RadialGradient>,
         other: Ref<::bevy_ui::gradients::RadialGradient>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::gradients::RadialGradient as ::std::cmp::PartialEq<
                            ::bevy_ui::gradients::RadialGradient,
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
        "in_color_space",
        |_self: Val<::bevy_ui::gradients::RadialGradient>,
         color_space: Val<::bevy_ui::gradients::InterpolationColorSpace>| {
            let output: Val<::bevy_ui::gradients::RadialGradient> = {
                {
                    let output: Val<::bevy_ui::gradients::RadialGradient> =
                        ::bevy_ui::gradients::RadialGradient::in_color_space(
                            _self.into_inner(),
                            color_space.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        "",
        &["_self", "color_space"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::gradients::RadialGradient,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_radial_gradient_shape_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::gradients::RadialGradientShape,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::gradients::RadialGradientShape>| {
            let output: Val<::bevy_ui::gradients::RadialGradientShape> = {
                {
                    let output: Val<::bevy_ui::gradients::RadialGradientShape> =
                        <::bevy_ui::gradients::RadialGradientShape as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::gradients::RadialGradientShape>,
         other: Ref<::bevy_ui::gradients::RadialGradientShape>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::gradients::RadialGradientShape as ::std::cmp::PartialEq<
                            ::bevy_ui::gradients::RadialGradientShape,
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
            ::bevy_ui::gradients::RadialGradientShape,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_conic_gradient_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::gradients::ConicGradient,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::gradients::ConicGradient>| {
            let output: Val<::bevy_ui::gradients::ConicGradient> = {
                {
                    let output: Val<::bevy_ui::gradients::ConicGradient> =
                        <::bevy_ui::gradients::ConicGradient as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_ui::gradients::ConicGradient>,
         other: Ref<::bevy_ui::gradients::ConicGradient>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::gradients::ConicGradient as ::std::cmp::PartialEq<
                            ::bevy_ui::gradients::ConicGradient,
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
        "in_color_space",
        |_self: Val<::bevy_ui::gradients::ConicGradient>,
         color_space: Val<::bevy_ui::gradients::InterpolationColorSpace>| {
            let output: Val<::bevy_ui::gradients::ConicGradient> = {
                {
                    let output: Val<::bevy_ui::gradients::ConicGradient> =
                        ::bevy_ui::gradients::ConicGradient::in_color_space(
                            _self.into_inner(),
                            color_space.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        "",
        &["_self", "color_space"],
    )
    .register_documented(
        "with_position",
        |_self: Val<::bevy_ui::gradients::ConicGradient>,
         position: Val<::bevy_ui::prelude::UiPosition>| {
            let output: Val<::bevy_ui::gradients::ConicGradient> = {
                {
                    let output: Val<::bevy_ui::gradients::ConicGradient> =
                        ::bevy_ui::gradients::ConicGradient::with_position(
                            _self.into_inner(),
                            position.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Sets the position of the gradient",
        &["_self", "position"],
    )
    .register_documented(
        "with_start",
        |_self: Val<::bevy_ui::gradients::ConicGradient>, start: f32| {
            let output: Val<::bevy_ui::gradients::ConicGradient> = {
                {
                    let output: Val<::bevy_ui::gradients::ConicGradient> =
                        ::bevy_ui::gradients::ConicGradient::with_start(_self.into_inner(), start)
                            .into();
                    output
                }
            };
            output
        },
        " Sets the starting angle of the gradient in radians",
        &["_self", "start"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::gradients::ConicGradient,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gradient_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::gradients::Gradient,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::gradients::Gradient>| {
            let output: Val<::bevy_ui::gradients::Gradient> = {
                {
                    let output: Val<::bevy_ui::gradients::Gradient> =
                        <::bevy_ui::gradients::Gradient as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_ui::gradients::Gradient>, other: Ref<::bevy_ui::gradients::Gradient>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_ui::gradients::Gradient as ::std::cmp::PartialEq<
                        ::bevy_ui::gradients::Gradient,
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
        "is_empty",
        |_self: Ref<::bevy_ui::gradients::Gradient>| {
            let output: bool = {
                {
                    let output: bool = ::bevy_ui::gradients::Gradient::is_empty(&_self).into();
                    output
                }
            };
            output
        },
        " Returns true if the gradient has no stops.",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::gradients::Gradient,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_background_gradient_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::gradients::BackgroundGradient,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::gradients::BackgroundGradient>| {
            let output: Val<::bevy_ui::gradients::BackgroundGradient> = {
                {
                    let output: Val<::bevy_ui::gradients::BackgroundGradient> =
                        <::bevy_ui::gradients::BackgroundGradient as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::gradients::BackgroundGradient>,
         other: Ref<::bevy_ui::gradients::BackgroundGradient>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::gradients::BackgroundGradient as ::std::cmp::PartialEq<
                            ::bevy_ui::gradients::BackgroundGradient,
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
            ::bevy_ui::gradients::BackgroundGradient,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_border_gradient_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::gradients::BorderGradient,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::gradients::BorderGradient>| {
            let output: Val<::bevy_ui::gradients::BorderGradient> = {
                {
                    let output: Val<::bevy_ui::gradients::BorderGradient> =
                        <::bevy_ui::gradients::BorderGradient as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::gradients::BorderGradient>,
         other: Ref<::bevy_ui::gradients::BorderGradient>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::gradients::BorderGradient as ::std::cmp::PartialEq<
                            ::bevy_ui::gradients::BorderGradient,
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
            ::bevy_ui::gradients::BorderGradient,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_val_2_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::ui_transform::Val2,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::ui_transform::Val2>| {
            let output: Val<::bevy_ui::ui_transform::Val2> = {
                {
                    let output: Val<::bevy_ui::ui_transform::Val2> =
                        <::bevy_ui::ui_transform::Val2 as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_ui::ui_transform::Val2>, other: Ref<::bevy_ui::ui_transform::Val2>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_ui::ui_transform::Val2 as ::std::cmp::PartialEq<
                        ::bevy_ui::ui_transform::Val2,
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
        "new",
        |x: Val<::bevy_ui::prelude::Val>, y: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::ui_transform::Val2> = {
                {
                    let output: Val<::bevy_ui::ui_transform::Val2> =
                        ::bevy_ui::ui_transform::Val2::new(x.into_inner(), y.into_inner()).into();
                    output
                }
            };
            output
        },
        " Creates a new [`Val2`]",
        &["x", "y"],
    )
    .register_documented(
        "percent",
        |x: f32, y: f32| {
            let output: Val<::bevy_ui::ui_transform::Val2> = {
                {
                    let output: Val<::bevy_ui::ui_transform::Val2> =
                        ::bevy_ui::ui_transform::Val2::percent(x, y).into();
                    output
                }
            };
            output
        },
        " Creates a new [`Val2`] where both components are percentage values",
        &["x", "y"],
    )
    .register_documented(
        "px",
        |x: f32, y: f32| {
            let output: Val<::bevy_ui::ui_transform::Val2> = {
                {
                    let output: Val<::bevy_ui::ui_transform::Val2> =
                        ::bevy_ui::ui_transform::Val2::px(x, y).into();
                    output
                }
            };
            output
        },
        " Creates a new [`Val2`] where both components are in logical pixels",
        &["x", "y"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::ui_transform::Val2,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_transform_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::ui_transform::UiTransform,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::ui_transform::UiTransform>| {
            let output: Val<::bevy_ui::ui_transform::UiTransform> = {
                {
                    let output: Val<::bevy_ui::ui_transform::UiTransform> =
                        <::bevy_ui::ui_transform::UiTransform as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::ui_transform::UiTransform>,
         other: Ref<::bevy_ui::ui_transform::UiTransform>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::ui_transform::UiTransform as ::std::cmp::PartialEq<
                            ::bevy_ui::ui_transform::UiTransform,
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
        "from_rotation",
        |rotation: Val<::bevy_math::Rot2>| {
            let output: Val<::bevy_ui::ui_transform::UiTransform> = {
                {
                    let output: Val<::bevy_ui::ui_transform::UiTransform> =
                        ::bevy_ui::ui_transform::UiTransform::from_rotation(rotation.into_inner())
                            .into();
                    output
                }
            };
            output
        },
        " Creates a UI transform representing a rotation.",
        &["rotation"],
    )
    .register_documented(
        "from_translation",
        |translation: Val<::bevy_ui::ui_transform::Val2>| {
            let output: Val<::bevy_ui::ui_transform::UiTransform> = {
                {
                    let output: Val<::bevy_ui::ui_transform::UiTransform> =
                        ::bevy_ui::ui_transform::UiTransform::from_translation(
                            translation.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Creates a UI transform representing a responsive translation.",
        &["translation"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::ui_transform::UiTransform,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_relative_cursor_position_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::RelativeCursorPosition,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::RelativeCursorPosition>| {
            let output: Val<::bevy_ui::RelativeCursorPosition> = {
                {
                    let output: Val<::bevy_ui::RelativeCursorPosition> =
                        <::bevy_ui::RelativeCursorPosition as ::std::clone::Clone>::clone(&_self)
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
        "cursor_over",
        |_self: Ref<::bevy_ui::RelativeCursorPosition>| {
            let output: bool = {
                {
                    let output: bool =
                        ::bevy_ui::RelativeCursorPosition::cursor_over(&_self).into();
                    output
                }
            };
            output
        },
        " A helper function to check if the mouse is over the node",
        &["_self"],
    )
    .register_documented(
        "eq",
        |_self: Ref<::bevy_ui::RelativeCursorPosition>,
         other: Ref<::bevy_ui::RelativeCursorPosition>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::RelativeCursorPosition as ::std::cmp::PartialEq<
                            ::bevy_ui::RelativeCursorPosition,
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
            ::bevy_ui::RelativeCursorPosition,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_rect_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::UiRect,
    >::new(world)
        .register_documented(
            "all",
            |value: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::all(
                                value.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`UiRect`] where all sides have the same value.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::all(Val::Px(10.0));\n assert_eq!(ui_rect.left, Val::Px(10.0));\n assert_eq!(ui_rect.right, Val::Px(10.0));\n assert_eq!(ui_rect.top, Val::Px(10.0));\n assert_eq!(ui_rect.bottom, Val::Px(10.0));\n ```",
            &["value"],
        )
        .register_documented(
            "axes",
            |
                horizontal: Val<::bevy_ui::prelude::Val>,
                vertical: Val<::bevy_ui::prelude::Val>|
            {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::axes(
                                horizontal.into_inner(),
                                vertical.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`UiRect`] where both `left` and `right` take the value of `horizontal`, and both `top` and `bottom` take the value of `vertical`.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::axes(Val::Px(10.0), Val::Percent(15.0));\n assert_eq!(ui_rect.left, Val::Px(10.0));\n assert_eq!(ui_rect.right, Val::Px(10.0));\n assert_eq!(ui_rect.top, Val::Percent(15.0));\n assert_eq!(ui_rect.bottom, Val::Percent(15.0));\n ```",
            &["horizontal", "vertical"],
        )
        .register_documented(
            "bottom",
            |bottom: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::bottom(
                                bottom.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`UiRect`] where `bottom` takes the given value,\n and the other fields are set to `Val::ZERO`.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::bottom(Val::Px(10.0));\n assert_eq!(ui_rect.left, Val::ZERO);\n assert_eq!(ui_rect.right, Val::ZERO);\n assert_eq!(ui_rect.top, Val::ZERO);\n assert_eq!(ui_rect.bottom, Val::Px(10.0));\n ```",
            &["bottom"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::prelude::UiRect>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = <::bevy_ui::prelude::UiRect as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::UiRect>,
                other: Ref<::bevy_ui::prelude::UiRect>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::UiRect as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::UiRect,
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
            "horizontal",
            |value: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::horizontal(
                                value.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`UiRect`] where `left` and `right` take the given value,\n and `top` and `bottom` set to zero `Val::ZERO`.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::horizontal(Val::Px(10.0));\n assert_eq!(ui_rect.left, Val::Px(10.0));\n assert_eq!(ui_rect.right, Val::Px(10.0));\n assert_eq!(ui_rect.top, Val::ZERO);\n assert_eq!(ui_rect.bottom, Val::ZERO);\n ```",
            &["value"],
        )
        .register_documented(
            "left",
            |left: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::left(
                                left.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`UiRect`] where `left` takes the given value, and\n the other fields are set to `Val::ZERO`.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::left(Val::Px(10.0));\n assert_eq!(ui_rect.left, Val::Px(10.0));\n assert_eq!(ui_rect.right, Val::ZERO);\n assert_eq!(ui_rect.top, Val::ZERO);\n assert_eq!(ui_rect.bottom, Val::ZERO);\n ```",
            &["left"],
        )
        .register_documented(
            "new",
            |
                left: Val<::bevy_ui::prelude::Val>,
                right: Val<::bevy_ui::prelude::Val>,
                top: Val<::bevy_ui::prelude::Val>,
                bottom: Val<::bevy_ui::prelude::Val>|
            {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::new(
                                left.into_inner(),
                                right.into_inner(),
                                top.into_inner(),
                                bottom.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`UiRect`] from the values specified.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::new(\n     Val::Px(10.0),\n     Val::Px(20.0),\n     Val::Px(30.0),\n     Val::Px(40.0),\n );\n assert_eq!(ui_rect.left, Val::Px(10.0));\n assert_eq!(ui_rect.right, Val::Px(20.0));\n assert_eq!(ui_rect.top, Val::Px(30.0));\n assert_eq!(ui_rect.bottom, Val::Px(40.0));\n ```",
            &["left", "right", "top", "bottom"],
        )
        .register_documented(
            "percent",
            |left: f32, right: f32, top: f32, bottom: f32| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::percent(
                                left,
                                right,
                                top,
                                bottom,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`UiRect`] from the values specified in percentages.\n This is a shortcut for [`UiRect::new()`], applying [`Val::Percent`] to all arguments.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::percent(5., 10., 2., 1.);\n assert_eq!(ui_rect.left, Val::Percent(5.));\n assert_eq!(ui_rect.right, Val::Percent(10.));\n assert_eq!(ui_rect.top, Val::Percent(2.));\n assert_eq!(ui_rect.bottom, Val::Percent(1.));\n ```",
            &["left", "right", "top", "bottom"],
        )
        .register_documented(
            "px",
            |left: f32, right: f32, top: f32, bottom: f32| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::px(
                                left,
                                right,
                                top,
                                bottom,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`UiRect`] from the values specified in logical pixels.\n This is a shortcut for [`UiRect::new()`], applying [`Val::Px`] to all arguments.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::px(10., 20., 30., 40.);\n assert_eq!(ui_rect.left, Val::Px(10.));\n assert_eq!(ui_rect.right, Val::Px(20.));\n assert_eq!(ui_rect.top, Val::Px(30.));\n assert_eq!(ui_rect.bottom, Val::Px(40.));\n ```",
            &["left", "right", "top", "bottom"],
        )
        .register_documented(
            "right",
            |right: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::right(
                                right.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`UiRect`] where `right` takes the given value,\n and the other fields are set to `Val::ZERO`.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::right(Val::Px(10.0));\n assert_eq!(ui_rect.left, Val::ZERO);\n assert_eq!(ui_rect.right, Val::Px(10.0));\n assert_eq!(ui_rect.top, Val::ZERO);\n assert_eq!(ui_rect.bottom, Val::ZERO);\n ```",
            &["right"],
        )
        .register_documented(
            "top",
            |top: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::top(
                                top.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`UiRect`] where `top` takes the given value,\n and the other fields are set to `Val::ZERO`.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::top(Val::Px(10.0));\n assert_eq!(ui_rect.left, Val::ZERO);\n assert_eq!(ui_rect.right, Val::ZERO);\n assert_eq!(ui_rect.top, Val::Px(10.0));\n assert_eq!(ui_rect.bottom, Val::ZERO);\n ```",
            &["top"],
        )
        .register_documented(
            "vertical",
            |value: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::vertical(
                                value.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`UiRect`] where `top` and `bottom` take the given value,\n and `left` and `right` are set to `Val::ZERO`.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::vertical(Val::Px(10.0));\n assert_eq!(ui_rect.left, Val::ZERO);\n assert_eq!(ui_rect.right, Val::ZERO);\n assert_eq!(ui_rect.top, Val::Px(10.0));\n assert_eq!(ui_rect.bottom, Val::Px(10.0));\n ```",
            &["value"],
        )
        .register_documented(
            "with_bottom",
            |
                _self: Val<::bevy_ui::prelude::UiRect>,
                bottom: Val<::bevy_ui::prelude::Val>|
            {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::with_bottom(
                                _self.into_inner(),
                                bottom.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`UiRect`] with its `bottom` field set to the given value.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::all(Val::Px(20.0)).with_bottom(Val::Px(10.0));\n assert_eq!(ui_rect.left, Val::Px(20.0));\n assert_eq!(ui_rect.right, Val::Px(20.0));\n assert_eq!(ui_rect.top, Val::Px(20.0));\n assert_eq!(ui_rect.bottom, Val::Px(10.0));\n ```",
            &["_self", "bottom"],
        )
        .register_documented(
            "with_left",
            |_self: Val<::bevy_ui::prelude::UiRect>, left: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::with_left(
                                _self.into_inner(),
                                left.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`UiRect`] with its `left` field set to the given value.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::all(Val::Px(20.0)).with_left(Val::Px(10.0));\n assert_eq!(ui_rect.left, Val::Px(10.0));\n assert_eq!(ui_rect.right, Val::Px(20.0));\n assert_eq!(ui_rect.top, Val::Px(20.0));\n assert_eq!(ui_rect.bottom, Val::Px(20.0));\n ```",
            &["_self", "left"],
        )
        .register_documented(
            "with_right",
            |
                _self: Val<::bevy_ui::prelude::UiRect>,
                right: Val<::bevy_ui::prelude::Val>|
            {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::with_right(
                                _self.into_inner(),
                                right.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`UiRect`] with its `right` field set to the given value.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::all(Val::Px(20.0)).with_right(Val::Px(10.0));\n assert_eq!(ui_rect.left, Val::Px(20.0));\n assert_eq!(ui_rect.right, Val::Px(10.0));\n assert_eq!(ui_rect.top, Val::Px(20.0));\n assert_eq!(ui_rect.bottom, Val::Px(20.0));\n ```",
            &["_self", "right"],
        )
        .register_documented(
            "with_top",
            |_self: Val<::bevy_ui::prelude::UiRect>, top: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::UiRect> = {
                    {
                        let output: Val<::bevy_ui::prelude::UiRect> = ::bevy_ui::prelude::UiRect::with_top(
                                _self.into_inner(),
                                top.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`UiRect`] with its `top` field set to the given value.\n # Example\n ```\n # use bevy_ui::{UiRect, Val};\n #\n let ui_rect = UiRect::all(Val::Px(20.0)).with_top(Val::Px(10.0));\n assert_eq!(ui_rect.left, Val::Px(20.0));\n assert_eq!(ui_rect.right, Val::Px(20.0));\n assert_eq!(ui_rect.top, Val::Px(10.0));\n assert_eq!(ui_rect.bottom, Val::Px(20.0));\n ```",
            &["_self", "top"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::prelude::UiRect,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_border_radius_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::BorderRadius,
    >::new(world)
        .register_documented(
            "all",
            |radius: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::all(
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Set all four corners to the same curvature.",
            &["radius"],
        )
        .register_documented(
            "bottom",
            |radius: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::bottom(
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the radii for the bottom left and bottom right corners.\n Remaining corners will be right-angled.",
            &["radius"],
        )
        .register_documented(
            "bottom_left",
            |radius: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::bottom_left(
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the radius for the bottom left corner.\n Remaining corners will be right-angled.",
            &["radius"],
        )
        .register_documented(
            "bottom_right",
            |radius: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::bottom_right(
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the radius for the bottom right corner.\n Remaining corners will be right-angled.",
            &["radius"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::prelude::BorderRadius>| {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = <::bevy_ui::prelude::BorderRadius as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::BorderRadius>,
                other: Ref<::bevy_ui::prelude::BorderRadius>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::BorderRadius as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::BorderRadius,
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
            "left",
            |radius: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::left(
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the radii for the top left and bottom left corners.\n Remaining corners will be right-angled.",
            &["radius"],
        )
        .register_documented(
            "new",
            |
                top_left: Val<::bevy_ui::prelude::Val>,
                top_right: Val<::bevy_ui::prelude::Val>,
                bottom_right: Val<::bevy_ui::prelude::Val>,
                bottom_left: Val<::bevy_ui::prelude::Val>|
            {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::new(
                                top_left.into_inner(),
                                top_right.into_inner(),
                                bottom_right.into_inner(),
                                bottom_left.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["top_left", "top_right", "bottom_right", "bottom_left"],
        )
        .register_documented(
            "percent",
            |top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32| {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::percent(
                                top_left,
                                top_right,
                                bottom_right,
                                bottom_left,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the radii to percentage values.",
            &["top_left", "top_right", "bottom_right", "bottom_left"],
        )
        .register_documented(
            "px",
            |top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32| {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::px(
                                top_left,
                                top_right,
                                bottom_right,
                                bottom_left,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the radii to logical pixel values.",
            &["top_left", "top_right", "bottom_right", "bottom_left"],
        )
        .register_documented(
            "right",
            |radius: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::right(
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the radii for the top right and bottom right corners.\n Remaining corners will be right-angled.",
            &["radius"],
        )
        .register_documented(
            "top",
            |radius: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::top(
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the radii for the top left and top right corners.\n Remaining corners will be right-angled.",
            &["radius"],
        )
        .register_documented(
            "top_left",
            |radius: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::top_left(
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the radius for the top left corner.\n Remaining corners will be right-angled.",
            &["radius"],
        )
        .register_documented(
            "top_right",
            |radius: Val<::bevy_ui::prelude::Val>| {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::top_right(
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the radius for the top right corner.\n Remaining corners will be right-angled.",
            &["radius"],
        )
        .register_documented(
            "with_bottom",
            |
                _self: Val<::bevy_ui::prelude::BorderRadius>,
                radius: Val<::bevy_ui::prelude::Val>|
            {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::with_bottom(
                                _self.into_inner(),
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`BorderRadius`] with its `bottom_left` and `bottom_right` fields set to the given value.",
            &["_self", "radius"],
        )
        .register_documented(
            "with_bottom_left",
            |
                _self: Val<::bevy_ui::prelude::BorderRadius>,
                radius: Val<::bevy_ui::prelude::Val>|
            {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::with_bottom_left(
                                _self.into_inner(),
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`BorderRadius`] with its `bottom_left` field set to the given value.",
            &["_self", "radius"],
        )
        .register_documented(
            "with_bottom_right",
            |
                _self: Val<::bevy_ui::prelude::BorderRadius>,
                radius: Val<::bevy_ui::prelude::Val>|
            {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::with_bottom_right(
                                _self.into_inner(),
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`BorderRadius`] with its `bottom_right` field set to the given value.",
            &["_self", "radius"],
        )
        .register_documented(
            "with_left",
            |
                _self: Val<::bevy_ui::prelude::BorderRadius>,
                radius: Val<::bevy_ui::prelude::Val>|
            {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::with_left(
                                _self.into_inner(),
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`BorderRadius`] with its `top_left` and `bottom_left` fields set to the given value.",
            &["_self", "radius"],
        )
        .register_documented(
            "with_right",
            |
                _self: Val<::bevy_ui::prelude::BorderRadius>,
                radius: Val<::bevy_ui::prelude::Val>|
            {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::with_right(
                                _self.into_inner(),
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`BorderRadius`] with its `top_right` and `bottom_right` fields set to the given value.",
            &["_self", "radius"],
        )
        .register_documented(
            "with_top",
            |
                _self: Val<::bevy_ui::prelude::BorderRadius>,
                radius: Val<::bevy_ui::prelude::Val>|
            {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::with_top(
                                _self.into_inner(),
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`BorderRadius`] with its `top_left` and `top_right` fields set to the given value.",
            &["_self", "radius"],
        )
        .register_documented(
            "with_top_left",
            |
                _self: Val<::bevy_ui::prelude::BorderRadius>,
                radius: Val<::bevy_ui::prelude::Val>|
            {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::with_top_left(
                                _self.into_inner(),
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`BorderRadius`] with its `top_left` field set to the given value.",
            &["_self", "radius"],
        )
        .register_documented(
            "with_top_right",
            |
                _self: Val<::bevy_ui::prelude::BorderRadius>,
                radius: Val<::bevy_ui::prelude::Val>|
            {
                let output: Val<::bevy_ui::prelude::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::prelude::BorderRadius> = ::bevy_ui::prelude::BorderRadius::with_top_right(
                                _self.into_inner(),
                                radius.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`BorderRadius`] with its `top_right` field set to the given value.",
            &["_self", "radius"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::prelude::BorderRadius,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_layout_config_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::LayoutConfig,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::LayoutConfig>| {
            let output: Val<::bevy_ui::prelude::LayoutConfig> = {
                {
                    let output: Val<::bevy_ui::prelude::LayoutConfig> =
                        <::bevy_ui::prelude::LayoutConfig as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_ui::prelude::LayoutConfig>,
         other: Ref<::bevy_ui::prelude::LayoutConfig>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::prelude::LayoutConfig as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::LayoutConfig,
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
            ::bevy_ui::prelude::LayoutConfig,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_outline_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::Outline,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::prelude::Outline>| {
                let output: Val<::bevy_ui::prelude::Outline> = {
                    {
                        let output: Val<::bevy_ui::prelude::Outline> = <::bevy_ui::prelude::Outline as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::Outline>,
                other: Ref<::bevy_ui::prelude::Outline>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::Outline as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::Outline,
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
            "new",
            |
                width: Val<::bevy_ui::prelude::Val>,
                offset: Val<::bevy_ui::prelude::Val>,
                color: Val<::bevy_color::Color>|
            {
                let output: Val<::bevy_ui::prelude::Outline> = {
                    {
                        let output: Val<::bevy_ui::prelude::Outline> = ::bevy_ui::prelude::Outline::new(
                                width.into_inner(),
                                offset.into_inner(),
                                color.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new outline",
            &["width", "offset", "color"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::prelude::Outline,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scroll_position_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::ScrollPosition,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::ScrollPosition>| {
            let output: Val<::bevy_ui::prelude::ScrollPosition> = {
                {
                    let output: Val<::bevy_ui::prelude::ScrollPosition> =
                        <::bevy_ui::prelude::ScrollPosition as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_ui::prelude::ScrollPosition,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_position_type_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::PositionType,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::PositionType>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::PositionType as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::PositionType>| {
                let output: Val<::bevy_ui::prelude::PositionType> = {
                    {
                        let output: Val<::bevy_ui::prelude::PositionType> = <::bevy_ui::prelude::PositionType as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::PositionType>,
                other: Ref<::bevy_ui::prelude::PositionType>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::PositionType as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::PositionType,
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
            ::bevy_ui::prelude::PositionType,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_align_self_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::AlignSelf,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::AlignSelf>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::AlignSelf as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::AlignSelf>| {
                let output: Val<::bevy_ui::prelude::AlignSelf> = {
                    {
                        let output: Val<::bevy_ui::prelude::AlignSelf> = <::bevy_ui::prelude::AlignSelf as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::AlignSelf>,
                other: Ref<::bevy_ui::prelude::AlignSelf>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::AlignSelf as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::AlignSelf,
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
            ::bevy_ui::prelude::AlignSelf,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_repeated_grid_track_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::RepeatedGridTrack,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::RepeatedGridTrack>| {
            let output: Val<::bevy_ui::prelude::RepeatedGridTrack> = {
                {
                    let output: Val<::bevy_ui::prelude::RepeatedGridTrack> =
                        <::bevy_ui::prelude::RepeatedGridTrack as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::prelude::RepeatedGridTrack>,
         other: Ref<::bevy_ui::prelude::RepeatedGridTrack>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::prelude::RepeatedGridTrack as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::RepeatedGridTrack,
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
            ::bevy_ui::prelude::RepeatedGridTrack,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_align_content_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::AlignContent,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::AlignContent>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::AlignContent as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::AlignContent>| {
                let output: Val<::bevy_ui::prelude::AlignContent> = {
                    {
                        let output: Val<::bevy_ui::prelude::AlignContent> = <::bevy_ui::prelude::AlignContent as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::AlignContent>,
                other: Ref<::bevy_ui::prelude::AlignContent>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::AlignContent as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::AlignContent,
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
            ::bevy_ui::prelude::AlignContent,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_align_items_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::AlignItems,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::AlignItems>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::AlignItems as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::AlignItems>| {
                let output: Val<::bevy_ui::prelude::AlignItems> = {
                    {
                        let output: Val<::bevy_ui::prelude::AlignItems> = <::bevy_ui::prelude::AlignItems as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::AlignItems>,
                other: Ref<::bevy_ui::prelude::AlignItems>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::AlignItems as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::AlignItems,
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
            ::bevy_ui::prelude::AlignItems,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_box_sizing_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::BoxSizing,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::BoxSizing>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::BoxSizing as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::BoxSizing>| {
                let output: Val<::bevy_ui::prelude::BoxSizing> = {
                    {
                        let output: Val<::bevy_ui::prelude::BoxSizing> = <::bevy_ui::prelude::BoxSizing as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::BoxSizing>,
                other: Ref<::bevy_ui::prelude::BoxSizing>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::BoxSizing as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::BoxSizing,
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
            ::bevy_ui::prelude::BoxSizing,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_flex_direction_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::FlexDirection,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::FlexDirection>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::FlexDirection as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::FlexDirection>| {
                let output: Val<::bevy_ui::prelude::FlexDirection> = {
                    {
                        let output: Val<::bevy_ui::prelude::FlexDirection> = <::bevy_ui::prelude::FlexDirection as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::FlexDirection>,
                other: Ref<::bevy_ui::prelude::FlexDirection>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::FlexDirection as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::FlexDirection,
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
            ::bevy_ui::prelude::FlexDirection,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_flex_wrap_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::FlexWrap,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::FlexWrap>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::FlexWrap as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::FlexWrap>| {
                let output: Val<::bevy_ui::prelude::FlexWrap> = {
                    {
                        let output: Val<::bevy_ui::prelude::FlexWrap> = <::bevy_ui::prelude::FlexWrap as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::FlexWrap>,
                other: Ref<::bevy_ui::prelude::FlexWrap>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::FlexWrap as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::FlexWrap,
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
            ::bevy_ui::prelude::FlexWrap,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_grid_auto_flow_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::GridAutoFlow,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::GridAutoFlow>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::GridAutoFlow as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::GridAutoFlow>| {
                let output: Val<::bevy_ui::prelude::GridAutoFlow> = {
                    {
                        let output: Val<::bevy_ui::prelude::GridAutoFlow> = <::bevy_ui::prelude::GridAutoFlow as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::GridAutoFlow>,
                other: Ref<::bevy_ui::prelude::GridAutoFlow>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::GridAutoFlow as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::GridAutoFlow,
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
            ::bevy_ui::prelude::GridAutoFlow,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_grid_placement_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::GridPlacement,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::GridPlacement>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::GridPlacement as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            "auto",
            || {
                let output: Val<::bevy_ui::prelude::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::prelude::GridPlacement> = ::bevy_ui::prelude::GridPlacement::auto()
                            .into();
                        output
                    }
                };
                output
            },
            " Place the grid item automatically (letting the `span` default to `1`).",
            &[],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::prelude::GridPlacement>| {
                let output: Val<::bevy_ui::prelude::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::prelude::GridPlacement> = <::bevy_ui::prelude::GridPlacement as ::std::clone::Clone>::clone(
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
            "end",
            |end: i16| {
                let output: Val<::bevy_ui::prelude::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::prelude::GridPlacement> = ::bevy_ui::prelude::GridPlacement::end(
                                end,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Place the grid item specifying the `end` grid line (letting the `span` default to `1`).\n # Panics\n Panics if `end` is `0`.",
            &["end"],
        )
        .register_documented(
            "end_span",
            |end: i16, span: u16| {
                let output: Val<::bevy_ui::prelude::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::prelude::GridPlacement> = ::bevy_ui::prelude::GridPlacement::end_span(
                                end,
                                span,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Place the grid item specifying the `end` grid line and how many tracks it should `span`.\n # Panics\n Panics if `end` or `span` is `0`.",
            &["end", "span"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_ui::prelude::GridPlacement>,
                other: Ref<::bevy_ui::prelude::GridPlacement>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::GridPlacement as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::GridPlacement,
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
            "get_end",
            |_self: Val<::bevy_ui::prelude::GridPlacement>| {
                let output: ::std::option::Option<i16> = {
                    {
                        let output: ::std::option::Option<i16> = ::bevy_ui::prelude::GridPlacement::get_end(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the grid line at which the item should end, or `None` if not set.",
            &["_self"],
        )
        .register_documented(
            "get_span",
            |_self: Val<::bevy_ui::prelude::GridPlacement>| {
                let output: ::std::option::Option<u16> = {
                    {
                        let output: ::std::option::Option<u16> = ::bevy_ui::prelude::GridPlacement::get_span(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns span for this grid item, or `None` if not set.",
            &["_self"],
        )
        .register_documented(
            "get_start",
            |_self: Val<::bevy_ui::prelude::GridPlacement>| {
                let output: ::std::option::Option<i16> = {
                    {
                        let output: ::std::option::Option<i16> = ::bevy_ui::prelude::GridPlacement::get_start(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the grid line at which the item should start, or `None` if not set.",
            &["_self"],
        )
        .register_documented(
            "set_end",
            |_self: Val<::bevy_ui::prelude::GridPlacement>, end: i16| {
                let output: Val<::bevy_ui::prelude::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::prelude::GridPlacement> = ::bevy_ui::prelude::GridPlacement::set_end(
                                _self.into_inner(),
                                end,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Mutate the item, setting the `end` grid line\n # Panics\n Panics if `end` is `0`.",
            &["_self", "end"],
        )
        .register_documented(
            "set_span",
            |_self: Val<::bevy_ui::prelude::GridPlacement>, span: u16| {
                let output: Val<::bevy_ui::prelude::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::prelude::GridPlacement> = ::bevy_ui::prelude::GridPlacement::set_span(
                                _self.into_inner(),
                                span,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Mutate the item, setting the number of tracks the item should `span`\n # Panics\n Panics if `span` is `0`.",
            &["_self", "span"],
        )
        .register_documented(
            "set_start",
            |_self: Val<::bevy_ui::prelude::GridPlacement>, start: i16| {
                let output: Val<::bevy_ui::prelude::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::prelude::GridPlacement> = ::bevy_ui::prelude::GridPlacement::set_start(
                                _self.into_inner(),
                                start,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Mutate the item, setting the `start` grid line\n # Panics\n Panics if `start` is `0`.",
            &["_self", "start"],
        )
        .register_documented(
            "span",
            |span: u16| {
                let output: Val<::bevy_ui::prelude::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::prelude::GridPlacement> = ::bevy_ui::prelude::GridPlacement::span(
                                span,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Place the grid item automatically, specifying how many tracks it should `span`.\n # Panics\n Panics if `span` is `0`.",
            &["span"],
        )
        .register_documented(
            "start",
            |start: i16| {
                let output: Val<::bevy_ui::prelude::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::prelude::GridPlacement> = ::bevy_ui::prelude::GridPlacement::start(
                                start,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Place the grid item specifying the `start` grid line (letting the `span` default to `1`).\n # Panics\n Panics if `start` is `0`.",
            &["start"],
        )
        .register_documented(
            "start_end",
            |start: i16, end: i16| {
                let output: Val<::bevy_ui::prelude::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::prelude::GridPlacement> = ::bevy_ui::prelude::GridPlacement::start_end(
                                start,
                                end,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Place the grid item specifying `start` and `end` grid lines (`span` will be inferred)\n # Panics\n Panics if `start` or `end` is `0`.",
            &["start", "end"],
        )
        .register_documented(
            "start_span",
            |start: i16, span: u16| {
                let output: Val<::bevy_ui::prelude::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::prelude::GridPlacement> = ::bevy_ui::prelude::GridPlacement::start_span(
                                start,
                                span,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Place the grid item specifying the `start` grid line and how many tracks it should `span`.\n # Panics\n Panics if `start` or `span` is `0`.",
            &["start", "span"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::prelude::GridPlacement,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_grid_track_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::GridTrack,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::GridTrack>| {
            let output: Val<::bevy_ui::prelude::GridTrack> = {
                {
                    let output: Val<::bevy_ui::prelude::GridTrack> =
                        <::bevy_ui::prelude::GridTrack as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_ui::prelude::GridTrack>, other: Ref<::bevy_ui::prelude::GridTrack>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_ui::prelude::GridTrack as ::std::cmp::PartialEq<
                        ::bevy_ui::prelude::GridTrack,
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
            ::bevy_ui::prelude::GridTrack,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_grid_track_repetition_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::GridTrackRepetition,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::GridTrackRepetition>| {
            let output: Val<::bevy_ui::prelude::GridTrackRepetition> = {
                {
                    let output: Val<::bevy_ui::prelude::GridTrackRepetition> =
                        <::bevy_ui::prelude::GridTrackRepetition as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::prelude::GridTrackRepetition>,
         other: Ref<::bevy_ui::prelude::GridTrackRepetition>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::prelude::GridTrackRepetition as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::GridTrackRepetition,
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
            ::bevy_ui::prelude::GridTrackRepetition,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_justify_content_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::JustifyContent,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::JustifyContent>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::JustifyContent as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::JustifyContent>| {
                let output: Val<::bevy_ui::prelude::JustifyContent> = {
                    {
                        let output: Val<::bevy_ui::prelude::JustifyContent> = <::bevy_ui::prelude::JustifyContent as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::JustifyContent>,
                other: Ref<::bevy_ui::prelude::JustifyContent>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::JustifyContent as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::JustifyContent,
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
            ::bevy_ui::prelude::JustifyContent,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_justify_items_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::JustifyItems,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::JustifyItems>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::JustifyItems as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::JustifyItems>| {
                let output: Val<::bevy_ui::prelude::JustifyItems> = {
                    {
                        let output: Val<::bevy_ui::prelude::JustifyItems> = <::bevy_ui::prelude::JustifyItems as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::JustifyItems>,
                other: Ref<::bevy_ui::prelude::JustifyItems>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::JustifyItems as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::JustifyItems,
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
            ::bevy_ui::prelude::JustifyItems,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_justify_self_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::JustifySelf,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::JustifySelf>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::JustifySelf as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::JustifySelf>| {
                let output: Val<::bevy_ui::prelude::JustifySelf> = {
                    {
                        let output: Val<::bevy_ui::prelude::JustifySelf> = <::bevy_ui::prelude::JustifySelf as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::JustifySelf>,
                other: Ref<::bevy_ui::prelude::JustifySelf>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::JustifySelf as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::JustifySelf,
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
            ::bevy_ui::prelude::JustifySelf,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_max_track_sizing_function_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::MaxTrackSizingFunction,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::MaxTrackSizingFunction>| {
            let output: Val<::bevy_ui::prelude::MaxTrackSizingFunction> = {
                {
                    let output: Val<::bevy_ui::prelude::MaxTrackSizingFunction> =
                        <::bevy_ui::prelude::MaxTrackSizingFunction as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::prelude::MaxTrackSizingFunction>,
         other: Ref<::bevy_ui::prelude::MaxTrackSizingFunction>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::prelude::MaxTrackSizingFunction as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::MaxTrackSizingFunction,
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
            ::bevy_ui::prelude::MaxTrackSizingFunction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_min_track_sizing_function_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::MinTrackSizingFunction,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::MinTrackSizingFunction>| {
            let output: Val<::bevy_ui::prelude::MinTrackSizingFunction> = {
                {
                    let output: Val<::bevy_ui::prelude::MinTrackSizingFunction> =
                        <::bevy_ui::prelude::MinTrackSizingFunction as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::prelude::MinTrackSizingFunction>,
         other: Ref<::bevy_ui::prelude::MinTrackSizingFunction>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::prelude::MinTrackSizingFunction as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::MinTrackSizingFunction,
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
            ::bevy_ui::prelude::MinTrackSizingFunction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_overflow_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::Overflow,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::Overflow>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::Overflow as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            "clip",
            || {
                let output: Val<::bevy_ui::prelude::Overflow> = {
                    {
                        let output: Val<::bevy_ui::prelude::Overflow> = ::bevy_ui::prelude::Overflow::clip()
                            .into();
                        output
                    }
                };
                output
            },
            " Clip overflowing items on both axes",
            &[],
        )
        .register_documented(
            "clip_x",
            || {
                let output: Val<::bevy_ui::prelude::Overflow> = {
                    {
                        let output: Val<::bevy_ui::prelude::Overflow> = ::bevy_ui::prelude::Overflow::clip_x()
                            .into();
                        output
                    }
                };
                output
            },
            " Clip overflowing items on the x axis",
            &[],
        )
        .register_documented(
            "clip_y",
            || {
                let output: Val<::bevy_ui::prelude::Overflow> = {
                    {
                        let output: Val<::bevy_ui::prelude::Overflow> = ::bevy_ui::prelude::Overflow::clip_y()
                            .into();
                        output
                    }
                };
                output
            },
            " Clip overflowing items on the y axis",
            &[],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::prelude::Overflow>| {
                let output: Val<::bevy_ui::prelude::Overflow> = {
                    {
                        let output: Val<::bevy_ui::prelude::Overflow> = <::bevy_ui::prelude::Overflow as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::Overflow>,
                other: Ref<::bevy_ui::prelude::Overflow>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::Overflow as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::Overflow,
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
            "hidden",
            || {
                let output: Val<::bevy_ui::prelude::Overflow> = {
                    {
                        let output: Val<::bevy_ui::prelude::Overflow> = ::bevy_ui::prelude::Overflow::hidden()
                            .into();
                        output
                    }
                };
                output
            },
            " Hide overflowing items on both axes by influencing layout and then clipping",
            &[],
        )
        .register_documented(
            "hidden_x",
            || {
                let output: Val<::bevy_ui::prelude::Overflow> = {
                    {
                        let output: Val<::bevy_ui::prelude::Overflow> = ::bevy_ui::prelude::Overflow::hidden_x()
                            .into();
                        output
                    }
                };
                output
            },
            " Hide overflowing items on the x axis by influencing layout and then clipping",
            &[],
        )
        .register_documented(
            "hidden_y",
            || {
                let output: Val<::bevy_ui::prelude::Overflow> = {
                    {
                        let output: Val<::bevy_ui::prelude::Overflow> = ::bevy_ui::prelude::Overflow::hidden_y()
                            .into();
                        output
                    }
                };
                output
            },
            " Hide overflowing items on the y axis by influencing layout and then clipping",
            &[],
        )
        .register_documented(
            "is_visible",
            |_self: Ref<::bevy_ui::prelude::Overflow>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ui::prelude::Overflow::is_visible(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Overflow is visible on both axes",
            &["_self"],
        )
        .register_documented(
            "scroll",
            || {
                let output: Val<::bevy_ui::prelude::Overflow> = {
                    {
                        let output: Val<::bevy_ui::prelude::Overflow> = ::bevy_ui::prelude::Overflow::scroll()
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &[],
        )
        .register_documented(
            "scroll_x",
            || {
                let output: Val<::bevy_ui::prelude::Overflow> = {
                    {
                        let output: Val<::bevy_ui::prelude::Overflow> = ::bevy_ui::prelude::Overflow::scroll_x()
                            .into();
                        output
                    }
                };
                output
            },
            " Scroll overflowing items on the x axis",
            &[],
        )
        .register_documented(
            "scroll_y",
            || {
                let output: Val<::bevy_ui::prelude::Overflow> = {
                    {
                        let output: Val<::bevy_ui::prelude::Overflow> = ::bevy_ui::prelude::Overflow::scroll_y()
                            .into();
                        output
                    }
                };
                output
            },
            " Scroll overflowing items on the y axis",
            &[],
        )
        .register_documented(
            "visible",
            || {
                let output: Val<::bevy_ui::prelude::Overflow> = {
                    {
                        let output: Val<::bevy_ui::prelude::Overflow> = ::bevy_ui::prelude::Overflow::visible()
                            .into();
                        output
                    }
                };
                output
            },
            " Show overflowing items on both axes",
            &[],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::prelude::Overflow,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_overflow_clip_margin_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::OverflowClipMargin,
    >::new(world)
        .register_documented(
            "border_box",
            || {
                let output: Val<::bevy_ui::prelude::OverflowClipMargin> = {
                    {
                        let output: Val<::bevy_ui::prelude::OverflowClipMargin> = ::bevy_ui::prelude::OverflowClipMargin::border_box()
                            .into();
                        output
                    }
                };
                output
            },
            " Clip any content that overflows outside the border box",
            &[],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::prelude::OverflowClipMargin>| {
                let output: Val<::bevy_ui::prelude::OverflowClipMargin> = {
                    {
                        let output: Val<::bevy_ui::prelude::OverflowClipMargin> = <::bevy_ui::prelude::OverflowClipMargin as ::std::clone::Clone>::clone(
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
            "content_box",
            || {
                let output: Val<::bevy_ui::prelude::OverflowClipMargin> = {
                    {
                        let output: Val<::bevy_ui::prelude::OverflowClipMargin> = ::bevy_ui::prelude::OverflowClipMargin::content_box()
                            .into();
                        output
                    }
                };
                output
            },
            " Clip any content that overflows outside the content box",
            &[],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_ui::prelude::OverflowClipMargin>,
                other: Ref<::bevy_ui::prelude::OverflowClipMargin>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::OverflowClipMargin as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::OverflowClipMargin,
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
            "padding_box",
            || {
                let output: Val<::bevy_ui::prelude::OverflowClipMargin> = {
                    {
                        let output: Val<::bevy_ui::prelude::OverflowClipMargin> = ::bevy_ui::prelude::OverflowClipMargin::padding_box()
                            .into();
                        output
                    }
                };
                output
            },
            " Clip any content that overflows outside the padding box",
            &[],
        )
        .register_documented(
            "with_margin",
            |_self: Val<::bevy_ui::prelude::OverflowClipMargin>, margin: f32| {
                let output: Val<::bevy_ui::prelude::OverflowClipMargin> = {
                    {
                        let output: Val<::bevy_ui::prelude::OverflowClipMargin> = ::bevy_ui::prelude::OverflowClipMargin::with_margin(
                                _self.into_inner(),
                                margin,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Add a margin on each edge of the visual box in logical pixels.\n The width of the margin will be zero if a negative value is set.",
            &["_self", "margin"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::prelude::OverflowClipMargin,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_global_z_index_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::GlobalZIndex,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::GlobalZIndex>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::GlobalZIndex as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::GlobalZIndex>| {
                let output: Val<::bevy_ui::prelude::GlobalZIndex> = {
                    {
                        let output: Val<::bevy_ui::prelude::GlobalZIndex> = <::bevy_ui::prelude::GlobalZIndex as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::GlobalZIndex>,
                other: Ref<::bevy_ui::prelude::GlobalZIndex>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::GlobalZIndex as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::GlobalZIndex,
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
            ::bevy_ui::prelude::GlobalZIndex,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_z_index_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::ZIndex,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::prelude::ZIndex>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::prelude::ZIndex as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::prelude::ZIndex>| {
                let output: Val<::bevy_ui::prelude::ZIndex> = {
                    {
                        let output: Val<::bevy_ui::prelude::ZIndex> = <::bevy_ui::prelude::ZIndex as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::prelude::ZIndex>,
                other: Ref<::bevy_ui::prelude::ZIndex>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::prelude::ZIndex as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::ZIndex,
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
            ::bevy_ui::prelude::ZIndex,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_resolved_border_radius_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::ResolvedBorderRadius,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::ResolvedBorderRadius>| {
            let output: Val<::bevy_ui::prelude::ResolvedBorderRadius> = {
                {
                    let output: Val<::bevy_ui::prelude::ResolvedBorderRadius> =
                        <::bevy_ui::prelude::ResolvedBorderRadius as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_ui::prelude::ResolvedBorderRadius>,
         other: Ref<::bevy_ui::prelude::ResolvedBorderRadius>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::prelude::ResolvedBorderRadius as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::ResolvedBorderRadius,
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
            ::bevy_ui::prelude::ResolvedBorderRadius,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_background_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::BackgroundColor,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::BackgroundColor>| {
            let output: Val<::bevy_ui::prelude::BackgroundColor> = {
                {
                    let output: Val<::bevy_ui::prelude::BackgroundColor> =
                        <::bevy_ui::prelude::BackgroundColor as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_ui::prelude::BackgroundColor>,
         other: Ref<::bevy_ui::prelude::BackgroundColor>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_ui::prelude::BackgroundColor as ::std::cmp::PartialEq<
                            ::bevy_ui::prelude::BackgroundColor,
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
            ::bevy_ui::prelude::BackgroundColor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_border_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::BorderColor,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::BorderColor>| {
            let output: Val<::bevy_ui::prelude::BorderColor> = {
                {
                    let output: Val<::bevy_ui::prelude::BorderColor> =
                        <::bevy_ui::prelude::BorderColor as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_ui::prelude::BorderColor>,
         other: Ref<::bevy_ui::prelude::BorderColor>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_ui::prelude::BorderColor as ::std::cmp::PartialEq<
                        ::bevy_ui::prelude::BorderColor,
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
        "is_fully_transparent",
        |_self: Ref<::bevy_ui::prelude::BorderColor>| {
            let output: bool = {
                {
                    let output: bool =
                        ::bevy_ui::prelude::BorderColor::is_fully_transparent(&_self).into();
                    output
                }
            };
            output
        },
        " Check if all contained border colors are transparent",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::prelude::BorderColor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_box_shadow_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::BoxShadow,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::BoxShadow>| {
            let output: Val<::bevy_ui::prelude::BoxShadow> = {
                {
                    let output: Val<::bevy_ui::prelude::BoxShadow> =
                        <::bevy_ui::prelude::BoxShadow as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_ui::prelude::BoxShadow>, other: Ref<::bevy_ui::prelude::BoxShadow>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_ui::prelude::BoxShadow as ::std::cmp::PartialEq<
                        ::bevy_ui::prelude::BoxShadow,
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
        "new",
        |color: Val<::bevy_color::Color>,
         x_offset: Val<::bevy_ui::prelude::Val>,
         y_offset: Val<::bevy_ui::prelude::Val>,
         spread_radius: Val<::bevy_ui::prelude::Val>,
         blur_radius: Val<::bevy_ui::prelude::Val>| {
            let output: Val<::bevy_ui::prelude::BoxShadow> = {
                {
                    let output: Val<::bevy_ui::prelude::BoxShadow> =
                        ::bevy_ui::prelude::BoxShadow::new(
                            color.into_inner(),
                            x_offset.into_inner(),
                            y_offset.into_inner(),
                            spread_radius.into_inner(),
                            blur_radius.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        " A single drop shadow",
        &[
            "color",
            "x_offset",
            "y_offset",
            "spread_radius",
            "blur_radius",
        ],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::prelude::BoxShadow,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_shadow_style_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::prelude::ShadowStyle,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_ui::prelude::ShadowStyle>| {
            let output: Val<::bevy_ui::prelude::ShadowStyle> = {
                {
                    let output: Val<::bevy_ui::prelude::ShadowStyle> =
                        <::bevy_ui::prelude::ShadowStyle as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_ui::prelude::ShadowStyle>,
         other: Ref<::bevy_ui::prelude::ShadowStyle>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_ui::prelude::ShadowStyle as ::std::cmp::PartialEq<
                        ::bevy_ui::prelude::ShadowStyle,
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
            ::bevy_ui::prelude::ShadowStyle,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyUiScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_display_functions(&mut world);
        register_ui_picking_camera_functions(&mut world);
        register_ui_picking_settings_functions(&mut world);
        register_text_functions(&mut world);
        register_text_shadow_functions(&mut world);
        register_button_functions(&mut world);
        register_image_node_functions(&mut world);
        register_label_functions(&mut world);
        register_node_image_mode_functions(&mut world);
        register_viewport_node_functions(&mut world);
        register_interaction_functions(&mut world);
        register_ui_scale_functions(&mut world);
        register_computed_ui_target_camera_functions(&mut world);
        register_computed_ui_render_target_info_functions(&mut world);
        register_content_size_functions(&mut world);
        register_ui_global_transform_functions(&mut world);
        register_calculated_clip_functions(&mut world);
        register_node_functions(&mut world);
        register_overflow_axis_functions(&mut world);
        register_ui_target_camera_functions(&mut world);
        register_computed_node_functions(&mut world);
        register_overflow_clip_box_functions(&mut world);
        register_focus_policy_functions(&mut world);
        register_image_node_size_functions(&mut world);
        register_text_node_flags_functions(&mut world);
        register_ui_position_functions(&mut world);
        register_val_functions(&mut world);
        register_color_stop_functions(&mut world);
        register_angular_color_stop_functions(&mut world);
        register_linear_gradient_functions(&mut world);
        register_interpolation_color_space_functions(&mut world);
        register_radial_gradient_functions(&mut world);
        register_radial_gradient_shape_functions(&mut world);
        register_conic_gradient_functions(&mut world);
        register_gradient_functions(&mut world);
        register_background_gradient_functions(&mut world);
        register_border_gradient_functions(&mut world);
        register_val_2_functions(&mut world);
        register_ui_transform_functions(&mut world);
        register_relative_cursor_position_functions(&mut world);
        register_ui_rect_functions(&mut world);
        register_border_radius_functions(&mut world);
        register_layout_config_functions(&mut world);
        register_outline_functions(&mut world);
        register_scroll_position_functions(&mut world);
        register_position_type_functions(&mut world);
        register_align_self_functions(&mut world);
        register_repeated_grid_track_functions(&mut world);
        register_align_content_functions(&mut world);
        register_align_items_functions(&mut world);
        register_box_sizing_functions(&mut world);
        register_flex_direction_functions(&mut world);
        register_flex_wrap_functions(&mut world);
        register_grid_auto_flow_functions(&mut world);
        register_grid_placement_functions(&mut world);
        register_grid_track_functions(&mut world);
        register_grid_track_repetition_functions(&mut world);
        register_justify_content_functions(&mut world);
        register_justify_items_functions(&mut world);
        register_justify_self_functions(&mut world);
        register_max_track_sizing_function_functions(&mut world);
        register_min_track_sizing_function_functions(&mut world);
        register_overflow_functions(&mut world);
        register_overflow_clip_margin_functions(&mut world);
        register_global_z_index_functions(&mut world);
        register_z_index_functions(&mut world);
        register_resolved_border_radius_functions(&mut world);
        register_background_color_functions(&mut world);
        register_border_color_functions(&mut world);
        register_box_shadow_functions(&mut world);
        register_shadow_style_functions(&mut world);
    }
}
