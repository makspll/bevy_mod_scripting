//! Commands for creating, updating and deleting scripts

use std::{sync::Arc, time::Duration};

use crate::{
    IntoScriptPluginParams, ScriptContexts,
    callbacks::ScriptCallbacks,
    error::ScriptError,
    event::{
        CallbackLabel, ForPlugin, ScriptAttachedEvent, ScriptCallbackResponseEvent,
        ScriptDetachedEvent,
    },
    handler::{ScriptingHandler, send_callback_response, send_script_errors},
    pipeline::RunProcessingPipelineOnce,
    script::Context,
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
    /// Optional handler run after the callback is finished with the response value
    /// Only applies when used as a command
    pub post_callback:
        fn(&mut World, attachment: ScriptAttachment, response: &Result<ScriptValue, ScriptError>),
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
            post_callback: |_, _, _| {},
        }
    }

    /// Sets the handler to be called after the callback is finished running.
    /// It will be triggered at the very end of the handling process, including after triggering the error event.
    /// Only applies when used as a command, or via the [`Self::run`] method.
    pub fn with_post_callback_handler(
        mut self,
        handler: fn(
            &mut World,
            attachment: ScriptAttachment,
            response: &Result<ScriptValue, ScriptError>,
        ),
    ) -> Self {
        self.post_callback = handler;
        self
    }

    /// Sets the context for the command, makes produced errors more useful.
    pub fn with_context(mut self, context: impl ToString) -> Self {
        self.context.push(context.to_string());
        self
    }

    fn handle_error(res: &Result<ScriptValue, ScriptError>, guard: WorldGuard) {
        if let Err(err) = res {
            send_script_errors(guard, [err]);
        }
    }

    /// Run the command on the given context.
    ///
    /// Assumes this context matches the attachment for the command.
    /// Does not send the error as a message, this needs to be done explicitly by the caller.
    ///
    /// calls [`std::mem::take`] on the arguments, to avoid cloning
    pub fn run_with_context(
        &mut self,
        guard: WorldGuard,
        ctxt: Arc<Mutex<P::C>>,
        script_callbacks: ScriptCallbacks<P>,
    ) -> Result<ScriptValue, ScriptError> {
        let mut ctxt_guard = ctxt.lock();
        let result = P::handle(
            std::mem::take(&mut self.args),
            &self.attachment,
            &self.callback,
            &mut ctxt_guard,
            script_callbacks,
            guard.clone(),
        );
        let result = result.map_err(|e| {
            let mut err = ScriptError::from(e).with_script(self.attachment.script().display());
            for ctxt in &self.context {
                err = err.with_context(ctxt.clone())
            }
            err.with_context(format!("in callback: {}", self.callback))
                .with_language(P::LANGUAGE)
        });

        drop(ctxt_guard);

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
                    self.callback.clone(),
                    self.attachment.clone(),
                    result.clone(),
                    P::LANGUAGE,
                ),
            );
        }

        result
    }

    /// Equivalent to [`Self::run`], but usable in the case where you already have [`ScriptContext`] and [`ScriptCallbacks`] resources available.
    ///
    /// Does not send the error as a message, this needs to be done explicitly by the caller.
    pub fn run_with_contexts(
        &mut self,
        guard: WorldGuard,
        script_contexts: ScriptContexts<P>,
        script_callbacks: ScriptCallbacks<P>,
    ) -> Result<ScriptValue, ScriptError> {
        let script_contexts = script_contexts.read();
        let ctxt = script_contexts.get_context(&self.attachment);
        let ctxt = match ctxt {
            Some(Context::LoadedAndActive(context)) => context,
            Some(s) => {
                return Err(ScriptError::new_boxed_without_type_info(
                    format!("Cannot run callback on script while in state of: {s}").into(),
                )
                .with_script(self.attachment.script().display())
                .with_language(P::LANGUAGE));
            }
            None => {
                return Err(ScriptError::new_boxed_without_type_info(
                    String::from("No context found for script").into(),
                )
                .with_script(self.attachment.script().display())
                .with_language(P::LANGUAGE));
            }
        };

        self.run_with_context(guard, ctxt.clone(), script_callbacks)
    }

    /// Equivalent to running the command, but also returns the result of the callback.
    ///
    /// The returned errors will NOT be sent as events or printed unless send errors is set to true
    pub fn run(mut self, world: &mut World, send_errors: bool) -> Result<ScriptValue, ScriptError> {
        let script_contexts = world.get_resource_or_init::<ScriptContexts<P>>().clone();
        let script_callbacks = world.get_resource_or_init::<ScriptCallbacks<P>>().clone();
        let guard = WorldGuard::new_exclusive(world);
        let res = self.run_with_contexts(guard.clone(), script_contexts, script_callbacks);
        if send_errors && res.is_err() {
            Self::handle_error(&res, guard);
        }

        // run hooks
        (self.post_callback)(world, self.attachment, &res);
        res
    }
}

impl<P: IntoScriptPluginParams> Command for RunScriptCallback<P> {
    fn apply(self, world: &mut World) {
        let _ = self.run(world, true);
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
        world.write_message(self.0);
        RunProcessingPipelineOnce::<P>::new(Some(Duration::from_secs(9999))).apply(world)
    }
}

impl<P: IntoScriptPluginParams> Command for DetachScript<P> {
    fn apply(self, world: &mut World) {
        world.write_message(self.0);
        RunProcessingPipelineOnce::<P>::new(Some(Duration::from_secs(9999))).apply(world)
    }
}
