use crate::{
    prelude::ScriptError,
    script::{Script, ScriptId},
    IntoScriptPluginParams,
};
use bevy::ecs::{entity::Entity, system::Resource, world::World};
use std::{collections::HashMap, sync::atomic::AtomicU32};

pub trait Context: 'static {}
impl<T: 'static> Context for T {}

pub type ContextId = u32;

#[derive(Resource)]
pub struct ScriptContexts<P: IntoScriptPluginParams> {
    pub(crate) contexts: HashMap<ContextId, P::C>,
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
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
        }
    }

    /// Allocates a new ContextId and inserts the context into the map
    pub fn insert(&mut self, ctxt: P::C) -> ContextId {
        let id = CONTEXT_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.contexts.insert(id, ctxt);
        id
    }

    /// Allocate new context id without inserting a context
    pub fn allocate_id(&self) -> ContextId {
        CONTEXT_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    pub fn remove(&mut self, id: ContextId) -> Option<P::C> {
        self.contexts.remove(&id)
    }

    pub fn get(&self, id: ContextId) -> Option<&P::C> {
        self.contexts.get(&id)
    }

    pub fn get_mut(&mut self, id: ContextId) -> Option<&mut P::C> {
        self.contexts.get_mut(&id)
    }
}

/// Initializer run once after creating a context but before executing it for the first time
pub type ContextInitializer<P> =
    fn(&ScriptId, &mut <P as IntoScriptPluginParams>::C) -> Result<(), ScriptError>;
/// Initializer run every time before executing or loading a script
pub type ContextPreHandlingInitializer<P> =
    fn(&ScriptId, Entity, &mut <P as IntoScriptPluginParams>::C) -> Result<(), ScriptError>;

#[derive(Resource)]
pub struct ContextLoadingSettings<P: IntoScriptPluginParams> {
    pub loader: Option<ContextBuilder<P>>,
    pub assigner: Option<ContextAssigner<P>>,
    pub context_initializers: Vec<ContextInitializer<P>>,
    pub context_pre_handling_initializers: Vec<ContextPreHandlingInitializer<P>>,
}

impl<P: IntoScriptPluginParams> Default for ContextLoadingSettings<P> {
    fn default() -> Self {
        Self {
            loader: None,
            assigner: None,
            context_initializers: Default::default(),
            context_pre_handling_initializers: Default::default(),
        }
    }
}

impl<P: IntoScriptPluginParams> Clone for ContextLoadingSettings<P> {
    fn clone(&self) -> Self {
        Self {
            loader: self.loader.clone(),
            assigner: self.assigner.clone(),
            context_initializers: self.context_initializers.clone(),
            context_pre_handling_initializers: self.context_pre_handling_initializers.clone(),
        }
    }
}

/// A strategy for loading and reloading contexts
pub struct ContextBuilder<P: IntoScriptPluginParams> {
    pub load: fn(
        script: &ScriptId,
        content: &[u8],
        context_initializers: &[ContextInitializer<P>],
        pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
        world: &mut World,
        runtime: &mut P::R,
    ) -> Result<P::C, ScriptError>,
    pub reload: fn(
        script: &ScriptId,
        new_content: &[u8],
        context: &mut P::C,
        context_initializers: &[ContextInitializer<P>],
        pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
        world: &mut World,
        runtime: &mut P::R,
    ) -> Result<(), ScriptError>,
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
    /// Assign a context to the script, if script is `None`, this is a new script, otherwise it is an existing script with a context inside `contexts`.
    /// Returning None means the script should be assigned a new context
    pub assign: fn(
        old_script: Option<&Script>,
        script_id: &ScriptId,
        new_content: &[u8],
        contexts: &ScriptContexts<P>,
    ) -> Option<ContextId>,

    /// Handle the removal of the script, if any clean up in contexts is necessary perform it here.
    /// This will also be called, when a script is assigned a contextId on reload different from the previous one
    /// the context_id in that case will be the old context_id and the one stored in the script will be the old one
    pub remove: fn(context_id: ContextId, script: &Script, contexts: &mut ScriptContexts<P>),
}

impl<P: IntoScriptPluginParams> Default for ContextAssigner<P> {
    fn default() -> Self {
        Self {
            assign: |old, _, _, _| old.map(|s| s.context_id),
            remove: |id, _, c| _ = c.remove(id),
        }
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
