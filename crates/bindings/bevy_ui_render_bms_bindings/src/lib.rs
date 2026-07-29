
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
pub struct BevyUiRenderScriptingPlugin;
pub(crate) fn register_global_ui_debug_options_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_render::GlobalUiDebugOptions,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_render::GlobalUiDebugOptions>| {
                let output: V<::bevy_ui_render::GlobalUiDebugOptions> = {
                    {
                        let output: V<::bevy_ui_render::GlobalUiDebugOptions> = <::bevy_ui_render::GlobalUiDebugOptions as ::std::clone::Clone>::clone(
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
            "toggle",
            |mut _self: M<::bevy_ui_render::GlobalUiDebugOptions>| {
                let output: () = {
                    {
                        let output: () = ::bevy_ui_render::GlobalUiDebugOptions::toggle(
                                &mut _self,
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
            ::bevy_ui_render::GlobalUiDebugOptions,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_debug_options_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_render::UiDebugOptions,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_render::UiDebugOptions>| {
                let output: V<::bevy_ui_render::UiDebugOptions> = {
                    {
                        let output: V<::bevy_ui_render::UiDebugOptions> = <::bevy_ui_render::UiDebugOptions as ::std::clone::Clone>::clone(
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
            "toggle",
            |mut _self: M<::bevy_ui_render::UiDebugOptions>| {
                let output: () = {
                    {
                        let output: () = ::bevy_ui_render::UiDebugOptions::toggle(
                                &mut _self,
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
            ::bevy_ui_render::UiDebugOptions,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_box_shadow_samples_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_render::BoxShadowSamples,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_render::BoxShadowSamples>| {
                let output: V<::bevy_ui_render::BoxShadowSamples> = {
                    {
                        let output: V<::bevy_ui_render::BoxShadowSamples> = <::bevy_ui_render::BoxShadowSamples as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_ui_render::BoxShadowSamples>,
                other: R<::bevy_ui_render::BoxShadowSamples>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui_render::BoxShadowSamples as ::std::cmp::PartialEq<
                            ::bevy_ui_render::BoxShadowSamples,
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
            ::bevy_ui_render::BoxShadowSamples,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ui_anti_alias_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_render::UiAntiAlias,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_ui_render::UiAntiAlias>| {
                let output: V<::bevy_ui_render::UiAntiAlias> = {
                    {
                        let output: V<::bevy_ui_render::UiAntiAlias> = <::bevy_ui_render::UiAntiAlias as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_ui_render::UiAntiAlias>,
                other: R<::bevy_ui_render::UiAntiAlias>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_ui_render::UiAntiAlias as ::std::cmp::PartialEq<
                            ::bevy_ui_render::UiAntiAlias,
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
            ::bevy_ui_render::UiAntiAlias,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyUiRenderScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_global_ui_debug_options_functions(&mut world);
        register_ui_debug_options_functions(&mut world);
        register_box_shadow_samples_functions(&mut world);
        register_ui_anti_alias_functions(&mut world);
    }
}
