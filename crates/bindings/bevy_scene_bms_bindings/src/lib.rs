
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]



use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{Ref, Mut, Val},
        namespace::NamespaceBuilder, glue::safe_transmute,
    },
};
use bevy_ecs::prelude::*;
use bevy_app::{App, Plugin};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevySceneScriptingPlugin;
pub(crate) fn register_dynamic_scene_root_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_scene::DynamicSceneRoot,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_scene::DynamicSceneRoot>| {
                let output: () = {
                    {
                        let output: () = <::bevy_scene::DynamicSceneRoot as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_scene::DynamicSceneRoot>| {
                let output: Val<::bevy_scene::DynamicSceneRoot> = {
                    {
                        let output: ::bevy_scene::DynamicSceneRoot = <::bevy_scene::DynamicSceneRoot as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_scene::DynamicSceneRoot>,
                other: Ref<::bevy_scene::DynamicSceneRoot>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_scene::DynamicSceneRoot as ::std::cmp::PartialEq<
                            ::bevy_scene::DynamicSceneRoot,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_scene::DynamicSceneRoot,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scene_root_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_scene::SceneRoot,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_scene::SceneRoot>| {
                let output: () = {
                    {
                        let output: () = <::bevy_scene::SceneRoot as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_scene::SceneRoot>| {
                let output: Val<::bevy_scene::SceneRoot> = {
                    {
                        let output: ::bevy_scene::SceneRoot = <::bevy_scene::SceneRoot as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |_self: Ref<::bevy_scene::SceneRoot>, other: Ref<::bevy_scene::SceneRoot>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_scene::SceneRoot as ::std::cmp::PartialEq<
                            ::bevy_scene::SceneRoot,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_scene::SceneRoot,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scene_instance_ready_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_scene::SceneInstanceReady,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_scene::SceneInstanceReady>| {
                let output: () = {
                    {
                        let output: () = <::bevy_scene::SceneInstanceReady as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_scene::SceneInstanceReady>| {
                let output: Val<::bevy_scene::SceneInstanceReady> = {
                    {
                        let output: ::bevy_scene::SceneInstanceReady = <::bevy_scene::SceneInstanceReady as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
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
                _self: Ref<::bevy_scene::SceneInstanceReady>,
                other: Ref<::bevy_scene::SceneInstanceReady>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_scene::SceneInstanceReady as ::std::cmp::PartialEq<
                            ::bevy_scene::SceneInstanceReady,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_scene::SceneInstanceReady,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_instance_id_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_scene::InstanceId,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_scene::InstanceId>| {
                let output: () = {
                    {
                        let output: () = <::bevy_scene::InstanceId as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_scene::InstanceId>| {
                let output: Val<::bevy_scene::InstanceId> = {
                    {
                        let output: ::bevy_scene::InstanceId = <::bevy_scene::InstanceId as ::std::clone::Clone>::clone(
                            safe_transmute(_self),
                        );
                        safe_transmute(output)
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |_self: Ref<::bevy_scene::InstanceId>, other: Ref<::bevy_scene::InstanceId>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_scene::InstanceId as ::std::cmp::PartialEq<
                            ::bevy_scene::InstanceId,
                        >>::eq(safe_transmute(_self), safe_transmute(other));
                        safe_transmute(output)
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
            ::bevy_scene::InstanceId,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevySceneScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_dynamic_scene_root_functions(&mut world);
        register_scene_root_functions(&mut world);
        register_scene_instance_ready_functions(&mut world);
        register_instance_id_functions(&mut world);
    }
}
