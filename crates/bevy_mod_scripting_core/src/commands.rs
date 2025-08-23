//! Commands for creating, updating and deleting scripts

use std::marker::PhantomData;

use crate::{
    IntoScriptPluginParams, ScriptContext,
    asset::ScriptAsset,
    bindings::{ScriptValue, WorldGuard},
    context::ScriptingLoader,
    error::{InteropError, ScriptError},
    event::{
        CallbackLabel, IntoCallbackLabel, OnScriptLoaded, OnScriptReloaded, OnScriptUnloaded,
        ScriptCallbackResponseEvent, ScriptEvent,
    },
    extractors::{HandlerContext, with_handler_system_state},
    handler::{handle_script_errors, send_callback_response},
    script::{DisplayProxy, ScriptAttachment, StaticScripts},
};
use ::{
    bevy_asset::{Assets, Handle},
    bevy_ecs::event::Events,
    bevy_log::{debug, warn},
};
use bevy_ecs::{system::Command, world::World};
use bevy_log::{error, info, trace};

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

        // first apply unload callback
        Command::apply(
            RunScriptCallback::<P>::new(
                self.context_key.clone(),
                OnScriptUnloaded::into_callback_label(),
                vec![],
                self.emit_responses,
            ),
            world,
        );
        match &self.context_key {
            ScriptAttachment::EntityScript(_, _) => {
                // nothing special needs to be done, just the context removal
            }
            ScriptAttachment::StaticScript(script) => {
                // remove the static script
                let mut scripts = world.get_resource_or_init::<StaticScripts>();
                if scripts.remove(script.id()) {
                    debug!("Deleted static script {}", script.display());
                } else {
                    warn!(
                        "Attempted to delete static script {}, but it was not found",
                        script.display()
                    );
                }
            }
        }

        let mut script_contexts = world.get_resource_or_init::<ScriptContext<P>>();
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
        handler_ctxt: &HandlerContext<P>,
    ) -> Result<(), ScriptError> {
        debug!("{}: reloading context {}", P::LANGUAGE, attachment);
        // reload context
        P::reload(
            attachment,
            content,
            context,
            &handler_ctxt.context_loading_settings.context_initializers,
            &handler_ctxt
                .context_loading_settings
                .context_pre_handling_initializers,
            guard.clone(),
            &handler_ctxt.runtime_container.runtime,
        )
    }

    fn load_context(
        attachment: &ScriptAttachment,
        content: &[u8],
        guard: WorldGuard,
        handler_ctxt: &HandlerContext<P>,
    ) -> Result<P::C, ScriptError> {
        debug!("{}: loading context {}", P::LANGUAGE, attachment);
        let context = P::load(
            attachment,
            content,
            &handler_ctxt.context_loading_settings.context_initializers,
            &handler_ctxt
                .context_loading_settings
                .context_pre_handling_initializers,
            guard.clone(),
            &handler_ctxt.runtime_container.runtime,
        )?;
        Ok(context)
    }

    fn before_reload(
        attachment: ScriptAttachment,
        world: WorldGuard,
        handler_ctxt: &HandlerContext<P>,
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
        .with_context(P::LANGUAGE)
        .with_context("saving reload state")
        .run_with_handler(world, handler_ctxt)
        .ok()
    }

    fn after_load(
        attachment: ScriptAttachment,
        world: WorldGuard,
        handler_ctxt: &HandlerContext<P>,
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
        .with_context(P::LANGUAGE)
        .with_context("on loaded callback")
        .run_with_handler(world.clone(), handler_ctxt);

        if is_reload {
            let _ = RunScriptCallback::<P>::new(
                attachment.clone(),
                OnScriptReloaded::into_callback_label(),
                vec![script_state.unwrap_or(ScriptValue::Unit)],
                emit_responses,
            )
            .with_context(P::LANGUAGE)
            .with_context("on reloaded callback")
            .run_with_handler(world, handler_ctxt);
        }
    }

    pub(crate) fn create_or_update_script(
        attachment: &ScriptAttachment,
        content: &[u8],
        guard: WorldGuard,
        handler_ctxt: &mut HandlerContext<P>,
        emit_responses: bool,
    ) -> Result<(), ScriptError> {
        // we demote to weak from here on out, so as not to hold the asset hostage
        let attachment = attachment.clone().into_weak();
        if let ScriptAttachment::StaticScript(id) = &attachment {
            // add to static scripts
            handler_ctxt.static_scripts.insert(id.clone());
        }

        let script_id = attachment.script();

        let phrase;
        let success;
        let mut script_state = None;
        // what callbacks we invoke depends whether or not this attachment
        // was already present in the context or not
        let is_reload = handler_ctxt.script_context.contains(&attachment);
        if is_reload {
            phrase = "reloading";
            success = "reloaded";
            script_state = Self::before_reload(
                attachment.clone(),
                guard.clone(),
                handler_ctxt,
                emit_responses,
            );
        } else {
            phrase = "loading";
            success = "loaded";
        };

        // whether or not we actually load vs reload the context (i.e. scrap the old one and create a new one)
        // depends on whether the context is already present in the script context
        let context = handler_ctxt.script_context.get(&attachment);
        let result_context_to_insert = match context {
            Some(context) => {
                let mut context = context.lock();

                Self::reload_context(
                    &attachment,
                    content,
                    &mut context,
                    guard.clone(),
                    handler_ctxt,
                )
                .map(|_| None)
            }
            None => Self::load_context(&attachment, content, guard.clone(), handler_ctxt).map(Some),
        };

        match result_context_to_insert {
            Ok(maybe_context) => {
                if let Some(context) = maybe_context
                    && handler_ctxt
                        .script_context
                        .insert(&attachment, context)
                        .is_err()
                {
                    warn!("Unable to insert script context for {}.", attachment);
                }

                // mark as resident in the context
                handler_ctxt
                    .script_context
                    .insert_resident(attachment.clone())
                    .map_err(|err| {
                        ScriptError::new(InteropError::invariant(format!(
                            "expected context to be present, could not mark attachment as resident in context, {err:?}"
                        )))
                    })?;

                debug!(
                    "{}: script {} successfully {}",
                    P::LANGUAGE,
                    attachment,
                    success,
                );

                Self::after_load(
                    attachment,
                    guard,
                    handler_ctxt,
                    script_state,
                    emit_responses,
                    is_reload,
                );

                Ok(())
            }
            Err(err) => {
                handle_script_errors(
                    guard,
                    vec![
                        err.clone()
                            .with_script(script_id.display())
                            .with_context(P::LANGUAGE)
                            .with_context(phrase),
                    ]
                    .into_iter(),
                );
                Err(err)
            }
        }
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

        with_handler_system_state(world, |guard, handler_ctxt: &mut HandlerContext<P>| {
            let _ = Self::create_or_update_script(
                &self.attachment,
                &content,
                guard.clone(),
                handler_ctxt,
                self.emit_responses,
            );
        });
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

    /// Equivalent to [`Self::run`], but usable in the case where you already have a [`HandlerContext`].
    pub fn run_with_handler(
        self,
        guard: WorldGuard,
        handler_ctxt: &HandlerContext<P>,
    ) -> Result<ScriptValue, ScriptError> {
        let result = handler_ctxt.call_dynamic_label(
            &self.callback,
            &self.attachment,
            None,
            self.args,
            guard.clone(),
        );

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
            let mut error_with_context = err
                .clone()
                .with_script(self.attachment.script().display())
                .with_context(P::LANGUAGE);
            for ctxt in self.context {
                error_with_context = error_with_context.with_context(ctxt);
            }

            handle_script_errors(guard, vec![error_with_context].into_iter());
        }
        result
    }

    /// Equivalent to running the command, but also returns the result of the callback.
    ///
    /// The returned error will already be handled and logged.
    pub fn run(self, world: &mut World) -> Result<ScriptValue, ScriptError> {
        with_handler_system_state(world, |guard, handler_ctxt: &mut HandlerContext<P>| {
            self.run_with_handler(guard, handler_ctxt)
        })
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
