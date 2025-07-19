use super::*;
use crate::IntoScriptPluginParams;
use bevy::{ecs::system::Resource, log::trace};
use parking_lot::Mutex;
use std::{sync::Arc, hash::Hash};

/// A kind of catch all type for script context selection
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Domain(u64);

impl Domain {
    /// Create a domain handle.
    pub fn new(hashable: impl Hash) -> Self {
        Domain(DefaultHashBuilder::default().hash_one(hashable))
    }
}

/// A generic script context provider
pub trait ScriptContextProvider<P: IntoScriptPluginParams> {
    /// Get the context.
    fn get(&self, context_key: &ContextKey) -> Option<&Arc<Mutex<P::C>>>;
    /// Insert a context.
    ///
    /// If the context cannot be inserted, it is returned as an `Err`.
    fn insert(&mut self, context_key: ContextKey, context: P::C) -> Result<(), P::C>;
    /// Returns true if there is a context.
    fn contains(&self, context_key: &ContextKey) -> bool;
    /// Hash for context.
    ///
    /// Useful for tracking what context will be returned by `get()` without
    /// requiring that `P::C` impl `Hash` and cheaper too.
    ///
    /// Note: The existence of the hash does not imply the context exists. It
    /// only declares what its hash will be.
    fn hash(&self, context_key: &ContextKey) -> Option<u64>;

    /// Iterate through contexts.
    fn values(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>>;
    /// Remove a context.
    ///
    /// Returns true if removed.
    fn remove(&mut self, context_key: &ContextKey) -> bool;

    /// Iterate through keys and contexts.
    fn iter(&self) -> impl Iterator<Item = (ContextKey, &Arc<Mutex<P::C>>)>;
    // fn values_if(&self, pred: impl FnMut(&ContextKey) -> bool) -> impl Iterator<Item = &Arc<Mutex<P::C>>>;
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
    EntityScriptId(Or<EntityScriptIdContext<P>, Or<ScriptIdContext<P>, SharedContext<P>>>),
    /// One script context per entity with domains
    DomainEntityScriptId(Or<DomainContext<P>, Or<EntityScriptIdContext<P>, Or<ScriptIdContext<P>, SharedContext<P>>>>),
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
            _ => {
                // It aleady handles domains.
                self
            }
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
        Self::EntityScriptId(Or(EntityScriptIdContext::default(), Or(ScriptIdContext::default(), SharedContext::default())))
    }
}

/// Use one script context per entity and script with domains by default; see
/// [ScriptContext::per_entity_and_script].
impl<P: IntoScriptPluginParams> Default for ScriptContext<P> {
    fn default() -> Self {
        Self::per_entity_and_script().with_domains()
    }
}

/// Compose two ScriptContextProviders in a short-circuit OR relationship. Use T
/// first, failing that use U.
///
/// The iter() method does not short-circuit but returns both results
pub struct Or<T, U>(pub T, pub U);

impl<T: ScriptContextProvider<P>, U: ScriptContextProvider<P>, P: IntoScriptPluginParams> ScriptContextProvider<P> for Or<T, U> {
    #[inline]
    fn get(&self, context_key: &ContextKey) -> Option<&Arc<Mutex<P::C>>> {
        self.0.get(context_key).or_else(|| self.1.get(context_key))
    }
    #[inline]
    fn insert(&mut self, context_key: ContextKey, context: P::C) -> Result<(), P::C> {
        trace!("insert context for {}", &context_key);
        match self.0.insert(context_key.clone(), context) {
            Ok(()) => Ok(()),
            Err(context) => self.1.insert(context_key, context)
        }
    }
    #[inline]
    fn contains(&self, context_key: &ContextKey) -> bool {
        self.0.contains(context_key) || self.1.contains(context_key)
    }
    #[inline]
    fn hash(&self, context_key: &ContextKey) -> Option<u64> {
        self.0.hash(context_key)
              .or_else(|| self.1.hash(context_key))
    }
    #[inline]
    fn values(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>> {
        self.0.values().chain(self.1.values())
    }
    #[inline]
    fn remove(&mut self, context_key: &ContextKey) -> bool {
        self.0.remove(context_key) || self.1.remove(context_key)
    }
    #[inline]
    fn iter(&self) -> impl Iterator<Item = (ContextKey, &Arc<Mutex<P::C>>)> {
        self.0.iter().chain(self.1.iter())
    }
}

macro_rules! delegate_to_variants {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis
            fn $fn_name:ident ($self:ty, $( $arg:ident : $arg_ty:ty ),* ) -> $ret:ty
        ),* $(,)?
    ) => {
        $(
            $(#[$meta])*
            $vis
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

// impl<P: IntoScriptPluginParams> ScriptContextProvider<P> for ScriptContext<P> {
//
/// We don't implement `ScriptContextProvider<P>` because we want to return a
/// `Box<dyn Iterator>` as a special case. So we implement mostly the same
/// interface but do not actually implement it.
impl<P: IntoScriptPluginParams> ScriptContext<P> {
    delegate_to_variants! {
        /// Get
        pub fn get(&Self, context_key: &ContextKey) -> Option<&Arc<Mutex<P::C>>>,
        /// Contains
        pub fn contains(&Self, context_key: &ContextKey) -> bool,
        /// Hash
        pub fn hash(&Self, context_key: &ContextKey) -> Option<u64>,
        /// Insert
        pub fn insert(&mut Self, context_key: ContextKey, context: P::C) -> Result<(), P::C>,
        /// Remove
        pub fn remove(&mut Self, context_key: &ContextKey) -> bool,
    }

    /// Return an iterator for contexts
    pub fn values(&self) -> Box<dyn Iterator<Item = &Arc<Mutex<P::C>>> + '_> {
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

    /// Return an iterator of the keys and contexts.
    pub fn iter(&self) -> Box<dyn Iterator<Item = (ContextKey, &Arc<Mutex<P::C>>)> + '_> {
        match self {
            ScriptContext::Shared(a) => Box::new(a.iter()),
            ScriptContext::ScriptId(a) => Box::new(a.iter()),
            ScriptContext::Entity(a) => Box::new(a.iter()),
            ScriptContext::EntityScriptId(a) => Box::new(a.iter()),
            ScriptContext::Domain(a) => Box::new(a.iter()),
            ScriptContext::DomainShared(a) => Box::new(a.iter()),
            ScriptContext::DomainScriptId(a) => Box::new(a.iter()),
            ScriptContext::DomainEntity(a) => Box::new(a.iter()),
            ScriptContext::DomainEntityScriptId(a) => Box::new(a.iter()),
        }
    }
}
