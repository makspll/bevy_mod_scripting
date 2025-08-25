#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_mod_scripting_core::bindings::{
    ReflectReference,
    function::{
        from::{Mut, Ref, Val},
        namespace::NamespaceBuilder,
    },
};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyGizmosScriptingPlugin;
pub(crate) fn register_aabb_gizmo_config_group_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::aabb::AabbGizmoConfigGroup,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_gizmos::aabb::AabbGizmoConfigGroup>| {
            let output: Val<::bevy_gizmos::aabb::AabbGizmoConfigGroup> = {
                {
                    let output: Val<::bevy_gizmos::aabb::AabbGizmoConfigGroup> =
                        <::bevy_gizmos::aabb::AabbGizmoConfigGroup as ::std::clone::Clone>::clone(
                            &_self,
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
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_show_aabb_gizmo_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::aabb::ShowAabbGizmo,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_gizmos::aabb::ShowAabbGizmo,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_default_gizmo_config_group_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::DefaultGizmoConfigGroup,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_gizmos::config::DefaultGizmoConfigGroup,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gizmo_config_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::GizmoConfig,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_gizmos::config::GizmoConfig>| {
            let output: Val<::bevy_gizmos::config::GizmoConfig> = {
                {
                    let output: Val<::bevy_gizmos::config::GizmoConfig> =
                        <::bevy_gizmos::config::GizmoConfig as ::std::clone::Clone>::clone(&_self)
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
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gizmo_config_store_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::GizmoConfigStore,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_gizmos::config::GizmoConfigStore,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gizmo_line_config_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::GizmoLineConfig,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_gizmos::config::GizmoLineConfig>| {
            let output: Val<::bevy_gizmos::config::GizmoLineConfig> = {
                {
                    let output: Val<::bevy_gizmos::config::GizmoLineConfig> =
                        <::bevy_gizmos::config::GizmoLineConfig as ::std::clone::Clone>::clone(
                            &_self,
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
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gizmo_line_joint_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::GizmoLineJoint,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_gizmos::config::GizmoLineJoint>| {
                let output: () = {
                    {
                        let output: () = <::bevy_gizmos::config::GizmoLineJoint as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
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
            |_self: Ref<::bevy_gizmos::config::GizmoLineJoint>| {
                let output: Val<::bevy_gizmos::config::GizmoLineJoint> = {
                    {
                        let output: Val<::bevy_gizmos::config::GizmoLineJoint> = <::bevy_gizmos::config::GizmoLineJoint as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
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
                _self: Ref<::bevy_gizmos::config::GizmoLineJoint>,
                other: Ref<::bevy_gizmos::config::GizmoLineJoint>|
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
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gizmo_line_style_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::GizmoLineStyle,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_gizmos::config::GizmoLineStyle>| {
            let output: Val<::bevy_gizmos::config::GizmoLineStyle> = {
                {
                    let output: Val<::bevy_gizmos::config::GizmoLineStyle> =
                        <::bevy_gizmos::config::GizmoLineStyle as ::std::clone::Clone>::clone(
                            &_self,
                        )
                        .into();
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
        |_self: Ref<::bevy_gizmos::config::GizmoLineStyle>,
         other: Ref<::bevy_gizmos::config::GizmoLineStyle>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_gizmos::config::GizmoLineStyle as ::std::cmp::PartialEq<
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
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gizmo_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::retained::Gizmo,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_gizmos::retained::Gizmo>| {
            let output: Val<::bevy_gizmos::retained::Gizmo> = {
                {
                    let output: Val<::bevy_gizmos::retained::Gizmo> =
                        <::bevy_gizmos::retained::Gizmo as ::std::clone::Clone>::clone(&_self)
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
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_light_gizmo_color_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::light::LightGizmoColor,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_gizmos::light::LightGizmoColor>| {
            let output: Val<::bevy_gizmos::light::LightGizmoColor> = {
                {
                    let output: Val<::bevy_gizmos::light::LightGizmoColor> =
                        <::bevy_gizmos::light::LightGizmoColor as ::std::clone::Clone>::clone(
                            &_self,
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
            ::bevy_gizmos::light::LightGizmoColor,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_light_gizmo_config_group_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::light::LightGizmoConfigGroup,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_gizmos::light::LightGizmoConfigGroup>| {
                let output: Val<::bevy_gizmos::light::LightGizmoConfigGroup> = {
                    {
                        let output: Val<::bevy_gizmos::light::LightGizmoConfigGroup> = <::bevy_gizmos::light::LightGizmoConfigGroup as ::std::clone::Clone>::clone(
                                &_self,
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
            ::bevy_gizmos::light::LightGizmoConfigGroup,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_show_light_gizmo_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::light::ShowLightGizmo,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_gizmos::light::ShowLightGizmo,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_erased_gizmo_config_group_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos::config::ErasedGizmoConfigGroup,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_gizmos::config::ErasedGizmoConfigGroup>| {
                let output: Val<::bevy_gizmos::config::ErasedGizmoConfigGroup> = {
                    {
                        let output: Val<::bevy_gizmos::config::ErasedGizmoConfigGroup> = <::bevy_gizmos::config::ErasedGizmoConfigGroup as ::std::clone::Clone>::clone(
                                &_self,
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
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyGizmosScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_aabb_gizmo_config_group_functions(&mut world);
        register_show_aabb_gizmo_functions(&mut world);
        register_default_gizmo_config_group_functions(&mut world);
        register_gizmo_config_functions(&mut world);
        register_gizmo_config_store_functions(&mut world);
        register_gizmo_line_config_functions(&mut world);
        register_gizmo_line_joint_functions(&mut world);
        register_gizmo_line_style_functions(&mut world);
        register_gizmo_functions(&mut world);
        register_light_gizmo_color_functions(&mut world);
        register_light_gizmo_config_group_functions(&mut world);
        register_show_light_gizmo_functions(&mut world);
        register_erased_gizmo_config_group_functions(&mut world);
    }
}
