//! Script bindings for bevy transforms: [`Transform`](bevy_transform::components::Transform) and [`GlobalTransform`](bevy_transform::components::GlobalTransform).

use bevy_app::{App, Plugin};

mod global_transform;
mod transform;

/// Registers the transform script bindings.
#[derive(Default)]
pub struct BevyTransformBindingsPlugin;

impl Plugin for BevyTransformBindingsPlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        transform::register(world);
        global_transform::register(world);
    }
}
