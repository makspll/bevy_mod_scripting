use super::*;
use crate::IntoScriptPluginParams;
use bevy::prelude::{default, Entity};
use bevy::{ecs::system::Resource};
use parking_lot::Mutex;
use std::{borrow::Cow, sync::Arc};

/// A kind of catch all type for script context selection
///
/// I believe this is what the original ScriptId was intended to be.
pub type Domain = Cow<'static, str>;

/// The key for a context.
#[derive(Debug, Hash, Clone, Default, PartialEq, Eq)]
pub struct ContextKey {
    entity: Option<Entity>,
    script_id: Option<ScriptId>,
    domain: Option<Domain>,
}

impl ContextKey {
    /// Is the key empty?
    pub fn is_empty(&self) -> bool {
        self.entity.is_none() && self.script_id.is_none() && self.domain.is_none()
    }

    /// or with other context
    pub fn or(self, other: ContextKey) -> Self {
        Self {
            entity: self.entity.or(other.entity),
            script_id: self.script_id.or(other.script_id),
            domain: self.domain.or(other.domain),
        }
    }

    /// Returns true if self is a subset of other.
    ///
    /// Subset meaning if `self.entity` is `Some`, then other must be `Some` and
    /// equal.
    ///
    /// If `self.entity` is `None`, then other.entity can be anything.
    ///
    /// An empty [ContextKey] is a subset of any context key.
    pub fn is_subset(&self, other: &ContextKey) -> bool {
        self.entity.map(|a| other.entity.map(|b| a == b).unwrap_or(false)).unwrap_or(true)
            || self.script_id.map(|a| other.script_id.map(|b| a == b).unwrap_or(false)).unwrap_or(true)
            || self.domain.as_ref().map(|a| other.domain.as_ref().map(|b| a == b).unwrap_or(false)).unwrap_or(true)
    }
}

impl From<Entity> for ContextKey {
    fn from(entity: Entity) -> Self {
        Self {
            entity: Some(entity),
            ..default()
        }
    }
}

impl From<ScriptId> for ContextKey {
    fn from(script_id: ScriptId) -> Self {
        Self {
            script_id: Some(script_id),
            ..default()
        }
    }
}

impl From<Domain> for ContextKey {
    fn from(domain: Domain) -> Self {
        Self {
            domain: Some(domain),
            ..default()
        }
    }
}


