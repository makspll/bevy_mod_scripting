//! Everything necessary to support scripts registering their own components

use super::{InteropError, ScriptComponentRegistration, ScriptValue};
use crate::ScriptNamespaceAppExt;
use ::{
    bevy_app::{App, Plugin},
    bevy_ecs::component::{Component, Mutable, StorageType},
};
use bevy_ecs::{resource::Resource, world::World};
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_platform::collections::HashMap;
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use parking_lot::RwLock;
use std::sync::Arc;

/// A dynamic script component
#[derive(Reflect, Clone, Default, Debug)]
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

/// The prefix all namespaces defined by scripts must start with.
pub const SCRIPT_NAMESPACE_PREFIX: &str = "script.";

/// Joins a script namespace and a name into the full path types are registered under, i.e. `script.game.Enemy`.
pub fn script_path(namespace: &str, name: &str) -> String {
    format!("{namespace}.{name}")
}

/// Checks that a namespace and name given by a script to register something are valid.
///
/// Namespaces defined by scripts must start with `script.` followed by at least one segment, segments are separated by dots and
/// made up of letters, digits and underscores. Names cannot contain dots.
pub fn validate_script_namespace_and_name(namespace: &str, name: &str) -> Result<(), InteropError> {
    let invalid_segment = |segment: &str| {
        segment.is_empty() || !segment.chars().all(|c| c.is_alphanumeric() || c == '_')
    };
    let Some(rest) = namespace.strip_prefix(SCRIPT_NAMESPACE_PREFIX) else {
        return Err(InteropError::string(format!(
            "script namespaces must start with `{SCRIPT_NAMESPACE_PREFIX}` (i.e. `{SCRIPT_NAMESPACE_PREFIX}my_game`), got `{namespace}`. \
            Other namespaces are the names of crates, which scripts cannot add to."
        )));
    };
    if rest.split('.').any(invalid_segment) {
        return Err(InteropError::string(format!(
            "invalid script namespace `{namespace}`: after `{SCRIPT_NAMESPACE_PREFIX}` it must be made up of non empty segments \
            of letters, digits and underscores separated by dots"
        )));
    }
    if name.is_empty() || name.contains('.') {
        return Err(InteropError::string(format!(
            "invalid name `{name}` for something registered in `{namespace}`: it cannot be empty or contain dots"
        )));
    }
    Ok(())
}

/// A function exported by a script, see [`ScriptComponentRegistry::register_export`].
///
/// The function itself stays in the script that exports it: it is a callback of that script, and calls to it run through the callback runners.
#[derive(Clone, Debug)]
pub struct ScriptExport {
    /// The name of the export within its namespace
    pub name: String,
    /// The script namespace the function was exported into
    pub namespace: String,
    /// The script that exported the function, and which runs it
    pub owner: ScriptAttachment,
    /// The label of the callback of the owner that is the exported function
    pub label: String,
}

impl ScriptExport {
    /// The full path of the export, which is unique, i.e. `script.game.spawn_enemy`
    pub fn path(&self) -> String {
        script_path(&self.namespace, &self.name)
    }
}

/// Removes everything a script registered which is tied to the script running: its exports.
///
/// Called when the script is unloaded (also when it is about to be reloaded, the script registers them again when it runs).
/// Components stay registered, as Bevy can not unregister components, scripts re-registering them
/// on reload use `allow_existing`.
pub fn remove_script_registrations(world: &mut World, attachment: &ScriptAttachment) {
    if let Some(registry) = world.get_resource::<AppScriptComponentRegistry>() {
        registry.write().remove_exports_of(attachment);
    }
}

/// Some metadata about dynamic script components
pub struct DynamicComponentInfo {
    /// The name of the component within its namespace
    pub name: String,
    /// The script namespace the component was registered in
    pub namespace: String,
    /// The type registration for the component
    pub registration: ScriptComponentRegistration,
}

