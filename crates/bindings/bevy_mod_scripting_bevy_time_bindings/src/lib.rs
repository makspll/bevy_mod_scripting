//! Script bindings for bevy time: durations, clocks, timers and stopwatches.

use bevy_app::{App, Plugin};

mod duration;
mod time;
mod timer;

/// Registers the time script bindings.
#[derive(Default)]
pub struct BevyTimeBindingsPlugin;

impl Plugin for BevyTimeBindingsPlugin {
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_time"],
            "time",
        );
    }

    fn build(&self, app: &mut App) {
        use bevy_mod_scripting_bindings::ScriptSystemSetAppExt;
        app.register_script_system_set("time", "TimeSystems", bevy_time::TimeSystems);
        let world = app.world_mut();
        duration::register(world);
        time::register(world);
        timer::register(world);
    }
}
