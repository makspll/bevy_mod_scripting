use std::{hash::Hash, sync::Arc};

use parking_lot::Mutex;

use super::*;
use crate::IntoScriptPluginParams;

/// Determines how contexts are grouped by manipulating the context key.
pub trait ContextKeySelector: Send + Sync + std::fmt::Debug + 'static {
    /// The given context key represents a possible script, entity that
    /// is requesting a context.
    ///
    /// This selector returns
    ///  - `None` when the given `context_key` is not relevant to its policy, or
    ///  - `Some(selected_key)` when the appropriate key has been determined.
    fn select(&self, context_key: &ScriptAttachment) -> Option<ContextKey>;
}

impl<F: Fn(&ScriptAttachment) -> Option<ContextKey> + Send + Sync + std::fmt::Debug + 'static>
    ContextKeySelector for F
{
    fn select(&self, context_key: &ScriptAttachment) -> Option<ContextKey> {
        (self)(context_key)
    }
}

/// A rule for context selection.
///
/// Maps a `ContextKey` to a `Option<ContextKey>`.
///
/// If the rule is not applicable, it returns `None`.
///
/// If the rule is applicable, it returns an equivalent or "susbset" `ContextKey` that represents the
/// context assignment
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextRule {
    /// If entity-script pair exists, return only that.
    EntityScript,
    /// If entity exists, return only that.
    Entity,
    /// If script exists, return only that.
    Script,
    /// Check nothing; return empty context key.
    Shared,
}

impl ContextKeySelector for ContextRule {
    /// Depending on the enum variant, executes that rule.
    fn select(&self, context_key: &ScriptAttachment) -> Option<ContextKey> {
        // extract the components from the input, i.e. entity, script, fill with None if not present
        let context_key: ContextKey = context_key.clone().into();

        match self {
            ContextRule::Entity => context_key.entity.map(|e| ContextKey {
                entity: Some(e),
                script: None,
            }),
            ContextRule::Script => context_key.script.map(|h| ContextKey {
                entity: None,
                script: Some(h),
            }),
            ContextRule::EntityScript => {
                context_key
                    .entity
                    .zip(context_key.script.clone())
                    .map(|(entity, script)| ContextKey {
                        entity: Some(entity),
                        script: Some(script),
                    })
            }
            ContextRule::Shared => Some(ContextKey::default()),
        }
    }
}

/// This is a configurable context policy based on priority.
#[derive(Debug)]
pub struct ContextPolicy {
    /// The rules in order of priority.
    pub priorities: Vec<Arc<dyn ContextKeySelector>>,
}

impl Clone for ContextPolicy {
    fn clone(&self) -> Self {
        Self {
            priorities: self.priorities.to_vec(),
        }
    }
}

/// Returns a default context policy. i.e. `[ContextPolicy::per_entity_and_script]`.
impl Default for ContextPolicy {
    fn default() -> Self {
        ContextPolicy::per_entity_and_script()
    }
}

impl ContextPolicy {
    /// Return which rule is used for context_key.
    pub fn which_rule(&self, context_key: &ScriptAttachment) -> Option<&dyn ContextKeySelector> {
        self.priorities
            .iter()
            .find_map(|rule| rule.select(context_key).is_some().then_some(rule.as_ref()))
    }

    /// Use a shared script context.
    pub fn shared() -> Self {
        ContextPolicy {
            priorities: vec![Arc::new(ContextRule::Shared)],
        }
    }

    /// Use one script context per entity or a shared context.
    ///
    /// For example, given:
    /// - `script_id: Some("script1")`
    /// - `entity: Some(1)`
    ///
    ///
    /// The context key will purely use the entity, resulting in a context key
    /// of `ContextKey { entity: Some(1) }`.
    ///
    /// resulting in each entity having its own context regardless of the script id.
    ///
    /// static scripts will get their own context per script asset.
    ///
    /// The default is then to use a shared context for no matches
    pub fn per_entity() -> Self {
        ContextPolicy {
            priorities: vec![
                Arc::new(ContextRule::Entity),
                Arc::new(ContextRule::Script),
                Arc::new(ContextRule::Shared),
            ],
        }
    }

