use bevy::{ecs::system::SystemState, prelude::*};
use std::any::type_name;

use crate::{
    asset::{ScriptAsset, ScriptAssetSettings},
    bindings::ReflectAllocator,
    commands::{CreateOrUpdateScript, DeleteScript},
    context::{Context, ContextLoadingSettings, ScriptContexts},
    error::{ScriptError, ScriptResult},
    event::{IntoCallbackLabel, ScriptCallbackEvent, ScriptErrorEvent},
    handler::{Args, CallbackSettings},
    prelude::RuntimeSettings,
    runtime::{Runtime, RuntimeContainer},
    script::{ScriptComponent, Scripts},
};

/// Cleans up dangling script allocations
pub fn garbage_collector(mut allocator: ResMut<ReflectAllocator>) {
    allocator.clean_garbage_allocations()
}

pub fn initialize_runtime<R: Runtime>(
    mut runtime: NonSendMut<RuntimeContainer<R>>,
    settings: Res<RuntimeSettings<R>>,
) {
    for initializer in settings.initializers.iter() {
        (initializer)(&mut runtime.runtime);
    }
}

/// Processes and reacts appropriately to script asset events, and queues commands to update the internal script state
pub fn sync_script_data<C: Context, R: Runtime>(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    script_assets: Res<Assets<ScriptAsset>>,
    asset_settings: Res<ScriptAssetSettings>,
    mut commands: Commands,
) {
    for event in events.read() {
        debug!("Responding to script asset event: {:?}", event);
        let (id, remove) = match event {
            // emitted when a new script asset is loaded for the first time
            AssetEvent::Added { id } => (id, false),
            AssetEvent::Modified { id } => (id, false),
            AssetEvent::Removed { id } | AssetEvent::Unused { id } => (id, true),
            _ => continue,
        };
        // get the path
        let asset = script_assets.get(*id);
        let asset = asset.as_ref().expect("Asset was expected to be loaded!");

        let path = &asset.asset_path;
        // convert it to script id
        let converter = asset_settings.script_id_mapper.map;
        let script_id = converter(path);

        if !remove {
            commands.queue(CreateOrUpdateScript::<C, R>::new(
                script_id,
                asset.content.clone(),
                Some(script_assets.reserve_handle()),
            ));
        } else {
            commands.queue(DeleteScript::<C, R>::new(script_id));
        }
    }
}

macro_rules! push_err_and_continue {
    ($errors:ident, $expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                $errors.push(e);
                continue;
            }
        }
    };
}

/// Passes events with the specified label to the script callback with the same name and runs the callback
pub fn event_handler<L: IntoCallbackLabel, A: Args, C: Context, R: Runtime>(
    world: &mut World,
    params: &mut SystemState<(
        EventReader<ScriptCallbackEvent<A>>,
        Res<CallbackSettings<A, C, R>>,
        Res<ContextLoadingSettings<C, R>>,
        Res<Scripts>,
        Query<(Entity, Ref<ScriptComponent>)>,
    )>,
) {
    debug!("Handling events with label `{}`", L::into_callback_label());

    let mut runtime_container = world
        .remove_non_send_resource::<RuntimeContainer<R>>()
        .unwrap_or_else(|| {
            panic!(
                "No runtime container for runtime {} found. Was the scripting plugin initialized correctly?",
                type_name::<R>()
            )
        });
    let runtime = &mut runtime_container.runtime;
    let mut script_contexts = world
        .remove_non_send_resource::<ScriptContexts<C>>()
        .unwrap_or_else(|| panic!("No script contexts found for context {}", type_name::<C>()));

    let (mut script_events, callback_settings, context_settings, scripts, entities) =
        params.get_mut(world);

    let handler = *callback_settings
        .callback_handler
        .as_ref()
        .unwrap_or_else(|| {
            panic!(
                "No handler registered for - Runtime: {}, Context: {}, Args: {}",
                type_name::<R>(),
                type_name::<C>(),
                type_name::<A>()
            )
        });
    let pre_handling_initializers = context_settings.context_pre_handling_initializers.clone();
    let scripts = scripts.clone();
    let mut errors = Vec::default();

    let events = script_events.read().cloned().collect::<Vec<_>>();
    let entity_scripts = entities
        .iter()
        .map(|(e, s)| (e, s.0.clone()))
        .collect::<Vec<_>>();

    for event in events
        .into_iter()
        .filter(|e| e.label == L::into_callback_label())
    {
        for (entity, entity_scripts) in entity_scripts.iter() {
            for script_id in entity_scripts.iter() {
                match &event.recipients {
                    crate::event::Recipients::Script(target_script_id)
                        if target_script_id != script_id =>
                    {
                        continue
                    }
                    crate::event::Recipients::Entity(target_entity) if target_entity != entity => {
                        continue
                    }
                    _ => (),
                }
                debug!(
                    "Handling event for script {} on entity {:?}",
                    script_id, entity
                );
                let script = match scripts.scripts.get(script_id) {
                    Some(s) => s,
                    None => {
                        info!(
                            "Script `{}` on entity `{:?}` is either still loading or doesn't exist, ignoring.",
                            script_id, entity
                        );
                        continue;
                    }
                };
                let ctxt = script_contexts
                    .contexts
                    .get_mut(&script.context_id)
                    .unwrap();

                let handler_result = (handler)(
                    event.args.clone(),
                    *entity,
                    &script.id,
                    &L::into_callback_label(),
                    ctxt,
                    &pre_handling_initializers,
                    runtime,
                    world,
                );

                push_err_and_continue!(errors, handler_result)
            }
        }
    }

    world.insert_non_send_resource(runtime_container);
    world.insert_non_send_resource(script_contexts);

    handle_script_errors(
        world,
        &format!(
            "Encountered error in event handling for: Runtime {}, Context: {}, Args: {}",
            type_name::<R>(),
            type_name::<C>(),
            type_name::<A>()
        ),
        errors,
    );
}

