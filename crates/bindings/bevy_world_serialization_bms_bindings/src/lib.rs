#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
extern crate std;

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
pub struct BevyWorldSerializationScriptingPlugin;
pub(crate) fn register_dynamic_world_root_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_world_serialization::DynamicWorldRoot,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_world_serialization::DynamicWorldRoot>| {
                let output: V<::bevy_world_serialization::DynamicWorldRoot> = {
                    {
                        let output: V<::bevy_world_serialization::DynamicWorldRoot> = <::bevy_world_serialization::DynamicWorldRoot as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_world_serialization::DynamicWorldRoot>,
                other: R<::bevy_world_serialization::DynamicWorldRoot>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_world_serialization::DynamicWorldRoot as ::std::cmp::PartialEq<
                            ::bevy_world_serialization::DynamicWorldRoot,
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
            ::bevy_world_serialization::DynamicWorldRoot,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_world_asset_root_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_world_serialization::WorldAssetRoot,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_world_serialization::WorldAssetRoot>| {
            let output: V<::bevy_world_serialization::WorldAssetRoot> = {
                {
                    let output: V<::bevy_world_serialization::WorldAssetRoot> =
                        <::bevy_world_serialization::WorldAssetRoot as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_world_serialization::WorldAssetRoot>,
         other: R<::bevy_world_serialization::WorldAssetRoot>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_world_serialization::WorldAssetRoot as ::std::cmp::PartialEq<
                            ::bevy_world_serialization::WorldAssetRoot,
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
            ::bevy_world_serialization::WorldAssetRoot,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_world_instance_ready_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_world_serialization::WorldInstanceReady,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_world_serialization::WorldInstanceReady>| {
                let output: V<::bevy_world_serialization::WorldInstanceReady> = {
                    {
                        let output: V<::bevy_world_serialization::WorldInstanceReady> = <::bevy_world_serialization::WorldInstanceReady as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_world_serialization::WorldInstanceReady>,
                other: R<::bevy_world_serialization::WorldInstanceReady>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_world_serialization::WorldInstanceReady as ::std::cmp::PartialEq<
                            ::bevy_world_serialization::WorldInstanceReady,
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
            ::bevy_world_serialization::WorldInstanceReady,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_instance_id_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_world_serialization::InstanceId,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_world_serialization::InstanceId>| {
            let output: V<::bevy_world_serialization::InstanceId> = {
                {
                    let output: V<::bevy_world_serialization::InstanceId> =
                        <::bevy_world_serialization::InstanceId as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_world_serialization::InstanceId>,
         other: R<::bevy_world_serialization::InstanceId>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_world_serialization::InstanceId as ::std::cmp::PartialEq<
                            ::bevy_world_serialization::InstanceId,
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
            ::bevy_world_serialization::InstanceId,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyWorldSerializationScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_dynamic_world_root_functions(&mut world);
        register_world_asset_root_functions(&mut world);
        register_world_instance_ready_functions(&mut world);
        register_instance_id_functions(&mut world);
    }
}
