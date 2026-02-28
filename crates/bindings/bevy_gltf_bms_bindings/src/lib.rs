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
pub struct BevyGltfScriptingPlugin;
pub(crate) fn register_gltf_extras_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gltf::GltfExtras,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gltf::GltfExtras>| {
                let output: V<::bevy_gltf::GltfExtras> = {
                    {
                        let output: V<::bevy_gltf::GltfExtras> = <::bevy_gltf::GltfExtras as ::std::clone::Clone>::clone(
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
            ::bevy_gltf::GltfExtras,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gltf_scene_extras_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gltf::GltfSceneExtras,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_gltf::GltfSceneExtras>| {
            let output: V<::bevy_gltf::GltfSceneExtras> = {
                {
                    let output: V<::bevy_gltf::GltfSceneExtras> =
                        <::bevy_gltf::GltfSceneExtras as ::std::clone::Clone>::clone(&_self).into();
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
            ::bevy_gltf::GltfSceneExtras,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gltf_mesh_extras_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gltf::GltfMeshExtras,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gltf::GltfMeshExtras>| {
                let output: V<::bevy_gltf::GltfMeshExtras> = {
                    {
                        let output: V<::bevy_gltf::GltfMeshExtras> = <::bevy_gltf::GltfMeshExtras as ::std::clone::Clone>::clone(
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
            ::bevy_gltf::GltfMeshExtras,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gltf_mesh_name_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gltf::GltfMeshName,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_gltf::GltfMeshName>| {
                let output: V<::bevy_gltf::GltfMeshName> = {
                    {
                        let output: V<::bevy_gltf::GltfMeshName> = <::bevy_gltf::GltfMeshName as ::std::clone::Clone>::clone(
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
            ::bevy_gltf::GltfMeshName,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gltf_material_extras_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gltf::GltfMaterialExtras,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_gltf::GltfMaterialExtras>| {
            let output: V<::bevy_gltf::GltfMaterialExtras> = {
                {
                    let output: V<::bevy_gltf::GltfMaterialExtras> =
                        <::bevy_gltf::GltfMaterialExtras as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_gltf::GltfMaterialExtras,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gltf_material_name_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gltf::GltfMaterialName,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_gltf::GltfMaterialName>| {
            let output: V<::bevy_gltf::GltfMaterialName> = {
                {
                    let output: V<::bevy_gltf::GltfMaterialName> =
                        <::bevy_gltf::GltfMaterialName as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_gltf::GltfMaterialName,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyGltfScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_gltf_extras_functions(&mut world);
        register_gltf_scene_extras_functions(&mut world);
        register_gltf_mesh_extras_functions(&mut world);
        register_gltf_mesh_name_functions(&mut world);
        register_gltf_material_extras_functions(&mut world);
        register_gltf_material_name_functions(&mut world);
    }
}