    /// Use one script context per script or a shared context.
    ///
    /// For example, given:
    /// - `script_id: Some("script1")`
    /// - `entity: Some(1)`
    ///
    /// The context key will purely use the script, resulting in a context key
    /// of `ContextKey { script: Some("script1") }`.
    ///
    /// resulting in each script having its own context regardless of the entity.
    ///
    /// If no script is given it will be the default, i.e. shared context.
    pub fn per_script() -> Self {
        ContextPolicy {
            priorities: vec![Arc::new(ContextRule::Script), Arc::new(ContextRule::Shared)],
        }
    }

    /// Use one script context per entity-script, or a script context, or a shared context.
    ///
    /// For example, given:
    /// - `script_id: Some("script1")`
    /// - `entity: Some(1)`
    ///
    /// The context key will use the entity-script pair, resulting in a context key
    /// of `ContextKey { entity: Some(1), script: Some("script1") }`.
    ///
    /// resulting in each entity-script combination having its own context.
    ///
    /// If no entity-script pair is given it will be the default, i.e. shared context.
    pub fn per_entity_and_script() -> Self {
        ContextPolicy {
            priorities: vec![
                Arc::new(ContextRule::EntityScript),
                Arc::new(ContextRule::Script),
                Arc::new(ContextRule::Shared),
            ],
        }
    }
}

impl ContextKeySelector for ContextPolicy {
    fn select(&self, context_key: &ScriptAttachment) -> Option<ContextKey> {
        self.priorities
            .iter()
            .find_map(|priority| priority.select(context_key))
    }
}

struct ContextEntry<P: IntoScriptPluginParams> {
    context: Arc<Mutex<P::C>>,
    residents: HashSet<ScriptAttachment>,
}

#[derive(Resource)]
/// Keeps track of script contexts and enforces the context selection policy.
pub struct ScriptContext<P: IntoScriptPluginParams> {
    /// script contexts and the counts of how many scripts are associated with them.
    map: HashMap<ContextKey, ContextEntry<P>>,
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

    fn get_entry(&self, context_key: &ScriptAttachment) -> Option<&ContextEntry<P>> {
        self.policy
            .select(context_key)
            .and_then(|key| self.map.get(&key))
    }

    fn get_entry_mut(&mut self, context_key: &ScriptAttachment) -> Option<&mut ContextEntry<P>> {
        self.policy
            .select(context_key)
            .and_then(|key| self.map.get_mut(&key))
    }

    /// Get the context.
    pub fn get(&self, context_key: &ScriptAttachment) -> Option<Arc<Mutex<P::C>>> {
        self.get_entry(context_key)
            .map(|entry| entry.context.clone())
    }

    /// Insert a context.
    ///
    /// If the context cannot be inserted, it is returned as an `Err`.
    ///
    /// The attachment is also inserted as resident into the context.
    pub fn insert(&mut self, context_key: &ScriptAttachment, context: P::C) -> Result<(), P::C> {
        match self.policy.select(context_key) {
            Some(key) => {
                let entry = ContextEntry {
                    context: Arc::new(Mutex::new(context)),
                    residents: HashSet::from_iter([context_key.clone()]), // context with a residency of one
                };
                self.map.insert(key.into_weak(), entry);
                Ok(())
            }
            None => Err(context),
        }
    }

    /// Mark a context as resident.
    /// This needs to be called when a script is added to a context.
    ///
    /// Returns true if the context was inserted as a resident, false if it was already present.
    /// Errors if no matching context is found for the given attachment.
    pub fn insert_resident(
        &mut self,
        context_key: ScriptAttachment,
    ) -> Result<bool, ScriptAttachment> {
        if let Some(entry) = self.get_entry_mut(&context_key) {
            Ok(entry.residents.insert(context_key))
        } else {
            Err(context_key)
        }
    }

    /// Remove a resident context.
    /// This needs to be called when a script is deleted.
    pub fn remove_resident(&mut self, context_key: &ScriptAttachment) {
        if let Some(entry) = self.get_entry_mut(context_key) {
            entry.residents.remove(context_key);
        }
    }

