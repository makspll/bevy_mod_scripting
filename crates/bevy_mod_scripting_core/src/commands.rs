//! Commands for creating, updating and deleting scripts

use crate::{
    ScriptQueue,
    AssetId,
    ScriptContext,
    asset::ScriptAsset,
    bindings::{ScriptValue, WorldGuard},
    context::ContextBuilder,
    error::{InteropError, ScriptError},
    event::{
        CallbackLabel, IntoCallbackLabel, OnScriptLoaded, OnScriptUnloaded,
        ScriptCallbackResponseEvent,
    },
    extractors::{with_handler_system_state, HandlerContext},
    handler::{handle_script_errors, send_callback_response},
    script::{StaticScripts, DisplayProxy, ScriptContextProvider, Domain, ContextKey},
    IntoScriptPluginParams,
};
use bevy::{asset::Handle, ecs::entity::Entity, log::{warn, debug}, prelude::{EntityCommand, Command}};
use std::marker::PhantomData;

/// Deletes a script with the given ID
pub struct DeleteScript<P: IntoScriptPluginParams> {
    /// The ID of the script to delete
    pub id: AssetId<ScriptAsset>,
    /// entity associated with script
    pub entity: Option<Entity>,
    /// domain associated with script
    pub domain: Option<Domain>,
    /// hack to make this Send, C does not need to be Send since it is not stored in the command
    pub _ph: PhantomData<fn(P::C, P::R)>,
}

impl<P: IntoScriptPluginParams> DeleteScript<P> {
    /// Creates a new DeleteScript command with the given ID
    pub fn new(entity: Option<Entity>, id: AssetId<ScriptAsset>, domain: Option<Domain>) -> Self {
        Self {
            id,
            domain,
            entity,
            _ph: PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> Command for DeleteScript<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
        // first apply unload callback
        RunScriptCallback::<P>::new(
            Handle::Weak(self.id),
            self.entity.clone(),
            self.domain.clone(),
            OnScriptUnloaded::into_callback_label(),
            vec![],
            false,
        )
        .apply(world);

        let mut deleted = false;
        {
            let mut scripts = world.get_resource_or_init::<StaticScripts>();
            if scripts.remove(self.id) {
                debug!("Deleted static script with id: {}", self.id);
                deleted = true;
            }
        }
        {
            let mut script_contexts = world.get_resource_or_init::<ScriptContext<P>>();

            let key = ContextKey {
                entity: self.entity,
                script_id: Some(self.id),
                domain: self.domain,
            };
            if script_contexts.remove(&key) {
                bevy::log::info!("{}: Deleted context for script {:?}", P::LANGUAGE, self.id);
                deleted = true;
            }
        }
        if !deleted {
            bevy::log::error!(
                "Attempted to delete script with id {} but it does not exist; doing nothing!",
                self.id
            );
        }
    }
}

/// Creates new script with the given ID, if a script with the given ID already exists, this is treated as an update
///
/// If script comes from an asset, expects it to be loaded, otherwise this command will fail to process the script.
pub struct CreateOrUpdateScript<P: IntoScriptPluginParams> {
    id: Handle<ScriptAsset>,
    // It feels like we're using a Box, which requires a clone merely to satisfy the Command trait.
    content: Option<Box<[u8]>>,
    domain: Option<Domain>,
    // Hack to make this Send, C does not need to be Send since it is not stored in the command
    _ph: std::marker::PhantomData<fn(P::R, P::C)>,
}

#[profiling::all_functions]
impl<P: IntoScriptPluginParams> CreateOrUpdateScript<P> {
    /// Creates a new CreateOrUpdateScript command with the given ID, content and asset
    pub fn new(id: Handle<ScriptAsset>, content: Option<Box<[u8]>>, domain: Option<Domain>) -> Self {
        Self {
            id,
            content,
            domain,
            _ph: std::marker::PhantomData,
        }
    }

