//! Commands for creating, updating and deleting scripts

use crate::{
    ScriptQueue,
    ScriptContext,
    asset::ScriptAsset,
    bindings::{ScriptValue, WorldGuard},
    context::ContextBuilder,
    error::{InteropError, ScriptError},
    event::{
        CallbackLabel, IntoCallbackLabel, OnScriptLoaded, OnScriptReloaded, OnScriptUnloaded,
        ScriptCallbackResponseEvent,
    },
    extractors::{with_handler_system_state, HandlerContext},
    handler::{handle_script_errors, send_callback_response},
    script::{StaticScripts, DisplayProxy, ScriptContextProvider, ContextKey},
    IntoScriptPluginParams,
};
use bevy::{asset::Handle, ecs::entity::Entity, log::{warn, debug}, prelude::{EntityCommand, Command}};
use std::marker::PhantomData;

/// Deletes a script with the given ID
pub struct DeleteScript<P: IntoScriptPluginParams> {
    /// The context key
    pub context_key: ContextKey,
    /// hack to make this Send, C does not need to be Send since it is not stored in the command
    pub _ph: PhantomData<fn(P::C, P::R)>,
}

impl<P: IntoScriptPluginParams> DeleteScript<P> {
    /// Creates a new DeleteScript command with the given ID
    pub fn new(context_key: impl Into<ContextKey>) -> Self {
        Self {
            context_key: context_key.into(),
            _ph: PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> Command for DeleteScript<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
        // first apply unload callback
        Command::apply(RunScriptCallback::<P>::new(
            self.context_key.clone(),
            OnScriptUnloaded::into_callback_label(),
            vec![],
            false,
        ), world);

        let mut deleted = false;
        if let Some(script_id) = self.context_key.script_id.as_ref() {
            {
                let mut scripts = world.get_resource_or_init::<StaticScripts>();
                if scripts.remove(script_id) {
                    debug!("Deleted static script {}", script_id.display());
                    deleted = true;
                }
            }
            {
                let mut script_contexts = world.get_resource_or_init::<ScriptContext<P>>();
                if script_contexts.remove(&self.context_key) {
                    bevy::log::info!("{}: Deleted context for script {:?}", P::LANGUAGE, script_id.display());
                    deleted = true;
                }
            }
        }
        if !deleted {
            bevy::log::error!(
                "Attempted to delete script context {} but it does not exist; doing nothing!",
                self.context_key
            );
        }
    }
}

impl<P: IntoScriptPluginParams> EntityCommand for DeleteScript<P> {
    fn apply(mut self, entity: Entity, world: &mut bevy::prelude::World) {
        self.context_key.entity = Some(entity);
        Command::apply(self, world)
    }
}

/// Creates new script with the given ID, if a script with the given ID already exists, this is treated as an update
///
/// If script comes from an asset, expects it to be loaded, otherwise this command will fail to process the script.
pub struct CreateOrUpdateScript<P: IntoScriptPluginParams> {
    context_key: ContextKey,
    // It feels like we're using a Box, which requires a clone merely to satisfy the Command trait.
    content: Option<Box<[u8]>>,
    // Hack to make this Send, C does not need to be Send since it is not stored in the command
    _ph: std::marker::PhantomData<fn(P::R, P::C)>,
}

#[profiling::all_functions]
impl<P: IntoScriptPluginParams> CreateOrUpdateScript<P> {
    /// Creates a new CreateOrUpdateScript command with the given ID, content
    pub fn new(context_key: impl Into<ContextKey>) -> Self {
        Self {
            context_key: context_key.into(),
            content: None,
            _ph: std::marker::PhantomData,
        }
    }
    /// Add content to be evaluated.
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        let content = content.into();
        self.content = Some(content.into_bytes().into_boxed_slice());
        self
    }

