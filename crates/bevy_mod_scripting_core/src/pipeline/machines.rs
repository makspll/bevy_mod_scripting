use bevy_log::info;

use crate::script::DisplayProxy;

use super::*;

#[derive(Event)]
/// Wrapper around a script event making it available to read by a specific plugin only
pub struct ForPlugin<T, P: IntoScriptPluginParams>(T, PhantomData<fn(P)>);

impl<T: std::fmt::Debug, P: IntoScriptPluginParams> std::fmt::Debug for ForPlugin<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ForPlugin").field(&self.0).finish()
    }
}

impl<T, P: IntoScriptPluginParams> From<T> for ForPlugin<T, P> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Clone, P: IntoScriptPluginParams> Clone for ForPlugin<T, P> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}

impl<T, P: IntoScriptPluginParams> ForPlugin<T, P> {
    /// Creates a new wrapper for the specific plugin
    pub fn new(event: T) -> Self {
        Self(event, Default::default())
    }

    /// Retrieves the inner event
    pub fn event(&self) -> &T {
        &self.0
    }

    /// Unpacks the inner event
    pub fn inner(self) -> T {
        self.0
    }
}

/// A reader that removes events immediately when read
/// Also requires they are wrapped in a [`ForPlugin`] event wrapper.
///
/// Allows re-publishing of the same events too
#[derive(SystemParam)]
pub struct StateMachine<'w, 's, T: Send + Sync + 'static, P: IntoScriptPluginParams> {
    events: ResMut<'w, Events<ForPlugin<T, P>>>,
    cursor: Local<'s, EventCursor<ForPlugin<T, P>>>,
}

impl<'w, 's, T: Send + Sync + 'static, P: IntoScriptPluginParams> StateMachine<'w, 's, T, P> {
    /// Returns the current number of machines outstanding with this state
    pub fn machines_outstanding(&self) -> usize {
        self.events.len()
    }

    /// returns a draining iterator which will consume all the state machine events for this state.
    ///
    /// Be careful, if intercepting between machine states, make sure to re-send any drained events if you wish for them
    /// to keep being processed, alternatively if you wish to stop the processing of a state machine, simply remove and do not re-send the machine
    pub fn drain(&mut self) -> impl Iterator<Item = T> {
        self.events.drain().map(ForPlugin::inner)
    }

    /// Returns a mutable iterator over the state machines, useful if you don't want to modify the machines but not interrupt the flow.
    pub fn intercept(&mut self) -> impl Iterator<Item = &mut T> {
        *self.cursor = self.events.get_cursor();
        self.cursor.read_mut(&mut self.events).map(|p| &mut p.0)
    }

    /// Returns all of the state machines without removing them. Useful if you want to plug into a state machine transition
    /// but not interrupt its outcome
    pub fn iter_cloned(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut cursor = self.events.get_cursor();
        cursor
            .read(&self.events)
            .cloned()
            .map(ForPlugin::inner)
            .collect()
    }

    /// Consumes an iterator of state machines and writes them to the asset pipe
    pub fn write_batch(&mut self, batch: impl IntoIterator<Item = T>) {
        self.events
            .send_batch(batch.into_iter().map(ForPlugin::new));
    }
}

/// Describes a script in the process of being loaded
#[derive(Event, Debug)]
pub struct Machine<K, S> {
    /// The current state
    pub state: S,
    /// The script attachment being loaded or reloaded
    pub attachment: ScriptAttachment,

    /// a set of metadata various interceptors can use to pass data along the chain
    pub blackboard: SmallVec<[(&'static str, Arc<dyn Any + Send + Sync + 'static>); 1]>,

    _ph: PhantomData<K>,
}

impl<K, S: Clone> Clone for Machine<K, S> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            attachment: self.attachment.clone(),
            blackboard: self.blackboard.clone(),
            _ph: self._ph,
        }
    }
}

impl<K, S> Machine<K, S> {
    /// Initialize a new state machine
    pub fn new(state: S, attachment: ScriptAttachment) -> Self {
        Self {
            state,
            attachment,
            blackboard: Default::default(),
            _ph: PhantomData,
        }
    }

