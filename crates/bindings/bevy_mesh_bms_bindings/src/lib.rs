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
pub struct BevyMeshScriptingPlugin;
pub(crate) fn register_indices_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::Indices,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::Indices>| {
                let output: Val<::bevy_mesh::Indices> = {
                    {
                        let output: Val<::bevy_mesh::Indices> = <::bevy_mesh::Indices as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_mesh::Indices>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_mesh::Indices::is_empty(&_self).into();
                        output
                    }
                };
                output
            },
            " Returns `true` if there are no indices.",
            &["_self"],
        )
        .register_documented(
            "len",
            |_self: Ref<::bevy_mesh::Indices>| {
                let output: usize = {
                    {
                        let output: usize = ::bevy_mesh::Indices::len(&_self).into();
                        output
                    }
                };
                output
            },
            " Returns the number of indices.",
            &["_self"],
        )
        .register_documented(
            "push",
            |mut _self: Mut<::bevy_mesh::Indices>, index: u32| {
                let output: () = {
                    {
                        let output: () = ::bevy_mesh::Indices::push(&mut _self, index)
                            .into();
                        output
                    }
                };
                output
            },
            " Add an index. If the index is greater than `u16::MAX`,\n the storage will be converted to `u32`.",
            &["_self", "index"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::Indices,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_mesh_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::Mesh,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::Mesh>| {
                let output: Val<::bevy_mesh::Mesh> = {
                    {
                        let output: Val<::bevy_mesh::Mesh> = <::bevy_mesh::Mesh as ::std::clone::Clone>::clone(
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
            "compute_flat_normals",
            |mut _self: Mut<::bevy_mesh::Mesh>| {
                let output: () = {
                    {
                        let output: () = ::bevy_mesh::Mesh::compute_flat_normals(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Calculates the [`Mesh::ATTRIBUTE_NORMAL`] of a mesh.\n # Panics\n Panics if [`Indices`] are set or [`Mesh::ATTRIBUTE_POSITION`] is not of type `float3`.\n Panics if the mesh has any other topology than [`PrimitiveTopology::TriangleList`].\n Consider calling [`Mesh::duplicate_vertices`] or exporting your mesh with normal\n attributes.\n FIXME: This should handle more cases since this is called as a part of gltf\n mesh loading where we can't really blame users for loading meshes that might\n not conform to the limitations here!",
            &["_self"],
        )
        .register_documented(
            "compute_normals",
            |mut _self: Mut<::bevy_mesh::Mesh>| {
                let output: () = {
                    {
                        let output: () = ::bevy_mesh::Mesh::compute_normals(&mut _self)
                            .into();
                        output
                    }
                };
                output
            },
            " Calculates the [`Mesh::ATTRIBUTE_NORMAL`] of a mesh.\n If the mesh is indexed, this defaults to smooth normals. Otherwise, it defaults to flat\n normals.\n # Panics\n Panics if [`Mesh::ATTRIBUTE_POSITION`] is not of type `float3`.\n Panics if the mesh has any other topology than [`PrimitiveTopology::TriangleList`].\n FIXME: This should handle more cases since this is called as a part of gltf\n mesh loading where we can't really blame users for loading meshes that might\n not conform to the limitations here!",
            &["_self"],
        )
        .register_documented(
            "compute_smooth_normals",
            |mut _self: Mut<::bevy_mesh::Mesh>| {
                let output: () = {
                    {
                        let output: () = ::bevy_mesh::Mesh::compute_smooth_normals(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Calculates the [`Mesh::ATTRIBUTE_NORMAL`] of an indexed mesh, smoothing normals for shared\n vertices.\n # Panics\n Panics if [`Mesh::ATTRIBUTE_POSITION`] is not of type `float3`.\n Panics if the mesh has any other topology than [`PrimitiveTopology::TriangleList`].\n Panics if the mesh does not have indices defined.\n FIXME: This should handle more cases since this is called as a part of gltf\n mesh loading where we can't really blame users for loading meshes that might\n not conform to the limitations here!",
            &["_self"],
        )
        .register_documented(
            "count_vertices",
            |_self: Ref<::bevy_mesh::Mesh>| {
                let output: usize = {
                    {
                        let output: usize = ::bevy_mesh::Mesh::count_vertices(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Counts all vertices of the mesh.\n If the attributes have different vertex counts, the smallest is returned.",
            &["_self"],
        )
        .register_documented(
            "create_packed_vertex_buffer_data",
            |_self: Ref<::bevy_mesh::Mesh>| {
                let output: ::std::vec::Vec<u8> = {
                    {
                        let output: ::std::vec::Vec<u8> = ::bevy_mesh::Mesh::create_packed_vertex_buffer_data(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Computes and returns the vertex data of the mesh as bytes.\n Therefore the attributes are located in the order of their [`MeshVertexAttribute::id`].\n This is used to transform the vertex data into a GPU friendly format.\n If the vertex attributes have different lengths, they are all truncated to\n the length of the smallest.\n This is a convenience method which allocates a Vec.\n Prefer pre-allocating and using [`Mesh::write_packed_vertex_buffer_data`] when possible.",
            &["_self"],
        )
        .register_documented(
            "duplicate_vertices",
            |mut _self: Mut<::bevy_mesh::Mesh>| {
                let output: () = {
                    {
                        let output: () = ::bevy_mesh::Mesh::duplicate_vertices(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Duplicates the vertex attributes so that no vertices are shared.\n This can dramatically increase the vertex count, so make sure this is what you want.\n Does nothing if no [Indices] are set.",
            &["_self"],
        )
        .register_documented(
            "get_vertex_buffer_size",
            |_self: Ref<::bevy_mesh::Mesh>| {
                let output: usize = {
                    {
                        let output: usize = ::bevy_mesh::Mesh::get_vertex_buffer_size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the size required for the vertex buffer in bytes.",
            &["_self"],
        )
        .register_documented(
            "get_vertex_size",
            |_self: Ref<::bevy_mesh::Mesh>| {
                let output: u64 = {
                    {
                        let output: u64 = ::bevy_mesh::Mesh::get_vertex_size(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the size of a vertex in bytes.",
            &["_self"],
        )
        .register_documented(
            "has_morph_targets",
            |_self: Ref<::bevy_mesh::Mesh>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_mesh::Mesh::has_morph_targets(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Whether this mesh has morph targets.",
            &["_self"],
        )
        .register_documented(
            "insert_indices",
            |mut _self: Mut<::bevy_mesh::Mesh>, indices: Val<::bevy_mesh::Indices>| {
                let output: () = {
                    {
                        let output: () = ::bevy_mesh::Mesh::insert_indices(
                                &mut _self,
                                indices.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the vertex indices of the mesh. They describe how triangles are constructed out of the\n vertex attributes and are therefore only useful for the [`PrimitiveTopology`] variants\n that use triangles.",
            &["_self", "indices"],
        )
        .register_documented(
            "normalize_joint_weights",
            |mut _self: Mut<::bevy_mesh::Mesh>| {
                let output: () = {
                    {
                        let output: () = ::bevy_mesh::Mesh::normalize_joint_weights(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Normalize joint weights so they sum to 1.",
            &["_self"],
        )
        .register_documented(
            "rotate_by",
            |mut _self: Mut<::bevy_mesh::Mesh>, rotation: Val<::bevy_math::Quat>| {
                let output: () = {
                    {
                        let output: () = ::bevy_mesh::Mesh::rotate_by(
                                &mut _self,
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates the vertex positions, normals, and tangents of the mesh in place by the given [`Quat`].\n `Aabb` of entities with modified mesh are not updated automatically.",
            &["_self", "rotation"],
        )
        .register_documented(
            "rotated_by",
            |_self: Val<::bevy_mesh::Mesh>, rotation: Val<::bevy_math::Quat>| {
                let output: Val<::bevy_mesh::Mesh> = {
                    {
                        let output: Val<::bevy_mesh::Mesh> = ::bevy_mesh::Mesh::rotated_by(
                                _self.into_inner(),
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates the vertex positions, normals, and tangents of the mesh by the given [`Quat`].\n `Aabb` of entities with modified mesh are not updated automatically.",
            &["_self", "rotation"],
        )
        .register_documented(
            "scale_by",
            |mut _self: Mut<::bevy_mesh::Mesh>, scale: Val<::bevy_math::Vec3>| {
                let output: () = {
                    {
                        let output: () = ::bevy_mesh::Mesh::scale_by(
                                &mut _self,
                                scale.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Scales the vertex positions, normals, and tangents of the mesh in place by the given [`Vec3`].\n `Aabb` of entities with modified mesh are not updated automatically.",
            &["_self", "scale"],
        )
        .register_documented(
            "scaled_by",
            |_self: Val<::bevy_mesh::Mesh>, scale: Val<::bevy_math::Vec3>| {
                let output: Val<::bevy_mesh::Mesh> = {
                    {
                        let output: Val<::bevy_mesh::Mesh> = ::bevy_mesh::Mesh::scaled_by(
                                _self.into_inner(),
                                scale.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Scales the vertex positions, normals, and tangents of the mesh by the given [`Vec3`].\n `Aabb` of entities with modified mesh are not updated automatically.",
            &["_self", "scale"],
        )
        .register_documented(
            "set_morph_target_names",
            |
                mut _self: Mut<::bevy_mesh::Mesh>,
                names: ::std::vec::Vec<::std::string::String>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_mesh::Mesh::set_morph_target_names(
                                &mut _self,
                                names,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the names of each morph target. This should correspond to the order of the morph targets in `set_morph_targets`.",
            &["_self", "names"],
        )
        .register_documented(
            "transform_by",
            |
                mut _self: Mut<::bevy_mesh::Mesh>,
                transform: Val<::bevy_transform::components::Transform>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_mesh::Mesh::transform_by(
                                &mut _self,
                                transform.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Transforms the vertex positions, normals, and tangents of the mesh in place by the given [`Transform`].\n `Aabb` of entities with modified mesh are not updated automatically.",
            &["_self", "transform"],
        )
        .register_documented(
            "transformed_by",
            |
                _self: Val<::bevy_mesh::Mesh>,
                transform: Val<::bevy_transform::components::Transform>|
            {
                let output: Val<::bevy_mesh::Mesh> = {
                    {
                        let output: Val<::bevy_mesh::Mesh> = ::bevy_mesh::Mesh::transformed_by(
                                _self.into_inner(),
                                transform.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Transforms the vertex positions, normals, and tangents of the mesh by the given [`Transform`].\n `Aabb` of entities with modified mesh are not updated automatically.",
            &["_self", "transform"],
        )
        .register_documented(
            "translate_by",
            |mut _self: Mut<::bevy_mesh::Mesh>, translation: Val<::bevy_math::Vec3>| {
                let output: () = {
                    {
                        let output: () = ::bevy_mesh::Mesh::translate_by(
                                &mut _self,
                                translation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Translates the vertex positions of the mesh in place by the given [`Vec3`].\n `Aabb` of entities with modified mesh are not updated automatically.",
            &["_self", "translation"],
        )
        .register_documented(
            "translated_by",
            |_self: Val<::bevy_mesh::Mesh>, translation: Val<::bevy_math::Vec3>| {
                let output: Val<::bevy_mesh::Mesh> = {
                    {
                        let output: Val<::bevy_mesh::Mesh> = ::bevy_mesh::Mesh::translated_by(
                                _self.into_inner(),
                                translation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Translates the vertex positions of the mesh by the given [`Vec3`].\n `Aabb` of entities with modified mesh are not updated automatically.",
            &["_self", "translation"],
        )
        .register_documented(
            "with_computed_flat_normals",
            |_self: Val<::bevy_mesh::Mesh>| {
                let output: Val<::bevy_mesh::Mesh> = {
                    {
                        let output: Val<::bevy_mesh::Mesh> = ::bevy_mesh::Mesh::with_computed_flat_normals(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Consumes the mesh and returns a mesh with calculated [`Mesh::ATTRIBUTE_NORMAL`].\n (Alternatively, you can use [`Mesh::compute_flat_normals`] to mutate an existing mesh in-place)\n # Panics\n Panics if [`Mesh::ATTRIBUTE_POSITION`] is not of type `float3`.\n Panics if the mesh has any other topology than [`PrimitiveTopology::TriangleList`].\n Panics if the mesh has indices defined",
            &["_self"],
        )
        .register_documented(
            "with_computed_normals",
            |_self: Val<::bevy_mesh::Mesh>| {
                let output: Val<::bevy_mesh::Mesh> = {
                    {
                        let output: Val<::bevy_mesh::Mesh> = ::bevy_mesh::Mesh::with_computed_normals(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Consumes the mesh and returns a mesh with calculated [`Mesh::ATTRIBUTE_NORMAL`].\n If the mesh is indexed, this defaults to smooth normals. Otherwise, it defaults to flat\n normals.\n (Alternatively, you can use [`Mesh::compute_normals`] to mutate an existing mesh in-place)\n # Panics\n Panics if [`Mesh::ATTRIBUTE_POSITION`] is not of type `float3`.\n Panics if the mesh has any other topology than [`PrimitiveTopology::TriangleList`].",
            &["_self"],
        )
        .register_documented(
            "with_computed_smooth_normals",
            |_self: Val<::bevy_mesh::Mesh>| {
                let output: Val<::bevy_mesh::Mesh> = {
                    {
                        let output: Val<::bevy_mesh::Mesh> = ::bevy_mesh::Mesh::with_computed_smooth_normals(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Consumes the mesh and returns a mesh with calculated [`Mesh::ATTRIBUTE_NORMAL`].\n (Alternatively, you can use [`Mesh::compute_smooth_normals`] to mutate an existing mesh in-place)\n # Panics\n Panics if [`Mesh::ATTRIBUTE_POSITION`] is not of type `float3`.\n Panics if the mesh has any other topology than [`PrimitiveTopology::TriangleList`].\n Panics if the mesh does not have indices defined.",
            &["_self"],
        )
        .register_documented(
            "with_duplicated_vertices",
            |_self: Val<::bevy_mesh::Mesh>| {
                let output: Val<::bevy_mesh::Mesh> = {
                    {
                        let output: Val<::bevy_mesh::Mesh> = ::bevy_mesh::Mesh::with_duplicated_vertices(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Consumes the mesh and returns a mesh with no shared vertices.\n This can dramatically increase the vertex count, so make sure this is what you want.\n Does nothing if no [`Indices`] are set.\n (Alternatively, you can use [`Mesh::duplicate_vertices`] to mutate an existing mesh in-place)",
            &["_self"],
        )
        .register_documented(
            "with_inserted_indices",
            |_self: Val<::bevy_mesh::Mesh>, indices: Val<::bevy_mesh::Indices>| {
                let output: Val<::bevy_mesh::Mesh> = {
                    {
                        let output: Val<::bevy_mesh::Mesh> = ::bevy_mesh::Mesh::with_inserted_indices(
                                _self.into_inner(),
                                indices.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Consumes the mesh and returns a mesh with the given vertex indices. They describe how triangles\n are constructed out of the vertex attributes and are therefore only useful for the\n [`PrimitiveTopology`] variants that use triangles.\n (Alternatively, you can use [`Mesh::insert_indices`] to mutate an existing mesh in-place)",
            &["_self", "indices"],
        )
        .register_documented(
            "with_morph_target_names",
            |
                _self: Val<::bevy_mesh::Mesh>,
                names: ::std::vec::Vec<::std::string::String>|
            {
                let output: Val<::bevy_mesh::Mesh> = {
                    {
                        let output: Val<::bevy_mesh::Mesh> = ::bevy_mesh::Mesh::with_morph_target_names(
                                _self.into_inner(),
                                names,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Consumes the mesh and returns a mesh with morph target names.\n Names should correspond to the order of the morph targets in `set_morph_targets`.\n (Alternatively, you can use [`Mesh::set_morph_target_names`] to mutate an existing mesh in-place)",
            &["_self", "names"],
        )
        .register_documented(
            "with_removed_indices",
            |_self: Val<::bevy_mesh::Mesh>| {
                let output: Val<::bevy_mesh::Mesh> = {
                    {
                        let output: Val<::bevy_mesh::Mesh> = ::bevy_mesh::Mesh::with_removed_indices(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Consumes the mesh and returns a mesh without the vertex `indices` of the mesh.\n (Alternatively, you can use [`Mesh::remove_indices`] to mutate an existing mesh in-place)",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::Mesh,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_morph_weights_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::morph::MorphWeights,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_mesh::morph::MorphWeights>| {
            let output: Val<::bevy_mesh::morph::MorphWeights> = {
                {
                    let output: Val<::bevy_mesh::morph::MorphWeights> =
                        <::bevy_mesh::morph::MorphWeights as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_mesh::morph::MorphWeights,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_mesh_morph_weights_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::morph::MeshMorphWeights,
    >::new(world)
    .register_documented(
        "clear_weights",
        |mut _self: Mut<::bevy_mesh::morph::MeshMorphWeights>| {
            let output: () = {
                {
                    let output: () =
                        ::bevy_mesh::morph::MeshMorphWeights::clear_weights(&mut _self).into();
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
        |_self: Ref<::bevy_mesh::morph::MeshMorphWeights>| {
            let output: Val<::bevy_mesh::morph::MeshMorphWeights> = {
                {
                    let output: Val<::bevy_mesh::morph::MeshMorphWeights> =
                        <::bevy_mesh::morph::MeshMorphWeights as ::std::clone::Clone>::clone(
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
            ::bevy_mesh::morph::MeshMorphWeights,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_circle_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::CircleMeshBuilder,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_mesh::primitives::CircleMeshBuilder>| {
            let output: Val<::bevy_mesh::primitives::CircleMeshBuilder> = {
                {
                    let output: Val<::bevy_mesh::primitives::CircleMeshBuilder> =
                        <::bevy_mesh::primitives::CircleMeshBuilder as ::std::clone::Clone>::clone(
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
        "new",
        |radius: f32, resolution: u32| {
            let output: Val<::bevy_mesh::primitives::CircleMeshBuilder> = {
                {
                    let output: Val<::bevy_mesh::primitives::CircleMeshBuilder> =
                        ::bevy_mesh::primitives::CircleMeshBuilder::new(radius, resolution).into();
                    output
                }
            };
            output
        },
        " Creates a new [`CircleMeshBuilder`] from a given radius and vertex count.",
        &["radius", "resolution"],
    )
    .register_documented(
        "resolution",
        |_self: Val<::bevy_mesh::primitives::CircleMeshBuilder>, resolution: u32| {
            let output: Val<::bevy_mesh::primitives::CircleMeshBuilder> = {
                {
                    let output: Val<::bevy_mesh::primitives::CircleMeshBuilder> =
                        ::bevy_mesh::primitives::CircleMeshBuilder::resolution(
                            _self.into_inner(),
                            resolution,
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Sets the number of vertices used for the circle mesh.",
        &["_self", "resolution"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::CircleMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_circular_mesh_uv_mode_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::CircularMeshUvMode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::CircularMeshUvMode>| {
                let output: Val<::bevy_mesh::primitives::CircularMeshUvMode> = {
                    {
                        let output: Val<::bevy_mesh::primitives::CircularMeshUvMode> = <::bevy_mesh::primitives::CircularMeshUvMode as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_mesh::primitives::CircularMeshUvMode>,
                other: Ref<::bevy_mesh::primitives::CircularMeshUvMode>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_mesh::primitives::CircularMeshUvMode as ::std::cmp::PartialEq<
                            ::bevy_mesh::primitives::CircularMeshUvMode,
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
            ::bevy_mesh::primitives::CircularMeshUvMode,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_circular_sector_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::CircularSectorMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::CircularSectorMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::CircularSectorMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::CircularSectorMeshBuilder,
                        > = <::bevy_mesh::primitives::CircularSectorMeshBuilder as ::std::clone::Clone>::clone(
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
            "new",
            |sector: Val<::bevy_math::primitives::CircularSector>| {
                let output: Val<::bevy_mesh::primitives::CircularSectorMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::CircularSectorMeshBuilder,
                        > = ::bevy_mesh::primitives::CircularSectorMeshBuilder::new(
                                sector.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`CircularSectorMeshBuilder`] from a given sector",
            &["sector"],
        )
        .register_documented(
            "resolution",
            |
                _self: Val<::bevy_mesh::primitives::CircularSectorMeshBuilder>,
                resolution: u32|
            {
                let output: Val<::bevy_mesh::primitives::CircularSectorMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::CircularSectorMeshBuilder,
                        > = ::bevy_mesh::primitives::CircularSectorMeshBuilder::resolution(
                                _self.into_inner(),
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of vertices used for the sector mesh.",
            &["_self", "resolution"],
        )
        .register_documented(
            "uv_mode",
            |
                _self: Val<::bevy_mesh::primitives::CircularSectorMeshBuilder>,
                uv_mode: Val<::bevy_mesh::primitives::CircularMeshUvMode>|
            {
                let output: Val<::bevy_mesh::primitives::CircularSectorMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::CircularSectorMeshBuilder,
                        > = ::bevy_mesh::primitives::CircularSectorMeshBuilder::uv_mode(
                                _self.into_inner(),
                                uv_mode.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the uv mode used for the sector mesh",
            &["_self", "uv_mode"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::CircularSectorMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_circular_segment_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::CircularSegmentMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::CircularSegmentMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::CircularSegmentMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::CircularSegmentMeshBuilder,
                        > = <::bevy_mesh::primitives::CircularSegmentMeshBuilder as ::std::clone::Clone>::clone(
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
            "new",
            |segment: Val<::bevy_math::primitives::CircularSegment>| {
                let output: Val<::bevy_mesh::primitives::CircularSegmentMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::CircularSegmentMeshBuilder,
                        > = ::bevy_mesh::primitives::CircularSegmentMeshBuilder::new(
                                segment.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`CircularSegmentMeshBuilder`] from a given segment",
            &["segment"],
        )
        .register_documented(
            "resolution",
            |
                _self: Val<::bevy_mesh::primitives::CircularSegmentMeshBuilder>,
                resolution: u32|
            {
                let output: Val<::bevy_mesh::primitives::CircularSegmentMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::CircularSegmentMeshBuilder,
                        > = ::bevy_mesh::primitives::CircularSegmentMeshBuilder::resolution(
                                _self.into_inner(),
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of vertices used for the segment mesh.",
            &["_self", "resolution"],
        )
        .register_documented(
            "uv_mode",
            |
                _self: Val<::bevy_mesh::primitives::CircularSegmentMeshBuilder>,
                uv_mode: Val<::bevy_mesh::primitives::CircularMeshUvMode>|
            {
                let output: Val<::bevy_mesh::primitives::CircularSegmentMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::CircularSegmentMeshBuilder,
                        > = ::bevy_mesh::primitives::CircularSegmentMeshBuilder::uv_mode(
                                _self.into_inner(),
                                uv_mode.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the uv mode used for the segment mesh",
            &["_self", "uv_mode"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::CircularSegmentMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_regular_polygon_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::RegularPolygonMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::RegularPolygonMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::RegularPolygonMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::RegularPolygonMeshBuilder,
                        > = <::bevy_mesh::primitives::RegularPolygonMeshBuilder as ::std::clone::Clone>::clone(
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
            "new",
            |circumradius: f32, sides: u32| {
                let output: Val<::bevy_mesh::primitives::RegularPolygonMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::RegularPolygonMeshBuilder,
                        > = ::bevy_mesh::primitives::RegularPolygonMeshBuilder::new(
                                circumradius,
                                sides,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`RegularPolygonMeshBuilder`] from the radius of a circumcircle and a number\n of sides.\n # Panics\n Panics in debug mode if `circumradius` is negative, or if `sides` is less than 3.",
            &["circumradius", "sides"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::RegularPolygonMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ellipse_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::EllipseMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::EllipseMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::EllipseMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::EllipseMeshBuilder> = <::bevy_mesh::primitives::EllipseMeshBuilder as ::std::clone::Clone>::clone(
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
            "new",
            |half_width: f32, half_height: f32, resolution: u32| {
                let output: Val<::bevy_mesh::primitives::EllipseMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::EllipseMeshBuilder> = ::bevy_mesh::primitives::EllipseMeshBuilder::new(
                                half_width,
                                half_height,
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`EllipseMeshBuilder`] from a given half width and half height and a vertex count.",
            &["half_width", "half_height", "resolution"],
        )
        .register_documented(
            "resolution",
            |_self: Val<::bevy_mesh::primitives::EllipseMeshBuilder>, resolution: u32| {
                let output: Val<::bevy_mesh::primitives::EllipseMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::EllipseMeshBuilder> = ::bevy_mesh::primitives::EllipseMeshBuilder::resolution(
                                _self.into_inner(),
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of vertices used for the ellipse mesh.",
            &["_self", "resolution"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::EllipseMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_annulus_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::AnnulusMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::AnnulusMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::AnnulusMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::AnnulusMeshBuilder> = <::bevy_mesh::primitives::AnnulusMeshBuilder as ::std::clone::Clone>::clone(
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
            "new",
            |inner_radius: f32, outer_radius: f32, resolution: u32| {
                let output: Val<::bevy_mesh::primitives::AnnulusMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::AnnulusMeshBuilder> = ::bevy_mesh::primitives::AnnulusMeshBuilder::new(
                                inner_radius,
                                outer_radius,
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create an [`AnnulusMeshBuilder`] with the given inner radius, outer radius, and angular vertex count.",
            &["inner_radius", "outer_radius", "resolution"],
        )
        .register_documented(
            "resolution",
            |_self: Val<::bevy_mesh::primitives::AnnulusMeshBuilder>, resolution: u32| {
                let output: Val<::bevy_mesh::primitives::AnnulusMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::AnnulusMeshBuilder> = ::bevy_mesh::primitives::AnnulusMeshBuilder::resolution(
                                _self.into_inner(),
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of vertices used in constructing the concentric circles of the annulus mesh.",
            &["_self", "resolution"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::AnnulusMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_rhombus_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::RhombusMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::RhombusMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::RhombusMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::RhombusMeshBuilder> = <::bevy_mesh::primitives::RhombusMeshBuilder as ::std::clone::Clone>::clone(
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
            "new",
            |horizontal_diagonal: f32, vertical_diagonal: f32| {
                let output: Val<::bevy_mesh::primitives::RhombusMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::RhombusMeshBuilder> = ::bevy_mesh::primitives::RhombusMeshBuilder::new(
                                horizontal_diagonal,
                                vertical_diagonal,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`RhombusMeshBuilder`] from a horizontal and vertical diagonal size.\n # Panics\n Panics in debug mode if `horizontal_diagonal` or `vertical_diagonal` is negative.",
            &["horizontal_diagonal", "vertical_diagonal"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::RhombusMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_triangle_2_d_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::Triangle2dMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::Triangle2dMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::Triangle2dMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::Triangle2dMeshBuilder,
                        > = <::bevy_mesh::primitives::Triangle2dMeshBuilder as ::std::clone::Clone>::clone(
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
            "new",
            |
                a: Val<::bevy_math::Vec2>,
                b: Val<::bevy_math::Vec2>,
                c: Val<::bevy_math::Vec2>|
            {
                let output: Val<::bevy_mesh::primitives::Triangle2dMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::Triangle2dMeshBuilder,
                        > = ::bevy_mesh::primitives::Triangle2dMeshBuilder::new(
                                a.into_inner(),
                                b.into_inner(),
                                c.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Triangle2dMeshBuilder`] from the points `a`, `b`, and `c`.",
            &["a", "b", "c"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::Triangle2dMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_rectangle_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::RectangleMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::RectangleMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::RectangleMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::RectangleMeshBuilder> = <::bevy_mesh::primitives::RectangleMeshBuilder as ::std::clone::Clone>::clone(
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
            "new",
            |width: f32, height: f32| {
                let output: Val<::bevy_mesh::primitives::RectangleMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::RectangleMeshBuilder> = ::bevy_mesh::primitives::RectangleMeshBuilder::new(
                                width,
                                height,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`RectangleMeshBuilder`] from a full width and height.\n # Panics\n Panics in debug mode if `width` or `height` is negative.",
            &["width", "height"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::RectangleMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_capsule_2_d_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::Capsule2dMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::Capsule2dMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::Capsule2dMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::Capsule2dMeshBuilder> = <::bevy_mesh::primitives::Capsule2dMeshBuilder as ::std::clone::Clone>::clone(
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
            "new",
            |radius: f32, length: f32, resolution: u32| {
                let output: Val<::bevy_mesh::primitives::Capsule2dMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::Capsule2dMeshBuilder> = ::bevy_mesh::primitives::Capsule2dMeshBuilder::new(
                                radius,
                                length,
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Capsule2dMeshBuilder`] from a given radius, length, and the number of vertices\n used for one hemicircle. The total number of vertices for the capsule mesh will be two times the resolution.",
            &["radius", "length", "resolution"],
        )
        .register_documented(
            "resolution",
            |_self: Val<::bevy_mesh::primitives::Capsule2dMeshBuilder>, resolution: u32| {
                let output: Val<::bevy_mesh::primitives::Capsule2dMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::Capsule2dMeshBuilder> = ::bevy_mesh::primitives::Capsule2dMeshBuilder::resolution(
                                _self.into_inner(),
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of vertices used for one hemicircle.\n The total number of vertices for the capsule mesh will be two times the resolution.",
            &["_self", "resolution"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::Capsule2dMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_capsule_uv_profile_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::CapsuleUvProfile,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_mesh::primitives::CapsuleUvProfile>| {
            let output: Val<::bevy_mesh::primitives::CapsuleUvProfile> = {
                {
                    let output: Val<::bevy_mesh::primitives::CapsuleUvProfile> =
                        <::bevy_mesh::primitives::CapsuleUvProfile as ::std::clone::Clone>::clone(
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
            ::bevy_mesh::primitives::CapsuleUvProfile,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_capsule_3_d_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::Capsule3dMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::Capsule3dMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder> = <::bevy_mesh::primitives::Capsule3dMeshBuilder as ::std::clone::Clone>::clone(
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
            "latitudes",
            |_self: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder>, latitudes: u32| {
                let output: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder> = ::bevy_mesh::primitives::Capsule3dMeshBuilder::latitudes(
                                _self.into_inner(),
                                latitudes,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of horizontal lines subdividing the hemispheres of the capsule.",
            &["_self", "latitudes"],
        )
        .register_documented(
            "longitudes",
            |_self: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder>, longitudes: u32| {
                let output: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder> = ::bevy_mesh::primitives::Capsule3dMeshBuilder::longitudes(
                                _self.into_inner(),
                                longitudes,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of vertical lines subdividing the hemispheres of the capsule.",
            &["_self", "longitudes"],
        )
        .register_documented(
            "new",
            |radius: f32, height: f32, longitudes: u32, latitudes: u32| {
                let output: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder> = ::bevy_mesh::primitives::Capsule3dMeshBuilder::new(
                                radius,
                                height,
                                longitudes,
                                latitudes,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Capsule3dMeshBuilder`] from a given radius, height, longitudes, and latitudes.\n Note that `height` is the distance between the centers of the hemispheres.\n `radius` will be added to both ends to get the real height of the mesh.",
            &["radius", "height", "longitudes", "latitudes"],
        )
        .register_documented(
            "rings",
            |_self: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder>, rings: u32| {
                let output: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder> = ::bevy_mesh::primitives::Capsule3dMeshBuilder::rings(
                                _self.into_inner(),
                                rings,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of horizontal lines subdividing the cylindrical part of the capsule.",
            &["_self", "rings"],
        )
        .register_documented(
            "uv_profile",
            |
                _self: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder>,
                uv_profile: Val<::bevy_mesh::primitives::CapsuleUvProfile>|
            {
                let output: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::Capsule3dMeshBuilder> = ::bevy_mesh::primitives::Capsule3dMeshBuilder::uv_profile(
                                _self.into_inner(),
                                uv_profile.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the manner in which UV coordinates are distributed vertically.",
            &["_self", "uv_profile"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::Capsule3dMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cone_anchor_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::ConeAnchor,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_mesh::primitives::ConeAnchor>| {
            let output: Val<::bevy_mesh::primitives::ConeAnchor> = {
                {
                    let output: Val<::bevy_mesh::primitives::ConeAnchor> =
                        <::bevy_mesh::primitives::ConeAnchor as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_mesh::primitives::ConeAnchor,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cone_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::ConeMeshBuilder,
    >::new(world)
        .register_documented(
            "anchor",
            |
                _self: Val<::bevy_mesh::primitives::ConeMeshBuilder>,
                anchor: Val<::bevy_mesh::primitives::ConeAnchor>|
            {
                let output: Val<::bevy_mesh::primitives::ConeMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::ConeMeshBuilder> = ::bevy_mesh::primitives::ConeMeshBuilder::anchor(
                                _self.into_inner(),
                                anchor.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets a custom anchor point for the mesh",
            &["_self", "anchor"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::ConeMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::ConeMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::ConeMeshBuilder> = <::bevy_mesh::primitives::ConeMeshBuilder as ::std::clone::Clone>::clone(
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
            "new",
            |radius: f32, height: f32, resolution: u32| {
                let output: Val<::bevy_mesh::primitives::ConeMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::ConeMeshBuilder> = ::bevy_mesh::primitives::ConeMeshBuilder::new(
                                radius,
                                height,
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`ConeMeshBuilder`] from a given radius, height,\n and number of vertices used for the base of the cone.",
            &["radius", "height", "resolution"],
        )
        .register_documented(
            "resolution",
            |_self: Val<::bevy_mesh::primitives::ConeMeshBuilder>, resolution: u32| {
                let output: Val<::bevy_mesh::primitives::ConeMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::ConeMeshBuilder> = ::bevy_mesh::primitives::ConeMeshBuilder::resolution(
                                _self.into_inner(),
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of vertices used for the base of the cone.",
            &["_self", "resolution"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::ConeMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_conical_frustum_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::ConicalFrustumMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::ConicalFrustumMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::ConicalFrustumMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::ConicalFrustumMeshBuilder,
                        > = <::bevy_mesh::primitives::ConicalFrustumMeshBuilder as ::std::clone::Clone>::clone(
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
            "new",
            |radius_top: f32, radius_bottom: f32, height: f32, resolution: u32| {
                let output: Val<::bevy_mesh::primitives::ConicalFrustumMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::ConicalFrustumMeshBuilder,
                        > = ::bevy_mesh::primitives::ConicalFrustumMeshBuilder::new(
                                radius_top,
                                radius_bottom,
                                height,
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`ConicalFrustumMeshBuilder`] from the given top and bottom radii, a height,\n and a resolution used for the top and bottom.",
            &["radius_top", "radius_bottom", "height", "resolution"],
        )
        .register_documented(
            "resolution",
            |
                _self: Val<::bevy_mesh::primitives::ConicalFrustumMeshBuilder>,
                resolution: u32|
            {
                let output: Val<::bevy_mesh::primitives::ConicalFrustumMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::ConicalFrustumMeshBuilder,
                        > = ::bevy_mesh::primitives::ConicalFrustumMeshBuilder::resolution(
                                _self.into_inner(),
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of vertices used for the top and bottom of the conical frustum.",
            &["_self", "resolution"],
        )
        .register_documented(
            "segments",
            |
                _self: Val<::bevy_mesh::primitives::ConicalFrustumMeshBuilder>,
                segments: u32|
            {
                let output: Val<::bevy_mesh::primitives::ConicalFrustumMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::ConicalFrustumMeshBuilder,
                        > = ::bevy_mesh::primitives::ConicalFrustumMeshBuilder::segments(
                                _self.into_inner(),
                                segments,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of horizontal lines subdividing the lateral surface of the conical frustum.",
            &["_self", "segments"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::ConicalFrustumMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cuboid_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::CuboidMeshBuilder,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_mesh::primitives::CuboidMeshBuilder>| {
            let output: Val<::bevy_mesh::primitives::CuboidMeshBuilder> = {
                {
                    let output: Val<::bevy_mesh::primitives::CuboidMeshBuilder> =
                        <::bevy_mesh::primitives::CuboidMeshBuilder as ::std::clone::Clone>::clone(
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
            ::bevy_mesh::primitives::CuboidMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cylinder_anchor_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::CylinderAnchor,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_mesh::primitives::CylinderAnchor>| {
            let output: Val<::bevy_mesh::primitives::CylinderAnchor> = {
                {
                    let output: Val<::bevy_mesh::primitives::CylinderAnchor> =
                        <::bevy_mesh::primitives::CylinderAnchor as ::std::clone::Clone>::clone(
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
            ::bevy_mesh::primitives::CylinderAnchor,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cylinder_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::CylinderMeshBuilder,
    >::new(world)
        .register_documented(
            "anchor",
            |
                _self: Val<::bevy_mesh::primitives::CylinderMeshBuilder>,
                anchor: Val<::bevy_mesh::primitives::CylinderAnchor>|
            {
                let output: Val<::bevy_mesh::primitives::CylinderMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::CylinderMeshBuilder> = ::bevy_mesh::primitives::CylinderMeshBuilder::anchor(
                                _self.into_inner(),
                                anchor.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets a custom anchor point for the mesh",
            &["_self", "anchor"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::CylinderMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::CylinderMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::CylinderMeshBuilder> = <::bevy_mesh::primitives::CylinderMeshBuilder as ::std::clone::Clone>::clone(
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
            "new",
            |radius: f32, height: f32, resolution: u32| {
                let output: Val<::bevy_mesh::primitives::CylinderMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::CylinderMeshBuilder> = ::bevy_mesh::primitives::CylinderMeshBuilder::new(
                                radius,
                                height,
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`CylinderMeshBuilder`] from the given radius, a height,\n and a resolution used for the top and bottom.",
            &["radius", "height", "resolution"],
        )
        .register_documented(
            "resolution",
            |_self: Val<::bevy_mesh::primitives::CylinderMeshBuilder>, resolution: u32| {
                let output: Val<::bevy_mesh::primitives::CylinderMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::CylinderMeshBuilder> = ::bevy_mesh::primitives::CylinderMeshBuilder::resolution(
                                _self.into_inner(),
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of vertices used for the top and bottom of the cylinder.",
            &["_self", "resolution"],
        )
        .register_documented(
            "segments",
            |_self: Val<::bevy_mesh::primitives::CylinderMeshBuilder>, segments: u32| {
                let output: Val<::bevy_mesh::primitives::CylinderMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::CylinderMeshBuilder> = ::bevy_mesh::primitives::CylinderMeshBuilder::segments(
                                _self.into_inner(),
                                segments,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of segments along the height of the cylinder.\n Must be greater than `0` for geometry to be generated.",
            &["_self", "segments"],
        )
        .register_documented(
            "without_caps",
            |_self: Val<::bevy_mesh::primitives::CylinderMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::CylinderMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::CylinderMeshBuilder> = ::bevy_mesh::primitives::CylinderMeshBuilder::without_caps(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Ignore the cylinder caps, making the mesh a shallow tube instead",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::CylinderMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_plane_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::PlaneMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::PlaneMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = <::bevy_mesh::primitives::PlaneMeshBuilder as ::std::clone::Clone>::clone(
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
            "from_length",
            |length: f32| {
                let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = ::bevy_mesh::primitives::PlaneMeshBuilder::from_length(
                                length,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`PlaneMeshBuilder`] from the given length, with the normal pointing upwards,\n and the resulting [`PlaneMeshBuilder`] being a square.",
            &["length"],
        )
        .register_documented(
            "from_size",
            |size: Val<::bevy_math::Vec2>| {
                let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = ::bevy_mesh::primitives::PlaneMeshBuilder::from_size(
                                size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`PlaneMeshBuilder`] from the given size, with the normal pointing upwards.",
            &["size"],
        )
        .register_documented(
            "new",
            |normal: Val<::bevy_math::Dir3>, size: Val<::bevy_math::Vec2>| {
                let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = ::bevy_mesh::primitives::PlaneMeshBuilder::new(
                                normal.into_inner(),
                                size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`PlaneMeshBuilder`] from a given normal and size.",
            &["normal", "size"],
        )
        .register_documented(
            "normal",
            |
                _self: Val<::bevy_mesh::primitives::PlaneMeshBuilder>,
                normal: Val<::bevy_math::Dir3>|
            {
                let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = ::bevy_mesh::primitives::PlaneMeshBuilder::normal(
                                _self.into_inner(),
                                normal.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the normal of the plane, aka the direction the plane is facing.",
            &["_self", "normal"],
        )
        .register_documented(
            "size",
            |
                _self: Val<::bevy_mesh::primitives::PlaneMeshBuilder>,
                width: f32,
                height: f32|
            {
                let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = ::bevy_mesh::primitives::PlaneMeshBuilder::size(
                                _self.into_inner(),
                                width,
                                height,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the size of the plane mesh.",
            &["_self", "width", "height"],
        )
        .register_documented(
            "subdivisions",
            |_self: Val<::bevy_mesh::primitives::PlaneMeshBuilder>, subdivisions: u32| {
                let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::PlaneMeshBuilder> = ::bevy_mesh::primitives::PlaneMeshBuilder::subdivisions(
                                _self.into_inner(),
                                subdivisions,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the subdivisions of the plane mesh.\n 0 - is the original plane geometry, the 4 points in the XZ plane.\n 1 - is split by 1 line in the middle of the plane on both the X axis and the Z axis,\n     resulting in a plane with 4 quads / 8 triangles.\n 2 - is a plane split by 2 lines on both the X and Z axes, subdividing the plane into 3\n     equal sections along each axis, resulting in a plane with 9 quads / 18 triangles.",
            &["_self", "subdivisions"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::PlaneMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sphere_kind_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::SphereKind,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_mesh::primitives::SphereKind>| {
            let output: Val<::bevy_mesh::primitives::SphereKind> = {
                {
                    let output: Val<::bevy_mesh::primitives::SphereKind> =
                        <::bevy_mesh::primitives::SphereKind as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_mesh::primitives::SphereKind,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sphere_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::SphereMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::SphereMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::SphereMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::SphereMeshBuilder> = <::bevy_mesh::primitives::SphereMeshBuilder as ::std::clone::Clone>::clone(
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
            "kind",
            |
                _self: Val<::bevy_mesh::primitives::SphereMeshBuilder>,
                kind: Val<::bevy_mesh::primitives::SphereKind>|
            {
                let output: Val<::bevy_mesh::primitives::SphereMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::SphereMeshBuilder> = ::bevy_mesh::primitives::SphereMeshBuilder::kind(
                                _self.into_inner(),
                                kind.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the [`SphereKind`] that will be used for building the mesh.",
            &["_self", "kind"],
        )
        .register_documented(
            "new",
            |radius: f32, kind: Val<::bevy_mesh::primitives::SphereKind>| {
                let output: Val<::bevy_mesh::primitives::SphereMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::SphereMeshBuilder> = ::bevy_mesh::primitives::SphereMeshBuilder::new(
                                radius,
                                kind.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`SphereMeshBuilder`] from a radius and [`SphereKind`].",
            &["radius", "kind"],
        )
        .register_documented(
            "uv",
            |
                _self: Ref<::bevy_mesh::primitives::SphereMeshBuilder>,
                sectors: u32,
                stacks: u32|
            {
                let output: Val<::bevy_mesh::Mesh> = {
                    {
                        let output: Val<::bevy_mesh::Mesh> = ::bevy_mesh::primitives::SphereMeshBuilder::uv(
                                &_self,
                                sectors,
                                stacks,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a UV sphere [`Mesh`] with the given number of\n longitudinal sectors and latitudinal stacks, aka horizontal and vertical resolution.\n A good default is `32` sectors and `18` stacks.",
            &["_self", "sectors", "stacks"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::SphereMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_tetrahedron_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::TetrahedronMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::TetrahedronMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::TetrahedronMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::TetrahedronMeshBuilder,
                        > = <::bevy_mesh::primitives::TetrahedronMeshBuilder as ::std::clone::Clone>::clone(
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
            ::bevy_mesh::primitives::TetrahedronMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_torus_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::TorusMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::TorusMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::TorusMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::TorusMeshBuilder> = <::bevy_mesh::primitives::TorusMeshBuilder as ::std::clone::Clone>::clone(
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
            "major_resolution",
            |_self: Val<::bevy_mesh::primitives::TorusMeshBuilder>, resolution: usize| {
                let output: Val<::bevy_mesh::primitives::TorusMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::TorusMeshBuilder> = ::bevy_mesh::primitives::TorusMeshBuilder::major_resolution(
                                _self.into_inner(),
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of segments used for the main ring of the torus.\n A resolution of `4` would make the torus appear rectangular,\n while a resolution of `32` resembles a circular ring.",
            &["_self", "resolution"],
        )
        .register_documented(
            "minor_resolution",
            |_self: Val<::bevy_mesh::primitives::TorusMeshBuilder>, resolution: usize| {
                let output: Val<::bevy_mesh::primitives::TorusMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::TorusMeshBuilder> = ::bevy_mesh::primitives::TorusMeshBuilder::minor_resolution(
                                _self.into_inner(),
                                resolution,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the number of vertices used for each circular segment\n in the ring or tube of the torus.",
            &["_self", "resolution"],
        )
        .register_documented(
            "new",
            |inner_radius: f32, outer_radius: f32| {
                let output: Val<::bevy_mesh::primitives::TorusMeshBuilder> = {
                    {
                        let output: Val<::bevy_mesh::primitives::TorusMeshBuilder> = ::bevy_mesh::primitives::TorusMeshBuilder::new(
                                inner_radius,
                                outer_radius,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`TorusMeshBuilder`] from an inner and outer radius.\n The inner radius is the radius of the hole, and the outer radius\n is the radius of the entire object.",
            &["inner_radius", "outer_radius"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_mesh::primitives::TorusMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_triangle_3_d_mesh_builder_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::primitives::Triangle3dMeshBuilder,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_mesh::primitives::Triangle3dMeshBuilder>| {
                let output: Val<::bevy_mesh::primitives::Triangle3dMeshBuilder> = {
                    {
                        let output: Val<
                            ::bevy_mesh::primitives::Triangle3dMeshBuilder,
                        > = <::bevy_mesh::primitives::Triangle3dMeshBuilder as ::std::clone::Clone>::clone(
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
            ::bevy_mesh::primitives::Triangle3dMeshBuilder,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_skinned_mesh_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_mesh::skinning::SkinnedMesh,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_mesh::skinning::SkinnedMesh>| {
            let output: Val<::bevy_mesh::skinning::SkinnedMesh> = {
                {
                    let output: Val<::bevy_mesh::skinning::SkinnedMesh> =
                        <::bevy_mesh::skinning::SkinnedMesh as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_mesh::skinning::SkinnedMesh,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyMeshScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_indices_functions(&mut world);
        register_mesh_functions(&mut world);
        register_morph_weights_functions(&mut world);
        register_mesh_morph_weights_functions(&mut world);
        register_circle_mesh_builder_functions(&mut world);
        register_circular_mesh_uv_mode_functions(&mut world);
        register_circular_sector_mesh_builder_functions(&mut world);
        register_circular_segment_mesh_builder_functions(&mut world);
        register_regular_polygon_mesh_builder_functions(&mut world);
        register_ellipse_mesh_builder_functions(&mut world);
        register_annulus_mesh_builder_functions(&mut world);
        register_rhombus_mesh_builder_functions(&mut world);
        register_triangle_2_d_mesh_builder_functions(&mut world);
        register_rectangle_mesh_builder_functions(&mut world);
        register_capsule_2_d_mesh_builder_functions(&mut world);
        register_capsule_uv_profile_functions(&mut world);
        register_capsule_3_d_mesh_builder_functions(&mut world);
        register_cone_anchor_functions(&mut world);
        register_cone_mesh_builder_functions(&mut world);
        register_conical_frustum_mesh_builder_functions(&mut world);
        register_cuboid_mesh_builder_functions(&mut world);
        register_cylinder_anchor_functions(&mut world);
        register_cylinder_mesh_builder_functions(&mut world);
        register_plane_mesh_builder_functions(&mut world);
        register_sphere_kind_functions(&mut world);
        register_sphere_mesh_builder_functions(&mut world);
        register_tetrahedron_mesh_builder_functions(&mut world);
        register_torus_mesh_builder_functions(&mut world);
        register_triangle_3_d_mesh_builder_functions(&mut world);
        register_skinned_mesh_functions(&mut world);
    }
}
