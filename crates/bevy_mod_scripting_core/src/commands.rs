//! Commands for creating, updating and deleting scripts

use std::{marker::PhantomData, sync::Arc};

use crate::{
    IntoScriptPluginParams, ScriptContext,
    context::ScriptingLoader,
    error::ScriptError,
    event::{
        CallbackLabel, IntoCallbackLabel, OnScriptLoaded, OnScriptReloaded, OnScriptUnloaded,
        ScriptCallbackResponseEvent, ScriptEvent,
    },
    extractors::CallContext,
    handler::{send_callback_response, send_script_errors},
    script::{DisplayProxy, ScriptAttachment},
};
use bevy_ecs::{system::Command, world::World};
use bevy_log::{error, info, trace};
use bevy_mod_scripting_asset::ScriptAsset;
use bevy_mod_scripting_bindings::{InteropError, ScriptValue, WorldGuard};
use parking_lot::Mutex;
use {
    bevy_asset::{Assets, Handle},
    bevy_ecs::event::Events,
    bevy_log::debug,
};

/// Detaches a script, invoking the `on_script_unloaded` callback if it exists, and removes the script from the static scripts collection.
pub struct DeleteScript<P: IntoScriptPluginParams> {
    /// The context key
    pub context_key: ScriptAttachment,
    /// Whether to emit responses for core callbacks, like `on_script_loaded`, `on_script_reloaded`, etc.
    pub emit_responses: bool,
    /// hack to make this Send, C does not need to be Send since it is not stored in the command
    pub _ph: PhantomData<fn(P::C, P::R)>,
}

impl<P: IntoScriptPluginParams> DeleteScript<P> {
    /// Creates a new DeleteScript command with the given ID
    pub fn new(context_key: ScriptAttachment) -> Self {
        Self {
            context_key,
            emit_responses: false,
            _ph: PhantomData,
        }
    }

    /// If set to true, will emit responses for core callbacks, like `on_script_loaded`, `on_script_reloaded`, etc.
    pub fn with_responses(mut self, emit_responses: bool) -> Self {
        self.emit_responses = emit_responses;
        self
    }
}

impl<P: IntoScriptPluginParams> Command for DeleteScript<P> {
    fn apply(mut self, world: &mut World) {
        // we demote to weak from here on out, so as not to hold the asset hostage
        self.context_key = self.context_key.into_weak();
        let script_contexts = world.get_resource_or_init::<ScriptContext<P>>().clone();

        // first check the script exists, if it does not it could have been deleted by another command
        {
            let script_contexts = script_contexts.read();
            if !script_contexts.contains(&self.context_key) {
                debug!(
                    "{}: No context found for {}, not deleting.",
                    P::LANGUAGE,
                    self.context_key
                );
                return;
            }
        }

        // apply unload callback
        Command::apply(
            RunScriptCallback::<P>::new(
                self.context_key.clone(),
                OnScriptUnloaded::into_callback_label(),
                vec![],
                self.emit_responses,
            ),
            world,
        );

        let mut script_contexts = script_contexts.write();
        let residents_count = script_contexts.residents_len(&self.context_key);
        let delete_context = residents_count == 1;
        let script_id = self.context_key.script();
        if delete_context && script_contexts.remove(&self.context_key).is_some() {
            info!(
                "{}: Deleted context for script {:?}",
                P::LANGUAGE,
                script_id.display()
            );
        } else {
            info!(
                "{}: Context for script {:?} was not deleted, as it still has {} residents",
                P::LANGUAGE,
                script_id.display(),
                residents_count
            );
        }
        script_contexts.remove_resident(&self.context_key);
    }
}

/// Creates new script with the given ID, if a script with the given ID already exists, this is treated as an update
///
/// If script comes from an asset, expects it to be loaded, otherwise this command will fail to process the script.
pub struct CreateOrUpdateScript<P: IntoScriptPluginParams> {
    attachment: ScriptAttachment,
    // It feels like we're using a Box, which requires a clone merely to satisfy the Command trait.
    content: Option<Box<[u8]>>,
    // Hack to make this Send, C does not need to be Send since it is not stored in the command
    _ph: std::marker::PhantomData<fn(P::R, P::C)>,

    // if set to true will emit responses for core callbacks, like `on_script_loaded`, `on_script_reloaded`, etc.
    emit_responses: bool,
}

