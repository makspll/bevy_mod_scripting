//! Contains the logic for handling script callback events
use crate::{
    ScriptAsset,
    bindings::{
        pretty_print::DisplayWithWorld, script_value::ScriptValue, ThreadWorldContainer,
        WorldContainer, WorldGuard,
    },
    context::ContextPreHandlingInitializer,
    error::{InteropErrorInner, ScriptError},
    event::{
        CallbackLabel, IntoCallbackLabel, ScriptCallbackEvent, ScriptCallbackResponseEvent,
        ScriptErrorEvent,
    },
    extractors::{HandlerContext, WithWorldGuard},
    script::{ScriptComponent, ScriptId, ScriptDomain, Domain},
    IntoScriptPluginParams,
};
use bevy::{
    asset::Handle,
    ecs::{
        entity::Entity,
        query::QueryState,
        system::{Local, Resource, SystemState},
        world::{Mut, World},
    },
    log::trace_once,
    prelude::{Events, Ref},
    utils::HashSet,
};

/// A function that handles a callback event
pub type HandlerFn<P> = fn(
    args: Vec<ScriptValue>,
    entity: Entity,
    script_id: &Handle<ScriptAsset>,
    callback: &CallbackLabel,
    context: &mut <P as IntoScriptPluginParams>::C,
    pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
    runtime: &<P as IntoScriptPluginParams>::R,
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
        script_id: &Handle<ScriptAsset>,
        callback: &CallbackLabel,
        script_ctxt: &mut P::C,
        pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
        runtime: &P::R,
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
    Local<'s, QueryState<(Entity, Ref<'w, ScriptComponent>, Option<Ref<'w, ScriptDomain>>)>>,
    crate::extractors::EventReaderScope<'s, ScriptCallbackEvent>,
    WithWorldGuard<'w, 's, HandlerContext<'s, P>>,
)>;

