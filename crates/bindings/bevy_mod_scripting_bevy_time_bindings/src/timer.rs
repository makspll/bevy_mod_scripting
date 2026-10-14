//! Timers and stopwatches: [`Timer`](bevy_time::Timer) and [`Stopwatch`](bevy_time::Stopwatch).

use std::time::Duration;

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::{
    InteropError,
    function::from::{M, R, Union, V},
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_time::{Stopwatch, Timer, TimerMode};

/// The duration to advance a clock by, from seconds or a duration.
fn tick_delta(delta: Union<f32, R<Duration>>) -> Result<Duration, InteropError> {
    match delta.into_left() {
        Ok(secs) => Duration::try_from_secs_f32(secs).map_err(|_| {
            InteropError::str("the time must be a finite, non-negative number of seconds")
        }),
        Err(duration) => Ok(*duration),
    }
}

#[script_bindings(
    remote,
    name = "timer_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Timer {
    /// A timer which runs for `secs` seconds, in `mode`. Errors if `secs` is negative or not finite.
    ///
    /// ```multi-lang-example,lua
    /// local timer = Timer.from_seconds(2, TimerMode.Repeating)
    /// timer:tick(1)
    /// assert(not timer:finished())
    /// timer:tick(1.5)
    /// assert(timer:finished())
    /// ```
    ///
    /// ```multi-lang-example,rhai
    /// let timer = Timer.from_seconds.call(2.0, TimerMode.Repeating);
    /// timer.tick.call(1.0);
    /// assert(!timer.finished.call());
    /// timer.tick.call(1.5);
    /// assert(timer.finished.call());
    /// ```
    fn from_seconds(secs: f32, mode: R<TimerMode>) -> Result<V<Timer>, InteropError> {
        let duration = Duration::try_from_secs_f32(secs).map_err(|_| {
            InteropError::str("the duration must be a finite, non-negative number of seconds")
        })?;
        Ok(V(Timer::new(duration, *mode)))
    }

    /// A timer which runs for `duration`, in `mode`.
    fn new(duration: R<Duration>, mode: R<TimerMode>) -> V<Timer> {
        V(Timer::new(*duration, *mode))
    }

    /// Advances the timer by `delta`, given in seconds or as a duration. Errors if the time is negative.
    fn tick(mut _self: M<Timer>, delta: Union<f32, R<Duration>>) -> Result<(), InteropError> {
        _self.tick(tick_delta(delta)?);
        Ok(())
    }

    /// Whether the timer has run for its full duration.
    fn finished(_self: R<Timer>) -> bool {
        _self.is_finished()
    }

    /// Whether the timer finished during the last tick.
    fn just_finished(_self: R<Timer>) -> bool {
        _self.just_finished()
    }

    /// Whether the timer is paused.
    fn paused(_self: R<Timer>) -> bool {
        _self.is_paused()
    }

    /// The time the timer has run for.
    fn elapsed(_self: R<Timer>) -> V<Duration> {
        V(_self.elapsed())
    }

    /// The time the timer has run for, in seconds.
    fn elapsed_secs(_self: R<Timer>) -> f32 {
        _self.elapsed_secs()
    }

    /// The time left until the timer finishes.
    fn remaining(_self: R<Timer>) -> V<Duration> {
        V(_self.remaining())
    }

    /// The time left until the timer finishes, in seconds.
    fn remaining_secs(_self: R<Timer>) -> f32 {
        _self.remaining_secs()
    }

    /// How much of the duration has passed, from `0` to `1`.
    fn fraction(_self: R<Timer>) -> f32 {
        _self.fraction()
    }

    /// How much of the duration is left, from `0` to `1`.
    fn fraction_remaining(_self: R<Timer>) -> f32 {
        _self.fraction_remaining()
    }

    /// The duration the timer runs for.
    fn duration(_self: R<Timer>) -> V<Duration> {
        V(_self.duration())
    }

    /// How many times the timer finished during the last tick, which can be more than once for a short timer.
    fn times_finished_this_tick(_self: R<Timer>) -> u32 {
        _self.times_finished_this_tick()
    }

    /// Whether the timer runs once, or repeats.
    fn mode(_self: R<Timer>) -> V<TimerMode> {
        V(_self.mode())
    }

    /// Stops the timer from advancing until it is unpaused.
    fn pause(mut _self: M<Timer>) {
        _self.pause();
    }

    /// Resumes a paused timer.
    fn unpause(mut _self: M<Timer>) {
        _self.unpause();
    }

    /// Sets the timer back to zero elapsed time, keeping its duration and mode.
    fn reset(mut _self: M<Timer>) {
        _self.reset();
    }

    /// Makes the timer finish on its next tick.
    fn almost_finish(mut _self: M<Timer>) {
        _self.almost_finish();
    }

    /// Sets the duration the timer runs for.
    fn set_duration(mut _self: M<Timer>, duration: R<Duration>) {
        _self.set_duration(*duration);
    }

    /// Sets whether the timer runs once, or repeats.
    fn set_mode(mut _self: M<Timer>, mode: R<TimerMode>) {
        _self.set_mode(*mode);
    }

    /// Sets the time the timer has run for.
    fn set_elapsed(mut _self: M<Timer>, elapsed: R<Duration>) {
        _self.set_elapsed(*elapsed);
    }
}

#[script_bindings(
    remote,
    name = "stopwatch_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Stopwatch {
    /// A stopwatch at zero, which is running.
    fn new() -> V<Stopwatch> {
        V(Stopwatch::new())
    }

    /// Advances the stopwatch by `delta`, given in seconds or as a duration. Errors if the time is negative.
    fn tick(mut _self: M<Stopwatch>, delta: Union<f32, R<Duration>>) -> Result<(), InteropError> {
        _self.tick(tick_delta(delta)?);
        Ok(())
    }

    /// The time the stopwatch has run for.
    fn elapsed(_self: R<Stopwatch>) -> V<Duration> {
        V(_self.elapsed())
    }

    /// The time the stopwatch has run for, in seconds.
    fn elapsed_secs(_self: R<Stopwatch>) -> f32 {
        _self.elapsed_secs()
    }

    /// Stops the stopwatch from advancing until it is unpaused.
    fn pause(mut _self: M<Stopwatch>) {
        _self.pause();
    }

    /// Resumes a paused stopwatch.
    fn unpause(mut _self: M<Stopwatch>) {
        _self.unpause();
    }

    /// Whether the stopwatch is paused.
    fn is_paused(_self: R<Stopwatch>) -> bool {
        _self.is_paused()
    }

    /// Sets the stopwatch back to zero elapsed time.
    fn reset(mut _self: M<Stopwatch>) {
        _self.reset();
    }

    /// Sets the time the stopwatch has run for.
    fn set_elapsed(mut _self: M<Stopwatch>, elapsed: R<Duration>) {
        _self.set_elapsed(*elapsed);
    }
}

/// Registers the functions of timers and stopwatches.
pub(crate) fn register(world: &mut World) {
    register_timer_functions(world);
    register_stopwatch_functions(world);
}
