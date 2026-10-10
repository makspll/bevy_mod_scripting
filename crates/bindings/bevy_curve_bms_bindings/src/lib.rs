
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
pub struct BevyCurveScriptingPlugin;
pub(crate) fn register_interval_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_curve::Interval,
    >::new(world)
        .register_documented(
            "clamp",
            |_self: V<::bevy_curve::Interval>, value: f32| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_curve::Interval::clamp(
                                _self.into_inner(),
                                value,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Clamp the given `value` to lie within this interval.",
            &["_self", "value"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_curve::Interval>| {
                let output: V<::bevy_curve::Interval> = {
                    {
                        let output: V<::bevy_curve::Interval> = <::bevy_curve::Interval as ::core::clone::Clone>::clone(
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
            "contains",
            |_self: V<::bevy_curve::Interval>, item: f32| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_curve::Interval::contains(
                                _self.into_inner(),
                                item,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if `item` is contained in this interval.",
            &["_self", "item"],
        )
        .register_documented(
            "contains_interval",
            |_self: V<::bevy_curve::Interval>, other: V<::bevy_curve::Interval>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_curve::Interval::contains_interval(
                                _self.into_inner(),
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the other interval is contained in this interval.\n This is non-strict: each interval will contain itself.",
            &["_self", "other"],
        )
        .register_documented(
            "end",
            |_self: V<::bevy_curve::Interval>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_curve::Interval::end(_self.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            " Get the end of this interval.",
            &["_self"],
        )
        .register_documented(
            "eq",
            |_self: R<::bevy_curve::Interval>, other: R<::bevy_curve::Interval>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_curve::Interval as ::core::cmp::PartialEq<
                            ::bevy_curve::Interval,
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
            "has_finite_end",
            |_self: V<::bevy_curve::Interval>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_curve::Interval::has_finite_end(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if this interval has a finite end.",
            &["_self"],
        )
        .register_documented(
            "has_finite_start",
            |_self: V<::bevy_curve::Interval>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_curve::Interval::has_finite_start(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if this interval has a finite start.",
            &["_self"],
        )
        .register_documented(
            "is_bounded",
            |_self: V<::bevy_curve::Interval>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_curve::Interval::is_bounded(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if this interval is bounded — that is, if both its start and end are finite.\n Equivalently, an interval is bounded if its length is finite.",
            &["_self"],
        )
        .register_documented(
            "length",
            |_self: V<::bevy_curve::Interval>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_curve::Interval::length(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of this interval. Note that the result may be infinite (`f32::INFINITY`).",
            &["_self"],
        )
        .register_documented(
            "start",
            |_self: V<::bevy_curve::Interval>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_curve::Interval::start(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the start of this interval.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_curve::Interval,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ease_function_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_curve::EaseFunction,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_curve::EaseFunction>| {
                let output: V<::bevy_curve::EaseFunction> = {
                    {
                        let output: V<::bevy_curve::EaseFunction> = <::bevy_curve::EaseFunction as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_curve::EaseFunction>, other: R<::bevy_curve::EaseFunction>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_curve::EaseFunction as ::core::cmp::PartialEq<
                            ::bevy_curve::EaseFunction,
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
            ::bevy_curve::EaseFunction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_jump_at_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_curve::JumpAt,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_curve::JumpAt>| {
                let output: V<::bevy_curve::JumpAt> = {
                    {
                        let output: V<::bevy_curve::JumpAt> = <::bevy_curve::JumpAt as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_curve::JumpAt>, other: R<::bevy_curve::JumpAt>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_curve::JumpAt as ::core::cmp::PartialEq<
                            ::bevy_curve::JumpAt,
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
            ::bevy_curve::JumpAt,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyCurveScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_interval_functions(&mut world);
        register_ease_function_functions(&mut world);
        register_jump_at_functions(&mut world);
    }
}
