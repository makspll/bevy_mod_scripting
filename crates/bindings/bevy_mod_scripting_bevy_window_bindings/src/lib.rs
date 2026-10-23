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
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_window"],
            "window",
        );
    }

    fn build(&self, app: &mut App) {
        use bevy_mod_scripting_bindings::ScriptSystemSetAppExt;
        app.register_script_system_set("window", "ExitSystems", bevy_window::ExitSystems)
            .register_script_system_set(
                "window.CursorSystems",
                "Update",
                bevy_window::CursorSystems::Update,
            );
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