#[profiling::function]
#[allow(deprecated)]
pub(crate) fn event_handler_inner<P: IntoScriptPluginParams>(
    callback_label: CallbackLabel,
    mut entity_query_state: Local<QueryState<(Entity, Ref<ScriptComponent>, Option<Ref<ScriptDomain>>)>>,
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
            .map(|(e, s, d)| (e, s.0.clone(), d.map(|x| x.0.clone())))
            .chain(
                handler_ctxt
                    .static_scripts
                    .scripts
                    .iter()
                    .map(|s| (Entity::from_raw(0), vec![s.clone()], None)),
            )
            .collect::<Vec<_>>()
    });

    let entity_and_static_scripts = match entity_and_static_scripts {
        Ok(entity_and_static_scripts) => entity_and_static_scripts,
        Err(e) => {
            bevy::log::error_once!(
                "{}: Failed to query entities with scripts: {}",
                P::LANGUAGE,
                e.display_with_world(guard.clone())
            );
            return;
        }
    };

    // TODO: Instead of `Domain` we should limit calls by `Context` but I don't
    // think we've got a hash for that.
    let mut called_domain: HashSet<Domain> = HashSet::new();
    for event in events.into_iter().filter(|e| e.label == callback_label) {
        for (entity, entity_scripts, domain) in entity_and_static_scripts.iter() {
            for script_id in entity_scripts.iter() {
                match &event.recipients {
                    crate::event::Recipients::Script(target_script_id)
                        if *target_script_id != script_id.id() =>
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
                    crate::event::Recipients::Domain(target_domain)
                        if domain.as_ref().map(|x| *x != *target_domain).unwrap_or(false) =>
                    {
                        continue
                    }
                    crate::event::Recipients::All => (),
                    _ => ()

                }
                if let Some(domain) = domain {
                    if called_domain.contains(domain) {
                        // Only call a domain once. Not once per script.
                        continue;
                    }
                    called_domain.insert(domain.clone());
                }


                let call_result = handler_ctxt.call_dynamic_label(
                    &callback_label,
                    &script_id,
                    *entity,
                    domain,
                    event.args.clone(),
                    guard.clone(),
                );

                if event.trigger_response {
                    send_callback_response(
                        guard.clone(),
                        ScriptCallbackResponseEvent::new(
                            callback_label.clone(),
                            script_id.id(),
                            call_result.clone(),
                        ),
                    );
                }

                match call_result {
                    Ok(_) => {}
                    Err(e) => {
                        match e.downcast_interop_inner() {
                            Some(InteropErrorInner::MissingScript { script_id }) => {
                                if let Some(path) = script_id.path() {
                                    trace_once!(
                                        "{}: Script path `{}` on entity `{:?}` is either still loading, doesn't exist, or is for another language, ignoring until the corresponding script is loaded.",
                                        P::LANGUAGE,
                                        path, entity
                                    );
                                } else {
                                    trace_once!(
                                        "{}: Script id `{}` on entity `{:?}` is either still loading, doesn't exist, or is for another language, ignoring until the corresponding script is loaded.",
                                        P::LANGUAGE,
                                        script_id.id(), entity
                                    );
                                }
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
                        let e = {

                            if let Some(path) = script_id.path() {
                                e.with_script(path)
                            } else {
                                e
                            }
                            .with_context(format!("Event handling for: Language: {}", P::LANGUAGE))
                        };
                        push_err_and_continue!(errors, Err(e));
                    }
                };
            }
        }
    }

    handle_script_errors(guard, errors.into_iter());
}

/// Sends a callback response event to the world
pub fn send_callback_response(world: WorldGuard, response: ScriptCallbackResponseEvent) {
    let err = world.with_resource_mut(|mut events: Mut<Events<ScriptCallbackResponseEvent>>| {
        events.send(response);
    });

    if let Err(err) = err {
        bevy::log::error!(
            "Failed to send script callback response: {}",
            err.display_with_world(world.clone())
        );
    }
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
    use std::{borrow::Cow, collections::HashMap, sync::Arc};

    use bevy::{
        app::{App, Update},
        asset::AssetPlugin,
        diagnostic::DiagnosticsPlugin,
        ecs::world::FromWorld,
    };
    use parking_lot::Mutex;
    use test_utils::make_test_plugin;

    use crate::{
        bindings::script_value::ScriptValue,
        context::{ContextBuilder, ContextLoadingSettings},
        event::{CallbackLabel, IntoCallbackLabel, ScriptCallbackEvent, ScriptErrorEvent},
        runtime::RuntimeContainer,
        script::{Script, ScriptComponent, ScriptId, Scripts, StaticScripts},
        BMSScriptingInfrastructurePlugin,
    };

    use super::*;
    struct OnTestCallback;

    impl IntoCallbackLabel for OnTestCallback {
        fn into_callback_label() -> CallbackLabel {
            "OnTest".into()
        }
    }

    make_test_plugin!(crate);

    fn assert_response_events(
        app: &mut World,
        expected: impl Iterator<Item = ScriptCallbackResponseEvent>,
    ) {
        let mut events = app
            .get_resource_mut::<Events<ScriptCallbackResponseEvent>>()
            .unwrap();
        let responses = events.drain().collect::<Vec<_>>();
        let expected: Vec<_> = expected.collect();
        assert_eq!(
            responses.len(),
            expected.len(),
            "Incorrect amount of events received"
        );
        for (a, b) in responses.iter().zip(expected.iter()) {
            assert_eq!(a.label, b.label);
            assert_eq!(a.script, b.script);
            assert_eq!(a.response, b.response);
        }
    }

    fn setup_app<L: IntoCallbackLabel + 'static>(
        runtime: TestRuntime,
        scripts: HashMap<ScriptId, Script<TestPlugin>>,
    ) -> App {
        let mut app = App::new();

        app.add_event::<ScriptCallbackEvent>();
        app.add_event::<ScriptCallbackResponseEvent>();
        app.add_event::<ScriptErrorEvent>();
        app.insert_resource::<CallbackSettings<TestPlugin>>(CallbackSettings {
            callback_handler: |args, entity, script, _, ctxt, _, runtime| {
                ctxt.invocations.extend(args);
                let mut runtime = runtime.invocations.lock();
                runtime.push((entity, script.clone()));
                Ok(ScriptValue::Unit)
            },
        });
        app.add_systems(Update, event_handler::<L, TestPlugin>);
        app.insert_resource::<Scripts<TestPlugin>>(Scripts { scripts });
        app.insert_resource(RuntimeContainer::<TestPlugin> { runtime });
        app.init_resource::<StaticScripts>();
        app.insert_resource(ContextLoadingSettings::<TestPlugin> {
            loader: ContextBuilder {
                load: |_, _, _, _, _| todo!(),
                reload: |_, _, _, _, _, _| todo!(),
            },
            assignment_strategy: Default::default(),
            context_initializers: vec![],
            context_pre_handling_initializers: vec![],
        });
        app.finish();
        app.cleanup();
        app
    }

    #[test]
    fn test_handler_emits_response_events() {
        let test_script_id = Cow::Borrowed("test_script");
        let test_script = Script {
            id: test_script_id.clone(),
            asset: None,
            context: Arc::new(Mutex::new(TestContext::default())),
        };
        let scripts = HashMap::from_iter(vec![(test_script_id.clone(), test_script.clone())]);
        let runtime = TestRuntime {
            invocations: vec![].into(),
        };
        let mut app = setup_app::<OnTestCallback>(runtime, scripts);
        app.world_mut()
            .spawn(ScriptComponent(vec![test_script_id.clone()]));

        app.world_mut().send_event(
            ScriptCallbackEvent::new(
                OnTestCallback::into_callback_label(),
                vec![ScriptValue::String("test_args".into())],
                crate::event::Recipients::All,
            )
            .with_response(),
        );
        app.update();

        assert_response_events(
            app.world_mut(),
            vec![ScriptCallbackResponseEvent::new(
                OnTestCallback::into_callback_label(),
                test_script_id.clone(),
                Ok(ScriptValue::Unit),
            )]
            .into_iter(),
        );
    }

    #[test]
    fn test_handler_called_with_right_args() {
        let test_script_id = Cow::Borrowed("test_script");
        let test_script = Script {
            id: test_script_id.clone(),
            asset: None,
            context: Arc::new(Mutex::new(TestContext::default())),
        };
        let scripts = HashMap::from_iter(vec![(test_script_id.clone(), test_script.clone())]);
        let runtime = TestRuntime {
            invocations: vec![].into(),
        };
        let mut app = setup_app::<OnTestCallback>(runtime, scripts);
        let test_entity_id = app
            .world_mut()
            .spawn(ScriptComponent(vec![test_script_id.clone()]))
            .id();

        app.world_mut().send_event(ScriptCallbackEvent::new_for_all(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_args".into())],
        ));
        app.update();
        {
            let test_script = app
                .world()
                .get_resource::<Scripts<TestPlugin>>()
                .unwrap()
                .scripts
                .get(&test_script_id)
                .unwrap();

            let test_context = test_script.context.lock();

            let test_runtime = app
                .world()
                .get_resource::<RuntimeContainer<TestPlugin>>()
                .unwrap();

            assert_eq!(
                test_context.invocations,
                vec![ScriptValue::String("test_args".into())]
            );

            let runtime_invocations = test_runtime.runtime.invocations.lock();
            assert_eq!(
                runtime_invocations
                    .iter()
                    .map(|(e, s)| (*e, s.clone()))
                    .collect::<Vec<_>>(),
                vec![(test_entity_id, test_script_id.clone())]
            );
        }
        assert_response_events(app.world_mut(), vec![].into_iter());
    }

    #[test]
    fn test_handler_called_on_right_recipients() {
        let test_script_id = Cow::Borrowed("test_script");
        let test_script = Script {
            id: test_script_id.clone(),
            asset: None,
            context: Arc::new(Mutex::new(TestContext::default())),
        };
        let scripts = HashMap::from_iter(vec![
            (test_script_id.clone(), test_script.clone()),
            (
                "wrong".into(),
                Script {
                    id: "wrong".into(),
                    asset: None,
                    context: Arc::new(Mutex::new(TestContext::default())),
                },
            ),
        ]);

        let runtime = TestRuntime {
            invocations: vec![].into(),
        };
        let mut app = setup_app::<OnTestCallback>(runtime, scripts);
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
        {
            let test_scripts = app.world().get_resource::<Scripts<TestPlugin>>().unwrap();
            let test_runtime = app
                .world()
                .get_resource::<RuntimeContainer<TestPlugin>>()
                .unwrap();
            let test_runtime = test_runtime.runtime.invocations.lock();
            let script_after = test_scripts.scripts.get(&test_script_id).unwrap();
            let context_after = script_after.context.lock();
            assert_eq!(
                context_after.invocations,
                vec![
                    ScriptValue::String("test_args_script".into()),
                    ScriptValue::String("test_args_entity".into())
                ]
            );

            assert_eq!(
                test_runtime
                    .iter()
                    .map(|(e, s)| (*e, s.clone()))
                    .collect::<Vec<_>>(),
                vec![
                    (test_entity_id, test_script_id.clone()),
                    (test_entity_id, test_script_id.clone())
                ]
            );
        }
        assert_response_events(app.world_mut(), vec![].into_iter());
    }

    #[test]
    fn test_handler_called_for_static_scripts() {
        let test_script_id = Cow::Borrowed("test_script");

        let scripts = HashMap::from_iter(vec![(
            test_script_id.clone(),
            Script {
                id: test_script_id.clone(),
                asset: None,
                context: Arc::new(Mutex::new(TestContext::default())),
            },
        )]);
        let runtime = TestRuntime {
            invocations: vec![].into(),
        };
        let mut app = setup_app::<OnTestCallback>(runtime, scripts);

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
        {
            let test_scripts = app.world().get_resource::<Scripts<TestPlugin>>().unwrap();
            let test_context = test_scripts
                .scripts
                .get(&test_script_id)
                .unwrap()
                .context
                .lock();

            assert_eq!(
                test_context.invocations,
                vec![
                    ScriptValue::String("test_args_script".into()),
                    ScriptValue::String("test_script_id".into())
                ]
            );
        }
        assert_response_events(app.world_mut(), vec![].into_iter());
    }

    #[test]
    fn event_handler_reinserts_resources() {
        let mut app = App::new();
        app.add_plugins((
            AssetPlugin::default(),
            DiagnosticsPlugin,
            TestPlugin::default(),
            BMSScriptingInfrastructurePlugin,
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
