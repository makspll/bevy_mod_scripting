#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]

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
pub struct BevySpriteScriptingPlugin;
pub(crate) fn register_sprite_picking_camera_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::SpritePickingCamera,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_sprite::SpritePickingCamera>| {
            let output: V<::bevy_sprite::SpritePickingCamera> = {
                {
                    let output: V<::bevy_sprite::SpritePickingCamera> =
                        <::bevy_sprite::SpritePickingCamera as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_sprite::SpritePickingCamera,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sprite_picking_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::SpritePickingMode,
    >::new(world)
    .register_documented(
        "clone",
        |mut _self: R<::bevy_sprite::SpritePickingMode>| {
            let output: V<::bevy_sprite::SpritePickingMode> = {
                {
                    let output: V<::bevy_sprite::SpritePickingMode> =
                        <::bevy_sprite::SpritePickingMode as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_sprite::SpritePickingMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sprite_picking_settings_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::SpritePickingSettings,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_sprite::SpritePickingSettings,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::Text2d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_sprite::Text2d>| {
                let output: V<::bevy_sprite::Text2d> = {
                    {
                        let output: V<::bevy_sprite::Text2d> = <::bevy_sprite::Text2d as ::std::clone::Clone>::clone(
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
        .register_type_data::<::bevy_sprite::Text2d, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_sprite_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::Sprite,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_sprite::Sprite>| {
                let output: V<::bevy_sprite::Sprite> = {
                    {
                        let output: V<::bevy_sprite::Sprite> = <::bevy_sprite::Sprite as ::std::clone::Clone>::clone(
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
            "sized",
            |custom_size: V<::bevy_math::Vec2>| {
                let output: V<::bevy_sprite::Sprite> = {
                    {
                        let output: V<::bevy_sprite::Sprite> = ::bevy_sprite::Sprite::sized(
                                custom_size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a Sprite with a custom size",
            &["custom_size"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_sprite::Sprite, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_sprite_image_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::SpriteImageMode,
    >::new(world)
        .register_documented(
            "clone",
            |mut _self: R<::bevy_sprite::SpriteImageMode>| {
                let output: V<::bevy_sprite::SpriteImageMode> = {
                    {
                        let output: V<::bevy_sprite::SpriteImageMode> = <::bevy_sprite::SpriteImageMode as ::std::clone::Clone>::clone(
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
                mut _self: R<::bevy_sprite::SpriteImageMode>,
                mut other: R<::bevy_sprite::SpriteImageMode>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite::SpriteImageMode as ::std::cmp::PartialEq<
                            ::bevy_sprite::SpriteImageMode,
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
            |mut _self: R<::bevy_sprite::SpriteImageMode>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_sprite::SpriteImageMode::uses_slices(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if this mode uses slices internally ([`SpriteImageMode::Sliced`] or [`SpriteImageMode::Tiled`])",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_sprite::SpriteImageMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_border_rect_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::BorderRect,
    >::new(world)
        .register_documented(
            "add",
            |_self: V<::bevy_sprite::BorderRect>, rhs: V<::bevy_sprite::BorderRect>| {
                let output: V<::bevy_sprite::BorderRect> = {
                    {
                        let output: V<::bevy_sprite::BorderRect> = <::bevy_sprite::BorderRect as ::std::ops::Add<
                            ::bevy_sprite::BorderRect,
                        >>::add(&_self, &rhs)
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
            "all",
            |inset: f32| {
                let output: V<::bevy_sprite::BorderRect> = {
                    {
                        let output: V<::bevy_sprite::BorderRect> = ::bevy_sprite::BorderRect::all(
                                inset,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a border with the same `inset` along each edge",
            &["inset"],
        )
        .register_documented(
            "axes",
            |horizontal: f32, vertical: f32| {
                let output: V<::bevy_sprite::BorderRect> = {
                    {
                        let output: V<::bevy_sprite::BorderRect> = ::bevy_sprite::BorderRect::axes(
                                horizontal,
                                vertical,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new border with the `min.x` and `max.x` insets equal to `horizontal`, and the `min.y` and `max.y` insets equal to `vertical`.",
            &["horizontal", "vertical"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_sprite::BorderRect>| {
                let output: V<::bevy_sprite::BorderRect> = {
                    {
                        let output: V<::bevy_sprite::BorderRect> = <::bevy_sprite::BorderRect as ::std::clone::Clone>::clone(
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
            |_self: V<::bevy_sprite::BorderRect>, rhs: f32| {
                let output: V<::bevy_sprite::BorderRect> = {
                    {
                        let output: V<::bevy_sprite::BorderRect> = <::bevy_sprite::BorderRect as ::std::ops::Div<
                            f32,
                        >>::div(&_self, rhs)
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
            |_self: R<::bevy_sprite::BorderRect>, other: R<::bevy_sprite::BorderRect>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite::BorderRect as ::std::cmp::PartialEq<
                            ::bevy_sprite::BorderRect,
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
            |_self: V<::bevy_sprite::BorderRect>, rhs: f32| {
                let output: V<::bevy_sprite::BorderRect> = {
                    {
                        let output: V<::bevy_sprite::BorderRect> = <::bevy_sprite::BorderRect as ::std::ops::Mul<
                            f32,
                        >>::mul(&_self, rhs)
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
            "sub",
            |_self: V<::bevy_sprite::BorderRect>, rhs: V<::bevy_sprite::BorderRect>| {
                let output: V<::bevy_sprite::BorderRect> = {
                    {
                        let output: V<::bevy_sprite::BorderRect> = <::bevy_sprite::BorderRect as ::std::ops::Sub<
                            ::bevy_sprite::BorderRect,
                        >>::sub(&_self, &rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_sprite::BorderRect,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_slice_scale_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::SliceScaleMode,
    >::new(world)
    .register_documented(
        "clone",
        |mut _self: R<::bevy_sprite::SliceScaleMode>| {
            let output: V<::bevy_sprite::SliceScaleMode> = {
                {
                    let output: V<::bevy_sprite::SliceScaleMode> =
                        <::bevy_sprite::SliceScaleMode as ::std::clone::Clone>::clone(&_self)
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
        |mut _self: R<::bevy_sprite::SliceScaleMode>,
         mut other: R<::bevy_sprite::SliceScaleMode>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_sprite::SliceScaleMode as ::std::cmp::PartialEq<
                        ::bevy_sprite::SliceScaleMode,
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
            ::bevy_sprite::SliceScaleMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_texture_slicer_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::TextureSlicer,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_sprite::TextureSlicer>| {
            let output: V<::bevy_sprite::TextureSlicer> = {
                {
                    let output: V<::bevy_sprite::TextureSlicer> =
                        <::bevy_sprite::TextureSlicer as ::std::clone::Clone>::clone(&_self).into();
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
        |_self: R<::bevy_sprite::TextureSlicer>, other: R<::bevy_sprite::TextureSlicer>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_sprite::TextureSlicer as ::std::cmp::PartialEq<
                        ::bevy_sprite::TextureSlicer,
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
            ::bevy_sprite::TextureSlicer,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sprite_scaling_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::SpriteScalingMode,
    >::new(world)
    .register_documented(
        "clone",
        |mut _self: R<::bevy_sprite::SpriteScalingMode>| {
            let output: V<::bevy_sprite::SpriteScalingMode> = {
                {
                    let output: V<::bevy_sprite::SpriteScalingMode> =
                        <::bevy_sprite::SpriteScalingMode as ::std::clone::Clone>::clone(&_self)
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
        |mut _self: R<::bevy_sprite::SpriteScalingMode>,
         mut other: R<::bevy_sprite::SpriteScalingMode>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_sprite::SpriteScalingMode as ::std::cmp::PartialEq<
                            ::bevy_sprite::SpriteScalingMode,
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
            ::bevy_sprite::SpriteScalingMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_anchor_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::Anchor,
    >::new(world)
        .register_documented(
            "as_vec",
            |_self: R<::bevy_sprite::Anchor>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_sprite::Anchor::as_vec(
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
            |_self: R<::bevy_sprite::Anchor>| {
                let output: V<::bevy_sprite::Anchor> = {
                    {
                        let output: V<::bevy_sprite::Anchor> = <::bevy_sprite::Anchor as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_sprite::Anchor>, other: R<::bevy_sprite::Anchor>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite::Anchor as ::std::cmp::PartialEq<
                            ::bevy_sprite::Anchor,
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
        .register_type_data::<::bevy_sprite::Anchor, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_text_2_d_shadow_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::Text2dShadow,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_sprite::Text2dShadow>| {
                let output: V<::bevy_sprite::Text2dShadow> = {
                    {
                        let output: V<::bevy_sprite::Text2dShadow> = <::bevy_sprite::Text2dShadow as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_sprite::Text2dShadow>,
                other: R<::bevy_sprite::Text2dShadow>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite::Text2dShadow as ::std::cmp::PartialEq<
                            ::bevy_sprite::Text2dShadow,
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
            ::bevy_sprite::Text2dShadow,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevySpriteScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_sprite_picking_camera_functions(&mut world);
        register_sprite_picking_mode_functions(&mut world);
        register_sprite_picking_settings_functions(&mut world);
        register_text_2_d_functions(&mut world);
        register_sprite_functions(&mut world);
        register_sprite_image_mode_functions(&mut world);
        register_border_rect_functions(&mut world);
        register_slice_scale_mode_functions(&mut world);
        register_texture_slicer_functions(&mut world);
        register_sprite_scaling_mode_functions(&mut world);
        register_anchor_functions(&mut world);
        register_text_2_d_shadow_functions(&mut world);
    }
}
