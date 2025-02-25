//! Traits and types for managing script contexts.

use crate::{
    bindings::{ThreadWorldContainer, WorldContainer, WorldGuard},
    error::{InteropError, ScriptError},
    script::{Script, ScriptId},
    IntoScriptPluginParams,
};
use bevy::ecs::{entity::Entity, system::Resource};
use std::{collections::HashMap, sync::atomic::AtomicU32};

/// A trait that all script contexts must implement.
pub trait Context: 'static + Send + Sync {}
impl<T: 'static + Send + Sync> Context for T {}

/// The type of a context id
pub type ContextId = u32;

/// Stores script state for a scripting plugin. Scripts are identified by their `ScriptId`, while contexts are identified by their `ContextId`.
#[derive(Resource)]
pub struct ScriptContexts<P: IntoScriptPluginParams> {
    /// The contexts of the scripts
    pub contexts: HashMap<ContextId, P::C>,
}

impl<P: IntoScriptPluginParams> Default for ScriptContexts<P> {
    fn default() -> Self {
        Self {
            contexts: Default::default(),
        }
    }
}

static CONTEXT_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
impl<P: IntoScriptPluginParams> ScriptContexts<P> {
    /// Allocates a new ContextId and inserts the context into the map
    pub fn insert(&mut self, ctxt: P::C) -> ContextId {
        let id = CONTEXT_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.contexts.insert(id, ctxt);
        id
    }

    /// Inserts a context with a specific id
    pub fn insert_with_id(&mut self, id: ContextId, ctxt: P::C) -> Option<P::C> {
        self.contexts.insert(id, ctxt)
    }

