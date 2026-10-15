//! Clocks: the [`Time`](bevy_time::Time) resources, and the `world` functions which return them.

use std::time::Duration;

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::{
    InteropError, ReflectReference,
    function::{
        from::{M, R, V},
        script_function::FunctionCallContext,
    },
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_time::{Fixed, Real, Time, Virtual};

/// Functions shared by all clocks.
// rustfmt keeps indenting the attribute inside the macro further on every run
#[rustfmt::skip]
macro_rules! time_functions {
    ($ty:ty, $name:literal) => {
        #[script_bindings(
            remote,
            name = $name,
            bms_bindings_path = "bevy_mod_scripting_bindings",
            namespace = "time",
            core
        )]
        impl $ty {
            /// The time that passed since the previous update.
            fn delta(_self: R<$ty>) -> V<Duration> {
                V(_self.delta())
            }

            /// The time that passed since the previous update, in seconds.
            fn delta_secs(_self: R<$ty>) -> f32 {
                _self.delta_secs()
            }

            /// The time that passed since the previous update, in seconds as a double precision number.
            fn delta_secs_f64(_self: R<$ty>) -> f64 {
                _self.delta_secs_f64()
            }

            /// The total time that passed since the clock started.
            fn elapsed(_self: R<$ty>) -> V<Duration> {
                V(_self.elapsed())
            }

            /// The total time that passed since the clock started, in seconds.
            fn elapsed_secs(_self: R<$ty>) -> f32 {
                _self.elapsed_secs()
            }

            /// The total time that passed since the clock started, in seconds as a double precision number.
            fn elapsed_secs_f64(_self: R<$ty>) -> f64 {
                _self.elapsed_secs_f64()
            }

            /// The total time that passed since the clock started, in seconds, wrapped to a period so it stays precise for long running apps.
            fn elapsed_secs_wrapped(_self: R<$ty>) -> f32 {
                _self.elapsed_secs_wrapped()
            }
        }
    };
}

time_functions!(Time<()>, "time_generic_shared_functions");
time_functions!(Time<Virtual>, "time_virtual_shared_functions");
time_functions!(Time<Real>, "time_real_shared_functions");
time_functions!(Time<Fixed>, "time_fixed_shared_functions");

#[script_bindings(
    remote,
    name = "time_virtual_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "time",
    core
)]
impl Time<Virtual> {
    /// Stops the clock until it is unpaused, the time does not advance while paused.
    fn pause(mut _self: M<Time<Virtual>>) {
        _self.pause();
    }

    /// Resumes a paused clock.
    fn unpause(mut _self: M<Time<Virtual>>) {
        _self.unpause();
    }

    /// Whether the clock is paused.
    fn is_paused(_self: R<Time<Virtual>>) -> bool {
        _self.is_paused()
    }

    /// The speed of the clock relative to real time, `1` is real time and `2` is twice as fast.
    fn relative_speed(_self: R<Time<Virtual>>) -> f32 {
        _self.relative_speed()
    }

    /// Sets the speed of the clock relative to real time. Errors if `speed` is negative or not finite.
    fn set_relative_speed(mut _self: M<Time<Virtual>>, speed: f32) -> Result<(), InteropError> {
        if !speed.is_finite() || speed < 0.0 {
            return Err(InteropError::str(
                "the relative speed must be a finite, non-negative number",
            ));
        }
        _self.set_relative_speed(speed);
        Ok(())
    }

    /// The largest time a single update can add to the clock, longer frames are cut short.
    fn max_delta(_self: R<Time<Virtual>>) -> V<Duration> {
        V(_self.max_delta())
    }

    /// Sets the largest time a single update can add to the clock.
    fn set_max_delta(mut _self: M<Time<Virtual>>, max_delta: R<Duration>) {
        _self.set_max_delta(*max_delta);
    }
}

#[script_bindings(
    remote,
    name = "time_fixed_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "time",
    core
)]
impl Time<Fixed> {
    /// The length of one fixed update step.
    fn timestep(_self: R<Time<Fixed>>) -> V<Duration> {
        V(_self.timestep())
    }

    /// Sets the length of one fixed update step, in seconds. Errors if `seconds` is not positive or not finite.
    fn set_timestep_seconds(mut _self: M<Time<Fixed>>, seconds: f64) -> Result<(), InteropError> {
        let timestep = Duration::try_from_secs_f64(seconds)
            .ok()
            .filter(|timestep| !timestep.is_zero())
            .ok_or_else(|| {
                InteropError::str("the timestep must be a finite, positive number of seconds")
            })?;
        _self.set_timestep(timestep);
        Ok(())
    }

    /// How far the clock is into the next fixed update step, from `0` to `1`.
    fn overstep_fraction(_self: R<Time<Fixed>>) -> f32 {
        _self.overstep_fraction()
    }
}

#[script_bindings(
    remote,
    name = "world_time_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    unregistered
)]
impl World {
    /// The default clock: in Update it advances by the frame time, in FixedUpdate by the fixed timestep.
    fn time(ctxt: FunctionCallContext) -> Result<ReflectReference, InteropError> {
        ReflectReference::new_resource_ref::<Time<()>>(ctxt.world()?)
    }

    /// The clock which can be paused and slowed down, it is the one games usually move objects with.
    fn virtual_time(ctxt: FunctionCallContext) -> Result<ReflectReference, InteropError> {
        ReflectReference::new_resource_ref::<Time<Virtual>>(ctxt.world()?)
    }

    /// The clock of the real world, which always advances at the system's pace.
    fn real_time(ctxt: FunctionCallContext) -> Result<ReflectReference, InteropError> {
        ReflectReference::new_resource_ref::<Time<Real>>(ctxt.world()?)
    }

    /// The clock of the fixed update step, which advances in steps of the fixed timestep.
    fn fixed_time(ctxt: FunctionCallContext) -> Result<ReflectReference, InteropError> {
        ReflectReference::new_resource_ref::<Time<Fixed>>(ctxt.world()?)
    }
}

/// Registers the functions of clocks.
pub(crate) fn register(world: &mut World) {
    register_time_generic_shared_functions(world);
    register_time_virtual_shared_functions(world);
    register_time_real_shared_functions(world);
    register_time_fixed_shared_functions(world);
    register_time_virtual_functions(world);
    register_time_fixed_functions(world);
    register_world_time_functions(world);
}