    fn reload_context(
        context_key: &ContextKey,
        content: &[u8],
        context: &mut P::C,
        guard: WorldGuard,
        handler_ctxt: &HandlerContext<P>,
    ) -> Result<(), ScriptError> {
// <<<<<<< HEAD
        // bevy::log::debug!("{}: reloading script with id: {}", P::LANGUAGE, self.id);
        // let existing_script = match handler_ctxt.scripts.scripts.get(&self.id) {
        //     Some(script) => script,
        //     None => {
        //         return Err(
        //             InteropError::invariant("Tried to reload script which doesn't exist").into(),
        //         )
        //     }
        // };

        // reload context
        (ContextBuilder::<P>::reload)(
            handler_ctxt.context_loading_settings.loader.reload,
            context_key,
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
        context_key: &ContextKey,
        content: &[u8],
        guard: WorldGuard,
        handler_ctxt: &HandlerContext<P>,
    ) -> Result<P::C, ScriptError> {
        bevy::log::debug!("{}: loading script into context {}", P::LANGUAGE, context_key);
        let context = (ContextBuilder::<P>::load)(
            handler_ctxt.context_loading_settings.loader.load,
            context_key,
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

    fn before_load(
        // context_key: &ContextKey,
        &self,
        world: WorldGuard,
        handler_ctxt: &mut HandlerContext<P>,
        is_reload: bool,
    ) -> Option<ScriptValue> {
        if is_reload {
            // if something goes wrong, the error will be handled in the command
            // but we will not pass the script state to the after_load
            return RunScriptCallback::<P>::new(
                self.context_key.clone(),
                OnScriptUnloaded::into_callback_label(),
                vec![],
                false,
            )
            .with_context(P::LANGUAGE)
            .with_context("saving reload state")
            .run_with_handler(world, handler_ctxt)
            .ok();
        }

        None
    }

    fn after_load(
        // context_key: &ContextKey,
        &self,
        world: WorldGuard,
        handler_ctxt: &mut HandlerContext<P>,
        script_state: Option<ScriptValue>,
        is_reload: bool,
    ) {
        let _ = RunScriptCallback::<P>::new(
            self.context_key.clone(),
            OnScriptLoaded::into_callback_label(),
            vec![],
            false,
        )
        .with_context(P::LANGUAGE)
        .with_context("on loaded callback")
        .run_with_handler(world.clone(), handler_ctxt);

        if is_reload {
            let state = script_state.unwrap_or(ScriptValue::Unit);
            let _ = RunScriptCallback::<P>::new(
                self.context_key.clone(),
                OnScriptReloaded::into_callback_label(),
                vec![state],
                false,
            )
            .with_context(P::LANGUAGE)
            .with_context("on reloaded callback")
            .run_with_handler(world, handler_ctxt);
        }
    }

// <<<<<<< HEAD
    // fn handle_global_context(
    //     &self,
    //     guard: WorldGuard,
    //     handler_ctxt: &mut HandlerContext<P>,
    // ) -> (Result<(), ScriptError>, Option<ScriptValue>, bool) {
    //     let existing_context = handler_ctxt
    //         .scripts
    //         .scripts
    //         .values()
    //         .next()
    //         .map(|s| s.context.clone());

    //     debug!(
    //         "{}: CreateOrUpdateScript command applying to global context (script_id: {}, new context?: {}, new script?: {})",
    //         P::LANGUAGE,
    //         self.id,
    //         existing_context.is_none(),
    //         !handler_ctxt.scripts.scripts.contains_key(&self.id)
    //     );

    //     let is_reload = existing_context.is_some();

    //     if let Some(context) = existing_context {
    //         // point all new scripts to the shared context
    //         handler_ctxt.scripts.scripts.insert(
    //             self.id.clone(),
    //             Script {
    //                 id: self.id.clone(),
    //                 asset: self.asset.clone(),
    //                 context,
    //             },
    //         );
    //     }

    //     let script_state = self.before_load(guard.clone(), handler_ctxt, is_reload);

    //     let result = if is_reload {
    //         self.reload_context(guard, handler_ctxt)
    //     } else {
    //         self.load_context(guard, handler_ctxt)
    //     };

    //     (result, script_state, is_reload)
    // }

    // fn handle_individual_context(
    //     &self,
    //     guard: WorldGuard,
    //     handler_ctxt: &mut HandlerContext<P>,
    // ) -> (Result<(), ScriptError>, Option<ScriptValue>, bool) {
    //     let is_new_script = !handler_ctxt.scripts.scripts.contains_key(&self.id);
    //     let is_reload = !is_new_script;

    //     debug!(
    //         "{}: CreateOrUpdateScript command applying (script_id: {}, new context?: {}, new script?: {})",
    //         P::LANGUAGE,
    //         self.id,
    //         is_new_script,
    //         !handler_ctxt.scripts.scripts.contains_key(&self.id)
    //     );

    //     let script_state = self.before_load(guard.clone(), handler_ctxt, is_reload);
    //     let result = if is_new_script {
    //         self.load_context(guard, handler_ctxt)
    //     } else {
    //         self.reload_context(guard, handler_ctxt)
    //     };
    //     (result, script_state, is_reload)
    // }

    pub(crate) fn create_or_update_script(
        context_key: &ContextKey,
        content: Option<&[u8]>,
        guard: WorldGuard,
        handler_ctxt: &mut HandlerContext<P>) -> Result<(), ScriptError> {

        let mut script_id = &Handle::default();
        let Some(content) = content.or_else(|| context_key.script_id.as_ref().and_then(|id| {
            script_id = id;
            handler_ctxt.scripts.get(script_id)
                                .map(|script| &*script.content)
                // .ok_or_else(|| ScriptError::new(InteropError::missing_script(id)))
        })) else {
            warn!("No content for context {} to create or update", context_key);
            match &context_key.script_id {
                Some(script_id) => {
                    return Err(ScriptError::new(InteropError::missing_script(script_id.clone())));
                }
                None => {
                    return Err(ScriptError::new(String::from("No content and no script given.")));
                }
            }
        };
        let phrase;
        let success;
        let result = match handler_ctxt.script_context.get(context_key) {
            Some(context) => {
                bevy::log::debug!("{}: reloading context {}", P::LANGUAGE, context_key);
                let mut lcontext = context.lock();
                phrase = "reloading";
                success = "updated";
                Self::reload_context(context_key, content, &mut lcontext, guard.clone(), handler_ctxt)
                    .map(|_| None)
            }
            None => {
                bevy::log::debug!("{}: loading context {}", P::LANGUAGE, context_key);
                phrase = "loading";
                success = "created";
                Self::load_context(context_key, content, guard.clone(), handler_ctxt)
                    .map(Some)
            }
        };

        match result {
            Ok(maybe_context) => {
                if let Some(context) = maybe_context {
                    if handler_ctxt.script_context.insert(context_key.clone(), context).is_err() {
                        warn!("Unable to insert script context for {}.", context_key);
                    }
                }

                bevy::log::debug!(
                    "{}: script {} successfully {}",
                    P::LANGUAGE,
                    context_key,
                    success,
                );
                Ok(())// none until individual context support added.
            }
            Err(err) => {
                handle_script_errors(
                    guard,
                    vec![err.clone()
                         .with_script(script_id.display())
                         .with_context(P::LANGUAGE)
                         .with_context(phrase)]
                        .into_iter(),
                );
                Err(err)
            }
        }
    }

}


#[profiling::all_functions]
impl<P: IntoScriptPluginParams> Command for CreateOrUpdateScript<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
        // with_handler_system_state(world, |guard, handler_ctxt: &mut HandlerContext<P>| {
        //     let (result, script_state, is_reload) =
        //         match handler_ctxt.context_loading_settings.assignment_strategy {
        //             crate::context::ContextAssignmentStrategy::Global => {
        //                 self.handle_global_context(guard.clone(), handler_ctxt)
        //             }
        //             crate::context::ContextAssignmentStrategy::Individual => {
        //                 self.handle_individual_context(guard.clone(), handler_ctxt)
        //             }
        //         };

        //     if let Err(err) = result {
        //         handle_script_errors(
        //             guard,
        //             vec![err
        //                 .with_script(self.id.clone())
        //                 .with_context(P::LANGUAGE)
        //                 .with_context(if is_reload {
        //                     "reloading an existing script or context"
        //                 } else {
        //                     "loading a new script or context"
        //                 })]
        //             .into_iter(),
        //         );
        //         return; // don't run after_load if there was an error
        //     }

        //     bevy::log::debug!(
        //         "{}: script with id: {} successfully created or updated",
        //         P::LANGUAGE,
        //         self.id
        //     );

        //     self.after_load(guard, handler_ctxt, script_state, is_reload);
        // });
        let result = with_handler_system_state(
            world,
            |guard, handler_ctxt: &mut HandlerContext<P>| {
               Self::create_or_update_script(&self.context_key, self.content.as_deref(),
                                             guard, handler_ctxt)
            });

        // Immediately run command for callback, but only if loading went fine.
        if result.is_ok() {
            Command::apply(RunScriptCallback::<P>::new(
                self.context_key,
                OnScriptLoaded::into_callback_label(),
                vec![],
                false,
            ), world);
        }
    }
}

#[profiling::all_functions]
impl<P: IntoScriptPluginParams> EntityCommand for CreateOrUpdateScript<P> {
    fn apply(mut self, entity: Entity, world: &mut bevy::prelude::World) {
        self.context_key.entity = Some(entity);
        Command::apply(self, world);
    }
}


/// Runs a callback on the script with the given ID if it exists
pub struct RunScriptCallback<P: IntoScriptPluginParams> {
    /// The context key
    pub context_key: ContextKey,
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
        context_key: impl Into<ContextKey>,
        callback: CallbackLabel,
        args: Vec<ScriptValue>,
        trigger_response: bool,
    ) -> Self {
        Self {
            context_key: context_key.into(),
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
        handler_ctxt: &mut HandlerContext<P>,
    ) -> Result<ScriptValue, ScriptError> {

            let result = handler_ctxt.call_dynamic_label(
                &self.callback,
                &self.context_key,
                None,
                self.args,
                guard.clone(),
            );

            if self.trigger_response {
                send_callback_response(
                    guard.clone(),
                    ScriptCallbackResponseEvent::new(
                        self.callback,
                        self.context_key.clone(),
                        result.clone(),
                    ),
                );
            }

            if let Err(ref err) = result {

                let mut error_with_context =
                    if let Some(script_id) = self.context_key .script_id .as_ref() {
                        err.clone().with_script(script_id.display())
                    } else {
                        err.clone()
                    }
                .with_context(P::LANGUAGE);
                for ctxt in self.context {
                    error_with_context = error_with_context.with_context(ctxt);
                }

                handle_script_errors(guard, vec![error_with_context].into_iter());
            }
        result
        // if !handler_ctxt.is_script_fully_loaded(self.id.clone()) {
        //     bevy::log::error!(
        //         "{}: Cannot apply callback {} command, as script does not exist: {}. Ignoring.",
        //         P::LANGUAGE,
        //         self.callback,
        //         self.id
        //     );
        //     return Err(ScriptError::new(InteropError::missing_script(
        //         self.id.clone(),
        //     )));
        // }

        // let result = handler_ctxt.call_dynamic_label(
        //     &self.callback,
        //     &self.id,
        //     self.entity,
        //     self.args,
        //     guard.clone(),
        // );

        // if self.trigger_response {
        //     send_callback_response(
        //         guard.clone(),
        //         ScriptCallbackResponseEvent::new(
        //             self.entity,
        //             self.callback,
        //             self.id.clone(),
        //             result.clone(),
        //         ),
        //     );
        // }

        // if let Err(err) = &result {
        //     let mut error_with_context = err.clone().with_script(self.id).with_context(P::LANGUAGE);
        //     for ctxt in &self.context {
        //         error_with_context = error_with_context.with_context(ctxt);
        //     }

        //     handle_script_errors(guard, vec![error_with_context].into_iter());
        // }

        // result
    }

    /// Equivalent to running the command, but also returns the result of the callback.
    ///
    /// The returned error will already be handled and logged.
    pub fn run(self, world: &mut bevy::prelude::World) -> Result<ScriptValue, ScriptError> {
        with_handler_system_state(world, |guard, handler_ctxt: &mut HandlerContext<P>| {
            self.run_with_handler(guard, handler_ctxt)
        })
    }
}

impl<P: IntoScriptPluginParams> Command for RunScriptCallback<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
// <<<<<<< HEAD
        // // internals handle this
        let _ = self.run(world);
        // with_handler_system_state(world, |guard, handler_ctxt: &mut HandlerContext<P>| {
            // if !handler_ctxt.is_script_fully_loaded(self.id.id()) {
            //     bevy::log::error!(
            //         "{}: Cannot apply callback command, as script {} does not exist. Ignoring.",
            //         P::LANGUAGE,
            //         self.id.display()
            //     );
            //     return;
            // }

    }
}

