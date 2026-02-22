use std::{
    future::ready,
    pin::Pin,
    task::{Poll, Waker},
    time::{Duration, Instant},
};

use bevy_ecs::event::Event;
use bevy_log::trace;
use bevy_mod_scripting_bindings::{InteropError, ScriptValue};
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_platform::collections::HashMap;

use super::*;

#[derive(Default)]
/// Data used by the script pipeline, stored against each attachment as it's loading
pub struct MachineData {
    /// the state stored by a machine during unloads and reloads
    /// used to re-store state between reloads
    pub reload_state: ScriptValue,
}

#[derive(Resource, Default)]
/// Stores [`MachineData`] related to each script attachment, cleared between loads for each attachment
pub struct ActiveMachinesData(pub HashMap<ScriptAttachment, MachineData>);

/// A resource containing all currently running or ready to run machines.
#[derive(Resource)]
pub struct ActiveMachines<P: IntoScriptPluginParams> {
    active_machine: Option<ScriptMachine<P>>,
    initialized_machines: VecDeque<Box<dyn MachineState<P>>>,
    uninitialized_machines: VecDeque<(
        MachineContext,
        Box<dyn FnOnce(&mut World) -> Vec<Box<dyn MachineState<P>>> + Send + Sync + 'static>,
    )>,
    /// The current time budget per frame
    pub budget: Option<Duration>,
}

impl<P: IntoScriptPluginParams> Default for ActiveMachines<P> {
    fn default() -> Self {
        Self {
            active_machine: Default::default(),
            initialized_machines: Default::default(),
            uninitialized_machines: Default::default(),
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
        context: &mut MachineContext,
    ) -> Result<(), ScriptError>;

    /// type erase the listener
    fn erased<P: IntoScriptPluginParams>(
        self,
    ) -> Box<
        dyn Fn(&mut dyn MachineState<P>, &mut World, &mut MachineContext) -> Result<(), ScriptError>
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
        self.active_machine.as_ref()
    }

    /// Ticks all active machines until either:
    /// - The budget is exhausted
    /// - All the machines are finished
    ///
    /// If no budget is provided machines will be ticked ad infinitum or until they all complete.
    pub fn tick_machines(&mut self, world: &mut World) {
        let start = Instant::now();
        let end = start + self.budget.unwrap_or(Duration::from_secs(99999));

        let left = end - Instant::now();
        while (self.queued_machines() > 0 || self.active_machine.is_some())
            && left > Duration::default()
        {
            bevy_log::trace!("Ticking machines for up to {:?}", left);

            if self.active_machine.is_some() {
                let final_state = match &mut self.active_machine {
                    Some(next) => next.tick(world),
                    None => continue, // pick up the next machine, should be unreachable
                };

                match final_state {
                    Some(Ok(_)) => {
                        self.active_machine = None;
                    }
                    Some(Err(err)) => {
                        _ = world
                            .write_message(ScriptErrorEvent::new(err.with_language(P::LANGUAGE)));

                        if let Some(active_machine) = self.active_machine.as_mut() {
                            let failed_state =
                                ProcessInterrupted(active_machine.context.attachment.clone());
                            world.trigger(failed_state);
                        } // Else unreachable

                        self.active_machine = None;
                    }
                    None => {
                        // keeping it as active
                    }
                }
            } else {
                // initialize a machine and re-check
                if let Some((context, initializer)) = self.uninitialized_machines.pop_front() {
                    let machines = initializer(world);
                    self.initialized_machines.extend(machines);
                    if let Some(machine) = self.initialized_machines.pop_front() {
                        trace!(
                            "State machine '{}' queued. For script: {}",
                            machine.state_name(),
                            context.attachment,
                        );
                        self.active_machine = Some(ScriptMachine {
                            context,
                            internal_state: MachineExecutionState::Initialized(machine),
                        });
                    }
                }
            }
        }
    }

