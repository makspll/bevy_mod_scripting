use crate::{
    asset::ScriptAsset,
    context::ContextBuilder,
    event::{IntoCallbackLabel, OnScriptLoaded, OnScriptUnloaded},
    extractors::{extract_handler_context, yield_handler_context, HandlerContext},
    handler::{handle_script_errors, CallbackSettings},
    script::{Script, ScriptId},
    IntoScriptPluginParams,
};
use bevy::{asset::Handle, log::debug, prelude::Command};
use std::marker::PhantomData;

pub struct DeleteScript<P: IntoScriptPluginParams> {
    pub id: ScriptId,
    // hack to make this Send, C does not need to be Send since it is not stored in the command
    pub _ph: PhantomData<fn(P::C, P::R)>,
}

impl<P: IntoScriptPluginParams> DeleteScript<P> {
    pub fn new(id: ScriptId) -> Self {
        Self {
            id,
            _ph: PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> Command for DeleteScript<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
        let mut res_ctxt = match extract_handler_context::<P>(world) {
            Ok(res_ctxt) => res_ctxt,
            Err(e) => {
                bevy::log::error_once!(
                    "Could not delete script: {}, as some plugin resources are missing: {}",
                    self.id,
                    e
                );
                return;
            }
        };

        if let Some(script) = res_ctxt.scripts.scripts.remove(&self.id) {
            debug!("Deleting script with id: {}", self.id);

            match res_ctxt.script_contexts.get_mut(script.context_id) {
                Some(context) => {
                    // first let the script uninstall itself
                    match (CallbackSettings::<P>::call)(
                        res_ctxt.callback_settings.callback_handler,
                        vec![],
                        bevy::ecs::entity::Entity::from_raw(0),
                        &self.id,
                        &OnScriptUnloaded::into_callback_label(),
                        context,
                        &res_ctxt
                            .context_loading_settings
                            .context_pre_handling_initializers,
                        &mut res_ctxt.runtime_container.runtime,
                        world,
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            handle_script_errors(
                                world,
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
                    (res_ctxt.context_loading_settings.assigner.remove)(
                        script.context_id,
                        &script,
                        &mut res_ctxt.script_contexts,
                    )
                }
                None => {
                    bevy::log::error!(
                            "Could not find context with id: {} corresponding to script with id: {}. Removing script without running callbacks.",
                            script.context_id,
                            self.id
                        );
                }
            };
        } else {
            bevy::log::error!(
                "Attempted to delete script with id: {} but it does not exist, doing nothing!",
                self.id
            );
        }

        yield_handler_context(world, res_ctxt);
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
    pub fn new(id: ScriptId, content: Box<[u8]>, asset: Option<Handle<ScriptAsset>>) -> Self {
        Self {
            id,
            content,
            asset,
            _ph: std::marker::PhantomData,
        }
    }

    fn run_on_load_callback(
        &self,
        res_ctxt: &mut HandlerContext<P>,
        world: &mut bevy::prelude::World,
        ctxt: &mut <P as IntoScriptPluginParams>::C,
    ) {
        bevy::log::debug!(
            "{}: Running on load callback for script with id: {}",
            P::LANGUAGE,
            self.id
        );

        match (CallbackSettings::<P>::call)(
            res_ctxt.callback_settings.callback_handler,
            vec![],
            bevy::ecs::entity::Entity::from_raw(0),
            &self.id,
            &OnScriptLoaded::into_callback_label(),
            ctxt,
            &res_ctxt
                .context_loading_settings
                .context_pre_handling_initializers,
            &mut res_ctxt.runtime_container.runtime,
            world,
        ) {
            Ok(_) => {}
            Err(e) => {
                handle_script_errors(
                    world,
                    [e.with_context(format!(
                        "{}: Running initialization hook for script with id: {}",
                        P::LANGUAGE,
                        self.id
                    ))]
                    .into_iter(),
                );
            }
        }
    }

    #[inline(always)]
    fn reload_context(
        &self,
        world: &mut bevy::prelude::World,
        res_ctxt: &mut HandlerContext<P>,
        previous_context_id: u32,
    ) {
        if let Some(mut previous_context) = res_ctxt.script_contexts.remove(previous_context_id) {
            match (ContextBuilder::<P>::reload)(
                res_ctxt.context_loading_settings.loader.reload,
                &self.id,
                &self.content,
                &mut previous_context,
                &res_ctxt.context_loading_settings.context_initializers,
                &res_ctxt
                    .context_loading_settings
                    .context_pre_handling_initializers,
                world,
                &mut res_ctxt.runtime_container.runtime,
            ) {
                Ok(_) => {}
                Err(e) => {
                    handle_script_errors(
                        world,
                        [e.with_context(format!("reloading script with id: {}", self.id))]
                            .into_iter(),
                    );
                }
            };

            self.run_on_load_callback(res_ctxt, world, &mut previous_context);

            res_ctxt
                .script_contexts
                .insert_with_id(previous_context_id, previous_context);
        } else {
            bevy::log::error!(
                "{}: Could not reload script with id: {}, as the context with id: {} does not exist.",
                P::LANGUAGE,
                self.id,
                previous_context_id
            );
        }
    }

    #[inline(always)]
    fn execute(
        self,
        world: &mut bevy::prelude::World,
        res_ctxt: &mut HandlerContext<P>,
        previous_context_id: Option<u32>,
    ) {
        match previous_context_id {
            Some(previous_context_id) => {
                bevy::log::debug!(
                    "{}: script with id already has a context: {}",
                    P::LANGUAGE,
                    self.id
                );
                self.reload_context(world, res_ctxt, previous_context_id);
            }
            None => {
                let log_context = format!("{}: Loading script: {}", P::LANGUAGE, self.id);

                let new_context_id = (res_ctxt.context_loading_settings.assigner.assign)(
                    &self.id,
                    &self.content,
                    &res_ctxt.script_contexts,
                )
                .unwrap_or_else(|| res_ctxt.script_contexts.allocate_id());
                if res_ctxt.script_contexts.contains(new_context_id) {
                    self.reload_context(world, res_ctxt, new_context_id);
                } else {
                    // load new context
                    bevy::log::debug!("{}", log_context);
                    let ctxt = (ContextBuilder::<P>::load)(
                        res_ctxt.context_loading_settings.loader.load,
                        &self.id,
                        &self.content,
                        &res_ctxt.context_loading_settings.context_initializers,
                        &res_ctxt
                            .context_loading_settings
                            .context_pre_handling_initializers,
                        world,
                        &mut res_ctxt.runtime_container.runtime,
                    );
                    let mut ctxt = match ctxt {
                        Ok(ctxt) => ctxt,
                        Err(e) => {
                            handle_script_errors(world, [e.with_context(log_context)].into_iter());
                            return;
                        }
                    };

                    self.run_on_load_callback(res_ctxt, world, &mut ctxt);

                    if res_ctxt
                        .script_contexts
                        .insert_with_id(new_context_id, ctxt)
                        .is_some()
                    {
                        bevy::log::warn!("{}: Context with id {} was not expected to exist. Overwriting it with a new context. This might happen if a script is not completely removed.", P::LANGUAGE, new_context_id);
                    }
                }

                res_ctxt.scripts.scripts.insert(
                    self.id.clone(),
                    Script {
                        id: self.id,
                        asset: self.asset,
                        context_id: new_context_id,
                    },
                );
            }
        }
    }
}

impl<P: IntoScriptPluginParams> Command for CreateOrUpdateScript<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
        let mut res_ctxt = match extract_handler_context::<P>(world) {
            Ok(res_ctxt) => res_ctxt,
            Err(e) => {
                bevy::log::error_once!(
                    "Could not create or update script: {}, as some plugin resources are missing: {}",
                    self.id,
                    e
                );
                return;
            }
        };

        let script = res_ctxt.scripts.scripts.get(&self.id);
        let previous_context_id = script.as_ref().map(|s| s.context_id);
        debug!(
            "{}: CreateOrUpdateScript command applying (script_id: {}, previous_context_id: {:?})",
            P::LANGUAGE,
            self.id,
            previous_context_id
        );

        // closure to prevent early returns from yielding the context
        self.execute(world, &mut res_ctxt, previous_context_id);

        yield_handler_context(world, res_ctxt);
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
        context::{ContextAssigner, ContextBuilder, ContextLoadingSettings, ScriptContexts},
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
            assigner: Default::default(),
            context_initializers: vec![|_, c| {
                c.push_str(" initialized");
                Ok(())
            }],
            context_pre_handling_initializers: vec![|_, _, c| {
                c.push_str(" pre-handling-initialized");
                Ok(())
            }],
        })
        .insert_non_send_resource(ScriptContexts::<DummyPlugin> {
            contexts: Default::default(),
        })
        .insert_non_send_resource(RuntimeContainer::<DummyPlugin> {
            runtime: "Runtime".to_string(),
        })
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

    fn assert_context_and_script(world: &World, id: &str, context: &str) {
        let contexts = world
            .get_non_send_resource::<ScriptContexts<DummyPlugin>>()
            .unwrap();
        let scripts = world.get_resource::<Scripts>().unwrap();

        let script = scripts.scripts.get(id).expect("Script not found");

        assert_eq!(id, script.id);
        let found_context = contexts
            .contexts
            .get(&script.context_id)
            .expect("Context not found");

        assert_eq!(found_context, context);
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
        );

        // delete both scripts
        let command = DeleteScript::<DummyPlugin>::new("script".into());
        command.apply(world);
        let command = DeleteScript::<DummyPlugin>::new("script2".into());
        command.apply(world);

        // check that the scripts are gone
        let scripts = world.get_resource::<Scripts>().unwrap();
        assert!(scripts.scripts.is_empty());

        let contexts = world
            .get_non_send_resource::<ScriptContexts<DummyPlugin>>()
            .unwrap();
        assert!(contexts.contexts.is_empty());
    }

