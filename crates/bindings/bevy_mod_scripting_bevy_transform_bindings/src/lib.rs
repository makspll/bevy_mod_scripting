//! Script bindings for bevy transforms: [`Transform`](bevy_transform::components::Transform) and [`GlobalTransform`](bevy_transform::components::GlobalTransform).

use bevy_app::{App, Plugin};

mod global_transform;
mod transform;

/// Registers the transform script bindings.
#[derive(Default)]
pub struct BevyTransformBindingsPlugin;

impl Plugin for BevyTransformBindingsPlugin {
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_transform"],
            "transform",
        );
    }

    fn build(&self, app: &mut App) {
        use bevy_mod_scripting_bindings::ScriptSystemSetAppExt;
        app.register_script_system_set(
            "transform.TransformSystems",
            "Propagate",
            bevy_transform::TransformSystems::Propagate,
        );
        let world = app.world_mut();
        transform::register(world);
        global_transform::register(world);
    }
}