    /// Appends a machine to the end of the queue.
    pub fn queue_machine(
        &mut self,
        context: MachineContext,
        init: impl FnOnce(&mut World) -> Vec<Box<dyn MachineState<P>>> + Send + Sync + 'static,
    ) {
        self.uninitialized_machines
            .push_back((context, Box::new(init)));
    }

    /// Returns the amount of queued machines minus any currently processing ones
    pub fn queued_machines(&self) -> usize {
        self.uninitialized_machines.len() + self.initialized_machines.len()
    }

    /// Returns the amount of queued and processing machines
    pub fn processing_and_queued_machines(&self) -> usize {
        self.queued_machines() + self.current_machine().map(|_| 1).unwrap_or(0)
    }
}

/// A machine, which combines the inputs to its states with the state of the world and generates state transitions,
/// in an async manner (can span multiple frames)
pub struct ScriptMachine<P> {
    /// Context for the machine
    pub context: MachineContext,
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
    ) -> Option<Result<Box<dyn MachineState<P>>, ScriptError>> {
        match &mut self.internal_state {
            MachineExecutionState::Initialized(machine_state) => {
                trace!(
                    "State '{}' entered. For script: {}",
                    machine_state.state_name(),
                    self.context.attachment,
                );

                // trigger observers, modify state potentially
                machine_state.trigger_event(world);
                world.flush();

                let next = machine_state.poll_next(&self.context, world);
                self.internal_state = MachineExecutionState::Running(next.into());
                return self.tick(world);
            }
            MachineExecutionState::Running(future) => {
                let waker = Waker::noop();
                let mut cx = std::task::Context::from_waker(waker);

                if let Poll::Ready(res) = Future::poll(future.as_mut(), &mut cx) {
                    match res {
                        Ok(next) => {
                            if next.is_final() {
                                trace!(
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
                            trace!(
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
pub struct MachineContext {
    /// The script attachment being loaded or reloaded
    pub attachment: ScriptAttachment,
}

/// Describes a state in a finite state machine
pub trait MachineState<P: IntoScriptPluginParams>: Send + Sync + 'static + Any {
    /// A readable state name
    fn state_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Polls the machine for the next state.
    ///
    /// Machines are allowed to take multiple frames in generating it.
    fn poll_next(
        &mut self,
        ctxt: &MachineContext,
        world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync>;

    /// Final states designate that a state machine should complete processing. Returning true will cause the state to be recognize as a final state
    fn is_final(&self) -> bool {
        false
    }

    /// Triggers an event corresponding to this machine state, the reason we need to repeat this logic in every implementation is
    /// that [`Event`] is not `dyn` safe.
    ///
    /// Implementors should emit their own type.
    fn trigger_event(&mut self, world: &mut World);

    /// Build script error event with the most context possible
    fn build_script_error_event(
        &self,
        attachment: &ScriptAttachment,
        base_error: ScriptError,
    ) -> ScriptErrorEvent {
        ScriptErrorEvent::new(
            base_error
                .with_context(attachment.to_string())
                .with_context(self.state_name())
                .with_language(P::LANGUAGE),
        )
    }
}

#[derive(Clone, Event)]
/// A catchall state for error transitions. If any loading/reloading or unloading process is interrupted
/// unexpectedly, it reaches this state.
pub struct ProcessInterrupted(pub ScriptAttachment);

/// A script loading state machine state, describes a script which has completed loading or reloading and has its context present within [`ScriptContext`]
#[derive(Clone, Event)]
pub struct LoadingCompleted(pub ScriptAttachment);

/// A script loading state machine state, describes a script which has completed unloading and its no longer attached
#[derive(Clone, Event)]
pub struct UnloadingCompleted(pub ScriptAttachment);

/// A script loading state machine state, describes the starting state of loading every script
#[derive(Clone, Event)]
pub struct LoadingInitialized {
    /// The attachment being loaded
    pub attachment: ScriptAttachment,
    /// The handle to source the script content and ID from
    pub source: Handle<ScriptAsset>,
    /// The contents of the script asset, preloaded so we don't need more resources.
    pub content: Box<[u8]>,
}

/// A script loading state machine state, describes the starting state of reloading every script
#[derive(Event)]
pub struct ReloadingInitialized<P: IntoScriptPluginParams> {
    /// The attachment being reloaded
    pub attachment: ScriptAttachment,
    /// The handle to source the script content and ID from
    pub source: Handle<ScriptAsset>,
    /// The contents of the script asset, preloaded so we don't need more resources.
    pub content: Box<[u8]>,
    /// The context which will be reloaded using the new content
    pub existing_context: Arc<Mutex<P::C>>,
}

/// A script unloading state machine state, describes the starting state of unloading every script.
#[derive(Clone, Event)]
pub struct UnloadingInitialized<P: IntoScriptPluginParams> {
    /// The attachment being unloaded
    pub attachment: ScriptAttachment,
    /// The context that the attachment is being unloaded
    pub existing_context: Arc<Mutex<P::C>>,
}

impl<P: IntoScriptPluginParams> Clone for ReloadingInitialized<P> {
    fn clone(&self) -> Self {
        Self {
            attachment: self.attachment.clone(),
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
#[derive(Event)]
pub struct ContextAssigned<P: IntoScriptPluginParams> {
    /// The attachment which got the context assigned
    pub attachment: ScriptAttachment,

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
            attachment: self.attachment.clone(),
        }
    }
}

/// A script unloading state machine state, describes the state in which an attachment is no longer resident in the context, but the context still persists as it
/// was not the last resident
#[derive(Event)]
pub struct ResidentRemoved<P: IntoScriptPluginParams> {
    /// The attachment having a resident removed
    pub attachment: ScriptAttachment,

    /// The context this attachment was removed from
    pub removed_from_context: Arc<Mutex<P::C>>,
}

impl<P: IntoScriptPluginParams> Clone for ResidentRemoved<P> {
    fn clone(&self) -> Self {
        Self {
            removed_from_context: self.removed_from_context.clone(),
            attachment: self.attachment.clone(),
        }
    }
}

/// A script unloading state machine state, describes the state in which an attachment is no longer resident in the context,
/// and the context itself was removed
#[derive(Event)]
pub struct ContextRemoved<P: IntoScriptPluginParams> {
    /// The attachment having its context removed
    pub attachment: ScriptAttachment,

    /// The context which was removed
    pub removed_context: Arc<Mutex<P::C>>,
}

impl<P: IntoScriptPluginParams> Clone for ContextRemoved<P> {
    fn clone(&self) -> Self {
        Self {
            removed_context: self.removed_context.clone(),
            attachment: self.attachment.clone(),
        }
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for LoadingInitialized {
    fn poll_next(
        &mut self,
        ctxt: &MachineContext,
        world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        let attachment = &ctxt.attachment;
        let guard = WorldGuard::new_exclusive(world);
        let ctxt = P::load(attachment, &self.content, guard.clone());
        Box::new(ready(ctxt.map_err(ScriptError::from).map(|context| {
            Box::new(ContextAssigned::<P> {
                attachment: attachment.clone(),
                context: Arc::new(Mutex::new(context)),
                is_new_context: true,
            }) as Box<dyn MachineState<P>>
        })))
    }

    fn trigger_event(&mut self, world: &mut World) {
        world.trigger_ref(self);
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for ReloadingInitialized<P> {
    fn poll_next(
        &mut self,
        ctxt: &MachineContext,
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
                attachment: attachment.clone(),
                context: self.existing_context.clone(),
                is_new_context: false,
            }) as Box<dyn MachineState<P>>
        })))
    }

    fn trigger_event(&mut self, world: &mut World) {
        world.trigger_ref(self)
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for UnloadingInitialized<P> {
    fn poll_next(
        &mut self,
        ctxt: &MachineContext,
        world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        let attachment = &ctxt.attachment;
        let contexts = world.get_resource_or_init::<ScriptContexts<P>>();
        let mut contexts_guard = contexts.write();
        let residents_len = contexts_guard.residents_len(attachment);
        if residents_len == 1 {
            contexts_guard.remove(attachment);
            Box::new(ready(Ok(Box::new(ContextRemoved {
                attachment: attachment.clone(),
                removed_context: self.existing_context.clone(),
            }) as Box<dyn MachineState<P>>)))
        } else {
            contexts_guard.remove_resident(attachment);
            // TODO: handle failures here
            let _ = contexts_guard.mark_active_if_not_loading(attachment);
            Box::new(ready(Ok(Box::new(ResidentRemoved {
                attachment: attachment.clone(),
                removed_from_context: self.existing_context.clone(),
            }) as Box<dyn MachineState<P>>)))
        }
    }

    fn trigger_event(&mut self, world: &mut World) {
        world.trigger_ref(self)
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for ContextAssigned<P> {
    fn poll_next(
        &mut self,
        ctxt: &MachineContext,
        world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        let attachment = &ctxt.attachment;
        let contexts = world.get_resource_or_init::<ScriptContexts<P>>();
        let mut contexts_guard = contexts.write();

        // drop any strong handles
        match contexts_guard.insert(
            attachment.clone(),
            crate::script::Context::LoadedAndActive(self.context.clone()),
        ) {
            Ok(_) => {}
            Err(_) => {
                drop(contexts_guard);
                _ = world.write_message(ScriptErrorEvent::new(
                    ScriptError::from(InteropError::str("no context policy matched"))
                        .with_language(P::LANGUAGE),
                ))
            }
        }
        Box::new(ready(Ok(
            Box::new(LoadingCompleted(attachment.clone())) as Box<dyn MachineState<P>>
        )))
    }
    fn trigger_event(&mut self, world: &mut World) {
        world.trigger_ref(self)
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for LoadingCompleted {
    fn poll_next(
        &mut self,
        ctxt: &MachineContext,
        _world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        Box::new(ready(Ok(
            Box::new(LoadingCompleted(ctxt.attachment.clone())) as Box<dyn MachineState<P>>,
        )))
    }

    fn is_final(&self) -> bool {
        true
    }

    fn trigger_event(&mut self, world: &mut World) {
        world.trigger_ref(self)
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for ContextRemoved<P> {
    fn poll_next(
        &mut self,
        _ctxt: &MachineContext,
        _world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        Box::new(ready(Ok(
            Box::new(UnloadingCompleted(self.attachment.clone())) as Box<dyn MachineState<P>>,
        )))
    }

    fn trigger_event(&mut self, world: &mut World) {
        world.trigger_ref(self)
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for ResidentRemoved<P> {
    fn poll_next(
        &mut self,
        _ctxt: &MachineContext,
        _world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        Box::new(ready(Ok(
            Box::new(UnloadingCompleted(self.attachment.clone())) as Box<dyn MachineState<P>>,
        )))
    }

    fn trigger_event(&mut self, world: &mut World) {
        world.trigger_ref(self)
    }
}

impl<P: IntoScriptPluginParams> MachineState<P> for UnloadingCompleted {
    fn poll_next(
        &mut self,
        _ctxt: &MachineContext,
        _world: &mut World,
    ) -> Box<dyn Future<Output = Result<Box<dyn MachineState<P>>, ScriptError>> + Send + Sync> {
        Box::new(ready(
            Ok(Box::new(self.clone()) as Box<dyn MachineState<P>>),
        ))
    }

    fn is_final(&self) -> bool {
        true
    }

    fn trigger_event(&mut self, world: &mut World) {
        world.trigger_ref(self)
    }
}
