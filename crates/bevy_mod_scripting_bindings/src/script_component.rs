//! Everything necessary to support scripts registering their own components

use super::{ScriptBundleRegistration, ScriptComponentRegistration, ScriptValue};
use ::{
    bevy_app::{App, Plugin},
    bevy_ecs::component::{Component, Mutable, StorageType},
    bevy_reflect::Reflect,
};
use bevy_ecs::resource::Resource;
use bevy_platform::collections::HashMap;
use bevy_reflect::std_traits::ReflectDefault;
use parking_lot::RwLock;
use std::sync::Arc;
/// A dynamic script component
#[derive(Reflect, Clone, Default)]
#[reflect(Default)]
pub struct DynamicComponent {
    data: ScriptValue,
}

impl DynamicComponent {
    /// Creates a new dynamic component holding the given data
    pub fn new(data: ScriptValue) -> Self {
        Self { data }
    }

    /// Returns the data held by this component
    pub fn data(&self) -> &ScriptValue {
        &self.data
    }
}

/// A dynamic script bundle value.
///
/// `data` maps the names of the bundle's members to their values, members without an entry are default constructed.
#[derive(Reflect, Clone, Default)]
#[reflect(Default)]
pub struct DynamicBundle {
    data: ScriptValue,
}

impl DynamicBundle {
    /// Creates a new dynamic bundle holding the given member values
    pub fn new(data: ScriptValue) -> Self {
        Self { data }
    }

    /// Returns the member values held by this bundle
    pub fn data(&self) -> &ScriptValue {
        &self.data
    }

    /// Consumes the bundle returning the member values
    pub fn into_data(self) -> ScriptValue {
        self.data
    }
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
/// A registry of dynamically registered script components and bundles
pub struct ScriptComponentRegistry {
    components: HashMap<String, DynamicComponentInfo>,
    bundles: HashMap<String, ScriptBundleRegistration>,
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

    /// Registers a dynamic script bundle, possibly overwriting an existing one
    pub fn register_bundle(&mut self, name: String, registration: ScriptBundleRegistration) {
        self.bundles.insert(name, registration);
    }

    /// Gets a dynamic script bundle by name
    pub fn get_bundle(&self, name: &str) -> Option<&ScriptBundleRegistration> {
        self.bundles.get(name)
    }
}

/// A plugin to support dynamic script components
pub struct DynamicScriptComponentPlugin;

impl Plugin for DynamicScriptComponentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppScriptComponentRegistry>()
            .register_type::<DynamicComponent>()
            .register_type::<DynamicBundle>();
    }
}

#[cfg(test)]
mod test {
    use bevy_ecs::world::World;
    use bevy_mod_scripting_world::{WorldAccessGuard, WorldGuard};

    use crate::{CurrentScriptAttachment, WorldExtensions};

    use super::*;

    #[test]
    fn test_script_component() {
        let mut world = World::new();
        world.init_resource::<AppScriptComponentRegistry>();
        let cache = WorldGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        let registration = {
            let guard = WorldAccessGuard::new_exclusive(&mut world, cache);

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

        assert_eq!(component.name(), "ScriptTest".into());
    }
}
