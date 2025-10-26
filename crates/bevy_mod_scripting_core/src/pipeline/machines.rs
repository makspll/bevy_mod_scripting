use std::{
    any::TypeId,
    future::ready,
    pin::Pin,
    task::{Poll, Waker},
    time::{Duration, Instant},
};

use bevy_log::debug;
use bevy_mod_scripting_bindings::InteropError;
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_platform::collections::HashMap;

use super::*;

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
        self.cursor
            .read_mut(&mut self.events)
            .map(|p| p.event_mut())
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

/// A resource containing all currently running or ready to run machines.
#[derive(Resource)]
pub struct ActiveMachines<P: IntoScriptPluginParams> {
    machines: VecDeque<ScriptMachine<P>>,
    on_state_listeners: HashMap<
        TypeId,
        Vec<
            Arc<
                dyn Fn(
                        &mut dyn MachineState<P>,
                        &mut World,
                        &mut Context,
                    ) -> Result<(), ScriptError>
                    + Send
                    + Sync,
            >,
        >,
    >,
    pub(crate) budget: Option<Duration>,
}

impl<P: IntoScriptPluginParams> Default for ActiveMachines<P> {
    fn default() -> Self {
        Self {
            machines: Default::default(),
            on_state_listeners: Default::default(),
            budget: Default::default(),
        }
    }
}

/// Trait describing subscribers to transition events
pub trait TransitionListener<State>: 'static + Send + Sync {
    /// The hook to call when entering the state being listened to
    fn on_enter(
        &self,
        state: &mut State,
        world: &mut World,
        context: &mut Context,
    ) -> Result<(), ScriptError>;

    /// type erase the listener
    fn erased<P: IntoScriptPluginParams>(
        self,
    ) -> Box<
        dyn Fn(&mut dyn MachineState<P>, &mut World, &mut Context) -> Result<(), ScriptError>
            + Send
            + Sync,
    >
    where
        Self: Sized,
        State: 'static,
    {
        Box::new(move |state, world, context| {
            let typed = (state as &mut dyn Any).downcast_mut::<State>();
            typed
                .ok_or(ScriptError::new_boxed_without_type_info(
                    format!(
                        "could not downcast script machine state to: '{}'. Could not execute transition listener",
                        std::any::type_name::<State>()
                    )
                    .into(),
                ))
                .and_then(|typed| self.on_enter(typed, world, context))
        })
    }
}

impl<P: IntoScriptPluginParams> ActiveMachines<P> {
    /// Returns the currently processing machine
    pub fn current_machine(&self) -> Option<&ScriptMachine<P>> {
        self.machines.front()
    }

    /// Adds a listener to the back of the listener list for the state
    pub fn push_listener<S: 'static>(&mut self, listener: impl TransitionListener<S> + 'static) {
        let erased = listener.erased::<P>();
        self.on_state_listeners
            .entry(std::any::TypeId::of::<S>())
            .or_default()
            .push(erased.into());
    }

    /// Ticks all active machines until either:
    /// - The budget is exhausted
    /// - All the machines are finished
    ///
    /// If no budget is provided machines will be ticked ad infinitum or until they all complete.
    pub fn tick_machines(&mut self, world: &mut World) {
        let start = Instant::now();
        let end = start + self.budget.unwrap_or(Duration::from_secs(99999));
        while !self.machines.is_empty() && Instant::now() < end {
            if let Some(mut next) = self.machines.pop_front() {
                let final_state = next.tick(world, &self.on_state_listeners);
                match final_state {
                    Some(Ok(_)) => {
                        // removed
                    }
                    Some(Err(err)) => {
                        _ = world.send_event(ScriptErrorEvent::new(err));
                        // removed
                    }
                    None => {
                        // re-insert for next tick
                        self.machines.push_front(next);
                    }
                }
            }
        }
    }

    /// Appends a machine to the end of the queue.
    pub fn queue_machine(&mut self, context: Context, state: impl MachineState<P>) {
        self.machines.push_back(ScriptMachine {
            context,
            internal_state: MachineExecutionState::Initialized(Box::new(state)),
        });
    }

    /// Returns the amount of active machines
    pub fn active_machines(&self) -> usize {
        self.machines.len()
    }
}

/// A machine, which combines the inputs to its states with the state of the world and generates state transitions,
/// in an async manner (can span multiple frames)
pub struct ScriptMachine<P> {
    /// Context for the machine
    pub context: Context,
    internal_state: MachineExecutionState<P>,
}

