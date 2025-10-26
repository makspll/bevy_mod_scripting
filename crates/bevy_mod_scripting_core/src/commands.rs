//! Commands for creating, updating and deleting scripts

use std::{sync::Arc, time::Duration};

use crate::{
    IntoScriptPluginParams, ScriptContext,
    callbacks::ScriptCallbacks,
    error::ScriptError,
    event::{
        CallbackLabel, ForPlugin, ScriptAttachedEvent, ScriptCallbackResponseEvent,
        ScriptDetachedEvent,
    },
    handler::{ScriptingHandler, send_callback_response, send_script_errors},
    pipeline::RunProcessingPipelineOnce,
};
use bevy_ecs::{system::Command, world::World};
use bevy_log::trace;
use bevy_mod_scripting_bindings::{ScriptValue, WorldGuard};
use bevy_mod_scripting_display::DisplayProxy;
use bevy_mod_scripting_script::ScriptAttachment;
use parking_lot::Mutex;

/// Runs a callback on the script with the given ID if it exists
pub struct RunScriptCallback<P: IntoScriptPluginParams> {
    /// The context key
    pub attachment: ScriptAttachment,
    /// The callback to run
    pub callback: CallbackLabel,
    /// optional context passed down to errors
    pub context: Vec<String>,
    /// The arguments to pass to the callback
    pub args: Vec<ScriptValue>,
    /// Whether the callback should emit a response event
    pub trigger_response: bool,
    /// Hack to make this Send, C does not need to be Send since it is not stored in the command
    pub _ph: std::marker::PhantomData<fn(P::R, P::C)>,
}

impl<P: IntoScriptPluginParams> RunScriptCallback<P> {
    /// Creates a new RunCallbackCommand with the given ID, callback and arguments
    pub fn new(
        attachment: ScriptAttachment,
        callback: CallbackLabel,
        args: Vec<ScriptValue>,
        trigger_response: bool,
    ) -> Self {
        Self {
            attachment,
            context: vec![],
            callback,
            args,
            trigger_response,
            _ph: std::marker::PhantomData,
        }
    }

    /// Sets the context for the command, makes produced errors more useful.
    pub fn with_context(mut self, context: impl ToString) -> Self {
        self.context.push(context.to_string());
        self
    }

    /// Run the command on the given context.
    ///
    /// Assumes this context matches the attachment for the command.
    pub fn run_with_context(
        self,
        guard: WorldGuard,
        ctxt: Arc<Mutex<P::C>>,
        script_callbacks: ScriptCallbacks<P>,
    ) -> Result<ScriptValue, ScriptError> {
        let mut ctxt_guard = ctxt.lock();
        let result = P::handle(
            self.args,
            &self.attachment,
            &self.callback,
            &mut ctxt_guard,
            script_callbacks,
            guard.clone(),
        );
        let result = result.map_err(|e| {
            let mut err = ScriptError::from(e).with_script(self.attachment.script().display());
            for ctxt in self.context {
                err = err.with_context(ctxt)
            }
            err.with_context(format!("in callback: {}", self.callback))
                .with_language(P::LANGUAGE)
        });

        if self.trigger_response {
            trace!(
                "{}: Sending callback response for callback: {}, attachment: {}",
                P::LANGUAGE,
                self.callback,
                self.attachment,
            );
            send_callback_response(
                guard.clone(),
                ScriptCallbackResponseEvent::new(
                    self.callback,
                    self.attachment.clone(),
                    result.clone(),
                    P::LANGUAGE,
                ),
            );
        }

        result
    }

    /// Equivalent to [`Self::run`], but usable in the case where you already have [`ScriptContext`] and [`ScriptCallbacks`] resources available.
    pub fn run_with_contexts(
        self,
        guard: WorldGuard,
        script_contexts: ScriptContext<P>,
        script_callbacks: ScriptCallbacks<P>,
    ) -> Result<ScriptValue, ScriptError> {
        let script_contexts = script_contexts.read();
        let ctxt = script_contexts.get_context(&self.attachment);
        let ctxt = match ctxt {
            Some(ctxt) => ctxt,
            None => {
                let err = ScriptError::new_boxed_without_type_info(
                    String::from("No context found for script").into(),
                )
                .with_script(self.attachment.script().display())
                .with_language(P::LANGUAGE);
                send_script_errors(guard, [&err]);
                return Err(err);
            }
        };

        self.run_with_context(guard, ctxt.clone(), script_callbacks)
    }

    /// Equivalent to running the command, but also returns the result of the callback.
    ///
    /// The returned errors will NOT be sent as events or printed
    pub fn run(self, world: &mut World) -> Result<ScriptValue, ScriptError> {
        let script_contexts = world.get_resource_or_init::<ScriptContext<P>>().clone();
        let script_callbacks = world.get_resource_or_init::<ScriptCallbacks<P>>().clone();
        let guard = WorldGuard::new_exclusive(world);
        self.run_with_contexts(guard, script_contexts, script_callbacks)
    }
}

impl<P: IntoScriptPluginParams> Command for RunScriptCallback<P> {
    fn apply(self, world: &mut World) {
        // Internals handle this.
        let _ = self.run(world);
    }
}

/// Command which emits a [`ScriptAttachedEvent`] and then runs the processing pipeline to immediately process it.
/// The end result is equivalent to attaching a script component or adding a static script and waiting for the normal pipeline to process it.
pub struct AttachScript<P: IntoScriptPluginParams>(pub ForPlugin<ScriptAttachedEvent, P>);

impl<P: IntoScriptPluginParams> AttachScript<P> {
    /// Creates a new [`AttachScript`] command, which will create the given attachment, run expected callbacks, and
    pub fn new(attachment: ScriptAttachment) -> Self {
        Self(ForPlugin::new(ScriptAttachedEvent(attachment)))
    }
}

/// Command which emits a [`ScriptDetachedEvent`] and then runs the processing pipeline to immediately process it.
/// The end result is equivalent to detaching a script component or removing a static script and waiting for the normal pipeline to process it.
pub struct DetachScript<P: IntoScriptPluginParams>(pub ForPlugin<ScriptDetachedEvent, P>);

impl<P: IntoScriptPluginParams> DetachScript<P> {
    /// Creates a new [`DetachScript`] command, which will create the given attachment, run all expected callbacks, and delete contexts if necessary.
    pub fn new(attachment: ScriptAttachment) -> Self {
        Self(ForPlugin::new(ScriptDetachedEvent(attachment)))
    }
}

impl<P: IntoScriptPluginParams> Command for AttachScript<P> {
    fn apply(self, world: &mut World) {
        world.send_event(self.0);
        RunProcessingPipelineOnce::<P>::new(Some(Duration::from_secs(9999))).apply(world)
    }
}

impl<P: IntoScriptPluginParams> Command for DetachScript<P> {
    fn apply(self, world: &mut World) {
        world.send_event(self.0);
        RunProcessingPipelineOnce::<P>::new(Some(Duration::from_secs(9999))).apply(world)
    }
}
