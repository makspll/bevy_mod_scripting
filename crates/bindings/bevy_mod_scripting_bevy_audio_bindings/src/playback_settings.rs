//! [`PlaybackSettings`] control how a sound starts playing: whether it loops, its starting volume and speed.

// `script_bindings` expects these in scope
use bevy_audio::{PlaybackSettings, Volume};
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "playback_settings_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "audio",
    core
)]
impl PlaybackSettings {
    /// Settings that play the sound once, then do nothing.
    fn once() -> V<PlaybackSettings> {
        V(PlaybackSettings::ONCE)
    }

    /// Settings that repeat the sound forever.
    fn looping() -> V<PlaybackSettings> {
        V(PlaybackSettings::LOOP)
    }

    /// Settings that play the sound once, then despawn the entity.
    fn despawn() -> V<PlaybackSettings> {
        V(PlaybackSettings::DESPAWN)
    }

    /// Settings that play the sound once, then remove the audio components from the entity.
    fn remove() -> V<PlaybackSettings> {
        V(PlaybackSettings::REMOVE)
    }

    /// A copy of the settings with the starting volume set to `linear`, where `1` is the original volume and `0` is silent.
    fn with_volume(_self: R<PlaybackSettings>, linear: f32) -> V<PlaybackSettings> {
        V((*_self).with_volume(Volume::Linear(linear)))
    }

    /// A copy of the settings with the starting speed set to `speed`, where `1` is normal speed.
    fn with_speed(_self: R<PlaybackSettings>, speed: f32) -> V<PlaybackSettings> {
        V((*_self).with_speed(speed))
    }

    /// A copy of the settings with spatial audio turned on or off.
    fn with_spatial(_self: R<PlaybackSettings>, spatial: bool) -> V<PlaybackSettings> {
        V((*_self).with_spatial(spatial))
    }
}

/// Registers the [`PlaybackSettings`] functions.
pub(crate) fn register(world: &mut World) {
    register_playback_settings_functions(world);
}
