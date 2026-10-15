//! Script bindings for bevy audio playback.

use bevy_app::{App, Plugin};

mod playback_settings;

/// Registers the audio script bindings.
#[derive(Default)]
pub struct BevyAudioBindingsPlugin;

impl Plugin for BevyAudioBindingsPlugin {
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_audio"],
            "audio",
        );
    }

    fn build(&self, app: &mut App) {
        // types without functions are only available to scripts once registered
        app.register_type::<bevy_audio::PlaybackMode>()
            .register_type::<bevy_audio::Volume>();
        let world = app.world_mut();
        playback_settings::register(world);
    }
}
