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
pub struct BevyRenderScriptingPlugin;
pub(crate) fn register_alpha_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::alpha::AlphaMode,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::alpha::AlphaMode>| {
            let output: Val<::bevy_render::alpha::AlphaMode> = {
                {
                    let output: Val<::bevy_render::alpha::AlphaMode> =
                        <::bevy_render::alpha::AlphaMode as ::std::clone::Clone>::clone(&_self)
                            .into();
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
        |_self: Ref<::bevy_render::alpha::AlphaMode>,
         other: Ref<::bevy_render::alpha::AlphaMode>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_render::alpha::AlphaMode as ::std::cmp::PartialEq<
                        ::bevy_render::alpha::AlphaMode,
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
            ::bevy_render::alpha::AlphaMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::Camera,
    >::new(world)
        .register_documented(
            "clip_from_view",
            |_self: Ref<::bevy_render::camera::Camera>| {
                let output: Val<::bevy_math::Mat4> = {
                    {
                        let output: Val<::bevy_math::Mat4> = ::bevy_render::camera::Camera::clip_from_view(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The projection matrix computed using this camera's [`CameraProjection`].",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::camera::Camera>| {
                let output: Val<::bevy_render::camera::Camera> = {
                    {
                        let output: Val<::bevy_render::camera::Camera> = <::bevy_render::camera::Camera as ::std::clone::Clone>::clone(
                                &_self,
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
            "depth_ndc_to_view_z",
            |_self: Ref<::bevy_render::camera::Camera>, ndc_depth: f32| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_render::camera::Camera::depth_ndc_to_view_z(
                                &_self,
                                ndc_depth,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Converts the depth in Normalized Device Coordinates\n to linear view z for perspective projections.\n Note: Depth values in front of the camera will be negative as -z is forward",
            &["_self", "ndc_depth"],
        )
        .register_documented(
            "depth_ndc_to_view_z_2d",
            |_self: Ref<::bevy_render::camera::Camera>, ndc_depth: f32| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_render::camera::Camera::depth_ndc_to_view_z_2d(
                                &_self,
                                ndc_depth,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Converts the depth in Normalized Device Coordinates\n to linear view z for orthographic projections.\n Note: Depth values in front of the camera will be negative as -z is forward",
            &["_self", "ndc_depth"],
        )
        .register_documented(
            "target_scaling_factor",
            |_self: Ref<::bevy_render::camera::Camera>| {
                let output: ::std::option::Option<f32> = {
                    {
                        let output: ::std::option::Option<f32> = ::bevy_render::camera::Camera::target_scaling_factor(
                                &_self,
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
            ::bevy_render::camera::Camera,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_clear_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::ClearColor,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::camera::ClearColor>| {
            let output: Val<::bevy_render::camera::ClearColor> = {
                {
                    let output: Val<::bevy_render::camera::ClearColor> =
                        <::bevy_render::camera::ClearColor as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_render::camera::ClearColor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_clear_color_config_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::ClearColorConfig,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::camera::ClearColorConfig>| {
            let output: Val<::bevy_render::camera::ClearColorConfig> = {
                {
                    let output: Val<::bevy_render::camera::ClearColorConfig> =
                        <::bevy_render::camera::ClearColorConfig as ::std::clone::Clone>::clone(
                            &_self,
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
            ::bevy_render::camera::ClearColorConfig,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_orthographic_projection_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::OrthographicProjection,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::camera::OrthographicProjection>| {
                let output: Val<::bevy_render::camera::OrthographicProjection> = {
                    {
                        let output: Val<::bevy_render::camera::OrthographicProjection> = <::bevy_render::camera::OrthographicProjection as ::std::clone::Clone>::clone(
                                &_self,
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
            "default_2d",
            || {
                let output: Val<::bevy_render::camera::OrthographicProjection> = {
                    {
                        let output: Val<::bevy_render::camera::OrthographicProjection> = ::bevy_render::camera::OrthographicProjection::default_2d()
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the default orthographic projection for a 2D context.\n The near plane is set to a negative value so that the camera can still\n render the scene when using positive z coordinates to order foreground elements.",
            &[],
        )
        .register_documented(
            "default_3d",
            || {
                let output: Val<::bevy_render::camera::OrthographicProjection> = {
                    {
                        let output: Val<::bevy_render::camera::OrthographicProjection> = ::bevy_render::camera::OrthographicProjection::default_3d()
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the default orthographic projection for a 3D context.\n The near plane is set to 0.0 so that the camera doesn't render\n objects that are behind it.",
            &[],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::camera::OrthographicProjection,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_perspective_projection_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::PerspectiveProjection,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::camera::PerspectiveProjection>| {
                let output: Val<::bevy_render::camera::PerspectiveProjection> = {
                    {
                        let output: Val<::bevy_render::camera::PerspectiveProjection> = <::bevy_render::camera::PerspectiveProjection as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_render::camera::PerspectiveProjection,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_projection_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::Projection,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::camera::Projection>| {
            let output: Val<::bevy_render::camera::Projection> = {
                {
                    let output: Val<::bevy_render::camera::Projection> =
                        <::bevy_render::camera::Projection as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_render::camera::Projection,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_mesh_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::mesh::Mesh2d,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::mesh::Mesh2d>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::mesh::Mesh2d as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: Ref<::bevy_render::mesh::Mesh2d>| {
                let output: Val<::bevy_render::mesh::Mesh2d> = {
                    {
                        let output: Val<::bevy_render::mesh::Mesh2d> = <::bevy_render::mesh::Mesh2d as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: Ref<::bevy_render::mesh::Mesh2d>,
                other: Ref<::bevy_render::mesh::Mesh2d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::mesh::Mesh2d as ::std::cmp::PartialEq<
                            ::bevy_render::mesh::Mesh2d,
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
            ::bevy_render::mesh::Mesh2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_mesh_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::mesh::Mesh3d,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::mesh::Mesh3d>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::mesh::Mesh3d as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: Ref<::bevy_render::mesh::Mesh3d>| {
                let output: Val<::bevy_render::mesh::Mesh3d> = {
                    {
                        let output: Val<::bevy_render::mesh::Mesh3d> = <::bevy_render::mesh::Mesh3d as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: Ref<::bevy_render::mesh::Mesh3d>,
                other: Ref<::bevy_render::mesh::Mesh3d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::mesh::Mesh3d as ::std::cmp::PartialEq<
                            ::bevy_render::mesh::Mesh3d,
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
            ::bevy_render::mesh::Mesh3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_inherited_visibility_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::visibility::InheritedVisibility,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::view::visibility::InheritedVisibility>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::view::visibility::InheritedVisibility as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: Ref<::bevy_render::view::visibility::InheritedVisibility>| {
                let output: Val<::bevy_render::view::visibility::InheritedVisibility> = {
                    {
                        let output: Val<
                            ::bevy_render::view::visibility::InheritedVisibility,
                        > = <::bevy_render::view::visibility::InheritedVisibility as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: Ref<::bevy_render::view::visibility::InheritedVisibility>,
                other: Ref<::bevy_render::view::visibility::InheritedVisibility>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::view::visibility::InheritedVisibility as ::std::cmp::PartialEq<
                            ::bevy_render::view::visibility::InheritedVisibility,
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
            "get",
            |_self: Val<::bevy_render::view::visibility::InheritedVisibility>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_render::view::visibility::InheritedVisibility::get(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the entity is visible in the hierarchy.\n Otherwise, returns `false`.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::visibility::InheritedVisibility,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_msaa_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::Msaa,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::view::Msaa>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::view::Msaa as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: Ref<::bevy_render::view::Msaa>| {
                let output: Val<::bevy_render::view::Msaa> = {
                    {
                        let output: Val<::bevy_render::view::Msaa> = <::bevy_render::view::Msaa as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: Ref<::bevy_render::view::Msaa>,
                other: Ref<::bevy_render::view::Msaa>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::view::Msaa as ::std::cmp::PartialEq<
                            ::bevy_render::view::Msaa,
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
            "from_samples",
            |samples: u32| {
                let output: Val<::bevy_render::view::Msaa> = {
                    {
                        let output: Val<::bevy_render::view::Msaa> = ::bevy_render::view::Msaa::from_samples(
                                samples,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["samples"],
        )
        .register_documented(
            "samples",
            |_self: Ref<::bevy_render::view::Msaa>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_render::view::Msaa::samples(&_self)
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
            ::bevy_render::view::Msaa,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_view_visibility_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::visibility::ViewVisibility,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::view::visibility::ViewVisibility>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::view::visibility::ViewVisibility as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: Ref<::bevy_render::view::visibility::ViewVisibility>| {
                let output: Val<::bevy_render::view::visibility::ViewVisibility> = {
                    {
                        let output: Val<
                            ::bevy_render::view::visibility::ViewVisibility,
                        > = <::bevy_render::view::visibility::ViewVisibility as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: Ref<::bevy_render::view::visibility::ViewVisibility>,
                other: Ref<::bevy_render::view::visibility::ViewVisibility>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::view::visibility::ViewVisibility as ::std::cmp::PartialEq<
                            ::bevy_render::view::visibility::ViewVisibility,
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
            "get",
            |_self: Val<::bevy_render::view::visibility::ViewVisibility>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_render::view::visibility::ViewVisibility::get(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the entity is visible in any view.\n Otherwise, returns `false`.",
            &["_self"],
        )
        .register_documented(
            "set",
            |mut _self: Mut<::bevy_render::view::visibility::ViewVisibility>| {
                let output: () = {
                    {
                        let output: () = ::bevy_render::view::visibility::ViewVisibility::set(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the visibility to `true`. This should not be considered reversible for a given frame,\n as this component tracks whether or not the entity visible in _any_ view.\n This will be automatically reset to `false` every frame in [`VisibilityPropagate`] and then set\n to the proper value in [`CheckVisibility`].\n You should only manually set this if you are defining a custom visibility system,\n in which case the system should be placed in the [`CheckVisibility`] set.\n For normal user-defined entity visibility, see [`Visibility`].\n [`VisibilityPropagate`]: VisibilitySystems::VisibilityPropagate\n [`CheckVisibility`]: VisibilitySystems::CheckVisibility",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::visibility::ViewVisibility,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_visibility_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::visibility::Visibility,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::view::visibility::Visibility>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::view::visibility::Visibility as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: Ref<::bevy_render::view::visibility::Visibility>| {
                let output: Val<::bevy_render::view::visibility::Visibility> = {
                    {
                        let output: Val<::bevy_render::view::visibility::Visibility> = <::bevy_render::view::visibility::Visibility as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: Ref<::bevy_render::view::visibility::Visibility>,
                other: Ref<::bevy_render::view::visibility::Visibility>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::view::visibility::Visibility as ::std::cmp::PartialEq<
                            ::bevy_render::view::visibility::Visibility,
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
            "toggle_inherited_hidden",
            |mut _self: Mut<::bevy_render::view::visibility::Visibility>| {
                let output: () = {
                    {
                        let output: () = ::bevy_render::view::visibility::Visibility::toggle_inherited_hidden(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Toggles between `Visibility::Inherited` and `Visibility::Hidden`.\n If the value is `Visibility::Visible`, it remains unaffected.",
            &["_self"],
        )
        .register_documented(
            "toggle_inherited_visible",
            |mut _self: Mut<::bevy_render::view::visibility::Visibility>| {
                let output: () = {
                    {
                        let output: () = ::bevy_render::view::visibility::Visibility::toggle_inherited_visible(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Toggles between `Visibility::Inherited` and `Visibility::Visible`.\n If the value is `Visibility::Hidden`, it remains unaffected.",
            &["_self"],
        )
        .register_documented(
            "toggle_visible_hidden",
            |mut _self: Mut<::bevy_render::view::visibility::Visibility>| {
                let output: () = {
                    {
                        let output: () = ::bevy_render::view::visibility::Visibility::toggle_visible_hidden(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Toggles between `Visibility::Visible` and `Visibility::Hidden`.\n If the value is `Visibility::Inherited`, it remains unaffected.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::visibility::Visibility,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sync_to_render_world_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::sync_world::SyncToRenderWorld,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::sync_world::SyncToRenderWorld>| {
                let output: Val<::bevy_render::sync_world::SyncToRenderWorld> = {
                    {
                        let output: Val<::bevy_render::sync_world::SyncToRenderWorld> = <::bevy_render::sync_world::SyncToRenderWorld as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_render::sync_world::SyncToRenderWorld,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_aabb_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::primitives::Aabb,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::primitives::Aabb>| {
            let output: Val<::bevy_render::primitives::Aabb> = {
                {
                    let output: Val<::bevy_render::primitives::Aabb> =
                        <::bevy_render::primitives::Aabb as ::std::clone::Clone>::clone(&_self)
                            .into();
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
        |_self: Ref<::bevy_render::primitives::Aabb>,
         other: Ref<::bevy_render::primitives::Aabb>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_render::primitives::Aabb as ::std::cmp::PartialEq<
                        ::bevy_render::primitives::Aabb,
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
        "from_min_max",
        |minimum: Val<::bevy_math::Vec3>, maximum: Val<::bevy_math::Vec3>| {
            let output: Val<::bevy_render::primitives::Aabb> = {
                {
                    let output: Val<::bevy_render::primitives::Aabb> =
                        ::bevy_render::primitives::Aabb::from_min_max(
                            minimum.into_inner(),
                            maximum.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        "",
        &["minimum", "maximum"],
    )
    .register_documented(
        "max",
        |_self: Ref<::bevy_render::primitives::Aabb>| {
            let output: Val<::bevy_math::Vec3A> = {
                {
                    let output: Val<::bevy_math::Vec3A> =
                        ::bevy_render::primitives::Aabb::max(&_self).into();
                    output
                }
            };
            output
        },
        "",
        &["_self"],
    )
    .register_documented(
        "min",
        |_self: Ref<::bevy_render::primitives::Aabb>| {
            let output: Val<::bevy_math::Vec3A> = {
                {
                    let output: Val<::bevy_math::Vec3A> =
                        ::bevy_render::primitives::Aabb::min(&_self).into();
                    output
                }
            };
            output
        },
        "",
        &["_self"],
    )
    .register_documented(
        "relative_radius",
        |_self: Ref<::bevy_render::primitives::Aabb>,
         p_normal: Ref<::bevy_math::Vec3A>,
         world_from_local: Ref<::bevy_math::Mat3A>| {
            let output: f32 = {
                {
                    let output: f32 = ::bevy_render::primitives::Aabb::relative_radius(
                        &_self,
                        &p_normal,
                        &world_from_local,
                    )
                    .into();
                    output
                }
            };
            output
        },
        " Calculate the relative radius of the AABB with respect to a plane",
        &["_self", "p_normal", "world_from_local"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::primitives::Aabb,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cascades_frusta_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::primitives::CascadesFrusta,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::primitives::CascadesFrusta>| {
            let output: Val<::bevy_render::primitives::CascadesFrusta> = {
                {
                    let output: Val<::bevy_render::primitives::CascadesFrusta> =
                        <::bevy_render::primitives::CascadesFrusta as ::std::clone::Clone>::clone(
                            &_self,
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
            ::bevy_render::primitives::CascadesFrusta,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cubemap_frusta_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::primitives::CubemapFrusta,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::primitives::CubemapFrusta>| {
            let output: Val<::bevy_render::primitives::CubemapFrusta> = {
                {
                    let output: Val<::bevy_render::primitives::CubemapFrusta> =
                        <::bevy_render::primitives::CubemapFrusta as ::std::clone::Clone>::clone(
                            &_self,
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
            ::bevy_render::primitives::CubemapFrusta,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_frustum_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::primitives::Frustum,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::primitives::Frustum>| {
                let output: Val<::bevy_render::primitives::Frustum> = {
                    {
                        let output: Val<::bevy_render::primitives::Frustum> = <::bevy_render::primitives::Frustum as ::std::clone::Clone>::clone(
                                &_self,
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
            "contains_aabb",
            |
                _self: Ref<::bevy_render::primitives::Frustum>,
                aabb: Ref<::bevy_render::primitives::Aabb>,
                world_from_local: Ref<::bevy_math::Affine3A>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_render::primitives::Frustum::contains_aabb(
                                &_self,
                                &aabb,
                                &world_from_local,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Check if the frustum contains the Axis-Aligned Bounding Box (AABB).\n Referenced from: [Frustum Culling](https://learnopengl.com/Guest-Articles/2021/Scene/Frustum-Culling)",
            &["_self", "aabb", "world_from_local"],
        )
        .register_documented(
            "from_clip_from_world",
            |clip_from_world: Ref<::bevy_math::Mat4>| {
                let output: Val<::bevy_render::primitives::Frustum> = {
                    {
                        let output: Val<::bevy_render::primitives::Frustum> = ::bevy_render::primitives::Frustum::from_clip_from_world(
                                &clip_from_world,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a frustum derived from `clip_from_world`.",
            &["clip_from_world"],
        )
        .register_documented(
            "from_clip_from_world_custom_far",
            |
                clip_from_world: Ref<::bevy_math::Mat4>,
                view_translation: Ref<::bevy_math::Vec3>,
                view_backward: Ref<::bevy_math::Vec3>,
                far: f32|
            {
                let output: Val<::bevy_render::primitives::Frustum> = {
                    {
                        let output: Val<::bevy_render::primitives::Frustum> = ::bevy_render::primitives::Frustum::from_clip_from_world_custom_far(
                                &clip_from_world,
                                &view_translation,
                                &view_backward,
                                far,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a frustum derived from `clip_from_world`,\n but with a custom far plane.",
            &["clip_from_world", "view_translation", "view_backward", "far"],
        )
        .register_documented(
            "intersects_obb",
            |
                _self: Ref<::bevy_render::primitives::Frustum>,
                aabb: Ref<::bevy_render::primitives::Aabb>,
                world_from_local: Ref<::bevy_math::Affine3A>,
                intersect_near: bool,
                intersect_far: bool|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_render::primitives::Frustum::intersects_obb(
                                &_self,
                                &aabb,
                                &world_from_local,
                                intersect_near,
                                intersect_far,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Checks if an Oriented Bounding Box (obb) intersects the frustum.",
            &["_self", "aabb", "world_from_local", "intersect_near", "intersect_far"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::primitives::Frustum,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_occlusion_culling_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::experimental::occlusion_culling::OcclusionCulling,
    >::new(world)
        .register_documented(
            "clone",
            |
                _self: Ref<
                    ::bevy_render::experimental::occlusion_culling::OcclusionCulling,
                >|
            {
                let output: Val<
                    ::bevy_render::experimental::occlusion_culling::OcclusionCulling,
                > = {
                    {
                        let output: Val<
                            ::bevy_render::experimental::occlusion_culling::OcclusionCulling,
                        > = <::bevy_render::experimental::occlusion_culling::OcclusionCulling as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_render::experimental::occlusion_culling::OcclusionCulling,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_render_graph_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::CameraRenderGraph,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::camera::CameraRenderGraph>| {
            let output: Val<::bevy_render::camera::CameraRenderGraph> = {
                {
                    let output: Val<::bevy_render::camera::CameraRenderGraph> =
                        <::bevy_render::camera::CameraRenderGraph as ::std::clone::Clone>::clone(
                            &_self,
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
            ::bevy_render::camera::CameraRenderGraph,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_main_texture_usages_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::CameraMainTextureUsages,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::camera::CameraMainTextureUsages>| {
                let output: Val<::bevy_render::camera::CameraMainTextureUsages> = {
                    {
                        let output: Val<
                            ::bevy_render::camera::CameraMainTextureUsages,
                        > = <::bevy_render::camera::CameraMainTextureUsages as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_render::camera::CameraMainTextureUsages,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_exposure_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::Exposure,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::camera::Exposure>| {
                let output: Val<::bevy_render::camera::Exposure> = {
                    {
                        let output: Val<::bevy_render::camera::Exposure> = <::bevy_render::camera::Exposure as ::std::clone::Clone>::clone(
                                &_self,
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
            "exposure",
            |_self: Ref<::bevy_render::camera::Exposure>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_render::camera::Exposure::exposure(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Converts EV100 values to exposure values.\n <https://google.github.io/filament/Filament.md.html#imagingpipeline/physicallybasedcamera/exposure>",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::camera::Exposure,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_temporal_jitter_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::TemporalJitter,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::camera::TemporalJitter>| {
            let output: Val<::bevy_render::camera::TemporalJitter> = {
                {
                    let output: Val<::bevy_render::camera::TemporalJitter> =
                        <::bevy_render::camera::TemporalJitter as ::std::clone::Clone>::clone(
                            &_self,
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
        "jitter_projection",
        |_self: Ref<::bevy_render::camera::TemporalJitter>,
         mut clip_from_view: Mut<::bevy_math::Mat4>,
         view_size: Val<::bevy_math::Vec2>| {
            let output: () = {
                {
                    let output: () = ::bevy_render::camera::TemporalJitter::jitter_projection(
                        &_self,
                        &mut clip_from_view,
                        view_size.into_inner(),
                    )
                    .into();
                    output
                }
            };
            output
        },
        "",
        &["_self", "clip_from_view", "view_size"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::camera::TemporalJitter,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_mip_bias_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::MipBias,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::camera::MipBias,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_manual_texture_view_handle_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::ManualTextureViewHandle,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::camera::ManualTextureViewHandle>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::camera::ManualTextureViewHandle as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: Ref<::bevy_render::camera::ManualTextureViewHandle>| {
                let output: Val<::bevy_render::camera::ManualTextureViewHandle> = {
                    {
                        let output: Val<
                            ::bevy_render::camera::ManualTextureViewHandle,
                        > = <::bevy_render::camera::ManualTextureViewHandle as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: Ref<::bevy_render::camera::ManualTextureViewHandle>,
                other: Ref<::bevy_render::camera::ManualTextureViewHandle>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::camera::ManualTextureViewHandle as ::std::cmp::PartialEq<
                            ::bevy_render::camera::ManualTextureViewHandle,
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
            ::bevy_render::camera::ManualTextureViewHandle,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_color_grading_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::ColorGrading,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::view::ColorGrading>| {
                let output: Val<::bevy_render::view::ColorGrading> = {
                    {
                        let output: Val<::bevy_render::view::ColorGrading> = <::bevy_render::view::ColorGrading as ::std::clone::Clone>::clone(
                                &_self,
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
            "with_identical_sections",
            |
                global: Val<::bevy_render::view::ColorGradingGlobal>,
                section: Val<::bevy_render::view::ColorGradingSection>|
            {
                let output: Val<::bevy_render::view::ColorGrading> = {
                    {
                        let output: Val<::bevy_render::view::ColorGrading> = ::bevy_render::view::ColorGrading::with_identical_sections(
                                global.into_inner(),
                                section.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`ColorGrading`] instance in which shadows, midtones, and\n highlights all have the same set of color grading values.",
            &["global", "section"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::ColorGrading,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_layers_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::visibility::RenderLayers,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::view::visibility::RenderLayers>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::view::visibility::RenderLayers as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: Ref<::bevy_render::view::visibility::RenderLayers>| {
                let output: Val<::bevy_render::view::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_render::view::visibility::RenderLayers> = <::bevy_render::view::visibility::RenderLayers as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: Ref<::bevy_render::view::visibility::RenderLayers>,
                other: Ref<::bevy_render::view::visibility::RenderLayers>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::view::visibility::RenderLayers as ::std::cmp::PartialEq<
                            ::bevy_render::view::visibility::RenderLayers,
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
            "intersection",
            |
                _self: Ref<::bevy_render::view::visibility::RenderLayers>,
                other: Ref<::bevy_render::view::visibility::RenderLayers>|
            {
                let output: Val<::bevy_render::view::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_render::view::visibility::RenderLayers> = ::bevy_render::view::visibility::RenderLayers::intersection(
                                &_self,
                                &other,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the set of [layers](Layer) shared by two instances of [`RenderLayers`].\n This corresponds to the `self & other` operation.",
            &["_self", "other"],
        )
        .register_documented(
            "intersects",
            |
                _self: Ref<::bevy_render::view::visibility::RenderLayers>,
                other: Ref<::bevy_render::view::visibility::RenderLayers>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_render::view::visibility::RenderLayers::intersects(
                                &_self,
                                &other,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Determine if a `RenderLayers` intersects another.\n `RenderLayers`s intersect if they share any common layers.\n A `RenderLayers` with no layers will not match any other\n `RenderLayers`, even another with no layers.",
            &["_self", "other"],
        )
        .register_documented(
            "layer",
            |n: usize| {
                let output: Val<::bevy_render::view::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_render::view::visibility::RenderLayers> = ::bevy_render::view::visibility::RenderLayers::layer(
                                n,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `RenderLayers` belonging to the given layer.\n This `const` constructor is limited to `size_of::<usize>()` layers.\n If you need to support an arbitrary number of layers, use [`with`](RenderLayers::with)\n or [`from_layers`](RenderLayers::from_layers).",
            &["n"],
        )
        .register_documented(
            "none",
            || {
                let output: Val<::bevy_render::view::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_render::view::visibility::RenderLayers> = ::bevy_render::view::visibility::RenderLayers::none()
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `RenderLayers` that belongs to no layers.\n This is distinct from [`RenderLayers::default`], which belongs to the first layer.",
            &[],
        )
        .register_documented(
            "symmetric_difference",
            |
                _self: Ref<::bevy_render::view::visibility::RenderLayers>,
                other: Ref<::bevy_render::view::visibility::RenderLayers>|
            {
                let output: Val<::bevy_render::view::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_render::view::visibility::RenderLayers> = ::bevy_render::view::visibility::RenderLayers::symmetric_difference(
                                &_self,
                                &other,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns all [layers](Layer) included in exactly one of the instances of [`RenderLayers`].\n This corresponds to the \"exclusive or\" (XOR) operation: `self ^ other`.",
            &["_self", "other"],
        )
        .register_documented(
            "union",
            |
                _self: Ref<::bevy_render::view::visibility::RenderLayers>,
                other: Ref<::bevy_render::view::visibility::RenderLayers>|
            {
                let output: Val<::bevy_render::view::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_render::view::visibility::RenderLayers> = ::bevy_render::view::visibility::RenderLayers::union(
                                &_self,
                                &other,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns all [layers](Layer) included in either instance of [`RenderLayers`].\n This corresponds to the `self | other` operation.",
            &["_self", "other"],
        )
        .register_documented(
            "with",
            |_self: Val<::bevy_render::view::visibility::RenderLayers>, layer: usize| {
                let output: Val<::bevy_render::view::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_render::view::visibility::RenderLayers> = ::bevy_render::view::visibility::RenderLayers::with(
                                _self.into_inner(),
                                layer,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Add the given layer.\n This may be called multiple times to allow an entity to belong\n to multiple rendering layers.",
            &["_self", "layer"],
        )
        .register_documented(
            "without",
            |_self: Val<::bevy_render::view::visibility::RenderLayers>, layer: usize| {
                let output: Val<::bevy_render::view::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_render::view::visibility::RenderLayers> = ::bevy_render::view::visibility::RenderLayers::without(
                                _self.into_inner(),
                                layer,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Removes the given rendering layer.",
            &["_self", "layer"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::visibility::RenderLayers,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_visible_entities_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::visibility::RenderVisibleEntities,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::view::visibility::RenderVisibleEntities>| {
                let output: Val<
                    ::bevy_render::view::visibility::RenderVisibleEntities,
                > = {
                    {
                        let output: Val<
                            ::bevy_render::view::visibility::RenderVisibleEntities,
                        > = <::bevy_render::view::visibility::RenderVisibleEntities as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_render::view::visibility::RenderVisibleEntities,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_visible_entities_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::visibility::VisibleEntities,
    >::new(world)
        .register_documented(
            "clear",
            |
                mut _self: Mut<::bevy_render::view::visibility::VisibleEntities>,
                type_id: Val<::std::any::TypeId>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_render::view::visibility::VisibleEntities::clear(
                                &mut _self,
                                type_id.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "type_id"],
        )
        .register_documented(
            "clear_all",
            |mut _self: Mut<::bevy_render::view::visibility::VisibleEntities>| {
                let output: () = {
                    {
                        let output: () = ::bevy_render::view::visibility::VisibleEntities::clear_all(
                                &mut _self,
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
            |_self: Ref<::bevy_render::view::visibility::VisibleEntities>| {
                let output: Val<::bevy_render::view::visibility::VisibleEntities> = {
                    {
                        let output: Val<
                            ::bevy_render::view::visibility::VisibleEntities,
                        > = <::bevy_render::view::visibility::VisibleEntities as ::std::clone::Clone>::clone(
                                &_self,
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
            "is_empty",
            |
                _self: Ref<::bevy_render::view::visibility::VisibleEntities>,
                type_id: Val<::std::any::TypeId>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_render::view::visibility::VisibleEntities::is_empty(
                                &_self,
                                type_id.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "type_id"],
        )
        .register_documented(
            "len",
            |
                _self: Ref<::bevy_render::view::visibility::VisibleEntities>,
                type_id: Val<::std::any::TypeId>|
            {
                let output: usize = {
                    {
                        let output: usize = ::bevy_render::view::visibility::VisibleEntities::len(
                                &_self,
                                type_id.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "type_id"],
        )
        .register_documented(
            "push",
            |
                mut _self: Mut<::bevy_render::view::visibility::VisibleEntities>,
                entity: Val<::bevy_ecs::entity::Entity>,
                type_id: Val<::std::any::TypeId>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_render::view::visibility::VisibleEntities::push(
                                &mut _self,
                                entity.into_inner(),
                                type_id.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "entity", "type_id"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::visibility::VisibleEntities,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_viewport_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::Viewport,
    >::new(world)
        .register_documented(
            "clamp_to_size",
            |
                mut _self: Mut<::bevy_render::camera::Viewport>,
                size: Val<::bevy_math::UVec2>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_render::camera::Viewport::clamp_to_size(
                                &mut _self,
                                size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Cut the viewport rectangle so that it lies inside a rectangle of the\n given size.\n If either of the viewport's position coordinates lies outside the given\n dimensions, it will be moved just inside first. If either of the given\n dimensions is zero, the position and size of the viewport rectangle will\n both be set to zero in that dimension.",
            &["_self", "size"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::camera::Viewport>| {
                let output: Val<::bevy_render::camera::Viewport> = {
                    {
                        let output: Val<::bevy_render::camera::Viewport> = <::bevy_render::camera::Viewport as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_render::camera::Viewport,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sub_camera_view_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::SubCameraView,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::camera::SubCameraView>| {
            let output: Val<::bevy_render::camera::SubCameraView> = {
                {
                    let output: Val<::bevy_render::camera::SubCameraView> =
                        <::bevy_render::camera::SubCameraView as ::std::clone::Clone>::clone(
                            &_self,
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
        |_self: Ref<::bevy_render::camera::SubCameraView>,
         other: Ref<::bevy_render::camera::SubCameraView>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_render::camera::SubCameraView as ::std::cmp::PartialEq<
                            ::bevy_render::camera::SubCameraView,
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
            ::bevy_render::camera::SubCameraView,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_target_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::RenderTarget,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::camera::RenderTarget>| {
            let output: Val<::bevy_render::camera::RenderTarget> = {
                {
                    let output: Val<::bevy_render::camera::RenderTarget> =
                        <::bevy_render::camera::RenderTarget as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_render::camera::RenderTarget,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_image_render_target_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::ImageRenderTarget,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::camera::ImageRenderTarget>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::camera::ImageRenderTarget as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: Ref<::bevy_render::camera::ImageRenderTarget>| {
                let output: Val<::bevy_render::camera::ImageRenderTarget> = {
                    {
                        let output: Val<::bevy_render::camera::ImageRenderTarget> = <::bevy_render::camera::ImageRenderTarget as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: Ref<::bevy_render::camera::ImageRenderTarget>,
                other: Ref<::bevy_render::camera::ImageRenderTarget>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::camera::ImageRenderTarget as ::std::cmp::PartialEq<
                            ::bevy_render::camera::ImageRenderTarget,
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
            ::bevy_render::camera::ImageRenderTarget,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_normalized_render_target_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::NormalizedRenderTarget,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::camera::NormalizedRenderTarget>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::camera::NormalizedRenderTarget as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: Ref<::bevy_render::camera::NormalizedRenderTarget>| {
                let output: Val<::bevy_render::camera::NormalizedRenderTarget> = {
                    {
                        let output: Val<::bevy_render::camera::NormalizedRenderTarget> = <::bevy_render::camera::NormalizedRenderTarget as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: Ref<::bevy_render::camera::NormalizedRenderTarget>,
                other: Ref<::bevy_render::camera::NormalizedRenderTarget>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::camera::NormalizedRenderTarget as ::std::cmp::PartialEq<
                            ::bevy_render::camera::NormalizedRenderTarget,
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
            ::bevy_render::camera::NormalizedRenderTarget,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_custom_projection_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::CustomProjection,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::camera::CustomProjection>| {
            let output: Val<::bevy_render::camera::CustomProjection> = {
                {
                    let output: Val<::bevy_render::camera::CustomProjection> =
                        <::bevy_render::camera::CustomProjection as ::std::clone::Clone>::clone(
                            &_self,
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
            ::bevy_render::camera::CustomProjection,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scaling_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::ScalingMode,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::camera::ScalingMode>| {
            let output: Val<::bevy_render::camera::ScalingMode> = {
                {
                    let output: Val<::bevy_render::camera::ScalingMode> =
                        <::bevy_render::camera::ScalingMode as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_render::camera::ScalingMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_globals_uniform_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::globals::GlobalsUniform,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::globals::GlobalsUniform>| {
            let output: Val<::bevy_render::globals::GlobalsUniform> = {
                {
                    let output: Val<::bevy_render::globals::GlobalsUniform> =
                        <::bevy_render::globals::GlobalsUniform as ::std::clone::Clone>::clone(
                            &_self,
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
            ::bevy_render::globals::GlobalsUniform,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_shader_storage_buffer_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::storage::ShaderStorageBuffer,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::storage::ShaderStorageBuffer>| {
                let output: Val<::bevy_render::storage::ShaderStorageBuffer> = {
                    {
                        let output: Val<::bevy_render::storage::ShaderStorageBuffer> = <::bevy_render::storage::ShaderStorageBuffer as ::std::clone::Clone>::clone(
                                &_self,
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
            "with_size",
            |
                size: usize,
                asset_usage: Val<::bevy_render::render_asset::RenderAssetUsages>|
            {
                let output: Val<::bevy_render::storage::ShaderStorageBuffer> = {
                    {
                        let output: Val<::bevy_render::storage::ShaderStorageBuffer> = ::bevy_render::storage::ShaderStorageBuffer::with_size(
                                size,
                                asset_usage.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new storage buffer with the given size and asset usage.",
            &["size", "asset_usage"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::storage::ShaderStorageBuffer,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_readback_complete_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::gpu_readback::ReadbackComplete,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::gpu_readback::ReadbackComplete,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_mesh_tag_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::mesh::MeshTag,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::mesh::MeshTag>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::mesh::MeshTag as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: Ref<::bevy_render::mesh::MeshTag>| {
                let output: Val<::bevy_render::mesh::MeshTag> = {
                    {
                        let output: Val<::bevy_render::mesh::MeshTag> = <::bevy_render::mesh::MeshTag as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: Ref<::bevy_render::mesh::MeshTag>,
                other: Ref<::bevy_render::mesh::MeshTag>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::mesh::MeshTag as ::std::cmp::PartialEq<
                            ::bevy_render::mesh::MeshTag,
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
            ::bevy_render::mesh::MeshTag,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_visibility_class_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::visibility::VisibilityClass,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::view::visibility::VisibilityClass>| {
                let output: Val<::bevy_render::view::visibility::VisibilityClass> = {
                    {
                        let output: Val<
                            ::bevy_render::view::visibility::VisibilityClass,
                        > = <::bevy_render::view::visibility::VisibilityClass as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_render::view::visibility::VisibilityClass,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_temporary_render_entity_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::sync_world::TemporaryRenderEntity,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::sync_world::TemporaryRenderEntity>| {
                let output: Val<::bevy_render::sync_world::TemporaryRenderEntity> = {
                    {
                        let output: Val<
                            ::bevy_render::sync_world::TemporaryRenderEntity,
                        > = <::bevy_render::sync_world::TemporaryRenderEntity as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_render::sync_world::TemporaryRenderEntity,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_no_frustum_culling_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::visibility::NoFrustumCulling,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::visibility::NoFrustumCulling,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_color_grading_global_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::ColorGradingGlobal,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::view::ColorGradingGlobal>| {
            let output: Val<::bevy_render::view::ColorGradingGlobal> = {
                {
                    let output: Val<::bevy_render::view::ColorGradingGlobal> =
                        <::bevy_render::view::ColorGradingGlobal as ::std::clone::Clone>::clone(
                            &_self,
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
            ::bevy_render::view::ColorGradingGlobal,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_color_grading_section_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::ColorGradingSection,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_render::view::ColorGradingSection>| {
            let output: Val<::bevy_render::view::ColorGradingSection> = {
                {
                    let output: Val<::bevy_render::view::ColorGradingSection> =
                        <::bevy_render::view::ColorGradingSection as ::std::clone::Clone>::clone(
                            &_self,
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
        |_self: Ref<::bevy_render::view::ColorGradingSection>,
         other: Ref<::bevy_render::view::ColorGradingSection>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_render::view::ColorGradingSection as ::std::cmp::PartialEq<
                            ::bevy_render::view::ColorGradingSection,
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
            ::bevy_render::view::ColorGradingSection,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_visibility_range_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::visibility::VisibilityRange,
    >::new(world)
        .register_documented(
            "abrupt",
            |start: f32, end: f32| {
                let output: Val<::bevy_render::view::visibility::VisibilityRange> = {
                    {
                        let output: Val<
                            ::bevy_render::view::visibility::VisibilityRange,
                        > = ::bevy_render::view::visibility::VisibilityRange::abrupt(
                                start,
                                end,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new *abrupt* visibility range, with no crossfade.\n There will be no crossfade; the object will immediately vanish if the\n camera is closer than `start` units or farther than `end` units from the\n model.\n The `start` value must be less than or equal to the `end` value.",
            &["start", "end"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::view::visibility::VisibilityRange>| {
                let output: Val<::bevy_render::view::visibility::VisibilityRange> = {
                    {
                        let output: Val<
                            ::bevy_render::view::visibility::VisibilityRange,
                        > = <::bevy_render::view::visibility::VisibilityRange as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: Ref<::bevy_render::view::visibility::VisibilityRange>,
                other: Ref<::bevy_render::view::visibility::VisibilityRange>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::view::visibility::VisibilityRange as ::std::cmp::PartialEq<
                            ::bevy_render::view::visibility::VisibilityRange,
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
            "is_abrupt",
            |_self: Ref<::bevy_render::view::visibility::VisibilityRange>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_render::view::visibility::VisibilityRange::is_abrupt(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if both the start and end transitions for this range are\n abrupt: that is, there is no crossfading.",
            &["_self"],
        )
        .register_documented(
            "is_culled",
            |
                _self: Ref<::bevy_render::view::visibility::VisibilityRange>,
                camera_distance: f32|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_render::view::visibility::VisibilityRange::is_culled(
                                &_self,
                                camera_distance,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if the object is completely invisible, given a camera\n `camera_distance` units away.\n This is equivalent to `!VisibilityRange::is_visible_at_all()`.",
            &["_self", "camera_distance"],
        )
        .register_documented(
            "is_visible_at_all",
            |
                _self: Ref<::bevy_render::view::visibility::VisibilityRange>,
                camera_distance: f32|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_render::view::visibility::VisibilityRange::is_visible_at_all(
                                &_self,
                                camera_distance,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if the object will be visible at all, given a camera\n `camera_distance` units away.\n Any amount of visibility, even with the heaviest dithering applied, is\n considered visible according to this check.",
            &["_self", "camera_distance"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::visibility::VisibilityRange,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_screenshot_captured_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::window::screenshot::ScreenshotCaptured,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::window::screenshot::ScreenshotCaptured,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_screenshot_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::window::screenshot::Screenshot,
    >::new(world)
    .register_documented(
        "primary_window",
        || {
            let output: Val<::bevy_render::view::window::screenshot::Screenshot> = {
                {
                    let output: Val<::bevy_render::view::window::screenshot::Screenshot> =
                        ::bevy_render::view::window::screenshot::Screenshot::primary_window()
                            .into();
                    output
                }
            };
            output
        },
        " Capture a screenshot of the primary window, if one exists.",
        &[],
    )
    .register_documented(
        "texture_view",
        |texture_view: Val<::bevy_render::camera::ManualTextureViewHandle>| {
            let output: Val<::bevy_render::view::window::screenshot::Screenshot> = {
                {
                    let output: Val<::bevy_render::view::window::screenshot::Screenshot> =
                        ::bevy_render::view::window::screenshot::Screenshot::texture_view(
                            texture_view.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Capture a screenshot of the provided manual texture view.",
        &["texture_view"],
    )
    .register_documented(
        "window",
        |window: Val<::bevy_ecs::entity::Entity>| {
            let output: Val<::bevy_render::view::window::screenshot::Screenshot> = {
                {
                    let output: Val<::bevy_render::view::window::screenshot::Screenshot> =
                        ::bevy_render::view::window::screenshot::Screenshot::window(
                            window.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Capture a screenshot of the provided window entity.",
        &["window"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::window::screenshot::Screenshot,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyRenderScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_alpha_mode_functions(&mut world);
        register_camera_functions(&mut world);
        register_clear_color_functions(&mut world);
        register_clear_color_config_functions(&mut world);
        register_orthographic_projection_functions(&mut world);
        register_perspective_projection_functions(&mut world);
        register_projection_functions(&mut world);
        register_mesh_2_d_functions(&mut world);
        register_mesh_3_d_functions(&mut world);
        register_inherited_visibility_functions(&mut world);
        register_msaa_functions(&mut world);
        register_view_visibility_functions(&mut world);
        register_visibility_functions(&mut world);
        register_sync_to_render_world_functions(&mut world);
        register_aabb_functions(&mut world);
        register_cascades_frusta_functions(&mut world);
        register_cubemap_frusta_functions(&mut world);
        register_frustum_functions(&mut world);
        register_occlusion_culling_functions(&mut world);
        register_camera_render_graph_functions(&mut world);
        register_camera_main_texture_usages_functions(&mut world);
        register_exposure_functions(&mut world);
        register_temporal_jitter_functions(&mut world);
        register_mip_bias_functions(&mut world);
        register_manual_texture_view_handle_functions(&mut world);
        register_color_grading_functions(&mut world);
        register_render_layers_functions(&mut world);
        register_render_visible_entities_functions(&mut world);
        register_visible_entities_functions(&mut world);
        register_viewport_functions(&mut world);
        register_sub_camera_view_functions(&mut world);
        register_render_target_functions(&mut world);
        register_image_render_target_functions(&mut world);
        register_normalized_render_target_functions(&mut world);
        register_custom_projection_functions(&mut world);
        register_scaling_mode_functions(&mut world);
        register_globals_uniform_functions(&mut world);
        register_shader_storage_buffer_functions(&mut world);
        register_readback_complete_functions(&mut world);
        register_mesh_tag_functions(&mut world);
        register_visibility_class_functions(&mut world);
        register_temporary_render_entity_functions(&mut world);
        register_no_frustum_culling_functions(&mut world);
        register_color_grading_global_functions(&mut world);
        register_color_grading_section_functions(&mut world);
        register_visibility_range_functions(&mut world);
        register_screenshot_captured_functions(&mut world);
        register_screenshot_functions(&mut world);
    }
}
