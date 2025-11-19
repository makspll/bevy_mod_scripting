
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
pub struct BevyUiScriptingPlugin;
pub(crate) fn register_display_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::Display,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::Display>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::Display as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::Display>| {
                let output: Val<::bevy_ui::Display> = {
                    {
                        let output: Val<::bevy_ui::Display> = <::bevy_ui::Display as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::Display>, other: Ref<::bevy_ui::Display>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::Display as ::std::cmp::PartialEq<
                            ::bevy_ui::Display,
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
            ::bevy_ui::Display,
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
                        let output: Val<::bevy_ui::widget::TextShadow> = <::bevy_ui::widget::TextShadow as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::widget::TextShadow>,
                other: Ref<::bevy_ui::widget::TextShadow>|
            {
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
                        let output: Val<::bevy_ui::widget::ViewportNode> = <::bevy_ui::widget::ViewportNode as ::std::clone::Clone>::clone(
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
            |camera: Val<::bevy_ecs::entity::Entity>| {
                let output: Val<::bevy_ui::widget::ViewportNode> = {
                    {
                        let output: Val<::bevy_ui::widget::ViewportNode> = ::bevy_ui::widget::ViewportNode::new(
                                camera.into_inner(),
                            )
                            .into();
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
        ::bevy_ui::Interaction,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::Interaction>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::Interaction as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::Interaction>| {
                let output: Val<::bevy_ui::Interaction> = {
                    {
                        let output: Val<::bevy_ui::Interaction> = <::bevy_ui::Interaction as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::Interaction>, other: Ref<::bevy_ui::Interaction>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::Interaction as ::std::cmp::PartialEq<
                            ::bevy_ui::Interaction,
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
            ::bevy_ui::Interaction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_scale_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::UiScale,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::UiScale,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_computed_ui_target_camera_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::ComputedUiTargetCamera,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::ComputedUiTargetCamera>| {
                let output: Val<::bevy_ui::ComputedUiTargetCamera> = {
                    {
                        let output: Val<::bevy_ui::ComputedUiTargetCamera> = <::bevy_ui::ComputedUiTargetCamera as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::ComputedUiTargetCamera>,
                other: Ref<::bevy_ui::ComputedUiTargetCamera>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::ComputedUiTargetCamera as ::std::cmp::PartialEq<
                            ::bevy_ui::ComputedUiTargetCamera,
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
            ::bevy_ui::ComputedUiTargetCamera,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_computed_ui_render_target_info_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::ComputedUiRenderTargetInfo,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::ComputedUiRenderTargetInfo>| {
                let output: Val<::bevy_ui::ComputedUiRenderTargetInfo> = {
                    {
                        let output: Val<::bevy_ui::ComputedUiRenderTargetInfo> = <::bevy_ui::ComputedUiRenderTargetInfo as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::ComputedUiRenderTargetInfo>,
                other: Ref<::bevy_ui::ComputedUiRenderTargetInfo>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::ComputedUiRenderTargetInfo as ::std::cmp::PartialEq<
                            ::bevy_ui::ComputedUiRenderTargetInfo,
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
            "logical_size",
            |_self: Ref<::bevy_ui::ComputedUiRenderTargetInfo>| {
                let output: Val<::glam::Vec2> = {
                    {
                        let output: Val<::glam::Vec2> = ::bevy_ui::ComputedUiRenderTargetInfo::logical_size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the size of the target camera's viewport in logical pixels.",
            &["_self"],
        )
        .register_documented(
            "physical_size",
            |_self: Ref<::bevy_ui::ComputedUiRenderTargetInfo>| {
                let output: Val<::glam::UVec2> = {
                    {
                        let output: Val<::glam::UVec2> = ::bevy_ui::ComputedUiRenderTargetInfo::physical_size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the size of the target camera's viewport in physical pixels.",
            &["_self"],
        )
        .register_documented(
            "scale_factor",
            |_self: Ref<::bevy_ui::ComputedUiRenderTargetInfo>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui::ComputedUiRenderTargetInfo::scale_factor(
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
            ::bevy_ui::ComputedUiRenderTargetInfo,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_content_size_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::ContentSize,
    >::new(world)
        .register_documented(
            "fixed_size",
            |size: Val<::glam::Vec2>| {
                let output: Val<::bevy_ui::ContentSize> = {
                    {
                        let output: Val<::bevy_ui::ContentSize> = ::bevy_ui::ContentSize::fixed_size(
                                size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a `ContentSize` with a `Measure` that always returns given `size` argument, regardless of the UI layout's constraints.",
            &["size"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::ContentSize,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_global_transform_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::UiGlobalTransform,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::UiGlobalTransform>| {
                let output: Val<::bevy_ui::UiGlobalTransform> = {
                    {
                        let output: Val<::bevy_ui::UiGlobalTransform> = <::bevy_ui::UiGlobalTransform as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::UiGlobalTransform>,
                other: Ref<::bevy_ui::UiGlobalTransform>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::UiGlobalTransform as ::std::cmp::PartialEq<
                            ::bevy_ui::UiGlobalTransform,
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
            ::bevy_ui::UiGlobalTransform,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_calculated_clip_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::CalculatedClip,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::CalculatedClip>| {
                let output: Val<::bevy_ui::CalculatedClip> = {
                    {
                        let output: Val<::bevy_ui::CalculatedClip> = <::bevy_ui::CalculatedClip as ::std::clone::Clone>::clone(
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
            ::bevy_ui::CalculatedClip,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_node_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::Node,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::Node>| {
                let output: Val<::bevy_ui::Node> = {
                    {
                        let output: Val<::bevy_ui::Node> = <::bevy_ui::Node as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::Node>, other: Ref<::bevy_ui::Node>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::Node as ::std::cmp::PartialEq<
                            ::bevy_ui::Node,
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
            ::bevy_ui::Node,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_overflow_axis_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::OverflowAxis,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::OverflowAxis>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::OverflowAxis as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::OverflowAxis>| {
                let output: Val<::bevy_ui::OverflowAxis> = {
                    {
                        let output: Val<::bevy_ui::OverflowAxis> = <::bevy_ui::OverflowAxis as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::OverflowAxis>, other: Ref<::bevy_ui::OverflowAxis>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::OverflowAxis as ::std::cmp::PartialEq<
                            ::bevy_ui::OverflowAxis,
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
            |_self: Ref<::bevy_ui::OverflowAxis>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ui::OverflowAxis::is_visible(&_self)
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
            ::bevy_ui::OverflowAxis,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_target_camera_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::UiTargetCamera,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::UiTargetCamera>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::UiTargetCamera as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::UiTargetCamera>| {
                let output: Val<::bevy_ui::UiTargetCamera> = {
                    {
                        let output: Val<::bevy_ui::UiTargetCamera> = <::bevy_ui::UiTargetCamera as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::UiTargetCamera>| {
                let output: Val<::bevy_ecs::entity::Entity> = {
                    {
                        let output: Val<::bevy_ecs::entity::Entity> = ::bevy_ui::UiTargetCamera::entity(
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
                _self: Ref<::bevy_ui::UiTargetCamera>,
                other: Ref<::bevy_ui::UiTargetCamera>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::UiTargetCamera as ::std::cmp::PartialEq<
                            ::bevy_ui::UiTargetCamera,
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
            ::bevy_ui::UiTargetCamera,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_computed_node_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::ComputedNode,
    >::new(world)
        .register_documented(
            "border",
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: Val<::bevy_sprite::BorderRect> = {
                    {
                        let output: Val<::bevy_sprite::BorderRect> = ::bevy_ui::ComputedNode::border(
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
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: Val<::bevy_ui::ResolvedBorderRadius> = {
                    {
                        let output: Val<::bevy_ui::ResolvedBorderRadius> = ::bevy_ui::ComputedNode::border_radius(
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
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: Val<::bevy_ui::ComputedNode> = {
                    {
                        let output: Val<::bevy_ui::ComputedNode> = <::bevy_ui::ComputedNode as ::std::clone::Clone>::clone(
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
            "contains_point",
            |
                _self: Ref<::bevy_ui::ComputedNode>,
                transform: Val<::bevy_ui::UiGlobalTransform>,
                point: Val<::glam::Vec2>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ui::ComputedNode::contains_point(
                                &_self,
                                transform.into_inner(),
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "transform", "point"],
        )
        .register_documented(
            "content_inset",
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: Val<::bevy_sprite::BorderRect> = {
                    {
                        let output: Val<::bevy_sprite::BorderRect> = ::bevy_ui::ComputedNode::content_inset(
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
            "content_size",
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: Val<::glam::Vec2> = {
                    {
                        let output: Val<::glam::Vec2> = ::bevy_ui::ComputedNode::content_size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The calculated node content size as width and height in physical pixels.\n Automatically calculated by [`ui_layout_system`](`super::layout::ui_layout_system`).",
            &["_self"],
        )
        .register_documented(
            "eq",
            |_self: Ref<::bevy_ui::ComputedNode>, other: Ref<::bevy_ui::ComputedNode>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::ComputedNode as ::std::cmp::PartialEq<
                            ::bevy_ui::ComputedNode,
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
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: Val<::bevy_ui::ResolvedBorderRadius> = {
                    {
                        let output: Val<::bevy_ui::ResolvedBorderRadius> = ::bevy_ui::ComputedNode::inner_radius(
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
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui::ComputedNode::inverse_scale_factor(
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
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ui::ComputedNode::is_empty(&_self)
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
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui::ComputedNode::outline_offset(&_self)
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
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: Val<::bevy_ui::ResolvedBorderRadius> = {
                    {
                        let output: Val<::bevy_ui::ResolvedBorderRadius> = ::bevy_ui::ComputedNode::outline_radius(
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
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui::ComputedNode::outline_width(&_self)
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
            "outlined_node_size",
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: Val<::glam::Vec2> = {
                    {
                        let output: Val<::glam::Vec2> = ::bevy_ui::ComputedNode::outlined_node_size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the size of the node when including its outline.\n Automatically calculated by [`ui_layout_system`](`super::layout::ui_layout_system`).",
            &["_self"],
        )
        .register_documented(
            "padding",
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: Val<::bevy_sprite::BorderRect> = {
                    {
                        let output: Val<::bevy_sprite::BorderRect> = ::bevy_ui::ComputedNode::padding(
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
                _self: Ref<::bevy_ui::ComputedNode>,
                overflow: Val<::bevy_ui::Overflow>,
                overflow_clip_margin: Val<::bevy_ui::OverflowClipMargin>|
            {
                let output: Val<::bevy_math::Rect> = {
                    {
                        let output: Val<::bevy_math::Rect> = ::bevy_ui::ComputedNode::resolve_clip_rect(
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
            "size",
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: Val<::glam::Vec2> = {
                    {
                        let output: Val<::glam::Vec2> = ::bevy_ui::ComputedNode::size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The calculated node size as width and height in physical pixels.\n Automatically calculated by [`ui_layout_system`](`super::layout::ui_layout_system`).",
            &["_self"],
        )
        .register_documented(
            "stack_index",
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_ui::ComputedNode::stack_index(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " The order of the node in the UI layout.\n Nodes with a higher stack index are drawn on top of and receive interactions before nodes with lower stack indices.\n Automatically calculated in [`UiSystems::Stack`](super::UiSystems::Stack).",
            &["_self"],
        )
        .register_documented(
            "unrounded_size",
            |_self: Ref<::bevy_ui::ComputedNode>| {
                let output: Val<::glam::Vec2> = {
                    {
                        let output: Val<::glam::Vec2> = ::bevy_ui::ComputedNode::unrounded_size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The calculated node size as width and height in physical pixels before rounding.\n Automatically calculated by [`ui_layout_system`](`super::layout::ui_layout_system`).",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::ComputedNode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_overflow_clip_box_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::OverflowClipBox,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::OverflowClipBox>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::OverflowClipBox as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::OverflowClipBox>| {
                let output: Val<::bevy_ui::OverflowClipBox> = {
                    {
                        let output: Val<::bevy_ui::OverflowClipBox> = <::bevy_ui::OverflowClipBox as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::OverflowClipBox>,
                other: Ref<::bevy_ui::OverflowClipBox>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::OverflowClipBox as ::std::cmp::PartialEq<
                            ::bevy_ui::OverflowClipBox,
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
            ::bevy_ui::OverflowClipBox,
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
        .register_type_data::<
            ::bevy_ui::FocusPolicy,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
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
                        let output: Val<::bevy_ui::widget::ImageNodeSize> = <::bevy_ui::widget::ImageNodeSize as ::std::clone::Clone>::clone(
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
            "size",
            |_self: Ref<::bevy_ui::widget::ImageNodeSize>| {
                let output: Val<::glam::UVec2> = {
                    {
                        let output: Val<::glam::UVec2> = ::bevy_ui::widget::ImageNodeSize::size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The size of the image's texture",
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
                        let output: Val<::bevy_ui::widget::TextNodeFlags> = <::bevy_ui::widget::TextNodeFlags as ::std::clone::Clone>::clone(
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
            ::bevy_ui::widget::TextNodeFlags,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_position_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::UiPosition,
    >::new(world)
        .register_documented(
            "anchor",
            |anchor: Val<::glam::Vec2>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::anchor(
                                anchor.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Position at the given normalized anchor point",
            &["anchor"],
        )
        .register_documented(
            "at",
            |
                _self: Val<::bevy_ui::UiPosition>,
                x: Val<::bevy_ui::Val>,
                y: Val<::bevy_ui::Val>|
            {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::at(
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
            |_self: Val<::bevy_ui::UiPosition>, x: f32, y: f32| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::at_percent(
                                _self.into_inner(),
                                x,
                                y,
                            )
                            .into();
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
            |_self: Val<::bevy_ui::UiPosition>, x: f32, y: f32| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::at_px(
                                _self.into_inner(),
                                x,
                                y,
                            )
                            .into();
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
            |_self: Val<::bevy_ui::UiPosition>, x: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::at_x(
                                _self.into_inner(),
                                x.into_inner(),
                            )
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
            |_self: Val<::bevy_ui::UiPosition>, y: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::at_y(
                                _self.into_inner(),
                                y.into_inner(),
                            )
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
            |x: Val<::bevy_ui::Val>, y: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::bottom(
                                x.into_inner(),
                                y.into_inner(),
                            )
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
            |x: Val<::bevy_ui::Val>, y: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::bottom_left(
                                x.into_inner(),
                                y.into_inner(),
                            )
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
            |x: Val<::bevy_ui::Val>, y: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::bottom_right(
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
            |x: Val<::bevy_ui::Val>, y: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::center(
                                x.into_inner(),
                                y.into_inner(),
                            )
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
            |_self: Ref<::bevy_ui::UiPosition>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = <::bevy_ui::UiPosition as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::UiPosition>, other: Ref<::bevy_ui::UiPosition>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::UiPosition as ::std::cmp::PartialEq<
                            ::bevy_ui::UiPosition,
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
            |x: Val<::bevy_ui::Val>, y: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::left(
                                x.into_inner(),
                                y.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Position relative to the left edge",
            &["x", "y"],
        )
        .register_documented(
            "new",
            |anchor: Val<::glam::Vec2>, x: Val<::bevy_ui::Val>, y: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::new(
                                anchor.into_inner(),
                                x.into_inner(),
                                y.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new position",
            &["anchor", "x", "y"],
        )
        .register_documented(
            "resolve",
            |
                _self: Val<::bevy_ui::UiPosition>,
                scale_factor: f32,
                physical_size: Val<::glam::Vec2>,
                physical_target_size: Val<::glam::Vec2>|
            {
                let output: Val<::glam::Vec2> = {
                    {
                        let output: Val<::glam::Vec2> = ::bevy_ui::UiPosition::resolve(
                                _self.into_inner(),
                                scale_factor,
                                physical_size.into_inner(),
                                physical_target_size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Resolves the `Position` into physical coordinates.",
            &["_self", "scale_factor", "physical_size", "physical_target_size"],
        )
        .register_documented(
            "right",
            |x: Val<::bevy_ui::Val>, y: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::right(
                                x.into_inner(),
                                y.into_inner(),
                            )
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
            |x: Val<::bevy_ui::Val>, y: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::top(
                                x.into_inner(),
                                y.into_inner(),
                            )
                            .into();
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
            |x: Val<::bevy_ui::Val>, y: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::top_left(
                                x.into_inner(),
                                y.into_inner(),
                            )
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
            |x: Val<::bevy_ui::Val>, y: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::top_right(
                                x.into_inner(),
                                y.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Position relative to the top-right corner",
            &["x", "y"],
        )
        .register_documented(
            "with_anchor",
            |_self: Val<::bevy_ui::UiPosition>, anchor: Val<::glam::Vec2>| {
                let output: Val<::bevy_ui::UiPosition> = {
                    {
                        let output: Val<::bevy_ui::UiPosition> = ::bevy_ui::UiPosition::with_anchor(
                                _self.into_inner(),
                                anchor.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a position from self with the given `anchor` point",
            &["_self", "anchor"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::UiPosition,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_val_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::Val,
    >::new(world)
        .register_documented(
            "all",
            |_self: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::Val::all(
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
            |_self: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::Val::bottom(
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
            |_self: Ref<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::Val> = {
                    {
                        let output: Val<::bevy_ui::Val> = <::bevy_ui::Val as ::std::clone::Clone>::clone(
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
            |_self: Val<::bevy_ui::Val>, rhs: f32| {
                let output: Val<::bevy_ui::Val> = {
                    {
                        let output: Val<::bevy_ui::Val> = <::bevy_ui::Val as ::std::ops::Div<
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
            |_self: Ref<::bevy_ui::Val>, other: Ref<::bevy_ui::Val>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::Val as ::std::cmp::PartialEq<
                            ::bevy_ui::Val,
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
            |_self: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::Val::horizontal(
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
            |_self: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::Val::left(
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
            |_self: Val<::bevy_ui::Val>, rhs: f32| {
                let output: Val<::bevy_ui::Val> = {
                    {
                        let output: Val<::bevy_ui::Val> = <::bevy_ui::Val as ::std::ops::Mul<
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
            |_self: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::Val> = {
                    {
                        let output: Val<::bevy_ui::Val> = <::bevy_ui::Val as ::std::ops::Neg>::neg(
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
            |_self: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::Val::right(
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
            |_self: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::Val::top(
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
            |_self: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::Val::vertical(
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
            ::bevy_ui::Val,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_color_stop_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::ColorStop,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::ColorStop>| {
                let output: Val<::bevy_ui::ColorStop> = {
                    {
                        let output: Val<::bevy_ui::ColorStop> = <::bevy_ui::ColorStop as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::ColorStop>, other: Ref<::bevy_ui::ColorStop>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::ColorStop as ::std::cmp::PartialEq<
                            ::bevy_ui::ColorStop,
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
            |_self: Val<::bevy_ui::ColorStop>, hint: f32| {
                let output: Val<::bevy_ui::ColorStop> = {
                    {
                        let output: Val<::bevy_ui::ColorStop> = ::bevy_ui::ColorStop::with_hint(
                                _self.into_inner(),
                                hint,
                            )
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
            ::bevy_ui::ColorStop,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_angular_color_stop_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::AngularColorStop,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::AngularColorStop>| {
                let output: Val<::bevy_ui::AngularColorStop> = {
                    {
                        let output: Val<::bevy_ui::AngularColorStop> = <::bevy_ui::AngularColorStop as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::AngularColorStop>,
                other: Ref<::bevy_ui::AngularColorStop>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::AngularColorStop as ::std::cmp::PartialEq<
                            ::bevy_ui::AngularColorStop,
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
            |_self: Val<::bevy_ui::AngularColorStop>, hint: f32| {
                let output: Val<::bevy_ui::AngularColorStop> = {
                    {
                        let output: Val<::bevy_ui::AngularColorStop> = ::bevy_ui::AngularColorStop::with_hint(
                                _self.into_inner(),
                                hint,
                            )
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
            ::bevy_ui::AngularColorStop,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_linear_gradient_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::LinearGradient,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::LinearGradient>| {
                let output: Val<::bevy_ui::LinearGradient> = {
                    {
                        let output: Val<::bevy_ui::LinearGradient> = <::bevy_ui::LinearGradient as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::LinearGradient>,
                other: Ref<::bevy_ui::LinearGradient>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::LinearGradient as ::std::cmp::PartialEq<
                            ::bevy_ui::LinearGradient,
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
            |
                _self: Val<::bevy_ui::LinearGradient>,
                color_space: Val<::bevy_ui::InterpolationColorSpace>|
            {
                let output: Val<::bevy_ui::LinearGradient> = {
                    {
                        let output: Val<::bevy_ui::LinearGradient> = ::bevy_ui::LinearGradient::in_color_space(
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
            ::bevy_ui::LinearGradient,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_interpolation_color_space_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::InterpolationColorSpace,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::InterpolationColorSpace>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::InterpolationColorSpace as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::InterpolationColorSpace>| {
                let output: Val<::bevy_ui::InterpolationColorSpace> = {
                    {
                        let output: Val<::bevy_ui::InterpolationColorSpace> = <::bevy_ui::InterpolationColorSpace as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::InterpolationColorSpace>,
                other: Ref<::bevy_ui::InterpolationColorSpace>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::InterpolationColorSpace as ::std::cmp::PartialEq<
                            ::bevy_ui::InterpolationColorSpace,
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
            ::bevy_ui::InterpolationColorSpace,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_radial_gradient_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::RadialGradient,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::RadialGradient>| {
                let output: Val<::bevy_ui::RadialGradient> = {
                    {
                        let output: Val<::bevy_ui::RadialGradient> = <::bevy_ui::RadialGradient as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::RadialGradient>,
                other: Ref<::bevy_ui::RadialGradient>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::RadialGradient as ::std::cmp::PartialEq<
                            ::bevy_ui::RadialGradient,
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
            |
                _self: Val<::bevy_ui::RadialGradient>,
                color_space: Val<::bevy_ui::InterpolationColorSpace>|
            {
                let output: Val<::bevy_ui::RadialGradient> = {
                    {
                        let output: Val<::bevy_ui::RadialGradient> = ::bevy_ui::RadialGradient::in_color_space(
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
            ::bevy_ui::RadialGradient,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_radial_gradient_shape_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::RadialGradientShape,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::RadialGradientShape>| {
                let output: Val<::bevy_ui::RadialGradientShape> = {
                    {
                        let output: Val<::bevy_ui::RadialGradientShape> = <::bevy_ui::RadialGradientShape as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::RadialGradientShape>,
                other: Ref<::bevy_ui::RadialGradientShape>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::RadialGradientShape as ::std::cmp::PartialEq<
                            ::bevy_ui::RadialGradientShape,
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
            "resolve",
            |
                _self: Val<::bevy_ui::RadialGradientShape>,
                position: Val<::glam::Vec2>,
                scale_factor: f32,
                physical_size: Val<::glam::Vec2>,
                physical_target_size: Val<::glam::Vec2>|
            {
                let output: Val<::glam::Vec2> = {
                    {
                        let output: Val<::glam::Vec2> = ::bevy_ui::RadialGradientShape::resolve(
                                _self.into_inner(),
                                position.into_inner(),
                                scale_factor,
                                physical_size.into_inner(),
                                physical_target_size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Resolve the physical dimensions of the end shape of the radial gradient",
            &[
                "_self",
                "position",
                "scale_factor",
                "physical_size",
                "physical_target_size",
            ],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::RadialGradientShape,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_conic_gradient_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::ConicGradient,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::ConicGradient>| {
                let output: Val<::bevy_ui::ConicGradient> = {
                    {
                        let output: Val<::bevy_ui::ConicGradient> = <::bevy_ui::ConicGradient as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::ConicGradient>, other: Ref<::bevy_ui::ConicGradient>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::ConicGradient as ::std::cmp::PartialEq<
                            ::bevy_ui::ConicGradient,
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
            |
                _self: Val<::bevy_ui::ConicGradient>,
                color_space: Val<::bevy_ui::InterpolationColorSpace>|
            {
                let output: Val<::bevy_ui::ConicGradient> = {
                    {
                        let output: Val<::bevy_ui::ConicGradient> = ::bevy_ui::ConicGradient::in_color_space(
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
            |_self: Val<::bevy_ui::ConicGradient>, position: Val<::bevy_ui::UiPosition>| {
                let output: Val<::bevy_ui::ConicGradient> = {
                    {
                        let output: Val<::bevy_ui::ConicGradient> = ::bevy_ui::ConicGradient::with_position(
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
            |_self: Val<::bevy_ui::ConicGradient>, start: f32| {
                let output: Val<::bevy_ui::ConicGradient> = {
                    {
                        let output: Val<::bevy_ui::ConicGradient> = ::bevy_ui::ConicGradient::with_start(
                                _self.into_inner(),
                                start,
                            )
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
            ::bevy_ui::ConicGradient,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gradient_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::Gradient,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::Gradient>| {
                let output: Val<::bevy_ui::Gradient> = {
                    {
                        let output: Val<::bevy_ui::Gradient> = <::bevy_ui::Gradient as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::Gradient>, other: Ref<::bevy_ui::Gradient>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::Gradient as ::std::cmp::PartialEq<
                            ::bevy_ui::Gradient,
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
            |_self: Ref<::bevy_ui::Gradient>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ui::Gradient::is_empty(&_self).into();
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
            ::bevy_ui::Gradient,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_background_gradient_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::BackgroundGradient,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::BackgroundGradient>| {
                let output: Val<::bevy_ui::BackgroundGradient> = {
                    {
                        let output: Val<::bevy_ui::BackgroundGradient> = <::bevy_ui::BackgroundGradient as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::BackgroundGradient>,
                other: Ref<::bevy_ui::BackgroundGradient>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::BackgroundGradient as ::std::cmp::PartialEq<
                            ::bevy_ui::BackgroundGradient,
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
            ::bevy_ui::BackgroundGradient,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_border_gradient_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::BorderGradient,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::BorderGradient>| {
                let output: Val<::bevy_ui::BorderGradient> = {
                    {
                        let output: Val<::bevy_ui::BorderGradient> = <::bevy_ui::BorderGradient as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::BorderGradient>,
                other: Ref<::bevy_ui::BorderGradient>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::BorderGradient as ::std::cmp::PartialEq<
                            ::bevy_ui::BorderGradient,
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
            ::bevy_ui::BorderGradient,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_val_2_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::Val2,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::Val2>| {
                let output: Val<::bevy_ui::Val2> = {
                    {
                        let output: Val<::bevy_ui::Val2> = <::bevy_ui::Val2 as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::Val2>, other: Ref<::bevy_ui::Val2>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::Val2 as ::std::cmp::PartialEq<
                            ::bevy_ui::Val2,
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
            |x: Val<::bevy_ui::Val>, y: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::Val2> = {
                    {
                        let output: Val<::bevy_ui::Val2> = ::bevy_ui::Val2::new(
                                x.into_inner(),
                                y.into_inner(),
                            )
                            .into();
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
                let output: Val<::bevy_ui::Val2> = {
                    {
                        let output: Val<::bevy_ui::Val2> = ::bevy_ui::Val2::percent(x, y)
                            .into();
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
                let output: Val<::bevy_ui::Val2> = {
                    {
                        let output: Val<::bevy_ui::Val2> = ::bevy_ui::Val2::px(x, y)
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Val2`] where both components are in logical pixels",
            &["x", "y"],
        )
        .register_documented(
            "resolve",
            |
                _self: Ref<::bevy_ui::Val2>,
                scale_factor: f32,
                base_size: Val<::glam::Vec2>,
                viewport_size: Val<::glam::Vec2>|
            {
                let output: Val<::glam::Vec2> = {
                    {
                        let output: Val<::glam::Vec2> = ::bevy_ui::Val2::resolve(
                                &_self,
                                scale_factor,
                                base_size.into_inner(),
                                viewport_size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Resolves this [`Val2`] from the given `scale_factor`, `parent_size`,\n and `viewport_size`.\n Component values of [`Val::Auto`] are resolved to 0.",
            &["_self", "scale_factor", "base_size", "viewport_size"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::Val2,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_transform_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::UiTransform,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::UiTransform>| {
                let output: Val<::bevy_ui::UiTransform> = {
                    {
                        let output: Val<::bevy_ui::UiTransform> = <::bevy_ui::UiTransform as ::std::clone::Clone>::clone(
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
            "compute_affine",
            |
                _self: Ref<::bevy_ui::UiTransform>,
                scale_factor: f32,
                base_size: Val<::glam::Vec2>,
                target_size: Val<::glam::Vec2>|
            {
                let output: Val<::glam::Affine2> = {
                    {
                        let output: Val<::glam::Affine2> = ::bevy_ui::UiTransform::compute_affine(
                                &_self,
                                scale_factor,
                                base_size.into_inner(),
                                target_size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Resolves the translation from the given `scale_factor`, `base_value`, and `target_size`\n and returns a 2d affine transform from the resolved translation, and the `UiTransform`'s rotation, and scale.",
            &["_self", "scale_factor", "base_size", "target_size"],
        )
        .register_documented(
            "eq",
            |_self: Ref<::bevy_ui::UiTransform>, other: Ref<::bevy_ui::UiTransform>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::UiTransform as ::std::cmp::PartialEq<
                            ::bevy_ui::UiTransform,
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
                let output: Val<::bevy_ui::UiTransform> = {
                    {
                        let output: Val<::bevy_ui::UiTransform> = ::bevy_ui::UiTransform::from_rotation(
                                rotation.into_inner(),
                            )
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
            "from_scale",
            |scale: Val<::glam::Vec2>| {
                let output: Val<::bevy_ui::UiTransform> = {
                    {
                        let output: Val<::bevy_ui::UiTransform> = ::bevy_ui::UiTransform::from_scale(
                                scale.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a UI transform representing a scaling.",
            &["scale"],
        )
        .register_documented(
            "from_translation",
            |translation: Val<::bevy_ui::Val2>| {
                let output: Val<::bevy_ui::UiTransform> = {
                    {
                        let output: Val<::bevy_ui::UiTransform> = ::bevy_ui::UiTransform::from_translation(
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
            ::bevy_ui::UiTransform,
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
                        let output: Val<::bevy_ui::RelativeCursorPosition> = <::bevy_ui::RelativeCursorPosition as ::std::clone::Clone>::clone(
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
            "cursor_over",
            |_self: Ref<::bevy_ui::RelativeCursorPosition>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ui::RelativeCursorPosition::cursor_over(
                                &_self,
                            )
                            .into();
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
            |
                _self: Ref<::bevy_ui::RelativeCursorPosition>,
                other: Ref<::bevy_ui::RelativeCursorPosition>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::RelativeCursorPosition as ::std::cmp::PartialEq<
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
        ::bevy_ui::UiRect,
    >::new(world)
        .register_documented(
            "all",
            |value: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::all(
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
            |horizontal: Val<::bevy_ui::Val>, vertical: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::axes(
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
            |bottom: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::bottom(
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
            |_self: Ref<::bevy_ui::UiRect>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = <::bevy_ui::UiRect as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::UiRect>, other: Ref<::bevy_ui::UiRect>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::UiRect as ::std::cmp::PartialEq<
                            ::bevy_ui::UiRect,
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
            |value: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::horizontal(
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
            |left: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::left(
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
                left: Val<::bevy_ui::Val>,
                right: Val<::bevy_ui::Val>,
                top: Val<::bevy_ui::Val>,
                bottom: Val<::bevy_ui::Val>|
            {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::new(
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
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::percent(
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
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::px(
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
            |right: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::right(
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
            |top: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::top(
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
            |value: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::vertical(
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
            |_self: Val<::bevy_ui::UiRect>, bottom: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::with_bottom(
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
            |_self: Val<::bevy_ui::UiRect>, left: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::with_left(
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
            |_self: Val<::bevy_ui::UiRect>, right: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::with_right(
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
            |_self: Val<::bevy_ui::UiRect>, top: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::UiRect> = {
                    {
                        let output: Val<::bevy_ui::UiRect> = ::bevy_ui::UiRect::with_top(
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
            ::bevy_ui::UiRect,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_border_radius_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::BorderRadius,
    >::new(world)
        .register_documented(
            "all",
            |radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::all(
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
            |radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::bottom(
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
            |radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::bottom_left(
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
            |radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::bottom_right(
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
            |_self: Ref<::bevy_ui::BorderRadius>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = <::bevy_ui::BorderRadius as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::BorderRadius>, other: Ref<::bevy_ui::BorderRadius>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::BorderRadius as ::std::cmp::PartialEq<
                            ::bevy_ui::BorderRadius,
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
            |radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::left(
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
                top_left: Val<::bevy_ui::Val>,
                top_right: Val<::bevy_ui::Val>,
                bottom_right: Val<::bevy_ui::Val>,
                bottom_left: Val<::bevy_ui::Val>|
            {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::new(
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
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::percent(
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
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::px(
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
            "resolve",
            |
                _self: Ref<::bevy_ui::BorderRadius>,
                scale_factor: f32,
                node_size: Val<::glam::Vec2>,
                viewport_size: Val<::glam::Vec2>|
            {
                let output: Val<::bevy_ui::ResolvedBorderRadius> = {
                    {
                        let output: Val<::bevy_ui::ResolvedBorderRadius> = ::bevy_ui::BorderRadius::resolve(
                                &_self,
                                scale_factor,
                                node_size.into_inner(),
                                viewport_size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Resolve the border radii for the corners from the given context values.\n Returns the radii of the each corner in physical pixels.",
            &["_self", "scale_factor", "node_size", "viewport_size"],
        )
        .register_documented(
            "resolve_single_corner",
            |
                radius: Val<::bevy_ui::Val>,
                scale_factor: f32,
                min_length: f32,
                viewport_size: Val<::glam::Vec2>|
            {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_ui::BorderRadius::resolve_single_corner(
                                radius.into_inner(),
                                scale_factor,
                                min_length,
                                viewport_size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Resolve the border radius for a single corner from the given context values.\n Returns the radius of the corner in physical pixels.",
            &["radius", "scale_factor", "min_length", "viewport_size"],
        )
        .register_documented(
            "right",
            |radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::right(
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
            |radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::top(
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
            |radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::top_left(
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
            |radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::top_right(
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
            |_self: Val<::bevy_ui::BorderRadius>, radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::with_bottom(
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
            |_self: Val<::bevy_ui::BorderRadius>, radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::with_bottom_left(
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
            |_self: Val<::bevy_ui::BorderRadius>, radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::with_bottom_right(
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
            |_self: Val<::bevy_ui::BorderRadius>, radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::with_left(
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
            |_self: Val<::bevy_ui::BorderRadius>, radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::with_right(
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
            |_self: Val<::bevy_ui::BorderRadius>, radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::with_top(
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
            |_self: Val<::bevy_ui::BorderRadius>, radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::with_top_left(
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
            |_self: Val<::bevy_ui::BorderRadius>, radius: Val<::bevy_ui::Val>| {
                let output: Val<::bevy_ui::BorderRadius> = {
                    {
                        let output: Val<::bevy_ui::BorderRadius> = ::bevy_ui::BorderRadius::with_top_right(
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
            ::bevy_ui::BorderRadius,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_layout_config_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::LayoutConfig,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::LayoutConfig>| {
                let output: Val<::bevy_ui::LayoutConfig> = {
                    {
                        let output: Val<::bevy_ui::LayoutConfig> = <::bevy_ui::LayoutConfig as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::LayoutConfig>, other: Ref<::bevy_ui::LayoutConfig>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::LayoutConfig as ::std::cmp::PartialEq<
                            ::bevy_ui::LayoutConfig,
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
            ::bevy_ui::LayoutConfig,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_outline_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::Outline,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::Outline>| {
                let output: Val<::bevy_ui::Outline> = {
                    {
                        let output: Val<::bevy_ui::Outline> = <::bevy_ui::Outline as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::Outline>, other: Ref<::bevy_ui::Outline>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::Outline as ::std::cmp::PartialEq<
                            ::bevy_ui::Outline,
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
                width: Val<::bevy_ui::Val>,
                offset: Val<::bevy_ui::Val>,
                color: Val<::bevy_color::Color>|
            {
                let output: Val<::bevy_ui::Outline> = {
                    {
                        let output: Val<::bevy_ui::Outline> = ::bevy_ui::Outline::new(
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
            ::bevy_ui::Outline,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scroll_position_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::ScrollPosition,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::ScrollPosition>| {
                let output: Val<::bevy_ui::ScrollPosition> = {
                    {
                        let output: Val<::bevy_ui::ScrollPosition> = <::bevy_ui::ScrollPosition as ::std::clone::Clone>::clone(
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
            ::bevy_ui::ScrollPosition,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_position_type_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::PositionType,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::PositionType>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::PositionType as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::PositionType>| {
                let output: Val<::bevy_ui::PositionType> = {
                    {
                        let output: Val<::bevy_ui::PositionType> = <::bevy_ui::PositionType as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::PositionType>, other: Ref<::bevy_ui::PositionType>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::PositionType as ::std::cmp::PartialEq<
                            ::bevy_ui::PositionType,
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
            ::bevy_ui::PositionType,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_align_self_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::AlignSelf,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::AlignSelf>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::AlignSelf as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::AlignSelf>| {
                let output: Val<::bevy_ui::AlignSelf> = {
                    {
                        let output: Val<::bevy_ui::AlignSelf> = <::bevy_ui::AlignSelf as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::AlignSelf>, other: Ref<::bevy_ui::AlignSelf>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::AlignSelf as ::std::cmp::PartialEq<
                            ::bevy_ui::AlignSelf,
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
            ::bevy_ui::AlignSelf,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_repeated_grid_track_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::RepeatedGridTrack,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::RepeatedGridTrack>| {
                let output: Val<::bevy_ui::RepeatedGridTrack> = {
                    {
                        let output: Val<::bevy_ui::RepeatedGridTrack> = <::bevy_ui::RepeatedGridTrack as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::RepeatedGridTrack>,
                other: Ref<::bevy_ui::RepeatedGridTrack>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::RepeatedGridTrack as ::std::cmp::PartialEq<
                            ::bevy_ui::RepeatedGridTrack,
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
            ::bevy_ui::RepeatedGridTrack,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_align_content_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::AlignContent,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::AlignContent>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::AlignContent as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::AlignContent>| {
                let output: Val<::bevy_ui::AlignContent> = {
                    {
                        let output: Val<::bevy_ui::AlignContent> = <::bevy_ui::AlignContent as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::AlignContent>, other: Ref<::bevy_ui::AlignContent>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::AlignContent as ::std::cmp::PartialEq<
                            ::bevy_ui::AlignContent,
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
            ::bevy_ui::AlignContent,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_align_items_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::AlignItems,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::AlignItems>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::AlignItems as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::AlignItems>| {
                let output: Val<::bevy_ui::AlignItems> = {
                    {
                        let output: Val<::bevy_ui::AlignItems> = <::bevy_ui::AlignItems as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::AlignItems>, other: Ref<::bevy_ui::AlignItems>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::AlignItems as ::std::cmp::PartialEq<
                            ::bevy_ui::AlignItems,
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
            ::bevy_ui::AlignItems,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_box_sizing_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::BoxSizing,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::BoxSizing>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::BoxSizing as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::BoxSizing>| {
                let output: Val<::bevy_ui::BoxSizing> = {
                    {
                        let output: Val<::bevy_ui::BoxSizing> = <::bevy_ui::BoxSizing as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::BoxSizing>, other: Ref<::bevy_ui::BoxSizing>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::BoxSizing as ::std::cmp::PartialEq<
                            ::bevy_ui::BoxSizing,
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
            ::bevy_ui::BoxSizing,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_flex_direction_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::FlexDirection,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::FlexDirection>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::FlexDirection as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::FlexDirection>| {
                let output: Val<::bevy_ui::FlexDirection> = {
                    {
                        let output: Val<::bevy_ui::FlexDirection> = <::bevy_ui::FlexDirection as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::FlexDirection>, other: Ref<::bevy_ui::FlexDirection>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::FlexDirection as ::std::cmp::PartialEq<
                            ::bevy_ui::FlexDirection,
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
            ::bevy_ui::FlexDirection,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_flex_wrap_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::FlexWrap,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::FlexWrap>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::FlexWrap as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::FlexWrap>| {
                let output: Val<::bevy_ui::FlexWrap> = {
                    {
                        let output: Val<::bevy_ui::FlexWrap> = <::bevy_ui::FlexWrap as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::FlexWrap>, other: Ref<::bevy_ui::FlexWrap>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::FlexWrap as ::std::cmp::PartialEq<
                            ::bevy_ui::FlexWrap,
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
            ::bevy_ui::FlexWrap,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_grid_auto_flow_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::GridAutoFlow,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::GridAutoFlow>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::GridAutoFlow as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::GridAutoFlow>| {
                let output: Val<::bevy_ui::GridAutoFlow> = {
                    {
                        let output: Val<::bevy_ui::GridAutoFlow> = <::bevy_ui::GridAutoFlow as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::GridAutoFlow>, other: Ref<::bevy_ui::GridAutoFlow>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::GridAutoFlow as ::std::cmp::PartialEq<
                            ::bevy_ui::GridAutoFlow,
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
            ::bevy_ui::GridAutoFlow,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_grid_placement_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::GridPlacement,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::GridPlacement>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::GridPlacement as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
                let output: Val<::bevy_ui::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::GridPlacement> = ::bevy_ui::GridPlacement::auto()
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
            |_self: Ref<::bevy_ui::GridPlacement>| {
                let output: Val<::bevy_ui::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::GridPlacement> = <::bevy_ui::GridPlacement as ::std::clone::Clone>::clone(
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
                let output: Val<::bevy_ui::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::GridPlacement> = ::bevy_ui::GridPlacement::end(
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
                let output: Val<::bevy_ui::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::GridPlacement> = ::bevy_ui::GridPlacement::end_span(
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
            |_self: Ref<::bevy_ui::GridPlacement>, other: Ref<::bevy_ui::GridPlacement>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::GridPlacement as ::std::cmp::PartialEq<
                            ::bevy_ui::GridPlacement,
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
            |_self: Val<::bevy_ui::GridPlacement>| {
                let output: ::std::option::Option<i16> = {
                    {
                        let output: ::std::option::Option<i16> = ::bevy_ui::GridPlacement::get_end(
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
            |_self: Val<::bevy_ui::GridPlacement>| {
                let output: ::std::option::Option<u16> = {
                    {
                        let output: ::std::option::Option<u16> = ::bevy_ui::GridPlacement::get_span(
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
            |_self: Val<::bevy_ui::GridPlacement>| {
                let output: ::std::option::Option<i16> = {
                    {
                        let output: ::std::option::Option<i16> = ::bevy_ui::GridPlacement::get_start(
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
            |_self: Val<::bevy_ui::GridPlacement>, end: i16| {
                let output: Val<::bevy_ui::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::GridPlacement> = ::bevy_ui::GridPlacement::set_end(
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
            |_self: Val<::bevy_ui::GridPlacement>, span: u16| {
                let output: Val<::bevy_ui::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::GridPlacement> = ::bevy_ui::GridPlacement::set_span(
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
            |_self: Val<::bevy_ui::GridPlacement>, start: i16| {
                let output: Val<::bevy_ui::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::GridPlacement> = ::bevy_ui::GridPlacement::set_start(
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
                let output: Val<::bevy_ui::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::GridPlacement> = ::bevy_ui::GridPlacement::span(
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
                let output: Val<::bevy_ui::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::GridPlacement> = ::bevy_ui::GridPlacement::start(
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
                let output: Val<::bevy_ui::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::GridPlacement> = ::bevy_ui::GridPlacement::start_end(
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
                let output: Val<::bevy_ui::GridPlacement> = {
                    {
                        let output: Val<::bevy_ui::GridPlacement> = ::bevy_ui::GridPlacement::start_span(
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
            ::bevy_ui::GridPlacement,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_grid_track_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::GridTrack,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::GridTrack>| {
                let output: Val<::bevy_ui::GridTrack> = {
                    {
                        let output: Val<::bevy_ui::GridTrack> = <::bevy_ui::GridTrack as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::GridTrack>, other: Ref<::bevy_ui::GridTrack>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::GridTrack as ::std::cmp::PartialEq<
                            ::bevy_ui::GridTrack,
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
            ::bevy_ui::GridTrack,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_grid_track_repetition_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::GridTrackRepetition,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::GridTrackRepetition>| {
                let output: Val<::bevy_ui::GridTrackRepetition> = {
                    {
                        let output: Val<::bevy_ui::GridTrackRepetition> = <::bevy_ui::GridTrackRepetition as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::GridTrackRepetition>,
                other: Ref<::bevy_ui::GridTrackRepetition>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::GridTrackRepetition as ::std::cmp::PartialEq<
                            ::bevy_ui::GridTrackRepetition,
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
            ::bevy_ui::GridTrackRepetition,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_justify_content_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::JustifyContent,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::JustifyContent>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::JustifyContent as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::JustifyContent>| {
                let output: Val<::bevy_ui::JustifyContent> = {
                    {
                        let output: Val<::bevy_ui::JustifyContent> = <::bevy_ui::JustifyContent as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::JustifyContent>,
                other: Ref<::bevy_ui::JustifyContent>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::JustifyContent as ::std::cmp::PartialEq<
                            ::bevy_ui::JustifyContent,
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
            ::bevy_ui::JustifyContent,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_justify_items_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::JustifyItems,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::JustifyItems>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::JustifyItems as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::JustifyItems>| {
                let output: Val<::bevy_ui::JustifyItems> = {
                    {
                        let output: Val<::bevy_ui::JustifyItems> = <::bevy_ui::JustifyItems as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::JustifyItems>, other: Ref<::bevy_ui::JustifyItems>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::JustifyItems as ::std::cmp::PartialEq<
                            ::bevy_ui::JustifyItems,
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
            ::bevy_ui::JustifyItems,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_justify_self_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::JustifySelf,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::JustifySelf>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::JustifySelf as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::JustifySelf>| {
                let output: Val<::bevy_ui::JustifySelf> = {
                    {
                        let output: Val<::bevy_ui::JustifySelf> = <::bevy_ui::JustifySelf as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::JustifySelf>, other: Ref<::bevy_ui::JustifySelf>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::JustifySelf as ::std::cmp::PartialEq<
                            ::bevy_ui::JustifySelf,
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
            ::bevy_ui::JustifySelf,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_max_track_sizing_function_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::MaxTrackSizingFunction,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::MaxTrackSizingFunction>| {
                let output: Val<::bevy_ui::MaxTrackSizingFunction> = {
                    {
                        let output: Val<::bevy_ui::MaxTrackSizingFunction> = <::bevy_ui::MaxTrackSizingFunction as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::MaxTrackSizingFunction>,
                other: Ref<::bevy_ui::MaxTrackSizingFunction>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::MaxTrackSizingFunction as ::std::cmp::PartialEq<
                            ::bevy_ui::MaxTrackSizingFunction,
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
            ::bevy_ui::MaxTrackSizingFunction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_min_track_sizing_function_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::MinTrackSizingFunction,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::MinTrackSizingFunction>| {
                let output: Val<::bevy_ui::MinTrackSizingFunction> = {
                    {
                        let output: Val<::bevy_ui::MinTrackSizingFunction> = <::bevy_ui::MinTrackSizingFunction as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::MinTrackSizingFunction>,
                other: Ref<::bevy_ui::MinTrackSizingFunction>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::MinTrackSizingFunction as ::std::cmp::PartialEq<
                            ::bevy_ui::MinTrackSizingFunction,
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
            ::bevy_ui::MinTrackSizingFunction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_overflow_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::Overflow,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::Overflow>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::Overflow as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
                let output: Val<::bevy_ui::Overflow> = {
                    {
                        let output: Val<::bevy_ui::Overflow> = ::bevy_ui::Overflow::clip()
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
                let output: Val<::bevy_ui::Overflow> = {
                    {
                        let output: Val<::bevy_ui::Overflow> = ::bevy_ui::Overflow::clip_x()
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
                let output: Val<::bevy_ui::Overflow> = {
                    {
                        let output: Val<::bevy_ui::Overflow> = ::bevy_ui::Overflow::clip_y()
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
            |_self: Ref<::bevy_ui::Overflow>| {
                let output: Val<::bevy_ui::Overflow> = {
                    {
                        let output: Val<::bevy_ui::Overflow> = <::bevy_ui::Overflow as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::Overflow>, other: Ref<::bevy_ui::Overflow>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::Overflow as ::std::cmp::PartialEq<
                            ::bevy_ui::Overflow,
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
                let output: Val<::bevy_ui::Overflow> = {
                    {
                        let output: Val<::bevy_ui::Overflow> = ::bevy_ui::Overflow::hidden()
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
                let output: Val<::bevy_ui::Overflow> = {
                    {
                        let output: Val<::bevy_ui::Overflow> = ::bevy_ui::Overflow::hidden_x()
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
                let output: Val<::bevy_ui::Overflow> = {
                    {
                        let output: Val<::bevy_ui::Overflow> = ::bevy_ui::Overflow::hidden_y()
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
            |_self: Ref<::bevy_ui::Overflow>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ui::Overflow::is_visible(&_self)
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
                let output: Val<::bevy_ui::Overflow> = {
                    {
                        let output: Val<::bevy_ui::Overflow> = ::bevy_ui::Overflow::scroll()
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
                let output: Val<::bevy_ui::Overflow> = {
                    {
                        let output: Val<::bevy_ui::Overflow> = ::bevy_ui::Overflow::scroll_x()
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
                let output: Val<::bevy_ui::Overflow> = {
                    {
                        let output: Val<::bevy_ui::Overflow> = ::bevy_ui::Overflow::scroll_y()
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
                let output: Val<::bevy_ui::Overflow> = {
                    {
                        let output: Val<::bevy_ui::Overflow> = ::bevy_ui::Overflow::visible()
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
            ::bevy_ui::Overflow,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_overflow_clip_margin_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::OverflowClipMargin,
    >::new(world)
        .register_documented(
            "border_box",
            || {
                let output: Val<::bevy_ui::OverflowClipMargin> = {
                    {
                        let output: Val<::bevy_ui::OverflowClipMargin> = ::bevy_ui::OverflowClipMargin::border_box()
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
            |_self: Ref<::bevy_ui::OverflowClipMargin>| {
                let output: Val<::bevy_ui::OverflowClipMargin> = {
                    {
                        let output: Val<::bevy_ui::OverflowClipMargin> = <::bevy_ui::OverflowClipMargin as ::std::clone::Clone>::clone(
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
                let output: Val<::bevy_ui::OverflowClipMargin> = {
                    {
                        let output: Val<::bevy_ui::OverflowClipMargin> = ::bevy_ui::OverflowClipMargin::content_box()
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
                _self: Ref<::bevy_ui::OverflowClipMargin>,
                other: Ref<::bevy_ui::OverflowClipMargin>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::OverflowClipMargin as ::std::cmp::PartialEq<
                            ::bevy_ui::OverflowClipMargin,
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
                let output: Val<::bevy_ui::OverflowClipMargin> = {
                    {
                        let output: Val<::bevy_ui::OverflowClipMargin> = ::bevy_ui::OverflowClipMargin::padding_box()
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
            |_self: Val<::bevy_ui::OverflowClipMargin>, margin: f32| {
                let output: Val<::bevy_ui::OverflowClipMargin> = {
                    {
                        let output: Val<::bevy_ui::OverflowClipMargin> = ::bevy_ui::OverflowClipMargin::with_margin(
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
            ::bevy_ui::OverflowClipMargin,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_global_z_index_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::GlobalZIndex,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::GlobalZIndex>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::GlobalZIndex as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::GlobalZIndex>| {
                let output: Val<::bevy_ui::GlobalZIndex> = {
                    {
                        let output: Val<::bevy_ui::GlobalZIndex> = <::bevy_ui::GlobalZIndex as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::GlobalZIndex>, other: Ref<::bevy_ui::GlobalZIndex>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::GlobalZIndex as ::std::cmp::PartialEq<
                            ::bevy_ui::GlobalZIndex,
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
            ::bevy_ui::GlobalZIndex,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_z_index_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::ZIndex,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_ui::ZIndex>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui::ZIndex as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_ui::ZIndex>| {
                let output: Val<::bevy_ui::ZIndex> = {
                    {
                        let output: Val<::bevy_ui::ZIndex> = <::bevy_ui::ZIndex as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::ZIndex>, other: Ref<::bevy_ui::ZIndex>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::ZIndex as ::std::cmp::PartialEq<
                            ::bevy_ui::ZIndex,
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
            ::bevy_ui::ZIndex,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_resolved_border_radius_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::ResolvedBorderRadius,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::ResolvedBorderRadius>| {
                let output: Val<::bevy_ui::ResolvedBorderRadius> = {
                    {
                        let output: Val<::bevy_ui::ResolvedBorderRadius> = <::bevy_ui::ResolvedBorderRadius as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::ResolvedBorderRadius>,
                other: Ref<::bevy_ui::ResolvedBorderRadius>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::ResolvedBorderRadius as ::std::cmp::PartialEq<
                            ::bevy_ui::ResolvedBorderRadius,
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
            ::bevy_ui::ResolvedBorderRadius,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_background_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::BackgroundColor,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::BackgroundColor>| {
                let output: Val<::bevy_ui::BackgroundColor> = {
                    {
                        let output: Val<::bevy_ui::BackgroundColor> = <::bevy_ui::BackgroundColor as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_ui::BackgroundColor>,
                other: Ref<::bevy_ui::BackgroundColor>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::BackgroundColor as ::std::cmp::PartialEq<
                            ::bevy_ui::BackgroundColor,
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
            ::bevy_ui::BackgroundColor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_border_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::BorderColor,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::BorderColor>| {
                let output: Val<::bevy_ui::BorderColor> = {
                    {
                        let output: Val<::bevy_ui::BorderColor> = <::bevy_ui::BorderColor as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::BorderColor>, other: Ref<::bevy_ui::BorderColor>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::BorderColor as ::std::cmp::PartialEq<
                            ::bevy_ui::BorderColor,
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
            |_self: Ref<::bevy_ui::BorderColor>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_ui::BorderColor::is_fully_transparent(
                                &_self,
                            )
                            .into();
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
            ::bevy_ui::BorderColor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_box_shadow_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::BoxShadow,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::BoxShadow>| {
                let output: Val<::bevy_ui::BoxShadow> = {
                    {
                        let output: Val<::bevy_ui::BoxShadow> = <::bevy_ui::BoxShadow as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::BoxShadow>, other: Ref<::bevy_ui::BoxShadow>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::BoxShadow as ::std::cmp::PartialEq<
                            ::bevy_ui::BoxShadow,
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
                color: Val<::bevy_color::Color>,
                x_offset: Val<::bevy_ui::Val>,
                y_offset: Val<::bevy_ui::Val>,
                spread_radius: Val<::bevy_ui::Val>,
                blur_radius: Val<::bevy_ui::Val>|
            {
                let output: Val<::bevy_ui::BoxShadow> = {
                    {
                        let output: Val<::bevy_ui::BoxShadow> = ::bevy_ui::BoxShadow::new(
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
            &["color", "x_offset", "y_offset", "spread_radius", "blur_radius"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_ui::BoxShadow,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_shadow_style_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui::ShadowStyle,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_ui::ShadowStyle>| {
                let output: Val<::bevy_ui::ShadowStyle> = {
                    {
                        let output: Val<::bevy_ui::ShadowStyle> = <::bevy_ui::ShadowStyle as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_ui::ShadowStyle>, other: Ref<::bevy_ui::ShadowStyle>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui::ShadowStyle as ::std::cmp::PartialEq<
                            ::bevy_ui::ShadowStyle,
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
            ::bevy_ui::ShadowStyle,
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
