
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
pub struct BevyCameraScriptingPlugin;
pub(crate) fn register_clear_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::ClearColor,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::ClearColor>| {
                let output: Val<::bevy_camera::ClearColor> = {
                    {
                        let output: Val<::bevy_camera::ClearColor> = <::bevy_camera::ClearColor as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::ClearColor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_inherited_visibility_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::visibility::InheritedVisibility,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_camera::visibility::InheritedVisibility>| {
                let output: () = {
                    {
                        let output: () = <::bevy_camera::visibility::InheritedVisibility as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
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
            |_self: Ref<::bevy_camera::visibility::InheritedVisibility>| {
                let output: Val<::bevy_camera::visibility::InheritedVisibility> = {
                    {
                        let output: Val<
                            ::bevy_camera::visibility::InheritedVisibility,
                        > = <::bevy_camera::visibility::InheritedVisibility as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::visibility::InheritedVisibility>,
                other: Ref<::bevy_camera::visibility::InheritedVisibility>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_camera::visibility::InheritedVisibility as ::std::cmp::PartialEq<
                            ::bevy_camera::visibility::InheritedVisibility,
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
            |_self: Val<::bevy_camera::visibility::InheritedVisibility>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_camera::visibility::InheritedVisibility::get(
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
            ::bevy_camera::visibility::InheritedVisibility,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_view_visibility_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::visibility::ViewVisibility,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_camera::visibility::ViewVisibility>| {
                let output: () = {
                    {
                        let output: () = <::bevy_camera::visibility::ViewVisibility as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
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
            |_self: Ref<::bevy_camera::visibility::ViewVisibility>| {
                let output: Val<::bevy_camera::visibility::ViewVisibility> = {
                    {
                        let output: Val<::bevy_camera::visibility::ViewVisibility> = <::bevy_camera::visibility::ViewVisibility as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::visibility::ViewVisibility>,
                other: Ref<::bevy_camera::visibility::ViewVisibility>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_camera::visibility::ViewVisibility as ::std::cmp::PartialEq<
                            ::bevy_camera::visibility::ViewVisibility,
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
            |_self: Val<::bevy_camera::visibility::ViewVisibility>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_camera::visibility::ViewVisibility::get(
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
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_camera::visibility::ViewVisibility,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_visibility_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::visibility::Visibility,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_camera::visibility::Visibility>| {
                let output: () = {
                    {
                        let output: () = <::bevy_camera::visibility::Visibility as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
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
            |_self: Ref<::bevy_camera::visibility::Visibility>| {
                let output: Val<::bevy_camera::visibility::Visibility> = {
                    {
                        let output: Val<::bevy_camera::visibility::Visibility> = <::bevy_camera::visibility::Visibility as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::visibility::Visibility>,
                other: Ref<::bevy_camera::visibility::Visibility>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_camera::visibility::Visibility as ::std::cmp::PartialEq<
                            ::bevy_camera::visibility::Visibility,
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
            |mut _self: Mut<::bevy_camera::visibility::Visibility>| {
                let output: () = {
                    {
                        let output: () = ::bevy_camera::visibility::Visibility::toggle_inherited_hidden(
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
            |mut _self: Mut<::bevy_camera::visibility::Visibility>| {
                let output: () = {
                    {
                        let output: () = ::bevy_camera::visibility::Visibility::toggle_inherited_visible(
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
            |mut _self: Mut<::bevy_camera::visibility::Visibility>| {
                let output: () = {
                    {
                        let output: () = ::bevy_camera::visibility::Visibility::toggle_visible_hidden(
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
            ::bevy_camera::visibility::Visibility,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::Camera,
    >::new(world)
        .register_documented(
            "clip_from_view",
            |_self: Ref<::bevy_camera::Camera>| {
                let output: Val<::bevy_math::Mat4> = {
                    {
                        let output: Val<::bevy_math::Mat4> = ::bevy_camera::Camera::clip_from_view(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The projection matrix computed using this camera's [`Projection`](super::projection::Projection).",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::Camera>| {
                let output: Val<::bevy_camera::Camera> = {
                    {
                        let output: Val<::bevy_camera::Camera> = <::bevy_camera::Camera as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
            |_self: Ref<::bevy_camera::Camera>, ndc_depth: f32| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_camera::Camera::depth_ndc_to_view_z(
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
            |_self: Ref<::bevy_camera::Camera>, ndc_depth: f32| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_camera::Camera::depth_ndc_to_view_z_2d(
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
            |_self: Ref<::bevy_camera::Camera>| {
                let output: ::std::option::Option<f32> = {
                    {
                        let output: ::std::option::Option<f32> = ::bevy_camera::Camera::target_scaling_factor(
                                &_self,
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
            ::bevy_camera::Camera,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::Camera2d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::Camera2d>| {
                let output: Val<::bevy_camera::Camera2d> = {
                    {
                        let output: Val<::bevy_camera::Camera2d> = <::bevy_camera::Camera2d as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::Camera2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::Camera3d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::Camera3d>| {
                let output: Val<::bevy_camera::Camera3d> = {
                    {
                        let output: Val<::bevy_camera::Camera3d> = <::bevy_camera::Camera3d as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::Camera3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_clear_color_config_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::ClearColorConfig,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::ClearColorConfig>| {
                let output: Val<::bevy_camera::ClearColorConfig> = {
                    {
                        let output: Val<::bevy_camera::ClearColorConfig> = <::bevy_camera::ClearColorConfig as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::ClearColorConfig,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_msaa_writeback_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::MsaaWriteback,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_camera::MsaaWriteback>| {
                let output: () = {
                    {
                        let output: () = <::bevy_camera::MsaaWriteback as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
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
            |_self: Ref<::bevy_camera::MsaaWriteback>| {
                let output: Val<::bevy_camera::MsaaWriteback> = {
                    {
                        let output: Val<::bevy_camera::MsaaWriteback> = <::bevy_camera::MsaaWriteback as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::MsaaWriteback>,
                other: Ref<::bevy_camera::MsaaWriteback>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_camera::MsaaWriteback as ::std::cmp::PartialEq<
                            ::bevy_camera::MsaaWriteback,
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
            ::bevy_camera::MsaaWriteback,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_orthographic_projection_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::OrthographicProjection,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::OrthographicProjection>| {
                let output: Val<::bevy_camera::OrthographicProjection> = {
                    {
                        let output: Val<::bevy_camera::OrthographicProjection> = <::bevy_camera::OrthographicProjection as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                let output: Val<::bevy_camera::OrthographicProjection> = {
                    {
                        let output: Val<::bevy_camera::OrthographicProjection> = ::bevy_camera::OrthographicProjection::default_2d()
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
                let output: Val<::bevy_camera::OrthographicProjection> = {
                    {
                        let output: Val<::bevy_camera::OrthographicProjection> = ::bevy_camera::OrthographicProjection::default_3d()
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
            ::bevy_camera::OrthographicProjection,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_perspective_projection_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::PerspectiveProjection,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::PerspectiveProjection>| {
                let output: Val<::bevy_camera::PerspectiveProjection> = {
                    {
                        let output: Val<::bevy_camera::PerspectiveProjection> = <::bevy_camera::PerspectiveProjection as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::PerspectiveProjection,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_projection_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::Projection,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::Projection>| {
                let output: Val<::bevy_camera::Projection> = {
                    {
                        let output: Val<::bevy_camera::Projection> = <::bevy_camera::Projection as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "is_perspective",
            |_self: Ref<::bevy_camera::Projection>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_camera::Projection::is_perspective(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Check if the projection is perspective.\n For [`CustomProjection`], this checks if the projection matrix's w-axis's w is 0.0.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_camera::Projection,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_frustum_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::primitives::Frustum,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::primitives::Frustum>| {
                let output: Val<::bevy_camera::primitives::Frustum> = {
                    {
                        let output: Val<::bevy_camera::primitives::Frustum> = <::bevy_camera::primitives::Frustum as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::primitives::Frustum>,
                aabb: Ref<::bevy_camera::primitives::Aabb>,
                world_from_local: Ref<::bevy_math::Affine3A>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_camera::primitives::Frustum::contains_aabb(
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
            " Check if the frustum contains the entire Axis-Aligned Bounding Box (AABB).\n Referenced from: [Frustum Culling](https://learnopengl.com/Guest-Articles/2021/Scene/Frustum-Culling)",
            &["_self", "aabb", "world_from_local"],
        )
        .register_documented(
            "contains_aabb_identity",
            |
                _self: Ref<::bevy_camera::primitives::Frustum>,
                aabb: Ref<::bevy_camera::primitives::Aabb>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_camera::primitives::Frustum::contains_aabb_identity(
                                &_self,
                                &aabb,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Optimized version of [`Self::contains_aabb`] when the AABB is already in world space.\n Use this when `world_from_local` would be [`Affine3A::IDENTITY`].",
            &["_self", "aabb"],
        )
        .register_documented(
            "from_clip_from_world",
            |clip_from_world: Ref<::bevy_math::Mat4>| {
                let output: Val<::bevy_camera::primitives::Frustum> = {
                    {
                        let output: Val<::bevy_camera::primitives::Frustum> = ::bevy_camera::primitives::Frustum::from_clip_from_world(
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
                let output: Val<::bevy_camera::primitives::Frustum> = {
                    {
                        let output: Val<::bevy_camera::primitives::Frustum> = ::bevy_camera::primitives::Frustum::from_clip_from_world_custom_far(
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
                _self: Ref<::bevy_camera::primitives::Frustum>,
                aabb: Ref<::bevy_camera::primitives::Aabb>,
                world_from_local: Ref<::bevy_math::Affine3A>,
                intersect_near: bool,
                intersect_far: bool|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_camera::primitives::Frustum::intersects_obb(
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
        )
        .register_documented(
            "intersects_obb_identity",
            |
                _self: Ref<::bevy_camera::primitives::Frustum>,
                aabb: Ref<::bevy_camera::primitives::Aabb>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_camera::primitives::Frustum::intersects_obb_identity(
                                &_self,
                                &aabb,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Optimized version of [`Frustum::intersects_obb`]\n where the transform is [`Affine3A::IDENTITY`] and both `intersect_near` and `intersect_far` are `true`.",
            &["_self", "aabb"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_camera::primitives::Frustum,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_visible_entities_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::visibility::VisibleEntities,
    >::new(world)
        .register_documented(
            "clear",
            |
                mut _self: Mut<::bevy_camera::visibility::VisibleEntities>,
                type_id: Val<::std::any::TypeId>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_camera::visibility::VisibleEntities::clear(
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
            |mut _self: Mut<::bevy_camera::visibility::VisibleEntities>| {
                let output: () = {
                    {
                        let output: () = ::bevy_camera::visibility::VisibleEntities::clear_all(
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
            |_self: Ref<::bevy_camera::visibility::VisibleEntities>| {
                let output: Val<::bevy_camera::visibility::VisibleEntities> = {
                    {
                        let output: Val<::bevy_camera::visibility::VisibleEntities> = <::bevy_camera::visibility::VisibleEntities as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::visibility::VisibleEntities>,
                type_id: Val<::std::any::TypeId>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_camera::visibility::VisibleEntities::is_empty(
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
                _self: Ref<::bevy_camera::visibility::VisibleEntities>,
                type_id: Val<::std::any::TypeId>|
            {
                let output: usize = {
                    {
                        let output: usize = ::bevy_camera::visibility::VisibleEntities::len(
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
                mut _self: Mut<::bevy_camera::visibility::VisibleEntities>,
                entity: Val<::bevy_ecs::entity::Entity>,
                type_id: Val<::std::any::TypeId>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_camera::visibility::VisibleEntities::push(
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
            ::bevy_camera::visibility::VisibleEntities,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_viewport_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::Viewport,
    >::new(world)
        .register_documented(
            "clamp_to_size",
            |mut _self: Mut<::bevy_camera::Viewport>, size: Val<::bevy_math::UVec2>| {
                let output: () = {
                    {
                        let output: () = ::bevy_camera::Viewport::clamp_to_size(
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
            |_self: Ref<::bevy_camera::Viewport>| {
                let output: Val<::bevy_camera::Viewport> = {
                    {
                        let output: Val<::bevy_camera::Viewport> = <::bevy_camera::Viewport as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::Viewport,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_main_pass_resolution_override_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::MainPassResolutionOverride,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_camera::MainPassResolutionOverride,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sub_camera_view_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::SubCameraView,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::SubCameraView>| {
                let output: Val<::bevy_camera::SubCameraView> = {
                    {
                        let output: Val<::bevy_camera::SubCameraView> = <::bevy_camera::SubCameraView as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::SubCameraView>,
                other: Ref<::bevy_camera::SubCameraView>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_camera::SubCameraView as ::std::cmp::PartialEq<
                            ::bevy_camera::SubCameraView,
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
            ::bevy_camera::SubCameraView,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_exposure_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::Exposure,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::Exposure>| {
                let output: Val<::bevy_camera::Exposure> = {
                    {
                        let output: Val<::bevy_camera::Exposure> = <::bevy_camera::Exposure as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
            |_self: Ref<::bevy_camera::Exposure>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_camera::Exposure::exposure(&_self)
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
            ::bevy_camera::Exposure,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_main_texture_usages_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::CameraMainTextureUsages,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::CameraMainTextureUsages>| {
                let output: Val<::bevy_camera::CameraMainTextureUsages> = {
                    {
                        let output: Val<::bevy_camera::CameraMainTextureUsages> = <::bevy_camera::CameraMainTextureUsages as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::CameraMainTextureUsages,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_target_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::RenderTarget,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::RenderTarget>| {
                let output: Val<::bevy_camera::RenderTarget> = {
                    {
                        let output: Val<::bevy_camera::RenderTarget> = <::bevy_camera::RenderTarget as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::RenderTarget,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_output_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::CameraOutputMode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::CameraOutputMode>| {
                let output: Val<::bevy_camera::CameraOutputMode> = {
                    {
                        let output: Val<::bevy_camera::CameraOutputMode> = <::bevy_camera::CameraOutputMode as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::CameraOutputMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_image_render_target_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::ImageRenderTarget,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::ImageRenderTarget>| {
                let output: Val<::bevy_camera::ImageRenderTarget> = {
                    {
                        let output: Val<::bevy_camera::ImageRenderTarget> = <::bevy_camera::ImageRenderTarget as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::ImageRenderTarget>,
                other: Ref<::bevy_camera::ImageRenderTarget>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_camera::ImageRenderTarget as ::std::cmp::PartialEq<
                            ::bevy_camera::ImageRenderTarget,
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
            ::bevy_camera::ImageRenderTarget,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_manual_texture_view_handle_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::ManualTextureViewHandle,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_camera::ManualTextureViewHandle>| {
                let output: () = {
                    {
                        let output: () = <::bevy_camera::ManualTextureViewHandle as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
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
            |_self: Ref<::bevy_camera::ManualTextureViewHandle>| {
                let output: Val<::bevy_camera::ManualTextureViewHandle> = {
                    {
                        let output: Val<::bevy_camera::ManualTextureViewHandle> = <::bevy_camera::ManualTextureViewHandle as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::ManualTextureViewHandle>,
                other: Ref<::bevy_camera::ManualTextureViewHandle>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_camera::ManualTextureViewHandle as ::std::cmp::PartialEq<
                            ::bevy_camera::ManualTextureViewHandle,
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
            ::bevy_camera::ManualTextureViewHandle,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_normalized_render_target_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::NormalizedRenderTarget,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_camera::NormalizedRenderTarget>| {
                let output: () = {
                    {
                        let output: () = <::bevy_camera::NormalizedRenderTarget as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
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
            |_self: Ref<::bevy_camera::NormalizedRenderTarget>| {
                let output: Val<::bevy_camera::NormalizedRenderTarget> = {
                    {
                        let output: Val<::bevy_camera::NormalizedRenderTarget> = <::bevy_camera::NormalizedRenderTarget as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::NormalizedRenderTarget>,
                other: Ref<::bevy_camera::NormalizedRenderTarget>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_camera::NormalizedRenderTarget as ::std::cmp::PartialEq<
                            ::bevy_camera::NormalizedRenderTarget,
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
            ::bevy_camera::NormalizedRenderTarget,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_3_d_depth_load_op_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::Camera3dDepthLoadOp,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::Camera3dDepthLoadOp>| {
                let output: Val<::bevy_camera::Camera3dDepthLoadOp> = {
                    {
                        let output: Val<::bevy_camera::Camera3dDepthLoadOp> = <::bevy_camera::Camera3dDepthLoadOp as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::Camera3dDepthLoadOp,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_3_d_depth_texture_usage_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::Camera3dDepthTextureUsage,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::Camera3dDepthTextureUsage>| {
                let output: Val<::bevy_camera::Camera3dDepthTextureUsage> = {
                    {
                        let output: Val<::bevy_camera::Camera3dDepthTextureUsage> = <::bevy_camera::Camera3dDepthTextureUsage as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::Camera3dDepthTextureUsage,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_screen_space_transmission_quality_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::ScreenSpaceTransmissionQuality,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::ScreenSpaceTransmissionQuality>| {
                let output: Val<::bevy_camera::ScreenSpaceTransmissionQuality> = {
                    {
                        let output: Val<::bevy_camera::ScreenSpaceTransmissionQuality> = <::bevy_camera::ScreenSpaceTransmissionQuality as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::ScreenSpaceTransmissionQuality>,
                other: Ref<::bevy_camera::ScreenSpaceTransmissionQuality>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_camera::ScreenSpaceTransmissionQuality as ::std::cmp::PartialEq<
                            ::bevy_camera::ScreenSpaceTransmissionQuality,
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
            ::bevy_camera::ScreenSpaceTransmissionQuality,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_aabb_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::primitives::Aabb,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::primitives::Aabb>| {
                let output: Val<::bevy_camera::primitives::Aabb> = {
                    {
                        let output: Val<::bevy_camera::primitives::Aabb> = <::bevy_camera::primitives::Aabb as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::primitives::Aabb>,
                other: Ref<::bevy_camera::primitives::Aabb>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_camera::primitives::Aabb as ::std::cmp::PartialEq<
                            ::bevy_camera::primitives::Aabb,
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
                let output: Val<::bevy_camera::primitives::Aabb> = {
                    {
                        let output: Val<::bevy_camera::primitives::Aabb> = ::bevy_camera::primitives::Aabb::from_min_max(
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
            |_self: Ref<::bevy_camera::primitives::Aabb>| {
                let output: Val<::bevy_math::Vec3A> = {
                    {
                        let output: Val<::bevy_math::Vec3A> = ::bevy_camera::primitives::Aabb::max(
                                &_self,
                            )
                            .into();
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
            |_self: Ref<::bevy_camera::primitives::Aabb>| {
                let output: Val<::bevy_math::Vec3A> = {
                    {
                        let output: Val<::bevy_math::Vec3A> = ::bevy_camera::primitives::Aabb::min(
                                &_self,
                            )
                            .into();
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
            |
                _self: Ref<::bevy_camera::primitives::Aabb>,
                p_normal: Ref<::bevy_math::Vec3A>,
                world_from_local: Ref<::bevy_math::Mat3A>|
            {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_camera::primitives::Aabb::relative_radius(
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
            ::bevy_camera::primitives::Aabb,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cubemap_frusta_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::primitives::CubemapFrusta,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::primitives::CubemapFrusta>| {
                let output: Val<::bevy_camera::primitives::CubemapFrusta> = {
                    {
                        let output: Val<::bevy_camera::primitives::CubemapFrusta> = <::bevy_camera::primitives::CubemapFrusta as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::primitives::CubemapFrusta,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cubemap_layout_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::primitives::CubemapLayout,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::primitives::CubemapLayout>| {
                let output: Val<::bevy_camera::primitives::CubemapLayout> = {
                    {
                        let output: Val<::bevy_camera::primitives::CubemapLayout> = <::bevy_camera::primitives::CubemapLayout as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::primitives::CubemapLayout,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cascades_frusta_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::primitives::CascadesFrusta,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::primitives::CascadesFrusta>| {
                let output: Val<::bevy_camera::primitives::CascadesFrusta> = {
                    {
                        let output: Val<::bevy_camera::primitives::CascadesFrusta> = <::bevy_camera::primitives::CascadesFrusta as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::primitives::CascadesFrusta,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_custom_projection_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::CustomProjection,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::CustomProjection>| {
                let output: Val<::bevy_camera::CustomProjection> = {
                    {
                        let output: Val<::bevy_camera::CustomProjection> = <::bevy_camera::CustomProjection as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::CustomProjection,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scaling_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::ScalingMode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::ScalingMode>| {
                let output: Val<::bevy_camera::ScalingMode> = {
                    {
                        let output: Val<::bevy_camera::ScalingMode> = <::bevy_camera::ScalingMode as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::ScalingMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_visibility_class_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::visibility::VisibilityClass,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::visibility::VisibilityClass>| {
                let output: Val<::bevy_camera::visibility::VisibilityClass> = {
                    {
                        let output: Val<::bevy_camera::visibility::VisibilityClass> = <::bevy_camera::visibility::VisibilityClass as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::visibility::VisibilityClass,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_no_frustum_culling_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::visibility::NoFrustumCulling,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_camera::visibility::NoFrustumCulling,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_visible_mesh_entities_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::visibility::VisibleMeshEntities,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::visibility::VisibleMeshEntities>| {
                let output: Val<::bevy_camera::visibility::VisibleMeshEntities> = {
                    {
                        let output: Val<
                            ::bevy_camera::visibility::VisibleMeshEntities,
                        > = <::bevy_camera::visibility::VisibleMeshEntities as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::visibility::VisibleMeshEntities,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cubemap_visible_entities_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::visibility::CubemapVisibleEntities,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::visibility::CubemapVisibleEntities>| {
                let output: Val<::bevy_camera::visibility::CubemapVisibleEntities> = {
                    {
                        let output: Val<
                            ::bevy_camera::visibility::CubemapVisibleEntities,
                        > = <::bevy_camera::visibility::CubemapVisibleEntities as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::visibility::CubemapVisibleEntities,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cascades_visible_entities_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::visibility::CascadesVisibleEntities,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::visibility::CascadesVisibleEntities>| {
                let output: Val<::bevy_camera::visibility::CascadesVisibleEntities> = {
                    {
                        let output: Val<
                            ::bevy_camera::visibility::CascadesVisibleEntities,
                        > = <::bevy_camera::visibility::CascadesVisibleEntities as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::visibility::CascadesVisibleEntities,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_no_auto_aabb_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::visibility::NoAutoAabb,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_camera::visibility::NoAutoAabb>| {
                let output: Val<::bevy_camera::visibility::NoAutoAabb> = {
                    {
                        let output: Val<::bevy_camera::visibility::NoAutoAabb> = <::bevy_camera::visibility::NoAutoAabb as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_camera::visibility::NoAutoAabb,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_layers_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::visibility::RenderLayers,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_camera::visibility::RenderLayers>| {
                let output: () = {
                    {
                        let output: () = <::bevy_camera::visibility::RenderLayers as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
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
            |_self: Ref<::bevy_camera::visibility::RenderLayers>| {
                let output: Val<::bevy_camera::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_camera::visibility::RenderLayers> = <::bevy_camera::visibility::RenderLayers as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::visibility::RenderLayers>,
                other: Ref<::bevy_camera::visibility::RenderLayers>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_camera::visibility::RenderLayers as ::std::cmp::PartialEq<
                            ::bevy_camera::visibility::RenderLayers,
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
                _self: Ref<::bevy_camera::visibility::RenderLayers>,
                other: Ref<::bevy_camera::visibility::RenderLayers>|
            {
                let output: Val<::bevy_camera::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_camera::visibility::RenderLayers> = ::bevy_camera::visibility::RenderLayers::intersection(
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
                _self: Ref<::bevy_camera::visibility::RenderLayers>,
                other: Ref<::bevy_camera::visibility::RenderLayers>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_camera::visibility::RenderLayers::intersects(
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
                let output: Val<::bevy_camera::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_camera::visibility::RenderLayers> = ::bevy_camera::visibility::RenderLayers::layer(
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
                let output: Val<::bevy_camera::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_camera::visibility::RenderLayers> = ::bevy_camera::visibility::RenderLayers::none()
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
                _self: Ref<::bevy_camera::visibility::RenderLayers>,
                other: Ref<::bevy_camera::visibility::RenderLayers>|
            {
                let output: Val<::bevy_camera::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_camera::visibility::RenderLayers> = ::bevy_camera::visibility::RenderLayers::symmetric_difference(
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
                _self: Ref<::bevy_camera::visibility::RenderLayers>,
                other: Ref<::bevy_camera::visibility::RenderLayers>|
            {
                let output: Val<::bevy_camera::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_camera::visibility::RenderLayers> = ::bevy_camera::visibility::RenderLayers::union(
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
            |_self: Val<::bevy_camera::visibility::RenderLayers>, layer: usize| {
                let output: Val<::bevy_camera::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_camera::visibility::RenderLayers> = ::bevy_camera::visibility::RenderLayers::with(
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
            |_self: Val<::bevy_camera::visibility::RenderLayers>, layer: usize| {
                let output: Val<::bevy_camera::visibility::RenderLayers> = {
                    {
                        let output: Val<::bevy_camera::visibility::RenderLayers> = ::bevy_camera::visibility::RenderLayers::without(
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
            ::bevy_camera::visibility::RenderLayers,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_visibility_range_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_camera::visibility::VisibilityRange,
    >::new(world)
        .register_documented(
            "abrupt",
            |start: f32, end: f32| {
                let output: Val<::bevy_camera::visibility::VisibilityRange> = {
                    {
                        let output: Val<::bevy_camera::visibility::VisibilityRange> = ::bevy_camera::visibility::VisibilityRange::abrupt(
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
            |_self: Ref<::bevy_camera::visibility::VisibilityRange>| {
                let output: Val<::bevy_camera::visibility::VisibilityRange> = {
                    {
                        let output: Val<::bevy_camera::visibility::VisibilityRange> = <::bevy_camera::visibility::VisibilityRange as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_camera::visibility::VisibilityRange>,
                other: Ref<::bevy_camera::visibility::VisibilityRange>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_camera::visibility::VisibilityRange as ::std::cmp::PartialEq<
                            ::bevy_camera::visibility::VisibilityRange,
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
            |_self: Ref<::bevy_camera::visibility::VisibilityRange>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_camera::visibility::VisibilityRange::is_abrupt(
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
                _self: Ref<::bevy_camera::visibility::VisibilityRange>,
                camera_distance: f32|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_camera::visibility::VisibilityRange::is_culled(
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
                _self: Ref<::bevy_camera::visibility::VisibilityRange>,
                camera_distance: f32|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_camera::visibility::VisibilityRange::is_visible_at_all(
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
            ::bevy_camera::visibility::VisibilityRange,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyCameraScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_clear_color_functions(&mut world);
        register_inherited_visibility_functions(&mut world);
        register_view_visibility_functions(&mut world);
        register_visibility_functions(&mut world);
        register_camera_functions(&mut world);
        register_camera_2_d_functions(&mut world);
        register_camera_3_d_functions(&mut world);
        register_clear_color_config_functions(&mut world);
        register_msaa_writeback_functions(&mut world);
        register_orthographic_projection_functions(&mut world);
        register_perspective_projection_functions(&mut world);
        register_projection_functions(&mut world);
        register_frustum_functions(&mut world);
        register_visible_entities_functions(&mut world);
        register_viewport_functions(&mut world);
        register_main_pass_resolution_override_functions(&mut world);
        register_sub_camera_view_functions(&mut world);
        register_exposure_functions(&mut world);
        register_camera_main_texture_usages_functions(&mut world);
        register_render_target_functions(&mut world);
        register_camera_output_mode_functions(&mut world);
        register_image_render_target_functions(&mut world);
        register_manual_texture_view_handle_functions(&mut world);
        register_normalized_render_target_functions(&mut world);
        register_camera_3_d_depth_load_op_functions(&mut world);
        register_camera_3_d_depth_texture_usage_functions(&mut world);
        register_screen_space_transmission_quality_functions(&mut world);
        register_aabb_functions(&mut world);
        register_cubemap_frusta_functions(&mut world);
        register_cubemap_layout_functions(&mut world);
        register_cascades_frusta_functions(&mut world);
        register_custom_projection_functions(&mut world);
        register_scaling_mode_functions(&mut world);
        register_visibility_class_functions(&mut world);
        register_no_frustum_culling_functions(&mut world);
        register_visible_mesh_entities_functions(&mut world);
        register_cubemap_visible_entities_functions(&mut world);
        register_cascades_visible_entities_functions(&mut world);
        register_no_auto_aabb_functions(&mut world);
        register_render_layers_functions(&mut world);
        register_visibility_range_functions(&mut world);
    }
}
