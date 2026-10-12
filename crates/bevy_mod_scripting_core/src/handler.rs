//! Contains the logic for handling script callback events
use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use bevy_ecs::{
    message::{MessageCursor, Messages},
    world::WorldId,
};
use bevy_mod_scripting_bindings::{
    AppScriptCallbackRunners, CurrentScriptAttachment, InteropError, ScriptCallbackResults,
    ScriptValue, WorldExtensions,
};
use bevy_mod_scripting_display::{DisplayProxy, WithTypeInfo};
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_mod_scripting_world::{WorldAccessGuard, WorldGuard};

use crate::{
    IntoScriptPluginParams,
    callbacks::ScriptCallbacks,
    error::ScriptError,
    event::{
        CallbackLabel, GetRecipients, IntoCallbackLabel, Recipients, ScriptCallbackMessage,
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
    ///
    /// Once the outermost callback on this thread finishes, the callbacks queued in the meantime are run, see [`run_queued_callbacks`].
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
        let result = {
            let _executing = ExecutingContext::enter(script_ctxt).map_err(|reason| {
                // the guard still carries the attachment of the callback which is running right now
                let caller = world.current_attachment().0;
                reason.into_error(caller.as_ref(), attachment, callback)
            })?;
            let mut script_ctxt = script_ctxt.lock();
            let script_ctxt = &mut *script_ctxt;
            WorldGuard::with_existing_static_guard(world.clone(), |world| {
                // restored afterwards, so that a callback running inside another one leaves the outer callback's attachment intact
                let previous_attachment = world
                    .replace_cached_registry(CurrentScriptAttachment(Some(attachment.clone())));
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
        };

        // the context is unlocked again, so once the outermost callback is done
        // the callbacks it queued can run, including ones for its own script
        if ExecutingContext::none_running() {
            run_queued_callbacks(world);
        }
        result
    }
}

/// Immediately runs the callback on every loaded script of plugin `P` among the given recipients.
///
/// This is the [`ScriptCallbackRunner`](bevy_mod_scripting_bindings::ScriptCallbackRunner) each [`crate::ScriptingPlugin`] registers.
/// Like the event handler, broadcast recipients skip scripts which are currently loading or unloading,
/// while a specific script which is not loaded is reported as an error for that script.
pub fn run_callback_on_recipients<P: IntoScriptPluginParams>(
    script_contexts: &ScriptContexts<P>,
    script_callbacks: &ScriptCallbacks<P>,
    world: WorldGuard,
    recipients: &Recipients,
    label: &str,
    args: Vec<ScriptValue>,
) -> Result<ScriptCallbackResults, InteropError> {
    let label = CallbackLabel::new(label)
        .ok_or_else(|| InteropError::string(format!("`{label}` is not a valid callback name")))?;
    let specific = matches!(
        recipients,
        Recipients::ScriptEntity(_, _) | Recipients::StaticScript(_)
    );

    Ok(recipients
        .get_recipients(script_contexts.clone())
        .into_iter()
        .filter_map(|(attachment, context)| {
            let result = match context.as_loaded() {
                Some(context) => P::handle(
                    args.clone(),
                    &attachment,
                    &label,
                    context,
                    script_callbacks.clone(),
                    world.clone(),
                ),
                None if specific => Err(InteropError::string(format!(
                    "Cannot run callback `{label}` on script {attachment}: the script is not loaded, its context is: {context}"
                ))),
                None => return None,
            };
            Some((attachment, result))
        })
        .collect())
}

/// The maximum number of queued callbacks [`run_queued_callbacks`] runs in one go.
///
/// Queued callbacks can queue more callbacks, this stops them from queueing each other endlessly.
pub const MAX_QUEUED_CALLBACKS_PER_RUN: usize = 1024;

thread_local! {
    /// Whether this thread is currently running queued callbacks.
    static RUNNING_QUEUED_CALLBACKS: Cell<bool> = const { Cell::new(false) };
}

/// Runs the callbacks queued via [`AppScriptCallbackRunners::queue`] in the order they were queued,
/// including the ones they queue in turn. Errors of the callbacks are sent as [`ScriptErrorEvent`]s.
///
/// Does nothing:
/// - if the guard does not have exclusive access to the world, as the callbacks may need any part of it,
///   e.g. inside non-exclusive script systems. [`run_queued_callbacks_system`] runs those later.
/// - if this thread is already running queued callbacks, the outer run picks up newly queued ones.
///
/// Stops after [`MAX_QUEUED_CALLBACKS_PER_RUN`] callbacks with an error, leaving the rest for the next run.
pub fn run_queued_callbacks(world: WorldGuard) {
    let runners = {
        let runners = world.script_callback_runners();
        if !runners.has_queued() {
            return;
        }
        runners.clone()
    };
    if !world.is_exclusive() || RUNNING_QUEUED_CALLBACKS.get() {
        return;
    }

    let mut errors = Vec::new();
    let running = RunningQueuedCallbacks::start();
    for _ in 0..MAX_QUEUED_CALLBACKS_PER_RUN {
        let Some(queued) = runners.pop_queued() else {
            break;
        };
        match runners.run(
            world.clone(),
            &queued.recipients,
            &queued.label,
            queued.args,
        ) {
            Ok(results) => errors.extend(results.into_iter().filter_map(|(attachment, result)| {
                result.err().map(|err| {
                    ScriptError::from(err)
                        .with_script(attachment.script().display())
                        .with_context(format!("in queued callback: {}", queued.label))
                })
            })),
            Err(err) => errors.push(
                ScriptError::from(err)
                    .with_context(format!("in queued callback: {}", queued.label)),
            ),
        }
    }
    drop(running);

    if runners.has_queued() {
        errors.push(ScriptError::from(InteropError::string(format!(
            "Stopped running queued callbacks after {MAX_QUEUED_CALLBACKS_PER_RUN}, the rest will run later. \
            Callbacks may be queueing each other endlessly."
        ))));
    }
    send_script_errors(world, errors.iter());
}

/// Marks this thread as running queued callbacks until dropped.
struct RunningQueuedCallbacks;

impl RunningQueuedCallbacks {
    fn start() -> Self {
        RUNNING_QUEUED_CALLBACKS.set(true);
        Self
    }
}

impl Drop for RunningQueuedCallbacks {
    fn drop(&mut self) {
        RUNNING_QUEUED_CALLBACKS.set(false);
    }
}

/// Runs callbacks which are still queued, e.g. ones queued from script systems without exclusive world access,
/// or ones left over after [`MAX_QUEUED_CALLBACKS_PER_RUN`] was reached.
pub fn run_queued_callbacks_system(world: &mut World) {
    if !world
        .get_resource::<AppScriptCallbackRunners>()
        .is_some_and(AppScriptCallbackRunners::has_queued)
    {
        return;
    }
    let cache = WorldAccessGuard::setup_cache(world, CurrentScriptAttachment::empty());
    run_queued_callbacks(WorldAccessGuard::new_exclusive(world, cache));
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

impl ExecutingContext {
    /// Whether no callback is running on this thread.
    fn none_running() -> bool {
        EXECUTING_CONTEXTS.with_borrow(|executing| executing.is_empty())
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
    state: &mut SystemState<Local<MessageCursor<ScriptCallbackMessage>>>,
) -> bevy_ecs::error::Result {
    // we wrap the inner event handler, so that we can guarantee that the handler context is released statically
    {
        let script_context = world.get_resource_or_init::<ScriptContexts<P>>().clone();
        let script_callbacks = world.get_resource_or_init::<ScriptCallbacks<P>>().clone();
        let event_cursor = state.get_mut(world)?;
        let cache = WorldAccessGuard::setup_cache(world, CurrentScriptAttachment::empty());
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
    mut event_cursor: Local<MessageCursor<ScriptCallbackMessage>>,
    script_context: ScriptContexts<P>,
    script_callbacks: ScriptCallbacks<P>,
    guard: WorldAccessGuard,
) {
    let mut errors = Vec::default();
    let events: Result<Vec<ScriptCallbackMessage>, InteropError> =
        guard.with_resource(|events: &Messages<ScriptCallbackMessage>| {
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

    if let Err(err) = guard.with_resource_mut(|mut writer: Mut<Messages<ScriptCallbackMessage>>| {
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
    let cache = WorldGuard::setup_cache(world, CurrentScriptAttachment::empty());
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
    use bevy_ecs::reflect::AppTypeRegistry;
    use bevy_mod_scripting_bindings::QueuedScriptCallback;
    use bevy_mod_scripting_world::AccessByteSet;

    use crate::{
        config::{GetPluginThreadConfig, ScriptingPluginConfiguration},
        script::{Context, ContextPolicy},
    };

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

    /// Runs `f` with an exclusive world guard, like the handler systems have
    fn with_guard<O>(f: impl FnOnce(WorldGuard) -> O) -> O {
        let mut world = World::new();
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        f(WorldAccessGuard::new_exclusive(&mut world, cache))
    }

    /// Runs the callback the same way the handler systems do
    fn run(
        context: &Arc<Mutex<TestContext>>,
        attachment: &ScriptAttachment,
        label: &str,
        callbacks: &ScriptCallbacks<TestPlugin>,
    ) -> Result<ScriptValue, InteropError> {
        with_guard(|guard| {
            TestPlugin::handle(
                vec![],
                attachment,
                &label.into(),
                context,
                callbacks.clone(),
                guard,
            )
        })
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

    /// Per script contexts with scripts 1 and 2 loaded and script 3 still loading.
    /// Each script answers `ping` with its own number.
    fn make_contexts() -> (ScriptContexts<TestPlugin>, ScriptCallbacks<TestPlugin>) {
        let contexts = ScriptContexts::<TestPlugin>::new(ContextPolicy::per_script());
        {
            let mut contexts = contexts.write();
            contexts.insert(script(1), new_context().into()).unwrap();
            contexts.insert(script(2), new_context().into()).unwrap();
            contexts.insert(script(3), Context::Loading).unwrap();
        }
        let callbacks = ScriptCallbacks::<TestPlugin>::default();
        for n in 1..=3 {
            register(&callbacks, &script(n), "ping", move || {
                Ok(ScriptValue::Integer(n as i64))
            });
        }
        (contexts, callbacks)
    }

    fn run_on_recipients(
        recipients: Recipients,
        label: &str,
    ) -> Result<ScriptCallbackResults, InteropError> {
        let (contexts, callbacks) = make_contexts();
        with_guard(|guard| {
            run_callback_on_recipients(&contexts, &callbacks, guard, &recipients, label, vec![])
        })
    }

    #[test]
    fn broadcast_runs_loaded_scripts_and_skips_loading_ones() {
        let results = run_on_recipients(Recipients::AllScripts, "ping").unwrap();
        let mut values = results
            .into_iter()
            .map(|(_, result)| match result {
                Ok(ScriptValue::Integer(n)) => n,
                other => panic!("unexpected result: {other:?}"),
            })
            .collect::<Vec<_>>();
        values.sort();
        assert_eq!(values, vec![1, 2]);
    }

    #[test]
    fn specific_script_which_is_not_loaded_errors() {
        let results =
            run_on_recipients(Recipients::StaticScript(script(3).script()), "ping").unwrap();
        assert_eq!(results.len(), 1);
        let (attachment, result) = &results[0];
        assert_eq!(attachment, &script(3));
        let err = result.as_ref().expect_err("script 3 is still loading");
        assert!(err.to_string().contains("not loaded"), "{err}");
    }

    #[test]
    fn specific_script_of_another_plugin_is_skipped() {
        let results =
            run_on_recipients(Recipients::StaticScript(script(99).script()), "ping").unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn invalid_callback_label_errors() {
        let err = run_on_recipients(Recipients::AllScripts, "not a label")
            .expect_err("label with spaces is invalid");
        assert!(
            err.to_string().contains("not a valid callback name"),
            "{err}"
        );
    }

    #[test]
    fn plugin_registers_runner_reachable_from_world_guard() {
        let mut app = App::default();
        app.add_plugins((bevy_asset::AssetPlugin::default(), TestPlugin::default()));
        app.finish();

        let world = app.world_mut();
        let context = new_context();
        world
            .resource::<ScriptContexts<TestPlugin>>()
            .write()
            .insert(script(1), context.into())
            .unwrap();
        register(
            world.resource::<ScriptCallbacks<TestPlugin>>(),
            &script(1),
            "ping",
            || Ok(ScriptValue::Integer(1)),
        );

        let cache = WorldAccessGuard::setup_cache(world, CurrentScriptAttachment::empty());
        let guard = WorldAccessGuard::new_exclusive(world, cache);
        let runners = guard.script_callback_runners().clone();
        let results = runners
            .run(
                guard,
                &Recipients::StaticScript(script(1).script()),
                "ping",
                vec![],
            )
            .unwrap();
        assert_eq!(results.len(), 1);
        assert!(matches!(results[0].1, Ok(ScriptValue::Integer(1))));
    }

    /// An app with the test plugin and loaded static scripts `1..=count`, each in its own context
    fn make_app(count: u128) -> App {
        let mut app = App::default();
        app.add_plugins((bevy_asset::AssetPlugin::default(), TestPlugin::default()));
        app.add_message::<ScriptErrorEvent>();
        app.finish();
        let contexts = app.world().resource::<ScriptContexts<TestPlugin>>().clone();
        let mut contexts = contexts.write();
        for n in 1..=count {
            contexts.insert(script(n), new_context().into()).unwrap();
        }
        drop(contexts);
        app
    }

    fn app_callbacks(app: &App) -> ScriptCallbacks<TestPlugin> {
        app.world()
            .resource::<ScriptCallbacks<TestPlugin>>()
            .clone()
    }

    /// The context of script `n` and the callbacks, to run callbacks with [`ScriptingHandler::handle`]
    fn loaded_context(
        world: &World,
        n: u128,
    ) -> (Arc<Mutex<TestContext>>, ScriptCallbacks<TestPlugin>) {
        let context = world
            .resource::<ScriptContexts<TestPlugin>>()
            .read()
            .get_context(&script(n))
            .and_then(|c| c.as_loaded().cloned())
            .expect("script is loaded");
        let callbacks = world.resource::<ScriptCallbacks<TestPlugin>>().clone();
        (context, callbacks)
    }

    /// Runs the callback on script `n` with an exclusive guard, as the handler systems do
    fn run_in_app_exclusive(
        app: &mut App,
        n: u128,
        label: &str,
    ) -> Result<ScriptValue, InteropError> {
        let world = app.world_mut();
        let (context, callbacks) = loaded_context(world, n);
        let cache = WorldAccessGuard::setup_cache(world, CurrentScriptAttachment::empty());
        let guard = WorldAccessGuard::new_exclusive(world, cache);
        TestPlugin::handle(
            vec![],
            &script(n),
            &label.into(),
            &context,
            callbacks,
            guard,
        )
    }

    /// Queues `label` on script `n`, from inside a running callback
    fn queue(n: u128, label: &str) {
        thread_world()
            .script_callback_runners()
            .queue(QueuedScriptCallback {
                recipients: Recipients::StaticScript(script(n).script()),
                label: label.to_owned(),
                args: vec![],
            });
    }

    fn script_errors(app: &App) -> Vec<String> {
        app.world()
            .resource::<Messages<ScriptErrorEvent>>()
            .iter_current_update_messages()
            .map(|e| e.error.to_string())
            .collect()
    }

    type Log = Arc<Mutex<Vec<&'static str>>>;

    fn log_entry(log: &Log, entry: &'static str) {
        log.lock().push(entry);
    }

    #[test]
    fn queued_callbacks_run_after_the_outermost_callback_finishes() {
        let mut app = make_app(2);
        let callbacks = app_callbacks(&app);
        let log = Log::default();

        let l = log.clone();
        register(&callbacks, &script(1), "outer", move || {
            log_entry(&l, "outer start");
            queue(2, "queued");
            log_entry(&l, "outer end");
            Ok(ScriptValue::Unit)
        });
        let l = log.clone();
        register(&callbacks, &script(2), "queued", move || {
            log_entry(&l, "queued");
            // back into the script which queued us, which is fine once it finished
            queue(1, "own");
            Ok(ScriptValue::Unit)
        });
        let l = log.clone();
        register(&callbacks, &script(1), "own", move || {
            log_entry(&l, "own");
            Ok(ScriptValue::Unit)
        });

        run_in_app_exclusive(&mut app, 1, "outer").unwrap();
        assert_eq!(
            *log.lock(),
            vec!["outer start", "outer end", "queued", "own"]
        );
        assert_eq!(script_errors(&app), Vec::<String>::new());
        assert!(
            !app.world()
                .resource::<AppScriptCallbackRunners>()
                .has_queued()
        );
    }

    #[test]
    fn endlessly_queued_callbacks_stop_at_the_limit() {
        let mut app = make_app(1);
        let callbacks = app_callbacks(&app);
        let count = Arc::new(Mutex::new(0usize));

        let c = count.clone();
        register(&callbacks, &script(1), "ping", move || {
            *c.lock() += 1;
            queue(1, "ping");
            Ok(ScriptValue::Unit)
        });

        run_in_app_exclusive(&mut app, 1, "ping").unwrap();
        // the first call plus the limit of queued ones
        assert_eq!(*count.lock(), 1 + MAX_QUEUED_CALLBACKS_PER_RUN);
        assert!(
            app.world()
                .resource::<AppScriptCallbackRunners>()
                .has_queued()
        );
        let errors = script_errors(&app);
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert!(
            errors[0].contains("Stopped running queued callbacks"),
            "{errors:?}"
        );
    }

    #[test]
    fn non_exclusive_guards_leave_queued_callbacks_for_the_system() {
        let mut app = make_app(2);
        let callbacks = app_callbacks(&app);
        let log = Log::default();

        let l = log.clone();
        register(&callbacks, &script(1), "outer", move || {
            log_entry(&l, "outer");
            queue(2, "queued");
            Ok(ScriptValue::Unit)
        });
        let l = log.clone();
        register(&callbacks, &script(2), "queued", move || {
            log_entry(&l, "queued");
            Ok(ScriptValue::Unit)
        });

        let world = app.world_mut();
        let (context, callbacks) = loaded_context(world, 1);
        let cache = WorldAccessGuard::setup_cache(world, CurrentScriptAttachment::empty());
        let type_registry = world.resource::<AppTypeRegistry>().0.clone();
        // safety: the guard gives out no access at all
        let guard = unsafe {
            WorldAccessGuard::new_non_exclusive(
                world.as_unsafe_world_cell(),
                AccessByteSet::from_allowed_list(&[]),
                type_registry,
                cache,
            )
        };
        TestPlugin::handle(
            vec![],
            &script(1),
            &"outer".into(),
            &context,
            callbacks,
            guard,
        )
        .unwrap();
        assert_eq!(*log.lock(), vec!["outer"]);

        run_queued_callbacks_system(world);
        assert_eq!(*log.lock(), vec!["outer", "queued"]);
    }
}