#[profiling::all_functions]
impl<P: IntoScriptPluginParams> CreateOrUpdateScript<P> {
    /// Creates a new CreateOrUpdateScript command with the given ID, content
    pub fn new(attachment: ScriptAttachment) -> Self {
        Self {
            attachment,
            content: None,
            _ph: std::marker::PhantomData,
            emit_responses: false,
        }
    }
    /// Add content to be evaluated.
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        let content = content.into();
        self.content = Some(content.into_bytes().into_boxed_slice());
        self
    }

    /// If set to true, will emit responses for core callbacks, like `on_script_loaded`, `on_script_reloaded`, etc.
    pub fn with_responses(mut self, emit_responses: bool) -> Self {
        self.emit_responses = emit_responses;
        self
    }

    fn reload_context(
        attachment: &ScriptAttachment,
        content: &[u8],
        context: &mut P::C,
        guard: WorldGuard,
    ) -> Result<(), InteropError> {
        debug!("{}: reloading context {}", P::LANGUAGE, attachment);
        // reload context
        P::reload(attachment, content, context, guard.clone())
    }

    fn load_context(
        attachment: &ScriptAttachment,
        content: &[u8],
        guard: WorldGuard,
    ) -> Result<P::C, InteropError> {
        debug!("{}: loading context {}", P::LANGUAGE, attachment);
        let context = P::load(attachment, content, guard.clone())?;
        Ok(context)
    }

    fn before_reload(
        attachment: ScriptAttachment,
        world: WorldGuard,
        ctxt: Arc<Mutex<P::C>>,
        emit_responses: bool,
    ) -> Option<ScriptValue> {
        // if something goes wrong, the error will be handled in the command
        // but we will not pass the script state to the after_load
        RunScriptCallback::<P>::new(
            attachment.clone(),
            OnScriptUnloaded::into_callback_label(),
            vec![],
            emit_responses,
        )
        .with_context("saving reload state")
        .run_with_context(world, ctxt)
        .ok()
    }

    fn after_load(
        attachment: ScriptAttachment,
        world: WorldGuard,
        script_context: Arc<Mutex<P::C>>,
        script_state: Option<ScriptValue>,
        emit_responses: bool,
        is_reload: bool,
    ) {
        let _ = RunScriptCallback::<P>::new(
            attachment.clone(),
            OnScriptLoaded::into_callback_label(),
            vec![],
            emit_responses,
        )
        .run_with_context(world.clone(), script_context.clone());

        if is_reload {
            let _ = RunScriptCallback::<P>::new(
                attachment.clone(),
                OnScriptReloaded::into_callback_label(),
                vec![script_state.unwrap_or(ScriptValue::Unit)],
                emit_responses,
            )
            .run_with_context(world, script_context);
        }
    }

    /// when a brand new script is created with no prior context
    pub(crate) fn create_new_script_and_context(
        attachment: &ScriptAttachment,
        content: &[u8],
        guard: WorldGuard,
        script_context: ScriptContext<P>,
        emit_responses: bool,
    ) -> Result<(), ScriptError> {
        let ctxt = Self::load_context(attachment, content, guard.clone())?;

        let ctxt = Arc::new(Mutex::new(ctxt));
        let mut script_context = script_context.write();

        script_context
            .insert_arc(attachment, ctxt.clone())
            .map_err(|_| {
                ScriptError::new_boxed_without_type_info(
                    String::from("No context policy applied").into(),
                )
                .with_context("creating new script and context")
            })?;

        Self::after_load(
            attachment.clone(),
            guard,
            ctxt,
            None, // no prior script state
            emit_responses,
            false, // first ever load
        );

        Ok(())
    }

    /// when a script is created in an existing context, e.g. when using shared contexts
    /// and we load a script into that context the first time (although not a reload from it's POV)
    pub(crate) fn create_script_in_existing_context(
        attachment: &ScriptAttachment,
        content: &[u8],
        guard: WorldGuard,
        context: Arc<Mutex<P::C>>,
        script_context: ScriptContext<P>,
        emit_responses: bool,
    ) -> Result<(), ScriptError> {
        let mut context_guard = context.lock();
        Self::reload_context(attachment, content, &mut context_guard, guard.clone())?;
        drop(context_guard);

        let mut script_context = script_context.write();

        script_context.insert_resident(attachment.clone()).map_err(|err| {
            ScriptError::new_boxed_without_type_info(format!(
                "expected context to be present, could not mark attachment as resident in context, {err:?}"
            ).into())
        })?;

        Self::after_load(
            attachment.clone(),
            guard,
            context.clone(),
            None, // no prior script state
            emit_responses,
            false, // first ever load
        );
        Ok(())
    }

    /// Reloads a script in an already existing context
    pub(crate) fn reload_script_in_context(
        attachment: &ScriptAttachment,
        content: &[u8],
        guard: WorldGuard,
        context: Arc<Mutex<P::C>>,
        emit_responses: bool,
    ) -> Result<(), ScriptError> {
        let script_state = Self::before_reload(
            attachment.clone(),
            guard.clone(),
            context.clone(),
            emit_responses,
        );

        let mut context_guard = context.lock();
        Self::reload_context(attachment, content, &mut context_guard, guard.clone())?;
        drop(context_guard);

        Self::after_load(
            attachment.clone(),
            guard,
            context.clone(),
            script_state,
            emit_responses,
            true, // this is definitely a reload
        );

        Ok(())
    }

    pub(crate) fn create_or_update_script(
        attachment: &ScriptAttachment,
        content: &[u8],
        guard: WorldGuard,
        script_context: ScriptContext<P>,
        emit_responses: bool,
    ) -> Result<(), ScriptError> {
        // determine if
        // - context exists
        // - script is RESIDENT in context

        let script_context_guard = script_context.read();
        let (context, is_resident) = script_context_guard.get_context_and_residency(attachment);
        drop(script_context_guard);

        let res = match (context, is_resident) {
            (None, _) => {
                // no context exists, create new context and script
                Self::create_new_script_and_context(
                    attachment,
                    content,
                    guard,
                    script_context,
                    emit_responses,
                )
                .map_err(|err| {
                    err.with_context(format!("creating new context for script {attachment}"))
                })
            }
            (Some(context), false) => {
                // context exists, but script is not resident in it, add script to existing context
                Self::create_script_in_existing_context(
                    attachment,
                    content,
                    guard,
                    context,
                    script_context,
                    emit_responses,
                )
                .map_err(|err| {
                    err.with_context(format!("creating script {attachment} in existing context"))
                })
            }
            (Some(context), true) => {
                // context exists, and script is resident in it, reload script in existing context
                Self::reload_script_in_context(attachment, content, guard, context, emit_responses)
                    .map_err(|err| {
                        err.with_context(format!(
                            "reloading script {attachment} in existing context"
                        ))
                    })
            }
        };

        res.map_err(|err| {
            err.with_script(attachment.script().display())
                .with_language(P::LANGUAGE)
        })
    }
}

