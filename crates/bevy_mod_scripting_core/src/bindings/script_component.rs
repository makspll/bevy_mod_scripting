//! Everything necessary to support scripts registering their own components

use std::{alloc::Layout, mem::needs_drop, sync::Arc};

use bevy::{
    ecs::{
        component::{Component, ComponentDescriptor, StorageType},
        reflect::ReflectComponent,
    },
    reflect::{GetTypeRegistration, Reflect, TypeRegistration},
    utils::HashMap,
};
use parking_lot::RwLock;

use crate::error::InteropError;

use super::{ScriptComponentRegistration, ScriptTypeRegistration, ScriptValue, WorldGuard};

/// A dynamic script component, with script set
#[derive(Reflect, Clone)]
#[reflect(Component)]
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
pub struct AppScriptComponentRegistry(pub Arc<RwLock<ScriptComponentRegistry>>);

pub struct ScriptComponentRegistry {
    components: HashMap<String, ScriptComponentInfo>,
}

impl WorldGuard<'_> {
    pub fn script_component_registry(&self) -> AppScriptComponentRegistry {
        let component_registry =
            self.with_resource(|app_component_registry: &AppScriptComponentRegistry| {
                app_component_registry.clone()
            })?;
    }

    /// Registers a dynamic script component, and returns a reference to its registration
    pub fn register_script_component(
        &self,
        component_name: String,
    ) -> Result<ScriptComponentRegistration, InteropError> {
        let component_id = self.with_global_access(|w| {
            let descriptor = unsafe {
                // Safety: same safety guarantees as ComponentDescriptor::new
                // we know the type in advance
                ComponentDescriptor::new_with_layout(
                    component_name,
                    ScriptComponent::STORAGE_TYPE,
                    Layout::new::<ScriptComponent>(),
                    needs_drop::<ScriptComponent>().then_some(|x| x.drop_as::<ScriptComponent>()),
                )
            };
            w.register_component_with_descriptor(descriptor)
        })?;

        // we need to register this as a type in the type registry with this name so its retrievable as any other type
        let type_registry = self.type_registry();
        let mut type_registry = type_registry.write();

        // TODO: we should probably retrieve this from the registry, but I don't see what people would want to register on this type
        // in addition to the existing registrations.
        Ok(ScriptComponentRegistration::new(
            ScriptTypeRegistration::new(Arc::new(
                <ScriptComponent as GetTypeRegistration>::get_type_registration(),
            )),
            component_id,
        ))
    }
}
