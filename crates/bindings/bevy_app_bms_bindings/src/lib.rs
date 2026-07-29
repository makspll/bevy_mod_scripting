
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
pub struct BevyAppScriptingPlugin;
pub(crate) fn register_app_exit_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_app::AppExit,
    >::new(world)
        .register_documented(
            "assert_fields_are_eq",
            |_self: R<::bevy_app::AppExit>| {
                let output: () = {
                    {
                        let output: () = <::bevy_app::AppExit as ::core::cmp::Eq>::assert_fields_are_eq(
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
            |_self: R<::bevy_app::AppExit>| {
                let output: V<::bevy_app::AppExit> = {
                    {
                        let output: V<::bevy_app::AppExit> = <::bevy_app::AppExit as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_app::AppExit>, other: R<::bevy_app::AppExit>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_app::AppExit as ::core::cmp::PartialEq<
                            ::bevy_app::AppExit,
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
            "error",
            || {
                let output: V<::bevy_app::AppExit> = {
                    {
                        let output: V<::bevy_app::AppExit> = ::bevy_app::AppExit::error()
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`AppExit::Error`] with an error code of 1.",
            &[],
        )
        .register_documented(
            "from_code",
            |code: u8| {
                let output: V<::bevy_app::AppExit> = {
                    {
                        let output: V<::bevy_app::AppExit> = ::bevy_app::AppExit::from_code(
                                code,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`AppExit`] from a code.\n When `code` is 0 a [`AppExit::Success`] is constructed otherwise a\n [`AppExit::Error`] is constructed.",
            &["code"],
        )
        .register_documented(
            "is_error",
            |_self: R<::bevy_app::AppExit>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_app::AppExit::is_error(&_self).into();
                        output
                    }
                };
                output
            },
            " Returns `true` if `self` is a [`AppExit::Error`].",
            &["_self"],
        )
        .register_documented(
            "is_success",
            |_self: R<::bevy_app::AppExit>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_app::AppExit::is_success(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if `self` is a [`AppExit::Success`].",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_app::AppExit,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyAppScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_app_exit_functions(&mut world);
    }
}
