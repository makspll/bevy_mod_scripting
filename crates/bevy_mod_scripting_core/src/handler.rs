//! Contains the logic for handling script callback events
use crate::{
    Language,
    bindings::{
        pretty_print::DisplayWithWorld, script_value::ScriptValue, ThreadWorldContainer,
        WorldContainer, WorldGuard,
    },
    context::ContextPreHandlingInitializer,
    error::{InteropErrorInner, ScriptError},
    event::{
        CallbackLabel, IntoCallbackLabel, Recipients, ScriptCallbackEvent,
        ScriptCallbackResponseEvent, ScriptErrorEvent,
    },
    extractors::{HandlerContext, WithWorldGuard},
    script::{Domain, ScriptComponent, ScriptContextProvider, ScriptDomain, ScriptId, DisplayProxy},
    IntoScriptPluginParams, ScriptAsset,
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
    entity: Option<Entity>,
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
        entity: Option<Entity>,
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
    Local<
        's,
        QueryState<(
            Entity,
            Ref<'w, ScriptComponent>,
            Option<Ref<'w, ScriptDomain>>,
        )>,
    >,
    crate::extractors::EventReaderScope<'s, ScriptCallbackEvent>,
    WithWorldGuard<'w, 's, HandlerContext<'s, P>>,
)>;