/// Handles errors caused by script execution and sends them to the error event channel
pub(crate) fn handle_script_errors<I: IntoIterator<Item = ScriptError>>(
    world: &mut World,
    context: &str,
    errors: I,
) {
    let mut error_events = world
        .get_resource_mut::<Events<ScriptErrorEvent>>()
        .expect("Missing events resource");

    for error in errors {
        bevy::log::error!("{}. {}", context, error.to_string());
        error_events.send(ScriptErrorEvent { error });
    }
}

#[cfg(test)]
mod test {
    use std::{borrow::Cow, collections::HashMap};

    use crate::{
        event::CallbackLabel,
        handler::HandlerFn,
        script::{Script, ScriptId},
    };

    use super::*;
    struct OnTestCallback;

    impl IntoCallbackLabel for OnTestCallback {
        fn into_callback_label() -> CallbackLabel {
            "OnTest".into()
        }
    }

    struct TestRuntime {
        pub invocations: Vec<(Entity, ScriptId)>,
    }

    struct TestContext {
        pub invocations: Vec<String>,
    }

    fn setup_app<L: IntoCallbackLabel + 'static, A: Args, C: Context, R: Runtime>(
        handler_fn: HandlerFn<A, C, R>,
        runtime: R,
        contexts: HashMap<u32, C>,
        scripts: HashMap<ScriptId, Script>,
    ) -> App {
        let mut app = App::new();

        app.add_event::<ScriptCallbackEvent<A>>();
        app.add_event::<ScriptErrorEvent>();
        app.insert_resource::<CallbackSettings<A, C, R>>(CallbackSettings {
            callback_handler: Some(handler_fn),
        });
        app.add_systems(Update, event_handler::<L, A, C, R>);
        app.insert_resource::<Scripts>(Scripts { scripts });
        app.insert_non_send_resource::<RuntimeContainer<R>>(RuntimeContainer { runtime });
        app.insert_non_send_resource::<ScriptContexts<C>>(ScriptContexts { contexts });
        app.insert_resource(ContextLoadingSettings::<C, R> {
            loader: None,
            assigner: None,
            context_initializers: vec![],
            context_pre_handling_initializers: vec![],
        });
        app.finish();
        app.cleanup();
        app
    }

    #[test]
    fn test_handler_called_with_right_args() {
        let test_script_id = Cow::Borrowed("test_script");
        let test_ctxt_id = 0;
        let test_script = Script {
            id: test_script_id.clone(),
            asset: None,
            context_id: test_ctxt_id,
        };
        let scripts = HashMap::from_iter(vec![(test_script_id.clone(), test_script.clone())]);
        let contexts = HashMap::from_iter(vec![(
            test_ctxt_id,
            TestContext {
                invocations: vec![],
            },
        )]);
        let runtime = TestRuntime {
            invocations: vec![],
        };
        let mut app = setup_app::<OnTestCallback, String, TestContext, TestRuntime>(
            |args, entity, script, _, ctxt, _, runtime, _| {
                ctxt.invocations.push(args);
                runtime.invocations.push((entity, script.clone()));
                Ok(())
            },
            runtime,
            contexts,
            scripts,
        );
        let test_entity_id = app
            .world_mut()
            .spawn(ScriptComponent(vec![test_script_id.clone()]))
            .id();

        app.world_mut()
            .send_event(ScriptCallbackEvent::<String>::new_for_all(
                OnTestCallback::into_callback_label(),
                "test_args".to_owned(),
            ));
        app.update();

        let test_context = app
            .world()
            .get_non_send_resource::<ScriptContexts<TestContext>>()
            .unwrap();
        let test_runtime = app
            .world()
            .get_non_send_resource::<RuntimeContainer<TestRuntime>>()
            .unwrap();

        assert_eq!(
            test_context
                .contexts
                .get(&test_ctxt_id)
                .unwrap()
                .invocations,
            vec!["test_args"]
        );

        assert_eq!(
            test_runtime
                .runtime
                .invocations
                .iter()
                .map(|(e, s)| (*e, s.clone()))
                .collect::<Vec<_>>(),
            vec![(test_entity_id, test_script_id.clone())]
        );
    }

    #[test]
    fn test_handler_called_on_right_recipients() {
        let test_script_id = Cow::Borrowed("test_script");
        let test_ctxt_id = 0;
        let test_script = Script {
            id: test_script_id.clone(),
            asset: None,
            context_id: test_ctxt_id,
        };
        let scripts = HashMap::from_iter(vec![
            (test_script_id.clone(), test_script.clone()),
            (
                "wrong".into(),
                Script {
                    id: "wrong".into(),
                    asset: None,
                    context_id: 1,
                },
            ),
        ]);
        let contexts = HashMap::from_iter(vec![
            (
                test_ctxt_id,
                TestContext {
                    invocations: vec![],
                },
            ),
            (
                1,
                TestContext {
                    invocations: vec![],
                },
            ),
        ]);
        let runtime = TestRuntime {
            invocations: vec![],
        };
        let mut app = setup_app::<OnTestCallback, String, TestContext, TestRuntime>(
            |args, entity, script, _, ctxt, _, runtime, _| {
                ctxt.invocations.push(args);
                runtime.invocations.push((entity, script.clone()));
                Ok(())
            },
            runtime,
            contexts,
            scripts,
        );
        let test_entity_id = app
            .world_mut()
            .spawn(ScriptComponent(vec![test_script_id.clone()]))
            .id();

        app.world_mut()
            .send_event(ScriptCallbackEvent::<String>::new(
                OnTestCallback::into_callback_label(),
                "test_args_script".to_owned(),
                crate::event::Recipients::Script(test_script_id.clone()),
            ));

        app.world_mut()
            .send_event(ScriptCallbackEvent::<String>::new(
                OnTestCallback::into_callback_label(),
                "test_args_entity".to_owned(),
                crate::event::Recipients::Entity(test_entity_id),
            ));

        app.update();

        let test_context = app
            .world()
            .get_non_send_resource::<ScriptContexts<TestContext>>()
            .unwrap();
        let test_runtime = app
            .world()
            .get_non_send_resource::<RuntimeContainer<TestRuntime>>()
            .unwrap();

        assert_eq!(
            test_context
                .contexts
                .get(&test_ctxt_id)
                .unwrap()
                .invocations,
            vec!["test_args_script", "test_args_entity"]
        );

        assert_eq!(
            test_runtime
                .runtime
                .invocations
                .iter()
                .map(|(e, s)| (*e, s.clone()))
                .collect::<Vec<_>>(),
            vec![
                (test_entity_id, test_script_id.clone()),
                (test_entity_id, test_script_id.clone())
            ]
        );
    }
}
