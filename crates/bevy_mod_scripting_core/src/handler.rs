//! Contains the logic for handling script callback events
use crate::{
    bindings::{
        pretty_print::DisplayWithWorld, script_value::ScriptValue, ThreadWorldContainer,
        WorldContainer, WorldGuard,
    },
    context::ContextPreHandlingInitializer,
    error::ScriptError,
    event::{
        CallbackLabel, IntoCallbackLabel, ScriptCallbackEvent, ScriptCallbackResponseEvent,
        ScriptErrorEvent,
    },
    extractors::{HandlerContext, WithWorldGuard},
    script::ScriptAttachment,
    IntoScriptPluginParams, Language,
};
use bevy::{
    ecs::{
        system::{Resource, SystemState},
        world::{Mut, World},
    },
    prelude::Events,
};

/// A function that handles a callback event
pub type HandlerFn<P> = fn(
    args: Vec<ScriptValue>,
    context_key: &ScriptAttachment,
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
            callback_handler: |_, _, _, _, _, _| Ok(ScriptValue::Unit),
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
        context_key: &ScriptAttachment,
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
                context_key,
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
        let (script_events, handler_ctxt) = state.get_mut(world);
        event_handler_inner::<P>(L::into_callback_label(), script_events, handler_ctxt);
    }
    state.apply(world);
}

#[allow(deprecated)]
pub(crate) type EventHandlerSystemState<'w, 's, P> = SystemState<(
    crate::extractors::EventReaderScope<'s, ScriptCallbackEvent>,
    WithWorldGuard<'w, 's, HandlerContext<'s, P>>,
)>;

#[profiling::function]
#[allow(deprecated)]
pub(crate) fn event_handler_inner<P: IntoScriptPluginParams>(
    callback_label: CallbackLabel,
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
        let recipients = event
            .recipients
            .get_recipients(&handler_ctxt.script_context);

        for (attachment, ctxt) in recipients {
            let call_result = handler_ctxt.call_dynamic_label(
                &callback_label,
                &attachment,
                Some(ctxt),
                event.args.clone(),
                guard.clone(),
            );

            if event.trigger_response {
                send_callback_response(
                    guard.clone(),
                    ScriptCallbackResponseEvent::new(
                        callback_label.clone(),
                        attachment,
                        call_result.clone(),
                    ),
                );
            }
            collect_errors(call_result, P::LANGUAGE, &mut errors);
        }
    }
    handle_script_errors(guard, errors.into_iter());
}

