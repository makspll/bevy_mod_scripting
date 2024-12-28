// @generated by cargo bevy-api-gen generate, modify the templates not this file
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use super::bevy_ecs::*;
use super::bevy_reflect::*;
use bevy_mod_scripting_core::{
    AddContextInitializer, StoreDocumentation,
    bindings::{ReflectReference, function::from::{Ref, Mut, Val}},
};
use crate::*;
pub struct BevyTimeScriptingPlugin;
impl bevy::app::Plugin for BevyTimeScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut world = app.world_mut();
        NamespaceBuilder::<::bevy::time::prelude::Fixed>::new(world)
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::time::prelude::Fixed>| {
                    let output: Val<bevy::time::prelude::Fixed> = ::bevy::time::prelude::Fixed::clone(
                            _self.into(),
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::time::prelude::Real>::new(world)
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::time::prelude::Real>| {
                    let output: Val<bevy::time::prelude::Real> = ::bevy::time::prelude::Real::clone(
                            _self.into(),
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::time::prelude::Timer>::new(world)
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: Val<bevy::time::prelude::Timer> = ::bevy::time::prelude::Timer::clone(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: () = ::bevy::time::prelude::Timer::assert_receiver_is_total_eq(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "from_seconds",
                |duration: f32, mode: Val<bevy::time::prelude::TimerMode>| {
                    let output: Val<bevy::time::prelude::Timer> = ::bevy::time::prelude::Timer::from_seconds(
                            duration.into(),
                            mode.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "finished",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: bool = ::bevy::time::prelude::Timer::finished(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "just_finished",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: bool = ::bevy::time::prelude::Timer::just_finished(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "elapsed_secs",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f32 = ::bevy::time::prelude::Timer::elapsed_secs(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "elapsed_secs_f64",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f64 = ::bevy::time::prelude::Timer::elapsed_secs_f64(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "mode",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: Val<bevy::time::prelude::TimerMode> = ::bevy::time::prelude::Timer::mode(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "set_mode",
                |
                    _self: Mut<bevy::time::prelude::Timer>,
                    mode: Val<bevy::time::prelude::TimerMode>|
                {
                    let output: () = ::bevy::time::prelude::Timer::set_mode(
                            _self.into(),
                            mode.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "pause",
                |_self: Mut<bevy::time::prelude::Timer>| {
                    let output: () = ::bevy::time::prelude::Timer::pause(_self.into())
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "unpause",
                |_self: Mut<bevy::time::prelude::Timer>| {
                    let output: () = ::bevy::time::prelude::Timer::unpause(_self.into())
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "paused",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: bool = ::bevy::time::prelude::Timer::paused(_self.into())
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "reset",
                |_self: Mut<bevy::time::prelude::Timer>| {
                    let output: () = ::bevy::time::prelude::Timer::reset(_self.into())
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "fraction",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f32 = ::bevy::time::prelude::Timer::fraction(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "fraction_remaining",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f32 = ::bevy::time::prelude::Timer::fraction_remaining(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "remaining_secs",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: f32 = ::bevy::time::prelude::Timer::remaining_secs(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "times_finished_this_tick",
                |_self: Ref<bevy::time::prelude::Timer>| {
                    let output: u32 = ::bevy::time::prelude::Timer::times_finished_this_tick(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "eq",
                |
                    _self: Ref<bevy::time::prelude::Timer>,
                    other: Ref<bevy::time::prelude::Timer>|
                {
                    let output: bool = ::bevy::time::prelude::Timer::eq(
                            _self.into(),
                            other.into(),
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::time::prelude::TimerMode>::new(world)
            .overwrite_script_function(
                "eq",
                |
                    _self: Ref<bevy::time::prelude::TimerMode>,
                    other: Ref<bevy::time::prelude::TimerMode>|
                {
                    let output: bool = ::bevy::time::prelude::TimerMode::eq(
                            _self.into(),
                            other.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::time::prelude::TimerMode>| {
                    let output: Val<bevy::time::prelude::TimerMode> = ::bevy::time::prelude::TimerMode::clone(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::time::prelude::TimerMode>| {
                    let output: () = ::bevy::time::prelude::TimerMode::assert_receiver_is_total_eq(
                            _self.into(),
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::time::prelude::Virtual>::new(world)
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::time::prelude::Virtual>| {
                    let output: Val<bevy::time::prelude::Virtual> = ::bevy::time::prelude::Virtual::clone(
                            _self.into(),
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::time::Stopwatch>::new(world)
            .overwrite_script_function(
                "new",
                || {
                    let output: Val<bevy::time::Stopwatch> = ::bevy::time::Stopwatch::new()
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "elapsed_secs",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: f32 = ::bevy::time::Stopwatch::elapsed_secs(_self.into())
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "elapsed_secs_f64",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: f64 = ::bevy::time::Stopwatch::elapsed_secs_f64(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "pause",
                |_self: Mut<bevy::time::Stopwatch>| {
                    let output: () = ::bevy::time::Stopwatch::pause(_self.into()).into();
                    output
                },
            )
            .overwrite_script_function(
                "unpause",
                |_self: Mut<bevy::time::Stopwatch>| {
                    let output: () = ::bevy::time::Stopwatch::unpause(_self.into())
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "is_paused",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: bool = ::bevy::time::Stopwatch::is_paused(_self.into())
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "reset",
                |_self: Mut<bevy::time::Stopwatch>| {
                    let output: () = ::bevy::time::Stopwatch::reset(_self.into()).into();
                    output
                },
            )
            .overwrite_script_function(
                "assert_receiver_is_total_eq",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: () = ::bevy::time::Stopwatch::assert_receiver_is_total_eq(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::time::Stopwatch>| {
                    let output: Val<bevy::time::Stopwatch> = ::bevy::time::Stopwatch::clone(
                            _self.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "eq",
                |_self: Ref<bevy::time::Stopwatch>, other: Ref<bevy::time::Stopwatch>| {
                    let output: bool = ::bevy::time::Stopwatch::eq(
                            _self.into(),
                            other.into(),
                        )
                        .into();
                    output
                },
            );
    }
}
