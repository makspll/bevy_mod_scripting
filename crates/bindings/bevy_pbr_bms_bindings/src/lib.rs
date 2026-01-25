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
pub struct BevyPbrScriptingPlugin;
pub(crate) fn register_distance_fog_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::DistanceFog,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_pbr::DistanceFog>| {
                let output: Val<::bevy_pbr::DistanceFog> = {
                    {
                        let output: Val<::bevy_pbr::DistanceFog> = <::bevy_pbr::DistanceFog as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::DistanceFog,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_fog_falloff_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::FogFalloff,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_pbr::FogFalloff>| {
                let output: Val<::bevy_pbr::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::FogFalloff> = <::bevy_pbr::FogFalloff as ::std::clone::Clone>::clone(
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
                let output: Val<::bevy_pbr::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::FogFalloff> = ::bevy_pbr::FogFalloff::from_visibility(
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
                let output: Val<::bevy_pbr::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::FogFalloff> = ::bevy_pbr::FogFalloff::from_visibility_color(
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
                let output: Val<::bevy_pbr::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::FogFalloff> = ::bevy_pbr::FogFalloff::from_visibility_colors(
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
                let output: Val<::bevy_pbr::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::FogFalloff> = ::bevy_pbr::FogFalloff::from_visibility_contrast(
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
                let output: Val<::bevy_pbr::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::FogFalloff> = ::bevy_pbr::FogFalloff::from_visibility_contrast_color(
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
                let output: Val<::bevy_pbr::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::FogFalloff> = ::bevy_pbr::FogFalloff::from_visibility_contrast_colors(
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
                let output: Val<::bevy_pbr::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::FogFalloff> = ::bevy_pbr::FogFalloff::from_visibility_contrast_squared(
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
                let output: Val<::bevy_pbr::FogFalloff> = {
                    {
                        let output: Val<::bevy_pbr::FogFalloff> = ::bevy_pbr::FogFalloff::from_visibility_squared(
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
                        let output: f32 = ::bevy_pbr::FogFalloff::koschmieder(v, c_t)
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
        .register_type_data::<::bevy_pbr::FogFalloff, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_parallax_mapping_method_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::ParallaxMappingMethod,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_pbr::ParallaxMappingMethod>| {
                let output: () = {
                    {
                        let output: () = <::bevy_pbr::ParallaxMappingMethod as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_pbr::ParallaxMappingMethod>| {
                let output: Val<::bevy_pbr::ParallaxMappingMethod> = {
                    {
                        let output: Val<::bevy_pbr::ParallaxMappingMethod> = <::bevy_pbr::ParallaxMappingMethod as ::std::clone::Clone>::clone(
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
                _self: Ref<::bevy_pbr::ParallaxMappingMethod>,
                other: Ref<::bevy_pbr::ParallaxMappingMethod>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_pbr::ParallaxMappingMethod as ::std::cmp::PartialEq<
                            ::bevy_pbr::ParallaxMappingMethod,
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
            ::bevy_pbr::ParallaxMappingMethod,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_standard_material_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::StandardMaterial,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::StandardMaterial>| {
            let output: Val<::bevy_pbr::StandardMaterial> = {
                {
                    let output: Val<::bevy_pbr::StandardMaterial> =
                        <::bevy_pbr::StandardMaterial as ::std::clone::Clone>::clone(&_self).into();
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
        |mut _self: Mut<::bevy_pbr::StandardMaterial>, horizontal: bool, vertical: bool| {
            let output: () = {
                {
                    let output: () =
                        ::bevy_pbr::StandardMaterial::flip(&mut _self, horizontal, vertical).into();
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
        |_self: Val<::bevy_pbr::StandardMaterial>, horizontal: bool, vertical: bool| {
            let output: Val<::bevy_pbr::StandardMaterial> = {
                {
                    let output: Val<::bevy_pbr::StandardMaterial> =
                        ::bevy_pbr::StandardMaterial::flipped(
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
            ::bevy_pbr::StandardMaterial,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_screen_space_ambient_occlusion_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_screen_space_reflections_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_default_opaque_renderer_method_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_material_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_config_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_wireframe_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_no_wireframe_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_mesh_3_d_wireframe_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_gpu_atmosphere_settings_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::GpuAtmosphereSettings,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_pbr::GpuAtmosphereSettings>| {
            let output: Val<::bevy_pbr::GpuAtmosphereSettings> = {
                {
                    let output: Val<::bevy_pbr::GpuAtmosphereSettings> =
                        <::bevy_pbr::GpuAtmosphereSettings as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_pbr::GpuAtmosphereSettings,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_atmosphere_settings_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_atmosphere_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::AtmosphereMode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_pbr::AtmosphereMode>| {
                let output: Val<::bevy_pbr::AtmosphereMode> = {
                    {
                        let output: Val<::bevy_pbr::AtmosphereMode> = <::bevy_pbr::AtmosphereMode as ::std::clone::Clone>::clone(
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
            ::bevy_pbr::AtmosphereMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_visible_mesh_entities_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_cubemap_visible_entities_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_render_cascades_visible_entities_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_forward_decal_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_pbr::decal::ForwardDecal,
    >::new(world);
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_pbr::decal::ForwardDecal,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_opaque_renderer_method_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_lightmap_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
        .register_type_data::<::bevy_pbr::Lightmap, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_material_binding_id_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_material_bind_group_slot_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_material_bind_group_index_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_uv_channel_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
        .register_type_data::<::bevy_pbr::UvChannel, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_screen_space_ambient_occlusion_quality_level_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
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
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyPbrScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_distance_fog_functions(&mut world);
        register_fog_falloff_functions(&mut world);
        register_parallax_mapping_method_functions(&mut world);
        register_standard_material_functions(&mut world);
        register_screen_space_ambient_occlusion_functions(&mut world);
        register_screen_space_reflections_functions(&mut world);
        register_default_opaque_renderer_method_functions(&mut world);
        register_wireframe_material_functions(&mut world);
        register_wireframe_config_functions(&mut world);
        register_wireframe_functions(&mut world);
        register_wireframe_color_functions(&mut world);
        register_no_wireframe_functions(&mut world);
        register_mesh_3_d_wireframe_functions(&mut world);
        register_gpu_atmosphere_settings_functions(&mut world);
        register_atmosphere_settings_functions(&mut world);
        register_atmosphere_mode_functions(&mut world);
        register_render_visible_mesh_entities_functions(&mut world);
        register_render_cubemap_visible_entities_functions(&mut world);
        register_render_cascades_visible_entities_functions(&mut world);
        register_forward_decal_functions(&mut world);
        register_opaque_renderer_method_functions(&mut world);
        register_lightmap_functions(&mut world);
        register_material_binding_id_functions(&mut world);
        register_material_bind_group_slot_functions(&mut world);
        register_material_bind_group_index_functions(&mut world);
        register_uv_channel_functions(&mut world);
        register_screen_space_ambient_occlusion_quality_level_functions(&mut world);
    }
}