impl<P: IntoScriptPluginParams> EntityCommand for RunScriptCallback<P> {
    fn apply(mut self, id: Entity, world: &mut bevy::prelude::World) {
        self.context_key.entity = Some(id);
        Command::apply(self, world);
    }
}

/// Adds a static script to the collection of static scripts
pub struct AddStaticScript {
    /// The ID of the script to add
    id: Handle<ScriptAsset>,
}

impl AddStaticScript {
    /// Creates a new AddStaticScript command with the given ID
    pub fn new(id: impl Into<Handle<ScriptAsset>>) -> Self {
        Self { id: id.into() }
    }
}

impl Command for AddStaticScript {
    fn apply(self, world: &mut bevy::prelude::World) {
        let mut static_scripts = world.get_resource_or_init::<StaticScripts>();
        static_scripts.insert(self.id.clone());
        let mut script_queue = world.resource_mut::<ScriptQueue>();
        script_queue.push_back(ContextKey::from(self.id));
    }
}

/// Removes a static script from the collection of static scripts
pub struct RemoveStaticScript {
    /// The ID of the script to remove
    id: Handle<ScriptAsset>,
}

impl RemoveStaticScript {
    /// Creates a new RemoveStaticScript command with the given ID
    pub fn new(id: Handle<ScriptAsset>) -> Self {
        Self { id }
    }
}

