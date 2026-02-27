
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]



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
pub(crate) fn register_box_shadow_samples_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_ui_render::BoxShadowSamples,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: R<::bevy_ui_render::BoxShadowSamples>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui_render::BoxShadowSamples as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
            "assert_receiver_is_total_eq",
            |_self: R<::bevy_ui_render::UiAntiAlias>| {
                let output: () = {
                    {
                        let output: () = <::bevy_ui_render::UiAntiAlias as ::std::cmp::Eq>::assert_receiver_is_total_eq(
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
        register_box_shadow_samples_functions(&mut world);
        register_ui_anti_alias_functions(&mut world);
    }
}