impl DynamicComponentInfo {
    /// The full path of the component, which is unique, i.e. `script.game.Enemy`
    pub fn path(&self) -> String {
        script_path(&self.namespace, &self.name)
    }
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
/// A registry of dynamically registered script components and exports
pub struct ScriptComponentRegistry {
    components: HashMap<String, DynamicComponentInfo>,
    exports: HashMap<String, ScriptExport>,
    generation: u64,
}

#[profiling::all_functions]
impl ScriptComponentRegistry {
    /// Registers a dynamic script component, possibly overwriting an existing one
    pub fn register(&mut self, info: DynamicComponentInfo) {
        self.components.insert(info.path(), info);
        self.generation += 1;
    }

    /// Gets a dynamic script component by its full path, i.e. `script.game.Enemy`
    pub fn get(&self, path: &str) -> Option<&DynamicComponentInfo> {
        self.components.get(path)
    }

    /// Gets a dynamic script component by its full path, or by its name alone if no component has that path.
    ///
    /// If several namespaces have a component with that name, the one with the alphabetically first path is returned.
    pub fn find(&self, path_or_name: &str) -> Option<&DynamicComponentInfo> {
        self.components.get(path_or_name).or_else(|| {
            self.components
                .values()
                .filter(|c| c.name == path_or_name)
                .min_by_key(|c| c.path())
        })
    }

    /// A counter which changes whenever a component is registered.
    ///
    /// Lookups of existing registrations do not change it.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Iterates over all registered script components
    pub fn iter_components(&self) -> impl Iterator<Item = &DynamicComponentInfo> {
        self.components.values()
    }

    /// Registers a function exported by a script.
    ///
    /// Exporting the same path again from the same script replaces the export (i.e. when a script runs again),
    /// exporting a path another script exported, or one used by a component, is an error.
    pub fn register_export(&mut self, export: ScriptExport) -> Result<(), InteropError> {
        let path = export.path();
        if let Some(existing) = self.exports.get(&path)
            && existing.owner != export.owner
        {
            return Err(InteropError::string(format!(
                "`{path}` is already exported by script {}, exports have to be unique",
                existing.owner
            )));
        }
        if self.components.contains_key(&path) {
            return Err(InteropError::string(format!(
                "cannot export `{path}`: a script registered component has that path"
            )));
        }
        self.exports.insert(path, export);
        Ok(())
    }

    /// Removes all exports of the given script, returns how many there were
    pub fn remove_exports_of(&mut self, owner: &ScriptAttachment) -> usize {
        let before = self.exports.len();
        self.exports.retain(|_, export| export.owner != *owner);
        before - self.exports.len()
    }

    /// Gets an export by its full path
    pub fn get_export(&self, path: &str) -> Option<&ScriptExport> {
        self.exports.get(path)
    }

    /// Iterates over the exports of the given namespace
    pub fn iter_exports_in<'a>(
        &'a self,
        namespace: &'a str,
    ) -> impl Iterator<Item = &'a ScriptExport> {
        self.exports
            .values()
            .filter(move |e| e.namespace == namespace)
    }

    /// The namespaces with at least one export, in no particular order, possibly repeated
    pub fn export_namespaces(&self) -> impl Iterator<Item = &str> {
        self.exports.values().map(|e| e.namespace.as_str())
    }
}

/// A plugin to support dynamic script components
pub struct DynamicScriptComponentPlugin;

impl Plugin for DynamicScriptComponentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppScriptComponentRegistry>()
            .register_type::<DynamicComponent>()
            .override_script_namespace::<DynamicComponent>("bms");
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
                .register_script_component("script.test", "ScriptTest", false)
                .unwrap()
        };

        let registry = world.get_resource::<AppScriptComponentRegistry>().unwrap();

        let registry = registry.read();
        let info = registry.get("script.test.ScriptTest").unwrap();
        assert_eq!(info.registration.component_id, registration.component_id);
        assert_eq!(info.name, "ScriptTest");
        assert_eq!(info.registration.namespace(), "script.test");

        // can get the component through the world
        let component = world
            .components()
            .get_info(info.registration.component_id)
            .unwrap();

        assert_eq!(component.name(), "script.test.ScriptTest".into());
    }
}
