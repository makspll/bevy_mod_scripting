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
pub struct BevyTimeScriptingPlugin;
pub(crate) fn register_fixed_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_time::prelude::Fixed,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_time::prelude::Fixed>| {
            let output: Val<::bevy_time::prelude::Fixed> = {
                {
                    let output: Val<::bevy_time::prelude::Fixed> =
                        <::bevy_time::prelude::Fixed as ::core::clone::Clone>::clone(&_self).into();
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
            ::bevy_time::prelude::Fixed,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_real_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_time::prelude::Real,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_time::prelude::Real>| {
            let output: Val<::bevy_time::prelude::Real> = {
                {
                    let output: Val<::bevy_time::prelude::Real> =
                        <::bevy_time::prelude::Real as ::core::clone::Clone>::clone(&_self).into();
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
            ::bevy_time::prelude::Real,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_timer_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_time::prelude::Timer,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: () = {
                    {
                        let output: () = <::bevy_time::prelude::Timer as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: Val<::bevy_time::prelude::Timer> = {
                    {
                        let output: Val<::bevy_time::prelude::Timer> = <::bevy_time::prelude::Timer as ::core::clone::Clone>::clone(
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
            "duration",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: Val<::core::time::Duration> = {
                    {
                        let output: Val<::core::time::Duration> = ::bevy_time::prelude::Timer::duration(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the duration of the timer.\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let timer = Timer::new(Duration::from_secs(1), TimerMode::Once);\n assert_eq!(timer.duration(), Duration::from_secs(1));\n ```",
            &["_self"],
        )
        .register_documented(
            "elapsed",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: Val<::core::time::Duration> = {
                    {
                        let output: Val<::core::time::Duration> = ::bevy_time::prelude::Timer::elapsed(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the time elapsed on the timer. Guaranteed to be between 0.0 and `duration`.\n Will only equal `duration` when the timer is finished and non repeating.\n See also [`Stopwatch::elapsed`](Stopwatch::elapsed).\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut timer = Timer::from_seconds(1.0, TimerMode::Once);\n timer.tick(Duration::from_secs_f32(0.5));\n assert_eq!(timer.elapsed(), Duration::from_secs_f32(0.5));\n ```",
            &["_self"],
        )
        .register_documented(
            "elapsed_secs",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_time::prelude::Timer::elapsed_secs(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the time elapsed on the timer as an `f32`.\n See also [`Timer::elapsed`](Timer::elapsed).",
            &["_self"],
        )
        .register_documented(
            "elapsed_secs_f64",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: f64 = {
                    {
                        let output: f64 = ::bevy_time::prelude::Timer::elapsed_secs_f64(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the time elapsed on the timer as an `f64`.\n See also [`Timer::elapsed`](Timer::elapsed).",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_time::prelude::Timer>,
                other: Ref<::bevy_time::prelude::Timer>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_time::prelude::Timer as ::core::cmp::PartialEq<
                            ::bevy_time::prelude::Timer,
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
            "finished",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_time::prelude::Timer::finished(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the timer has reached its duration.\n For repeating timers, this method behaves identically to [`Timer::just_finished`].\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut timer_once = Timer::from_seconds(1.0, TimerMode::Once);\n timer_once.tick(Duration::from_secs_f32(1.5));\n assert!(timer_once.finished());\n timer_once.tick(Duration::from_secs_f32(0.5));\n assert!(timer_once.finished());\n let mut timer_repeating = Timer::from_seconds(1.0, TimerMode::Repeating);\n timer_repeating.tick(Duration::from_secs_f32(1.1));\n assert!(timer_repeating.finished());\n timer_repeating.tick(Duration::from_secs_f32(0.8));\n assert!(!timer_repeating.finished());\n timer_repeating.tick(Duration::from_secs_f32(0.6));\n assert!(timer_repeating.finished());\n ```",
            &["_self"],
        )
        .register_documented(
            "fraction",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_time::prelude::Timer::fraction(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the fraction of the timer elapsed time (goes from 0.0 to 1.0).\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut timer = Timer::from_seconds(2.0, TimerMode::Once);\n timer.tick(Duration::from_secs_f32(0.5));\n assert_eq!(timer.fraction(), 0.25);\n ```",
            &["_self"],
        )
        .register_documented(
            "fraction_remaining",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_time::prelude::Timer::fraction_remaining(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the fraction of the timer remaining time (goes from 1.0 to 0.0).\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut timer = Timer::from_seconds(2.0, TimerMode::Once);\n timer.tick(Duration::from_secs_f32(0.5));\n assert_eq!(timer.fraction_remaining(), 0.75);\n ```",
            &["_self"],
        )
        .register_documented(
            "from_seconds",
            |duration: f32, mode: Val<::bevy_time::prelude::TimerMode>| {
                let output: Val<::bevy_time::prelude::Timer> = {
                    {
                        let output: Val<::bevy_time::prelude::Timer> = ::bevy_time::prelude::Timer::from_seconds(
                                duration,
                                mode.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new timer with a given duration in seconds.\n # Example\n ```\n # use bevy_time::*;\n let mut timer = Timer::from_seconds(1.0, TimerMode::Once);\n ```",
            &["duration", "mode"],
        )
        .register_documented(
            "just_finished",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_time::prelude::Timer::just_finished(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` only on the tick the timer reached its duration.\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut timer = Timer::from_seconds(1.0, TimerMode::Once);\n timer.tick(Duration::from_secs_f32(1.5));\n assert!(timer.just_finished());\n timer.tick(Duration::from_secs_f32(0.5));\n assert!(!timer.just_finished());\n ```",
            &["_self"],
        )
        .register_documented(
            "mode",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: Val<::bevy_time::prelude::TimerMode> = {
                    {
                        let output: Val<::bevy_time::prelude::TimerMode> = ::bevy_time::prelude::Timer::mode(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the mode of the timer.\n # Examples\n ```\n # use bevy_time::*;\n let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);\n assert_eq!(timer.mode(), TimerMode::Repeating);\n ```",
            &["_self"],
        )
        .register_documented(
            "new",
            |
                duration: Val<::core::time::Duration>,
                mode: Val<::bevy_time::prelude::TimerMode>|
            {
                let output: Val<::bevy_time::prelude::Timer> = {
                    {
                        let output: Val<::bevy_time::prelude::Timer> = ::bevy_time::prelude::Timer::new(
                                duration.into_inner(),
                                mode.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new timer with a given duration.\n See also [`Timer::from_seconds`](Timer::from_seconds).",
            &["duration", "mode"],
        )
        .register_documented(
            "pause",
            |mut _self: Mut<::bevy_time::prelude::Timer>| {
                let output: () = {
                    {
                        let output: () = ::bevy_time::prelude::Timer::pause(&mut _self)
                            .into();
                        output
                    }
                };
                output
            },
            " Pauses the Timer. Disables the ticking of the timer.\n See also [`Stopwatch::pause`](Stopwatch::pause).\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut timer = Timer::from_seconds(1.0, TimerMode::Once);\n timer.pause();\n timer.tick(Duration::from_secs_f32(0.5));\n assert_eq!(timer.elapsed_secs(), 0.0);\n ```",
            &["_self"],
        )
        .register_documented(
            "paused",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_time::prelude::Timer::paused(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the timer is paused.\n See also [`Stopwatch::is_paused`](Stopwatch::is_paused).\n # Examples\n ```\n # use bevy_time::*;\n let mut timer = Timer::from_seconds(1.0, TimerMode::Once);\n assert!(!timer.paused());\n timer.pause();\n assert!(timer.paused());\n timer.unpause();\n assert!(!timer.paused());\n ```",
            &["_self"],
        )
        .register_documented(
            "remaining",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: Val<::core::time::Duration> = {
                    {
                        let output: Val<::core::time::Duration> = ::bevy_time::prelude::Timer::remaining(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the remaining time using Duration\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut timer = Timer::from_seconds(2.0, TimerMode::Once);\n timer.tick(Duration::from_secs_f32(0.5));\n assert_eq!(timer.remaining(), Duration::from_secs_f32(1.5));\n ```",
            &["_self"],
        )
        .register_documented(
            "remaining_secs",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_time::prelude::Timer::remaining_secs(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the remaining time in seconds\n # Examples\n ```\n # use bevy_time::*;\n use std::cmp::Ordering;\n use std::time::Duration;\n let mut timer = Timer::from_seconds(2.0, TimerMode::Once);\n timer.tick(Duration::from_secs_f32(0.5));\n let result = timer.remaining_secs().total_cmp(&1.5);\n assert_eq!(Ordering::Equal, result);\n ```",
            &["_self"],
        )
        .register_documented(
            "reset",
            |mut _self: Mut<::bevy_time::prelude::Timer>| {
                let output: () = {
                    {
                        let output: () = ::bevy_time::prelude::Timer::reset(&mut _self)
                            .into();
                        output
                    }
                };
                output
            },
            " Resets the timer. The reset doesn't affect the `paused` state of the timer.\n See also [`Stopwatch::reset`](Stopwatch::reset).\n Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut timer = Timer::from_seconds(1.0, TimerMode::Once);\n timer.tick(Duration::from_secs_f32(1.5));\n timer.reset();\n assert!(!timer.finished());\n assert!(!timer.just_finished());\n assert_eq!(timer.elapsed_secs(), 0.0);\n ```",
            &["_self"],
        )
        .register_documented(
            "set_duration",
            |
                mut _self: Mut<::bevy_time::prelude::Timer>,
                duration: Val<::core::time::Duration>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_time::prelude::Timer::set_duration(
                                &mut _self,
                                duration.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the duration of the timer.\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut timer = Timer::from_seconds(1.5, TimerMode::Once);\n timer.set_duration(Duration::from_secs(1));\n assert_eq!(timer.duration(), Duration::from_secs(1));\n ```",
            &["_self", "duration"],
        )
        .register_documented(
            "set_elapsed",
            |
                mut _self: Mut<::bevy_time::prelude::Timer>,
                time: Val<::core::time::Duration>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_time::prelude::Timer::set_elapsed(
                                &mut _self,
                                time.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the elapsed time of the timer without any other considerations.\n See also [`Stopwatch::set`](Stopwatch::set).\n #\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut timer = Timer::from_seconds(1.0, TimerMode::Once);\n timer.set_elapsed(Duration::from_secs(2));\n assert_eq!(timer.elapsed(), Duration::from_secs(2));\n // the timer is not finished even if the elapsed time is greater than the duration.\n assert!(!timer.finished());\n ```",
            &["_self", "time"],
        )
        .register_documented(
            "set_mode",
            |
                mut _self: Mut<::bevy_time::prelude::Timer>,
                mode: Val<::bevy_time::prelude::TimerMode>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_time::prelude::Timer::set_mode(
                                &mut _self,
                                mode.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the mode of the timer.\n # Examples\n ```\n # use bevy_time::*;\n let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);\n timer.set_mode(TimerMode::Once);\n assert_eq!(timer.mode(), TimerMode::Once);\n ```",
            &["_self", "mode"],
        )
        .register_documented(
            "times_finished_this_tick",
            |_self: Ref<::bevy_time::prelude::Timer>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_time::prelude::Timer::times_finished_this_tick(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the number of times a repeating timer\n finished during the last [`tick`](Timer<T>::tick) call.\n For non repeating-timers, this method will only ever\n return 0 or 1.\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);\n timer.tick(Duration::from_secs_f32(6.0));\n assert_eq!(timer.times_finished_this_tick(), 6);\n timer.tick(Duration::from_secs_f32(2.0));\n assert_eq!(timer.times_finished_this_tick(), 2);\n timer.tick(Duration::from_secs_f32(0.5));\n assert_eq!(timer.times_finished_this_tick(), 0);\n ```",
            &["_self"],
        )
        .register_documented(
            "unpause",
            |mut _self: Mut<::bevy_time::prelude::Timer>| {
                let output: () = {
                    {
                        let output: () = ::bevy_time::prelude::Timer::unpause(&mut _self)
                            .into();
                        output
                    }
                };
                output
            },
            " Unpauses the Timer. Resumes the ticking of the timer.\n See also [`Stopwatch::unpause()`](Stopwatch::unpause).\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut timer = Timer::from_seconds(1.0, TimerMode::Once);\n timer.pause();\n timer.tick(Duration::from_secs_f32(0.5));\n timer.unpause();\n timer.tick(Duration::from_secs_f32(0.5));\n assert_eq!(timer.elapsed_secs(), 0.5);\n ```",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_time::prelude::Timer,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_timer_mode_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_time::prelude::TimerMode,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_time::prelude::TimerMode>| {
                let output: () = {
                    {
                        let output: () = <::bevy_time::prelude::TimerMode as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_time::prelude::TimerMode>| {
                let output: Val<::bevy_time::prelude::TimerMode> = {
                    {
                        let output: Val<::bevy_time::prelude::TimerMode> = <::bevy_time::prelude::TimerMode as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_time::prelude::TimerMode>,
                other: Ref<::bevy_time::prelude::TimerMode>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_time::prelude::TimerMode as ::core::cmp::PartialEq<
                            ::bevy_time::prelude::TimerMode,
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
            ::bevy_time::prelude::TimerMode,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_virtual_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_time::prelude::Virtual,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_time::prelude::Virtual>| {
            let output: Val<::bevy_time::prelude::Virtual> = {
                {
                    let output: Val<::bevy_time::prelude::Virtual> =
                        <::bevy_time::prelude::Virtual as ::core::clone::Clone>::clone(&_self)
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
            ::bevy_time::prelude::Virtual,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_stopwatch_functions(world: &mut World) {
    bevy_mod_scripting_core::bindings::function::namespace::NamespaceBuilder::<
        ::bevy_time::Stopwatch,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_time::Stopwatch>| {
                let output: () = {
                    {
                        let output: () = <::bevy_time::Stopwatch as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_time::Stopwatch>| {
                let output: Val<::bevy_time::Stopwatch> = {
                    {
                        let output: Val<::bevy_time::Stopwatch> = <::bevy_time::Stopwatch as ::core::clone::Clone>::clone(
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
            "elapsed",
            |_self: Ref<::bevy_time::Stopwatch>| {
                let output: Val<::core::time::Duration> = {
                    {
                        let output: Val<::core::time::Duration> = ::bevy_time::Stopwatch::elapsed(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the elapsed time since the last [`reset`](Stopwatch::reset)\n of the stopwatch.\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut stopwatch = Stopwatch::new();\n stopwatch.tick(Duration::from_secs(1));\n assert_eq!(stopwatch.elapsed(), Duration::from_secs(1));\n ```\n # See Also\n [`elapsed_secs`](Stopwatch::elapsed_secs) - if an `f32` value is desirable instead.\n [`elapsed_secs_f64`](Stopwatch::elapsed_secs_f64) - if an `f64` is desirable instead.",
            &["_self"],
        )
        .register_documented(
            "elapsed_secs",
            |_self: Ref<::bevy_time::Stopwatch>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_time::Stopwatch::elapsed_secs(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the elapsed time since the last [`reset`](Stopwatch::reset)\n of the stopwatch, in seconds.\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut stopwatch = Stopwatch::new();\n stopwatch.tick(Duration::from_secs(1));\n assert_eq!(stopwatch.elapsed_secs(), 1.0);\n ```\n # See Also\n [`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.\n [`elapsed_secs_f64`](Stopwatch::elapsed_secs_f64) - if an `f64` is desirable instead.",
            &["_self"],
        )
        .register_documented(
            "elapsed_secs_f64",
            |_self: Ref<::bevy_time::Stopwatch>| {
                let output: f64 = {
                    {
                        let output: f64 = ::bevy_time::Stopwatch::elapsed_secs_f64(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the elapsed time since the last [`reset`](Stopwatch::reset)\n of the stopwatch, in seconds, as f64.\n # See Also\n [`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.\n [`elapsed_secs`](Stopwatch::elapsed_secs) - if an `f32` is desirable instead.",
            &["_self"],
        )
        .register_documented(
            "eq",
            |_self: Ref<::bevy_time::Stopwatch>, other: Ref<::bevy_time::Stopwatch>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_time::Stopwatch as ::core::cmp::PartialEq<
                            ::bevy_time::Stopwatch,
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
            "is_paused",
            |_self: Ref<::bevy_time::Stopwatch>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_time::Stopwatch::is_paused(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the stopwatch is paused.\n # Examples\n ```\n # use bevy_time::*;\n let mut stopwatch = Stopwatch::new();\n assert!(!stopwatch.is_paused());\n stopwatch.pause();\n assert!(stopwatch.is_paused());\n stopwatch.unpause();\n assert!(!stopwatch.is_paused());\n ```",
            &["_self"],
        )
        .register_documented(
            "new",
            || {
                let output: Val<::bevy_time::Stopwatch> = {
                    {
                        let output: Val<::bevy_time::Stopwatch> = ::bevy_time::Stopwatch::new()
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new unpaused `Stopwatch` with no elapsed time.\n # Examples\n ```\n # use bevy_time::*;\n let stopwatch = Stopwatch::new();\n assert_eq!(stopwatch.elapsed_secs(), 0.0);\n assert_eq!(stopwatch.is_paused(), false);\n ```",
            &[],
        )
        .register_documented(
            "pause",
            |mut _self: Mut<::bevy_time::Stopwatch>| {
                let output: () = {
                    {
                        let output: () = ::bevy_time::Stopwatch::pause(&mut _self)
                            .into();
                        output
                    }
                };
                output
            },
            " Pauses the stopwatch. Any call to [`tick`](Stopwatch::tick) while\n paused will not have any effect on the elapsed time.\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut stopwatch = Stopwatch::new();\n stopwatch.pause();\n stopwatch.tick(Duration::from_secs_f32(1.5));\n assert!(stopwatch.is_paused());\n assert_eq!(stopwatch.elapsed_secs(), 0.0);\n ```",
            &["_self"],
        )
        .register_documented(
            "reset",
            |mut _self: Mut<::bevy_time::Stopwatch>| {
                let output: () = {
                    {
                        let output: () = ::bevy_time::Stopwatch::reset(&mut _self)
                            .into();
                        output
                    }
                };
                output
            },
            " Resets the stopwatch. The reset doesn't affect the paused state of the stopwatch.\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut stopwatch = Stopwatch::new();\n stopwatch.tick(Duration::from_secs_f32(1.5));\n stopwatch.reset();\n assert_eq!(stopwatch.elapsed_secs(), 0.0);\n ```",
            &["_self"],
        )
        .register_documented(
            "set_elapsed",
            |mut _self: Mut<::bevy_time::Stopwatch>, time: Val<::core::time::Duration>| {
                let output: () = {
                    {
                        let output: () = ::bevy_time::Stopwatch::set_elapsed(
                                &mut _self,
                                time.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Sets the elapsed time of the stopwatch.\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut stopwatch = Stopwatch::new();\n stopwatch.set_elapsed(Duration::from_secs_f32(1.0));\n assert_eq!(stopwatch.elapsed_secs(), 1.0);\n ```",
            &["_self", "time"],
        )
        .register_documented(
            "unpause",
            |mut _self: Mut<::bevy_time::Stopwatch>| {
                let output: () = {
                    {
                        let output: () = ::bevy_time::Stopwatch::unpause(&mut _self)
                            .into();
                        output
                    }
                };
                output
            },
            " Unpauses the stopwatch. Resume the effect of ticking on elapsed time.\n # Examples\n ```\n # use bevy_time::*;\n use std::time::Duration;\n let mut stopwatch = Stopwatch::new();\n stopwatch.pause();\n stopwatch.tick(Duration::from_secs_f32(1.0));\n stopwatch.unpause();\n stopwatch.tick(Duration::from_secs_f32(1.0));\n assert!(!stopwatch.is_paused());\n assert_eq!(stopwatch.elapsed_secs(), 1.0);\n ```",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_time::Stopwatch,
            bevy_mod_scripting_core::bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyTimeScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_fixed_functions(&mut world);
        register_real_functions(&mut world);
        register_timer_functions(&mut world);
        register_timer_mode_functions(&mut world);
        register_virtual_functions(&mut world);
        register_stopwatch_functions(&mut world);
    }
}
