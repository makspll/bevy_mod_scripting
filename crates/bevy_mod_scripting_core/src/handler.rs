//! Contains the logic for handling script callback events
use crate::{
    bindings::{
        pretty_print::DisplayWithWorld, script_value::ScriptValue, ThreadWorldContainer,
        WorldAccessGuard, WorldContainer, WorldGuard,
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
        event::EventCursor,
        resource::Resource,
        system::{Local, SystemState},
        world::{Mut, World},
    },
    log::error,
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
    state: &mut EventHandlerSystemState,
) {
    // we wrap the inner event handler, so that we can guarantee that the handler context is released statically
    {
        let handler_ctxt = HandlerContext::<P>::yoink(world);
        let (event_cursor, mut guard) = state.get_mut(world);
        let (guard, _) = guard.get_mut();
        let handler_ctxt =
            event_handler_inner::<P>(L::into_callback_label(), event_cursor, handler_ctxt, guard);
        handler_ctxt.release(world);
    }
}

type EventHandlerSystemState<'w, 's> = SystemState<(
    Local<'s, EventCursor<ScriptCallbackEvent>>,
    WithWorldGuard<'w, 's, ()>,
)>;

#[profiling::function]
#[allow(deprecated)]
pub(crate) fn event_handler_inner<P: IntoScriptPluginParams>(
    callback_label: CallbackLabel,
    mut event_cursor: Local<EventCursor<ScriptCallbackEvent>>,
    handler_ctxt: HandlerContext<P>,
    guard: WorldAccessGuard,
) -> HandlerContext<P> {
    let mut errors = Vec::default();
    // let events = guard.with_resour events.read().cloned().collect::<Vec<_>>();
    let events = guard.with_resource(|events: &Events<ScriptCallbackEvent>| {
        event_cursor
            .read(events)
            .filter(|e| e.label == callback_label)
            .cloned()
            .collect::<Vec<_>>()
    });

    let events = match events {
        Ok(events) => events,
        Err(err) => {
            error!(
                "Failed to read script callback events: {}",
                err.display_with_world(guard)
            );
            return handler_ctxt;
        }
    };

    for event in events.into_iter().filter(|e| {
        e.label == callback_label && e.language.as_ref().is_none_or(|l| l == &P::LANGUAGE)
    }) {
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
                        P::LANGUAGE,
                    ),
                );
            }
            collect_errors(call_result, P::LANGUAGE, &mut errors);
        }
    }
    handle_script_errors(guard, errors.into_iter());
    return handler_ctxt;
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