enum MachineExecutionState<P> {
    Initialized(Box<dyn MachineState<P>>),
    Running(
        Pin<Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync>>,
    ),
    Finished,
}

impl<P: IntoScriptPluginParams> ScriptMachine<P> {
    /// Ticks the machine until it reaches the Finished state.
    /// Further tick's do nothing.
    /// If the machine has not been started yet, it will be started and the underlying future ticked at least once.
    /// If the machine is finished it will return the final state or an error.
    /// Once the final state has been returned, ticking will return in errors
    pub fn tick(
        &mut self,
        world: &mut World,
        listeners: &HashMap<
            TypeId,
            Vec<
                Arc<
                    dyn Fn(
                            &mut dyn MachineState<P>,
                            &mut World,
                            &mut Context,
                        ) -> Result<(), ScriptError>
                        + Send
                        + Sync,
                >,
            >,
        >,
    ) -> Option<Result<Box<dyn MachineState<P>>, ScriptError>> {
        match &mut self.internal_state {
            MachineExecutionState::Initialized(machine_state) => {
                debug!(
                    "State '{}' entered. For script: {}",
                    machine_state.state_name(),
                    self.context.attachment,
                );

                if let Some(listeners) = listeners.get(&machine_state.as_ref().type_id()) {
                    for on_entered in listeners {
                        if let Err(err) =
                            (on_entered)(machine_state.as_mut(), world, &mut self.context)
                        {
                            _ = world.send_event(ScriptErrorEvent::new(
                                err.with_context(self.context.attachment.to_string())
                                    .with_context(machine_state.state_name())
                                    .with_language(P::LANGUAGE),
                            ))
                        }
                    }
                }
                let next = machine_state.poll_next(&self.context, world);
                self.internal_state = MachineExecutionState::Running(next.into());
                return self.tick(world, listeners);
            }
            MachineExecutionState::Running(future) => {
                let waker = Waker::noop();
                let mut cx = std::task::Context::from_waker(waker);

                if let Poll::Ready(res) = Future::poll(future.as_mut(), &mut cx) {
                    match res {
                        Ok(next) => {
                            if next.is_final() {
                                debug!(
                                    "Reached final state '{}'. For script {}",
                                    next.state_name(),
                                    &self.context.attachment
                                );
                                self.internal_state = MachineExecutionState::Finished;
                                return Some(Ok(next));
                            } else {
                                self.internal_state = MachineExecutionState::Initialized(next)
                            }
                        }
                        res => {
                            debug!(
                                "Error in progressing to next state. For script {}",
                                &self.context.attachment
                            );
                            self.internal_state = MachineExecutionState::Finished;
                            return Some(res);
                        }
                    }
                }
            }
            MachineExecutionState::Finished => {
                return Some(Err(ScriptError::new_boxed_without_type_info(
                    String::from("cannot tick machine twice").into(),
                )
                .with_context(self.context.attachment.to_string())
                .with_language(P::LANGUAGE)));
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
/// Each state machine is run in the context of a script, and can contain additional metadata.
/// The context struct contains all this additional metadata.
pub struct Context {
    /// The script attachment being loaded or reloaded
    pub attachment: ScriptAttachment,

    /// a set of metadata various interceptors can use to pass data along the chain
    pub blackboard: SmallVec<[(&'static str, Arc<dyn Any + Send + Sync + 'static>); 1]>,
}
impl Context {
    /// push a value onto the blackboard
    pub fn insert(&mut self, key: &'static str, val: impl Any + Send + Sync + 'static) {
        self.blackboard.push((key, Arc::new(val)));
    }

    /// tries to find a value and cast it to the given type
    pub fn get_first_typed<T: Any + Clone>(&self, key: &'static str) -> Option<T> {
        self.blackboard
            .iter()
            .find_map(|(k, v)| (*k == key).then_some(v.downcast_ref().cloned()))
            .flatten()
    }
}

/// Describes a state in a finite state machine
pub trait MachineState<P>: Send + Sync + 'static + Any {
    /// A readable state name
    fn state_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Polls the machine for the next state.
    ///
    /// Machines are allowed to take multiple frames in generating it.
    fn poll_next(
        &mut self,
        ctxt: &Context,
        world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync>;

    /// Final states designate that a state machine should complete processing. Returning true will cause the state to be recognize as a final state
    fn is_final(&self) -> bool {
        false
    }
}

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
    pub source: Handle<ScriptAsset>,
    /// The contents of the script asset, preloaded so we don't need more resources.
    pub content: Box<[u8]>,
}

/// A script loading state machine state, describes the starting state of reloading every script
pub struct ReloadingInitialized<P: IntoScriptPluginParams> {
    /// The handle to source the script content and ID from
    pub source: Handle<ScriptAsset>,
    /// The contents of the script asset, preloaded so we don't need more resources.
    pub content: Box<[u8]>,
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
            content: self.content.clone(),
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

impl<P: IntoScriptPluginParams> Clone for ResidentRemoved<P> {
    fn clone(&self) -> Self {
        Self {
            removed_from_context: self.removed_from_context.clone(),
        }
    }
}

/// A script unloading state machine state, describes the state in which an attachment is no longer resident in the context,
/// and the context itself was removed
pub struct ContextRemoved<P: IntoScriptPluginParams> {
    /// The context which was removed
    pub removed_context: Arc<Mutex<P::C>>,
}

impl<P: IntoScriptPluginParams> Clone for ContextRemoved<P> {
    fn clone(&self) -> Self {
        Self {
            removed_context: self.removed_context.clone(),
        }
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for LoadingInitialized {
    fn poll_next(
        &mut self,
        ctxt: &Context,
        world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        let attachment = &ctxt.attachment;
        let guard = WorldGuard::new_exclusive(world);
        let ctxt = P::load(attachment, &self.content, guard.clone());
        Box::new(ready(ctxt.map_err(ScriptError::from).map(|context| {
            Box::new(ContextAssigned::<P> {
                context: Arc::new(Mutex::new(context)),
                is_new_context: true,
            }) as Box<dyn MachineState<P>>
        })))
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for ReloadingInitialized<P> {
    fn poll_next(
        &mut self,
        ctxt: &Context,
        world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        let attachment = &ctxt.attachment;
        let guard = WorldGuard::new_exclusive(world);
        let mut previous_context_guard = self.existing_context.lock();
        let ctxt = P::reload(
            attachment,
            &self.content,
            &mut previous_context_guard,
            guard.clone(),
        );

        Box::new(ready(ctxt.map_err(ScriptError::from).map(|_| {
            Box::new(ContextAssigned::<P> {
                context: self.existing_context.clone(),
                is_new_context: false,
            }) as Box<dyn MachineState<P>>
        })))
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for UnloadingInitialized<P> {
    fn poll_next(
        &mut self,
        ctxt: &Context,
        world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        let attachment = &ctxt.attachment;
        let contexts = world.get_resource_or_init::<ScriptContext<P>>();
        let mut contexts_guard = contexts.write();
        let residents_len = contexts_guard.residents_len(attachment);
        if residents_len == 1 {
            contexts_guard.remove(attachment);
            Box::new(ready(Ok(Box::new(ContextRemoved {
                removed_context: self.existing_context.clone(),
            }) as Box<dyn MachineState<P>>)))
        } else {
            contexts_guard.remove_resident(attachment);
            Box::new(ready(Ok(Box::new(ResidentRemoved {
                removed_from_context: self.existing_context.clone(),
            }) as Box<dyn MachineState<P>>)))
        }
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for ContextAssigned<P> {
    fn poll_next(
        &mut self,
        ctxt: &Context,
        world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        let attachment = &ctxt.attachment;
        let contexts = world.get_resource_or_init::<ScriptContext<P>>();
        let mut contexts_guard = contexts.write();

        // drop any strong handles
        match contexts_guard.insert(attachment.clone().into_weak(), self.context.clone()) {
            Ok(_) => {}
            Err(_) => {
                drop(contexts_guard);
                _ = world.send_event(ScriptErrorEvent::new(ScriptError::from(InteropError::str(
                    "no context policy matched",
                ))))
            }
        }
        Box::new(ready(Ok(
            Box::new(LoadingCompleted) as Box<dyn MachineState<P>>
        )))
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for LoadingCompleted {
    fn poll_next(
        &mut self,
        _ctxt: &Context,
        _world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        Box::new(ready(Ok(Box::new(Self) as Box<dyn MachineState<P>>)))
    }

    fn is_final(&self) -> bool {
        true
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for ContextRemoved<P> {
    fn poll_next(
        &mut self,
        _ctxt: &Context,
        _world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        Box::new(ready(
            Ok(Box::new(self.clone()) as Box<dyn MachineState<P>>),
        ))
    }

    fn is_final(&self) -> bool {
        true
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for ResidentRemoved<P> {
    fn poll_next(
        &mut self,
        _ctxt: &Context,
        _world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        Box::new(ready(
            Ok(Box::new(self.clone()) as Box<dyn MachineState<P>>),
        ))
    }

    fn is_final(&self) -> bool {
        true
    }
}
