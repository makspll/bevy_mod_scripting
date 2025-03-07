//! Script related types, functions and components

use crate::{asset::ScriptAsset, IntoScriptPluginParams};
use bevy::{asset::Handle, ecs::system::Resource, reflect::Reflect, utils::HashSet};
use parking_lot::Mutex;
use std::{borrow::Cow, collections::HashMap, ops::Deref, sync::Arc};

/// A unique identifier for a script, by default corresponds to the path of the asset excluding the asset source.
///
/// I.e. an asset with the path `path/to/asset.ext` will have the script id `path/to/asset.ext`
pub type ScriptId = Cow<'static, str>;

#[derive(bevy::ecs::component::Component, Reflect, Clone)]

/// A component which identifies the scripts existing on an entity.
///
/// Event handlers search for components with this component to figure out which scripts to run and on which entities.
pub struct ScriptComponent(pub Vec<ScriptId>);

impl Deref for ScriptComponent {
    type Target = Vec<ScriptId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ScriptComponent {
    /// Creates a new [`ScriptComponent`] with the given ScriptID's
    pub fn new<S: Into<ScriptId>, I: IntoIterator<Item = S>>(components: I) -> Self {
        Self(components.into_iter().map(Into::into).collect())
    }
}

/// All the scripts which are currently loaded or loading and their mapping to contexts
#[derive(Resource)]
pub struct Scripts<P: IntoScriptPluginParams> {
    pub(crate) scripts: HashMap<ScriptId, Script<P>>,
}

impl<P: IntoScriptPluginParams> Default for Scripts<P> {
    fn default() -> Self {
        Self {
            scripts: Default::default(),
        }
    }
}

/// A script
pub struct Script<P: IntoScriptPluginParams> {
    /// The id of the script
    pub id: ScriptId,
    /// the asset holding the content of the script if it comes from an asset
    pub asset: Option<Handle<ScriptAsset>>,
    /// The context of the script, possibly shared with other scripts
    pub context: Arc<Mutex<P::C>>,
}

impl<P: IntoScriptPluginParams> Clone for Script<P> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            asset: self.asset.clone(),
            context: self.context.clone(),
        }
    }
}

/// A collection of scripts, not associated with any entity.
///
/// Useful for `global` or `static` scripts which operate over a larger scope than a single entity.
#[derive(Default, Resource)]
pub struct StaticScripts {
    pub(crate) scripts: HashSet<ScriptId>,
}

impl StaticScripts {
    /// Inserts a static script into the collection
    pub fn insert<S: Into<ScriptId>>(&mut self, script: S) {
        self.scripts.insert(script.into());
    }

    /// Removes a static script from the collection, returning `true` if the script was in the collection, `false` otherwise
    pub fn remove<S: Into<ScriptId>>(&mut self, script: S) -> bool {
        self.scripts.remove(&script.into())
    }

    /// Checks if a static script is in the collection
    /// Returns `true` if the script is in the collection, `false` otherwise
    pub fn contains<S: Into<ScriptId>>(&self, script: S) -> bool {
        self.scripts.contains(&script.into())
    }

    /// Returns an iterator over the static scripts
    pub fn iter(&self) -> impl Iterator<Item = &ScriptId> {
        self.scripts.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_scripts_insert() {
        let mut static_scripts = StaticScripts::default();
        static_scripts.insert("script1");
        assert_eq!(static_scripts.scripts.len(), 1);
        assert!(static_scripts.scripts.contains("script1"));
    }

    #[test]
    fn static_scripts_remove() {
        let mut static_scripts = StaticScripts::default();
        static_scripts.insert("script1");
        assert_eq!(static_scripts.scripts.len(), 1);
        assert!(static_scripts.scripts.contains("script1"));
        assert!(static_scripts.remove("script1"));
        assert_eq!(static_scripts.scripts.len(), 0);
        assert!(!static_scripts.scripts.contains("script1"));
    }

    #[test]
    fn static_scripts_contains() {
        let mut static_scripts = StaticScripts::default();
        static_scripts.insert("script1");
        assert!(static_scripts.contains("script1"));
        assert!(!static_scripts.contains("script2"));
    }
}
