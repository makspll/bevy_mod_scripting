//! Commands for creating, updating and deleting scripts

use crate::{
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
    script::{Script, ScriptId, Scripts, StaticScripts},
    IntoScriptPluginParams,
};
use bevy::{asset::Handle, ecs::entity::Entity, log::debug, prelude::Command};
use parking_lot::Mutex;
use std::{marker::PhantomData, sync::Arc};

/// Deletes a script with the given ID
pub struct DeleteScript<P: IntoScriptPluginParams> {
    /// The ID of the script to delete
    pub id: ScriptId,
    /// hack to make this Send, C does not need to be Send since it is not stored in the command
    pub _ph: PhantomData<fn(P::C, P::R)>,
}

impl<P: IntoScriptPluginParams> DeleteScript<P> {
    /// Creates a new DeleteScript command with the given ID
    pub fn new(id: ScriptId) -> Self {
        Self {
            id,
            _ph: PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> Command for DeleteScript<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
        // first apply unload callback
        RunScriptCallback::<P>::new(
            self.id.clone(),
            Entity::from_raw(0),
            OnScriptUnloaded::into_callback_label(),
            vec![],
            false,
        )
        .apply(world);

        let mut scripts = world.get_resource_or_init::<Scripts<P>>();
        if scripts.remove(self.id.clone()) {
            debug!("Deleted script with id: {}", self.id);
        } else {
            bevy::log::error!(
                "Attempted to delete script with id: {} but it does not exist, doing nothing!",
                self.id
            );
        }
    }
}

/// Creates new script with the given ID, if a script with the given ID already exists, this is treated as an update
///
/// If script comes from an asset, expects it to be loaded, otherwise this command will fail to process the script.
pub struct CreateOrUpdateScript<P: IntoScriptPluginParams> {
    id: ScriptId,
    content: Box<[u8]>,
    asset: Option<Handle<ScriptAsset>>,
    // Hack to make this Send, C does not need to be Send since it is not stored in the command
    _ph: std::marker::PhantomData<fn(P::R, P::C)>,
}

#[profiling::all_functions]
impl<P: IntoScriptPluginParams> CreateOrUpdateScript<P> {
    /// Creates a new CreateOrUpdateScript command with the given ID, content and asset
    pub fn new(id: ScriptId, content: Box<[u8]>, asset: Option<Handle<ScriptAsset>>) -> Self {
        Self {
            id,
            content,
            asset,
            _ph: std::marker::PhantomData,
        }
    }

    fn reload_context(
        &self,
        guard: WorldGuard,
        handler_ctxt: &HandlerContext<P>,
    ) -> Result<(), ScriptError> {
        bevy::log::debug!("{}: reloading script with id: {}", P::LANGUAGE, self.id);
        let existing_script = match handler_ctxt.scripts.scripts.get(&self.id) {
            Some(script) => script,
            None => {
                return Err(
                    InteropError::invariant("Tried to reload script which doesn't exist").into(),
                )
            }
        };

        // reload context
        let mut context = existing_script.context.lock();

        (ContextBuilder::<P>::reload)(
            handler_ctxt.context_loading_settings.loader.reload,
            &self.id,
            &self.content,
            &mut context,
            &handler_ctxt.context_loading_settings.context_initializers,
            &handler_ctxt
                .context_loading_settings
                .context_pre_handling_initializers,
            guard.clone(),
            &handler_ctxt.runtime_container.runtime,
        )?;

        Ok(())
    }

    fn load_context(
        &self,
        guard: WorldGuard,
        handler_ctxt: &mut HandlerContext<P>,
    ) -> Result<(), ScriptError> {
        bevy::log::debug!("{}: loading script with id: {}", P::LANGUAGE, self.id);

        let context = (ContextBuilder::<P>::load)(
            handler_ctxt.context_loading_settings.loader.load,
            &self.id,
            &self.content,
            &handler_ctxt.context_loading_settings.context_initializers,
            &handler_ctxt
                .context_loading_settings
                .context_pre_handling_initializers,
            guard.clone(),
            &handler_ctxt.runtime_container.runtime,
        )?;

        let context = Arc::new(Mutex::new(context));

        handler_ctxt.scripts.scripts.insert(
            self.id.clone(),
            Script {
                id: self.id.clone(),
                asset: self.asset.clone(),
                context,
            },
        );
        Ok(())
    }

