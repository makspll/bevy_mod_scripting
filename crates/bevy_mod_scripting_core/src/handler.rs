use std::any::type_name;

use crate::{
    bindings::{
        pretty_print::DisplayWithWorld, script_value::ScriptValue, WorldAccessGuard, WorldGuard,
    },
    context::{ContextLoadingSettings, ContextPreHandlingInitializer, ScriptContexts},
    error::ScriptError,
    event::{CallbackLabel, IntoCallbackLabel, ScriptCallbackEvent, ScriptErrorEvent},
    runtime::RuntimeContainer,
    script::{ScriptComponent, ScriptId, Scripts},
    IntoScriptPluginParams,
};
use bevy::{
    ecs::{
        entity::Entity,
        system::{Resource, SystemState},
        world::World,
    },
    log::{debug, trace},
    prelude::{EventReader, Events, Query, Ref, Res},
};

pub trait Args: Clone + Send + Sync + 'static {}
impl<T: Clone + Send + Sync + 'static> Args for T {}

pub type HandlerFn<P> = fn(
    args: Vec<ScriptValue>,
    entity: Entity,
    script_id: &ScriptId,
    callback: &CallbackLabel,
    context: &mut <P as IntoScriptPluginParams>::C,
    pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
    runtime: &mut <P as IntoScriptPluginParams>::R,
    world: &mut World,
) -> Result<ScriptValue, ScriptError>;

/// A resource that holds the settings for the callback handler for a specific combination of type parameters
#[derive(Resource)]
pub struct CallbackSettings<P: IntoScriptPluginParams> {
    pub callback_handler: Option<HandlerFn<P>>,
}

impl<P: IntoScriptPluginParams> Default for CallbackSettings<P> {
    fn default() -> Self {
        Self {
            callback_handler: None,
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

    use bevy::app::{App, Update};

    use crate::{
        bindings::script_value::ScriptValue,
        context::{ContextAssigner, ContextBuilder, ContextLoadingSettings, ScriptContexts},
        event::{CallbackLabel, IntoCallbackLabel, ScriptCallbackEvent, ScriptErrorEvent},
        handler::HandlerFn,
        runtime::RuntimeContainer,
        script::{Script, ScriptComponent, ScriptId, Scripts},
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

        fn build_runtime() -> Self::R {
            TestRuntime {
                invocations: vec![],
            }
        }
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
            loader: ContextBuilder {
                load: |_, _, _, _, _, _| todo!(),
                reload: |_, _, _, _, _, _, _| todo!(),
            },
            assigner: ContextAssigner {
                assign: |_, _, _, _| todo!(),
                remove: |_, _, _| todo!(),
            },
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
