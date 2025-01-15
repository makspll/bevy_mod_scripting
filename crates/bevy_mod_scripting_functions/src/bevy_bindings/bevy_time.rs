// @generated by cargo bevy-api-gen generate, modify the templates not this file
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use super::bevy_ecs::*;
use super::bevy_reflect::*;
use bevy_mod_scripting_core::bindings::{
    ReflectReference,
    function::{
        from::{Ref, Mut, Val},
        namespace::NamespaceBuilder,
    },
};
use crate::*;
pub struct BevyTimeScriptingPlugin;
impl ::bevy::app::Plugin for BevyTimeScriptingPlugin {
    fn build(&self, app: &mut ::bevy::prelude::App) {
        let mut world = app.world_mut();
        NamespaceBuilder::<::bevy::time::prelude::Fixed>::new(world)
            .register(
                "clone",
                |_self: Ref<bevy::time::prelude::Fixed>| {
                    let output: Val<bevy::time::prelude::Fixed> = <bevy::time::prelude::Fixed as std::clone::Clone>::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::time::prelude::Real>::new(world)
            .register(
                "clone",
                |_self: Ref<bevy::time::prelude::Real>| {
                    let output: Val<bevy::time::prelude::Real> = <bevy::time::prelude::Real as std::clone::Clone>::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::time::prelude::Timer>::new(world)
            .register(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: () = <bevy::time::prelude::Timer as std::cmp::Eq>::assert_receiver_is_total_eq(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "clone",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: Val<bevy::time::prelude::Timer> = <bevy::time::prelude::Timer as std::clone::Clone>::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "elapsed_secs",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f32 = bevy::time::prelude::Timer::elapsed_secs(&_self)
                        .into();
                    output
                },
            )
            .register(
                "elapsed_secs_f64",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f64 = bevy::time::prelude::Timer::elapsed_secs_f64(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "eq",
                |
                    _self: Ref<bevy::time::prelude::Timer>,
                    other: Ref<bevy::time::prelude::Timer>|
                {
                    let output: bool = <bevy::time::prelude::Timer as std::cmp::PartialEq<
                        bevy::time::prelude::Timer,
                    >>::eq(&_self, &other)
                        .into();
                    output
                },
            )
            .register(
                "finished",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: bool = bevy::time::prelude::Timer::finished(&_self)
                        .into();
                    output
                },
            )
            .register(
                "fraction",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f32 = bevy::time::prelude::Timer::fraction(&_self)
                        .into();
                    output
                },
            )
            .register(
                "fraction_remaining",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f32 = bevy::time::prelude::Timer::fraction_remaining(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "from_seconds",
                |duration: f32, mode: Val<bevy::time::prelude::TimerMode>| {
                    let output: Val<bevy::time::prelude::Timer> = bevy::time::prelude::Timer::from_seconds(
                            duration,
                            mode.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "just_finished",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: bool = bevy::time::prelude::Timer::just_finished(&_self)
                        .into();
                    output
                },
            )
            .register(
                "mode",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: Val<bevy::time::prelude::TimerMode> = bevy::time::prelude::Timer::mode(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "pause",
                |mut _self: Mut<bevy::time::prelude::Timer>| {
                    let output: () = bevy::time::prelude::Timer::pause(&mut _self)
                        .into();
                    output
                },
            )
            .register(
                "paused",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: bool = bevy::time::prelude::Timer::paused(&_self).into();
                    output
                },
            )
            .register(
                "remaining_secs",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f32 = bevy::time::prelude::Timer::remaining_secs(&_self)
                        .into();
                    output
                },
            )
            .register(
                "reset",
                |mut _self: Mut<bevy::time::prelude::Timer>| {
                    let output: () = bevy::time::prelude::Timer::reset(&mut _self)
                        .into();
                    output
                },
            )
            .register(
                "set_mode",
                |
                    mut _self: Mut<bevy::time::prelude::Timer>,
                    mode: Val<bevy::time::prelude::TimerMode>|
                {
                    let output: () = bevy::time::prelude::Timer::set_mode(
                            &mut _self,
                            mode.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "times_finished_this_tick",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: u32 = bevy::time::prelude::Timer::times_finished_this_tick(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "unpause",
                |mut _self: Mut<bevy::time::prelude::Timer>| {
                    let output: () = bevy::time::prelude::Timer::unpause(&mut _self)
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::time::prelude::TimerMode>::new(world)
            .register(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::time::prelude::TimerMode>| {
                    let output: () = <bevy::time::prelude::TimerMode as std::cmp::Eq>::assert_receiver_is_total_eq(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "clone",
                |_self: Ref<bevy::time::prelude::TimerMode>| {
                    let output: Val<bevy::time::prelude::TimerMode> = <bevy::time::prelude::TimerMode as std::clone::Clone>::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "eq",
                |
                    _self: Ref<bevy::time::prelude::TimerMode>,
                    other: Ref<bevy::time::prelude::TimerMode>|
                {
                    let output: bool = <bevy::time::prelude::TimerMode as std::cmp::PartialEq<
                        bevy::time::prelude::TimerMode,
                    >>::eq(&_self, &other)
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::time::prelude::Virtual>::new(world)
            .register(
                "clone",
                |_self: Ref<bevy::time::prelude::Virtual>| {
                    let output: Val<bevy::time::prelude::Virtual> = <bevy::time::prelude::Virtual as std::clone::Clone>::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::time::Stopwatch>::new(world)
            .register(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: () = <bevy::time::Stopwatch as std::cmp::Eq>::assert_receiver_is_total_eq(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "clone",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: Val<bevy::time::Stopwatch> = <bevy::time::Stopwatch as std::clone::Clone>::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "elapsed_secs",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: f32 = bevy::time::Stopwatch::elapsed_secs(&_self).into();
                    output
                },
            )
            .register(
                "elapsed_secs_f64",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: f64 = bevy::time::Stopwatch::elapsed_secs_f64(&_self)
                        .into();
                    output
                },
            )
            .register(
                "eq",
                |_self: Ref<bevy::time::Stopwatch>, other: Ref<bevy::time::Stopwatch>| {
                    let output: bool = <bevy::time::Stopwatch as std::cmp::PartialEq<
                        bevy::time::Stopwatch,
                    >>::eq(&_self, &other)
                        .into();
                    output
                },
            )
            .register(
                "is_paused",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: bool = bevy::time::Stopwatch::is_paused(&_self).into();
                    output
                },
            )
            .register(
                "new",
                || {
                    let output: Val<bevy::time::Stopwatch> = bevy::time::Stopwatch::new()
                        .into();
                    output
                },
            )
            .register(
                "pause",
                |mut _self: Mut<bevy::time::Stopwatch>| {
                    let output: () = bevy::time::Stopwatch::pause(&mut _self).into();
                    output
                },
            )
            .register(
                "reset",
                |mut _self: Mut<bevy::time::Stopwatch>| {
                    let output: () = bevy::time::Stopwatch::reset(&mut _self).into();
                    output
                },
            )
            .register(
                "unpause",
                |mut _self: Mut<bevy::time::Stopwatch>| {
                    let output: () = bevy::time::Stopwatch::unpause(&mut _self).into();
                    output
                },
            );
    }
}