    fn before_load(
        &self,
        world: WorldGuard,
        handler_ctxt: &mut HandlerContext<P>,
        is_reload: bool,
    ) -> Option<ScriptValue> {
        if is_reload {
            // if something goes wrong, the error will be handled in the command
            // but we will not pass the script state to the after_load
            return RunScriptCallback::<P>::new(
                self.id.clone(),
                Entity::from_raw(0),
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
        &self,
        world: WorldGuard,
        handler_ctxt: &mut HandlerContext<P>,
        script_state: Option<ScriptValue>,
        is_reload: bool,
    ) {
        let _ = RunScriptCallback::<P>::new(
            self.id.clone(),
            Entity::from_raw(0),
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
                self.id.clone(),
                Entity::from_raw(0),
                OnScriptReloaded::into_callback_label(),
                vec![state],
                false,
            )
            .with_context(P::LANGUAGE)
            .with_context("on reloaded callback")
            .run_with_handler(world, handler_ctxt);
        }
    }

    fn handle_global_context(
        &self,
        guard: WorldGuard,
        handler_ctxt: &mut HandlerContext<P>,
    ) -> (Result<(), ScriptError>, Option<ScriptValue>, bool) {
        let existing_context = handler_ctxt
            .scripts
            .scripts
            .values()
            .next()
            .map(|s| s.context.clone());

        debug!(
            "{}: CreateOrUpdateScript command applying to global context (script_id: {}, new context?: {}, new script?: {})",
            P::LANGUAGE,
            self.id,
            existing_context.is_none(),
            !handler_ctxt.scripts.scripts.contains_key(&self.id)
        );

        let is_reload = existing_context.is_some();

        if let Some(context) = existing_context {
            // point all new scripts to the shared context
            handler_ctxt.scripts.scripts.insert(
                self.id.clone(),
                Script {
                    id: self.id.clone(),
                    asset: self.asset.clone(),
                    context,
                },
            );
        }

        let script_state = self.before_load(guard.clone(), handler_ctxt, is_reload);

        let result = if is_reload {
            self.reload_context(guard, handler_ctxt)
        } else {
            self.load_context(guard, handler_ctxt)
        };

        (result, script_state, is_reload)
    }

    fn handle_individual_context(
        &self,
        guard: WorldGuard,
        handler_ctxt: &mut HandlerContext<P>,
    ) -> (Result<(), ScriptError>, Option<ScriptValue>, bool) {
        let is_new_script = !handler_ctxt.scripts.scripts.contains_key(&self.id);
        let is_reload = !is_new_script;

        debug!(
            "{}: CreateOrUpdateScript command applying (script_id: {}, new context?: {}, new script?: {})",
            P::LANGUAGE,
            self.id,
            is_new_script,
            !handler_ctxt.scripts.scripts.contains_key(&self.id)
        );

        let script_state = self.before_load(guard.clone(), handler_ctxt, is_reload);
        let result = if is_new_script {
            self.load_context(guard, handler_ctxt)
        } else {
            self.reload_context(guard, handler_ctxt)
        };
        (result, script_state, is_reload)
    }
}

#[profiling::all_functions]
impl<P: IntoScriptPluginParams> Command for CreateOrUpdateScript<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
        with_handler_system_state(world, |guard, handler_ctxt: &mut HandlerContext<P>| {
            let (result, script_state, is_reload) =
                match handler_ctxt.context_loading_settings.assignment_strategy {
                    crate::context::ContextAssignmentStrategy::Global => {
                        self.handle_global_context(guard.clone(), handler_ctxt)
                    }
                    crate::context::ContextAssignmentStrategy::Individual => {
                        self.handle_individual_context(guard.clone(), handler_ctxt)
                    }
                };

            if let Err(err) = result {
                handle_script_errors(
                    guard,
                    vec![err
                        .with_script(self.id.clone())
                        .with_context(P::LANGUAGE)
                        .with_context(if is_reload {
                            "reloading an existing script or context"
                        } else {
                            "loading a new script or context"
                        })]
                    .into_iter(),
                );
                return; // don't run after_load if there was an error
            }

            bevy::log::debug!(
                "{}: script with id: {} successfully created or updated",
                P::LANGUAGE,
                self.id
            );

            self.after_load(guard, handler_ctxt, script_state, is_reload);
        });
    }
}

