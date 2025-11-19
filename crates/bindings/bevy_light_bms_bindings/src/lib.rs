
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
pub struct BevyLightScriptingPlugin;
pub(crate) fn register_clustered_decal_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::ClusteredDecal,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::ClusteredDecal>| {
                let output: Val<::bevy_light::ClusteredDecal> = {
                    {
                        let output: Val<::bevy_light::ClusteredDecal> = <::bevy_light::ClusteredDecal as ::std::clone::Clone>::clone(
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
            ::bevy_light::ClusteredDecal,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ambient_light_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::AmbientLight,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::AmbientLight>| {
                let output: Val<::bevy_light::AmbientLight> = {
                    {
                        let output: Val<::bevy_light::AmbientLight> = <::bevy_light::AmbientLight as ::std::clone::Clone>::clone(
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
            ::bevy_light::AmbientLight,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_environment_map_light_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::EnvironmentMapLight,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::EnvironmentMapLight>| {
                let output: Val<::bevy_light::EnvironmentMapLight> = {
                    {
                        let output: Val<::bevy_light::EnvironmentMapLight> = <::bevy_light::EnvironmentMapLight as ::std::clone::Clone>::clone(
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
            ::bevy_light::EnvironmentMapLight,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_generated_environment_map_light_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::GeneratedEnvironmentMapLight,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::GeneratedEnvironmentMapLight>| {
                let output: Val<::bevy_light::GeneratedEnvironmentMapLight> = {
                    {
                        let output: Val<::bevy_light::GeneratedEnvironmentMapLight> = <::bevy_light::GeneratedEnvironmentMapLight as ::std::clone::Clone>::clone(
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
            ::bevy_light::GeneratedEnvironmentMapLight,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_irradiance_volume_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::IrradianceVolume,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::IrradianceVolume>| {
                let output: Val<::bevy_light::IrradianceVolume> = {
                    {
                        let output: Val<::bevy_light::IrradianceVolume> = <::bevy_light::IrradianceVolume as ::std::clone::Clone>::clone(
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
            ::bevy_light::IrradianceVolume,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_light_probe_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::LightProbe,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::LightProbe>| {
                let output: Val<::bevy_light::LightProbe> = {
                    {
                        let output: Val<::bevy_light::LightProbe> = <::bevy_light::LightProbe as ::std::clone::Clone>::clone(
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
            || {
                let output: Val<::bevy_light::LightProbe> = {
                    {
                        let output: Val<::bevy_light::LightProbe> = ::bevy_light::LightProbe::new()
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new light probe component.",
            &[],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_light::LightProbe,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_fog_volume_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::FogVolume,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::FogVolume>| {
                let output: Val<::bevy_light::FogVolume> = {
                    {
                        let output: Val<::bevy_light::FogVolume> = <::bevy_light::FogVolume as ::std::clone::Clone>::clone(
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
            ::bevy_light::FogVolume,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_volumetric_fog_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::VolumetricFog,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::VolumetricFog>| {
                let output: Val<::bevy_light::VolumetricFog> = {
                    {
                        let output: Val<::bevy_light::VolumetricFog> = <::bevy_light::VolumetricFog as ::std::clone::Clone>::clone(
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
            ::bevy_light::VolumetricFog,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_volumetric_light_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::VolumetricLight,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::VolumetricLight>| {
                let output: Val<::bevy_light::VolumetricLight> = {
                    {
                        let output: Val<::bevy_light::VolumetricLight> = <::bevy_light::VolumetricLight as ::std::clone::Clone>::clone(
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
            ::bevy_light::VolumetricLight,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cascade_shadow_config_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::CascadeShadowConfig,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::CascadeShadowConfig>| {
                let output: Val<::bevy_light::CascadeShadowConfig> = {
                    {
                        let output: Val<::bevy_light::CascadeShadowConfig> = <::bevy_light::CascadeShadowConfig as ::std::clone::Clone>::clone(
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
            ::bevy_light::CascadeShadowConfig,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cascades_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::Cascades,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::Cascades>| {
                let output: Val<::bevy_light::Cascades> = {
                    {
                        let output: Val<::bevy_light::Cascades> = <::bevy_light::Cascades as ::std::clone::Clone>::clone(
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
            ::bevy_light::Cascades,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_point_light_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::PointLight,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::PointLight>| {
                let output: Val<::bevy_light::PointLight> = {
                    {
                        let output: Val<::bevy_light::PointLight> = <::bevy_light::PointLight as ::std::clone::Clone>::clone(
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
            ::bevy_light::PointLight,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_point_light_shadow_map_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::PointLightShadowMap,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::PointLightShadowMap>| {
                let output: Val<::bevy_light::PointLightShadowMap> = {
                    {
                        let output: Val<::bevy_light::PointLightShadowMap> = <::bevy_light::PointLightShadowMap as ::std::clone::Clone>::clone(
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
            ::bevy_light::PointLightShadowMap,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_point_light_texture_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::PointLightTexture,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::PointLightTexture>| {
                let output: Val<::bevy_light::PointLightTexture> = {
                    {
                        let output: Val<::bevy_light::PointLightTexture> = <::bevy_light::PointLightTexture as ::std::clone::Clone>::clone(
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
            ::bevy_light::PointLightTexture,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_spot_light_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::SpotLight,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::SpotLight>| {
                let output: Val<::bevy_light::SpotLight> = {
                    {
                        let output: Val<::bevy_light::SpotLight> = <::bevy_light::SpotLight as ::std::clone::Clone>::clone(
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
            ::bevy_light::SpotLight,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_spot_light_texture_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::SpotLightTexture,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::SpotLightTexture>| {
                let output: Val<::bevy_light::SpotLightTexture> = {
                    {
                        let output: Val<::bevy_light::SpotLightTexture> = <::bevy_light::SpotLightTexture as ::std::clone::Clone>::clone(
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
            ::bevy_light::SpotLightTexture,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_directional_light_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::DirectionalLight,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::DirectionalLight>| {
                let output: Val<::bevy_light::DirectionalLight> = {
                    {
                        let output: Val<::bevy_light::DirectionalLight> = <::bevy_light::DirectionalLight as ::std::clone::Clone>::clone(
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
            ::bevy_light::DirectionalLight,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_directional_light_shadow_map_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::DirectionalLightShadowMap,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::DirectionalLightShadowMap>| {
                let output: Val<::bevy_light::DirectionalLightShadowMap> = {
                    {
                        let output: Val<::bevy_light::DirectionalLightShadowMap> = <::bevy_light::DirectionalLightShadowMap as ::std::clone::Clone>::clone(
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
            ::bevy_light::DirectionalLightShadowMap,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_directional_light_texture_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::DirectionalLightTexture,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::DirectionalLightTexture>| {
                let output: Val<::bevy_light::DirectionalLightTexture> = {
                    {
                        let output: Val<::bevy_light::DirectionalLightTexture> = <::bevy_light::DirectionalLightTexture as ::std::clone::Clone>::clone(
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
            ::bevy_light::DirectionalLightTexture,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_not_shadow_caster_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::NotShadowCaster,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::NotShadowCaster>| {
                let output: Val<::bevy_light::NotShadowCaster> = {
                    {
                        let output: Val<::bevy_light::NotShadowCaster> = <::bevy_light::NotShadowCaster as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_light::NotShadowCaster>,
                other: Ref<::bevy_light::NotShadowCaster>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_light::NotShadowCaster as ::std::cmp::PartialEq<
                            ::bevy_light::NotShadowCaster,
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
            ::bevy_light::NotShadowCaster,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_not_shadow_receiver_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::NotShadowReceiver,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_light::NotShadowReceiver,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_transmitted_shadow_receiver_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::TransmittedShadowReceiver,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_light::TransmittedShadowReceiver,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_shadow_filtering_method_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::ShadowFilteringMethod,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_light::ShadowFilteringMethod>| {
                let output: () = {
                    {
                        let output: () = <::bevy_light::ShadowFilteringMethod as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_light::ShadowFilteringMethod>| {
                let output: Val<::bevy_light::ShadowFilteringMethod> = {
                    {
                        let output: Val<::bevy_light::ShadowFilteringMethod> = <::bevy_light::ShadowFilteringMethod as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_light::ShadowFilteringMethod>,
                other: Ref<::bevy_light::ShadowFilteringMethod>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_light::ShadowFilteringMethod as ::std::cmp::PartialEq<
                            ::bevy_light::ShadowFilteringMethod,
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
            ::bevy_light::ShadowFilteringMethod,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cluster_far_z_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::cluster::ClusterFarZMode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::cluster::ClusterFarZMode>| {
                let output: Val<::bevy_light::cluster::ClusterFarZMode> = {
                    {
                        let output: Val<::bevy_light::cluster::ClusterFarZMode> = <::bevy_light::cluster::ClusterFarZMode as ::std::clone::Clone>::clone(
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
            ::bevy_light::cluster::ClusterFarZMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cluster_z_config_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::cluster::ClusterZConfig,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::cluster::ClusterZConfig>| {
                let output: Val<::bevy_light::cluster::ClusterZConfig> = {
                    {
                        let output: Val<::bevy_light::cluster::ClusterZConfig> = <::bevy_light::cluster::ClusterZConfig as ::std::clone::Clone>::clone(
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
            ::bevy_light::cluster::ClusterZConfig,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cluster_config_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::cluster::ClusterConfig,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::cluster::ClusterConfig>| {
                let output: Val<::bevy_light::cluster::ClusterConfig> = {
                    {
                        let output: Val<::bevy_light::cluster::ClusterConfig> = <::bevy_light::cluster::ClusterConfig as ::std::clone::Clone>::clone(
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
            ::bevy_light::cluster::ClusterConfig,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cascade_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_light::cascade::Cascade,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_light::cascade::Cascade>| {
                let output: Val<::bevy_light::cascade::Cascade> = {
                    {
                        let output: Val<::bevy_light::cascade::Cascade> = <::bevy_light::cascade::Cascade as ::std::clone::Clone>::clone(
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
            ::bevy_light::cascade::Cascade,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyLightScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_clustered_decal_functions(&mut world);
        register_ambient_light_functions(&mut world);
        register_environment_map_light_functions(&mut world);
        register_generated_environment_map_light_functions(&mut world);
        register_irradiance_volume_functions(&mut world);
        register_light_probe_functions(&mut world);
        register_fog_volume_functions(&mut world);
        register_volumetric_fog_functions(&mut world);
        register_volumetric_light_functions(&mut world);
        register_cascade_shadow_config_functions(&mut world);
        register_cascades_functions(&mut world);
        register_point_light_functions(&mut world);
        register_point_light_shadow_map_functions(&mut world);
        register_point_light_texture_functions(&mut world);
        register_spot_light_functions(&mut world);
        register_spot_light_texture_functions(&mut world);
        register_directional_light_functions(&mut world);
        register_directional_light_shadow_map_functions(&mut world);
        register_directional_light_texture_functions(&mut world);
        register_not_shadow_caster_functions(&mut world);
        register_not_shadow_receiver_functions(&mut world);
        register_transmitted_shadow_receiver_functions(&mut world);
        register_shadow_filtering_method_functions(&mut world);
        register_cluster_far_z_mode_functions(&mut world);
        register_cluster_z_config_functions(&mut world);
        register_cluster_config_functions(&mut world);
        register_cascade_functions(&mut world);
    }
}
