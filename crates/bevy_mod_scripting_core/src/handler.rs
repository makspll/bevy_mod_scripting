//! Contains the logic for handling script callback events
use crate::{
    bindings::{
        pretty_print::DisplayWithWorld, script_value::ScriptValue, ThreadWorldContainer,
        WorldContainer, WorldGuard,
    },
    context::ContextPreHandlingInitializer,
    error::{InteropErrorInner, ScriptError},
    event::{CallbackLabel, IntoCallbackLabel, ScriptCallbackEvent, ScriptErrorEvent},
    extractors::{HandlerContext, WithWorldGuard},
    script::{ScriptComponent, ScriptId},
    IntoScriptPluginParams,
};
use bevy::{
    ecs::{
        entity::Entity,
        query::QueryState,
        system::{Local, Resource, SystemState},
        world::{Mut, World},
    },
    log::trace_once,
    prelude::{Events, Ref},
};

/// A function that handles a callback event
pub type HandlerFn<P> = fn(
    args: Vec<ScriptValue>,
    entity: Entity,
    script_id: &ScriptId,
    callback: &CallbackLabel,
    context: &mut <P as IntoScriptPluginParams>::C,
    pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
    runtime: &mut <P as IntoScriptPluginParams>::R,
) -> Result<ScriptValue, ScriptError>;

/// A resource that holds the settings for the callback handler for a specific combination of type parameters
#[derive(Resource)]
pub struct CallbackSettings<P: IntoScriptPluginParams> {
    /// The callback handler function
    pub callback_handler: HandlerFn<P>,
}

impl<P: IntoScriptPluginParams> Default for CallbackSettings<P> {
    fn default() -> Self {
        Self {
            callback_handler: |_, _, _, _, _, _, _| Ok(ScriptValue::Unit),
        }
    }
}

impl<P: IntoScriptPluginParams> Clone for CallbackSettings<P> {
    fn clone(&self) -> Self {
        Self {
            callback_handler: self.callback_handler,
        }
    }
}

#[profiling::all_functions]
impl<P: IntoScriptPluginParams> CallbackSettings<P> {
    /// Creates a new callback settings resource with the given handler function
    pub fn new(callback_handler: HandlerFn<P>) -> Self {
        Self { callback_handler }
    }