#[profiling::all_functions]
impl Command for RemoveStaticScript {
    fn apply(self, world: &mut bevy::prelude::World) {
        let mut static_scripts = world.get_resource_or_init::<StaticScripts>();
        static_scripts.remove(&self.id);
    }
}

#[cfg(test)]
mod test {
    use bevy::{
        log::{Level, LogPlugin},
        prelude::*,
    };

    use crate::{
        ManageStaticScripts,
        asset::{ScriptQueue, Language},
        script::ScriptContext,
        bindings::script_value::ScriptValue,
        context::{ContextBuilder, ContextLoadingSettings},
        handler::CallbackSettings,
        runtime::RuntimeContainer,
    };

    use super::*;

    fn setup_app() -> App {
        // setup all the resources necessary
        let mut app = App::new();

        app.add_event::<ScriptCallbackResponseEvent>();
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.init_asset::<ScriptAsset>();
        app.init_resource::<ScriptQueue>();
        app.add_plugins(LogPlugin {
            filter: "bevy_mod_scripting_core=debug,info".to_owned(),
            level: Level::TRACE,
            ..Default::default()
        });

        app.insert_resource(ContextLoadingSettings::<DummyPlugin> {
            loader: ContextBuilder {
                load: |name, c, init, pre_run_init, _| {
                    let mut context = String::from_utf8_lossy(c).into();
                    for init in init {
                        init(name, &mut context)?;
                    }
                    for init in pre_run_init {
                        init(name, &mut context)?;
                    }
                    Ok(context)
                },
                reload: |name, new, existing, init, pre_run_init, _| {
                    let mut new = String::from_utf8_lossy(new).to_string();
                    for init in init {
                        init(name, &mut new)?;
                    }
                    for init in pre_run_init {
// <<<<<<< HEAD
                        // init(name, Entity::from_raw(0), &mut new)?;
                        init(name, existing)?;
                    }
                    existing.push_str(" | ");
                    existing.push_str(&new);
                    Ok(())
                },
            },
            assignment_strategy: Default::default(),
            context_initializers: vec![|_, c| {
                c.push_str(" initialized");
                Ok(())
            }],
            context_pre_handling_initializers: vec![|_, c| {
                c.push_str(" pre-handling-initialized");
                Ok(())
            }],
        })
        .insert_resource(RuntimeContainer::<DummyPlugin> {
            runtime: "Runtime".to_string(),
        })
        .init_resource::<StaticScripts>()
        .insert_resource(CallbackSettings::<DummyPlugin> {
            callback_handler: |_, _, callback, c, _, _| {
                c.push_str(format!(" callback-ran-{callback}").as_str());
                Ok(ScriptValue::Unit)
            },
        })
            ;

        app
    }

