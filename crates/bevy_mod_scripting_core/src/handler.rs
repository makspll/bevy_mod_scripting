//! Contains the logic for handling script callback events
use bevy_ecs::world::WorldId;
use bevy_mod_scripting_bindings::{
    InteropError, ScriptValue, ThreadWorldContainer, WorldAccessGuard, WorldContainer, WorldGuard,
};
use bevy_mod_scripting_display::WithTypeInfo;

use crate::extractors::CallContext;
use crate::script::DisplayProxy;
use crate::{
    IntoScriptPluginParams,
    error::ScriptError,
    event::{
        CallbackLabel, IntoCallbackLabel, ScriptCallbackEvent, ScriptCallbackResponseEvent,
        ScriptErrorEvent,
    },
    extractors::WithWorldGuard,
    script::{ScriptAttachment, ScriptContext},
};
use {
    bevy_ecs::{
        event::EventCursor,
        event::Events,
        system::{Local, SystemState},
        world::{Mut, World},
    },
    bevy_log::error,
};

/// A function that handles a callback event
pub type HandlerFn<P> = fn(
    args: Vec<ScriptValue>,
    context_key: &ScriptAttachment,
    callback: &CallbackLabel,
    context: &mut <P as IntoScriptPluginParams>::C,
    world_id: WorldId,
) -> Result<ScriptValue, InteropError>;

/// A utility trait, implemented for all types implementing `IntoScriptPluginParams`.
///
/// Calls the underlying handler function with the provided arguments and context.
/// Implementations will handle the necessary thread local context emplacement and retrieval.
pub trait ScriptingHandler<P: IntoScriptPluginParams> {
    /// Calls the handler function with the given arguments and context
    fn handle(
        args: Vec<ScriptValue>,
        context_key: &ScriptAttachment,
        callback: &CallbackLabel,
        script_ctxt: &mut P::C,
        world: WorldGuard,
    ) -> Result<ScriptValue, InteropError>;
}

impl<P: IntoScriptPluginParams> ScriptingHandler<P> for P {
    /// Calls the handler function while providing the necessary thread local context
    fn handle(
        args: Vec<ScriptValue>,
        context_key: &ScriptAttachment,
        callback: &CallbackLabel,
        script_ctxt: &mut P::C,
        world: WorldGuard,
    ) -> Result<ScriptValue, InteropError> {
        WorldGuard::with_existing_static_guard(world.clone(), |world| {
            let world_id = world.id();
            ThreadWorldContainer.set_world(world)?;
            Self::handler()(args, context_key, callback, script_ctxt, world_id)
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
        let script_context = world.get_resource_or_init::<ScriptContext<P>>().clone();
        let (event_cursor, mut guard) = state.get_mut(world);
        let (guard, _) = guard.get_mut();
        event_handler_inner::<P>(
            L::into_callback_label(),
            event_cursor,
            script_context,
            guard,
        );
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
    script_context: ScriptContext<P>,
    guard: WorldAccessGuard,
) {
    let mut errors = Vec::default();
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
                WithTypeInfo::new_with_info(&err, &guard)
            );
            return;
        }
    };

    for event in events.into_iter().filter(|e| {
        e.label == callback_label && e.language.as_ref().is_none_or(|l| l == &P::LANGUAGE)
    }) {
        let recipients = event.recipients.get_recipients(script_context.clone());

        for (attachment, ctxt) in recipients {
            let mut ctxt = ctxt.lock();
            let call_result = ctxt.call_context_dynamic(
                &attachment,
                &callback_label,
                event.args.clone(),
                guard.clone(),
            );
            let call_result = call_result.map_err(|e| {
                ScriptError::from(e)
                    .with_script(attachment.script().display())
                    .with_context(format!("callback: {}", event.label))
                    .with_type_info_context(Some("args: "), event.args.clone())
                    .with_language(P::LANGUAGE)
            });
            drop(ctxt);

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
            collect_errors(call_result, &mut errors);
        }
    }
    send_script_errors(guard, errors.iter());
}

fn collect_errors(call_result: Result<ScriptValue, ScriptError>, errors: &mut Vec<ScriptError>) {
    match call_result {
        Ok(_) => {}
        Err(e) => {
            errors.push(e);
        }
    }
}

/// Sends a callback response event to the world
pub fn send_callback_response(world: WorldGuard, response: ScriptCallbackResponseEvent) {
    let err = world.with_resource_mut(|mut events: Mut<Events<ScriptCallbackResponseEvent>>| {
        events.send(response);
    });

    if let Err(err) = err {
        error!(
            "Failed to send script callback response: {}",
            WithTypeInfo::new_with_info(&err, &world)
        );
    }
}

/// sends the given errors to the error event channel
pub fn send_script_errors<'e>(
    world: WorldGuard,
    errors: impl IntoIterator<Item = &'e ScriptError>,
) {
    let iter = errors.into_iter();
    let err = world.with_resource_mut(|mut error_events: Mut<Events<ScriptErrorEvent>>| {
        for error in iter {
            error_events.send(ScriptErrorEvent {
                error: error.clone(),
            });
        }
    });

    if let Err(err) = err {
        error!(
            "Failed to send script error events: {}",
            WithTypeInfo::new_with_info(&err, &world)
        );
    }
}

/// A system which receives all script errors and logs them to console
pub fn script_error_logger(
    world: &mut World,
    mut errors_cursor: Local<EventCursor<ScriptErrorEvent>>,
) {
    let guard = WorldGuard::new_exclusive(world);
    let errors = guard.with_resource(|events: &Events<ScriptErrorEvent>| {
        errors_cursor
            .read(events)
            .map(|e| e.error.clone())
            .collect::<Vec<_>>()
    });

    match errors {
        Ok(errors) => {
            for error in errors {
                error!("{}", &WithTypeInfo::new_with_info(&error, &guard))
            }
        }
        Err(err) => {
            error!(
                "Script errors occured but could not be accessed:\n{}",
                WithTypeInfo::new_with_info(&err, &guard)
            );
        }
    }
}
