//! Script related types, functions and components

use crate::{asset::ScriptAsset, IntoScriptPluginParams};
use bevy::prelude::{Component, ReflectComponent, Deref, DerefMut, Entity};
use bevy::{asset::{Asset, AssetId, Handle}, ecs::system::Resource, reflect::Reflect, utils::HashSet};
use parking_lot::Mutex;
use std::{borrow::Cow, collections::HashMap, ops::Deref, sync::Arc, fmt};

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
            write!(f, "path {}", path)
        } else {
            write!(f, "id {}", self.0.id())
        }
    }
}

impl<'a, A: Asset> fmt::Debug for HandleDisplay<'a, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(path) = self.0.path() {
            write!(f, "path {:?}", path)
        } else {
            write!(f, "id {:?}", self.0.id())
        }
    }
}

/// Make a type display-able.
pub trait DisplayProxy {
    /// The type that does the displaying.
    type D<'a>: fmt::Display + fmt::Debug where Self: 'a;
    /// Return a display-able reference.
    fn display<'a>(&'a self) -> Self::D<'a>;
}

impl<A: Asset> DisplayProxy for Handle<A> {
    type D<'a> = HandleDisplay<'a, A>;

    fn display<'a>(&'a self) -> Self::D<'a> {
        HandleDisplay(self)
    }
}

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

/// A kind of catch all type for script context selection
///
/// I believe this is what the original ScriptId was intended to be.
pub type Domain = Option<Cow<'static, str>>;

/// A generic script context provider
pub trait ScriptContextProvider<P: IntoScriptPluginParams> {
    /// Get the context.
    fn get(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Domain) -> Option<&Arc<Mutex<P::C>>>;
    /// Insert a context.
    fn insert(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Domain, context: P::C) -> Result<(), P::C>;
    /// Returns true if there is a context.
    fn contains(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Domain) -> bool;
}

#[derive(Resource)]
/// Keeps track of contexts
pub enum ScriptContext<P: IntoScriptPluginParams> {
    /// One shared script context
    Shared(SharedContext<P>),
    /// One script context per entity
    ///
    /// Stores context by entity with a shared context as a last resort when no
    /// entity is provided.
    Entity(EntityContext<P>, SharedContext<P>)
}

impl<P: IntoScriptPluginParams> ScriptContext<P> {
    /// Use a shared script context
    pub fn shared() -> Self {
        Self::Shared(SharedContext::default())
    }
    /// Use one script ontext per entity
    pub fn per_entity() -> Self {
        Self::Entity(EntityContext::default(), SharedContext::default())
    }
}

impl<P: IntoScriptPluginParams> Default for ScriptContext<P> {
    fn default() -> Self {
        Self::shared()
    }
}

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for ScriptContext<P> {
    fn get(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Domain) -> Option<&Arc<Mutex<P::C>>> {
        match self {
            ScriptContext::Shared(a) => a.get(id, script_id, domain),
            ScriptContext::Entity(a, b) => a.get(id, script_id, domain)
                                            .or_else(|| b.get(id, script_id, domain)),
        }
    }
    fn insert(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Domain, context: P::C) -> Result<(), P::C> {
        match self {
            ScriptContext::Shared(a) => a.insert(id, script_id, domain, context),
            ScriptContext::Entity(a, b) => match a.insert(id, script_id, domain, context) {
                Ok(()) => Ok(()),
                Err(context) => b.insert(id, script_id, domain, context)
            }
        }
    }
    fn contains(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Domain) -> bool {
        match self {
            ScriptContext::Shared(a) => a.contains(id, script_id, domain),
            ScriptContext::Entity(a, b) => a.contains(id, script_id, domain) || b.contains(id, script_id, domain),
        }
    }
}

/// Stores the script context by entity.
pub struct EntityContext<P: IntoScriptPluginParams>(HashMap<Entity, Arc<Mutex<P::C>>>);

impl<P: IntoScriptPluginParams> Default for EntityContext<P> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for EntityContext<P> {
    fn get(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Domain) -> Option<&Arc<Mutex<P::C>>> {
        id.and_then(|id| self.0.get(&id))
    }
    fn insert(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Domain, context: P::C) -> Result<(), P::C> {
        if let Some(id) = id {
            self.0.insert(id, Arc::new(Mutex::new(context)));
            Ok(())
        } else {
            Err(context)
        }
    }
    fn contains(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Domain) -> bool {
        id.map(|id| self.0.contains_key(&id)).unwrap_or(false)
    }
}

/// Contains the shared context.
pub struct SharedContext<P: IntoScriptPluginParams>(pub Option<Arc<Mutex<P::C>>>);

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for SharedContext<P> {
    fn get(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Domain) -> Option<&Arc<Mutex<P::C>>> {
        self.0.as_ref()
    }
    fn insert(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Domain, context: P::C) -> Result<(), P::C> {
        self.0 = Some(Arc::new(Mutex::new(context)));
        Ok(())
    }
    fn contains(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Domain) -> bool {
        self.0.is_some()
    }
}

impl<P: IntoScriptPluginParams> Default for SharedContext<P> {
    fn default() -> Self {
        Self(None)
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
    pub fn remove(&mut self, script_id: &ScriptId) -> bool {
        self.scripts.extract_if(|handle| handle.id() == *script_id).next().is_some()
    }

    /// Checks if a static script is in the collection
    /// Returns `true` if the script is in the collection, `false` otherwise
    pub fn contains(&self, script_id: &ScriptId) -> bool {
        self.scripts.iter().any(|handle| handle.id() == *script_id)
    }

    /// Returns an iterator over the static scripts
    pub fn iter(&self) -> impl Iterator<Item = &Handle<ScriptAsset>> {
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
