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
pub struct BevySceneScriptingPlugin;
pub(crate) fn register_dynamic_scene_root_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_scene::DynamicSceneRoot,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: R<::bevy_scene::DynamicSceneRoot>| {
                let output: () = {
                    {
                        let output: () = <::bevy_scene::DynamicSceneRoot as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: R<::bevy_scene::DynamicSceneRoot>| {
                let output: V<::bevy_scene::DynamicSceneRoot> = {
                    {
                        let output: V<::bevy_scene::DynamicSceneRoot> = <::bevy_scene::DynamicSceneRoot as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: R<::bevy_scene::DynamicSceneRoot>,
                other: R<::bevy_scene::DynamicSceneRoot>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_scene::DynamicSceneRoot as ::std::cmp::PartialEq<
                            ::bevy_scene::DynamicSceneRoot,
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
            |_self: R<::bevy_scene::SceneRoot>| {
                let output: () = {
                    {
                        let output: () = <::bevy_scene::SceneRoot as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: R<::bevy_scene::SceneRoot>| {
                let output: V<::bevy_scene::SceneRoot> = {
                    {
                        let output: V<::bevy_scene::SceneRoot> = <::bevy_scene::SceneRoot as ::std::clone::Clone>::clone(
                                &_self,
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
            |_self: R<::bevy_scene::SceneRoot>, other: R<::bevy_scene::SceneRoot>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_scene::SceneRoot as ::std::cmp::PartialEq<
                            ::bevy_scene::SceneRoot,
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
            |_self: R<::bevy_scene::SceneInstanceReady>| {
                let output: () = {
                    {
                        let output: () = <::bevy_scene::SceneInstanceReady as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: R<::bevy_scene::SceneInstanceReady>| {
                let output: V<::bevy_scene::SceneInstanceReady> = {
                    {
                        let output: V<::bevy_scene::SceneInstanceReady> = <::bevy_scene::SceneInstanceReady as ::std::clone::Clone>::clone(
                                &_self,
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
                _self: R<::bevy_scene::SceneInstanceReady>,
                other: R<::bevy_scene::SceneInstanceReady>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_scene::SceneInstanceReady as ::std::cmp::PartialEq<
                            ::bevy_scene::SceneInstanceReady,
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
            |_self: R<::bevy_scene::InstanceId>| {
                let output: () = {
                    {
                        let output: () = <::bevy_scene::InstanceId as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
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
            |_self: R<::bevy_scene::InstanceId>| {
                let output: V<::bevy_scene::InstanceId> = {
                    {
                        let output: V<::bevy_scene::InstanceId> = <::bevy_scene::InstanceId as ::std::clone::Clone>::clone(
                                &_self,
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
            |_self: R<::bevy_scene::InstanceId>, other: R<::bevy_scene::InstanceId>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_scene::InstanceId as ::std::cmp::PartialEq<
                            ::bevy_scene::InstanceId,
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
