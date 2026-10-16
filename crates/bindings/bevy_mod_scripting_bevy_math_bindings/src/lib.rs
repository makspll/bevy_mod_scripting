//! Script bindings for bevy math: vectors, quaternions, matrices, directions, rectangles, rays, shapes and easing.

use bevy_app::{App, Plugin};

mod dir;
mod ease;
mod ivec;
mod mat;
mod quat;
mod ray;
mod rect;
mod rot2;
mod shapes;
mod vec;

/// Registers the math script bindings.
#[derive(Default)]
pub struct BevyMathBindingsPlugin;

impl Plugin for BevyMathBindingsPlugin {
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["glam", "bevy_math", "bevy_curve", "bevy_shape"],
            "math",
        );
    }

    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        vec::register(world);
        ivec::register(world);
        quat::register(world);
        mat::register(world);
        dir::register(world);
        rot2::register(world);
        rect::register(world);
        ray::register(world);
        shapes::register(world);
        ease::register(world);
    }
}
