use super::*;
use crate::IntoScriptPluginParams;
use bevy::ecs::system::Resource;
use parking_lot::Mutex;
use std::{hash::Hash, sync::Arc};

/// A kind of catch all type for script context selection
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Domain(u64);

impl Domain {
    /// Create a domain handle.
    pub fn new(hashable: impl Hash) -> Self {
        Domain(DefaultHashBuilder::default().hash_one(hashable))
    }
}

/// Determines how contexts are grouped by manipulating the context key.
pub trait ContextKeySelector {
    /// The given context key represents a possible script, entity, domain that
    /// is requesting a context.
    ///
    /// This selector returns
    ///  - `None` when the given `context_key` is not relevant to its policy, or
    ///  - `Some(selected_key)` when the appropriate key has been determined.
    fn select(&self, context_key: &ContextKey) -> Option<ContextKey>;
}

impl<F: Fn(&ContextKey) -> Option<ContextKey>> ContextKeySelector for F {
    fn select(&self, context_key: &ContextKey) -> Option<ContextKey> {
        (self)(context_key)
    }
}

/// A rule for context selection
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextRule {
    /// If domain exists, return only that.
    Domain,
    /// If entity-script pair exists, return only that.
    EntityScript,
    /// If entity exists, return only that.
    Entity,
    /// If script exists, return only that.
    Script,
    /// Check nothing; return empty context key.
    Shared,
    /// A custom rule
    Custom(fn(&ContextKey) -> Option<ContextKey>)
    // XXX: Custom rule with this opaque type makes it harder to have the
    // derives above that we might want. So maybe we drop this variant.
    // Custom(Box<dyn ContextKeySelector + 'static + Sync + Send>)
}

impl ContextKeySelector for ContextRule {
    /// Depending on the enum variant, executes that rule.
    ///
    /// For example a rule of `Domain` will check for a domain in the
    /// `context_key`. If it is present, a ContextKey that only
    /// has that domain will be returned.
    fn select(&self, context_key: &ContextKey) -> Option<ContextKey> {
        match self {
            ContextRule::Domain => context_key.domain.map(ContextKey::from),
            ContextRule::Entity => context_key.entity.map(ContextKey::from),
            ContextRule::Script => context_key.script.clone().map(ContextKey::from),
            ContextRule::EntityScript => context_key.entity.zip(context_key.script.clone()).map(|(entity, script)| ContextKey {
                entity: Some(entity),
                script: Some(script),
                domain: None
            }),
            ContextRule::Shared => Some(ContextKey::default()),
            ContextRule::Custom(rule) => rule.select(context_key),
        }
    }
}

/// This is a configurable context policy based on priority.
#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextPolicy {
    /// The rules in order of priority.
    pub priorities: Vec<ContextRule>,
}

/// Returns a `[Domain, EntityScript, Script, Shared]` policy.
impl Default for ContextPolicy {
    fn default() -> Self {
        ContextPolicy { priorities: vec![
            ContextRule::Domain,
            ContextRule::EntityScript,
            ContextRule::Script,
            ContextRule::Shared,
        ] }
    }
}

impl ContextPolicy {
    /// Return which rule is used for context_key.
    pub fn which_rule(&self, context_key: &ContextKey) -> Option<&ContextRule> {
        self.priorities.iter().find(|rule| rule.select(context_key).is_some())
    }
    /// Use a shared script context.
    pub fn shared() -> Self {
        ContextPolicy { priorities: vec![ContextRule::Shared] }
    }
    /// If a domain is given, use that first.
    pub fn with_domains(mut self) -> Self {
        if ! self.priorities.contains(&ContextRule::Domain) {
            self.priorities.insert(0, ContextRule::Domain);
        }
        self
    }
    /// Domain contexts or a shared context.
    pub fn domains() -> Self {
        ContextPolicy { priorities: vec![ContextRule::Domain, ContextRule::Shared] }
    }
    /// Use one script context per entity or a shared context.
    pub fn per_entity() -> Self {
        ContextPolicy { priorities: vec![ContextRule::Entity, ContextRule::Shared] }
    }
    /// Use one script context per entity or a shared context.
    pub fn per_script() -> Self {
        ContextPolicy { priorities: vec![ContextRule::Script, ContextRule::Shared] }
    }
    /// Use one script context per entity-script, or a script context, or a shared context.
    pub fn per_entity_and_script() -> Self {
        ContextPolicy { priorities: vec![ContextRule::EntityScript, ContextRule::Script, ContextRule::Shared] }
    }
}

impl ContextKeySelector for ContextPolicy {
    fn select(&self, context_key: &ContextKey) -> Option<ContextKey> {
        self.priorities.iter().find_map(|priority| priority.select(context_key))
    }
}

#[derive(Resource)]
/// Keeps track of script contexts and enforces the context selection policy.
pub struct ScriptContext<P: IntoScriptPluginParams> {
    map: HashMap<ContextKey, Arc<Mutex<P::C>>>,
    /// The policy used to determine the context key.
    pub policy: ContextPolicy,
}

impl<P: IntoScriptPluginParams> ScriptContext<P> {
    /// Construct a new ScriptContext with the given policy.
    pub fn new(policy: ContextPolicy) -> Self {
        Self {
            map: HashMap::default(),
            policy,
        }
    }

    /// Get the context.
    pub fn get(&self, context_key: &ContextKey) -> Option<&Arc<Mutex<P::C>>> {
        self.policy.select(context_key).and_then(|key| self.map.get(&key))
    }
    /// Insert a context.
    ///
    /// If the context cannot be inserted, it is returned as an `Err`.
    pub fn insert(&mut self, context_key: &ContextKey, context: P::C) -> Result<(), P::C> {
        match self.policy.select(context_key) {
            Some(key) => {
                self.map.insert(key.into_weak(), Arc::new(Mutex::new(context)));
                Ok(())
            }
            None => Err(context)
        }
    }
    /// Returns true if there is a context.
    pub fn contains(&self, context_key: &ContextKey) -> bool {
        self.policy.select(context_key).map(|key| self.map.contains_key(&key)).unwrap_or(false)
    }
    /// Hash for context.
    ///
    /// Useful for tracking what context will be returned by `get()` without
    /// requiring that `P::C` impl `Hash` and cheaper too.
    ///
    /// Note: The existence of the hash does not imply the context exists. It
    /// only declares what its hash will be.
    pub fn hash(&self, context_key: &ContextKey) -> Option<u64> {
        self.policy.select(context_key).map(|key| DefaultHashBuilder::default().hash_one(&key))
    }
    /// Iterate through contexts.
    pub fn values(&self) -> impl Iterator<Item = &Arc<Mutex<P::C>>> {
        self.map.values()
    }
    /// Remove a context.
    ///
    /// Returns context if removed.
    pub fn remove(&mut self, context_key: &ContextKey) -> Option<Arc<Mutex<P::C>>> {
        self.policy.select(context_key).and_then(|key| self.map.remove(&key))
    }

    /// Iterate through keys and contexts.
    pub fn iter(&self) -> impl Iterator<Item = (&ContextKey, &Arc<Mutex<P::C>>)> {
        self.map.iter()
    }
}


/// Use one script context per entity and script with domains by default; see
/// [ScriptContext::per_entity_and_script].
impl<P: IntoScriptPluginParams> Default for ScriptContext<P> {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
            policy: ContextPolicy::default(),
        }
    }
}
