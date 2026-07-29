
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
extern crate std;


use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{R, M, V},
        namespace::NamespaceBuilder,
    },
};
use bevy_ecs::prelude::*;
use bevy_app::{App, Plugin};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyGizmosScriptingPlugin;
pub(crate) fn register_aabb_gizmo_config_group_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::aabb::AabbGizmoConfigGroup,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gizmos::aabb::AabbGizmoConfigGroup>| {
                let output: V<::bevy_gizmos::aabb::AabbGizmoConfigGroup> = {
                    {
                        let output: V<::bevy_gizmos::aabb::AabbGizmoConfigGroup> = <::bevy_gizmos::aabb::AabbGizmoConfigGroup as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_gizmos::aabb::AabbGizmoConfigGroup,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_show_aabb_gizmo_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::aabb::ShowAabbGizmo,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_gizmos::aabb::ShowAabbGizmo,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_frustum_gizmo_config_group_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::frustum::FrustumGizmoConfigGroup,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gizmos::frustum::FrustumGizmoConfigGroup>| {
                let output: V<::bevy_gizmos::frustum::FrustumGizmoConfigGroup> = {
                    {
                        let output: V<::bevy_gizmos::frustum::FrustumGizmoConfigGroup> = <::bevy_gizmos::frustum::FrustumGizmoConfigGroup as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_gizmos::frustum::FrustumGizmoConfigGroup,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_show_frustum_gizmo_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::frustum::ShowFrustumGizmo,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_gizmos::frustum::ShowFrustumGizmo,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_show_skinned_mesh_bounds_gizmo_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::skinned_mesh_bounds::ShowSkinnedMeshBoundsGizmo,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_gizmos::skinned_mesh_bounds::ShowSkinnedMeshBoundsGizmo,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_skinned_mesh_bounds_gizmo_config_group_functions(
    world: &mut World,
) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::skinned_mesh_bounds::SkinnedMeshBoundsGizmoConfigGroup,
    >::new(world)
        .register_documented(
            "clone",
            |
                _self: R<
                    ::bevy_gizmos::skinned_mesh_bounds::SkinnedMeshBoundsGizmoConfigGroup,
                >|
            {
                let output: V<
                    ::bevy_gizmos::skinned_mesh_bounds::SkinnedMeshBoundsGizmoConfigGroup,
                > = {
                    {
                        let output: V<
                            ::bevy_gizmos::skinned_mesh_bounds::SkinnedMeshBoundsGizmoConfigGroup,
                        > = <::bevy_gizmos::skinned_mesh_bounds::SkinnedMeshBoundsGizmoConfigGroup as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_gizmos::skinned_mesh_bounds::SkinnedMeshBoundsGizmoConfigGroup,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_default_gizmo_config_group_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::DefaultGizmoConfigGroup,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_gizmos::config::DefaultGizmoConfigGroup,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gizmo_config_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::GizmoConfig,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gizmos::config::GizmoConfig>| {
                let output: V<::bevy_gizmos::config::GizmoConfig> = {
                    {
                        let output: V<::bevy_gizmos::config::GizmoConfig> = <::bevy_gizmos::config::GizmoConfig as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_gizmos::config::GizmoConfig,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gizmo_config_store_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::GizmoConfigStore,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_gizmos::config::GizmoConfigStore,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gizmo_line_config_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::GizmoLineConfig,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gizmos::config::GizmoLineConfig>| {
                let output: V<::bevy_gizmos::config::GizmoLineConfig> = {
                    {
                        let output: V<::bevy_gizmos::config::GizmoLineConfig> = <::bevy_gizmos::config::GizmoLineConfig as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_gizmos::config::GizmoLineConfig,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gizmo_line_joint_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::GizmoLineJoint,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gizmos::config::GizmoLineJoint>| {
                let output: V<::bevy_gizmos::config::GizmoLineJoint> = {
                    {
                        let output: V<::bevy_gizmos::config::GizmoLineJoint> = <::bevy_gizmos::config::GizmoLineJoint as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: R<::bevy_gizmos::config::GizmoLineJoint>,
                other: R<::bevy_gizmos::config::GizmoLineJoint>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_gizmos::config::GizmoLineJoint as ::std::cmp::PartialEq<
                            ::bevy_gizmos::config::GizmoLineJoint,
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
            ::bevy_gizmos::config::GizmoLineJoint,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gizmo_line_style_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::GizmoLineStyle,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gizmos::config::GizmoLineStyle>| {
                let output: V<::bevy_gizmos::config::GizmoLineStyle> = {
                    {
                        let output: V<::bevy_gizmos::config::GizmoLineStyle> = <::bevy_gizmos::config::GizmoLineStyle as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: R<::bevy_gizmos::config::GizmoLineStyle>,
                other: R<::bevy_gizmos::config::GizmoLineStyle>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_gizmos::config::GizmoLineStyle as ::std::cmp::PartialEq<
                            ::bevy_gizmos::config::GizmoLineStyle,
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
            ::bevy_gizmos::config::GizmoLineStyle,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gizmo_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::retained::Gizmo,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gizmos::retained::Gizmo>| {
                let output: V<::bevy_gizmos::retained::Gizmo> = {
                    {
                        let output: V<::bevy_gizmos::retained::Gizmo> = <::bevy_gizmos::retained::Gizmo as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_gizmos::retained::Gizmo,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_transform_gizmo_axis_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::transform_gizmo::TransformGizmoAxis,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gizmos::transform_gizmo::TransformGizmoAxis>| {
                let output: V<::bevy_gizmos::transform_gizmo::TransformGizmoAxis> = {
                    {
                        let output: V<
                            ::bevy_gizmos::transform_gizmo::TransformGizmoAxis,
                        > = <::bevy_gizmos::transform_gizmo::TransformGizmoAxis as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: R<::bevy_gizmos::transform_gizmo::TransformGizmoAxis>,
                other: R<::bevy_gizmos::transform_gizmo::TransformGizmoAxis>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_gizmos::transform_gizmo::TransformGizmoAxis as ::std::cmp::PartialEq<
                            ::bevy_gizmos::transform_gizmo::TransformGizmoAxis,
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
            ::bevy_gizmos::transform_gizmo::TransformGizmoAxis,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_transform_gizmo_camera_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::transform_gizmo::TransformGizmoCamera,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gizmos::transform_gizmo::TransformGizmoCamera>| {
                let output: V<::bevy_gizmos::transform_gizmo::TransformGizmoCamera> = {
                    {
                        let output: V<
                            ::bevy_gizmos::transform_gizmo::TransformGizmoCamera,
                        > = <::bevy_gizmos::transform_gizmo::TransformGizmoCamera as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_gizmos::transform_gizmo::TransformGizmoCamera,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_transform_gizmo_focus_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::transform_gizmo::TransformGizmoFocus,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gizmos::transform_gizmo::TransformGizmoFocus>| {
                let output: V<::bevy_gizmos::transform_gizmo::TransformGizmoFocus> = {
                    {
                        let output: V<
                            ::bevy_gizmos::transform_gizmo::TransformGizmoFocus,
                        > = <::bevy_gizmos::transform_gizmo::TransformGizmoFocus as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_gizmos::transform_gizmo::TransformGizmoFocus,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_transform_gizmo_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::transform_gizmo::TransformGizmoMode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gizmos::transform_gizmo::TransformGizmoMode>| {
                let output: V<::bevy_gizmos::transform_gizmo::TransformGizmoMode> = {
                    {
                        let output: V<
                            ::bevy_gizmos::transform_gizmo::TransformGizmoMode,
                        > = <::bevy_gizmos::transform_gizmo::TransformGizmoMode as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: R<::bevy_gizmos::transform_gizmo::TransformGizmoMode>,
                other: R<::bevy_gizmos::transform_gizmo::TransformGizmoMode>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_gizmos::transform_gizmo::TransformGizmoMode as ::std::cmp::PartialEq<
                            ::bevy_gizmos::transform_gizmo::TransformGizmoMode,
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
            ::bevy_gizmos::transform_gizmo::TransformGizmoMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_transform_gizmo_settings_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::transform_gizmo::TransformGizmoSettings,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_gizmos::transform_gizmo::TransformGizmoSettings,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_transform_gizmo_space_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::transform_gizmo::TransformGizmoSpace,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gizmos::transform_gizmo::TransformGizmoSpace>| {
                let output: V<::bevy_gizmos::transform_gizmo::TransformGizmoSpace> = {
                    {
                        let output: V<
                            ::bevy_gizmos::transform_gizmo::TransformGizmoSpace,
                        > = <::bevy_gizmos::transform_gizmo::TransformGizmoSpace as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: R<::bevy_gizmos::transform_gizmo::TransformGizmoSpace>,
                other: R<::bevy_gizmos::transform_gizmo::TransformGizmoSpace>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_gizmos::transform_gizmo::TransformGizmoSpace as ::std::cmp::PartialEq<
                            ::bevy_gizmos::transform_gizmo::TransformGizmoSpace,
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
            ::bevy_gizmos::transform_gizmo::TransformGizmoSpace,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_transform_gizmo_state_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::transform_gizmo::TransformGizmoState,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_gizmos::transform_gizmo::TransformGizmoState,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_erased_gizmo_config_group_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::ErasedGizmoConfigGroup,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gizmos::config::ErasedGizmoConfigGroup>| {
                let output: V<::bevy_gizmos::config::ErasedGizmoConfigGroup> = {
                    {
                        let output: V<::bevy_gizmos::config::ErasedGizmoConfigGroup> = <::bevy_gizmos::config::ErasedGizmoConfigGroup as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_gizmos::config::ErasedGizmoConfigGroup,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyGizmosScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_aabb_gizmo_config_group_functions(&mut world);
        register_show_aabb_gizmo_functions(&mut world);
        register_frustum_gizmo_config_group_functions(&mut world);
        register_show_frustum_gizmo_functions(&mut world);
        register_show_skinned_mesh_bounds_gizmo_functions(&mut world);
        register_skinned_mesh_bounds_gizmo_config_group_functions(&mut world);
        register_default_gizmo_config_group_functions(&mut world);
        register_gizmo_config_functions(&mut world);
        register_gizmo_config_store_functions(&mut world);
        register_gizmo_line_config_functions(&mut world);
        register_gizmo_line_joint_functions(&mut world);
        register_gizmo_line_style_functions(&mut world);
        register_gizmo_functions(&mut world);
        register_transform_gizmo_axis_functions(&mut world);
        register_transform_gizmo_camera_functions(&mut world);
        register_transform_gizmo_focus_functions(&mut world);
        register_transform_gizmo_mode_functions(&mut world);
        register_transform_gizmo_settings_functions(&mut world);
        register_transform_gizmo_space_functions(&mut world);
        register_transform_gizmo_state_functions(&mut world);
        register_erased_gizmo_config_group_functions(&mut world);
    }
}
