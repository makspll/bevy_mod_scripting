//! [`AnimationPlayer`](bevy_animation::AnimationPlayer): plays, pauses and seeks the animations of an entity.

// `script_bindings` expects these in scope
use bevy_animation::{AnimationPlayer, graph::AnimationNodeIndex};
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::{
    InteropError,
    function::from::{M, R},
};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "animation_player_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "animation",
    core
)]
impl AnimationPlayer {
    /// Starts playing the animation `node`. Does nothing if it is already playing.
    fn play(mut _self: M<AnimationPlayer>, node: R<AnimationNodeIndex>) {
        _self.play(*node);
    }

    /// Stops the animation `node` and removes it from the player.
    fn stop(mut _self: M<AnimationPlayer>, node: R<AnimationNodeIndex>) {
        _self.stop(*node);
    }

    /// Stops all animations and removes them from the player.
    fn stop_all(mut _self: M<AnimationPlayer>) {
        _self.stop_all();
    }

    /// Pauses all playing animations.
    fn pause_all(mut _self: M<AnimationPlayer>) {
        _self.pause_all();
    }

    /// Resumes all paused animations.
    fn resume_all(mut _self: M<AnimationPlayer>) {
        _self.resume_all();
    }

    /// Whether the animation `node` is playing or paused.
    fn is_playing_animation(_self: R<AnimationPlayer>, node: R<AnimationNodeIndex>) -> bool {
        _self.is_playing_animation(*node)
    }

    /// Whether every playing animation has finished. True when nothing is playing.
    fn all_finished(_self: R<AnimationPlayer>) -> bool {
        _self.all_finished()
    }

    /// Whether every playing animation is paused. True when nothing is playing.
    fn all_paused(_self: R<AnimationPlayer>) -> bool {
        _self.all_paused()
    }

    /// Whether the animation `node` has finished, or nil if it is not playing.
    fn animation_is_finished(
        _self: R<AnimationPlayer>,
        node: R<AnimationNodeIndex>,
    ) -> Option<bool> {
        _self
            .animation(*node)
            .map(|animation| animation.is_finished())
    }

    /// Sets the playback speed of the animation `node`, where `1.0` is normal speed. Errors if it is not playing.
    fn set_speed(
        mut _self: M<AnimationPlayer>,
        node: R<AnimationNodeIndex>,
        speed: f32,
    ) -> Result<(), InteropError> {
        match _self.animation_mut(*node) {
            Some(animation) => {
                animation.set_speed(speed);
                Ok(())
            }
            None => Err(InteropError::str("the animation is not playing")),
        }
    }

    /// Makes the animation `node` repeat forever. Errors if it is not playing.
    fn set_repeat_forever(
        mut _self: M<AnimationPlayer>,
        node: R<AnimationNodeIndex>,
    ) -> Result<(), InteropError> {
        match _self.animation_mut(*node) {
            Some(animation) => {
                animation.repeat();
                Ok(())
            }
            None => Err(InteropError::str("the animation is not playing")),
        }
    }

    /// Jumps the animation `node` to `seconds` into its clip. Errors if it is not playing.
    fn seek_to(
        mut _self: M<AnimationPlayer>,
        node: R<AnimationNodeIndex>,
        seconds: f32,
    ) -> Result<(), InteropError> {
        match _self.animation_mut(*node) {
            Some(animation) => {
                animation.seek_to(seconds);
                Ok(())
            }
            None => Err(InteropError::str("the animation is not playing")),
        }
    }
}

/// Registers the [`AnimationPlayer`] functions.
pub(crate) fn register(world: &mut World) {
    register_animation_player_functions(world);
}