    struct DummyPlugin;

    impl IntoScriptPluginParams for DummyPlugin {
        type R = String;
        type C = String;
        const LANGUAGE: Language = Language::Unknown;

        fn build_runtime() -> Self::R {
            "Runtime".to_string()
        }
    }

    fn assert_context_and_script(world: &World, context_key: impl Into<ContextKey>, context: &str, message: &str) {
        let scripts = world.get_resource::<ScriptContext<DummyPlugin>>().unwrap();

        let context_key: ContextKey = context_key.into();
        let context_arc = scripts.get(&context_key)
            .unwrap_or_else(|| panic!("Context not found {message}"));

        // assert_eq!(id, script.id);
        let found_context = context_arc.lock();

        assert_eq!(context, found_context.as_str(), "{message}");
    }

    fn assert_response_events(
        app: &mut World,
        expected: impl Iterator<Item = ScriptCallbackResponseEvent>,
        context: &'static str,
    ) {
        let mut events = app
            .get_resource_mut::<Events<ScriptCallbackResponseEvent>>()
            .unwrap();
        let responses = events.drain().collect::<Vec<_>>();
        let expected: Vec<_> = expected.collect();
        assert_eq!(
            responses.len(),
            expected.len(),
            "Incorrect amount of events received {context}"
        );
        for (a, b) in responses.iter().zip(expected.iter()) {
            assert_eq!(a.label, b.label, "{context}");
            assert_eq!(a.context_key, b.context_key, "{context}");
            assert_eq!(a.response, b.response, "{context}");
        }
    }