/// A generic script context provider
pub trait ScriptContextProvider<P: IntoScriptPluginParams> {
    /// Get the context.
    fn get(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<&Arc<Mutex<P::C>>>;
    /// Insert a context.
    ///
    /// If the context cannot be inserted, it is returned as an `Err`.
    fn insert(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>, context: P::C) -> Result<(), P::C>;
    /// Returns true if there is a context.
    fn contains(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool;
    /// Hash for context.
    ///
    /// Useful for tracking what context will be returned by `get()` without
    /// requiring that `P::C` impl `Hash` and cheaper too.
    ///
    /// Note: The existence of the hash does not imply the context exists. It
    /// only declares what its hash will be.
    fn hash(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<u64>;

    /// Iterate through contexts.
    fn values(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>>;
    /// Remove a context.
    ///
    /// Returns true if removed.
    fn remove(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool;
}

#[derive(Resource)]
/// Keeps track of script contexts
pub enum ScriptContext<P: IntoScriptPluginParams> {
    /// One shared script context
    Shared(SharedContext<P>),
    /// One shared script context with domains
    DomainShared(Or<DomainContext<P>, SharedContext<P>>),
    /// One script context per entity
    ///
    /// Stores context by entity with a shared context as a last resort when no
    /// entity is provided.
    ///
    /// TODO: Should check for entity despawns and remove from this
    /// EntityContext.
    Entity(Or<EntityContext<P>, SharedContext<P>>),
    /// One script context per entity with domains
    DomainEntity(Or<DomainContext<P>, Or<EntityContext<P>, SharedContext<P>>>),
    /// Domain contexts or shared context
    ///
    /// Stores context by domain with a shared context as a last resort when no
    /// domain is provided.
    Domain(Or<DomainContext<P>, SharedContext<P>>),
    /// A script context per script
    ScriptId(Or<ScriptIdContext<P>, SharedContext<P>>),
    /// A script context per script with domains
    DomainScriptId(Or<DomainContext<P>, Or<ScriptIdContext<P>, SharedContext<P>>>),

    /// One script context per entity per script ID
    ///
    /// Stores context by entity with a shared context as a last resort when no
    /// entity is provided.
    ///
    /// TODO: Should check for entity despawns and remove from this
    /// EntityContext.
    EntityScriptId(Or<EntityScriptIdContext<P>, SharedContext<P>>),
    /// One script context per entity with domains
    DomainEntityScriptId(Or<DomainContext<P>, Or<EntityScriptIdContext<P>, SharedContext<P>>>),
}

impl<P: IntoScriptPluginParams> ScriptContext<P> {
    /// Use a shared script context
    pub fn shared() -> Self {
        Self::Shared(SharedContext::default())
    }
    /// If a domain is given, use that first.
    pub fn with_domains(self) -> Self {
        match self {
            ScriptContext::Shared(a) => ScriptContext::DomainShared(Or(DomainContext::default(), a)),
            ScriptContext::Entity(a) => ScriptContext::DomainEntity(Or(DomainContext::default(), a)),
            ScriptContext::ScriptId(a) => ScriptContext::DomainScriptId(Or(DomainContext::default(), a)),
            ScriptContext::EntityScriptId(a) => ScriptContext::DomainEntityScriptId(Or(DomainContext::default(), a)),
            _ => panic!("Expected `Shared`, `Entity`, `ScriptId`, or `EntityScriptId` for with_domains"),
        }
    }
    /// Domain contexts or a shared context
    pub fn domains() -> Self {
        Self::Domain(Or(DomainContext::default(), SharedContext::default()))
    }
    /// Use one script context per entity
    pub fn per_entity() -> Self {
        Self::Entity(Or(EntityContext::default(), SharedContext::default()))
    }
    /// Use one script context per entity
    pub fn per_script() -> Self {
        Self::ScriptId(Or(ScriptIdContext::default(), SharedContext::default()))
    }
    /// Use one script context per entity and script
    pub fn per_entity_and_script() -> Self {
        Self::EntityScriptId(Or(EntityScriptIdContext::default(), SharedContext::default()))
    }
}

/// Use one script context per entity by default; see [ScriptContext::per_script].
impl<P: IntoScriptPluginParams> Default for ScriptContext<P> {
    fn default() -> Self {
        Self::per_entity_and_script()
    }
}

/// Compose two ScriptContextProviders in a short-circuit OR relationship. Use T
/// first, failing that use U.
///
/// The iter() method does not short-circuit but returns both results
pub struct Or<T, U>(pub T, pub U);

impl<T: ScriptContextProvider<P>, U: ScriptContextProvider<P>, P: IntoScriptPluginParams> ScriptContextProvider<P> for Or<T, U> {
    #[inline]
    fn get(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<&Arc<Mutex<P::C>>> {
        self.0.get(id, script_id, domain).or_else(|| self.1.get(id, script_id, domain))
    }
    #[inline]
    fn insert(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>, context: P::C) -> Result<(), P::C> {
        match self.0.insert(id, script_id, domain, context) {
            Ok(()) => Ok(()),
            Err(context) => self.1.insert(id, script_id, domain, context)
        }
    }
    #[inline]
    fn contains(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool {
        self.0.contains(id, script_id, domain) || self.1.contains(id, script_id, domain)
    }
    #[inline]
    fn hash(&self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<u64> {
        self.0.hash(id, script_id, domain)
              .or_else(|| self.1.hash(id, script_id, domain))
    }
    #[inline]
    fn values(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>> {
        self.0.values().chain(self.1.values())
    }
    #[inline]
    fn remove(&mut self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool {
        self.0.remove(id, script_id, domain) || self.1.remove(id, script_id, domain)
    }
}

macro_rules! delegate_to_variants {
    (
        $(
            fn $fn_name:ident ($self:ty, $( $arg:ident : $arg_ty:ty ),* ) -> $ret:ty
        ),* $(,)?
    ) => {
        $(
            fn $fn_name(self: $self, $( $arg: $arg_ty ),*) -> $ret {
                match self {
                    ScriptContext::Shared(a) => a.$fn_name($( $arg ),*),
                    ScriptContext::ScriptId(a) => a.$fn_name($( $arg ),*),
                    ScriptContext::Entity(a) => a.$fn_name($( $arg ),*),
                    ScriptContext::EntityScriptId(a) => a.$fn_name($( $arg ),*),
                    ScriptContext::Domain(a) => a.$fn_name($( $arg ),*),
                    ScriptContext::DomainShared(a) => a.$fn_name($( $arg ),*),
                    ScriptContext::DomainScriptId(a) => a.$fn_name($( $arg ),*),
                    ScriptContext::DomainEntity(a) => a.$fn_name($( $arg ),*),
                    ScriptContext::DomainEntityScriptId(a) => a.$fn_name($( $arg ),*),
                }
            }
        )*
    };
}

impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for ScriptContext<P> {
    delegate_to_variants! {
        fn get(&Self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<&Arc<Mutex<P::C>>>,
        fn contains(&Self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool,
        fn hash(&Self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> Option<u64>,
        fn insert(&mut Self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>, context: P::C) -> Result<(), P::C>,
        fn remove(&mut Self, id: Option<Entity>, script_id: &ScriptId, domain: &Option<Domain>) -> bool,
    }

    fn values(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>> {
        panic!("Must used iter_box() with ScriptContext<P>.");
        std::iter::empty()
    }
}

impl<P: IntoScriptPluginParams> ScriptContext<P> {
    /// Return an iterator for contexts
    pub fn values_box(&self) -> Box<dyn Iterator<Item = &Arc<Mutex<P::C>>> + '_> {
        match self {
            ScriptContext::Shared(a) => Box::new(a.values()),
            ScriptContext::ScriptId(a) => Box::new(a.values()),
            ScriptContext::Entity(a) => Box::new(a.values()),
            ScriptContext::EntityScriptId(a) => Box::new(a.values()),
            ScriptContext::Domain(a) => Box::new(a.values()),
            ScriptContext::DomainShared(a) => Box::new(a.values()),
            ScriptContext::DomainScriptId(a) => Box::new(a.values()),
            ScriptContext::DomainEntity(a) => Box::new(a.values()),
            ScriptContext::DomainEntityScriptId(a) => Box::new(a.values()),
        }
    }
}
