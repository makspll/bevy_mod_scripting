
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
pub struct BevySpriteRenderScriptingPlugin;
pub(crate) fn register_color_material_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite_render::ColorMaterial,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_sprite_render::ColorMaterial>| {
                let output: Val<::bevy_sprite_render::ColorMaterial> = {
                    {
                        let output: Val<::bevy_sprite_render::ColorMaterial> = <::bevy_sprite_render::ColorMaterial as ::std::clone::Clone>::clone(
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
            ::bevy_sprite_render::ColorMaterial,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_alpha_mode_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite_render::AlphaMode2d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_sprite_render::AlphaMode2d>| {
                let output: Val<::bevy_sprite_render::AlphaMode2d> = {
                    {
                        let output: Val<::bevy_sprite_render::AlphaMode2d> = <::bevy_sprite_render::AlphaMode2d as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_sprite_render::AlphaMode2d>,
                other: Ref<::bevy_sprite_render::AlphaMode2d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite_render::AlphaMode2d as ::std::cmp::PartialEq<
                            ::bevy_sprite_render::AlphaMode2d,
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
            ::bevy_sprite_render::AlphaMode2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_2_d_material_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite_render::Wireframe2dMaterial,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_sprite_render::Wireframe2dMaterial>| {
                let output: Val<::bevy_sprite_render::Wireframe2dMaterial> = {
                    {
                        let output: Val<::bevy_sprite_render::Wireframe2dMaterial> = <::bevy_sprite_render::Wireframe2dMaterial as ::std::clone::Clone>::clone(
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
            ::bevy_sprite_render::Wireframe2dMaterial,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_2_d_config_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite_render::Wireframe2dConfig,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_sprite_render::Wireframe2dConfig>| {
                let output: Val<::bevy_sprite_render::Wireframe2dConfig> = {
                    {
                        let output: Val<::bevy_sprite_render::Wireframe2dConfig> = <::bevy_sprite_render::Wireframe2dConfig as ::std::clone::Clone>::clone(
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
            ::bevy_sprite_render::Wireframe2dConfig,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite_render::Wireframe2d,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_sprite_render::Wireframe2d>| {
                let output: () = {
                    {
                        let output: () = <::bevy_sprite_render::Wireframe2d as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_sprite_render::Wireframe2d>| {
                let output: Val<::bevy_sprite_render::Wireframe2d> = {
                    {
                        let output: Val<::bevy_sprite_render::Wireframe2d> = <::bevy_sprite_render::Wireframe2d as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_sprite_render::Wireframe2d>,
                other: Ref<::bevy_sprite_render::Wireframe2d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite_render::Wireframe2d as ::std::cmp::PartialEq<
                            ::bevy_sprite_render::Wireframe2d,
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
            ::bevy_sprite_render::Wireframe2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_2_d_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite_render::Wireframe2dColor,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_sprite_render::Wireframe2dColor>| {
                let output: Val<::bevy_sprite_render::Wireframe2dColor> = {
                    {
                        let output: Val<::bevy_sprite_render::Wireframe2dColor> = <::bevy_sprite_render::Wireframe2dColor as ::std::clone::Clone>::clone(
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
            ::bevy_sprite_render::Wireframe2dColor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_no_wireframe_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite_render::NoWireframe2d,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_sprite_render::NoWireframe2d>| {
                let output: () = {
                    {
                        let output: () = <::bevy_sprite_render::NoWireframe2d as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_sprite_render::NoWireframe2d>| {
                let output: Val<::bevy_sprite_render::NoWireframe2d> = {
                    {
                        let output: Val<::bevy_sprite_render::NoWireframe2d> = <::bevy_sprite_render::NoWireframe2d as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_sprite_render::NoWireframe2d>,
                other: Ref<::bevy_sprite_render::NoWireframe2d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite_render::NoWireframe2d as ::std::cmp::PartialEq<
                            ::bevy_sprite_render::NoWireframe2d,
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
            ::bevy_sprite_render::NoWireframe2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_mesh_2_d_wireframe_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite_render::Mesh2dWireframe,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_sprite_render::Mesh2dWireframe>| {
                let output: () = {
                    {
                        let output: () = <::bevy_sprite_render::Mesh2dWireframe as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_sprite_render::Mesh2dWireframe>| {
                let output: Val<::bevy_sprite_render::Mesh2dWireframe> = {
                    {
                        let output: Val<::bevy_sprite_render::Mesh2dWireframe> = <::bevy_sprite_render::Mesh2dWireframe as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_sprite_render::Mesh2dWireframe>,
                other: Ref<::bevy_sprite_render::Mesh2dWireframe>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_sprite_render::Mesh2dWireframe as ::std::cmp::PartialEq<
                            ::bevy_sprite_render::Mesh2dWireframe,
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
            ::bevy_sprite_render::Mesh2dWireframe,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_tilemap_chunk_mesh_cache_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite_render::TilemapChunkMeshCache,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_sprite_render::TilemapChunkMeshCache,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_tilemap_chunk_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite_render::TilemapChunk,
    >::new(world)
        .register_documented(
            "calculate_tile_transform",
            |
                _self: Ref<::bevy_sprite_render::TilemapChunk>,
                position: Val<::glam::UVec2>|
            {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = ::bevy_sprite_render::TilemapChunk::calculate_tile_transform(
                                &_self,
                                position.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "position"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_sprite_render::TilemapChunk>| {
                let output: Val<::bevy_sprite_render::TilemapChunk> = {
                    {
                        let output: Val<::bevy_sprite_render::TilemapChunk> = <::bevy_sprite_render::TilemapChunk as ::std::clone::Clone>::clone(
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
            ::bevy_sprite_render::TilemapChunk,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_tile_data_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite_render::TileData,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_sprite_render::TileData>| {
                let output: Val<::bevy_sprite_render::TileData> = {
                    {
                        let output: Val<::bevy_sprite_render::TileData> = <::bevy_sprite_render::TileData as ::std::clone::Clone>::clone(
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
            "from_tileset_index",
            |tileset_index: u16| {
                let output: Val<::bevy_sprite_render::TileData> = {
                    {
                        let output: Val<::bevy_sprite_render::TileData> = ::bevy_sprite_render::TileData::from_tileset_index(
                                tileset_index,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new `TileData` with the given tileset index and default values.",
            &["tileset_index"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_sprite_render::TileData,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_tilemap_chunk_tile_data_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_sprite_render::TilemapChunkTileData,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_sprite_render::TilemapChunkTileData>| {
                let output: Val<::bevy_sprite_render::TilemapChunkTileData> = {
                    {
                        let output: Val<::bevy_sprite_render::TilemapChunkTileData> = <::bevy_sprite_render::TilemapChunkTileData as ::std::clone::Clone>::clone(
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
            ::bevy_sprite_render::TilemapChunkTileData,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevySpriteRenderScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_color_material_functions(&mut world);
        register_alpha_mode_2_d_functions(&mut world);
        register_wireframe_2_d_material_functions(&mut world);
        register_wireframe_2_d_config_functions(&mut world);
        register_wireframe_2_d_functions(&mut world);
        register_wireframe_2_d_color_functions(&mut world);
        register_no_wireframe_2_d_functions(&mut world);
        register_mesh_2_d_wireframe_functions(&mut world);
        register_tilemap_chunk_mesh_cache_functions(&mut world);
        register_tilemap_chunk_functions(&mut world);
        register_tile_data_functions(&mut world);
        register_tilemap_chunk_tile_data_functions(&mut world);
    }
}
