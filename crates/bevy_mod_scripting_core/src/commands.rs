//! Commands for creating, updating and deleting scripts

use crate::{
    asset::ScriptAsset,
    bindings::WorldGuard,
    context::{ContextBuilder, DowncastContext},
    error::{InteropError, ScriptError},
    event::{IntoCallbackLabel, OnScriptLoaded, OnScriptUnloaded},
    extractors::{with_handler_system_state, HandlerContext},
    handler::{handle_script_errors, CallbackSettings},
    script::{Script, ScriptId, StaticScripts},
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
        with_handler_system_state(world, |guard, handler_ctxt: &mut HandlerContext<P>| {
            if let Some(script) = handler_ctxt.scripts.scripts.remove(&self.id) {
                debug!("Deleting script with id: {}", self.id);

                // first let the script uninstall itself
                let mut context = script.context.lock();
                let downcast_context = match context.downcast_mut::<P::C>() {
                    Some(downcast_context) => downcast_context,
                    None => {
                        bevy::log::error!(
                            "{}: Tried to delete script context for id: {}, but the context is of the wrong type for this script plugin.",
                            P::LANGUAGE,
                            self.id
                        );
                        return;
                    }
                };

                match (CallbackSettings::<P>::call)(
                    handler_ctxt.callback_settings.callback_handler,
                    vec![],
                    bevy::ecs::entity::Entity::from_raw(0),
                    &self.id,
                    &OnScriptUnloaded::into_callback_label(),
                    downcast_context,
                    &handler_ctxt
                        .context_loading_settings
                        .context_pre_handling_initializers,
                    &handler_ctxt.runtime_container.runtime,
                    guard.clone(),
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        handle_script_errors(
                            guard,
                            [e.with_context(format!(
                                "Running unload hook for script with id: {}. Language: {}",
                                self.id,
                                P::LANGUAGE
                            ))]
                            .into_iter(),
                        );
                    }
                }

                debug!("Removing script with id: {}", self.id);
                // since we removed the script and are dropping the context,
                // it's going to get de-allocated if it's the last context irrespective if we're
                // using a global or individual allocation strategy
            } else {
                bevy::log::error!(
                    "Attempted to delete script with id: {} but it does not exist, doing nothing!",
                    self.id
                );
            }
        })
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
        let downcast_context = match context.downcast_mut::<P::C>() {
            Some(downcast_context) => downcast_context,
            None => {
                return Err(InteropError::unsupported_operation(None,None,
                    "Tried to reload script but the context loaded on it is of the wrong type for this script plugin."
                ).into());
            }
        };

        (ContextBuilder::<P>::reload)(
            handler_ctxt.context_loading_settings.loader.reload,
            &self.id,
            &self.content,
            downcast_context,
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
}

