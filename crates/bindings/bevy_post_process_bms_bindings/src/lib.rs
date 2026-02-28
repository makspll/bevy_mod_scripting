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
pub struct BevyPostProcessScriptingPlugin;
pub(crate) fn register_auto_exposure_compensation_curve_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_post_process::auto_exposure::AutoExposureCompensationCurve,
    >::new(world)
        .register_documented(
            "clone",
            |
                _self: R<
                    ::bevy_post_process::auto_exposure::AutoExposureCompensationCurve,
                >|
            {
                let output: V<
                    ::bevy_post_process::auto_exposure::AutoExposureCompensationCurve,
                > = {
                    {
                        let output: V<
                            ::bevy_post_process::auto_exposure::AutoExposureCompensationCurve,
                        > = <::bevy_post_process::auto_exposure::AutoExposureCompensationCurve as ::std::clone::Clone>::clone(
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
            ::bevy_post_process::auto_exposure::AutoExposureCompensationCurve,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_auto_exposure_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_post_process::auto_exposure::AutoExposure,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_post_process::auto_exposure::AutoExposure>| {
                let output: V<::bevy_post_process::auto_exposure::AutoExposure> = {
                    {
                        let output: V<
                            ::bevy_post_process::auto_exposure::AutoExposure,
                        > = <::bevy_post_process::auto_exposure::AutoExposure as ::std::clone::Clone>::clone(
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
            ::bevy_post_process::auto_exposure::AutoExposure,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bloom_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_post_process::bloom::Bloom,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_post_process::bloom::Bloom>| {
            let output: V<::bevy_post_process::bloom::Bloom> = {
                {
                    let output: V<::bevy_post_process::bloom::Bloom> =
                        <::bevy_post_process::bloom::Bloom as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_post_process::bloom::Bloom,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bloom_composite_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_post_process::bloom::BloomCompositeMode,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |mut _self: R<::bevy_post_process::bloom::BloomCompositeMode>| {
                let output: () = {
                    {
                        let output: () = <::bevy_post_process::bloom::BloomCompositeMode as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |mut _self: R<::bevy_post_process::bloom::BloomCompositeMode>| {
                let output: V<::bevy_post_process::bloom::BloomCompositeMode> = {
                    {
                        let output: V<::bevy_post_process::bloom::BloomCompositeMode> = <::bevy_post_process::bloom::BloomCompositeMode as ::std::clone::Clone>::clone(
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
                mut _self: R<::bevy_post_process::bloom::BloomCompositeMode>,
                mut other: R<::bevy_post_process::bloom::BloomCompositeMode>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_post_process::bloom::BloomCompositeMode as ::std::cmp::PartialEq<
                            ::bevy_post_process::bloom::BloomCompositeMode,
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
            ::bevy_post_process::bloom::BloomCompositeMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bloom_prefilter_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_post_process::bloom::BloomPrefilter,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_post_process::bloom::BloomPrefilter>| {
            let output: V<::bevy_post_process::bloom::BloomPrefilter> = {
                {
                    let output: V<::bevy_post_process::bloom::BloomPrefilter> =
                        <::bevy_post_process::bloom::BloomPrefilter as ::std::clone::Clone>::clone(
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
            ::bevy_post_process::bloom::BloomPrefilter,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_depth_of_field_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_post_process::dof::DepthOfField,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_post_process::dof::DepthOfField>| {
            let output: V<::bevy_post_process::dof::DepthOfField> = {
                {
                    let output: V<::bevy_post_process::dof::DepthOfField> =
                        <::bevy_post_process::dof::DepthOfField as ::std::clone::Clone>::clone(
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
            ::bevy_post_process::dof::DepthOfField,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_depth_of_field_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_post_process::dof::DepthOfFieldMode,
    >::new(world)
    .register_documented(
        "clone",
        |mut _self: R<::bevy_post_process::dof::DepthOfFieldMode>| {
            let output: V<::bevy_post_process::dof::DepthOfFieldMode> = {
                {
                    let output: V<::bevy_post_process::dof::DepthOfFieldMode> =
                        <::bevy_post_process::dof::DepthOfFieldMode as ::std::clone::Clone>::clone(
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
        |mut _self: R<::bevy_post_process::dof::DepthOfFieldMode>,
         mut other: R<::bevy_post_process::dof::DepthOfFieldMode>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_post_process::dof::DepthOfFieldMode as ::std::cmp::PartialEq<
                            ::bevy_post_process::dof::DepthOfFieldMode,
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
            ::bevy_post_process::dof::DepthOfFieldMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_chromatic_aberration_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_post_process::effect_stack::ChromaticAberration,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_post_process::effect_stack::ChromaticAberration>| {
                let output: V<::bevy_post_process::effect_stack::ChromaticAberration> = {
                    {
                        let output: V<
                            ::bevy_post_process::effect_stack::ChromaticAberration,
                        > = <::bevy_post_process::effect_stack::ChromaticAberration as ::std::clone::Clone>::clone(
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
            ::bevy_post_process::effect_stack::ChromaticAberration,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_motion_blur_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_post_process::motion_blur::MotionBlur,
    >::new(world)
        .register_documented(
            "clone",
            |mut _self: R<::bevy_post_process::motion_blur::MotionBlur>| {
                let output: V<::bevy_post_process::motion_blur::MotionBlur> = {
                    {
                        let output: V<::bevy_post_process::motion_blur::MotionBlur> = <::bevy_post_process::motion_blur::MotionBlur as ::std::clone::Clone>::clone(
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
            ::bevy_post_process::motion_blur::MotionBlur,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyPostProcessScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_auto_exposure_compensation_curve_functions(&mut world);
        register_auto_exposure_functions(&mut world);
        register_bloom_functions(&mut world);
        register_bloom_composite_mode_functions(&mut world);
        register_bloom_prefilter_functions(&mut world);
        register_depth_of_field_functions(&mut world);
        register_depth_of_field_mode_functions(&mut world);
        register_chromatic_aberration_functions(&mut world);
        register_motion_blur_functions(&mut world);
    }
}
