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
pub struct BevySpriteScriptingPlugin;
pub(crate) fn register_sprite_picking_camera_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::prelude::SpritePickingCamera,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_sprite::prelude::SpritePickingCamera>| {
                let output: Val<::bevy_sprite::prelude::SpritePickingCamera> = {
                    {
                        let output: Val<::bevy_sprite::prelude::SpritePickingCamera> = <::bevy_sprite::prelude::SpritePickingCamera as ::std::clone::Clone>::clone(
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
            ::bevy_sprite::prelude::SpritePickingCamera,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sprite_picking_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::prelude::SpritePickingMode,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_sprite::prelude::SpritePickingMode>| {
            let output: Val<::bevy_sprite::prelude::SpritePickingMode> = {
                {
                    let output: Val<::bevy_sprite::prelude::SpritePickingMode> =
                        <::bevy_sprite::prelude::SpritePickingMode as ::std::clone::Clone>::clone(
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
            ::bevy_sprite::prelude::SpritePickingMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sprite_picking_settings_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::prelude::SpritePickingSettings,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_sprite::prelude::SpritePickingSettings,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_text_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::prelude::Text2d,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_sprite::prelude::Text2d>| {
            let output: Val<::bevy_sprite::prelude::Text2d> = {
                {
                    let output: Val<::bevy_sprite::prelude::Text2d> =
                        <::bevy_sprite::prelude::Text2d as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_sprite::prelude::Text2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sprite_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::prelude::Sprite,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_sprite::prelude::Sprite>| {
            let output: Val<::bevy_sprite::prelude::Sprite> = {
                {
                    let output: Val<::bevy_sprite::prelude::Sprite> =
                        <::bevy_sprite::prelude::Sprite as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_sprite::prelude::Sprite,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sprite_image_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::prelude::SpriteImageMode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_sprite::prelude::SpriteImageMode>| {
                let output: Val<::bevy_sprite::prelude::SpriteImageMode> = {
                    {
                        let output: Val<::bevy_sprite::prelude::SpriteImageMode> = <::bevy_sprite::prelude::SpriteImageMode as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_sprite::prelude::SpriteImageMode>,
                other: Ref<::bevy_sprite::prelude::SpriteImageMode>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite::prelude::SpriteImageMode as ::std::cmp::PartialEq<
                            ::bevy_sprite::prelude::SpriteImageMode,
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
            |_self: Ref<::bevy_sprite::prelude::SpriteImageMode>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_sprite::prelude::SpriteImageMode::uses_slices(
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
            ::bevy_sprite::prelude::SpriteImageMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_border_rect_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::prelude::BorderRect,
    >::new(world)
        .register_documented(
            "add",
            |
                _self: Val<::bevy_sprite::prelude::BorderRect>,
                rhs: Val<::bevy_sprite::prelude::BorderRect>|
            {
                let output: Val<::bevy_sprite::prelude::BorderRect> = {
                    {
                        let output: Val<::bevy_sprite::prelude::BorderRect> = <::bevy_sprite::prelude::BorderRect as ::std::ops::Add<
                            ::bevy_sprite::prelude::BorderRect,
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
            "all",
            |extent: f32| {
                let output: Val<::bevy_sprite::prelude::BorderRect> = {
                    {
                        let output: Val<::bevy_sprite::prelude::BorderRect> = ::bevy_sprite::prelude::BorderRect::all(
                                extent,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a border with the same `extent` along each edge",
            &["extent"],
        )
        .register_documented(
            "axes",
            |horizontal: f32, vertical: f32| {
                let output: Val<::bevy_sprite::prelude::BorderRect> = {
                    {
                        let output: Val<::bevy_sprite::prelude::BorderRect> = ::bevy_sprite::prelude::BorderRect::axes(
                                horizontal,
                                vertical,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new border with the `left` and `right` extents equal to `horizontal`, and `top` and `bottom` extents equal to `vertical`.",
            &["horizontal", "vertical"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_sprite::prelude::BorderRect>| {
                let output: Val<::bevy_sprite::prelude::BorderRect> = {
                    {
                        let output: Val<::bevy_sprite::prelude::BorderRect> = <::bevy_sprite::prelude::BorderRect as ::std::clone::Clone>::clone(
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
            |_self: Val<::bevy_sprite::prelude::BorderRect>, rhs: f32| {
                let output: Val<::bevy_sprite::prelude::BorderRect> = {
                    {
                        let output: Val<::bevy_sprite::prelude::BorderRect> = <::bevy_sprite::prelude::BorderRect as ::std::ops::Div<
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
                _self: Ref<::bevy_sprite::prelude::BorderRect>,
                other: Ref<::bevy_sprite::prelude::BorderRect>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite::prelude::BorderRect as ::std::cmp::PartialEq<
                            ::bevy_sprite::prelude::BorderRect,
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
            |_self: Val<::bevy_sprite::prelude::BorderRect>, rhs: f32| {
                let output: Val<::bevy_sprite::prelude::BorderRect> = {
                    {
                        let output: Val<::bevy_sprite::prelude::BorderRect> = <::bevy_sprite::prelude::BorderRect as ::std::ops::Mul<
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
            "sub",
            |
                _self: Val<::bevy_sprite::prelude::BorderRect>,
                rhs: Val<::bevy_sprite::prelude::BorderRect>|
            {
                let output: Val<::bevy_sprite::prelude::BorderRect> = {
                    {
                        let output: Val<::bevy_sprite::prelude::BorderRect> = <::bevy_sprite::prelude::BorderRect as ::std::ops::Sub<
                            ::bevy_sprite::prelude::BorderRect,
                        >>::sub(_self.into_inner(), rhs.into_inner())
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
            ::bevy_sprite::prelude::BorderRect,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_slice_scale_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::prelude::SliceScaleMode,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_sprite::prelude::SliceScaleMode>| {
            let output: Val<::bevy_sprite::prelude::SliceScaleMode> = {
                {
                    let output: Val<::bevy_sprite::prelude::SliceScaleMode> =
                        <::bevy_sprite::prelude::SliceScaleMode as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_sprite::prelude::SliceScaleMode>,
         other: Ref<::bevy_sprite::prelude::SliceScaleMode>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_sprite::prelude::SliceScaleMode as ::std::cmp::PartialEq<
                            ::bevy_sprite::prelude::SliceScaleMode,
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
            ::bevy_sprite::prelude::SliceScaleMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_texture_slicer_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::prelude::TextureSlicer,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_sprite::prelude::TextureSlicer>| {
            let output: Val<::bevy_sprite::prelude::TextureSlicer> = {
                {
                    let output: Val<::bevy_sprite::prelude::TextureSlicer> =
                        <::bevy_sprite::prelude::TextureSlicer as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_sprite::prelude::TextureSlicer>,
         other: Ref<::bevy_sprite::prelude::TextureSlicer>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_sprite::prelude::TextureSlicer as ::std::cmp::PartialEq<
                            ::bevy_sprite::prelude::TextureSlicer,
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
            ::bevy_sprite::prelude::TextureSlicer,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scaling_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::prelude::ScalingMode,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_sprite::prelude::ScalingMode>| {
            let output: Val<::bevy_sprite::prelude::ScalingMode> = {
                {
                    let output: Val<::bevy_sprite::prelude::ScalingMode> =
                        <::bevy_sprite::prelude::ScalingMode as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_sprite::prelude::ScalingMode>,
         other: Ref<::bevy_sprite::prelude::ScalingMode>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_sprite::prelude::ScalingMode as ::std::cmp::PartialEq<
                            ::bevy_sprite::prelude::ScalingMode,
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
            ::bevy_sprite::prelude::ScalingMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_anchor_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::Anchor,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_sprite::Anchor>| {
                let output: Val<::bevy_sprite::Anchor> = {
                    {
                        let output: Val<::bevy_sprite::Anchor> = <::bevy_sprite::Anchor as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_sprite::Anchor>, other: Ref<::bevy_sprite::Anchor>| {
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
            |_self: Ref<::bevy_sprite::Text2dShadow>| {
                let output: Val<::bevy_sprite::Text2dShadow> = {
                    {
                        let output: Val<::bevy_sprite::Text2dShadow> = <::bevy_sprite::Text2dShadow as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_sprite::Text2dShadow>,
                other: Ref<::bevy_sprite::Text2dShadow>|
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
        register_scaling_mode_functions(&mut world);
        register_anchor_functions(&mut world);
        register_text_2_d_shadow_functions(&mut world);
    }
}