impl<P: IntoScriptPluginParams> Command for CreateOrUpdateScript<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
        with_handler_system_state(world, |guard, handler_ctxt: &mut HandlerContext<P>| {
            let is_new_script = !handler_ctxt.scripts.scripts.contains_key(&self.id);

            let assigned_shared_context =
                match handler_ctxt.context_loading_settings.assignment_strategy {
                    crate::context::ContextAssignmentStrategy::Individual => None,
                    crate::context::ContextAssignmentStrategy::Global => {
                        let is_new_context = handler_ctxt.scripts.scripts.is_empty();
                        if !is_new_context {
                            handler_ctxt
                                .scripts
                                .scripts
                                .values()
                                .next()
                                .map(|s| s.context.clone())
                        } else {
                            None
                        }
                    }
                };

            debug!(
                "{}: CreateOrUpdateScript command applying (script_id: {}, new context?: {}, new script?: {})",
                P::LANGUAGE,
                self.id,
                assigned_shared_context.is_none(),
                is_new_script
            );

            let result = match &assigned_shared_context {
                Some(assigned_shared_context) => {
                    if is_new_script {
                        // this will happen when sharing contexts
                        // make a new script with the shared context
                        let script = Script {
                            id: self.id.clone(),
                            asset: self.asset.clone(),
                            context: assigned_shared_context.clone(),
                        };
                        // it can potentially be loaded but without a successful script reload but that
                        // leaves us in an okay state
                        handler_ctxt.scripts.scripts.insert(self.id.clone(), script);
                    }
                    bevy::log::debug!("{}: reloading script with id: {}", P::LANGUAGE, self.id);
                    self.reload_context(guard.clone(), handler_ctxt)
                }
                None => self.load_context(guard.clone(), handler_ctxt),
            };

            let result = result.and_then(|()| {
                handler_ctxt.call::<OnScriptLoaded>(
                    &self.id,
                    Entity::from_raw(0),
                    vec![],
                    guard.clone(),
                )?;
                Ok(())
            });

            match result {
                Ok(_) => {}
                Err(e) => {
                    let phrase = if assigned_shared_context.is_some() {
                        "reloading"
                    } else {
                        "loading"
                    };
                    handle_script_errors(
                        guard,
                        vec![e
                            .with_script(self.id)
                            .with_context(format!("{}: {phrase} script", P::LANGUAGE))]
                        .into_iter(),
                    );
                }
            }
        })
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
                    *existing = String::from_utf8_lossy(new).into();
                    for init in init {
                        init(name, existing)?;
                    }
                    for init in pre_run_init {
                        init(name, Entity::from_raw(0), existing)?;
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
                c.push_str(format!(" callback-ran-{}", callback).as_str());
                Ok(ScriptValue::Unit)
            },
        })
        .insert_resource(Scripts {
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
        let scripts = world.get_resource::<Scripts>().unwrap();

        let script = scripts
            .scripts
            .get(id)
            .unwrap_or_else(|| panic!("Script not found {message}"));

        assert_eq!(id, script.id);
        let found_context = script.context.lock();
        let found_context = &found_context.downcast_ref::<String>().unwrap().to_string();

        assert_eq!(&context, &found_context, "{}", message);
    }

    #[test]
    fn test_commands_with_default_assigner() {
        let mut app = setup_app();

        let world = app.world_mut();
        let content = "content".as_bytes().to_vec().into_boxed_slice();
        let command = CreateOrUpdateScript::<DummyPlugin>::new("script".into(), content, None);
        command.apply(world);

        // check script
        assert_context_and_script(
            world,
            "script",
            "content initialized pre-handling-initialized callback-ran-on_script_loaded",
            "Initial script creation failed",
        );

        // update the script
        let content = "new content".as_bytes().to_vec().into_boxed_slice();
        let command = CreateOrUpdateScript::<DummyPlugin>::new("script".into(), content, None);
        command.apply(world);

        // check script
        assert_context_and_script(
            world,
            "script",
            "new content initialized pre-handling-initialized callback-ran-on_script_loaded",
            "Script update failed",
        );

        // create second script
        let content = "content2".as_bytes().to_vec().into_boxed_slice();
        let command = CreateOrUpdateScript::<DummyPlugin>::new("script2".into(), content, None);

        command.apply(world);

        // check second script

        assert_context_and_script(
            world,
            "script2",
            "content2 initialized pre-handling-initialized callback-ran-on_script_loaded",
            "Second script creation failed",
        );

        // delete both scripts
        let command = DeleteScript::<DummyPlugin>::new("script".into());
        command.apply(world);
        let command = DeleteScript::<DummyPlugin>::new("script2".into());
        command.apply(world);

        // check that the scripts are gone
        let scripts = world.get_resource::<Scripts>().unwrap();
        assert!(scripts.scripts.is_empty());
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
        assert_context_and_script(
            app.world(),
            "script",
            "content initialized pre-handling-initialized callback-ran-on_script_loaded",
            "Initial script creation failed",
        );

        // update the script

        let content = "new content".as_bytes().to_vec().into_boxed_slice();
        let command = CreateOrUpdateScript::<DummyPlugin>::new("script".into(), content, None);

        command.apply(app.world_mut());

        // check script

        assert_context_and_script(
            app.world(),
            "script",
            "new content initialized pre-handling-initialized callback-ran-on_script_loaded",
            "Script update failed",
        );

        // create second script

        let content = "content2".as_bytes().to_vec().into_boxed_slice();
        let command = CreateOrUpdateScript::<DummyPlugin>::new("script2".into(), content, None);

        command.apply(app.world_mut());

        // check both scripts have the new context

        assert_context_and_script(
            app.world(),
            "script2",
            "content2 initialized pre-handling-initialized callback-ran-on_script_loaded",
            "second script context was not created correctly",
        );
        assert_context_and_script(
            app.world(),
            "script",
            "content2 initialized pre-handling-initialized callback-ran-on_script_loaded",
            "First script context was not updated on second script insert",
        );

        let scripts = app.world().get_resource::<Scripts>().unwrap();
        assert!(scripts.scripts.len() == 2);

        // delete first script
        let command = DeleteScript::<DummyPlugin>::new("script".into());

        command.apply(app.world_mut());

        // check second script still has the context, and on unload was called
        assert_context_and_script(
            app.world(),
            "script2",
            "content2 initialized pre-handling-initialized callback-ran-on_script_loaded callback-ran-on_script_unloaded",
            "first script unload didn't have the desired effect",
        );

        // delete second script

        let command = DeleteScript::<DummyPlugin>::new("script2".into());

        command.apply(app.world_mut());

        // check that the scripts are gone, and so is the context

        let scripts = app.world().get_resource::<Scripts>().unwrap();
        assert!(scripts.scripts.is_empty());

        let scripts = app.world().get_resource::<Scripts>().unwrap();

        assert_eq!(scripts.scripts.len(), 0, "scripts weren't removed");
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