#[profiling::function]
#[allow(deprecated)]
pub(crate) fn event_handler_inner<P: IntoScriptPluginParams>(
    callback_label: CallbackLabel,
    mut entity_query_state: Local<
        QueryState<(Entity, Ref<ScriptComponent>, Option<Ref<ScriptDomain>>)>,
    >,
    mut script_events: crate::extractors::EventReaderScope<ScriptCallbackEvent>,
    mut handler_ctxt: WithWorldGuard<HandlerContext<P>>,
) {
    let events = script_events.read();
    if events.len() == 0 {
        return;
    }
    let mut errors = Vec::default();

    let (guard, handler_ctxt) = handler_ctxt.get_mut();
    for event in events.filter(|e| e.label == callback_label) {
        // Most of these recipients call a single context. In those cases, the
        // following 4 variables will be initialized.
        //
        // But targeting an entity or language or all, may call multiple contexts. In these cases they are handled
        let script_handle;
        let domain;
        let entity;
        let context;
        match &event.recipients {
            Recipients::Script(target_script_id) => {
                // This is not a one and done.
                // TODO: This could match many things.
                script_handle = Handle::Weak(*target_script_id);
                domain = None;
                entity = None;
                context = None;
            }
            Recipients::Entity(target_entity) => {
                if let Err(e) = guard.with_global_access(|world| {
                    if let Ok((_, script_component, script_domain_maybe)) =
                        entity_query_state.get(world, *target_entity)
                    {
                        let domain = script_domain_maybe.as_ref().map(|x| x.0.clone());

                        // Keep track of the contexts that have been called. Don't duplicate the
                        // calls on account of multiple matches.
                        //
                        // If I have five scripts all in one shared context, and I fire a call to
                        // `Recipients::All`, then I want that call to go to the shared context
                        // once.
                        //
                        // If I have four scripts in three different contexts, and I fire a call to
                        // `Recipients::All`, then I want that call to be evaluated three times,
                        // once in each context.
                        let mut called_contexts: HashSet<u64> = HashSet::new();
                        for script_handle in &script_component.0 {
                            if let Some(hash) = handler_ctxt.script_context.hash(
                                Some(*target_entity),
                                &script_handle.id(),
                                &domain,
                            ) {
                                if called_contexts.insert(hash) {
                                    // contexts.push(handler_ctxt.script_context.get(
                                    //     Some(*target_entity),
                                    //     &script_handle.id(),
                                    //     &domain,
                                    // ));

                                let call_result = handler_ctxt.call_dynamic_label(
                                    &callback_label,
                                    &script_handle,
                                    Some(*target_entity),
                                    &domain,
                                    None,
                                    event.args.clone(),
                                    guard.clone(),
                                );

                                if event.trigger_response {
                                    send_callback_response(
                                        guard.clone(),
                                        ScriptCallbackResponseEvent::new(
                                            callback_label.clone(),
                                            script_handle.id(),
                                            call_result.clone(),
                                        ),
                                    );
                                }
                                collect_errors(call_result, Some(*target_entity), P::LANGUAGE, &mut errors);
                                }
                            }
                        }
                    } else {
                        todo!()
                    }
                }) {
                    bevy::log::error_once!(
                        "{}: Failed to query entity {} with scripts: {}",
                        P::LANGUAGE,
                        target_entity,
                        e.display_with_world(guard.clone())
                    );
                }
                continue;
            }
            Recipients::Domain(target_domain) => {

                script_handle = Handle::default();
                domain = Some(target_domain.clone());
                entity = None;
                context = None;
            }
            Recipients::Language(target_language) if *target_language != P::LANGUAGE => {
                continue;
            }
            Recipients::Language(_) | Recipients::All => {
                // All and language are effectively the same modulo the other
                // languages, which are handled by the other P handlers.
                script_handle = Handle::default();
                for context in handler_ctxt.script_context.iter_box() {
                    let call_result = handler_ctxt.call_dynamic_label(
                        &callback_label,
                        &script_handle,
                        None,
                        &None,
                        Some(context),
                        event.args.clone(),
                        guard.clone(),
                    );

                    if event.trigger_response {
                        send_callback_response(
                            guard.clone(),
                            ScriptCallbackResponseEvent::new(
                                callback_label.clone(),
                                script_handle.id(),
                                call_result.clone(),
                            ),
                        );
                    }
                    // The tricky thing here is a context _could_ be associated
                    // with an entity, but we don't quite have means to express
                    // that relationship yet. Maybe
                    // `ScriptContextProvider.iter()` ought to provide that
                    // association using a struct like this:
                    //
                    // ContextKeys {
                    //    script: Option<Handle<ScriptAsset>>,
                    //    entity: Option<Entity>,
                    //    domain: Option<Domain>,
                    // }
                    collect_errors(call_result, None, P::LANGUAGE, &mut errors);
                }
                continue;
            }
        }

        let call_result = handler_ctxt.call_dynamic_label(
            &callback_label,
            &script_handle,
            entity.clone(),
            &domain,
            context,
            event.args.clone(),
            guard.clone(),
        );

        if event.trigger_response {
            send_callback_response(
                guard.clone(),
                ScriptCallbackResponseEvent::new(
                    callback_label.clone(),
                    script_handle.id(),
                    call_result.clone(),
                ),
            );
        }
        collect_errors(call_result, entity, P::LANGUAGE, &mut errors);
    }
    handle_script_errors(guard, errors.into_iter());
}

