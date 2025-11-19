
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
pub struct BevyRenderScriptingPlugin;
pub(crate) fn register_alpha_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::alpha::AlphaMode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::alpha::AlphaMode>| {
                let output: Val<::bevy_render::alpha::AlphaMode> = {
                    {
                        let output: Val<::bevy_render::alpha::AlphaMode> = <::bevy_render::alpha::AlphaMode as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_render::alpha::AlphaMode>,
                other: Ref<::bevy_render::alpha::AlphaMode>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::alpha::AlphaMode as ::std::cmp::PartialEq<
                            ::bevy_render::alpha::AlphaMode,
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
            ::bevy_render::alpha::AlphaMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_msaa_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::Msaa,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::view::Msaa>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::view::Msaa as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_render::view::Msaa>| {
                let output: Val<::bevy_render::view::Msaa> = {
                    {
                        let output: Val<::bevy_render::view::Msaa> = <::bevy_render::view::Msaa as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_render::view::Msaa>,
                other: Ref<::bevy_render::view::Msaa>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::view::Msaa as ::std::cmp::PartialEq<
                            ::bevy_render::view::Msaa,
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
            "from_samples",
            |samples: u32| {
                let output: Val<::bevy_render::view::Msaa> = {
                    {
                        let output: Val<::bevy_render::view::Msaa> = ::bevy_render::view::Msaa::from_samples(
                                samples,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["samples"],
        )
        .register_documented(
            "samples",
            |_self: Ref<::bevy_render::view::Msaa>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_render::view::Msaa::samples(&_self)
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
            ::bevy_render::view::Msaa,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_main_entity_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::sync_world::MainEntity,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::sync_world::MainEntity>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::sync_world::MainEntity as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_render::sync_world::MainEntity>| {
                let output: Val<::bevy_render::sync_world::MainEntity> = {
                    {
                        let output: Val<::bevy_render::sync_world::MainEntity> = <::bevy_render::sync_world::MainEntity as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_render::sync_world::MainEntity>,
                other: Ref<::bevy_render::sync_world::MainEntity>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::sync_world::MainEntity as ::std::cmp::PartialEq<
                            ::bevy_render::sync_world::MainEntity,
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
            "id",
            |_self: Ref<::bevy_render::sync_world::MainEntity>| {
                let output: Val<::bevy_ecs::entity::Entity> = {
                    {
                        let output: Val<::bevy_ecs::entity::Entity> = ::bevy_render::sync_world::MainEntity::id(
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
            ::bevy_render::sync_world::MainEntity,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_occlusion_culling_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::experimental::occlusion_culling::OcclusionCulling,
    >::new(world)
        .register_documented(
            "clone",
            |
                _self: Ref<
                    ::bevy_render::experimental::occlusion_culling::OcclusionCulling,
                >|
            {
                let output: Val<
                    ::bevy_render::experimental::occlusion_culling::OcclusionCulling,
                > = {
                    {
                        let output: Val<
                            ::bevy_render::experimental::occlusion_culling::OcclusionCulling,
                        > = <::bevy_render::experimental::occlusion_culling::OcclusionCulling as ::std::clone::Clone>::clone(
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
            ::bevy_render::experimental::occlusion_culling::OcclusionCulling,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_entity_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::sync_world::RenderEntity,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::sync_world::RenderEntity>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::sync_world::RenderEntity as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_render::sync_world::RenderEntity>| {
                let output: Val<::bevy_render::sync_world::RenderEntity> = {
                    {
                        let output: Val<::bevy_render::sync_world::RenderEntity> = <::bevy_render::sync_world::RenderEntity as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_render::sync_world::RenderEntity>,
                other: Ref<::bevy_render::sync_world::RenderEntity>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::sync_world::RenderEntity as ::std::cmp::PartialEq<
                            ::bevy_render::sync_world::RenderEntity,
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
            "id",
            |_self: Ref<::bevy_render::sync_world::RenderEntity>| {
                let output: Val<::bevy_ecs::entity::Entity> = {
                    {
                        let output: Val<::bevy_ecs::entity::Entity> = ::bevy_render::sync_world::RenderEntity::id(
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
            ::bevy_render::sync_world::RenderEntity,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sync_to_render_world_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::sync_world::SyncToRenderWorld,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::sync_world::SyncToRenderWorld>| {
                let output: Val<::bevy_render::sync_world::SyncToRenderWorld> = {
                    {
                        let output: Val<::bevy_render::sync_world::SyncToRenderWorld> = <::bevy_render::sync_world::SyncToRenderWorld as ::std::clone::Clone>::clone(
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
            ::bevy_render::sync_world::SyncToRenderWorld,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_color_grading_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::ColorGrading,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::view::ColorGrading>| {
                let output: Val<::bevy_render::view::ColorGrading> = {
                    {
                        let output: Val<::bevy_render::view::ColorGrading> = <::bevy_render::view::ColorGrading as ::std::clone::Clone>::clone(
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
            "with_identical_sections",
            |
                global: Val<::bevy_render::view::ColorGradingGlobal>,
                section: Val<::bevy_render::view::ColorGradingSection>|
            {
                let output: Val<::bevy_render::view::ColorGrading> = {
                    {
                        let output: Val<::bevy_render::view::ColorGrading> = ::bevy_render::view::ColorGrading::with_identical_sections(
                                global.into_inner(),
                                section.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`ColorGrading`] instance in which shadows, midtones, and\n highlights all have the same set of color grading values.",
            &["global", "section"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::ColorGrading,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_hdr_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::Hdr,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_render::view::Hdr>| {
                let output: () = {
                    {
                        let output: () = <::bevy_render::view::Hdr as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_render::view::Hdr>| {
                let output: Val<::bevy_render::view::Hdr> = {
                    {
                        let output: Val<::bevy_render::view::Hdr> = <::bevy_render::view::Hdr as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_render::view::Hdr>, other: Ref<::bevy_render::view::Hdr>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::view::Hdr as ::std::cmp::PartialEq<
                            ::bevy_render::view::Hdr,
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
            ::bevy_render::view::Hdr,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_visible_entities_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::RenderVisibleEntities,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::view::RenderVisibleEntities>| {
                let output: Val<::bevy_render::view::RenderVisibleEntities> = {
                    {
                        let output: Val<::bevy_render::view::RenderVisibleEntities> = <::bevy_render::view::RenderVisibleEntities as ::std::clone::Clone>::clone(
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
            ::bevy_render::view::RenderVisibleEntities,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_render_graph_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::CameraRenderGraph,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::camera::CameraRenderGraph>| {
                let output: Val<::bevy_render::camera::CameraRenderGraph> = {
                    {
                        let output: Val<::bevy_render::camera::CameraRenderGraph> = <::bevy_render::camera::CameraRenderGraph as ::std::clone::Clone>::clone(
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
            ::bevy_render::camera::CameraRenderGraph,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_temporal_jitter_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::TemporalJitter,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::camera::TemporalJitter>| {
                let output: Val<::bevy_render::camera::TemporalJitter> = {
                    {
                        let output: Val<::bevy_render::camera::TemporalJitter> = <::bevy_render::camera::TemporalJitter as ::std::clone::Clone>::clone(
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
            "jitter_projection",
            |
                _self: Ref<::bevy_render::camera::TemporalJitter>,
                mut clip_from_view: Mut<::glam::Mat4>,
                view_size: Val<::glam::Vec2>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_render::camera::TemporalJitter::jitter_projection(
                                &_self,
                                &mut clip_from_view,
                                view_size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "clip_from_view", "view_size"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::camera::TemporalJitter,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_mip_bias_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::MipBias,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::camera::MipBias>| {
                let output: Val<::bevy_render::camera::MipBias> = {
                    {
                        let output: Val<::bevy_render::camera::MipBias> = <::bevy_render::camera::MipBias as ::std::clone::Clone>::clone(
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
            ::bevy_render::camera::MipBias,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_globals_uniform_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::globals::GlobalsUniform,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::globals::GlobalsUniform>| {
                let output: Val<::bevy_render::globals::GlobalsUniform> = {
                    {
                        let output: Val<::bevy_render::globals::GlobalsUniform> = <::bevy_render::globals::GlobalsUniform as ::std::clone::Clone>::clone(
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
            ::bevy_render::globals::GlobalsUniform,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_shader_storage_buffer_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::storage::ShaderStorageBuffer,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::storage::ShaderStorageBuffer>| {
                let output: Val<::bevy_render::storage::ShaderStorageBuffer> = {
                    {
                        let output: Val<::bevy_render::storage::ShaderStorageBuffer> = <::bevy_render::storage::ShaderStorageBuffer as ::std::clone::Clone>::clone(
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
            "with_size",
            |size: usize, asset_usage: Val<::bevy_asset::RenderAssetUsages>| {
                let output: Val<::bevy_render::storage::ShaderStorageBuffer> = {
                    {
                        let output: Val<::bevy_render::storage::ShaderStorageBuffer> = ::bevy_render::storage::ShaderStorageBuffer::with_size(
                                size,
                                asset_usage.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new storage buffer with the given size and asset usage.",
            &["size", "asset_usage"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::storage::ShaderStorageBuffer,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_readback_complete_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::gpu_readback::ReadbackComplete,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::gpu_readback::ReadbackComplete,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_temporary_render_entity_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::sync_world::TemporaryRenderEntity,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::sync_world::TemporaryRenderEntity>| {
                let output: Val<::bevy_render::sync_world::TemporaryRenderEntity> = {
                    {
                        let output: Val<
                            ::bevy_render::sync_world::TemporaryRenderEntity,
                        > = <::bevy_render::sync_world::TemporaryRenderEntity as ::std::clone::Clone>::clone(
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
            ::bevy_render::sync_world::TemporaryRenderEntity,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_color_grading_global_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::ColorGradingGlobal,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::view::ColorGradingGlobal>| {
                let output: Val<::bevy_render::view::ColorGradingGlobal> = {
                    {
                        let output: Val<::bevy_render::view::ColorGradingGlobal> = <::bevy_render::view::ColorGradingGlobal as ::std::clone::Clone>::clone(
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
            ::bevy_render::view::ColorGradingGlobal,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_color_grading_section_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::ColorGradingSection,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_render::view::ColorGradingSection>| {
                let output: Val<::bevy_render::view::ColorGradingSection> = {
                    {
                        let output: Val<::bevy_render::view::ColorGradingSection> = <::bevy_render::view::ColorGradingSection as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_render::view::ColorGradingSection>,
                other: Ref<::bevy_render::view::ColorGradingSection>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::view::ColorGradingSection as ::std::cmp::PartialEq<
                            ::bevy_render::view::ColorGradingSection,
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
            ::bevy_render::view::ColorGradingSection,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_screenshot_captured_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::screenshot::ScreenshotCaptured,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::screenshot::ScreenshotCaptured,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_screenshot_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::screenshot::Screenshot,
    >::new(world)
        .register_documented(
            "primary_window",
            || {
                let output: Val<::bevy_render::view::screenshot::Screenshot> = {
                    {
                        let output: Val<::bevy_render::view::screenshot::Screenshot> = ::bevy_render::view::screenshot::Screenshot::primary_window()
                            .into();
                        output
                    }
                };
                output
            },
            " Capture a screenshot of the primary window, if one exists.",
            &[],
        )
        .register_documented(
            "texture_view",
            |texture_view: Val<::bevy_camera::ManualTextureViewHandle>| {
                let output: Val<::bevy_render::view::screenshot::Screenshot> = {
                    {
                        let output: Val<::bevy_render::view::screenshot::Screenshot> = ::bevy_render::view::screenshot::Screenshot::texture_view(
                                texture_view.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Capture a screenshot of the provided manual texture view.",
            &["texture_view"],
        )
        .register_documented(
            "window",
            |window: Val<::bevy_ecs::entity::Entity>| {
                let output: Val<::bevy_render::view::screenshot::Screenshot> = {
                    {
                        let output: Val<::bevy_render::view::screenshot::Screenshot> = ::bevy_render::view::screenshot::Screenshot::window(
                                window.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Capture a screenshot of the provided window entity.",
            &["window"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::screenshot::Screenshot,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyRenderScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_alpha_mode_functions(&mut world);
        register_msaa_functions(&mut world);
        register_main_entity_functions(&mut world);
        register_occlusion_culling_functions(&mut world);
        register_render_entity_functions(&mut world);
        register_sync_to_render_world_functions(&mut world);
        register_color_grading_functions(&mut world);
        register_hdr_functions(&mut world);
        register_render_visible_entities_functions(&mut world);
        register_camera_render_graph_functions(&mut world);
        register_temporal_jitter_functions(&mut world);
        register_mip_bias_functions(&mut world);
        register_globals_uniform_functions(&mut world);
        register_shader_storage_buffer_functions(&mut world);
        register_readback_complete_functions(&mut world);
        register_temporary_render_entity_functions(&mut world);
        register_color_grading_global_functions(&mut world);
        register_color_grading_section_functions(&mut world);
        register_screenshot_captured_functions(&mut world);
        register_screenshot_functions(&mut world);
    }
}
