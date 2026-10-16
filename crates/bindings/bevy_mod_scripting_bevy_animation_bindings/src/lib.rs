//! Script bindings for bevy animation playback.

use bevy_app::{App, Plugin};

mod animation_graph;
mod animation_node_index;
mod animation_player;

/// Registers the animation script bindings.
#[derive(Default)]
pub struct BevyAnimationBindingsPlugin;

impl Plugin for BevyAnimationBindingsPlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        animation_player::register(world);
        animation_node_index::register(world);
        animation_graph::register(world);
    }
}
