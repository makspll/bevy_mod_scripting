use super::*;
use crate::{asset::ScriptAsset, IntoScriptPluginParams};
use bevy::prelude::{Component, ReflectComponent, Deref, DerefMut, Entity};
use bevy::{asset::{Asset, AssetId, Handle}, ecs::system::Resource, reflect::Reflect, utils::HashSet};
use parking_lot::Mutex;
use std::{borrow::Cow, collections::HashMap, ops::Deref, sync::Arc, fmt};

/// A kind of catch all type for script context selection
///
/// I believe this is what the original ScriptId was intended to be.
pub type Domain = Cow<'static, str>;

/// A generic script context provider
pub trait ScriptContextProvider<P: IntoScriptPluginParams> {
    /// Get the context.
    fn get(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<&Arc<Mutex<P::C>>>;
    /// Insert a context.
    fn insert(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>, context: P::C) -> Result<(), P::C>;
    /// Returns true if there is a context.
    fn contains(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool;
}

#[derive(Resource)]
/// Keeps track of script contexts
pub enum ScriptContext<P: IntoScriptPluginParams> {
    /// One shared script context
    Shared(SharedContext<P>),
    /// One script context per entity
    ///
    /// Stores context by entity with a shared context as a last resort when no
    /// entity is provided.
    Entity(EntityContext<P>, SharedContext<P>),
    /// Domain contexts or shared context
    ///
    /// Stores context by domain with a shared context as a last resort when no
    /// domain is provided.
    Domain(DomainContext<P>, SharedContext<P>),
    /// A script context per script
    ScriptId(ScriptIdContext<P>),
    // NOTE: We could also have the following:
    // DomainEntity(DomainContext<P>, EntityContext<P>, SharedContext<P>),
}

impl<P: IntoScriptPluginParams> ScriptContext<P> {
    /// Use a shared script context
    pub fn shared() -> Self {
        Self::Shared(SharedContext::default())
    }
    /// Domain contexts or a shared context
    pub fn with_domains() -> Self {
        Self::Domain(DomainContext::default(), SharedContext::default())
    }
    /// Use one script ontext per entity
    pub fn per_entity() -> Self {
        Self::Entity(EntityContext::default(), SharedContext::default())
    }
    /// Use one script ontext per entity
    pub fn per_script() -> Self {
        Self::ScriptId(ScriptIdContext::default())
    }
}

impl<P: IntoScriptPluginParams> Default for ScriptContext<P> {
    fn default() -> Self {
        Self::shared()
    }
}

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for ScriptContext<P> {
    fn get(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<&Arc<Mutex<P::C>>> {
        match self {
            ScriptContext::Shared(a) => a.get(id, script_id, domain),
            ScriptContext::ScriptId(a) => a.get(id, script_id, domain),
            ScriptContext::Entity(a, b) => a.get(id, script_id, domain)
                                            .or_else(|| b.get(id, script_id, domain)),
            ScriptContext::Domain(a, b) => a.get(id, script_id, domain)
                                            .or_else(|| b.get(id, script_id, domain)),
        }
    }
    fn insert(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>, context: P::C) -> Result<(), P::C> {
        match self {
            ScriptContext::Shared(a) => a.insert(id, script_id, domain, context),
            ScriptContext::ScriptId(a) => a.insert(id, script_id, domain, context),
            ScriptContext::Entity(a, b) => match a.insert(id, script_id, domain, context) {
                Ok(()) => Ok(()),
                Err(context) => b.insert(id, script_id, domain, context)
            }
            ScriptContext::Domain(a, b) => match a.insert(id, script_id, domain, context) {
                Ok(()) => Ok(()),
                Err(context) => b.insert(id, script_id, domain, context)
            }
        }
    }
    fn contains(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool {
        match self {
            ScriptContext::Shared(a) => a.contains(id, script_id, domain),
            ScriptContext::ScriptId(a) => a.contains(id, script_id, domain),
            ScriptContext::Entity(a, b) => a.contains(id, script_id, domain) || b.contains(id, script_id, domain),
            ScriptContext::Domain(a, b) => a.contains(id, script_id, domain) || b.contains(id, script_id, domain),
        }
    }
}

