//! Script bindings for bevy input: keyboard, mouse, gamepads, touch and focus.
//!
//! The button states are read from their resources, so `KeyCode.just_pressed(KeyCode.Space)` works without holding any reference.

use std::{any::TypeId, sync::Arc};

use bevy_app::{App, Plugin};
use bevy_ecs::{component::Component, resource::Resource};
use bevy_mod_scripting_bindings::{
    InteropError, ScriptComponentRegistration, ScriptTypeRegistration, WorldExtensions, WorldGuard,
};

mod button;
mod focus;
mod gamepad;

/// Registers the input script bindings.
#[derive(Default)]
pub struct BevyInputBindingsPlugin;

impl Plugin for BevyInputBindingsPlugin {
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_input", "bevy_input_focus", "bevy_picking"],
            "input",
        );
    }

    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        button::register(world);
        gamepad::register(world);
        focus::register(world);
    }
}

/// Runs `f` on a resource, or returns `default` if the resource is not in the world.
pub(crate) fn with_resource_or<Res: Resource, O>(
    world: &WorldGuard,
    default: O,
    f: impl FnOnce(&Res) -> O,
) -> Result<O, InteropError> {
    match world.with_resource(f) {
        Ok(value) => Ok(value),
        Err(
            InteropError::MissingResource { .. }
            | InteropError::UnregisteredComponentOrResourceType { .. },
        ) => Ok(default),
        Err(error) => Err(error),
    }
}

/// The script registration of the component `C`, or nil if the component is not registered.
pub(crate) fn component_registration<C: Component>(
    world: &WorldGuard,
) -> Option<ScriptComponentRegistration> {
    let registry = world.type_registry();
    let registration = registry.read().get(TypeId::of::<C>()).cloned()?;
    world
        .get_component_type(ScriptTypeRegistration::new(Arc::new(registration)))
        .ok()?
        .ok()
}
