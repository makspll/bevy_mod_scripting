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
pub struct BevyA11YScriptingPlugin;
pub(crate) fn register_accessibility_requested_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_a11y::AccessibilityRequested,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_a11y::AccessibilityRequested>| {
                let output: Val<::bevy_a11y::AccessibilityRequested> = {
                    {
                        let output: Val<::bevy_a11y::AccessibilityRequested> = <::bevy_a11y::AccessibilityRequested as ::core::clone::Clone>::clone(
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
            "get",
            |_self: Ref<::bevy_a11y::AccessibilityRequested>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_a11y::AccessibilityRequested::get(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if an access technology is active and accessibility tree\n updates should be sent.",
            &["_self"],
        )
        .register_documented(
            "set",
            |_self: Ref<::bevy_a11y::AccessibilityRequested>, value: bool| {
                let output: () = {
                    {
                        let output: () = ::bevy_a11y::AccessibilityRequested::set(
                                &_self,
                                value,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets whether accessibility updates were requested by an access technology.",
            &["_self", "value"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_a11y::AccessibilityRequested,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_manage_accessibility_updates_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_a11y::ManageAccessibilityUpdates,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_a11y::ManageAccessibilityUpdates>| {
            let output: Val<::bevy_a11y::ManageAccessibilityUpdates> = {
                {
                    let output: Val<::bevy_a11y::ManageAccessibilityUpdates> =
                        <::bevy_a11y::ManageAccessibilityUpdates as ::core::clone::Clone>::clone(
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
        "get",
        |_self: Ref<::bevy_a11y::ManageAccessibilityUpdates>| {
            let output: bool = {
                {
                    let output: bool = ::bevy_a11y::ManageAccessibilityUpdates::get(&_self).into();
                    output
                }
            };
            output
        },
        " Returns `true` if the ECS should update the accessibility tree.",
        &["_self"],
    )
    .register_documented(
        "set",
        |mut _self: Mut<::bevy_a11y::ManageAccessibilityUpdates>, value: bool| {
            let output: () = {
                {
                    let output: () =
                        ::bevy_a11y::ManageAccessibilityUpdates::set(&mut _self, value).into();
                    output
                }
            };
            output
        },
        " Sets whether the ECS should update the accessibility tree.",
        &["_self", "value"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_a11y::ManageAccessibilityUpdates,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_accessibility_system_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_a11y::AccessibilitySystem,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_a11y::AccessibilitySystem>| {
                let output: () = {
                    {
                        let output: () = <::bevy_a11y::AccessibilitySystem as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_a11y::AccessibilitySystem>| {
                let output: Val<::bevy_a11y::AccessibilitySystem> = {
                    {
                        let output: Val<::bevy_a11y::AccessibilitySystem> = <::bevy_a11y::AccessibilitySystem as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_a11y::AccessibilitySystem>,
                other: Ref<::bevy_a11y::AccessibilitySystem>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_a11y::AccessibilitySystem as ::core::cmp::PartialEq<
                            ::bevy_a11y::AccessibilitySystem,
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
            ::bevy_a11y::AccessibilitySystem,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyA11YScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_accessibility_requested_functions(&mut world);
        register_manage_accessibility_updates_functions(&mut world);
        register_accessibility_system_functions(&mut world);
    }
}