    /// Returns a machine with an item inserted into its blackboard
    pub fn with_blackboard_insert(
        mut self,
        key: &'static str,
        payload: impl Any + Send + Sync + 'static,
    ) -> Self {
        self.blackboard.push((key, Arc::new(payload)));
        self
    }

    /// Checks if the blackboard contains the given key
    pub fn has_blackboard_key(&self, key: &str) -> bool {
        self.blackboard.iter().any(|(k, _)| *k == key)
    }

    /// Retrieves the given blackboard key casting it to the given type, if it exists and is of the correct type
    pub fn get_blackboard_key<T: Any + Send + Sync + 'static>(&self, key: &str) -> Option<Arc<T>> {
        self.blackboard.iter().find_map(|(k, v)| {
            (*k == key)
                .then_some(v)
                .and_then(|v| v.clone().downcast::<T>().ok())
        })
    }
}

/// State machine marker type
#[derive(Clone, Copy, Debug)]
pub struct Loading;
/// State machine marker type
#[derive(Clone, Copy, Debug)]
pub struct Unloading;

/// A script loading state machine state, describes a script which has completed loading and has its context present within [`ScriptContext`]
#[derive(Clone, Copy)]
pub struct LoadingCompleted;

/// A script loading state machine state, describes a script which has completed unloading and its no longer attached
#[derive(Clone, Copy)]
pub struct UnloadingCompleted;

/// A script loading state machine state, describes the starting state of loading every script
#[derive(Clone)]
pub struct LoadingInitialized {
    /// The handle to source the script content and ID from
    pub source: StrongScriptHandle,
}

/// A script loading state machine state, describes the starting state of reloading every script
pub struct ReloadingInitialized<P: IntoScriptPluginParams> {
    /// The handle to source the script content and ID from
    pub source: StrongScriptHandle,
    /// The context which will be reloaded using the new content
    pub existing_context: Arc<Mutex<P::C>>,
}

/// A script unloading state machine state, describes the starting state of unloading every script.
#[derive(Clone)]
pub struct UnloadingInitialized<P: IntoScriptPluginParams> {
    /// The context that the attachment is being unloaded
    pub existing_context: Arc<Mutex<P::C>>,
}

impl<P: IntoScriptPluginParams> Clone for ReloadingInitialized<P> {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            existing_context: self.existing_context.clone(),
        }
    }
}

impl<P: IntoScriptPluginParams> std::fmt::Debug for ReloadingInitialized<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReloadingInitialized")
            .field("source", &self.source)
            .finish()
    }
}

/// A script loading state machine state, describes a script which has a context assigned
pub struct ContextAssigned<P: IntoScriptPluginParams> {
    /// The context assigned for the script, either pre-existing or new
    pub context: Arc<Mutex<P::C>>,

    /// True if this is a new context and not one that was reloaded
    pub is_new_context: bool,
}

impl<P: IntoScriptPluginParams> Clone for ContextAssigned<P> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            is_new_context: self.is_new_context,
        }
    }
}

/// A script unloading state machine state, describes the state in which an attachment is no longer resident in the context, but the context still persists as it
/// was not the last resident
pub struct ResidentRemoved<P: IntoScriptPluginParams> {
    /// The context this attachment was removed from
    pub removed_from_context: Arc<Mutex<P::C>>,
}

/// A script unloading state machine state, describes the state in which an attachment is no longer resident in the context,
/// and the context itself was removed
pub struct ContextRemoved<P: IntoScriptPluginParams> {
    /// The context which was removed
    pub removed_context: Arc<Mutex<P::C>>,
}

impl Machine<Loading, LoadingInitialized> {
    /// Initialize a script load state machine
    pub fn start_load(attachment: ScriptAttachment, source: StrongScriptHandle) -> Self {
        info!(
            "Loading Initialized - Script attachment '{attachment}' from asset: '{}'",
            source.handle().display()
        );
        Self::new(LoadingInitialized { source }, attachment)
    }

