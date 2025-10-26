#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{Mut, Ref, Val},
        namespace::NamespaceBuilder,
    },
};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyCorePipelineScriptingPlugin;
pub(crate) fn register_skybox_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::Skybox,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_core_pipeline::Skybox>| {
            let output: Val<::bevy_core_pipeline::Skybox> = {
                {
                    let output: Val<::bevy_core_pipeline::Skybox> =
                        <::bevy_core_pipeline::Skybox as ::std::clone::Clone>::clone(&_self).into();
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
            ::bevy_core_pipeline::Skybox,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::core_2d::Camera2d,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_core_pipeline::core_2d::Camera2d>| {
            let output: Val<::bevy_core_pipeline::core_2d::Camera2d> = {
                {
                    let output: Val<::bevy_core_pipeline::core_2d::Camera2d> =
                        <::bevy_core_pipeline::core_2d::Camera2d as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::core_2d::Camera2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::core_3d::Camera3d,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_core_pipeline::core_3d::Camera3d>| {
            let output: Val<::bevy_core_pipeline::core_3d::Camera3d> = {
                {
                    let output: Val<::bevy_core_pipeline::core_3d::Camera3d> =
                        <::bevy_core_pipeline::core_3d::Camera3d as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::core_3d::Camera3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_deferred_prepass_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::prepass::DeferredPrepass,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_core_pipeline::prepass::DeferredPrepass,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_depth_prepass_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::prepass::DepthPrepass,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::prepass::DepthPrepass>| {
                let output: Val<::bevy_core_pipeline::prepass::DepthPrepass> = {
                    {
                        let output: Val<::bevy_core_pipeline::prepass::DepthPrepass> = <::bevy_core_pipeline::prepass::DepthPrepass as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::prepass::DepthPrepass,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_motion_vector_prepass_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::prepass::MotionVectorPrepass,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::prepass::MotionVectorPrepass>| {
                let output: Val<::bevy_core_pipeline::prepass::MotionVectorPrepass> = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::prepass::MotionVectorPrepass,
                        > = <::bevy_core_pipeline::prepass::MotionVectorPrepass as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::prepass::MotionVectorPrepass,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_normal_prepass_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::prepass::NormalPrepass,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::prepass::NormalPrepass>| {
                let output: Val<::bevy_core_pipeline::prepass::NormalPrepass> = {
                    {
                        let output: Val<::bevy_core_pipeline::prepass::NormalPrepass> = <::bevy_core_pipeline::prepass::NormalPrepass as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::prepass::NormalPrepass,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_auto_exposure_compensation_curve_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::auto_exposure::AutoExposureCompensationCurve,
    >::new(world)
        .register_documented(
            "clone",
            |
                _self: Ref<
                    ::bevy_core_pipeline::auto_exposure::AutoExposureCompensationCurve,
                >|
            {
                let output: Val<
                    ::bevy_core_pipeline::auto_exposure::AutoExposureCompensationCurve,
                > = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::auto_exposure::AutoExposureCompensationCurve,
                        > = <::bevy_core_pipeline::auto_exposure::AutoExposureCompensationCurve as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::auto_exposure::AutoExposureCompensationCurve,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_auto_exposure_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::auto_exposure::AutoExposure,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::auto_exposure::AutoExposure>| {
                let output: Val<::bevy_core_pipeline::auto_exposure::AutoExposure> = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::auto_exposure::AutoExposure,
                        > = <::bevy_core_pipeline::auto_exposure::AutoExposure as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::auto_exposure::AutoExposure,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_tonemapping_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::tonemapping::Tonemapping,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_core_pipeline::tonemapping::Tonemapping>| {
                let output: () = {
                    {
                        let output: () = <::bevy_core_pipeline::tonemapping::Tonemapping as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_core_pipeline::tonemapping::Tonemapping>| {
                let output: Val<::bevy_core_pipeline::tonemapping::Tonemapping> = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::tonemapping::Tonemapping,
                        > = <::bevy_core_pipeline::tonemapping::Tonemapping as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_core_pipeline::tonemapping::Tonemapping>,
                other: Ref<::bevy_core_pipeline::tonemapping::Tonemapping>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_core_pipeline::tonemapping::Tonemapping as ::std::cmp::PartialEq<
                            ::bevy_core_pipeline::tonemapping::Tonemapping,
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
            |_self: Ref<::bevy_core_pipeline::tonemapping::Tonemapping>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_core_pipeline::tonemapping::Tonemapping::is_enabled(
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
            ::bevy_core_pipeline::tonemapping::Tonemapping,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bloom_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::bloom::Bloom,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_core_pipeline::bloom::Bloom>| {
            let output: Val<::bevy_core_pipeline::bloom::Bloom> = {
                {
                    let output: Val<::bevy_core_pipeline::bloom::Bloom> =
                        <::bevy_core_pipeline::bloom::Bloom as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_core_pipeline::bloom::Bloom,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bloom_composite_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::bloom::BloomCompositeMode,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_core_pipeline::bloom::BloomCompositeMode>| {
                let output: () = {
                    {
                        let output: () = <::bevy_core_pipeline::bloom::BloomCompositeMode as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_core_pipeline::bloom::BloomCompositeMode>| {
                let output: Val<::bevy_core_pipeline::bloom::BloomCompositeMode> = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::bloom::BloomCompositeMode,
                        > = <::bevy_core_pipeline::bloom::BloomCompositeMode as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_core_pipeline::bloom::BloomCompositeMode>,
                other: Ref<::bevy_core_pipeline::bloom::BloomCompositeMode>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_core_pipeline::bloom::BloomCompositeMode as ::std::cmp::PartialEq<
                            ::bevy_core_pipeline::bloom::BloomCompositeMode,
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
            ::bevy_core_pipeline::bloom::BloomCompositeMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bloom_prefilter_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::bloom::BloomPrefilter,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::bloom::BloomPrefilter>| {
                let output: Val<::bevy_core_pipeline::bloom::BloomPrefilter> = {
                    {
                        let output: Val<::bevy_core_pipeline::bloom::BloomPrefilter> = <::bevy_core_pipeline::bloom::BloomPrefilter as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::bloom::BloomPrefilter,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_contrast_adaptive_sharpening_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::contrast_adaptive_sharpening::ContrastAdaptiveSharpening,
    >::new(world)
        .register_documented(
            "clone",
            |
                _self: Ref<
                    ::bevy_core_pipeline::contrast_adaptive_sharpening::ContrastAdaptiveSharpening,
                >|
            {
                let output: Val<
                    ::bevy_core_pipeline::contrast_adaptive_sharpening::ContrastAdaptiveSharpening,
                > = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::contrast_adaptive_sharpening::ContrastAdaptiveSharpening,
                        > = <::bevy_core_pipeline::contrast_adaptive_sharpening::ContrastAdaptiveSharpening as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::contrast_adaptive_sharpening::ContrastAdaptiveSharpening,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_denoise_cas_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::contrast_adaptive_sharpening::DenoiseCas,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::contrast_adaptive_sharpening::DenoiseCas>| {
                let output: Val<
                    ::bevy_core_pipeline::contrast_adaptive_sharpening::DenoiseCas,
                > = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::contrast_adaptive_sharpening::DenoiseCas,
                        > = <::bevy_core_pipeline::contrast_adaptive_sharpening::DenoiseCas as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::contrast_adaptive_sharpening::DenoiseCas,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_fxaa_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::fxaa::Fxaa,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_core_pipeline::fxaa::Fxaa>| {
            let output: Val<::bevy_core_pipeline::fxaa::Fxaa> = {
                {
                    let output: Val<::bevy_core_pipeline::fxaa::Fxaa> =
                        <::bevy_core_pipeline::fxaa::Fxaa as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_core_pipeline::fxaa::Fxaa,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_smaa_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::smaa::Smaa,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_core_pipeline::smaa::Smaa>| {
            let output: Val<::bevy_core_pipeline::smaa::Smaa> = {
                {
                    let output: Val<::bevy_core_pipeline::smaa::Smaa> =
                        <::bevy_core_pipeline::smaa::Smaa as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_core_pipeline::smaa::Smaa,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_deband_dither_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::tonemapping::DebandDither,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_core_pipeline::tonemapping::DebandDither>| {
                let output: () = {
                    {
                        let output: () = <::bevy_core_pipeline::tonemapping::DebandDither as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_core_pipeline::tonemapping::DebandDither>| {
                let output: Val<::bevy_core_pipeline::tonemapping::DebandDither> = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::tonemapping::DebandDither,
                        > = <::bevy_core_pipeline::tonemapping::DebandDither as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_core_pipeline::tonemapping::DebandDither>,
                other: Ref<::bevy_core_pipeline::tonemapping::DebandDither>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_core_pipeline::tonemapping::DebandDither as ::std::cmp::PartialEq<
                            ::bevy_core_pipeline::tonemapping::DebandDither,
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
            ::bevy_core_pipeline::tonemapping::DebandDither,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_motion_blur_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::motion_blur::MotionBlur,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::motion_blur::MotionBlur>| {
                let output: Val<::bevy_core_pipeline::motion_blur::MotionBlur> = {
                    {
                        let output: Val<::bevy_core_pipeline::motion_blur::MotionBlur> = <::bevy_core_pipeline::motion_blur::MotionBlur as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::motion_blur::MotionBlur,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_depth_of_field_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::dof::DepthOfField,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_core_pipeline::dof::DepthOfField>| {
            let output: Val<::bevy_core_pipeline::dof::DepthOfField> = {
                {
                    let output: Val<::bevy_core_pipeline::dof::DepthOfField> =
                        <::bevy_core_pipeline::dof::DepthOfField as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::dof::DepthOfField,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_screen_space_transmission_quality_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::core_3d::ScreenSpaceTransmissionQuality,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::core_3d::ScreenSpaceTransmissionQuality>| {
                let output: Val<
                    ::bevy_core_pipeline::core_3d::ScreenSpaceTransmissionQuality,
                > = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::core_3d::ScreenSpaceTransmissionQuality,
                        > = <::bevy_core_pipeline::core_3d::ScreenSpaceTransmissionQuality as ::std::clone::Clone>::clone(
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
                _self: Ref<
                    ::bevy_core_pipeline::core_3d::ScreenSpaceTransmissionQuality,
                >,
                other: Ref<
                    ::bevy_core_pipeline::core_3d::ScreenSpaceTransmissionQuality,
                >|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_core_pipeline::core_3d::ScreenSpaceTransmissionQuality as ::std::cmp::PartialEq<
                            ::bevy_core_pipeline::core_3d::ScreenSpaceTransmissionQuality,
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
            ::bevy_core_pipeline::core_3d::ScreenSpaceTransmissionQuality,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_3_d_depth_load_op_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::core_3d::Camera3dDepthLoadOp,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::core_3d::Camera3dDepthLoadOp>| {
                let output: Val<::bevy_core_pipeline::core_3d::Camera3dDepthLoadOp> = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::core_3d::Camera3dDepthLoadOp,
                        > = <::bevy_core_pipeline::core_3d::Camera3dDepthLoadOp as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::core_3d::Camera3dDepthLoadOp,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_camera_3_d_depth_texture_usage_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::core_3d::Camera3dDepthTextureUsage,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::core_3d::Camera3dDepthTextureUsage>| {
                let output: Val<
                    ::bevy_core_pipeline::core_3d::Camera3dDepthTextureUsage,
                > = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::core_3d::Camera3dDepthTextureUsage,
                        > = <::bevy_core_pipeline::core_3d::Camera3dDepthTextureUsage as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::core_3d::Camera3dDepthTextureUsage,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_depth_of_field_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::dof::DepthOfFieldMode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::dof::DepthOfFieldMode>| {
                let output: Val<::bevy_core_pipeline::dof::DepthOfFieldMode> = {
                    {
                        let output: Val<::bevy_core_pipeline::dof::DepthOfFieldMode> = <::bevy_core_pipeline::dof::DepthOfFieldMode as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_core_pipeline::dof::DepthOfFieldMode>,
                other: Ref<::bevy_core_pipeline::dof::DepthOfFieldMode>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_core_pipeline::dof::DepthOfFieldMode as ::std::cmp::PartialEq<
                            ::bevy_core_pipeline::dof::DepthOfFieldMode,
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
            ::bevy_core_pipeline::dof::DepthOfFieldMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_temporal_anti_aliasing_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::experimental::taa::TemporalAntiAliasing,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::experimental::taa::TemporalAntiAliasing>| {
                let output: Val<
                    ::bevy_core_pipeline::experimental::taa::TemporalAntiAliasing,
                > = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::experimental::taa::TemporalAntiAliasing,
                        > = <::bevy_core_pipeline::experimental::taa::TemporalAntiAliasing as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::experimental::taa::TemporalAntiAliasing,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sensitivity_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::fxaa::Sensitivity,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_core_pipeline::fxaa::Sensitivity>| {
                let output: () = {
                    {
                        let output: () = <::bevy_core_pipeline::fxaa::Sensitivity as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_core_pipeline::fxaa::Sensitivity>| {
                let output: Val<::bevy_core_pipeline::fxaa::Sensitivity> = {
                    {
                        let output: Val<::bevy_core_pipeline::fxaa::Sensitivity> = <::bevy_core_pipeline::fxaa::Sensitivity as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_core_pipeline::fxaa::Sensitivity>,
                other: Ref<::bevy_core_pipeline::fxaa::Sensitivity>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_core_pipeline::fxaa::Sensitivity as ::std::cmp::PartialEq<
                            ::bevy_core_pipeline::fxaa::Sensitivity,
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
            ::bevy_core_pipeline::fxaa::Sensitivity,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_order_independent_transparency_settings_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::oit::OrderIndependentTransparencySettings,
    >::new(world)
        .register_documented(
            "clone",
            |
                _self: Ref<
                    ::bevy_core_pipeline::oit::OrderIndependentTransparencySettings,
                >|
            {
                let output: Val<
                    ::bevy_core_pipeline::oit::OrderIndependentTransparencySettings,
                > = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::oit::OrderIndependentTransparencySettings,
                        > = <::bevy_core_pipeline::oit::OrderIndependentTransparencySettings as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::oit::OrderIndependentTransparencySettings,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_chromatic_aberration_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::post_process::ChromaticAberration,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::post_process::ChromaticAberration>| {
                let output: Val<
                    ::bevy_core_pipeline::post_process::ChromaticAberration,
                > = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::post_process::ChromaticAberration,
                        > = <::bevy_core_pipeline::post_process::ChromaticAberration as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::post_process::ChromaticAberration,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_smaa_preset_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::smaa::SmaaPreset,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_core_pipeline::smaa::SmaaPreset>| {
                let output: () = {
                    {
                        let output: () = <::bevy_core_pipeline::smaa::SmaaPreset as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_core_pipeline::smaa::SmaaPreset>| {
                let output: Val<::bevy_core_pipeline::smaa::SmaaPreset> = {
                    {
                        let output: Val<::bevy_core_pipeline::smaa::SmaaPreset> = <::bevy_core_pipeline::smaa::SmaaPreset as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_core_pipeline::smaa::SmaaPreset>,
                other: Ref<::bevy_core_pipeline::smaa::SmaaPreset>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_core_pipeline::smaa::SmaaPreset as ::std::cmp::PartialEq<
                            ::bevy_core_pipeline::smaa::SmaaPreset,
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
            ::bevy_core_pipeline::smaa::SmaaPreset,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyCorePipelineScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_skybox_functions(&mut world);
        register_camera_2_d_functions(&mut world);
        register_camera_3_d_functions(&mut world);
        register_deferred_prepass_functions(&mut world);
        register_depth_prepass_functions(&mut world);
        register_motion_vector_prepass_functions(&mut world);
        register_normal_prepass_functions(&mut world);
        register_auto_exposure_compensation_curve_functions(&mut world);
        register_auto_exposure_functions(&mut world);
        register_tonemapping_functions(&mut world);
        register_bloom_functions(&mut world);
        register_bloom_composite_mode_functions(&mut world);
        register_bloom_prefilter_functions(&mut world);
        register_contrast_adaptive_sharpening_functions(&mut world);
        register_denoise_cas_functions(&mut world);
        register_fxaa_functions(&mut world);
        register_smaa_functions(&mut world);
        register_deband_dither_functions(&mut world);
        register_motion_blur_functions(&mut world);
        register_depth_of_field_functions(&mut world);
        register_screen_space_transmission_quality_functions(&mut world);
        register_camera_3_d_depth_load_op_functions(&mut world);
        register_camera_3_d_depth_texture_usage_functions(&mut world);
        register_depth_of_field_mode_functions(&mut world);
        register_temporal_anti_aliasing_functions(&mut world);
        register_sensitivity_functions(&mut world);
        register_order_independent_transparency_settings_functions(&mut world);
        register_chromatic_aberration_functions(&mut world);
        register_smaa_preset_functions(&mut world);
    }
}
