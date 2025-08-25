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
pub struct BevyPbrScriptingPlugin;
pub(crate) fn register_fog_volume_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::FogVolume,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_pbr::FogVolume>| {
                let output: Val<::bevy_pbr::FogVolume> = {
                    {
                        let output: Val<::bevy_pbr::FogVolume> = <::bevy_pbr::FogVolume as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::FogVolume,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_volumetric_fog_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::VolumetricFog,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::VolumetricFog>| {
            let output: Val<::bevy_pbr::VolumetricFog> = {
                {
                    let output: Val<::bevy_pbr::VolumetricFog> =
                        <::bevy_pbr::VolumetricFog as ::std::clone::Clone>::clone(&_self).into();
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
            ::bevy_pbr::VolumetricFog,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_volumetric_light_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::VolumetricLight,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::VolumetricLight>| {
            let output: Val<::bevy_pbr::VolumetricLight> = {
                {
                    let output: Val<::bevy_pbr::VolumetricLight> =
                        <::bevy_pbr::VolumetricLight as ::std::clone::Clone>::clone(&_self).into();
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
            ::bevy_pbr::VolumetricLight,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_distance_fog_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::prelude::DistanceFog,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::prelude::DistanceFog>| {
            let output: Val<::bevy_pbr::prelude::DistanceFog> = {
                {
                    let output: Val<::bevy_pbr::prelude::DistanceFog> =
                        <::bevy_pbr::prelude::DistanceFog as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_pbr::prelude::DistanceFog,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_fog_falloff_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::prelude::FogFalloff,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_pbr::prelude::FogFalloff>| {
                let output: Val<::bevy_pbr::prelude::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::prelude::FogFalloff> = <::bevy_pbr::prelude::FogFalloff as ::std::clone::Clone>::clone(
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
            "from_visibility",
            |visibility: f32| {
                let output: Val<::bevy_pbr::prelude::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::prelude::FogFalloff> = ::bevy_pbr::prelude::FogFalloff::from_visibility(
                                visibility,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`FogFalloff::Exponential`] value from the given visibility distance in world units,\n using the revised Koschmieder contrast threshold, [`FogFalloff::REVISED_KOSCHMIEDER_CONTRAST_THRESHOLD`].",
            &["visibility"],
        )
        .register_documented(
            "from_visibility_color",
            |visibility: f32, extinction_inscattering_color: Val<::bevy_color::Color>| {
                let output: Val<::bevy_pbr::prelude::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::prelude::FogFalloff> = ::bevy_pbr::prelude::FogFalloff::from_visibility_color(
                                visibility,
                                extinction_inscattering_color.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`FogFalloff::Atmospheric`] value from the given visibility distance in world units,\n and a shared color for both extinction and inscattering, using the revised Koschmieder contrast threshold,\n [`FogFalloff::REVISED_KOSCHMIEDER_CONTRAST_THRESHOLD`].",
            &["visibility", "extinction_inscattering_color"],
        )
        .register_documented(
            "from_visibility_colors",
            |
                visibility: f32,
                extinction_color: Val<::bevy_color::Color>,
                inscattering_color: Val<::bevy_color::Color>|
            {
                let output: Val<::bevy_pbr::prelude::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::prelude::FogFalloff> = ::bevy_pbr::prelude::FogFalloff::from_visibility_colors(
                                visibility,
                                extinction_color.into_inner(),
                                inscattering_color.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`FogFalloff::Atmospheric`] value from the given visibility distance in world units,\n extinction and inscattering colors, using the revised Koschmieder contrast threshold,\n [`FogFalloff::REVISED_KOSCHMIEDER_CONTRAST_THRESHOLD`].\n ## Tips\n - Alpha values of the provided colors can modulate the `extinction` and `inscattering` effects;\n - Using an `extinction_color` of [`Color::WHITE`] or [`Color::NONE`] disables the extinction effect;\n - Using an `inscattering_color` of [`Color::BLACK`] or [`Color::NONE`] disables the inscattering effect.",
            &["visibility", "extinction_color", "inscattering_color"],
        )
        .register_documented(
            "from_visibility_contrast",
            |visibility: f32, contrast_threshold: f32| {
                let output: Val<::bevy_pbr::prelude::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::prelude::FogFalloff> = ::bevy_pbr::prelude::FogFalloff::from_visibility_contrast(
                                visibility,
                                contrast_threshold,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`FogFalloff::Exponential`] value from the given visibility distance in world units,\n and a given contrast threshold in the range of `0.0` to `1.0`.",
            &["visibility", "contrast_threshold"],
        )
        .register_documented(
            "from_visibility_contrast_color",
            |
                visibility: f32,
                contrast_threshold: f32,
                extinction_inscattering_color: Val<::bevy_color::Color>|
            {
                let output: Val<::bevy_pbr::prelude::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::prelude::FogFalloff> = ::bevy_pbr::prelude::FogFalloff::from_visibility_contrast_color(
                                visibility,
                                contrast_threshold,
                                extinction_inscattering_color.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`FogFalloff::Atmospheric`] value from the given visibility distance in world units,\n a contrast threshold in the range of `0.0` to `1.0`, and a shared color for both extinction and inscattering.",
            &["visibility", "contrast_threshold", "extinction_inscattering_color"],
        )
        .register_documented(
            "from_visibility_contrast_colors",
            |
                visibility: f32,
                contrast_threshold: f32,
                extinction_color: Val<::bevy_color::Color>,
                inscattering_color: Val<::bevy_color::Color>|
            {
                let output: Val<::bevy_pbr::prelude::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::prelude::FogFalloff> = ::bevy_pbr::prelude::FogFalloff::from_visibility_contrast_colors(
                                visibility,
                                contrast_threshold,
                                extinction_color.into_inner(),
                                inscattering_color.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`FogFalloff::Atmospheric`] value from the given visibility distance in world units,\n a contrast threshold in the range of `0.0` to `1.0`, extinction and inscattering colors.\n ## Tips\n - Alpha values of the provided colors can modulate the `extinction` and `inscattering` effects;\n - Using an `extinction_color` of [`Color::WHITE`] or [`Color::NONE`] disables the extinction effect;\n - Using an `inscattering_color` of [`Color::BLACK`] or [`Color::NONE`] disables the inscattering effect.",
            &[
                "visibility",
                "contrast_threshold",
                "extinction_color",
                "inscattering_color",
            ],
        )
        .register_documented(
            "from_visibility_contrast_squared",
            |visibility: f32, contrast_threshold: f32| {
                let output: Val<::bevy_pbr::prelude::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::prelude::FogFalloff> = ::bevy_pbr::prelude::FogFalloff::from_visibility_contrast_squared(
                                visibility,
                                contrast_threshold,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`FogFalloff::ExponentialSquared`] value from the given visibility distance in world units,\n and a given contrast threshold in the range of `0.0` to `1.0`.",
            &["visibility", "contrast_threshold"],
        )
        .register_documented(
            "from_visibility_squared",
            |visibility: f32| {
                let output: Val<::bevy_pbr::prelude::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::prelude::FogFalloff> = ::bevy_pbr::prelude::FogFalloff::from_visibility_squared(
                                visibility,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`FogFalloff::ExponentialSquared`] value from the given visibility distance in world units,\n using the revised Koschmieder contrast threshold, [`FogFalloff::REVISED_KOSCHMIEDER_CONTRAST_THRESHOLD`].",
            &["visibility"],
        )
        .register_documented(
            "koschmieder",
            |v: f32, c_t: f32| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_pbr::prelude::FogFalloff::koschmieder(
                                v,
                                c_t,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Calculates the extinction coefficient β, from V and Cₜ, where:\n - Cₜ is the contrast threshold, in the range of `0.0` to `1.0`\n - V is the visibility distance in which a perfectly black object is still identifiable\n   against the horizon sky within the contrast threshold\n We start with Koschmieder's equation:\n ```text\n       -ln(Cₜ)\n  V = ─────────\n          β\n ```\n Multiplying both sides by β/V, that gives us:\n ```text\n       -ln(Cₜ)\n  β = ─────────\n          V\n ```\n See:\n - <https://en.wikipedia.org/wiki/Visibility>\n - <https://www.biral.com/wp-content/uploads/2015/02/Introduction_to_visibility-v2-2.pdf>",
            &["v", "c_t"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_pbr::prelude::FogFalloff,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ambient_light_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::prelude::AmbientLight,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::prelude::AmbientLight>| {
            let output: Val<::bevy_pbr::prelude::AmbientLight> = {
                {
                    let output: Val<::bevy_pbr::prelude::AmbientLight> =
                        <::bevy_pbr::prelude::AmbientLight as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_pbr::prelude::AmbientLight,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_directional_light_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::prelude::DirectionalLight,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::prelude::DirectionalLight>| {
            let output: Val<::bevy_pbr::prelude::DirectionalLight> = {
                {
                    let output: Val<::bevy_pbr::prelude::DirectionalLight> =
                        <::bevy_pbr::prelude::DirectionalLight as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::prelude::DirectionalLight,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_point_light_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::prelude::PointLight,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::prelude::PointLight>| {
            let output: Val<::bevy_pbr::prelude::PointLight> = {
                {
                    let output: Val<::bevy_pbr::prelude::PointLight> =
                        <::bevy_pbr::prelude::PointLight as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_pbr::prelude::PointLight,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_spot_light_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::prelude::SpotLight,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::prelude::SpotLight>| {
            let output: Val<::bevy_pbr::prelude::SpotLight> = {
                {
                    let output: Val<::bevy_pbr::prelude::SpotLight> =
                        <::bevy_pbr::prelude::SpotLight as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_pbr::prelude::SpotLight,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_environment_map_light_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::prelude::EnvironmentMapLight,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::prelude::EnvironmentMapLight>| {
            let output: Val<::bevy_pbr::prelude::EnvironmentMapLight> = {
                {
                    let output: Val<::bevy_pbr::prelude::EnvironmentMapLight> =
                        <::bevy_pbr::prelude::EnvironmentMapLight as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::prelude::EnvironmentMapLight,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_light_probe_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::prelude::LightProbe,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::prelude::LightProbe>| {
            let output: Val<::bevy_pbr::prelude::LightProbe> = {
                {
                    let output: Val<::bevy_pbr::prelude::LightProbe> =
                        <::bevy_pbr::prelude::LightProbe as ::std::clone::Clone>::clone(&_self)
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
            let output: Val<::bevy_pbr::prelude::LightProbe> = {
                {
                    let output: Val<::bevy_pbr::prelude::LightProbe> =
                        ::bevy_pbr::prelude::LightProbe::new().into();
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
            ::bevy_pbr::prelude::LightProbe,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_parallax_mapping_method_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::prelude::ParallaxMappingMethod,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_pbr::prelude::ParallaxMappingMethod>| {
                let output: () = {
                    {
                        let output: () = <::bevy_pbr::prelude::ParallaxMappingMethod as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_pbr::prelude::ParallaxMappingMethod>| {
                let output: Val<::bevy_pbr::prelude::ParallaxMappingMethod> = {
                    {
                        let output: Val<::bevy_pbr::prelude::ParallaxMappingMethod> = <::bevy_pbr::prelude::ParallaxMappingMethod as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_pbr::prelude::ParallaxMappingMethod>,
                other: Ref<::bevy_pbr::prelude::ParallaxMappingMethod>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_pbr::prelude::ParallaxMappingMethod as ::std::cmp::PartialEq<
                            ::bevy_pbr::prelude::ParallaxMappingMethod,
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
            ::bevy_pbr::prelude::ParallaxMappingMethod,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_standard_material_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::prelude::StandardMaterial,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::prelude::StandardMaterial>| {
            let output: Val<::bevy_pbr::prelude::StandardMaterial> = {
                {
                    let output: Val<::bevy_pbr::prelude::StandardMaterial> =
                        <::bevy_pbr::prelude::StandardMaterial as ::std::clone::Clone>::clone(
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
        "flip",
        |mut _self: Mut<::bevy_pbr::prelude::StandardMaterial>,
         horizontal: bool,
         vertical: bool| {
            let output: () = {
                {
                    let output: () = ::bevy_pbr::prelude::StandardMaterial::flip(
                        &mut _self, horizontal, vertical,
                    )
                    .into();
                    output
                }
            };
            output
        },
        " Flip the texture coordinates of the material.",
        &["_self", "horizontal", "vertical"],
    )
    .register_documented(
        "flipped",
        |_self: Val<::bevy_pbr::prelude::StandardMaterial>, horizontal: bool, vertical: bool| {
            let output: Val<::bevy_pbr::prelude::StandardMaterial> = {
                {
                    let output: Val<::bevy_pbr::prelude::StandardMaterial> =
                        ::bevy_pbr::prelude::StandardMaterial::flipped(
                            _self.into_inner(),
                            horizontal,
                            vertical,
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Consumes the material and returns a material with flipped texture coordinates",
        &["_self", "horizontal", "vertical"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_pbr::prelude::StandardMaterial,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_screen_space_ambient_occlusion_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::ScreenSpaceAmbientOcclusion,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::ScreenSpaceAmbientOcclusion>| {
            let output: Val<::bevy_pbr::ScreenSpaceAmbientOcclusion> = {
                {
                    let output: Val<::bevy_pbr::ScreenSpaceAmbientOcclusion> =
                        <::bevy_pbr::ScreenSpaceAmbientOcclusion as ::std::clone::Clone>::clone(
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
        |_self: Ref<::bevy_pbr::ScreenSpaceAmbientOcclusion>,
         other: Ref<::bevy_pbr::ScreenSpaceAmbientOcclusion>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_pbr::ScreenSpaceAmbientOcclusion as ::std::cmp::PartialEq<
                            ::bevy_pbr::ScreenSpaceAmbientOcclusion,
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
            ::bevy_pbr::ScreenSpaceAmbientOcclusion,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_screen_space_reflections_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::ScreenSpaceReflections,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::ScreenSpaceReflections>| {
            let output: Val<::bevy_pbr::ScreenSpaceReflections> = {
                {
                    let output: Val<::bevy_pbr::ScreenSpaceReflections> =
                        <::bevy_pbr::ScreenSpaceReflections as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_pbr::ScreenSpaceReflections,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cascade_shadow_config_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::CascadeShadowConfig,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::CascadeShadowConfig>| {
            let output: Val<::bevy_pbr::CascadeShadowConfig> = {
                {
                    let output: Val<::bevy_pbr::CascadeShadowConfig> =
                        <::bevy_pbr::CascadeShadowConfig as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_pbr::CascadeShadowConfig,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cascades_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::Cascades,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_pbr::Cascades>| {
                let output: Val<::bevy_pbr::Cascades> = {
                    {
                        let output: Val<::bevy_pbr::Cascades> = <::bevy_pbr::Cascades as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::Cascades,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cascades_visible_entities_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::CascadesVisibleEntities,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::CascadesVisibleEntities>| {
            let output: Val<::bevy_pbr::CascadesVisibleEntities> = {
                {
                    let output: Val<::bevy_pbr::CascadesVisibleEntities> =
                        <::bevy_pbr::CascadesVisibleEntities as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_pbr::CascadesVisibleEntities,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_visible_mesh_entities_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::VisibleMeshEntities,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::VisibleMeshEntities>| {
            let output: Val<::bevy_pbr::VisibleMeshEntities> = {
                {
                    let output: Val<::bevy_pbr::VisibleMeshEntities> =
                        <::bevy_pbr::VisibleMeshEntities as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_pbr::VisibleMeshEntities,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cluster_config_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::ClusterConfig,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::ClusterConfig>| {
            let output: Val<::bevy_pbr::ClusterConfig> = {
                {
                    let output: Val<::bevy_pbr::ClusterConfig> =
                        <::bevy_pbr::ClusterConfig as ::std::clone::Clone>::clone(&_self).into();
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
            ::bevy_pbr::ClusterConfig,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cubemap_visible_entities_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::CubemapVisibleEntities,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::CubemapVisibleEntities>| {
            let output: Val<::bevy_pbr::CubemapVisibleEntities> = {
                {
                    let output: Val<::bevy_pbr::CubemapVisibleEntities> =
                        <::bevy_pbr::CubemapVisibleEntities as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_pbr::CubemapVisibleEntities,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_directional_light_shadow_map_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::DirectionalLightShadowMap,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::DirectionalLightShadowMap>| {
            let output: Val<::bevy_pbr::DirectionalLightShadowMap> = {
                {
                    let output: Val<::bevy_pbr::DirectionalLightShadowMap> =
                        <::bevy_pbr::DirectionalLightShadowMap as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::DirectionalLightShadowMap,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_not_shadow_caster_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::NotShadowCaster,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_pbr::NotShadowCaster,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_not_shadow_receiver_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::NotShadowReceiver,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_pbr::NotShadowReceiver,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_point_light_shadow_map_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::PointLightShadowMap,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::PointLightShadowMap>| {
            let output: Val<::bevy_pbr::PointLightShadowMap> = {
                {
                    let output: Val<::bevy_pbr::PointLightShadowMap> =
                        <::bevy_pbr::PointLightShadowMap as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_pbr::PointLightShadowMap,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_shadow_filtering_method_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::ShadowFilteringMethod,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_pbr::ShadowFilteringMethod>| {
                let output: () = {
                    {
                        let output: () = <::bevy_pbr::ShadowFilteringMethod as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_pbr::ShadowFilteringMethod>| {
                let output: Val<::bevy_pbr::ShadowFilteringMethod> = {
                    {
                        let output: Val<::bevy_pbr::ShadowFilteringMethod> = <::bevy_pbr::ShadowFilteringMethod as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_pbr::ShadowFilteringMethod>,
                other: Ref<::bevy_pbr::ShadowFilteringMethod>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_pbr::ShadowFilteringMethod as ::std::cmp::PartialEq<
                            ::bevy_pbr::ShadowFilteringMethod,
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
            ::bevy_pbr::ShadowFilteringMethod,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_default_opaque_renderer_method_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::DefaultOpaqueRendererMethod,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::DefaultOpaqueRendererMethod>| {
            let output: Val<::bevy_pbr::DefaultOpaqueRendererMethod> = {
                {
                    let output: Val<::bevy_pbr::DefaultOpaqueRendererMethod> =
                        <::bevy_pbr::DefaultOpaqueRendererMethod as ::std::clone::Clone>::clone(
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
        "deferred",
        || {
            let output: Val<::bevy_pbr::DefaultOpaqueRendererMethod> = {
                {
                    let output: Val<::bevy_pbr::DefaultOpaqueRendererMethod> =
                        ::bevy_pbr::DefaultOpaqueRendererMethod::deferred().into();
                    output
                }
            };
            output
        },
        "",
        &[],
    )
    .register_documented(
        "forward",
        || {
            let output: Val<::bevy_pbr::DefaultOpaqueRendererMethod> = {
                {
                    let output: Val<::bevy_pbr::DefaultOpaqueRendererMethod> =
                        ::bevy_pbr::DefaultOpaqueRendererMethod::forward().into();
                    output
                }
            };
            output
        },
        "",
        &[],
    )
    .register_documented(
        "set_to_deferred",
        |mut _self: Mut<::bevy_pbr::DefaultOpaqueRendererMethod>| {
            let output: () = {
                {
                    let output: () =
                        ::bevy_pbr::DefaultOpaqueRendererMethod::set_to_deferred(&mut _self).into();
                    output
                }
            };
            output
        },
        "",
        &["_self"],
    )
    .register_documented(
        "set_to_forward",
        |mut _self: Mut<::bevy_pbr::DefaultOpaqueRendererMethod>| {
            let output: () = {
                {
                    let output: () =
                        ::bevy_pbr::DefaultOpaqueRendererMethod::set_to_forward(&mut _self).into();
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
            ::bevy_pbr::DefaultOpaqueRendererMethod,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_material_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::wireframe::WireframeMaterial,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::wireframe::WireframeMaterial>| {
            let output: Val<::bevy_pbr::wireframe::WireframeMaterial> = {
                {
                    let output: Val<::bevy_pbr::wireframe::WireframeMaterial> =
                        <::bevy_pbr::wireframe::WireframeMaterial as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::wireframe::WireframeMaterial,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_no_wireframe_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::wireframe::NoWireframe,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_pbr::wireframe::NoWireframe>| {
                let output: () = {
                    {
                        let output: () = <::bevy_pbr::wireframe::NoWireframe as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_pbr::wireframe::NoWireframe>| {
                let output: Val<::bevy_pbr::wireframe::NoWireframe> = {
                    {
                        let output: Val<::bevy_pbr::wireframe::NoWireframe> = <::bevy_pbr::wireframe::NoWireframe as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_pbr::wireframe::NoWireframe>,
                other: Ref<::bevy_pbr::wireframe::NoWireframe>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_pbr::wireframe::NoWireframe as ::std::cmp::PartialEq<
                            ::bevy_pbr::wireframe::NoWireframe,
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
            ::bevy_pbr::wireframe::NoWireframe,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_config_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::wireframe::WireframeConfig,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::wireframe::WireframeConfig>| {
            let output: Val<::bevy_pbr::wireframe::WireframeConfig> = {
                {
                    let output: Val<::bevy_pbr::wireframe::WireframeConfig> =
                        <::bevy_pbr::wireframe::WireframeConfig as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::wireframe::WireframeConfig,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_color_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::wireframe::WireframeColor,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::wireframe::WireframeColor>| {
            let output: Val<::bevy_pbr::wireframe::WireframeColor> = {
                {
                    let output: Val<::bevy_pbr::wireframe::WireframeColor> =
                        <::bevy_pbr::wireframe::WireframeColor as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::wireframe::WireframeColor,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::wireframe::Wireframe,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_pbr::wireframe::Wireframe>| {
                let output: () = {
                    {
                        let output: () = <::bevy_pbr::wireframe::Wireframe as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_pbr::wireframe::Wireframe>| {
                let output: Val<::bevy_pbr::wireframe::Wireframe> = {
                    {
                        let output: Val<::bevy_pbr::wireframe::Wireframe> = <::bevy_pbr::wireframe::Wireframe as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_pbr::wireframe::Wireframe>,
                other: Ref<::bevy_pbr::wireframe::Wireframe>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_pbr::wireframe::Wireframe as ::std::cmp::PartialEq<
                            ::bevy_pbr::wireframe::Wireframe,
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
            ::bevy_pbr::wireframe::Wireframe,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_mesh_3_d_wireframe_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::wireframe::Mesh3dWireframe,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_pbr::wireframe::Mesh3dWireframe>| {
                let output: () = {
                    {
                        let output: () = <::bevy_pbr::wireframe::Mesh3dWireframe as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_pbr::wireframe::Mesh3dWireframe>| {
                let output: Val<::bevy_pbr::wireframe::Mesh3dWireframe> = {
                    {
                        let output: Val<::bevy_pbr::wireframe::Mesh3dWireframe> = <::bevy_pbr::wireframe::Mesh3dWireframe as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_pbr::wireframe::Mesh3dWireframe>,
                other: Ref<::bevy_pbr::wireframe::Mesh3dWireframe>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_pbr::wireframe::Mesh3dWireframe as ::std::cmp::PartialEq<
                            ::bevy_pbr::wireframe::Mesh3dWireframe,
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
            ::bevy_pbr::wireframe::Mesh3dWireframe,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_atmosphere_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::Atmosphere,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::Atmosphere>| {
            let output: Val<::bevy_pbr::Atmosphere> = {
                {
                    let output: Val<::bevy_pbr::Atmosphere> =
                        <::bevy_pbr::Atmosphere as ::std::clone::Clone>::clone(&_self).into();
                    output
                }
            };
            output
        },
        "",
        &["_self"],
    )
    .register_documented(
        "with_density_multiplier",
        |_self: Val<::bevy_pbr::Atmosphere>, mult: f32| {
            let output: Val<::bevy_pbr::Atmosphere> = {
                {
                    let output: Val<::bevy_pbr::Atmosphere> =
                        ::bevy_pbr::Atmosphere::with_density_multiplier(_self.into_inner(), mult)
                            .into();
                    output
                }
            };
            output
        },
        "",
        &["_self", "mult"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_pbr::Atmosphere,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_atmosphere_settings_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::AtmosphereSettings,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::AtmosphereSettings>| {
            let output: Val<::bevy_pbr::AtmosphereSettings> = {
                {
                    let output: Val<::bevy_pbr::AtmosphereSettings> =
                        <::bevy_pbr::AtmosphereSettings as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_pbr::AtmosphereSettings,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cluster_far_z_mode_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::ClusterFarZMode,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::ClusterFarZMode>| {
            let output: Val<::bevy_pbr::ClusterFarZMode> = {
                {
                    let output: Val<::bevy_pbr::ClusterFarZMode> =
                        <::bevy_pbr::ClusterFarZMode as ::std::clone::Clone>::clone(&_self).into();
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
            ::bevy_pbr::ClusterFarZMode,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cluster_z_config_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::ClusterZConfig,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::ClusterZConfig>| {
            let output: Val<::bevy_pbr::ClusterZConfig> = {
                {
                    let output: Val<::bevy_pbr::ClusterZConfig> =
                        <::bevy_pbr::ClusterZConfig as ::std::clone::Clone>::clone(&_self).into();
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
            ::bevy_pbr::ClusterZConfig,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_clustered_decal_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::decal::clustered::ClusteredDecal,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_pbr::decal::clustered::ClusteredDecal>| {
                let output: Val<::bevy_pbr::decal::clustered::ClusteredDecal> = {
                    {
                        let output: Val<::bevy_pbr::decal::clustered::ClusteredDecal> = <::bevy_pbr::decal::clustered::ClusteredDecal as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::decal::clustered::ClusteredDecal,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_irradiance_volume_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::irradiance_volume::IrradianceVolume,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_pbr::irradiance_volume::IrradianceVolume>| {
                let output: Val<::bevy_pbr::irradiance_volume::IrradianceVolume> = {
                    {
                        let output: Val<
                            ::bevy_pbr::irradiance_volume::IrradianceVolume,
                        > = <::bevy_pbr::irradiance_volume::IrradianceVolume as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::irradiance_volume::IrradianceVolume,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_visible_mesh_entities_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::RenderVisibleMeshEntities,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::RenderVisibleMeshEntities>| {
            let output: Val<::bevy_pbr::RenderVisibleMeshEntities> = {
                {
                    let output: Val<::bevy_pbr::RenderVisibleMeshEntities> =
                        <::bevy_pbr::RenderVisibleMeshEntities as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::RenderVisibleMeshEntities,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_cubemap_visible_entities_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::RenderCubemapVisibleEntities,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::RenderCubemapVisibleEntities>| {
            let output: Val<::bevy_pbr::RenderCubemapVisibleEntities> = {
                {
                    let output: Val<::bevy_pbr::RenderCubemapVisibleEntities> =
                        <::bevy_pbr::RenderCubemapVisibleEntities as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::RenderCubemapVisibleEntities,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_cascades_visible_entities_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::RenderCascadesVisibleEntities,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::RenderCascadesVisibleEntities>| {
            let output: Val<::bevy_pbr::RenderCascadesVisibleEntities> = {
                {
                    let output: Val<::bevy_pbr::RenderCascadesVisibleEntities> =
                        <::bevy_pbr::RenderCascadesVisibleEntities as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::RenderCascadesVisibleEntities,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_forward_decal_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::decal::ForwardDecal,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_pbr::decal::ForwardDecal,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_opaque_renderer_method_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::OpaqueRendererMethod,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::OpaqueRendererMethod>| {
            let output: Val<::bevy_pbr::OpaqueRendererMethod> = {
                {
                    let output: Val<::bevy_pbr::OpaqueRendererMethod> =
                        <::bevy_pbr::OpaqueRendererMethod as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_pbr::OpaqueRendererMethod>,
         other: Ref<::bevy_pbr::OpaqueRendererMethod>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_pbr::OpaqueRendererMethod as ::std::cmp::PartialEq<
                            ::bevy_pbr::OpaqueRendererMethod,
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
            ::bevy_pbr::OpaqueRendererMethod,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cascade_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::Cascade,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_pbr::Cascade>| {
                let output: Val<::bevy_pbr::Cascade> = {
                    {
                        let output: Val<::bevy_pbr::Cascade> = <::bevy_pbr::Cascade as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::Cascade,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_transmitted_shadow_receiver_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::TransmittedShadowReceiver,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_pbr::TransmittedShadowReceiver,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_lightmap_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::Lightmap,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_pbr::Lightmap>| {
                let output: Val<::bevy_pbr::Lightmap> = {
                    {
                        let output: Val<::bevy_pbr::Lightmap> = <::bevy_pbr::Lightmap as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::Lightmap,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_material_binding_id_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::MaterialBindingId,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::MaterialBindingId>| {
            let output: Val<::bevy_pbr::MaterialBindingId> = {
                {
                    let output: Val<::bevy_pbr::MaterialBindingId> =
                        <::bevy_pbr::MaterialBindingId as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_pbr::MaterialBindingId,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_material_bind_group_slot_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::MaterialBindGroupSlot,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::MaterialBindGroupSlot>| {
            let output: Val<::bevy_pbr::MaterialBindGroupSlot> = {
                {
                    let output: Val<::bevy_pbr::MaterialBindGroupSlot> =
                        <::bevy_pbr::MaterialBindGroupSlot as ::std::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_pbr::MaterialBindGroupSlot>,
         other: Ref<::bevy_pbr::MaterialBindGroupSlot>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_pbr::MaterialBindGroupSlot as ::std::cmp::PartialEq<
                            ::bevy_pbr::MaterialBindGroupSlot,
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
            ::bevy_pbr::MaterialBindGroupSlot,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_material_bind_group_index_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::MaterialBindGroupIndex,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_pbr::MaterialBindGroupIndex>| {
                let output: () = {
                    {
                        let output: () = <::bevy_pbr::MaterialBindGroupIndex as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_pbr::MaterialBindGroupIndex>| {
                let output: Val<::bevy_pbr::MaterialBindGroupIndex> = {
                    {
                        let output: Val<::bevy_pbr::MaterialBindGroupIndex> = <::bevy_pbr::MaterialBindGroupIndex as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_pbr::MaterialBindGroupIndex>,
                other: Ref<::bevy_pbr::MaterialBindGroupIndex>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_pbr::MaterialBindGroupIndex as ::std::cmp::PartialEq<
                            ::bevy_pbr::MaterialBindGroupIndex,
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
            ::bevy_pbr::MaterialBindGroupIndex,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_uv_channel_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::UvChannel,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_pbr::UvChannel>| {
                let output: () = {
                    {
                        let output: () = <::bevy_pbr::UvChannel as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_pbr::UvChannel>| {
                let output: Val<::bevy_pbr::UvChannel> = {
                    {
                        let output: Val<::bevy_pbr::UvChannel> = <::bevy_pbr::UvChannel as ::std::clone::Clone>::clone(
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
            |_self: Ref<::bevy_pbr::UvChannel>, other: Ref<::bevy_pbr::UvChannel>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_pbr::UvChannel as ::std::cmp::PartialEq<
                            ::bevy_pbr::UvChannel,
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
            ::bevy_pbr::UvChannel,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_screen_space_ambient_occlusion_quality_level_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::ScreenSpaceAmbientOcclusionQualityLevel,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_pbr::ScreenSpaceAmbientOcclusionQualityLevel>| {
                let output: () = {
                    {
                        let output: () = <::bevy_pbr::ScreenSpaceAmbientOcclusionQualityLevel as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_pbr::ScreenSpaceAmbientOcclusionQualityLevel>| {
                let output: Val<::bevy_pbr::ScreenSpaceAmbientOcclusionQualityLevel> = {
                    {
                        let output: Val<
                            ::bevy_pbr::ScreenSpaceAmbientOcclusionQualityLevel,
                        > = <::bevy_pbr::ScreenSpaceAmbientOcclusionQualityLevel as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_pbr::ScreenSpaceAmbientOcclusionQualityLevel>,
                other: Ref<::bevy_pbr::ScreenSpaceAmbientOcclusionQualityLevel>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_pbr::ScreenSpaceAmbientOcclusionQualityLevel as ::std::cmp::PartialEq<
                            ::bevy_pbr::ScreenSpaceAmbientOcclusionQualityLevel,
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
            ::bevy_pbr::ScreenSpaceAmbientOcclusionQualityLevel,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyPbrScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_fog_volume_functions(&mut world);
        register_volumetric_fog_functions(&mut world);
        register_volumetric_light_functions(&mut world);
        register_distance_fog_functions(&mut world);
        register_fog_falloff_functions(&mut world);
        register_ambient_light_functions(&mut world);
        register_directional_light_functions(&mut world);
        register_point_light_functions(&mut world);
        register_spot_light_functions(&mut world);
        register_environment_map_light_functions(&mut world);
        register_light_probe_functions(&mut world);
        register_parallax_mapping_method_functions(&mut world);
        register_standard_material_functions(&mut world);
        register_screen_space_ambient_occlusion_functions(&mut world);
        register_screen_space_reflections_functions(&mut world);
        register_cascade_shadow_config_functions(&mut world);
        register_cascades_functions(&mut world);
        register_cascades_visible_entities_functions(&mut world);
        register_visible_mesh_entities_functions(&mut world);
        register_cluster_config_functions(&mut world);
        register_cubemap_visible_entities_functions(&mut world);
        register_directional_light_shadow_map_functions(&mut world);
        register_not_shadow_caster_functions(&mut world);
        register_not_shadow_receiver_functions(&mut world);
        register_point_light_shadow_map_functions(&mut world);
        register_shadow_filtering_method_functions(&mut world);
        register_default_opaque_renderer_method_functions(&mut world);
        register_wireframe_material_functions(&mut world);
        register_no_wireframe_functions(&mut world);
        register_wireframe_config_functions(&mut world);
        register_wireframe_color_functions(&mut world);
        register_wireframe_functions(&mut world);
        register_mesh_3_d_wireframe_functions(&mut world);
        register_atmosphere_functions(&mut world);
        register_atmosphere_settings_functions(&mut world);
        register_cluster_far_z_mode_functions(&mut world);
        register_cluster_z_config_functions(&mut world);
        register_clustered_decal_functions(&mut world);
        register_irradiance_volume_functions(&mut world);
        register_render_visible_mesh_entities_functions(&mut world);
        register_render_cubemap_visible_entities_functions(&mut world);
        register_render_cascades_visible_entities_functions(&mut world);
        register_forward_decal_functions(&mut world);
        register_opaque_renderer_method_functions(&mut world);
        register_cascade_functions(&mut world);
        register_transmitted_shadow_receiver_functions(&mut world);
        register_lightmap_functions(&mut world);
        register_material_binding_id_functions(&mut world);
        register_material_bind_group_slot_functions(&mut world);
        register_material_bind_group_index_functions(&mut world);
        register_uv_channel_functions(&mut world);
        register_screen_space_ambient_occlusion_quality_level_functions(&mut world);
    }
}
