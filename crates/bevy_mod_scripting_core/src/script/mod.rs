//! Script related types, functions and components

use crate::{asset::ScriptAsset, IntoScriptPluginParams};
use bevy::prelude::{Component, Entity, ReflectComponent};
use bevy::utils::hashbrown::hash_map::DefaultHashBuilder;
use bevy::{
    asset::{Asset, AssetId, Handle},
    ecs::system::Resource,
    reflect::Reflect,
    utils::HashSet,
};
use parking_lot::Mutex;
use std::{collections::HashMap, fmt, hash::BuildHasher, ops::Deref, sync::Arc};

mod key;
mod script_context;
pub use key::*;
pub use script_context::*;

/// A unique identifier for a script, by default corresponds to the path of the asset excluding the asset source.
///
/// I.e. an asset with the path `path/to/asset.ext` will have the script id `path/to/asset.ext`
pub type ScriptId = AssetId<ScriptAsset>;

/// Display the path of a script or its asset ID.
#[doc(hidden)]
pub struct HandleDisplay<'a, T: Asset>(&'a Handle<T>);

impl<'a, A: Asset> fmt::Display for HandleDisplay<'a, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(path) = self.0.path() {
            write!(f, "path {path}")
        } else {
            write!(f, "id {}", self.0.id())
        }
    }
}

impl<'a, A: Asset> fmt::Debug for HandleDisplay<'a, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(path) = self.0.path() {
            write!(f, "path {path:?}")
        } else {
            write!(f, "id {:?}", self.0.id())
        }
    }
}

/// Make a type display-able.
pub trait DisplayProxy {
    /// The type that does the displaying.
    type D<'a>: fmt::Display + fmt::Debug
    where
        Self: 'a;
    /// Return a display-able reference.
    fn display<'a>(&'a self) -> Self::D<'a>;
}

impl<A: Asset> DisplayProxy for Handle<A> {
    type D<'a> = HandleDisplay<'a, A>;

    fn display<'a>(&'a self) -> Self::D<'a> {
        HandleDisplay(self)
    }
}

/// Defines the domain of a script
#[derive(Component)]
pub struct ScriptDomain(pub Domain);

#[derive(bevy::ecs::component::Component, Reflect, Clone)]
#[reflect(Component)]
/// A component which identifies the scripts existing on an entity.
///
/// Event handlers search for components with this component to figure out which scripts to run and on which entities.
pub struct ScriptComponent(pub Vec<Handle<ScriptAsset>>);

impl Deref for ScriptComponent {
    type Target = Vec<Handle<ScriptAsset>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ScriptComponent {
    /// Creates a new [`ScriptComponent`] with the given ScriptID's
    pub fn new<S: Into<Handle<ScriptAsset>>, I: IntoIterator<Item = S>>(components: I) -> Self {
        Self(components.into_iter().map(Into::into).collect())
    }
}

/// A collection of scripts, not associated with any entity.
///
/// Useful for `global` or `static` scripts which operate over a larger scope than a single entity.
#[derive(Default, Resource)]
pub struct StaticScripts {
    pub(crate) scripts: HashSet<Handle<ScriptAsset>>,
}

#[profiling::all_functions]
impl StaticScripts {
    /// Inserts a static script into the collection
    pub fn insert<S: Into<Handle<ScriptAsset>>>(&mut self, script: S) {
        self.scripts.insert(script.into());
    }

    /// Removes a static script from the collection, returning `true` if the script was in the collection, `false` otherwise
    pub fn remove(&mut self, script_id: impl Into<ScriptId>) -> bool {
        let script_id = script_id.into();
        self.scripts
            .extract_if(|handle| handle.id() == script_id)
            .next()
            .is_some()
    }

    /// Checks if a static script is in the collection
    /// Returns `true` if the script is in the collection, `false` otherwise
    pub fn contains(&self, script_id: impl Into<ScriptId>) -> bool {
        let script_id = script_id.into();
        self.scripts.iter().any(|handle| handle.id() == script_id)
    }

    /// Returns an iterator over the static scripts
    pub fn values(&self) -> impl Iterator<Item = &Handle<ScriptAsset>> {
        self.scripts.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_scripts_insert() {
        let mut static_scripts = StaticScripts::default();
        let script1 = Handle::default();
        static_scripts.insert(script1.clone());
        assert_eq!(static_scripts.scripts.len(), 1);
        assert!(static_scripts.scripts.contains(&script1));
    }

    #[test]
    fn static_scripts_remove() {
        let mut static_scripts = StaticScripts::default();
        let script1 = Handle::default();
        static_scripts.insert(script1.clone());
        assert_eq!(static_scripts.scripts.len(), 1);
        assert!(static_scripts.scripts.contains(&script1));
        assert!(static_scripts.remove(&script1));
        assert_eq!(static_scripts.scripts.len(), 0);
        assert!(!static_scripts.scripts.contains(&script1));
    }

    fn scriptid_from_u128(uuid: u128) -> ScriptId {
        ScriptId::from(uuid::Builder::from_random_bytes(uuid.to_le_bytes()).into_uuid())
    }

    fn handle_from_u128(uuid: u128) -> Handle<ScriptAsset> {
        Handle::Weak(scriptid_from_u128(uuid))
    }

    #[test]
    fn static_scripts_contains() {
        let mut static_scripts = StaticScripts::default();
        let script1 = handle_from_u128(0);
        let script2 = handle_from_u128(1);
        static_scripts.insert(script1.clone());
        assert!(static_scripts.contains(&script1));
        assert!(!static_scripts.contains(&script2));
    }
}