    #[test]
    fn test_commands_with_global_assigner() {
        // setup all the resources necessary
        let mut app = setup_app();

        let mut settings = app
            .world_mut()
            .get_resource_mut::<ContextLoadingSettings<DummyPlugin>>()
            .unwrap();

        settings.assigner = ContextAssigner::new_global_context_assigner();

        // create a script
        let content = "content".as_bytes().to_vec().into_boxed_slice();
        let command = CreateOrUpdateScript::<DummyPlugin>::new("script".into(), content, None);

        command.apply(app.world_mut());

        // check script
        assert_context_and_script(
            app.world(),
            "script",
            "content initialized pre-handling-initialized callback-ran-on_script_loaded",
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
        );

        // create second script

        let content = "content2".as_bytes().to_vec().into_boxed_slice();
        let command = CreateOrUpdateScript::<DummyPlugin>::new("script2".into(), content, None);

        command.apply(app.world_mut());

        // check both scripts have the new context

        assert_context_and_script(
            app.world(),
            "script",
            "content2 initialized pre-handling-initialized callback-ran-on_script_loaded",
        );

        assert_context_and_script(
            app.world(),
            "script2",
            "content2 initialized pre-handling-initialized callback-ran-on_script_loaded",
        );

        // check one context exists only
        let context = app
            .world()
            .get_non_send_resource::<ScriptContexts<DummyPlugin>>()
            .unwrap();
        assert!(context.contexts.len() == 1);

        // delete first script
        let command = DeleteScript::<DummyPlugin>::new("script".into());

        command.apply(app.world_mut());

        // check second script still has the context, and on unload was called
        assert_context_and_script(
            app.world(),
            "script2",
            "content2 initialized pre-handling-initialized callback-ran-on_script_loaded callback-ran-on_script_unloaded",
        );

        // delete second script

        let command = DeleteScript::<DummyPlugin>::new("script2".into());

        command.apply(app.world_mut());

        // check that the scripts are gone, but context is still there

        let scripts = app.world().get_resource::<Scripts>().unwrap();
        assert!(scripts.scripts.is_empty());

        let contexts = app
            .world()
            .get_non_send_resource::<ScriptContexts<DummyPlugin>>()
            .unwrap();

        assert!(contexts.contexts.len() == 1);
    }
}
