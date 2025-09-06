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
    )
    .register_documented(
        "sized",
        |custom_size: Val<::bevy_math::Vec2>| {
            let output: Val<::bevy_sprite::prelude::Sprite> = {
                {
                    let output: Val<::bevy_sprite::prelude::Sprite> =
                        ::bevy_sprite::prelude::Sprite::sized(custom_size.into_inner()).into();
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
pub(crate) fn register_color_material_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::prelude::ColorMaterial,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_sprite::prelude::ColorMaterial>| {
            let output: Val<::bevy_sprite::prelude::ColorMaterial> = {
                {
                    let output: Val<::bevy_sprite::prelude::ColorMaterial> =
                        <::bevy_sprite::prelude::ColorMaterial as ::std::clone::Clone>::clone(
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
            ::bevy_sprite::prelude::ColorMaterial,
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
            "as_vec",
            |_self: Ref<::bevy_sprite::Anchor>| {
                let output: Val<::bevy_math::Vec2> = {
                    {
                        let output: Val<::bevy_math::Vec2> = ::bevy_sprite::Anchor::as_vec(
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
pub(crate) fn register_alpha_mode_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::AlphaMode2d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_sprite::AlphaMode2d>| {
                let output: Val<::bevy_sprite::AlphaMode2d> = {
                    {
                        let output: Val<::bevy_sprite::AlphaMode2d> = <::bevy_sprite::AlphaMode2d as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_sprite::AlphaMode2d>,
                other: Ref<::bevy_sprite::AlphaMode2d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite::AlphaMode2d as ::std::cmp::PartialEq<
                            ::bevy_sprite::AlphaMode2d,
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
            ::bevy_sprite::AlphaMode2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_2_d_material_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::Wireframe2dMaterial,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_sprite::Wireframe2dMaterial>| {
            let output: Val<::bevy_sprite::Wireframe2dMaterial> = {
                {
                    let output: Val<::bevy_sprite::Wireframe2dMaterial> =
                        <::bevy_sprite::Wireframe2dMaterial as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_sprite::Wireframe2dMaterial,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_no_wireframe_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::NoWireframe2d,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_sprite::NoWireframe2d>| {
                let output: () = {
                    {
                        let output: () = <::bevy_sprite::NoWireframe2d as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_sprite::NoWireframe2d>| {
                let output: Val<::bevy_sprite::NoWireframe2d> = {
                    {
                        let output: Val<::bevy_sprite::NoWireframe2d> = <::bevy_sprite::NoWireframe2d as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_sprite::NoWireframe2d>,
                other: Ref<::bevy_sprite::NoWireframe2d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite::NoWireframe2d as ::std::cmp::PartialEq<
                            ::bevy_sprite::NoWireframe2d,
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
            ::bevy_sprite::NoWireframe2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_2_d_config_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::Wireframe2dConfig,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_sprite::Wireframe2dConfig>| {
            let output: Val<::bevy_sprite::Wireframe2dConfig> = {
                {
                    let output: Val<::bevy_sprite::Wireframe2dConfig> =
                        <::bevy_sprite::Wireframe2dConfig as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_sprite::Wireframe2dConfig,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_2_d_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::Wireframe2dColor,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_sprite::Wireframe2dColor>| {
            let output: Val<::bevy_sprite::Wireframe2dColor> = {
                {
                    let output: Val<::bevy_sprite::Wireframe2dColor> =
                        <::bevy_sprite::Wireframe2dColor as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_sprite::Wireframe2dColor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::Wireframe2d,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_sprite::Wireframe2d>| {
                let output: () = {
                    {
                        let output: () = <::bevy_sprite::Wireframe2d as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_sprite::Wireframe2d>| {
                let output: Val<::bevy_sprite::Wireframe2d> = {
                    {
                        let output: Val<::bevy_sprite::Wireframe2d> = <::bevy_sprite::Wireframe2d as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_sprite::Wireframe2d>,
                other: Ref<::bevy_sprite::Wireframe2d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite::Wireframe2d as ::std::cmp::PartialEq<
                            ::bevy_sprite::Wireframe2d,
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
            ::bevy_sprite::Wireframe2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_mesh_2_d_wireframe_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite::Mesh2dWireframe,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_sprite::Mesh2dWireframe>| {
                let output: () = {
                    {
                        let output: () = <::bevy_sprite::Mesh2dWireframe as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_sprite::Mesh2dWireframe>| {
                let output: Val<::bevy_sprite::Mesh2dWireframe> = {
                    {
                        let output: Val<::bevy_sprite::Mesh2dWireframe> = <::bevy_sprite::Mesh2dWireframe as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_sprite::Mesh2dWireframe>,
                other: Ref<::bevy_sprite::Mesh2dWireframe>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite::Mesh2dWireframe as ::std::cmp::PartialEq<
                            ::bevy_sprite::Mesh2dWireframe,
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
            ::bevy_sprite::Mesh2dWireframe,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevySpriteScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_sprite_picking_camera_functions(&mut world);
        register_sprite_picking_mode_functions(&mut world);
        register_sprite_picking_settings_functions(&mut world);
        register_sprite_functions(&mut world);
        register_sprite_image_mode_functions(&mut world);
        register_border_rect_functions(&mut world);
        register_slice_scale_mode_functions(&mut world);
        register_texture_slicer_functions(&mut world);
        register_color_material_functions(&mut world);
        register_scaling_mode_functions(&mut world);
        register_anchor_functions(&mut world);
        register_alpha_mode_2_d_functions(&mut world);
        register_wireframe_2_d_material_functions(&mut world);
        register_no_wireframe_2_d_functions(&mut world);
        register_wireframe_2_d_config_functions(&mut world);
        register_wireframe_2_d_color_functions(&mut world);
        register_wireframe_2_d_functions(&mut world);
        register_mesh_2_d_wireframe_functions(&mut world);
    }
}