    /// Calls the handler function while providing the necessary thread local context
    pub fn call(
        handler: HandlerFn<P>,
        args: Vec<ScriptValue>,
        entity: Entity,
        script_id: &ScriptId,
        callback: &CallbackLabel,
        script_ctxt: &mut P::C,
        pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
        runtime: &mut P::R,
        world: WorldGuard,
    ) -> Result<ScriptValue, ScriptError> {
        WorldGuard::with_existing_static_guard(world.clone(), |world| {
            ThreadWorldContainer.set_world(world)?;
            (handler)(
                args,
                entity,
                script_id,
                callback,
                script_ctxt,
                pre_handling_initializers,
                runtime,
            )
        })
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

/// Passes events with the specified label to the script callback with the same name and runs the callback.
///
/// If any of the resources required for the handler are missing, the system will log this issue and do nothing.
#[allow(deprecated)]
pub fn event_handler<L: IntoCallbackLabel, P: IntoScriptPluginParams>(
    world: &mut World,
    state: &mut EventHandlerSystemState<P>,
) {
    // we wrap the inner event handler, so that we can immediately re-insert all the resources back.
    // otherwise this would happen in the next schedule
    {
        let (entity_query_state, script_events, handler_ctxt) = state.get_mut(world);
        event_handler_inner::<P>(
            L::into_callback_label(),
            entity_query_state,
            script_events,
            handler_ctxt,
        );
    }
    state.apply(world);
}

#[allow(deprecated)]
pub(crate) type EventHandlerSystemState<'w, 's, P> = SystemState<(
    Local<'s, QueryState<(Entity, Ref<'w, ScriptComponent>)>>,
    crate::extractors::EventReaderScope<'s, ScriptCallbackEvent>,
    WithWorldGuard<'w, 's, HandlerContext<'s, P>>,
)>;

#[profiling::function]
#[allow(deprecated)]
pub(crate) fn event_handler_inner<P: IntoScriptPluginParams>(
    callback_label: CallbackLabel,
    mut entity_query_state: Local<QueryState<(Entity, Ref<ScriptComponent>)>>,
    mut script_events: crate::extractors::EventReaderScope<ScriptCallbackEvent>,
    mut handler_ctxt: WithWorldGuard<HandlerContext<P>>,
) {
    let (guard, handler_ctxt) = handler_ctxt.get_mut();

    let mut errors = Vec::default();

    let events = script_events.read().cloned().collect::<Vec<_>>();

    // query entities + chain static scripts
    let entity_and_static_scripts = guard.with_global_access(|world| {
        entity_query_state
            .iter(world)
            .map(|(e, s)| (e, s.0.clone()))
            .chain(
                handler_ctxt
                    .static_scripts
                    .scripts
                    .iter()
                    .map(|s| (Entity::from_raw(0), vec![s.clone()])),
            )
            .collect::<Vec<_>>()
    });

    let entity_and_static_scripts = match entity_and_static_scripts {
        Ok(entity_and_static_scripts) => entity_and_static_scripts,
        Err(e) => {
            bevy::log::error!(
                "{}: Failed to query entities with scripts: {}",
                P::LANGUAGE,
                e.display_with_world(guard.clone())
            );
            return;
        }
    };

    for event in events.into_iter().filter(|e| e.label == callback_label) {
        for (entity, entity_scripts) in entity_and_static_scripts.iter() {
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
                    crate::event::Recipients::Language(target_language)
                        if *target_language != P::LANGUAGE =>
                    {
                        continue
                    }
                    _ => {}
                }

                let call_result = handler_ctxt.call_dynamic_label(
                    &callback_label,
                    script_id.clone(),
                    *entity,
                    event.args.clone(),
                    guard.clone(),
                );

                match call_result {
                    Ok(_) => {}
                    Err(e) => {
                        match e.downcast_interop_inner() {
                            Some(InteropErrorInner::MissingScript { script_id }) => {
                                trace_once!(
                                "{}: Script `{}` on entity `{:?}` is either still loading or doesn't exist, ignoring until the corresponding script is loaded.",
                                P::LANGUAGE,
                                script_id, entity
                            );
                                continue;
                            }
                            Some(InteropErrorInner::MissingContext { .. }) => {
                                // if we don't have a context for the script, it's either:
                                // 1. a script for a different language, in which case we ignore it
                                // 2. something went wrong. This should not happen though and it's best we ignore this
                                continue;
                            }
                            _ => {}
                        }
                        let e = e
                            .with_script(script_id.clone())
                            .with_context(format!("Event handling for: Language: {}", P::LANGUAGE));
                        push_err_and_continue!(errors, Err(e));
                    }
                };
            }
        }
    }

    handle_script_errors(guard, errors.into_iter());
}

/// Handles errors caused by script execution and sends them to the error event channel
pub fn handle_script_errors<I: Iterator<Item = ScriptError> + Clone>(world: WorldGuard, errors: I) {
    let err = world.with_resource_mut(|mut error_events: Mut<Events<ScriptErrorEvent>>| {
        for error in errors.clone() {
            error_events.send(ScriptErrorEvent { error });
        }
    });

    if let Err(err) = err {
        bevy::log::error!(
            "Failed to send script error events: {}",
            err.display_with_world(world.clone())
        );
    }

    for error in errors {
        bevy::log::error!("{}", error.display_with_world(world.clone()));
    }
}

#[cfg(test)]
#[allow(clippy::todo)]
mod test {
    use std::{borrow::Cow, collections::HashMap};

    use bevy::{
        app::{App, Update},
        asset::AssetPlugin,
        diagnostic::DiagnosticsPlugin,
        ecs::world::FromWorld,
    };
    use test_utils::make_test_plugin;

