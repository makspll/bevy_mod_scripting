//! Contains the logic for handling script callback events
use std::{cell::RefCell, sync::Arc};

use bevy_ecs::{
    message::{MessageCursor, Messages},
    world::WorldId,
};
use bevy_mod_scripting_bindings::{
    CurrentScriptAttachment, InteropError, ScriptValue, WorldExtensions,
};
use bevy_mod_scripting_display::{DisplayProxy, WithTypeInfo};
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_mod_scripting_world::{WorldAccessGuard, WorldGuard};

use crate::{
    IntoScriptPluginParams,
    callbacks::ScriptCallbacks,
    error::ScriptError,
    event::{
        CallbackLabel, IntoCallbackLabel, Recipients, ScriptCallbackEvent,
        ScriptCallbackResponseEvent, ScriptErrorEvent,
    },
    script::ScriptContexts,
};
use {
    bevy_ecs::{
        system::{Local, SystemState},
        world::{Mut, World},
    },
    bevy_log::error,
    parking_lot::Mutex,
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
    /// Locks the given context and calls the handler function in it with the given arguments.
    ///
    /// Callbacks can run other callbacks while their own context is locked. Re-entering a context which is already
    /// running a callback on this thread would deadlock, so this returns an error instead.
    /// It also errors if callbacks are nested more than [`MAX_NESTED_CALLBACK_DEPTH`] levels deep.
    /// Contexts locked by other threads are waited on as usual.
    fn handle(
        args: Vec<ScriptValue>,
        context_key: &ScriptAttachment,
        callback: &CallbackLabel,
        script_ctxt: &Arc<Mutex<P::C>>,
        script_callbacks: ScriptCallbacks<P>,
        world: WorldGuard,
    ) -> Result<ScriptValue, InteropError>;
}

impl<P: IntoScriptPluginParams> ScriptingHandler<P> for P {
    /// Calls the handler function while providing the necessary thread local context
    fn handle(
        args: Vec<ScriptValue>,
        attachment: &ScriptAttachment,
        callback: &CallbackLabel,
        script_ctxt: &Arc<Mutex<P::C>>,
        script_callbacks: ScriptCallbacks<P>,
        world: WorldGuard,
    ) -> Result<ScriptValue, InteropError> {
        let _executing = ExecutingContext::enter(script_ctxt).map_err(|reason| {
            // the guard still carries the attachment of the callback which is running right now
            let caller = world.current_attachment().0;
            reason.into_error(caller.as_ref(), attachment, callback)
        })?;
        let mut script_ctxt = script_ctxt.lock();
        let script_ctxt = &mut *script_ctxt;

        WorldGuard::with_existing_static_guard(world.clone(), |world| {
            // restored afterwards, so that a callback running inside another one leaves the outer callback's attachment intact
            let previous_attachment =
                world.replace_cached_registry(CurrentScriptAttachment(Some(attachment.clone())));
            let world_id = world.id();
            let callbacks = script_callbacks.callbacks.read();
            let result = if let Some(callback) = callbacks
                .get(&(attachment.clone(), callback.to_string()))
                .cloned()
            {
                drop(callbacks);
                callback(args, script_ctxt, world_id)
            } else {
                drop(callbacks);
                Self::handler()(args, attachment, callback, script_ctxt, world_id)
            };
            world.replace_cached_registry(previous_attachment);
            result
        })
    }
}

/// The maximum number of callbacks which can run nested inside each other on one thread,
/// i.e. a callback running another callback, which runs another one and so on.
pub const MAX_NESTED_CALLBACK_DEPTH: usize = 64;

thread_local! {
    /// The contexts currently executing a callback on this thread, innermost last.
    static EXECUTING_CONTEXTS: RefCell<Vec<ExecutingContextAddress>> = const { RefCell::new(Vec::new()) };
}

