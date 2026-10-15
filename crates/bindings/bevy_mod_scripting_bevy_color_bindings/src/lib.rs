//! Script bindings for bevy color.

use bevy_app::{App, Plugin};

mod color;
mod colorspace;
mod linear_rgba;
mod srgba;

/// Registers the color script bindings.
#[derive(Default)]
pub struct BevyColorBindingsPlugin;

impl Plugin for BevyColorBindingsPlugin {
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_color"],
            "color",
        );
    }

    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        color::register(world);
        srgba::register(world);
        linear_rgba::register(world);
        colorspace::register(world);
    }
}