    use crate::{
        bindings::script_value::ScriptValue,
        context::{ContextAssigner, ContextBuilder, ContextLoadingSettings, ScriptContexts},
        event::{CallbackLabel, IntoCallbackLabel, ScriptCallbackEvent, ScriptErrorEvent},
        handler::HandlerFn,
        runtime::RuntimeContainer,
        script::{Script, ScriptComponent, ScriptId, Scripts, StaticScripts},
    };

    use super::*;
    struct OnTestCallback;

    impl IntoCallbackLabel for OnTestCallback {
        fn into_callback_label() -> CallbackLabel {
            "OnTest".into()
        }
    }

    make_test_plugin!(crate);

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
            callback_handler: handler_fn,
        });
        app.add_systems(Update, event_handler::<L, P>);
        app.insert_resource::<Scripts>(Scripts { scripts });
        app.insert_resource(RuntimeContainer::<P> { runtime });
        app.insert_resource(ScriptContexts::<P> { contexts });
        app.init_resource::<StaticScripts>();
        app.insert_resource(ContextLoadingSettings::<P> {
            loader: ContextBuilder {
                load: |_, _, _, _, _| todo!(),
                reload: |_, _, _, _, _, _| todo!(),
            },
            assigner: ContextAssigner {
                assign: |_, _, _| todo!(),
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
            |args, entity, script, _, ctxt, _, runtime| {
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
            .get_resource::<ScriptContexts<TestPlugin>>()
            .unwrap();
        let test_runtime = app
            .world()
            .get_resource::<RuntimeContainer<TestPlugin>>()
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
            |args, entity, script, _, ctxt, _, runtime| {
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
            .get_resource::<ScriptContexts<TestPlugin>>()
            .unwrap();
        let test_runtime = app
            .world()
            .get_resource::<RuntimeContainer<TestPlugin>>()
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

    #[test]
    fn test_handler_called_for_static_scripts() {
        let test_script_id = Cow::Borrowed("test_script");
        let test_ctxt_id = 0;

        let scripts = HashMap::from_iter(vec![(
            test_script_id.clone(),
            Script {
                id: test_script_id.clone(),
                asset: None,
                context_id: test_ctxt_id,
            },
        )]);
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
            |args, entity, script, _, ctxt, _, runtime| {
                ctxt.invocations.extend(args);
                runtime.invocations.push((entity, script.clone()));
                Ok(ScriptValue::Unit)
            },
            runtime,
            contexts,
            scripts,
        );

        app.world_mut().insert_resource(StaticScripts {
            scripts: vec![test_script_id.clone()].into_iter().collect(),
        });

        app.world_mut().send_event(ScriptCallbackEvent::new(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_args_script".into())],
            crate::event::Recipients::All,
        ));

        app.world_mut().send_event(ScriptCallbackEvent::new(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_script_id".into())],
            crate::event::Recipients::Script(test_script_id.clone()),
        ));

        app.update();

        let test_context = app
            .world()
            .get_resource::<ScriptContexts<TestPlugin>>()
            .unwrap();

        assert_eq!(
            test_context
                .contexts
                .get(&test_ctxt_id)
                .unwrap()
                .invocations,
            vec![
                ScriptValue::String("test_args_script".into()),
                ScriptValue::String("test_script_id".into())
            ]
        );
    }

    #[test]
    fn event_handler_reinserts_resources() {
        let mut app = App::new();
        app.add_plugins((
            AssetPlugin::default(),
            DiagnosticsPlugin,
            TestPlugin::default(),
        ));

        assert!(app
            .world()
            .contains_resource::<Events<ScriptCallbackEvent>>());

        let mut local = SystemState::from_world(app.world_mut());

        assert!(app
            .world()
            .contains_resource::<Events<ScriptCallbackEvent>>());

        event_handler::<OnTestCallback, TestPlugin>(app.world_mut(), &mut local);

        assert!(app
            .world()
            .get_resource::<Events<ScriptCallbackEvent>>()
            .is_some());
    }
}
