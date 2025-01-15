use crate::{
    asset::ScriptAsset,
    context::{ContextLoadingSettings, ScriptContexts},
    event::{IntoCallbackLabel, OnScriptLoaded, OnScriptUnloaded},
    handler::{handle_script_errors, CallbackSettings, HandlerFn},
    runtime::RuntimeContainer,
    script::{Script, ScriptId, Scripts},
    IntoScriptPluginParams,
};
use bevy::{asset::Handle, ecs::world::Mut, log::debug, prelude::Command};
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
        let settings = world
            .get_resource::<ContextLoadingSettings<P>>()
            .expect("No ScriptLoadingSettings resource found")
            .clone();

        let runner = world
            .get_resource::<CallbackSettings<P>>()
            .expect("No CallbackSettings resource found")
            .callback_handler;

        let mut ctxts = world
            .remove_non_send_resource::<ScriptContexts<P>>()
            .unwrap();

        let mut runtime_container = world
            .remove_non_send_resource::<RuntimeContainer<P>>()
            .unwrap();

        world.resource_scope(|world, mut scripts: Mut<Scripts>| {
            if let Some(script) = scripts.scripts.remove(&self.id) {
                debug!("Deleting script with id: {}", self.id);

                // first let the script uninstall itself
                match (runner)(
                    vec![],
                    bevy::ecs::entity::Entity::from_raw(0),
                    &self.id,
                    &OnScriptUnloaded::into_callback_label(),
                    ctxts.get_mut(script.context_id).unwrap(),
                    &settings.context_pre_handling_initializers,
                    &mut runtime_container.runtime,
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
                (settings.assigner.remove)(script.context_id, &script, &mut ctxts)
            } else {
                bevy::log::error!(
                    "Attempted to delete script with id: {} but it does not exist, doing nothing!",
                    self.id
                );
            }
        });

        world.insert_resource(settings);
        world.insert_non_send_resource(ctxts);
        world.insert_non_send_resource(runtime_container);
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
        settings: &ContextLoadingSettings<P>,
        runtime: &mut RuntimeContainer<P>,
        runner: HandlerFn<P>,
        world: &mut bevy::prelude::World,
        ctxt: &mut <P as IntoScriptPluginParams>::C,
    ) {
        match (runner)(
            vec![],
            bevy::ecs::entity::Entity::from_raw(0),
            &self.id,
            &OnScriptLoaded::into_callback_label(),
            ctxt,
            &settings.context_pre_handling_initializers,
            &mut runtime.runtime,
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

    fn reload_context(
        &self,
        world: &mut bevy::prelude::World,
        settings: &ContextLoadingSettings<P>,
        runtime: &mut RuntimeContainer<P>,
        builder: &crate::context::ContextBuilder<P>,
        log_context: String,
        previous_context: &mut <P as IntoScriptPluginParams>::C,
    ) -> bool {
        match (builder.reload)(
            &self.id,
            &self.content,
            previous_context,
            &settings.context_initializers,
            &settings.context_pre_handling_initializers,
            world,
            &mut runtime.runtime,
        ) {
            Ok(_) => {}
            Err(e) => {
                handle_script_errors(world, [e.with_context(log_context)].into_iter());
                return false;
            }
        };
        true
    }

    #[inline(always)]
    fn execute(
        self,
        world: &mut bevy::prelude::World,
        settings: &ContextLoadingSettings<P>,
        contexts: &mut ScriptContexts<P>,
        runtime: &mut RuntimeContainer<P>,
        scripts: &mut Scripts,
        assigner: crate::context::ContextAssigner<P>,
        builder: crate::context::ContextBuilder<P>,
        runner: HandlerFn<P>,
        previous_context_id: Option<u32>,
    ) {
        match previous_context_id {
            Some(previous_context_id) => {
                if let Some(previous_context) = contexts.get_mut(previous_context_id) {
                    let log_context = format!("{}: Reloading script: {}.", P::LANGUAGE, self.id);
                    bevy::log::info!("{}", log_context);
                    if !self.reload_context(
                        world,
                        settings,
                        runtime,
                        &builder,
                        log_context,
                        previous_context,
                    ) {
                        return;
                    }
                    self.run_on_load_callback(settings, runtime, runner, world, previous_context);
                } else {
                    bevy::log::error!("{}: Could not find previous context with id: {}. Could not reload script: {}. Someone deleted the context.", P::LANGUAGE, previous_context_id, self.id);
                }
            }
            None => {
                let log_context = format!("{}: Loading script: {}", P::LANGUAGE, self.id);

                let new_context_id = (assigner.assign)(&self.id, &self.content, contexts)
                    .unwrap_or_else(|| contexts.allocate_id());
                if let Some(existing_context) = contexts.get_mut(new_context_id) {
                    // this can happen if we're sharing contexts between scripts
                    if !self.reload_context(
                        world,
                        settings,
                        runtime,
                        &builder,
                        log_context,
                        existing_context,
                    ) {
                        return;
                    }

                    self.run_on_load_callback(settings, runtime, runner, world, existing_context);
                } else {
                    // load new context
                    bevy::log::info!("{}", log_context);
                    let ctxt = (builder.load)(
                        &self.id,
                        &self.content,
                        &settings.context_initializers,
                        &settings.context_pre_handling_initializers,
                        world,
                        &mut runtime.runtime,
                    );
                    let mut ctxt = match ctxt {
                        Ok(ctxt) => ctxt,
                        Err(e) => {
                            handle_script_errors(world, [e.with_context(log_context)].into_iter());
                            return;
                        }
                    };

                    self.run_on_load_callback(settings, runtime, runner, world, &mut ctxt);

                    if contexts.insert_with_id(new_context_id, ctxt).is_some() {
                        bevy::log::warn!("{}: Context with id {} was not expected to exist. Overwriting it with a new context. This might happen if a script is not completely removed.", P::LANGUAGE, new_context_id);
                    }
                }

                scripts.scripts.insert(
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
        let settings = world
            .get_resource::<ContextLoadingSettings<P>>()
            .expect(
                "Missing ContextLoadingSettings resource. Was the plugin initialized correctly?",
            )
            .clone();
        let mut contexts = world
            .remove_non_send_resource::<ScriptContexts<P>>()
            .expect("No ScriptContexts resource found. Was the plugin initialized correctly?");
        let mut runtime = world
            .remove_non_send_resource::<RuntimeContainer<P>>()
            .expect("No RuntimeContainer resource found. Was the plugin initialized correctly?");
        let mut scripts = world
            .remove_resource::<Scripts>()
            .expect("No Scripts resource found. Was the plugin initialized correctly?");

        let runner = world.get_resource::<CallbackSettings<P>>().unwrap();
        // assign context
        let assigner = settings.assigner.clone();
        let builder = settings.loader.clone();
        let runner = runner.callback_handler;

        let script = scripts.scripts.get(&self.id);
        let previous_context_id = script.as_ref().map(|s| s.context_id);
        debug!(
            "{}: CreateOrUpdateScript command applying (script_id: {}, previous_context_id: {:?})",
            P::LANGUAGE,
            self.id,
            previous_context_id
        );

        // closure to prevent returns from re-inserting resources
        self.execute(
            world,
            &settings,
            &mut contexts,
            &mut runtime,
            &mut scripts,
            assigner,
            builder,
            runner,
            previous_context_id,
        );

        world.insert_resource(scripts);
        world.insert_resource(settings);
        world.insert_non_send_resource(runtime);
        world.insert_non_send_resource(contexts);
    }
}

#[cfg(test)]
mod test {
    use bevy::{
        app::App,
        prelude::{Entity, World},
    };

    use crate::{
        asset::Language,
        bindings::script_value::ScriptValue,
        context::{ContextAssigner, ContextBuilder},
    };

    use super::*;

    fn setup_app() -> App {
        // setup all the resources necessary
        let mut app = App::new();

        app.insert_resource(ContextLoadingSettings::<DummyPlugin> {
            loader: ContextBuilder {
                load: |name, c, init, pre_run_init, _, _| {
                    let mut context = String::from_utf8_lossy(c).into();
                    for init in init {
                        init(name, &mut context)?;
                    }
                    for init in pre_run_init {
                        init(name, Entity::from_raw(0), &mut context)?;
                    }
                    Ok(context)
                },
                reload: |name, new, existing, init, pre_run_init, _, _| {
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
            callback_handler: |_, _, _, callback, c, _, _, _| {
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
