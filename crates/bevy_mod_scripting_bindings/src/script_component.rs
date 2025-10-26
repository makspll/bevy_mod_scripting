//! Everything necessary to support scripts registering their own components

use super::{ScriptComponentRegistration, ScriptTypeRegistration, ScriptValue, WorldAccessGuard};
use crate::error::InteropError;
use ::{
    bevy_app::{App, Plugin},
    bevy_ecs::component::{
        Component, ComponentCloneBehavior, ComponentDescriptor, Mutable, StorageType,
    },
    bevy_reflect::{GetTypeRegistration, Reflect, prelude::ReflectDefault},
};
use bevy_ecs::resource::Resource;
use bevy_platform::collections::HashMap;
use parking_lot::RwLock;
use std::{alloc::Layout, mem::needs_drop, sync::Arc};
/// A dynamic script component
#[derive(Reflect, Clone, Default)]
#[reflect(Default)]
pub struct DynamicComponent {
    data: ScriptValue,
}

/// Some metadata about dynamic script components
pub struct DynamicComponentInfo {
    /// The name of the component
    pub name: String,
    /// The type registration for the component
    pub registration: ScriptComponentRegistration,
}

impl Component for DynamicComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
}

/// A registry of dynamically registered script components
#[derive(Clone, Resource, Default)]
pub struct AppScriptComponentRegistry(pub Arc<RwLock<ScriptComponentRegistry>>);

#[profiling::all_functions]
impl AppScriptComponentRegistry {
    /// Reads the underlying registry
    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, ScriptComponentRegistry> {
        self.0.read()
    }

    /// Writes to the underlying registry
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<'_, ScriptComponentRegistry> {
        self.0.write()
    }
}

#[derive(Default)]
/// A registry of dynamically registered script components
pub struct ScriptComponentRegistry {
    components: HashMap<String, DynamicComponentInfo>,
}

#[profiling::all_functions]
impl ScriptComponentRegistry {
    /// Registers a dynamic script component, possibly overwriting an existing one
    pub fn register(&mut self, info: DynamicComponentInfo) {
        self.components.insert(info.name.clone(), info);
    }

    /// Gets a dynamic script component by name
    pub fn get(&self, name: &str) -> Option<&DynamicComponentInfo> {
        self.components.get(name)
    }
}

#[profiling::all_functions]
impl WorldAccessGuard<'_> {
    /// Registers a dynamic script component, and returns a reference to its registration
    pub fn register_script_component(
        &self,
        component_name: String,
    ) -> Result<ScriptComponentRegistration, InteropError> {
        let component_registry = self.component_registry();
        let component_registry_read = component_registry.read();
        if component_registry_read.get(&component_name).is_some() {
            return Err(InteropError::unsupported_operation(
                None,
                None,
                "script registered component already exists",
            ));
        }

        let component_id = self.with_global_access(|w| {
            let descriptor = unsafe {
                // Safety: same safety guarantees as ComponentDescriptor::new
                // we know the type in advance
                // we only use this method to name the component
                ComponentDescriptor::new_with_layout(
                    component_name.clone(),
                    DynamicComponent::STORAGE_TYPE,
                    Layout::new::<DynamicComponent>(),
                    needs_drop::<DynamicComponent>().then_some(|x| x.drop_as::<DynamicComponent>()),
                    true,
                    ComponentCloneBehavior::Default,
                )
            };
            w.register_component_with_descriptor(descriptor)
        })?;
        drop(component_registry_read);
        let mut component_registry = component_registry.write();

        let registration = ScriptComponentRegistration::new(
            ScriptTypeRegistration::new(Arc::new(
                <DynamicComponent as GetTypeRegistration>::get_type_registration(),
            )),
            component_id,
        );

        let component_info = DynamicComponentInfo {
            name: component_name.clone(),
            registration: registration.clone(),
        };

        component_registry.register(component_info);

        // TODO: we should probably retrieve this from the registry, but I don't see what people would want to register on this type
        // in addition to the existing registrations.
        Ok(registration)
    }
}

/// A plugin to support dynamic script components
pub struct DynamicScriptComponentPlugin;

impl Plugin for DynamicScriptComponentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppScriptComponentRegistry>()
            .register_type::<DynamicComponent>();
    }
}

#[cfg(test)]
mod test {
    use bevy_ecs::world::World;

    use super::*;

    #[test]
    fn test_script_component() {
        let mut world = World::new();
        let registration = {
            let guard = WorldAccessGuard::new_exclusive(&mut world);

            guard
                .register_script_component("ScriptTest".to_string())
                .unwrap()
        };

        let registry = world.get_resource::<AppScriptComponentRegistry>().unwrap();

        let registry = registry.read();
        let info = registry.get("ScriptTest").unwrap();
        assert_eq!(info.registration.component_id, registration.component_id);
        assert_eq!(info.name, "ScriptTest");

        // can get the component through the world
        let component = world
            .components()
            .get_info(info.registration.component_id)
            .unwrap();

        assert_eq!(component.name(), "ScriptTest");
    }
}