    /// Progress to the context assigned state by assigning a context to the script being initialized
    pub fn assign_new_context<P: IntoScriptPluginParams>(
        self,
        context: Arc<Mutex<P::C>>,
    ) -> Machine<Loading, ContextAssigned<P>> {
        info!(
            "Loading Context Assigned - Script attachment '{}' was assigned to a new context",
            &self.attachment
        );
        Machine::<Loading, ContextAssigned<P>> {
            state: ContextAssigned {
                context,
                is_new_context: true,
            },
            attachment: self.attachment,
            blackboard: self.blackboard,
            _ph: PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> Machine<Loading, ReloadingInitialized<P>> {
    /// Initialize a script reload state machine
    pub fn start_reload(
        attachment: ScriptAttachment,
        source: StrongScriptHandle,
        existing_context: Arc<Mutex<P::C>>,
    ) -> Self {
        info!(
            "Reloading Initialized - Script attachment '{attachment}' from '{}' against existing context",
            source.handle().display()
        );
        Self::new(
            ReloadingInitialized {
                source,
                existing_context,
            },
            attachment,
        )
    }

    /// Progress to the context assigned state by selecting the context used for reloading
    pub fn assign_reloaded_context(self) -> Machine<Loading, ContextAssigned<P>> {
        info!(
            "Reloading Context Reloaded - Script attachment '{}' had it's assigned context reloaded",
            &self.attachment
        );
        Machine::<Loading, ContextAssigned<P>> {
            state: ContextAssigned {
                context: self.state.existing_context,
                is_new_context: false,
            },
            attachment: self.attachment,
            blackboard: self.blackboard,
            _ph: PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> Machine<Unloading, UnloadingInitialized<P>> {
    /// Initialize a script unload state machine
    pub fn start_unload(attachment: ScriptAttachment, existing_context: Arc<Mutex<P::C>>) -> Self {
        info!("Unloading Initialized - Script attachment '{attachment}'",);
        Self::new(UnloadingInitialized { existing_context }, attachment)
    }

    /// Progress to the context removed state by stating the context marked for initializing was removed from contexts fully
    pub fn remove_context(self) -> Machine<Unloading, ContextRemoved<P>> {
        Machine {
            state: ContextRemoved {
                removed_context: self.state.existing_context,
            },
            attachment: self.attachment,
            blackboard: self.blackboard,
            _ph: PhantomData,
        }
    }
    /// Progress to the ResidentRemoved state by stating that the attachment was removed as resident but the context remains in contexts.
    pub fn remove_resident(self) -> Machine<Unloading, ResidentRemoved<P>> {
        Machine {
            state: ResidentRemoved {
                removed_from_context: self.state.existing_context,
            },
            attachment: self.attachment,
            blackboard: self.blackboard,
            _ph: PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> Machine<Unloading, ContextRemoved<P>> {
    /// Finishes this state machine
    pub fn complete_unloading(self) -> Machine<Unloading, UnloadingCompleted> {
        Machine {
            state: UnloadingCompleted,
            attachment: self.attachment,
            blackboard: self.blackboard,
            _ph: PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> Machine<Unloading, ResidentRemoved<P>> {
    /// Finishes this state machine
    pub fn complete_unloading(self) -> Machine<Unloading, UnloadingCompleted> {
        Machine {
            state: UnloadingCompleted,
            attachment: self.attachment,
            blackboard: self.blackboard,
            _ph: PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> Machine<Loading, ContextAssigned<P>> {
    /// Completes the state machine
    pub fn complete_loading(self) -> Machine<Loading, LoadingCompleted> {
        info!(
            "{} completed - Script attachent '{}'",
            if self.state.is_new_context {
                "Loading"
            } else {
                "Reloading"
            },
            self.attachment,
        );
        Machine {
            state: LoadingCompleted,
            attachment: self.attachment,
            blackboard: self.blackboard,
            _ph: PhantomData,
        }
    }
}
