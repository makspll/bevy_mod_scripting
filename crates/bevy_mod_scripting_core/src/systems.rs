use crate::{
    asset::{ScriptAsset, ScriptAssetSettings, ScriptMetadata, ScriptMetadataStore},
    bindings::{pretty_print::DisplayWithWorld, AppReflectAllocator, WorldAccessGuard, WorldGuard},
    commands::{CreateOrUpdateScript, DeleteScript},
    context::{ContextLoadingSettings, ScriptContexts},
    error::ScriptError,
    event::{IntoCallbackLabel, ScriptCallbackEvent, ScriptErrorEvent},
    handler::CallbackSettings,
    runtime::{RuntimeContainer, RuntimeSettings},
    script::{ScriptComponent, Scripts},
    IntoScriptPluginParams,
};
use bevy::{ecs::system::SystemState, prelude::*};
use std::any::type_name;

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
/// Labels for various BMS systems
pub enum ScriptingSystemSet {
    // Post Setup processes
    RuntimeInitialization,

    // Post Update processes
    GarbageCollection,

    ScriptMetadataInsertion,
    ScriptCommandDispatch,
    ScriptMetadataRemoval,
}

/// Cleans up dangling script allocations
pub fn garbage_collector(allocator: ResMut<AppReflectAllocator>) {
    let mut allocator = allocator.write();
    allocator.clean_garbage_allocations()
}

pub fn initialize_runtime<P: IntoScriptPluginParams>(
    mut runtime: NonSendMut<RuntimeContainer<P>>,
    settings: Res<RuntimeSettings<P>>,
) {
    for initializer in settings.initializers.iter() {
        (initializer)(&mut runtime.runtime);
    }
}

/// Listens to `AssetEvent<ScriptAsset>::Added` events and populates the script metadata store
pub fn insert_script_metadata(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    script_assets: Res<Assets<ScriptAsset>>,
    mut asset_path_map: ResMut<ScriptMetadataStore>,
    settings: Res<ScriptAssetSettings>,
) {
    for event in events.read() {
        if let AssetEvent::Added { id } = event {
            let asset = script_assets.get(*id);
            if let Some(asset) = asset {
                let path = &asset.asset_path;
                let converter = settings.script_id_mapper.map;
                let script_id = converter(path);

                let language = settings.select_script_language(path);
                let metadata = ScriptMetadata {
                    script_id,
                    language,
                };
                info!("Populating script metadata for script: {:?}:", metadata);
                asset_path_map.insert(*id, metadata);
            } else {
                error!("A script was added but it's asset was not found, failed to compute metadata. This script will not be loaded. {}", id);
            }
        }
    }
}

/// Listens to [`AssetEvent<ScriptAsset>::Removed`] events and removes the corresponding script metadata
pub fn remove_script_metadata(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    mut asset_path_map: ResMut<ScriptMetadataStore>,
) {
    for event in events.read() {
        if let AssetEvent::Removed { id } = event {
            let previous = asset_path_map.remove(*id);
            if let Some(previous) = previous {
                info!("Removed script metadata for removed script: {:?}", previous);
            }
        }
    }
}