/// Runs a callback on the script with the given ID if it exists
pub struct RunScriptCallback<P: IntoScriptPluginParams> {
    /// The ID of the script to run the callback on
    pub id: ScriptId,
    /// The entity to use for the callback
    pub entity: Entity,
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
        id: ScriptId,
        entity: Entity,
        callback: CallbackLabel,
        args: Vec<ScriptValue>,
        trigger_response: bool,
    ) -> Self {
        Self {
            id,
            entity,
            context: Default::default(),
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
        if !handler_ctxt.is_script_fully_loaded(self.id.clone()) {
            bevy::log::error!(
                "{}: Cannot apply callback {} command, as script does not exist: {}. Ignoring.",
                P::LANGUAGE,
                self.callback,
                self.id
            );
            return Err(ScriptError::new(InteropError::missing_script(
                self.id.clone(),
            )));
        }

        let result = handler_ctxt.call_dynamic_label(
            &self.callback,
            &self.id,
            self.entity,
            self.args,
            guard.clone(),
        );

        if self.trigger_response {
            send_callback_response(
                guard.clone(),
                ScriptCallbackResponseEvent::new(
                    self.entity,
                    self.callback,
                    self.id.clone(),
                    result.clone(),
                ),
            );
        }

        if let Err(err) = &result {
            let mut error_with_context = err.clone().with_script(self.id).with_context(P::LANGUAGE);
            for ctxt in &self.context {
                error_with_context = error_with_context.with_context(ctxt);
            }

            handle_script_errors(guard, vec![error_with_context].into_iter());
        }

        result
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
        // internals handle this
        let _ = self.run(world);
    }
}

/// Adds a static script to the collection of static scripts
pub struct AddStaticScript {
    /// The ID of the script to add
    id: ScriptId,
}

impl AddStaticScript {
    /// Creates a new AddStaticScript command with the given ID
    pub fn new(id: impl Into<ScriptId>) -> Self {
        Self { id: id.into() }
    }
}

impl Command for AddStaticScript {
    fn apply(self, world: &mut bevy::prelude::World) {
        let mut static_scripts = world.get_resource_or_init::<StaticScripts>();
        static_scripts.insert(self.id);
    }
}

/// Removes a static script from the collection of static scripts
pub struct RemoveStaticScript {
    /// The ID of the script to remove
    id: ScriptId,
}

impl RemoveStaticScript {
    /// Creates a new RemoveStaticScript command with the given ID
    pub fn new(id: ScriptId) -> Self {
        Self { id }
    }
}

#[profiling::all_functions]
impl Command for RemoveStaticScript {
    fn apply(self, world: &mut bevy::prelude::World) {
        let mut static_scripts = world.get_resource_or_init::<StaticScripts>();
        static_scripts.remove(self.id);
    }
}

#[cfg(test)]
mod test {
    use bevy::{
        app::App,
        ecs::event::Events,
        log::{Level, LogPlugin},
        prelude::{Entity, World},
    };

    use crate::{
        asset::Language,
        bindings::script_value::ScriptValue,
        context::{ContextBuilder, ContextLoadingSettings},
        handler::CallbackSettings,
        runtime::RuntimeContainer,
        script::Scripts,
    };

    use super::*;