#[profiling::all_functions]
impl<P: IntoScriptPluginParams> Command for CreateOrUpdateScript<P> {
    fn apply(self, world: &mut World) {
        let content = match self.content {
            Some(content) => content,
            None => match world
                .get_resource::<Assets<ScriptAsset>>()
                .and_then(|assets| assets.get(&self.attachment.script()))
                .map(|a| a.content.clone())
            {
                Some(content) => content,
                None => {
                    error!(
                        "{}: No content provided for script attachment {}. Cannot attach script.",
                        P::LANGUAGE,
                        self.attachment.script().display()
                    );
                    return;
                }
            },
        };
        let script_context = world.get_resource_or_init::<ScriptContext<P>>().clone();
        let guard = WorldGuard::new_exclusive(world);
        let res = Self::create_or_update_script(
            &self.attachment,
            &content,
            guard,
            script_context,
            self.emit_responses,
        );

        if let Err(err) = res {
            send_script_errors(WorldGuard::new_exclusive(world), [&err]);
        }
    }
}

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
    ) -> Result<ScriptValue, ScriptError> {
        let mut ctxt_guard = ctxt.lock();
        let result = ctxt_guard.call_context_dynamic(
            &self.attachment,
            &self.callback,
            self.args,
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

        if let Err(ref err) = result {
            send_script_errors(guard, [err]);
        }
        result
    }

    /// Equivalent to [`Self::run`], but usable in the case where you already have a [`HandlerContext`].
    pub fn run_with_contexts(
        self,
        guard: WorldGuard,
        script_contexts: ScriptContext<P>,
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

        self.run_with_context(guard, ctxt.clone())
    }

    /// Equivalent to running the command, but also returns the result of the callback.
    ///
    /// The returned error will already be handled and logged.
    pub fn run(self, world: &mut World) -> Result<ScriptValue, ScriptError> {
        let script_contexts = world.get_resource_or_init::<ScriptContext<P>>().clone();
        let guard = WorldGuard::new_exclusive(world);
        self.run_with_contexts(guard, script_contexts)
    }
}

impl<P: IntoScriptPluginParams> Command for RunScriptCallback<P> {
    fn apply(self, world: &mut World) {
        // Internals handle this.
        let _ = self.run(world);
    }
}

/// Attaches a static script, by initializing the appropriate script event, which is handled by the BMS systems.
pub struct AddStaticScript {
    /// The ID of the script to add
    pub(crate) id: Handle<ScriptAsset>,
}

impl AddStaticScript {
    /// Creates a new AddStaticScript command with the given ID
    pub fn new(id: impl Into<Handle<ScriptAsset>>) -> Self {
        Self { id: id.into() }
    }

    /// Runs the command emitting the appropriate script event
    pub fn run(self, events: &mut Events<ScriptEvent>) {
        events.send(ScriptEvent::Attached {
            key: ScriptAttachment::StaticScript(self.id.clone()),
        });
    }
}

impl Command for AddStaticScript {
    fn apply(self, world: &mut World) {
        let mut events = world.get_resource_or_init::<Events<ScriptEvent>>();
        self.run(&mut events);
    }
}

/// Detaches a static script, by initializing the appropriate script event, which is handled by the BMS systems.
pub struct RemoveStaticScript {
    /// The ID of the script to remove
    id: Handle<ScriptAsset>,
}

impl RemoveStaticScript {
    /// Creates a new RemoveStaticScript command with the given ID
    pub fn new(id: Handle<ScriptAsset>) -> Self {
        Self { id }
    }

    /// Runs the command emitting the appropriate script event
    pub fn run(self, events: &mut Events<ScriptEvent>) {
        events.send(ScriptEvent::Detached {
            key: ScriptAttachment::StaticScript(self.id.clone()),
        });
    }
}

#[profiling::all_functions]
impl Command for RemoveStaticScript {
    fn apply(self, world: &mut World) {
        let mut events = world.get_resource_or_init::<Events<ScriptEvent>>();
        self.run(&mut events);
    }
}