/// Listens to [`AssetEvent<ScriptAsset>`] events and dispatches [`CreateOrUpdateScript`] and [`DeleteScript`] commands accordingly.
///
/// Allows for hot-reloading of scripts.
pub fn sync_script_data<P: IntoScriptPluginParams>(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    script_assets: Res<Assets<ScriptAsset>>,
    script_metadata: Res<ScriptMetadataStore>,
    mut commands: Commands,
) {
    for event in events.read() {
        trace!("{}: Received script asset event: {:?}", P::LANGUAGE, event);
        match event {
            // emitted when a new script asset is loaded for the first time
            AssetEvent::Added { id } | AssetEvent::Modified { id } => {
                let metadata = match script_metadata.get(*id) {
                    Some(m) => m,
                    None => {
                        error!(
                            "{}: Script metadata not found for script asset with id: {}. Cannot load script.",
                            P::LANGUAGE,
                            id
                        );
                        continue;
                    }
                };

                if metadata.language != P::LANGUAGE {
                    trace!(
                        "{}: Script asset with id: {} is for a different langauge than this sync system. Skipping.",
                        P::LANGUAGE,
                        metadata.script_id
                    );
                    continue;
                }

                info!(
                    "{}: Dispatching Creation/Modification command for script: {:?}. Asset Id: {}",
                    P::LANGUAGE,
                    metadata,
                    id
                );

                if let Some(asset) = script_assets.get(*id) {
                    commands.queue(CreateOrUpdateScript::<P>::new(
                        metadata.script_id.clone(),
                        asset.content.clone(),
                        Some(script_assets.reserve_handle().clone_weak()),
                    ));
                }
            }
            AssetEvent::Removed { id } => {
                let metadata = match script_metadata.get(*id) {
                    Some(m) => m,
                    None => {
                        error!(
                            "{}: Script metadata not found for script asset with id: {}. Cannot delete script.",
                            P::LANGUAGE,
                            id
                        );
                        return;
                    }
                };

                info!(
                    "{}: Dispatching Deletion command for script: {:?}. Asset Id: {}",
                    P::LANGUAGE,
                    metadata,
                    id
                );
                commands.queue(DeleteScript::<P>::new(metadata.script_id.clone()));
            }
            _ => return,
        };
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
pub fn event_handler<L: IntoCallbackLabel, P: IntoScriptPluginParams>(
    world: &mut World,
    params: &mut SystemState<(
        EventReader<ScriptCallbackEvent>,
        Res<CallbackSettings<P>>,
        Res<ContextLoadingSettings<P>>,
        Res<Scripts>,
        Query<(Entity, Ref<ScriptComponent>)>,
    )>,
) {
    let mut runtime_container = world
        .remove_non_send_resource::<RuntimeContainer<P>>()
        .unwrap_or_else(|| {
            panic!(
                "No runtime container for runtime {} found. Was the scripting plugin initialized correctly?",
                type_name::<P::R>()
            )
        });
    let runtime = &mut runtime_container.runtime;
    let mut script_contexts = world
        .remove_non_send_resource::<ScriptContexts<P>>()
        .unwrap_or_else(|| panic!("No script contexts found for context {}", type_name::<P>()));

    let (mut script_events, callback_settings, context_settings, scripts, entities) =
        params.get_mut(world);

    let handler = *callback_settings
        .callback_handler
        .as_ref()
        .unwrap_or_else(|| {
            panic!(
                "No handler registered for - Runtime: {}, Context: {}",
                type_name::<P::R>(),
                type_name::<P::C>()
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
                        trace!(
                            "Script `{}` on entity `{:?}` is either still loading or doesn't exist, ignoring.",
                            script_id, entity
                        );
                        continue;
                    }
                };

                let ctxt = match script_contexts.contexts.get_mut(&script.context_id) {
                    Some(ctxt) => ctxt,
                    None => {
                        // if we don't have a context for the script, it's either:
                        // 1. a script for a different language, in which case we ignore it
                        // 2. something went wrong. This should not happen though and it's best we ignore this
                        continue;
                    }
                };

                let handler_result = (handler)(
                    event.args.clone(),
                    *entity,
                    &script.id,
                    &L::into_callback_label(),
                    ctxt,
                    &pre_handling_initializers,
                    runtime,
                    world,
                )
                .map_err(|e| {
                    e.with_script(script.id.clone())
                        .with_context(format!("Event handling for: Language: {}", P::LANGUAGE))
                });

                let _ = push_err_and_continue!(errors, handler_result);
            }
        }
    }

    world.insert_non_send_resource(runtime_container);
    world.insert_non_send_resource(script_contexts);

    handle_script_errors(world, errors.into_iter());
}

/// Handles errors caused by script execution and sends them to the error event channel
pub(crate) fn handle_script_errors<I: Iterator<Item = ScriptError> + Clone>(
    world: &mut World,
    errors: I,
) {
    let mut error_events = world
        .get_resource_mut::<Events<ScriptErrorEvent>>()
        .expect("Missing events resource");

    for error in errors.clone() {
        error_events.send(ScriptErrorEvent { error });
    }

    for error in errors {
        let arc_world = WorldGuard::new(WorldAccessGuard::new(world));
        bevy::log::error!("{}", error.display_with_world(arc_world));
    }
}