    fn setup_app() -> App {
        // setup all the resources necessary
        let mut app = App::new();

        app.add_event::<ScriptCallbackResponseEvent>();
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
                        init(name, Entity::from_raw(0), &mut context)?;
                    }
                    Ok(context)
                },
                reload: |name, new, existing, init, pre_run_init, _| {
                    let mut new = String::from_utf8_lossy(new).to_string();
                    for init in init {
                        init(name, &mut new)?;
                    }
                    for init in pre_run_init {
                        init(name, Entity::from_raw(0), &mut new)?;
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
            context_pre_handling_initializers: vec![|_, _, c| {
                c.push_str(" pre-handling-initialized");
                Ok(())
            }],
        })
        .insert_resource(RuntimeContainer::<DummyPlugin> {
            runtime: "Runtime".to_string(),
        })
        .init_resource::<StaticScripts>()
        .insert_resource(CallbackSettings::<DummyPlugin> {
            callback_handler: |_, _, _, callback, c, _, _| {
                c.push_str(format!(" callback-ran-{callback}").as_str());
                Ok(ScriptValue::Unit)
            },
        })
        .insert_resource(Scripts::<DummyPlugin> {
            scripts: Default::default(),
        });

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

    fn assert_context_and_script(world: &World, id: &str, context: &str, message: &str) {
        let scripts = world.get_resource::<Scripts<DummyPlugin>>().unwrap();

        let script = scripts
            .scripts
            .get(id)
            .unwrap_or_else(|| panic!("Script not found {message}"));

        assert_eq!(id, script.id);
        let found_context = script.context.lock();
        pretty_assertions::assert_eq!(
            *context,
            *found_context,
            "expected context != actual context. {message}"
        );
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
            assert_eq!(a.script, b.script, "{context}");
            assert_eq!(a.response, b.response, "{context}");
        }
    }

    #[test]
    fn test_commands_with_default_assigner() {
        let mut app = setup_app();

        let content = "content".as_bytes().to_vec().into_boxed_slice();
        let command = CreateOrUpdateScript::<DummyPlugin>::new("script".into(), content, None);
        command.apply(app.world_mut());

        // check script
        let loaded_script_expected_content =
            "content initialized pre-handling-initialized callback-ran-on_script_loaded";
        assert_context_and_script(
            app.world_mut(),
            "script",
            loaded_script_expected_content,
            "Initial script creation failed",
        );

        // update the script
        let content = "new content".as_bytes().to_vec().into_boxed_slice();
        let command = CreateOrUpdateScript::<DummyPlugin>::new("script".into(), content, None);
        command.apply(app.world_mut());

        // check script
        let reloaded_script_expected_content = format!("{loaded_script_expected_content} callback-ran-on_script_unloaded \
            | new content initialized pre-handling-initialized callback-ran-on_script_loaded callback-ran-on_script_reloaded");

        assert_context_and_script(
            app.world_mut(),
            "script",
            &reloaded_script_expected_content,
            "Script update failed",
        );

        // create second script
        let content = "content2".as_bytes().to_vec().into_boxed_slice();
        let command = CreateOrUpdateScript::<DummyPlugin>::new("script2".into(), content, None);

        command.apply(app.world_mut());

        // check second script
        assert_context_and_script(
            app.world_mut(),
            "script2",
            "content2 initialized pre-handling-initialized callback-ran-on_script_loaded",
            "Second script creation failed",
        );

        // run a specific callback on the first script
        RunScriptCallback::<DummyPlugin>::new(
            "script".into(),
            Entity::from_raw(0),
            OnScriptLoaded::into_callback_label(),
            vec![],
            true,
        )
        .apply(app.world_mut());

        // check this has applied
        assert_context_and_script(
            app.world_mut(),
            "script",
            &format!("{reloaded_script_expected_content} callback-ran-on_script_loaded"),
            "Script callback failed",
        );
        // assert events sent
        assert_response_events(
            app.world_mut(),
            vec![ScriptCallbackResponseEvent::new(
                Entity::from_raw(0),
                OnScriptLoaded::into_callback_label(),
                "script".into(),
                Ok(ScriptValue::Unit),
            )]
            .into_iter(),
            "script callback failed",
        );

        // delete both scripts
        let command = DeleteScript::<DummyPlugin>::new("script".into());
        command.apply(app.world_mut());
        let command = DeleteScript::<DummyPlugin>::new("script2".into());
        command.apply(app.world_mut());

        // check that the scripts are gone
        let scripts = app
            .world_mut()
            .get_resource::<Scripts<DummyPlugin>>()
            .unwrap();
        assert!(scripts.scripts.is_empty());

        assert_response_events(
            app.world_mut(),
            vec![].into_iter(),
            "did not expect response events",
        );
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

        // create a script
        let content = "content".as_bytes().to_vec().into_boxed_slice();
        let command = CreateOrUpdateScript::<DummyPlugin>::new("script".into(), content, None);

        command.apply(app.world_mut());

        // check script
        let loaded_script_expected_content =
            "content initialized pre-handling-initialized callback-ran-on_script_loaded";
        assert_context_and_script(
            app.world(),
            "script",
            loaded_script_expected_content,
            "Initial script creation failed",
        );

        // update the script

        let content = "new content".as_bytes().to_vec().into_boxed_slice();
        let command = CreateOrUpdateScript::<DummyPlugin>::new("script".into(), content, None);

        command.apply(app.world_mut());

        // check script

        let second_loaded_script_expected_content =
            format!("{loaded_script_expected_content} callback-ran-on_script_unloaded \
            | new content initialized pre-handling-initialized callback-ran-on_script_loaded callback-ran-on_script_reloaded");
        assert_context_and_script(
            app.world(),
            "script",
            &second_loaded_script_expected_content,
            "Script update failed",
        );

        // create second script

        let content = "content2".as_bytes().to_vec().into_boxed_slice();
        let command = CreateOrUpdateScript::<DummyPlugin>::new("script2".into(), content, None);

        command.apply(app.world_mut());

        // check both scripts have the new context
        let third_loaded_script_expected_content = format!(
            "{second_loaded_script_expected_content} callback-ran-on_script_unloaded \
            | content2 initialized pre-handling-initialized callback-ran-on_script_loaded callback-ran-on_script_reloaded",
        );
        assert_context_and_script(
            app.world(),
            "script2",
            &third_loaded_script_expected_content,
            "second script context was not created correctly",
        );
        assert_context_and_script(
            app.world(),
            "script",
            &third_loaded_script_expected_content,
            "First script context was not updated on second script insert",
        );

        let scripts = app.world().get_resource::<Scripts<DummyPlugin>>().unwrap();
        assert!(scripts.scripts.len() == 2);

        // delete first script
        let command = DeleteScript::<DummyPlugin>::new("script".into());

        command.apply(app.world_mut());

        // check second script still has the context, and on unload was called
        assert_context_and_script(
            app.world(),
            "script2",
            &format!("{third_loaded_script_expected_content} callback-ran-on_script_unloaded"),
            "first script unload didn't have the desired effect",
        );

        // delete second script

        let command = DeleteScript::<DummyPlugin>::new("script2".into());

        command.apply(app.world_mut());

        // check that the scripts are gone, and so is the context

        let scripts = app.world().get_resource::<Scripts<DummyPlugin>>().unwrap();
        assert!(scripts.scripts.is_empty());

        let scripts = app.world().get_resource::<Scripts<DummyPlugin>>().unwrap();

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

        let world = app.world_mut();

        let command = AddStaticScript::new("script");
        command.apply(world);

        let static_scripts = world.get_resource::<StaticScripts>().unwrap();
        assert!(static_scripts.contains("script"));

        let command = RemoveStaticScript::new("script".into());
        command.apply(world);

        let static_scripts = world.get_resource::<StaticScripts>().unwrap();
        assert!(!static_scripts.contains("script"));
    }
}
