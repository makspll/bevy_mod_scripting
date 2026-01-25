
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
                        let output: Val<::bevy_core_pipeline::Skybox> = <::bevy_core_pipeline::Skybox as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::Skybox,
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
pub(crate) fn register_deferred_prepass_double_buffer_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::prepass::DeferredPrepassDoubleBuffer,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::prepass::DeferredPrepassDoubleBuffer>| {
                let output: Val<
                    ::bevy_core_pipeline::prepass::DeferredPrepassDoubleBuffer,
                > = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::prepass::DeferredPrepassDoubleBuffer,
                        > = <::bevy_core_pipeline::prepass::DeferredPrepassDoubleBuffer as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::prepass::DeferredPrepassDoubleBuffer,
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
pub(crate) fn register_depth_prepass_double_buffer_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_core_pipeline::prepass::DepthPrepassDoubleBuffer,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_core_pipeline::prepass::DepthPrepassDoubleBuffer>| {
                let output: Val<
                    ::bevy_core_pipeline::prepass::DepthPrepassDoubleBuffer,
                > = {
                    {
                        let output: Val<
                            ::bevy_core_pipeline::prepass::DepthPrepassDoubleBuffer,
                        > = <::bevy_core_pipeline::prepass::DepthPrepassDoubleBuffer as ::std::clone::Clone>::clone(
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
            ::bevy_core_pipeline::prepass::DepthPrepassDoubleBuffer,
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
pub(crate) fn register_order_independent_transparency_settings_functions(
    world: &mut World,
) {
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
impl Plugin for BevyCorePipelineScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_skybox_functions(&mut world);
        register_tonemapping_functions(&mut world);
        register_deband_dither_functions(&mut world);
        register_deferred_prepass_functions(&mut world);
        register_deferred_prepass_double_buffer_functions(&mut world);
        register_depth_prepass_functions(&mut world);
        register_depth_prepass_double_buffer_functions(&mut world);
        register_motion_vector_prepass_functions(&mut world);
        register_normal_prepass_functions(&mut world);
        register_order_independent_transparency_settings_functions(&mut world);
    }
}
