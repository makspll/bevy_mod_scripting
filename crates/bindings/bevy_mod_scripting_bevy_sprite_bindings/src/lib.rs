//! Script bindings for bevy sprites and texture atlases.

use bevy_app::{App, Plugin};

mod anchor;
mod sprite;
mod texture_atlas_layout;

/// Registers the sprite script bindings.
#[derive(Default)]
pub struct BevySpriteBindingsPlugin;

impl Plugin for BevySpriteBindingsPlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        sprite::register(world);
        anchor::register(world);
        texture_atlas_layout::register(world);
    }
}
