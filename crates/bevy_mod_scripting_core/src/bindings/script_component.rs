//! Everything necessary to support scripts registering their own components

use super::{ScriptComponentRegistration, ScriptTypeRegistration, ScriptValue, WorldAccessGuard};
use crate::error::InteropError;
use bevy::{
    app::{App, Plugin},
    ecs::{
        component::{Component, ComponentDescriptor, StorageType},
        system::Resource,
    },
    reflect::{prelude::ReflectDefault, GetTypeRegistration, Reflect},
    utils::HashMap,
};
use parking_lot::RwLock;
use std::{alloc::Layout, mem::needs_drop, sync::Arc};

/// A dynamic script component, with script set
#[derive(Reflect, Clone, Default)]
#[reflect(Default)]
pub struct ScriptComponent {
    data: ScriptValue,
}

/// Some metadata about dynamic script components
pub struct ScriptComponentInfo {
    /// The name of the component
    pub name: String,
    /// The type registration for the component
    pub registration: ScriptComponentRegistration,
}

impl Component for ScriptComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;
}

/// A registry of dynamically registered script components
#[derive(Clone, Resource, Default)]
pub struct AppScriptComponentRegistry(pub Arc<RwLock<ScriptComponentRegistry>>);

impl AppScriptComponentRegistry {
    /// Reads the underlying registry
    pub fn read(&self) -> parking_lot::RwLockReadGuard<ScriptComponentRegistry> {
        self.0.read()
    }

    /// Writes to the underlying registry
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<ScriptComponentRegistry> {
        self.0.write()
    }
}

#[derive(Default)]
/// A registry of dynamically registered script components
pub struct ScriptComponentRegistry {
    components: HashMap<String, ScriptComponentInfo>,
}

impl ScriptComponentRegistry {
    /// Registers a dynamic script component, possibly overwriting an existing one
    pub fn register(&mut self, info: ScriptComponentInfo) {
        self.components.insert(info.name.clone(), info);
    }

    /// Gets a dynamic script component by name
    pub fn get(&self, name: &str) -> Option<&ScriptComponentInfo> {
        self.components.get(name)
    }
}

impl WorldAccessGuard<'_> {
    /// Registers a dynamic script component, and returns a reference to its registration
    pub fn register_script_component(
        &self,
        component_name: String,
    ) -> Result<ScriptComponentRegistration, InteropError> {
        if !component_name.starts_with("Script") {
            return Err(InteropError::unsupported_operation(
                None,
                None,
                "script registered component name must start with 'Script'",
            ));
        }
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
            bevy::log::info!(
                "components present: {}. script: {}. World id: {:?}",
                w.components().len(),
                component_registry_read.components.len(),
                w.id()
            );
            let descriptor = unsafe {
                // Safety: same safety guarantees as ComponentDescriptor::new
                // we know the type in advance
                // we only use this method to name the component
                ComponentDescriptor::new_with_layout(
                    component_name.clone(),
                    ScriptComponent::STORAGE_TYPE,
                    Layout::new::<ScriptComponent>(),
                    needs_drop::<ScriptComponent>().then_some(|x| x.drop_as::<ScriptComponent>()),
                )
            };
            let o = w.register_component_with_descriptor(descriptor);
            bevy::log::info!("components present after: {}", w.components().len());
            o
        })?;
        drop(component_registry_read);
        let mut component_registry = component_registry.write();

        let registration = ScriptComponentRegistration::new(
            ScriptTypeRegistration::new(Arc::new(
                <ScriptComponent as GetTypeRegistration>::get_type_registration(),
            )),
            component_id,
        );

        bevy::log::debug!(
            "Registering dynamic script component: {}, component id assigned: {:?}",
            component_name,
            component_id
        );

        let component_info = ScriptComponentInfo {
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
pub(crate) struct DynamicScriptComponentPlugin;

impl Plugin for DynamicScriptComponentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppScriptComponentRegistry>()
            .register_type::<ScriptComponent>();
    }
}

#[cfg(test)]
mod test {
    use std::ptr::NonNull;

    use super::*;
    use bevy::{ecs::world::World, ptr::OwningPtr};

    #[test]
    fn test_script_component() {
        let mut world = World::new();
        let component_name = "MyScriptComponent";

        #[derive(Reflect, Component)]
        struct UnderlyingComponent;

        // initialize component descriptor dynamically
        let descriptor = unsafe {
            // Safety: same safety guarantees as ComponentDescriptor::new
            // we know the type in advance
            // we only use this method to name the component
            ComponentDescriptor::new_with_layout(
                component_name,
                UnderlyingComponent::STORAGE_TYPE,
                Layout::new::<UnderlyingComponent>(),
                needs_drop::<UnderlyingComponent>()
                    .then_some(|x| x.drop_as::<UnderlyingComponent>()),
            )
        };

        // register with the world
        let component_id = world.register_component_with_descriptor(descriptor.clone());
        let component_id_2 = world.register_component_with_descriptor(descriptor);
        assert_eq!(component_id, component_id_2); // iam getting a double free for this in scritps somehow

        // insert into the entity
        let entity = world.spawn_empty().id();
        let mut entity = world.entity_mut(entity);

        let value = Box::new(UnderlyingComponent);
        let value_ref = Box::into_raw(value).cast::<u8>();
        let ptr = unsafe { OwningPtr::new(NonNull::new(value_ref).unwrap()) };
        unsafe { entity.insert_by_id(component_id, ptr) };

        // check it gets inserted
        assert!(
            entity.contains_id(component_id),
            "entity does not contain freshly inserted component"
        )
    }
}
