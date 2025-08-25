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
pub struct BevySceneScriptingPlugin;
pub(crate) fn register_dynamic_scene_root_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_scene::prelude::DynamicSceneRoot,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_scene::prelude::DynamicSceneRoot>| {
                let output: () = {
                    {
                        let output: () = <::bevy_scene::prelude::DynamicSceneRoot as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_scene::prelude::DynamicSceneRoot>| {
                let output: Val<::bevy_scene::prelude::DynamicSceneRoot> = {
                    {
                        let output: Val<::bevy_scene::prelude::DynamicSceneRoot> = <::bevy_scene::prelude::DynamicSceneRoot as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_scene::prelude::DynamicSceneRoot>,
                other: Ref<::bevy_scene::prelude::DynamicSceneRoot>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_scene::prelude::DynamicSceneRoot as ::std::cmp::PartialEq<
                            ::bevy_scene::prelude::DynamicSceneRoot,
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
            ::bevy_scene::prelude::DynamicSceneRoot,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scene_root_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_scene::prelude::SceneRoot,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_scene::prelude::SceneRoot>| {
                let output: () = {
                    {
                        let output: () = <::bevy_scene::prelude::SceneRoot as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_scene::prelude::SceneRoot>| {
                let output: Val<::bevy_scene::prelude::SceneRoot> = {
                    {
                        let output: Val<::bevy_scene::prelude::SceneRoot> = <::bevy_scene::prelude::SceneRoot as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_scene::prelude::SceneRoot>,
                other: Ref<::bevy_scene::prelude::SceneRoot>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_scene::prelude::SceneRoot as ::std::cmp::PartialEq<
                            ::bevy_scene::prelude::SceneRoot,
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
            ::bevy_scene::prelude::SceneRoot,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_scene_instance_ready_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_scene::SceneInstanceReady,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_scene::SceneInstanceReady>| {
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
            |_self: Ref<::bevy_scene::SceneInstanceReady>| {
                let output: Val<::bevy_scene::SceneInstanceReady> = {
                    {
                        let output: Val<::bevy_scene::SceneInstanceReady> = <::bevy_scene::SceneInstanceReady as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_scene::SceneInstanceReady>,
                other: Ref<::bevy_scene::SceneInstanceReady>|
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
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_instance_id_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_scene::InstanceId,
    >::new(world)
    .register_documented(
        "assert_receiver_is_total_eq",
        |_self: Ref<::bevy_scene::InstanceId>| {
            let output: () = {
                {
                    let output: () =
                        <::bevy_scene::InstanceId as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
        |_self: Ref<::bevy_scene::InstanceId>| {
            let output: Val<::bevy_scene::InstanceId> = {
                {
                    let output: Val<::bevy_scene::InstanceId> =
                        <::bevy_scene::InstanceId as ::std::clone::Clone>::clone(&_self).into();
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
        |_self: Ref<::bevy_scene::InstanceId>, other: Ref<::bevy_scene::InstanceId>| {
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
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
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
