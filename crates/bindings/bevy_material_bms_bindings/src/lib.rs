
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
extern crate std;


use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{R, M, V},
        namespace::NamespaceBuilder,
    },
};
use bevy_ecs::prelude::*;
use bevy_app::{App, Plugin};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyMaterialScriptingPlugin;
pub(crate) fn register_alpha_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_material::AlphaMode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_material::AlphaMode>| {
                let output: V<::bevy_material::AlphaMode> = {
                    {
                        let output: V<::bevy_material::AlphaMode> = <::bevy_material::AlphaMode as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_material::AlphaMode>, other: R<::bevy_material::AlphaMode>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_material::AlphaMode as ::std::cmp::PartialEq<
                            ::bevy_material::AlphaMode,
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
            ::bevy_material::AlphaMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_opaque_renderer_method_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_material::OpaqueRendererMethod,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_material::OpaqueRendererMethod>| {
                let output: V<::bevy_material::OpaqueRendererMethod> = {
                    {
                        let output: V<::bevy_material::OpaqueRendererMethod> = <::bevy_material::OpaqueRendererMethod as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_material::OpaqueRendererMethod>,
                other: R<::bevy_material::OpaqueRendererMethod>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_material::OpaqueRendererMethod as ::std::cmp::PartialEq<
                            ::bevy_material::OpaqueRendererMethod,
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
            ::bevy_material::OpaqueRendererMethod,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyMaterialScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_alpha_mode_functions(&mut world);
        register_opaque_renderer_method_functions(&mut world);
    }
}