    fn reload_context(
        id: &Handle<ScriptAsset>,
        content: &[u8],
        context: &mut P::C,
        guard: WorldGuard,
        handler_ctxt: &HandlerContext<P>,
    ) -> Result<(), ScriptError> {
        // let mut context = script
        //     .contexts
        //     .get_mut(&id.id())
        //     .ok_or_else(|| InteropError::invariant("Tried to reload script which doesn't have a context"))?;

        // reload context
        // let mut context = context.lock();

        (ContextBuilder::<P>::reload)(
            handler_ctxt.context_loading_settings.loader.reload,
            id,
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
        id: &Handle<ScriptAsset>,
        content: &[u8],
        guard: WorldGuard,
        handler_ctxt: &HandlerContext<P>,
    ) -> Result<P::C, ScriptError> {
        let context = (ContextBuilder::<P>::load)(
            handler_ctxt.context_loading_settings.loader.load,
            id,
            content,
            &handler_ctxt.context_loading_settings.context_initializers,
            &handler_ctxt
                .context_loading_settings
                .context_pre_handling_initializers,
            guard.clone(),
            &handler_ctxt.runtime_container.runtime,
        )?;

        // Ok(Arc::new(Mutex::new(context)))
        Ok(context)
    }

    pub(crate) fn create_or_update_script(
        context_key: &ContextKey,
        content: Option<&[u8]>,
        guard: WorldGuard,
        handler_ctxt: &mut HandlerContext<P>) -> Result<(), ScriptError> {

        let script_id;
        let Some(content) = content.or_else(|| context_key.script_id.and_then(|id| {
            script_id = id;
            handler_ctxt.scripts.get(id).map(|script| &*script.content)
        })) else {
            warn!("No content for context {} to create or update", context_key);
            return Err(ScriptError::new(InteropError::missing_script(
                context_key.script_id.unwrap_or_default()
                // id.clone(),
            )));
        };
        let phrase;
        let success;
        let result = match handler_ctxt.script_context.get(context_key) {
            Some(context) => {
                bevy::log::debug!("{}: reloading context {}", P::LANGUAGE, context_key);
                let mut lcontext = context.lock();
                phrase = "reloading";
                success = "updated";
                Self::reload_context(script_id, content, &mut lcontext, guard.clone(), handler_ctxt)
                    .map(|_| None)
            }
            None => {
                bevy::log::debug!("{}: loading context {}", P::LANGUAGE, context_key);
                phrase = "loading";
                success = "created";
                Self::load_context(script_id, content, guard.clone(), handler_ctxt)
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
                         .with_script(id.display())
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
        let result = with_handler_system_state(
            world,
            |guard, handler_ctxt: &mut HandlerContext<P>| {
               Self::create_or_update_script(None, &self.id, self.content.as_deref(), &self.domain,
                                             guard, handler_ctxt)
            });

        // immediately run command for callback, but only if loading went fine
        if let Ok(_) = result {

            RunScriptCallback::<P>::new(
                self.id,
                None,
                self.domain,
                OnScriptLoaded::into_callback_label(),
                vec![],
                false,
            )
                .apply(world)
        }
    }
}

#[profiling::all_functions]
impl<P: IntoScriptPluginParams> EntityCommand for CreateOrUpdateScript<P> {
    fn apply(self, entity: Entity, world: &mut bevy::prelude::World) {
        let result = with_handler_system_state(
            world,
            |guard, handler_ctxt: &mut HandlerContext<P>| {
               Self::create_or_update_script(Some(entity), &self.id, self.content.as_deref(), &self.domain,
                                             guard, handler_ctxt)
            });

        // Immediately run command for callback, but only if loading went fine.
        if result.is_ok() {

            RunScriptCallback::<P>::new(
                self.id,
                Some(entity),
                self.domain,
                OnScriptLoaded::into_callback_label(),
                vec![],
                false,
            )
                .apply(world)
        }
    }
}


/// Runs a callback on the script with the given ID if it exists
pub struct RunScriptCallback<P: IntoScriptPluginParams> {
    /// The ID of the script to run the callback on
    pub id: Handle<ScriptAsset>,
    /// The entity to use for the callback
    pub entity: Option<Entity>,
    /// The domain if any
    pub domain: Option<Domain>,
    /// The callback to run
    pub callback: CallbackLabel,
    /// optional context passed down to errors
    pub context: Option<&'static str>,
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
        id: Handle<ScriptAsset>,
        entity: Option<Entity>,
        domain: Option<Domain>,
        callback: CallbackLabel,
        args: Vec<ScriptValue>,
        trigger_response: bool,
    ) -> Self {
        Self {
            id,
            entity,
            domain,
            context: None,
            callback,
            args,
            trigger_response,
            _ph: std::marker::PhantomData,
        }
    }

    /// Sets the context for the command, makes produced errors more useful.
    pub fn with_context(mut self, context: &'static str) -> Self {
        self.context = Some(context);
        self
    }
}

impl<P: IntoScriptPluginParams> Command for RunScriptCallback<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
        with_handler_system_state(world, |guard, handler_ctxt: &mut HandlerContext<P>| {
            // if !handler_ctxt.is_script_fully_loaded(self.id.id()) {
            //     bevy::log::error!(
            //         "{}: Cannot apply callback command, as script {} does not exist. Ignoring.",
            //         P::LANGUAGE,
            //         self.id.display()
            //     );
            //     return;
            // }

            let result = handler_ctxt.call_dynamic_label(
                &self.callback,
                &self.id,
                self.entity,
                &self.domain,
                None,
                self.args,
                guard.clone(),
            );

            if self.trigger_response {
                send_callback_response(
                    guard.clone(),
                    ScriptCallbackResponseEvent::new(
                        self.callback,
                        self.id.id(),
                        result.clone(),
                    ),
                );
            }

            if let Err(err) = result {
                let mut error_with_context = err.with_script(self.id.display()).with_context(P::LANGUAGE);
                if let Some(ctxt) = self.context {
                    error_with_context = error_with_context.with_context(ctxt);
                }

                handle_script_errors(guard, vec![error_with_context].into_iter());
            }
        })
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
        script_queue.push_back((None, self.id, None));
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
        script::{ScriptId, ScriptContext},
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
                        init(name, None, &mut context)?;
                    }
                    Ok(context)
                },
                reload: |name, new, existing, init, pre_run_init, _| {
                    *existing = String::from_utf8_lossy(new).into();
                    for init in init {
                        init(name, existing)?;
                    }
                    for init in pre_run_init {
                        init(name, None, existing)?;
                    }
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

    fn assert_context_and_script(world: &World, id: &ScriptId, context: &str, message: &str) {
        let scripts = world.get_resource::<ScriptContext<DummyPlugin>>().unwrap();

        let context_arc = scripts.get(None, id, &None)
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

        let script = add_script(&mut app, "content");
        let command = CreateOrUpdateScript::<DummyPlugin>::new(script.clone(), None, None);
        Command::apply(command, app.world_mut());

        // check script
        assert_context_and_script(
            app.world_mut(),
            &script.id(),
            "content initialized pre-handling-initialized callback-ran-on_script_loaded",
            "Initial script creation failed",
        );

        // update the script
        let content = "new content".as_bytes().to_vec().into_boxed_slice();
        // let command = CreateOrUpdateScript::<DummyPlugin>::new(script.clone(), Some(content), None);
        let command = CreateOrUpdateScript::<DummyPlugin>::new(script.clone(), Some(content), None);
        Command::apply(command, app.world_mut());

        // check script
        assert_context_and_script(
            app.world_mut(),
            &script.id(),
            "new content initialized pre-handling-initialized callback-ran-on_script_loaded",
            "Script update failed",
        );

        // create second script
        let script2 = add_script(&mut app, "content2");
        let command = CreateOrUpdateScript::<DummyPlugin>::new(script2.clone(), None, None);

        Command::apply(command, app.world_mut());

        // check second script
        assert_context_and_script(
            app.world_mut(),
            &script2.id(),
            "content2 initialized pre-handling-initialized callback-ran-on_script_loaded",
            "Second script creation failed",
        );

        // run a specific callback on the first script
        RunScriptCallback::<DummyPlugin>::new(
            script.clone(),
            None,
            None,
            OnScriptLoaded::into_callback_label(),
            vec![],
            true,
        )
        .apply(app.world_mut());

        // check this has applied
        assert_context_and_script(
            app.world_mut(),
            &script.id(),
            "BLAH new content initialized pre-handling-initialized callback-ran-on_script_loaded callback-ran-on_script_loaded",
            "Script callback failed",
        );
        // assert events sent
        assert_response_events(
            app.world_mut(),
            vec![ScriptCallbackResponseEvent::new(
                OnScriptLoaded::into_callback_label(),
                script.id(),
                Ok(ScriptValue::Unit),
            )]
            .into_iter(),
            "script callback failed",
        );

        // delete both scripts
        let command = DeleteScript::<DummyPlugin>::new(None, script.id(), None);
        command.apply(app.world_mut());
        let command = DeleteScript::<DummyPlugin>::new(None, script2.id(), None);
        command.apply(app.world_mut());

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
        let mut script_asset = scripts.get_mut(handle).unwrap();
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
        let command = CreateOrUpdateScript::<DummyPlugin>::new(script.clone(), None, None);

        app.add_static_script(script.clone());
        Command::apply(command, app.world_mut());

        // check script
        assert_context_and_script(
            app.world(),
            &script.id(),
            "content initialized pre-handling-initialized callback-ran-on_script_loaded",
            "Initial script creation failed",
        );

        // update the script

        update_script(&mut app, script.id(), "new content");
        let command = CreateOrUpdateScript::<DummyPlugin>::new(script.clone(), None, None);

        Command::apply(command, app.world_mut());

        // check script

        assert_context_and_script(
            app.world(),
            &script.id(),
            "new content initialized pre-handling-initialized callback-ran-on_script_loaded",
            "Script update failed",
        );

        // create second script

        let script2 = add_script(&mut app, "content2");
        app.add_static_script(script2.clone());
        let command = CreateOrUpdateScript::<DummyPlugin>::new(script2.clone(), None, None);

        Command::apply(command, app.world_mut());

        // check both scripts have the new context

        assert_context_and_script(
            app.world(),
            &script2.id(),
            "content2 initialized pre-handling-initialized callback-ran-on_script_loaded",
            "second script context was not created correctly",
        );
        assert_context_and_script(
            app.world(),
            &script2.id(),
            "content2 initialized pre-handling-initialized callback-ran-on_script_loaded",
            "First script context was not updated on second script insert",
        );

        let scripts = app.world().get_resource::<StaticScripts>().unwrap();
        assert_eq!(scripts.scripts.len(), 2);

        // delete first script
        let command = DeleteScript::<DummyPlugin>::new(None,script.id(), None);

        Command::apply(command, app.world_mut());

        // check second script still has the context, and on unload was called
        assert_context_and_script(
            app.world(),
            &script2.id(),
            "content2 initialized pre-handling-initialized callback-ran-on_script_loaded callback-ran-on_script_unloaded",
            "first script unload didn't have the desired effect",
        );

        // delete second script

        let command = DeleteScript::<DummyPlugin>::new(None,script2.id(), None);

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
        command.apply(world);

        let static_scripts = world.get_resource::<StaticScripts>().unwrap();
        assert!(static_scripts.contains(&script));

        let command = RemoveStaticScript::new(script.clone());
        command.apply(world);

        let static_scripts = world.get_resource::<StaticScripts>().unwrap();
        assert!(!static_scripts.contains(&script));
    }
}