    #[test]
    fn test_commands_with_default_assigner() {
        let mut app = setup_app();
        // app.insert_resource(ScriptContext::<DummyPlugin>::per_script());
        let script = add_script(&mut app, "content");
        let command = CreateOrUpdateScript::<DummyPlugin>::new(script.clone());
        Command::apply(command, app.world_mut());

        // check script
        let loaded_script_expected_content =
            "content initialized pre-handling-initialized callback-ran-on_script_loaded";
        assert_context_and_script(
            app.world_mut(),
            script.id(),
            loaded_script_expected_content,
            "Initial script creation failed",
        );

        // update the script
        let content = "new content";
        // let command = CreateOrUpdateScript::<DummyPlugin>::new(script.clone(), Some(content), None);
        let command = CreateOrUpdateScript::<DummyPlugin>::new(script.clone()).with_content(content);
        Command::apply(command, app.world_mut());

        // check script
        let reloaded_script_expected_content = format!("{loaded_script_expected_content} callback-ran-on_script_unloaded \
            | new content initialized pre-handling-initialized callback-ran-on_script_loaded callback-ran-on_script_reloaded");

        assert_context_and_script(
            app.world_mut(),
            script.id(),
            &reloaded_script_expected_content,
            "Script update failed",
        );

        // create second script
        let script2 = add_script(&mut app, "content2");
        let command = CreateOrUpdateScript::<DummyPlugin>::new(script2.clone());

        Command::apply(command, app.world_mut());

        // check second script
        assert_context_and_script(
            app.world_mut(),
            script2.id(),
            "content2 initialized pre-handling-initialized callback-ran-on_script_loaded",
            "Second script creation failed",
        );

        // run a specific callback on the first script
        Command::apply(RunScriptCallback::<DummyPlugin>::new(
            script.clone(),
            OnScriptLoaded::into_callback_label(),
            vec![],
            true,
        ), app.world_mut());

        // check this has applied
        assert_context_and_script(
            app.world_mut(),
            script.id(),
            &format!("{reloaded_script_expected_content} callback-ran-on_script_loaded"),
            "Script callback failed",
        );
        // assert events sent
        assert_response_events(
            app.world_mut(),
            vec![ScriptCallbackResponseEvent::new(
                Entity::from_raw(0),
                OnScriptLoaded::into_callback_label(),
                script.id(),
                Ok(ScriptValue::Unit),
            )]
            .into_iter(),
            "script callback response failed",
        );

        // delete both scripts
        let command = DeleteScript::<DummyPlugin>::new(script.id());
        Command::apply(command, app.world_mut());
        let command = DeleteScript::<DummyPlugin>::new(script2.id());
        Command::apply(command, app.world_mut());

        // check that the scripts are gone
        let scripts = app
            .world_mut()
            .get_resource::<StaticScripts>()
            .unwrap();
        assert!(scripts.scripts.is_empty());

        assert_response_events(
            app.world_mut(),
            vec![].into_iter(),
            "did not expect response events",
        );
    }

    fn add_script(app: &mut App, content: impl Into<String>) -> Handle<ScriptAsset> {
        app.world_mut().resource_mut::<Assets<ScriptAsset>>().add(ScriptAsset::from(content.into()))
    }