/// Identifies a context by the address of its shared allocation.
///
/// All clones of an [`Arc`] share one address and different contexts never do, so the address is a free unique id.
/// It can only be reused once the context is freed, which cannot happen while it is executing, as the caller holds a reference to it.
/// It is stored as a plain number, so that contexts of every language fit in one (non-generic) thread local.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExecutingContextAddress(usize);

impl ExecutingContextAddress {
    fn of<C>(context: &Arc<C>) -> Self {
        Self(Arc::as_ptr(context).cast::<()>() as usize)
    }
}

/// Marks a context as executing on this thread until dropped.
struct ExecutingContext;

/// Why a callback could not start running in its context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CannotEnter {
    /// The context is the one running the current (innermost) callback.
    RunningCurrentCallback,
    /// The context is running a callback further up the chain of nested callbacks.
    RunningOuterCallback,
    /// Too many callbacks are nested inside each other.
    TooDeep,
}

impl ExecutingContext {
    /// Marks the given context as executing, unless that would deadlock or nest callbacks too deeply.
    fn enter<C>(context: &Arc<C>) -> Result<Self, CannotEnter> {
        let context = ExecutingContextAddress::of(context);
        EXECUTING_CONTEXTS.with_borrow_mut(|executing| {
            match executing.iter().rposition(|c| *c == context) {
                Some(i) if i + 1 == executing.len() => {
                    return Err(CannotEnter::RunningCurrentCallback);
                }
                Some(_) => return Err(CannotEnter::RunningOuterCallback),
                None => {}
            }
            if executing.len() >= MAX_NESTED_CALLBACK_DEPTH {
                return Err(CannotEnter::TooDeep);
            }
            executing.push(context);
            Ok(Self)
        })
    }
}

impl CannotEnter {
    /// Builds a user facing error, `caller` is the script running the current callback, if any.
    fn into_error(
        self,
        caller: Option<&ScriptAttachment>,
        target: &ScriptAttachment,
        callback: &CallbackLabel,
    ) -> InteropError {
        const QUEUE_HINT: &str = "Use `queue_script_callback` instead, to run it as soon as the current callback finishes.";
        const SHARED_CONTEXT_NOTE: &str = "Note: whether scripts share a context depends on the context policy of the scripting plugin, \
            e.g. with `ContextPolicy::shared()` all scripts share one context, with `per_entity()` all scripts on an entity do, \
            and with `per_script()` all attachments of the same script do.";

        let message = match (self, caller) {
            (CannotEnter::TooDeep, _) => format!(
                "Cannot run callback `{callback}` on script {target}: callbacks are nested more than {MAX_NESTED_CALLBACK_DEPTH} levels deep."
            ),
            (CannotEnter::RunningCurrentCallback, Some(caller)) if caller == target => format!(
                "Cannot run callback `{callback}` on script {target} immediately: it is the script currently running. \
                Call the function directly instead. {QUEUE_HINT}"
            ),
            (CannotEnter::RunningCurrentCallback, Some(caller)) => format!(
                "Cannot run callback `{callback}` on script {target} immediately from script {caller}: \
                both scripts share a script context, which is busy running the current callback. {QUEUE_HINT}\n{SHARED_CONTEXT_NOTE}"
            ),
            (CannotEnter::RunningCurrentCallback, None) => format!(
                "Cannot run callback `{callback}` on script {target} immediately: its script context is busy running the current callback. {QUEUE_HINT}\n{SHARED_CONTEXT_NOTE}"
            ),
            (CannotEnter::RunningOuterCallback, _) => format!(
                "Cannot run callback `{callback}` on script {target} immediately: its script context is already running a callback \
                further up the current chain of callbacks, i.e. the chain would loop back into it. {QUEUE_HINT}\n{SHARED_CONTEXT_NOTE}"
            ),
        };
        InteropError::string(message)
    }
}

impl Drop for ExecutingContext {
    fn drop(&mut self) {
        EXECUTING_CONTEXTS.with_borrow_mut(|executing| executing.pop());
    }
}