fn collect_errors(
    call_result: Result<ScriptValue, ScriptError>,
    entity: Option<Entity>,
    language: Language,
    errors: &mut Vec<ScriptError>) {
    match call_result {
        Ok(_) => {}
        Err(e) => {
            match e.downcast_interop_inner() {
                Some(InteropErrorInner::MissingScript { script_id }) => {
                    trace_once!(
                        "{}: Script `{}` on entity `{:?}` is either still loading, doesn't exist, or is for another language; ignoring until the corresponding script is loaded.",
                        language,
                        script_id.display(), entity
                    );
                    return;
                }
                Some(InteropErrorInner::MissingContext { .. }) => {
                    // If we don't have a context for the script, it's either:
                    // 1. A script for a different language, in which case we ignore it.
                    // 2. Something went wrong. This should not happen though and it's best we ignore this.
                    return;
                }
                _ => {}
            }
            // let e = {
            //     // if let Some(path) =
            //     // script_id.path().map(|path| e.with_script(path)).unwrap_or_else(|| e)//  {
            //     //     e.with_script(path)
            //     // } else {
            //     //     e
            //     // }
            // };
            // push_err_and_continue!(errors, Err(e));
            errors.push(e.with_context(format!("Event handling for Language: {}", language)));
        }
    }
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
    // use std::{borrow::Cow, collections::HashMap, sync::Arc};

    use bevy::{
        app::{App, Update},
        asset::{AssetApp, AssetPlugin, Assets},
        diagnostic::DiagnosticsPlugin,
        ecs::world::FromWorld,
    };
    use test_utils::make_test_plugin;

    use crate::{
        ScriptQueue,
        bindings::script_value::ScriptValue,
        context::{ContextBuilder, ContextLoadingSettings},
        event::{CallbackLabel, IntoCallbackLabel, ScriptCallbackEvent, ScriptErrorEvent},
        runtime::RuntimeContainer,
        script::{ScriptComponent, StaticScripts},
        BMSScriptingInfrastructurePlugin, ManageStaticScripts, ScriptContext,
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

    fn setup_app<L: IntoCallbackLabel + 'static>(runtime: TestRuntime) -> App {
        let mut app = App::new();
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.init_asset::<ScriptAsset>();
        app.init_resource::<ScriptQueue>();
        app.add_event::<ScriptCallbackEvent>();
        app.add_event::<ScriptCallbackResponseEvent>();
        app.add_event::<ScriptErrorEvent>();
        app.insert_resource::<CallbackSettings<TestPlugin>>(CallbackSettings {
            callback_handler: |args, entity, script, _, ctxt, _, runtime| {
                ctxt.invocations.extend(args);
                let mut runtime = runtime.invocations.lock();
                runtime.push((entity, script.id()));
                Ok(ScriptValue::Unit)
            },
        });
        app.add_plugins(crate::configure_asset_systems_for_plugin::<TestPlugin>);
        app.add_systems(Update, event_handler::<L, TestPlugin>);
        app.insert_resource(RuntimeContainer::<TestPlugin> { runtime });
        app.init_resource::<StaticScripts>();
        app.insert_resource(ContextLoadingSettings::<TestPlugin> {
            loader: ContextBuilder {
                load: |_, _, _, _, _| Ok(TestContext::default()),
                reload: |_, _, _, _, _, _| Ok(()),
            },
            assignment_strategy: Default::default(),
            context_initializers: vec![],
            context_pre_handling_initializers: vec![],
        });
        app.finish();
        app.cleanup();
        app
    }

    fn add_script(app: &mut App, content: impl Into<String>) -> Handle<ScriptAsset> {
        app.world_mut()
            .resource_mut::<Assets<ScriptAsset>>()
            .add(ScriptAsset::from(content.into()))
    }

    #[test]
    fn test_handler_emits_response_events() {
        let runtime = TestRuntime {
            invocations: vec![].into(),
        };
        let mut app = setup_app::<OnTestCallback>(runtime);
        let test_script_id = add_script(&mut app, "");
        app.world_mut()
            .spawn(ScriptComponent(vec![test_script_id.clone()]));
        app.update();

        app.world_mut().send_event(
            ScriptCallbackEvent::new(
                OnTestCallback::into_callback_label(),
                vec![ScriptValue::String("test_args".into())],
                Recipients::All,
            )
            .with_response(),
        );
        app.update();

        assert_response_events(
            app.world_mut(),
            vec![ScriptCallbackResponseEvent::new(
                OnTestCallback::into_callback_label(),
                test_script_id.id(),
                Ok(ScriptValue::Unit),
            )]
            .into_iter(),
        );
    }

    #[test]
    fn test_handler_emits_response_events_for_static_script() {
        let runtime = TestRuntime {
            invocations: vec![].into(),
        };
        let mut app = setup_app::<OnTestCallback>(runtime);
        let test_script_id = add_script(&mut app, "");
        app.add_static_script(test_script_id.clone());
        app.update();

        app.world_mut().send_event(
            ScriptCallbackEvent::new(
                OnTestCallback::into_callback_label(),
                vec![ScriptValue::String("test_args".into())],
                Recipients::All,
            )
            .with_response(),
        );
        app.update();

        assert_response_events(
            app.world_mut(),
            vec![ScriptCallbackResponseEvent::new(
                OnTestCallback::into_callback_label(),
                test_script_id.id(),
                Ok(ScriptValue::Unit),
            )]
            .into_iter(),
        );
    }

    #[test]
    fn test_handler_called_with_right_args() {
        let runtime = TestRuntime {
            invocations: vec![].into(),
        };
        let mut app = setup_app::<OnTestCallback>(runtime);
        let test_script_id = add_script(&mut app, "");
        // app.add_static_script(test_script_id.clone());
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
            let script_context = app
                .world()
                .get_resource::<ScriptContext<TestPlugin>>()
                .unwrap();
            let context_arc = script_context
                .get(Some(test_entity_id), &test_script_id.id(), &None)
                .cloned()
                .expect("script context");

            let test_context = context_arc.lock();

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
                vec![(Some(test_entity_id), test_script_id.id())]
            );
        }
        assert_response_events(app.world_mut(), vec![].into_iter());
    }

    #[test]
    fn test_handler_called_on_right_recipients() {
        let runtime = TestRuntime {
            invocations: vec![].into(),
        };
        let mut app = setup_app::<OnTestCallback>(runtime);
        // app.insert_resource(ScriptContext::<TestPlugin>::per_entity_and_scriptid());
        let test_script_id = add_script(&mut app, "");
        let test_script2_id = add_script(&mut app, "wrong");
        // app.add_static_script(test_script_id.clone());
        let test_entity_id = app
            .world_mut()
            .spawn(ScriptComponent(vec![test_script_id.clone()]))
            .id();

        let test_entity2_id = app
            .world_mut()
            .spawn(ScriptComponent(vec![test_script2_id.clone()]))
            .id();
        app.update();

        app.world_mut().send_event(ScriptCallbackEvent::new(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_args_script".into())],
            Recipients::Script(test_script_id.id()),
        ));

        app.world_mut().send_event(ScriptCallbackEvent::new(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_args_entity".into())],
            Recipients::Entity(test_entity_id),
        ));

        app.update();
        {
            // let test_scripts = app.world().get_resource::<StaticScripts>().unwrap();
            let test_runtime = app
                .world()
                .get_resource::<RuntimeContainer<TestPlugin>>()
                .unwrap();
            let test_runtime = test_runtime.runtime.invocations.lock();

            let script_context = app
                .world()
                .get_resource::<ScriptContext<TestPlugin>>()
                .unwrap();
            let context_arc = script_context
                .get(Some(test_entity_id), &test_script_id.id(), &None)
                .cloned()
                .expect("script context");
            let context_after = context_arc.lock();
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
                    (Some(test_entity_id), test_script_id.id()),
                    (Some(test_entity_id), test_script_id.id())
                ]
            );
        }
        assert_response_events(app.world_mut(), vec![].into_iter());
    }

    #[test]
    fn test_handler_called_for_static_scripts() {
        let runtime = TestRuntime {
            invocations: vec![].into(),
        };

        let mut app = setup_app::<OnTestCallback>(runtime);
        let test_script_id = add_script(&mut app, "");
        app.add_static_script(test_script_id.clone());
        app.update();

        app.world_mut().send_event(ScriptCallbackEvent::new(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_args_script".into())],
            Recipients::All,
        ));

        app.world_mut().send_event(ScriptCallbackEvent::new(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_script_id".into())],
            Recipients::Script(test_script_id.id()),
        ));

        app.update();
        {
            let script_context = app
                .world()
                .get_resource::<ScriptContext<TestPlugin>>()
                .unwrap();
            let context_arc = script_context
                .get(None, &test_script_id.id(), &None)
                .cloned()
                .expect("script context");
            let test_context = context_arc.lock();

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
