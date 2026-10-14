//! Script bindings for bevy windows: the [`Window`] component and its [`WindowResolution`].

use std::{any::TypeId, sync::Arc};

use bevy_app::{App, Plugin};
use bevy_ecs::component::Component;
use bevy_mod_scripting_bindings::{
    ScriptComponentRegistration, ScriptTypeRegistration, WorldExtensions, WorldGuard,
};

mod resolution;
mod window;

/// Registers the window script bindings.
#[derive(Default)]
pub struct BevyWindowBindingsPlugin;

impl Plugin for BevyWindowBindingsPlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        window::register(world);
        resolution::register(world);
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