    fn update_script(app: &mut App, handle: AssetId<ScriptAsset>, content: impl Into<String>) {
        let mut scripts = app.world_mut().resource_mut::<Assets<ScriptAsset>>();
        let script_asset = scripts.get_mut(handle).unwrap();
        script_asset.content = content.into().into_bytes().into_boxed_slice();
    }

    #[test]
    fn test_commands_with_global_assigner() {
        // setup all the resources necessary
        let mut app = setup_app();

        let mut settings = app
            .world_mut()
            .get_resource_mut::<ContextLoadingSettings<DummyPlugin>>()
            .unwrap();

        settings.assignment_strategy = crate::context::ContextAssignmentStrategy::Global;
        app.insert_resource(ScriptContext::<DummyPlugin>::shared());

        // create a script
        let script = add_script(&mut app, "content");
        let command = CreateOrUpdateScript::<DummyPlugin>::new(script.clone());

        app.add_static_script(script.clone());
        Command::apply(command, app.world_mut());

        // check script
        let loaded_script_expected_content =
            "content initialized pre-handling-initialized callback-ran-on_script_loaded";
        assert_context_and_script(
            app.world(),
            script.id(),
            loaded_script_expected_content,
            "Initial script creation failed",
        );

        // update the script

        update_script(&mut app, script.id(), "new content");
        let command = CreateOrUpdateScript::<DummyPlugin>::new(script.clone());

        Command::apply(command, app.world_mut());

        // check script

        let second_loaded_script_expected_content =
            format!("{loaded_script_expected_content} callback-ran-on_script_unloaded \
            | new content initialized pre-handling-initialized callback-ran-on_script_loaded callback-ran-on_script_reloaded");
        assert_context_and_script(
            app.world(),
            script.id(),
            &second_loaded_script_expected_content,
            "Script update failed",
        );

        // create second script

        let script2 = add_script(&mut app, "content2");
        app.add_static_script(script2.clone());
        let command = CreateOrUpdateScript::<DummyPlugin>::new(script2.clone());

        Command::apply(command, app.world_mut());

        // check both scripts have the new context
        let third_loaded_script_expected_content = format!(
            "{second_loaded_script_expected_content} callback-ran-on_script_unloaded \
            | content2 initialized pre-handling-initialized callback-ran-on_script_loaded callback-ran-on_script_reloaded",
        );
        assert_context_and_script(
            app.world(),
            script2.id(),
            &third_loaded_script_expected_content,
            "second script context was not created correctly",
        );
        assert_context_and_script(
            app.world(),
            script2.id(),
            // "script",
            &third_loaded_script_expected_content,
            "First script context was not updated on second script insert",
        );

        let scripts = app.world().get_resource::<StaticScripts>().unwrap();
        assert_eq!(scripts.scripts.len(), 2);

        // delete first script
        let command = DeleteScript::<DummyPlugin>::new(script);

        Command::apply(command, app.world_mut());

        // check second script still has the context, and on unload was called
        assert_context_and_script(
            app.world(),
            script2.id(),
            &format!("{third_loaded_script_expected_content} callback-ran-on_script_unloaded"),
            "first script unload didn't have the desired effect",
        );

        // delete second script

        let command = DeleteScript::<DummyPlugin>::new(script2);

        Command::apply(command, app.world_mut());

        // check that the scripts are gone, and so is the context

        let scripts = app.world().get_resource::<StaticScripts>().unwrap();
        assert!(scripts.scripts.is_empty());

        let scripts = app.world().get_resource::<StaticScripts>().unwrap();
        assert_eq!(scripts.scripts.len(), 0, "scripts weren't removed");
        assert_response_events(
            app.world_mut(),
            vec![].into_iter(),
            "did not expect any response events",
        );
    }

    #[test]
    fn test_static_scripts() {
        let mut app = setup_app();

        let script = add_script(&mut app, "");
        let world = app.world_mut();

        let command = AddStaticScript::new(script.clone());
        Command::apply(command, world);

        let static_scripts = world.get_resource::<StaticScripts>().unwrap();
        assert!(static_scripts.contains(&script));

        let command = RemoveStaticScript::new(script.clone());
        Command::apply(command, world);

        let static_scripts = world.get_resource::<StaticScripts>().unwrap();
        assert!(!static_scripts.contains(&script));
    }
}