#[cfg(test)]
mod test {
    use std::{borrow::Cow, collections::HashMap};

    use crate::{
        bindings::script_value::ScriptValue,
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

    struct TestPlugin;

    impl IntoScriptPluginParams for TestPlugin {
        type C = TestContext;
        type R = TestRuntime;

        const LANGUAGE: crate::asset::Language = crate::asset::Language::Unknown;
    }

    struct TestRuntime {
        pub invocations: Vec<(Entity, ScriptId)>,
    }

    struct TestContext {
        pub invocations: Vec<ScriptValue>,
    }

    fn setup_app<L: IntoCallbackLabel + 'static, P: IntoScriptPluginParams>(
        handler_fn: HandlerFn<P>,
        runtime: P::R,
        contexts: HashMap<u32, P::C>,
        scripts: HashMap<ScriptId, Script>,
    ) -> App {
        let mut app = App::new();

        app.add_event::<ScriptCallbackEvent>();
        app.add_event::<ScriptErrorEvent>();
        app.insert_resource::<CallbackSettings<P>>(CallbackSettings {
            callback_handler: Some(handler_fn),
        });
        app.add_systems(Update, event_handler::<L, P>);
        app.insert_resource::<Scripts>(Scripts { scripts });
        app.insert_non_send_resource(RuntimeContainer::<P> { runtime });
        app.insert_non_send_resource(ScriptContexts::<P> { contexts });
        app.insert_resource(ContextLoadingSettings::<P> {
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
        let mut app = setup_app::<OnTestCallback, TestPlugin>(
            |args, entity, script, _, ctxt, _, runtime, _| {
                ctxt.invocations.extend(args);
                runtime.invocations.push((entity, script.clone()));
                Ok(ScriptValue::Unit)
            },
            runtime,
            contexts,
            scripts,
        );
        let test_entity_id = app
            .world_mut()
            .spawn(ScriptComponent(vec![test_script_id.clone()]))
            .id();

        app.world_mut().send_event(ScriptCallbackEvent::new_for_all(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_args".into())],
        ));
        app.update();

        let test_context = app
            .world()
            .get_non_send_resource::<ScriptContexts<TestPlugin>>()
            .unwrap();
        let test_runtime = app
            .world()
            .get_non_send_resource::<RuntimeContainer<TestPlugin>>()
            .unwrap();

        assert_eq!(
            test_context
                .contexts
                .get(&test_ctxt_id)
                .unwrap()
                .invocations,
            vec![ScriptValue::String("test_args".into())]
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
        let mut app = setup_app::<OnTestCallback, TestPlugin>(
            |args, entity, script, _, ctxt, _, runtime, _| {
                ctxt.invocations.extend(args);
                runtime.invocations.push((entity, script.clone()));
                Ok(ScriptValue::Unit)
            },
            runtime,
            contexts,
            scripts,
        );
        let test_entity_id = app
            .world_mut()
            .spawn(ScriptComponent(vec![test_script_id.clone()]))
            .id();

        app.world_mut().send_event(ScriptCallbackEvent::new(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_args_script".into())],
            crate::event::Recipients::Script(test_script_id.clone()),
        ));

        app.world_mut().send_event(ScriptCallbackEvent::new(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_args_entity".into())],
            crate::event::Recipients::Entity(test_entity_id),
        ));

        app.update();

        let test_context = app
            .world()
            .get_non_send_resource::<ScriptContexts<TestPlugin>>()
            .unwrap();
        let test_runtime = app
            .world()
            .get_non_send_resource::<RuntimeContainer<TestPlugin>>()
            .unwrap();

        assert_eq!(
            test_context
                .contexts
                .get(&test_ctxt_id)
                .unwrap()
                .invocations,
            vec![
                ScriptValue::String("test_args_script".into()),
                ScriptValue::String("test_args_entity".into())
            ]
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