    /// Allocate new context id without inserting a context
    pub fn allocate_id(&self) -> ContextId {
        CONTEXT_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    /// Removes a context from the map
    pub fn remove(&mut self, id: ContextId) -> Option<P::C> {
        self.contexts.remove(&id)
    }

    /// Get a reference to a context
    pub fn get(&self, id: ContextId) -> Option<&P::C> {
        self.contexts.get(&id)
    }

    /// Get a mutable reference to a context
    pub fn get_mut(&mut self, id: ContextId) -> Option<&mut P::C> {
        self.contexts.get_mut(&id)
    }

    /// Check if a context exists
    pub fn contains(&self, id: ContextId) -> bool {
        self.contexts.contains_key(&id)
    }
}

/// Initializer run once after creating a context but before executing it for the first time as well as after re-loading the script
pub type ContextInitializer<P> =
    fn(&str, &mut <P as IntoScriptPluginParams>::C) -> Result<(), ScriptError>;

/// Initializer run every time before executing or loading/re-loading a script
pub type ContextPreHandlingInitializer<P> =
    fn(&str, Entity, &mut <P as IntoScriptPluginParams>::C) -> Result<(), ScriptError>;

/// Settings concerning the creation and assignment of script contexts as well as their initialization.
#[derive(Resource)]
pub struct ContextLoadingSettings<P: IntoScriptPluginParams> {
    /// Defines the strategy used to load and reload contexts
    pub loader: ContextBuilder<P>,
    /// Defines the strategy used to assign contexts to scripts
    pub assigner: ContextAssigner<P>,
    /// Initializers run once after creating a context but before executing it for the first time
    pub context_initializers: Vec<ContextInitializer<P>>,
    /// Initializers run every time before executing or loading a script
    pub context_pre_handling_initializers: Vec<ContextPreHandlingInitializer<P>>,
}

impl<P: IntoScriptPluginParams> Default for ContextLoadingSettings<P> {
    fn default() -> Self {
        Self {
            loader: ContextBuilder::default(),
            assigner: ContextAssigner::default(),
            context_initializers: Default::default(),
            context_pre_handling_initializers: Default::default(),
        }
    }
}

impl<T: IntoScriptPluginParams> Clone for ContextLoadingSettings<T> {
    fn clone(&self) -> Self {
        Self {
            loader: self.loader.clone(),
            assigner: self.assigner.clone(),
            context_initializers: self.context_initializers.clone(),
            context_pre_handling_initializers: self.context_pre_handling_initializers.clone(),
        }
    }
}
/// A strategy for loading contexts
pub type ContextLoadFn<P> = fn(
    script_id: &ScriptId,
    content: &[u8],
    context_initializers: &[ContextInitializer<P>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
    runtime: &mut <P as IntoScriptPluginParams>::R,
) -> Result<<P as IntoScriptPluginParams>::C, ScriptError>;

/// A strategy for reloading contexts
pub type ContextReloadFn<P> = fn(
    script_id: &ScriptId,
    content: &[u8],
    previous_context: &mut <P as IntoScriptPluginParams>::C,
    context_initializers: &[ContextInitializer<P>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
    runtime: &mut <P as IntoScriptPluginParams>::R,
) -> Result<(), ScriptError>;

/// A strategy for loading and reloading contexts
pub struct ContextBuilder<P: IntoScriptPluginParams> {
    /// The function to load a context
    pub load: ContextLoadFn<P>,
    /// The function to reload a context
    pub reload: ContextReloadFn<P>,
}

impl<P: IntoScriptPluginParams> Default for ContextBuilder<P> {
    fn default() -> Self {
        Self {
            load: |_, _, _, _, _| Err(InteropError::invariant("no context loader set").into()),
            reload: |_, _, _, _, _, _| {
                Err(InteropError::invariant("no context reloader set").into())
            },
        }
    }
}

impl<P: IntoScriptPluginParams> ContextBuilder<P> {
    /// load a context
    pub fn load(
        loader: ContextLoadFn<P>,
        script: &ScriptId,
        content: &[u8],
        context_initializers: &[ContextInitializer<P>],
        pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
        world: WorldGuard,
        runtime: &mut P::R,
    ) -> Result<P::C, ScriptError> {
        WorldGuard::with_existing_static_guard(world.clone(), |world| {
            ThreadWorldContainer.set_world(world)?;
            (loader)(
                script,
                content,
                context_initializers,
                pre_handling_initializers,
                runtime,
            )
        })
    }

    /// reload a context
    pub fn reload(
        reloader: ContextReloadFn<P>,
        script: &ScriptId,
        content: &[u8],
        previous_context: &mut P::C,
        context_initializers: &[ContextInitializer<P>],
        pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
        world: WorldGuard,
        runtime: &mut P::R,
    ) -> Result<(), ScriptError> {
        WorldGuard::with_existing_static_guard(world, |world| {
            ThreadWorldContainer.set_world(world)?;
            (reloader)(
                script,
                content,
                previous_context,
                context_initializers,
                pre_handling_initializers,
                runtime,
            )
        })
    }
}

impl<P: IntoScriptPluginParams> Clone for ContextBuilder<P> {
    fn clone(&self) -> Self {
        Self {
            load: self.load,
            reload: self.reload,
        }
    }
}

/// A strategy for assigning contexts to new and existing but re-loaded scripts as well as for managing old contexts
pub struct ContextAssigner<P: IntoScriptPluginParams> {
    /// Assign a context to the script.
    /// The assigner can either return `Some(id)` or `None`.
    /// Returning None will request the processor to assign a new context id to assign to this script.
    ///
    /// Regardless, whether a script gets a new context id or not, the processor will check if the given context exists.
    /// If it does not exist, it will create a new context and assign it to the script.
    /// If it does exist, it will NOT create a new context, but assign the existing one to the script, and re-load the context.
    ///
    /// This function is only called once for each script, when it is loaded for the first time.
    pub assign: fn(
        script_id: &ScriptId,
        new_content: &[u8],
        contexts: &ScriptContexts<P>,
    ) -> Option<ContextId>,

    /// Handle the removal of the script, if any clean up in contexts is necessary perform it here.
    ///
    /// If you do not clean up the context here, it will stay in the context map!
    pub remove: fn(context_id: ContextId, script: &Script, contexts: &mut ScriptContexts<P>),
}

impl<P: IntoScriptPluginParams> ContextAssigner<P> {
    /// Create an assigner which re-uses a single global context for all scripts, only use if you know what you're doing.
    /// Will not perform any clean up on removal.
    pub fn new_global_context_assigner() -> Self {
        Self {
            assign: |_, _, _| Some(0), // always use the same id in rotation
            remove: |_, _, _| {},      // do nothing
        }
    }

    /// Create an assigner which assigns a new context to each script. This is the default strategy.
    pub fn new_individual_context_assigner() -> Self {
        Self {
            assign: |_, _, _| None,
            remove: |id, _, c| _ = c.remove(id),
        }
    }
}

impl<P: IntoScriptPluginParams> Default for ContextAssigner<P> {
    fn default() -> Self {
        Self::new_individual_context_assigner()
    }
}

impl<P: IntoScriptPluginParams> Clone for ContextAssigner<P> {
    fn clone(&self) -> Self {
        Self {
            assign: self.assign,
            remove: self.remove,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::asset::Language;

    use super::*;

    struct DummyParams;
    impl IntoScriptPluginParams for DummyParams {
        type C = String;
        type R = ();

        const LANGUAGE: Language = Language::Lua;

        fn build_runtime() -> Self::R {}
    }

    #[test]
    fn test_script_contexts_insert_get() {
        let mut contexts: ScriptContexts<DummyParams> = ScriptContexts::default();
        let id = contexts.insert("context1".to_string());
        assert_eq!(contexts.contexts.get(&id), Some(&"context1".to_string()));
        assert_eq!(
            contexts.contexts.get_mut(&id),
            Some(&mut "context1".to_string())
        );
    }

    #[test]
    fn test_script_contexts_allocate_id() {
        let contexts: ScriptContexts<DummyParams> = ScriptContexts::default();
        let id = contexts.allocate_id();
        let next_id = contexts.allocate_id();
        assert_eq!(next_id, id + 1);
    }

    #[test]
    fn test_script_contexts_remove() {
        let mut contexts: ScriptContexts<DummyParams> = ScriptContexts::default();
        let id = contexts.insert("context1".to_string());
        let removed = contexts.remove(id);
        assert_eq!(removed, Some("context1".to_string()));
        assert!(!contexts.contexts.contains_key(&id));

        // assert next id is still incremented
        let next_id = contexts.allocate_id();
        assert_eq!(next_id, id + 1);
    }
}
