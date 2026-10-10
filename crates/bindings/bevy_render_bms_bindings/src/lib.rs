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
pub struct BevyRenderScriptingPlugin;
pub(crate) fn register_msaa_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::Msaa,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_render::view::Msaa>| {
                let output: V<::bevy_render::view::Msaa> = {
                    {
                        let output: V<::bevy_render::view::Msaa> = <::bevy_render::view::Msaa as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_render::view::Msaa>, other: R<::bevy_render::view::Msaa>| {
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
                let output: V<::bevy_render::view::Msaa> = {
                    {
                        let output: V<::bevy_render::view::Msaa> = ::bevy_render::view::Msaa::from_samples(
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
            |_self: R<::bevy_render::view::Msaa>| {
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
pub(crate) fn register_occlusion_culling_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::occlusion_culling::OcclusionCulling,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_render::occlusion_culling::OcclusionCulling>| {
                let output: V<::bevy_render::occlusion_culling::OcclusionCulling> = {
                    {
                        let output: V<
                            ::bevy_render::occlusion_culling::OcclusionCulling,
                        > = <::bevy_render::occlusion_culling::OcclusionCulling as ::std::clone::Clone>::clone(
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
            ::bevy_render::occlusion_culling::OcclusionCulling,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_color_grading_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::ColorGrading,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_render::view::ColorGrading>| {
                let output: V<::bevy_render::view::ColorGrading> = {
                    {
                        let output: V<::bevy_render::view::ColorGrading> = <::bevy_render::view::ColorGrading as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_render::view::ColorGrading>,
                other: R<::bevy_render::view::ColorGrading>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::view::ColorGrading as ::std::cmp::PartialEq<
                            ::bevy_render::view::ColorGrading,
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
            "with_identical_sections",
            |
                global: V<::bevy_render::view::ColorGradingGlobal>,
                section: V<::bevy_render::view::ColorGradingSection>|
            {
                let output: V<::bevy_render::view::ColorGrading> = {
                    {
                        let output: V<::bevy_render::view::ColorGrading> = ::bevy_render::view::ColorGrading::with_identical_sections(
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
pub(crate) fn register_render_visible_entities_class_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::RenderVisibleEntitiesClass,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_render::view::RenderVisibleEntitiesClass>| {
                let output: V<::bevy_render::view::RenderVisibleEntitiesClass> = {
                    {
                        let output: V<::bevy_render::view::RenderVisibleEntitiesClass> = <::bevy_render::view::RenderVisibleEntitiesClass as ::std::clone::Clone>::clone(
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
            "entity_pair_is_visible",
            |
                _self: R<::bevy_render::view::RenderVisibleEntitiesClass>,
                entity: V<::bevy_ecs::entity::Entity>,
                main_entity: V<::bevy_render::sync_world::MainEntity>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_render::view::RenderVisibleEntitiesClass::entity_pair_is_visible(
                                &_self,
                                entity.into_inner(),
                                main_entity.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if the given entity pair is known to be visible.\n This checks both the CPU culling visible entries table and the\n no-CPU-culling visible entries table.",
            &["_self", "entity", "main_entity"],
        )
        .register_documented(
            "prepare_for_new_frame",
            |mut _self: M<::bevy_render::view::RenderVisibleEntitiesClass>| {
                let output: () = {
                    {
                        let output: () = ::bevy_render::view::RenderVisibleEntitiesClass::prepare_for_new_frame(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Clears out the lists of added and removed entities in preparation for a\n new frame.",
            &["_self"],
        )
        .register_documented(
            "sort_added_entities",
            |mut _self: M<::bevy_render::view::RenderVisibleEntitiesClass>| {
                let output: () = {
                    {
                        let output: () = ::bevy_render::view::RenderVisibleEntitiesClass::sort_added_entities(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sorts the [`Self::added_entities`] list.\n You must call this after pushing entities onto the list, as the\n `DirtySpecializations` iterator will binary search it.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::view::RenderVisibleEntitiesClass,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_render_graph_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::camera::CameraRenderGraph,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_render::camera::CameraRenderGraph>| {
            let output: V<::bevy_render::camera::CameraRenderGraph> = {
                {
                    let output: V<::bevy_render::camera::CameraRenderGraph> =
                        <::bevy_render::camera::CameraRenderGraph as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_render::camera::TemporalJitter>| {
            let output: V<::bevy_render::camera::TemporalJitter> = {
                {
                    let output: V<::bevy_render::camera::TemporalJitter> =
                        <::bevy_render::camera::TemporalJitter as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_render::camera::TemporalJitter>,
         mut clip_from_view: M<::bevy_math::Mat4>,
         view_size: V<::bevy_math::Vec2>| {
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
        |_self: R<::bevy_render::camera::MipBias>| {
            let output: V<::bevy_render::camera::MipBias> = {
                {
                    let output: V<::bevy_render::camera::MipBias> =
                        <::bevy_render::camera::MipBias as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_render::globals::GlobalsUniform>| {
            let output: V<::bevy_render::globals::GlobalsUniform> = {
                {
                    let output: V<::bevy_render::globals::GlobalsUniform> =
                        <::bevy_render::globals::GlobalsUniform as ::std::clone::Clone>::clone(
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
pub(crate) fn register_shader_buffer_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::storage::ShaderBuffer,
    >::new(world)
        .register_documented(
            "buffer_size",
            |_self: R<::bevy_render::storage::ShaderBuffer>| {
                let output: u64 = {
                    {
                        let output: u64 = ::bevy_render::storage::ShaderBuffer::buffer_size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the size of the buffer in bytes.",
            &["_self"],
        )
        .register_documented(
            "clear",
            |mut _self: M<::bevy_render::storage::ShaderBuffer>| {
                let output: () = {
                    {
                        let output: () = ::bevy_render::storage::ShaderBuffer::clear(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Clear [`Self::data`] if it is [`ShaderBufferData::Initialized`] with its capacity and buffer size reserved.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_render::storage::ShaderBuffer>| {
                let output: V<::bevy_render::storage::ShaderBuffer> = {
                    {
                        let output: V<::bevy_render::storage::ShaderBuffer> = <::bevy_render::storage::ShaderBuffer as ::std::clone::Clone>::clone(
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
            "resize",
            |mut _self: M<::bevy_render::storage::ShaderBuffer>, new_size: u64| {
                let output: () = {
                    {
                        let output: () = ::bevy_render::storage::ShaderBuffer::resize(
                                &mut _self,
                                new_size,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Resizes the CPU data and buffer to the new size.\n If CPU data is present, the GPU buffer will be re-populated using CPU data.\n Any remaining part that lacks CPU data is implicitly zero-initialized by wgpu.\n If CPU data is not present, the entire GPU buffer will be reallocated and implicitly zero-initialized by wgpu.\n If `copy_on_resize` is true, previous buffer will attempt to be copied to this buffer.\n CPU data is truncated or zero-extended, too.",
            &["_self", "new_size"],
        )
        .register_documented(
            "resize_buffer",
            |mut _self: M<::bevy_render::storage::ShaderBuffer>, new_size: u64| {
                let output: () = {
                    {
                        let output: () = ::bevy_render::storage::ShaderBuffer::resize_buffer(
                                &mut _self,
                                new_size,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Resizes the GPU buffer to the new size.\n If CPU data is present, the GPU buffer will be re-populated using CPU data.\n Any remaining part that lacks CPU data is implicitly zero-initialized by wgpu.\n If CPU data is not present, the entire GPU buffer will be reallocated and implicitly zero-initialized by wgpu.\n If `copy_on_resize` is true, previous buffer will attempt to be copied to this buffer.\n CPU data is unchanged.",
            &["_self", "new_size"],
        )
        .register_documented(
            "with_size",
            |size: u64, asset_usage: V<::bevy_asset::RenderAssetUsages>| {
                let output: V<::bevy_render::storage::ShaderBuffer> = {
                    {
                        let output: V<::bevy_render::storage::ShaderBuffer> = ::bevy_render::storage::ShaderBuffer::with_size(
                                size,
                                asset_usage.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new uninitialized storage buffer with the given size and asset usage.",
            &["size", "asset_usage"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_render::storage::ShaderBuffer,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_shader_buffer_data_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::storage::ShaderBufferData,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_render::storage::ShaderBufferData>| {
            let output: V<::bevy_render::storage::ShaderBufferData> = {
                {
                    let output: V<::bevy_render::storage::ShaderBufferData> =
                        <::bevy_render::storage::ShaderBufferData as ::std::clone::Clone>::clone(
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
            ::bevy_render::storage::ShaderBufferData,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_readback_once_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::gpu_readback::ReadbackOnce,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_render::gpu_readback::ReadbackOnce>| {
            let output: V<::bevy_render::gpu_readback::ReadbackOnce> = {
                {
                    let output: V<::bevy_render::gpu_readback::ReadbackOnce> =
                        <::bevy_render::gpu_readback::ReadbackOnce as ::std::clone::Clone>::clone(
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
            ::bevy_render::gpu_readback::ReadbackOnce,
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
pub(crate) fn register_material_binding_id_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::material_bind_groups::MaterialBindingId,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_render::material_bind_groups::MaterialBindingId>| {
                let output: V<::bevy_render::material_bind_groups::MaterialBindingId> = {
                    {
                        let output: V<
                            ::bevy_render::material_bind_groups::MaterialBindingId,
                        > = <::bevy_render::material_bind_groups::MaterialBindingId as ::std::clone::Clone>::clone(
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
            ::bevy_render::material_bind_groups::MaterialBindingId,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_material_bind_group_slot_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::material_bind_groups::MaterialBindGroupSlot,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_render::material_bind_groups::MaterialBindGroupSlot>| {
                let output: V<
                    ::bevy_render::material_bind_groups::MaterialBindGroupSlot,
                > = {
                    {
                        let output: V<
                            ::bevy_render::material_bind_groups::MaterialBindGroupSlot,
                        > = <::bevy_render::material_bind_groups::MaterialBindGroupSlot as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_render::material_bind_groups::MaterialBindGroupSlot>,
                other: R<::bevy_render::material_bind_groups::MaterialBindGroupSlot>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::material_bind_groups::MaterialBindGroupSlot as ::std::cmp::PartialEq<
                            ::bevy_render::material_bind_groups::MaterialBindGroupSlot,
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
            ::bevy_render::material_bind_groups::MaterialBindGroupSlot,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_material_bind_group_index_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::material_bind_groups::MaterialBindGroupIndex,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_render::material_bind_groups::MaterialBindGroupIndex>| {
                let output: V<
                    ::bevy_render::material_bind_groups::MaterialBindGroupIndex,
                > = {
                    {
                        let output: V<
                            ::bevy_render::material_bind_groups::MaterialBindGroupIndex,
                        > = <::bevy_render::material_bind_groups::MaterialBindGroupIndex as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_render::material_bind_groups::MaterialBindGroupIndex>,
                other: R<::bevy_render::material_bind_groups::MaterialBindGroupIndex>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_render::material_bind_groups::MaterialBindGroupIndex as ::std::cmp::PartialEq<
                            ::bevy_render::material_bind_groups::MaterialBindGroupIndex,
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
            ::bevy_render::material_bind_groups::MaterialBindGroupIndex,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_tonemapping_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::Tonemapping,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_render::view::Tonemapping>| {
            let output: V<::bevy_render::view::Tonemapping> = {
                {
                    let output: V<::bevy_render::view::Tonemapping> =
                        <::bevy_render::view::Tonemapping as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_render::view::Tonemapping>, other: R<::bevy_render::view::Tonemapping>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_render::view::Tonemapping as ::std::cmp::PartialEq<
                            ::bevy_render::view::Tonemapping,
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
        "is_enabled",
        |_self: R<::bevy_render::view::Tonemapping>| {
            let output: bool = {
                {
                    let output: bool = ::bevy_render::view::Tonemapping::is_enabled(&_self).into();
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
            ::bevy_render::view::Tonemapping,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_deband_dither_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::DebandDither,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_render::view::DebandDither>| {
            let output: V<::bevy_render::view::DebandDither> = {
                {
                    let output: V<::bevy_render::view::DebandDither> =
                        <::bevy_render::view::DebandDither as ::std::clone::Clone>::clone(&_self)
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
        |_self: R<::bevy_render::view::DebandDither>,
         other: R<::bevy_render::view::DebandDither>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_render::view::DebandDither as ::std::cmp::PartialEq<
                            ::bevy_render::view::DebandDither,
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
            ::bevy_render::view::DebandDither,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_color_grading_global_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::ColorGradingGlobal,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_render::view::ColorGradingGlobal>| {
            let output: V<::bevy_render::view::ColorGradingGlobal> = {
                {
                    let output: V<::bevy_render::view::ColorGradingGlobal> =
                        <::bevy_render::view::ColorGradingGlobal as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_render::view::ColorGradingGlobal>,
         other: R<::bevy_render::view::ColorGradingGlobal>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_render::view::ColorGradingGlobal as ::std::cmp::PartialEq<
                            ::bevy_render::view::ColorGradingGlobal,
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
        |_self: R<::bevy_render::view::ColorGradingSection>| {
            let output: V<::bevy_render::view::ColorGradingSection> = {
                {
                    let output: V<::bevy_render::view::ColorGradingSection> =
                        <::bevy_render::view::ColorGradingSection as ::std::clone::Clone>::clone(
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
        |_self: R<::bevy_render::view::ColorGradingSection>,
         other: R<::bevy_render::view::ColorGradingSection>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_render::view::ColorGradingSection as ::std::cmp::PartialEq<
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
pub(crate) fn register_render_shadow_map_visible_entities_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_render::view::RenderShadowMapVisibleEntities,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_render::view::RenderShadowMapVisibleEntities>| {
                let output: V<::bevy_render::view::RenderShadowMapVisibleEntities> = {
                    {
                        let output: V<
                            ::bevy_render::view::RenderShadowMapVisibleEntities,
                        > = <::bevy_render::view::RenderShadowMapVisibleEntities as ::std::clone::Clone>::clone(
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
            ::bevy_render::view::RenderShadowMapVisibleEntities,
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
            let output: V<::bevy_render::view::screenshot::Screenshot> = {
                {
                    let output: V<::bevy_render::view::screenshot::Screenshot> =
                        ::bevy_render::view::screenshot::Screenshot::primary_window().into();
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
        |texture_view: V<::bevy_camera::ManualTextureViewHandle>| {
            let output: V<::bevy_render::view::screenshot::Screenshot> = {
                {
                    let output: V<::bevy_render::view::screenshot::Screenshot> =
                        ::bevy_render::view::screenshot::Screenshot::texture_view(
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
        |window: V<::bevy_ecs::entity::Entity>| {
            let output: V<::bevy_render::view::screenshot::Screenshot> = {
                {
                    let output: V<::bevy_render::view::screenshot::Screenshot> =
                        ::bevy_render::view::screenshot::Screenshot::window(window.into_inner())
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
        register_msaa_functions(&mut world);
        register_occlusion_culling_functions(&mut world);
        register_color_grading_functions(&mut world);
        register_render_visible_entities_class_functions(&mut world);
        register_camera_render_graph_functions(&mut world);
        register_temporal_jitter_functions(&mut world);
        register_mip_bias_functions(&mut world);
        register_globals_uniform_functions(&mut world);
        register_shader_buffer_functions(&mut world);
        register_shader_buffer_data_functions(&mut world);
        register_readback_once_functions(&mut world);
        register_readback_complete_functions(&mut world);
        register_material_binding_id_functions(&mut world);
        register_material_bind_group_slot_functions(&mut world);
        register_material_bind_group_index_functions(&mut world);
        register_tonemapping_functions(&mut world);
        register_deband_dither_functions(&mut world);
        register_color_grading_global_functions(&mut world);
        register_color_grading_section_functions(&mut world);
        register_render_shadow_map_visible_entities_functions(&mut world);
        register_screenshot_captured_functions(&mut world);
        register_screenshot_functions(&mut world);
    }
}