/// Passes events with the specified label to the script callback with the same name and runs the callback.
///
/// If any of the resources required for the handler are missing, the system will log this issue and do nothing.
#[allow(deprecated)]
pub fn event_handler<L: IntoCallbackLabel, P: IntoScriptPluginParams>(
    world: &mut World,
    state: &mut SystemState<Local<MessageCursor<ScriptCallbackEvent>>>,
) -> bevy_ecs::error::Result {
    // we wrap the inner event handler, so that we can guarantee that the handler context is released statically
    {
        let script_context = world.get_resource_or_init::<ScriptContexts<P>>().clone();
        let script_callbacks = world.get_resource_or_init::<ScriptCallbacks<P>>().clone();
        let event_cursor = state.get_mut(world)?;
        let cache = WorldAccessGuard::setup_cache(world, CurrentScriptAttachment::default());
        let guard = WorldAccessGuard::new_exclusive(world, cache);
        event_handler_inner::<P>(
            L::into_callback_label(),
            event_cursor,
            script_context,
            script_callbacks,
            guard,
        );
        Ok(())
    }
}

#[profiling::function]
#[allow(deprecated)]
pub(crate) fn event_handler_inner<P: IntoScriptPluginParams>(
    callback_label: CallbackLabel,
    mut event_cursor: Local<MessageCursor<ScriptCallbackEvent>>,
    script_context: ScriptContexts<P>,
    script_callbacks: ScriptCallbacks<P>,
    guard: WorldAccessGuard,
) {
    let mut errors = Vec::default();
    let events: Result<Vec<ScriptCallbackEvent>, InteropError> =
        guard.with_resource(|events: &Messages<ScriptCallbackEvent>| {
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

    let mut events_to_requeue = vec![];

    for event in events.into_iter().filter(|e| {
        e.label == callback_label && e.language.as_ref().is_none_or(|l| l == &P::LANGUAGE)
    }) {
        let recipients = event.recipients.get_recipients(script_context.clone());
        let highly_specific = matches!(
            event.recipients,
            Recipients::ScriptEntity(_, _) | Recipients::StaticScript(_)
        );
        let might_not_have_reached_pipeline_if_new = event.iteration == 0;
        for (attachment, ctxt) in recipients {
            // we don't issue callbacks to scripts which are currently loading/unloading/reloading
            let ctxt = if let Some(ctxt) = ctxt.as_loaded() {
                ctxt
                // due to how the pipeline runs, we might see scripts which have just been
                // loaded, and attached, but the pipeline hasn't run yet
                // we want to give those one frame to trigger
            } else if highly_specific && ctxt.is_loading_or_reloading()
                || might_not_have_reached_pipeline_if_new
            {
                // events with high specificity, have their callbacks re-queued in this case
                // i.e. we don't want `on_update` to queue up, but a directed `on_collision_with_entity` callback will have
                events_to_requeue.push(event.clone().with_incremented_iteration());
                continue;
            } else {
                continue;
            };
            let call_result = P::handle(
                event.args.clone(),
                &attachment,
                &callback_label,
                ctxt,
                script_callbacks.clone(),
                guard.clone(),
            );
            let call_result = call_result.map_err(|e| {
                ScriptError::from(e)
                    .with_script(attachment.script().display())
                    .with_context(format!("callback: {}", event.label))
                    .with_type_info_context(Some("args: "), event.args.clone())
                    .with_language(P::LANGUAGE)
            });

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

    if let Err(err) = guard.with_resource_mut(|mut writer: Mut<Messages<ScriptCallbackEvent>>| {
        writer.write_batch(events_to_requeue);
    }) {
        errors.push(err.into());
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
    let err = world.with_resource_mut(|mut events: Mut<Messages<ScriptCallbackResponseEvent>>| {
        events.write(response);
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
    let err = world.with_resource_mut(|mut error_events: Mut<Messages<ScriptErrorEvent>>| {
        for error in iter {
            error_events.write(ScriptErrorEvent {
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
    mut errors_cursor: Local<MessageCursor<ScriptErrorEvent>>,
) {
    let cache = WorldGuard::setup_cache(world, CurrentScriptAttachment::default());
    let guard = WorldGuard::new_exclusive(world, cache);
    let errors = guard.with_resource(|events: &Messages<ScriptErrorEvent>| {
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

#[cfg(test)]
mod test {
    use ::{
        bevy_app::{App, Plugin},
        bevy_asset::Handle,
        bevy_ecs::entity::Entity,
    };
    use bevy_mod_scripting_world::ThreadWorldContainer;
    use test_utils::make_test_plugin;
    use uuid::Uuid;

    use super::*;
    use crate::config::{GetPluginThreadConfig, ScriptingPluginConfiguration};

    make_test_plugin!(crate);

    fn script(n: u128) -> ScriptAttachment {
        ScriptAttachment::StaticScript(Handle::Uuid(Uuid::from_u128(n), Default::default()))
    }

    fn new_context() -> Arc<Mutex<TestContext>> {
        Arc::new(Mutex::new(TestContext::default()))
    }

    /// The world seen by the currently running callback
    fn thread_world() -> WorldGuard<'static> {
        ThreadWorldContainer
            .try_get_context()
            .expect("no world available inside callback")
            .world
    }

    fn register(
        callbacks: &ScriptCallbacks<TestPlugin>,
        attachment: &ScriptAttachment,
        label: &str,
        callback: impl Fn() -> Result<ScriptValue, InteropError> + Send + Sync + 'static,
    ) {
        callbacks.callbacks.write().insert(
            (attachment.clone(), label.to_owned()),
            Arc::new(move |_, _, _| callback()),
        );
    }

    /// Runs the callback the same way the handler systems do
    fn run(
        context: &Arc<Mutex<TestContext>>,
        attachment: &ScriptAttachment,
        label: &str,
        callbacks: &ScriptCallbacks<TestPlugin>,
    ) -> Result<ScriptValue, InteropError> {
        let mut world = World::new();
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::default());
        let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
        TestPlugin::handle(
            vec![],
            attachment,
            &label.into(),
            context,
            callbacks.clone(),
            guard,
        )
    }

    fn no_contexts_executing() -> bool {
        EXECUTING_CONTEXTS.with_borrow(|executing| executing.is_empty())
    }

    #[test]
    fn nested_callback_restores_world_and_attachment_of_outer_callback() {
        let callbacks = ScriptCallbacks::<TestPlugin>::default();
        let (outer, inner) = (script(1), script(2));
        let inner_context = new_context();

        let expected_inner = inner.clone();
        register(&callbacks, &inner, "inner", move || {
            assert_eq!(
                thread_world().current_attachment().0,
                Some(expected_inner.clone())
            );
            Ok(ScriptValue::Integer(42))
        });

        let (inner_callbacks, expected_outer) = (callbacks.clone(), outer.clone());
        register(&callbacks, &outer, "outer", move || {
            let result = TestPlugin::handle(
                vec![],
                &inner,
                &"inner".into(),
                &inner_context,
                inner_callbacks.clone(),
                thread_world(),
            )?;
            let world = thread_world();
            assert!(
                world.as_unsafe_world_cell().is_ok(),
                "outer callback lost its world"
            );
            assert_eq!(
                world.current_attachment().0,
                Some(expected_outer.clone()),
                "outer callback lost its attachment"
            );
            Ok(result)
        });

        let result = run(&new_context(), &outer, "outer", &callbacks);
        assert!(matches!(result, Ok(ScriptValue::Integer(42))), "{result:?}");
        assert!(no_contexts_executing());
    }

    #[test]
    fn reentering_running_context_errors_instead_of_deadlocking() {
        let callbacks = ScriptCallbacks::<TestPlugin>::default();
        let attachment = script(1);
        let context = new_context();

        let (inner_callbacks, inner_context, inner_attachment) =
            (callbacks.clone(), context.clone(), attachment.clone());
        register(&callbacks, &attachment, "reenter", move || {
            TestPlugin::handle(
                vec![],
                &inner_attachment,
                &"other".into(),
                &inner_context,
                inner_callbacks.clone(),
                thread_world(),
            )
        });

        let result = run(&context, &attachment, "reenter", &callbacks);
        let err = result.expect_err("re-entering a running context should fail");
        assert!(
            err.to_string()
                .contains("it is the script currently running"),
            "{err}"
        );
        assert!(no_contexts_executing());
        // the context is usable again afterwards
        assert!(!context.is_locked());
    }

    #[test]
    fn calling_script_sharing_the_running_context_errors_with_note() {
        let callbacks = ScriptCallbacks::<TestPlugin>::default();
        let (caller, target) = (script(1), script(2));
        let shared_context = new_context();

        let (inner_callbacks, inner_context, inner_target) =
            (callbacks.clone(), shared_context.clone(), target.clone());
        register(&callbacks, &caller, "caller", move || {
            TestPlugin::handle(
                vec![],
                &inner_target,
                &"target".into(),
                &inner_context,
                inner_callbacks.clone(),
                thread_world(),
            )
        });

        let err = run(&shared_context, &caller, "caller", &callbacks)
            .expect_err("calling into a shared, running context should fail");
        let err = err.to_string();
        assert!(err.contains("both scripts share a script context"), "{err}");
        assert!(err.contains("queue_script_callback"), "{err}");
        assert!(err.contains("ContextPolicy::shared()"), "{err}");
        assert!(no_contexts_executing());
    }

    #[test]
    fn callback_chain_looping_back_errors() {
        let callbacks = ScriptCallbacks::<TestPlugin>::default();
        let (a, b) = (script(1), script(2));
        let (context_a, context_b) = (new_context(), new_context());

        // a -> b -> a
        let (inner_callbacks, inner_context, inner_b) =
            (callbacks.clone(), context_b.clone(), b.clone());
        register(&callbacks, &a, "start", move || {
            TestPlugin::handle(
                vec![],
                &inner_b,
                &"bounce".into(),
                &inner_context,
                inner_callbacks.clone(),
                thread_world(),
            )
        });
        let (inner_callbacks, inner_context, inner_a) =
            (callbacks.clone(), context_a.clone(), a.clone());
        register(&callbacks, &b, "bounce", move || {
            TestPlugin::handle(
                vec![],
                &inner_a,
                &"start".into(),
                &inner_context,
                inner_callbacks.clone(),
                thread_world(),
            )
        });

        let err = run(&context_a, &a, "start", &callbacks)
            .expect_err("looping back into a running context should fail");
        assert!(
            err.to_string().contains("further up the current chain"),
            "{err}"
        );
        assert!(no_contexts_executing());
    }

    #[test]
    fn deeply_nested_callbacks_error() {
        let callbacks = ScriptCallbacks::<TestPlugin>::default();
        let attachment = script(1);

        let (inner_callbacks, inner_attachment) = (callbacks.clone(), attachment.clone());
        // every level runs in a fresh context, so only the depth limit can stop this
        register(&callbacks, &attachment, "recurse", move || {
            TestPlugin::handle(
                vec![],
                &inner_attachment,
                &"recurse".into(),
                &new_context(),
                inner_callbacks.clone(),
                thread_world(),
            )
        });

        let result = run(&new_context(), &attachment, "recurse", &callbacks);
        let err = result.expect_err("unbounded recursion should hit the depth limit");
        assert!(err.to_string().contains("nested more than"), "{err}");
        assert!(no_contexts_executing());
    }
}