    /// Iterates through all context & corresponding script attachment pairs.
    pub fn all_residents(
        &self,
    ) -> impl Iterator<Item = (ScriptAttachment, Arc<Mutex<P::C>>)> + use<'_, P> {
        self.map.values().flat_map(|entry| {
            entry
                .residents
                .iter()
                .map(move |resident| (resident.clone(), entry.context.clone()))
        })
    }

    /// Retrieves the first resident from each context.
    ///
    /// For example if using a single global context, and with 2 scripts:
    /// `script1` and `script2`
    /// this will return:
    /// `(&context_key, &script1)`
    pub fn first_resident_from_each_context(
        &self,
    ) -> impl Iterator<Item = (ScriptAttachment, Arc<Mutex<P::C>>)> + use<'_, P> {
        self.map.values().filter_map(|entry| {
            entry
                .residents
                .iter()
                .next()
                .map(|resident| (resident.clone(), entry.context.clone()))
        })
    }

    /// Iterates over the residents living in the same script context as the one mapped to by the context policy input
    pub fn residents(
        &self,
        context_key: &ScriptAttachment,
    ) -> impl Iterator<Item = (ScriptAttachment, Arc<Mutex<P::C>>)> + use<'_, P> {
        self.get_entry(context_key).into_iter().flat_map(|entry| {
            entry
                .residents
                .iter()
                .map(move |resident| (resident.clone(), entry.context.clone()))
        })
    }

    /// Returns the number of residents in the context shared by the given attachment.
    pub fn residents_len(&self, context_key: &ScriptAttachment) -> usize {
        self.get_entry(context_key)
            .map_or(0, |entry| entry.residents.len())
    }

    /// Returns true if a context contains this given attachment
    pub fn contains(&self, context_key: &ScriptAttachment) -> bool {
        self.get_entry(context_key)
            .is_some_and(|entry| entry.residents.contains(context_key))
    }

    /// Remove a context.
    ///
    /// Returns context if removed.
    pub fn remove(&mut self, context_key: &ScriptAttachment) -> Option<Arc<Mutex<P::C>>> {
        self.policy
            .select(context_key)
            .and_then(|key| self.map.remove(&key).map(|entry| entry.context))
    }
}

/// Use one script context per entity and script by default; see
/// [ScriptContext::per_entity_and_script].
impl<P: IntoScriptPluginParams> Default for ScriptContext<P> {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
            policy: ContextPolicy::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy_app::{App, Plugin};
    use bevy_asset::AssetIndex;
    use test_utils::make_test_plugin;

    use super::*;

    make_test_plugin!(crate);

    #[test]
    fn test_insertion_per_script_policy() {
        let policy = ContextPolicy::per_script();

        let mut script_context = ScriptContext::<TestPlugin>::new(policy.clone());
        let context_key = ScriptAttachment::EntityScript(
            Entity::from_raw(1),
            Handle::Weak(AssetIndex::from_bits(1).into()),
        );
        let context_key2 = ScriptAttachment::EntityScript(
            Entity::from_raw(2),
            Handle::Weak(AssetIndex::from_bits(1).into()),
        );
        assert_eq!(policy.select(&context_key), policy.select(&context_key2));

        script_context
            .insert(&context_key, TestContext::default())
            .unwrap();

        assert!(script_context.contains(&context_key));
        assert_eq!(script_context.residents_len(&context_key), 1);
        let resident = script_context.residents(&context_key).next().unwrap();
        assert_eq!(resident.0, context_key);
        assert!(script_context.get(&context_key).is_some());

        // insert another into the same context
        assert!(
            script_context
                .insert_resident(context_key2.clone())
                .unwrap()
        );

        assert!(script_context.contains(&context_key2));
        let mut residents = script_context.residents(&context_key2).collect::<Vec<_>>();
        residents.sort_by_key(|r| r.0.entity());
        assert_eq!(residents[0].0, context_key);
        assert_eq!(residents[1].0, context_key2);
        assert_eq!(residents.len(), 2);
        assert_eq!(script_context.residents_len(&context_key2), 2);
    }
}
