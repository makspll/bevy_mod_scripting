use std::{collections::HashMap, sync::atomic::AtomicU32};

use bevy::ecs::{entity::Entity, system::Resource, world::World};

use crate::{
    prelude::{Runtime, ScriptError},
    script::{Script, ScriptId},
};

pub trait Context: 'static {}
impl<T: 'static> Context for T {}

pub type ContextId = u32;

#[derive(Resource)]
pub struct ScriptContexts<T: Context> {
    pub(crate) contexts: HashMap<ContextId, T>,
}

impl<T: Context> Default for ScriptContexts<T> {
    fn default() -> Self {
        Self {
            contexts: Default::default(),
        }
    }
}

static CONTEXT_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
impl<T: Context> ScriptContexts<T> {
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
        }
    }

    /// Allocates a new ContextId and inserts the context into the map
    pub fn insert(&mut self, ctxt: T) -> ContextId {
        let id = CONTEXT_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.contexts.insert(id, ctxt);
        id
    }

    /// Allocate new context id without inserting a context
    pub fn allocate_id(&self) -> ContextId {
        CONTEXT_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    pub fn remove(&mut self, id: ContextId) -> Option<T> {
        self.contexts.remove(&id)
    }
}

/// Initializer run once after creating a context but before executing it for the first time
pub type ContextInitializer<C> = fn(&ScriptId, &mut C) -> Result<(), ScriptError>;
/// Initializer run every time before executing or loading a script
pub type ContextPreHandlingInitializer<C> =
    fn(&ScriptId, Entity, &mut C) -> Result<(), ScriptError>;

#[derive(Resource)]
pub struct ContextLoadingSettings<C: Context, R: Runtime> {
    pub loader: Option<ContextBuilder<C, R>>,
    pub assigner: Option<ContextAssigner<C>>,
    pub context_initializers: Vec<ContextInitializer<C>>,
    pub context_pre_handling_initializers: Vec<ContextPreHandlingInitializer<C>>,
}

impl<C: Context, R: Runtime> Default for ContextLoadingSettings<C, R> {
    fn default() -> Self {
        Self {
            loader: None,
            assigner: None,
            context_initializers: Default::default(),
            context_pre_handling_initializers: Default::default(),
        }
    }
}

impl<C: Context, R: Runtime> Clone for ContextLoadingSettings<C, R> {
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
pub struct ContextBuilder<C: Context, R: Runtime> {
    pub load: fn(
        script: &ScriptId,
        content: &[u8],
        &[ContextInitializer<C>],
        &[ContextPreHandlingInitializer<C>],
        &mut World,
        runtime: &mut R,
    ) -> Result<C, ScriptError>,
    pub reload: fn(
        script: &ScriptId,
        new_content: &[u8],
        context: &mut C,
        &[ContextInitializer<C>],
        &[ContextPreHandlingInitializer<C>],
        &mut World,
        &mut R,
    ) -> Result<(), ScriptError>,
}

impl<C: Context, R: Runtime> Clone for ContextBuilder<C, R> {
    fn clone(&self) -> Self {
        Self {
            load: self.load,
            reload: self.reload,
        }
    }
}

/// A strategy for assigning contexts to new and existing but re-loaded scripts as well as for managing old contexts
pub struct ContextAssigner<C: Context> {
    /// Assign a context to the script, if script is `None`, this is a new script, otherwise it is an existing script with a context inside `contexts`.
    /// Returning None means the script should be assigned a new context
    pub assign: fn(
        old_script: Option<&Script>,
        script_id: &ScriptId,
        new_content: &[u8],
        contexts: &ScriptContexts<C>,
    ) -> Option<ContextId>,

    /// Handle the removal of the script, if any clean up in contexts is necessary perform it here.
    /// This will also be called, when a script is assigned a contextId on reload different from the previous one
    /// the context_id in that case will be the old context_id and the one stored in the script will be the old one
    pub remove: fn(context_id: ContextId, script: &Script, contexts: &mut ScriptContexts<C>),
}

impl<C: Context> Default for ContextAssigner<C> {
    fn default() -> Self {
        Self {
            assign: |_, _, _, c| Some(c.allocate_id()),
            remove: |id, _, c| _ = c.remove(id),
        }
    }
}

impl<C: Context> Clone for ContextAssigner<C> {
    fn clone(&self) -> Self {
        Self {
            assign: self.assign,
            remove: self.remove,
        }
    }
}
