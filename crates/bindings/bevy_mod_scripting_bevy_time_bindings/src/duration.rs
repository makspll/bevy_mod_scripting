//! Durations and instants: [`Duration`](std::time::Duration) and [`Instant`](std::time::Instant).

use std::time::{Duration, Instant};

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::{
    InteropError,
    function::from::{R, V},
};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "duration_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "time",
    core
)]
impl Duration {
    /// A duration of `secs` seconds. Errors if `secs` is negative or not finite.
    fn from_secs(secs: f32) -> Result<V<Duration>, InteropError> {
        Duration::try_from_secs_f32(secs).map(V).map_err(|_| {
            InteropError::str("seconds must be a finite, non-negative number of seconds")
        })
    }

    /// A duration of `millis` milliseconds.
    fn from_millis(millis: u64) -> V<Duration> {
        V(Duration::from_millis(millis))
    }

    /// A duration of `micros` microseconds.
    fn from_micros(micros: u64) -> V<Duration> {
        V(Duration::from_micros(micros))
    }

    /// A duration of `nanos` nanoseconds.
    fn from_nanos(nanos: u64) -> V<Duration> {
        V(Duration::from_nanos(nanos))
    }

    /// A duration of no time.
    fn zero() -> V<Duration> {
        V(Duration::ZERO)
    }

    /// The duration in seconds.
    fn as_secs(_self: R<Duration>) -> f32 {
        _self.as_secs_f32()
    }

    /// The duration in seconds, as a double precision number.
    fn as_secs_f64(_self: R<Duration>) -> f64 {
        _self.as_secs_f64()
    }

    /// The duration in whole milliseconds, rounded down.
    fn as_millis(_self: R<Duration>) -> u64 {
        u64::try_from(_self.as_millis()).unwrap_or(u64::MAX)
    }

    /// Whether the duration is zero.
    fn is_zero(_self: R<Duration>) -> bool {
        _self.is_zero()
    }

    /// The sum of two durations, capped at the largest representable duration.
    fn add(_self: R<Duration>, rhs: R<Duration>) -> V<Duration> {
        V(_self.saturating_add(*rhs))
    }

    /// The difference of two durations, or zero if `rhs` is the longer one.
    fn sub(_self: R<Duration>, rhs: R<Duration>) -> V<Duration> {
        V(_self.saturating_sub(*rhs))
    }

    /// The duration scaled by `factor`. Errors if `factor` is negative or not finite.
    fn mul(_self: R<Duration>, factor: f32) -> Result<V<Duration>, InteropError> {
        Duration::try_from_secs_f64(_self.as_secs_f64() * f64::from(factor))
            .map(V)
            .map_err(|_| InteropError::str("the factor must be a finite, non-negative number"))
    }

    /// The duration divided by `divisor`. Errors if `divisor` is not positive.
    fn div(_self: R<Duration>, divisor: f32) -> Result<V<Duration>, InteropError> {
        if divisor.is_nan() || divisor <= 0.0 {
            return Err(InteropError::str("the divisor must be a positive number"));
        }
        Duration::try_from_secs_f64(_self.as_secs_f64() / f64::from(divisor))
            .map(V)
            .map_err(|_| InteropError::str("the result is too large to be a duration"))
    }

    /// Whether two durations are equal.
    fn eq(_self: R<Duration>, other: R<Duration>) -> bool {
        *_self == *other
    }

    /// Whether this duration is shorter than `other`.
    fn lt(_self: R<Duration>, other: R<Duration>) -> bool {
        *_self < *other
    }
}

#[script_bindings(
    remote,
    name = "instant_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "time",
    core
)]
impl Instant {
    /// The duration since this instant was taken.
    fn elapsed(_self: R<Instant>) -> V<Duration> {
        V(_self.elapsed())
    }

    /// The duration from `earlier` to this instant, or zero if `earlier` is later.
    fn duration_since(_self: R<Instant>, earlier: R<Instant>) -> V<Duration> {
        V(_self.saturating_duration_since(*earlier))
    }
}

/// Registers the functions of durations and instants.
pub(crate) fn register(world: &mut World) {
    register_duration_functions(world);
    register_instant_functions(world);
}