fn collect_errors(
    call_result: Result<ScriptValue, ScriptError>,
    language: Language,
    errors: &mut Vec<ScriptError>,
) {
    match call_result {
        Ok(_) => {}
        Err(e) => {
            errors.push(e.with_context(format!("Event handling for language {language}")));
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
        asset::{AssetApp, AssetId, AssetPlugin, Assets, Handle},
        diagnostic::DiagnosticsPlugin,
        ecs::world::FromWorld,
    };
    use test_utils::make_test_plugin;

    use crate::{
        asset::ScriptAsset,
        bindings::script_value::ScriptValue,
        context::{ContextBuilder, ContextLoadingSettings},
        event::{
            CallbackLabel, IntoCallbackLabel, Recipients, ScriptCallbackEvent, ScriptErrorEvent,
            ScriptEvent,
        },
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
            assert_eq!(a.context_key, b.context_key);
            assert_eq!(a.response, b.response);
        }
    }

    fn setup_app<L: IntoCallbackLabel + 'static>(runtime: TestRuntime) -> App {
        let mut app = App::new();
        // app.add_plugins(bevy::log::LogPlugin::default());
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.init_asset::<ScriptAsset>();
        app.add_event::<ScriptCallbackEvent>();
        app.add_event::<ScriptCallbackResponseEvent>();
        app.add_event::<ScriptErrorEvent>();
        app.add_event::<ScriptEvent>();
        app.insert_resource::<CallbackSettings<TestPlugin>>(CallbackSettings {
            callback_handler: |args, context_key, _, ctxt, _, runtime| {
                ctxt.invocations.extend(args);
                let mut runtime = runtime.invocations.lock();
                runtime.push((context_key.entity(), Some(context_key.script().id())));
                Ok(ScriptValue::Unit)
            },
        });
        app.add_plugins(crate::configure_asset_systems);
        app.add_plugins(crate::configure_asset_systems_for_plugin::<TestPlugin>);
        app.add_systems(Update, event_handler::<L, TestPlugin>);
        app.insert_resource(RuntimeContainer::<TestPlugin> { runtime });
        app.init_resource::<StaticScripts>();
        app.insert_resource(ContextLoadingSettings::<TestPlugin> {
            loader: ContextBuilder {
                load: |_, _, _, _, _| Ok(TestContext::default()),
                reload: |_, _, _, _, _, _| Ok(()),
            },
            context_initializers: vec![],
            context_pre_handling_initializers: vec![],
            emit_responses: false,
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
        let test_script = add_script(&mut app, "");
        let test_entity_id = app
            .world_mut()
            .spawn(ScriptComponent(vec![test_script.clone()]))
            .id();
        app.update();

        app.world_mut().send_event(
            ScriptCallbackEvent::new(
                OnTestCallback::into_callback_label(),
                vec![ScriptValue::String("test_args".into())],
                Recipients::AllScripts,
                None,
            )
            .with_response(),
        );
        app.update();

        assert_response_events(
            app.world_mut(),
            vec![ScriptCallbackResponseEvent::new(
                OnTestCallback::into_callback_label(),
                ScriptAttachment::EntityScript(test_entity_id, test_script.clone()),
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
        let test_script = add_script(&mut app, "");
        app.add_static_script(test_script.clone());
        app.update();

        app.world_mut().send_event(
            ScriptCallbackEvent::new(
                OnTestCallback::into_callback_label(),
                vec![ScriptValue::String("test_args".into())],
                Recipients::AllScripts,
                None,
            )
            .with_response(),
        );
        app.update();

        assert_response_events(
            app.world_mut(),
            vec![ScriptCallbackResponseEvent::new(
                OnTestCallback::into_callback_label(),
                ScriptAttachment::StaticScript(test_script.clone()),
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
        let test_script = add_script(&mut app, "");
        // app.add_static_script(test_script.clone());
        let test_entity_id = app
            .world_mut()
            .spawn(ScriptComponent(vec![test_script.clone()]))
            .id();

        app.world_mut()
            .send_event(ScriptCallbackEvent::new_for_all_scripts(
                OnTestCallback::into_callback_label(),
                vec![ScriptValue::String("test_args".into())],
            ));
        app.update();
        {
            let script_context = app
                .world()
                .get_resource::<ScriptContext<TestPlugin>>()
                .unwrap();

            let test_runtime = app
                .world()
                .get_resource::<RuntimeContainer<TestPlugin>>()
                .unwrap();

            let test_context = script_context
                .get(&ScriptAttachment::EntityScript(
                    test_entity_id,
                    test_script.clone(),
                ))
                .expect("script context");
            let test_context = test_context.lock();
            assert_eq!(
                test_context.invocations,
                vec![ScriptValue::String("test_args".into())]
            );

            let runtime_invocations = test_runtime.runtime.invocations.lock();
            assert_eq!(
                runtime_invocations
                    .iter()
                    .map(|(e, s)| (*e, *s))
                    .collect::<Vec<_>>(),
                vec![
                    // Once for evaluating the script.
                    (Some(test_entity_id), Some(test_script.id())),
                    // Once for the callback.
                    (Some(test_entity_id), Some(test_script.id()))
                ]
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
        let test_script = add_script(&mut app, "");
        let test_script2_id = add_script(&mut app, "wrong");
        // app.add_static_script(test_script.clone());
        let test_entity_id = app
            .world_mut()
            .spawn(ScriptComponent(vec![test_script.clone()]))
            .id();

        let test_entity2_id = app
            .world_mut()
            .spawn(ScriptComponent(vec![test_script2_id.clone()]))
            .id();
        app.update();

        app.world_mut().send_event(ScriptCallbackEvent::new(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_args_script".into())],
            Recipients::ScriptEntity(test_script.id(), test_entity_id),
            None,
        ));

        app.world_mut().send_event(ScriptCallbackEvent::new(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_args_entity".into())],
            Recipients::ScriptEntity(test_script.id(), test_entity_id),
            None,
        ));

        app.update();
        {
            let test_runtime = app
                .world()
                .get_resource::<RuntimeContainer<TestPlugin>>()
                .unwrap();
            let test_runtime = test_runtime.runtime.invocations.lock();

            let script_context = app
                .world()
                .get_resource::<ScriptContext<TestPlugin>>()
                .unwrap();

            let key = ScriptAttachment::EntityScript(test_entity_id, test_script.clone());
            let context_arc = script_context.get(&key).expect("script context");
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
                    .map(|(e, s)| (*e, *s))
                    .collect::<Vec<_>>(),
                vec![
                    // Load 1
                    (Some(test_entity_id), Some(test_script.id())),
                    // Load 2
                    (Some(test_entity2_id), Some(test_script2_id.id())),
                    // Call 1,
                    (Some(test_entity_id), Some(test_script.id())),
                    // Call 2,
                    (Some(test_entity_id), Some(test_script.id())),
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
        let test_script = add_script(&mut app, "");
        app.add_static_script(test_script.clone());
        app.update();

        app.world_mut().send_event(ScriptCallbackEvent::new(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_args_script".into())],
            Recipients::AllScripts,
            None,
        ));

        app.world_mut().send_event(ScriptCallbackEvent::new(
            OnTestCallback::into_callback_label(),
            vec![ScriptValue::String("test_script".into())],
            Recipients::StaticScript(test_script.id()),
            None,
        ));

        app.update();
        {
            let script_context = app
                .world()
                .get_resource::<ScriptContext<TestPlugin>>()
                .unwrap();
            let key = ScriptAttachment::StaticScript(test_script.clone());
            let context_arc = script_context.get(&key).expect("script context");
            let test_context = context_arc.lock();

            assert_eq!(
                test_context.invocations,
                vec![
                    ScriptValue::String("test_args_script".into()),
                    ScriptValue::String("test_script".into())
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

    #[test]
    fn default_script_asset() {
        let default_handle: Handle<ScriptAsset> = Handle::default();
        let handle: Handle<ScriptAsset> = Handle::Weak(AssetId::Uuid {
            uuid: uuid::uuid!("97128bb1-2588-480b-bdc6-87b4adbec477"),
        });
        assert_eq!(default_handle.id(), handle.id());
    }
}
