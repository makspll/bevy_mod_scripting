//! Type erased runners, letting language agnostic code run script callbacks in any scripting language.

use std::{
    collections::VecDeque,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use bevy_ecs::resource::Resource;
use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_script::{Recipients, ScriptAttachment};
use bevy_mod_scripting_world::WorldGuard;
use parking_lot::{Mutex, RwLock};

use crate::{InteropError, ScriptValue};

/// The result of running a callback on each of the scripts it ran on.
pub type ScriptCallbackResults = Vec<(ScriptAttachment, Result<ScriptValue, InteropError>)>;

/// Runs the callback with the given label and arguments immediately, on every recipient handled by one scripting plugin.
///
/// Recipients belonging to other plugins are skipped. Errors only if the callback label is invalid,
/// errors of the callbacks themselves are returned per script.
pub type ScriptCallbackRunner = dyn Fn(
        WorldGuard,
        &Recipients,
        &str,
        Vec<ScriptValue>,
    ) -> Result<ScriptCallbackResults, InteropError>
    + Send
    + Sync
    + 'static;

/// A callback waiting to run once the callback which queued it finishes, see [`AppScriptCallbackRunners::queue`].
#[derive(Clone, Debug)]
pub struct QueuedScriptCallback {
    /// The scripts to run the callback on
    pub recipients: Recipients,
    /// The name of the callback
    pub label: String,
    /// The arguments to pass to the callback
    pub args: Vec<ScriptValue>,
}

/// The [`ScriptCallbackRunner`]s of every scripting plugin, registered by each plugin when it is built,
/// and the callbacks queued to run later.
///
/// Available to script functions via the world guard, so they can run callbacks in scripts of any language,
/// even from script systems without access to the plugins' resources.
#[derive(Clone, Resource, Default)]
pub struct AppScriptCallbackRunners {
    runners: Arc<RwLock<Vec<(Language, Arc<ScriptCallbackRunner>)>>>,
    queue: Arc<CallbackQueue>,
}

#[derive(Default)]
struct CallbackQueue {
    /// Lets the hot path check for queued callbacks without locking, only changed while holding the lock.
    not_empty: AtomicBool,
    callbacks: Mutex<VecDeque<QueuedScriptCallback>>,
}

#[profiling::all_functions]
impl AppScriptCallbackRunners {
    /// Registers the runner for the given language, replacing the previous one if there was one.
    pub fn register(&self, language: Language, runner: Arc<ScriptCallbackRunner>) {
        let mut runners = self.runners.write();
        match runners.iter_mut().find(|(l, _)| *l == language) {
            Some((_, existing)) => *existing = runner,
            None => runners.push((language, runner)),
        }
    }

    /// Runs the callback immediately on the given recipients, in every registered language.
    pub fn run(
        &self,
        world: WorldGuard,
        recipients: &Recipients,
        label: &str,
        args: Vec<ScriptValue>,
    ) -> Result<ScriptCallbackResults, InteropError> {
        // callbacks can run other callbacks, so the lock must not be held while they run
        let runners = self
            .runners
            .read()
            .iter()
            .map(|(_, runner)| runner.clone())
            .collect::<Vec<_>>();

        let mut results = Vec::new();
        for runner in runners {
            results.extend(runner(world.clone(), recipients, label, args.clone())?);
        }
        Ok(results)
    }

    /// Runs the callback immediately on the given recipients, like [`Self::run`], except for scripts whose context is busy
    /// running another callback: for those the callback is queued, see [`Self::queue`], and runs as soon as it can.
    ///
    /// Returns the results of the callbacks which ran.
    pub fn run_or_queue(
        &self,
        world: WorldGuard,
        recipients: &Recipients,
        label: &str,
        args: Vec<ScriptValue>,
    ) -> Result<ScriptCallbackResults, InteropError> {
        let mut results = self.run(world, recipients, label, args.clone())?;
        results.retain(|(attachment, result)| match result {
            Err(e) if e.is_context_busy() => {
                self.queue(QueuedScriptCallback {
                    recipients: Recipients::from(attachment.clone()),
                    label: label.to_owned(),
                    args: args.clone(),
                });
                false
            }
            _ => true,
        });
        Ok(results)
    }

    /// Queues a callback to run as soon as the outermost running callback finishes,
    /// or on the next run of the system running leftover queued callbacks.
    ///
    /// Queued callbacks run in the order they were queued, and can queue more callbacks themselves.
    pub fn queue(&self, callback: QueuedScriptCallback) {
        let mut callbacks = self.queue.callbacks.lock();
        callbacks.push_back(callback);
        self.queue.not_empty.store(true, Ordering::Release);
    }

    /// Whether any callbacks are queued, cheap enough to call after every callback.
    pub fn has_queued(&self) -> bool {
        self.queue.not_empty.load(Ordering::Acquire)
    }

    /// Takes the next queued callback, in the order they were queued.
    pub fn pop_queued(&self) -> Option<QueuedScriptCallback> {
        let mut callbacks = self.queue.callbacks.lock();
        let next = callbacks.pop_front();
        self.queue
            .not_empty
            .store(!callbacks.is_empty(), Ordering::Release);
        next
    }
}

#[cfg(test)]
mod test {
    use bevy_asset::Handle;
    use bevy_ecs::world::World;

    use super::*;
    use crate::{CurrentScriptAttachment, WorldAccessGuard, WorldExtensions};

    fn runner_answering(value: i64) -> Arc<ScriptCallbackRunner> {
        Arc::new(move |_, _, _, _| {
            Ok(vec![(
                ScriptAttachment::StaticScript(Handle::default()),
                Ok(ScriptValue::Integer(value)),
            )])
        })
    }

    fn run(runners: &AppScriptCallbackRunners) -> Vec<i64> {
        let mut world = World::new();
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
        runners
            .run(guard, &Recipients::AllScripts, "ping", vec![])
            .unwrap()
            .into_iter()
            .map(|(_, result)| match result {
                Ok(ScriptValue::Integer(n)) => n,
                other => panic!("unexpected result: {other:?}"),
            })
            .collect()
    }

    #[test]
    fn queued_callbacks_are_taken_in_order() {
        let runners = AppScriptCallbackRunners::default();
        assert!(!runners.has_queued());

        for label in ["first", "second"] {
            runners.queue(QueuedScriptCallback {
                recipients: Recipients::AllScripts,
                label: label.to_owned(),
                args: vec![],
            });
        }
        assert!(runners.has_queued());
        assert_eq!(
            runners.pop_queued().map(|c| c.label).as_deref(),
            Some("first")
        );
        assert!(runners.has_queued());
        assert_eq!(
            runners.pop_queued().map(|c| c.label).as_deref(),
            Some("second")
        );
        assert!(!runners.has_queued());
        assert!(runners.pop_queued().is_none());
    }

    #[test]
    fn runs_every_language_and_replaces_runner_of_same_language() {
        let runners = AppScriptCallbackRunners::default();
        runners.register(Language::Lua, runner_answering(1));
        runners.register(Language::Rhai, runner_answering(2));
        assert_eq!(run(&runners), vec![1, 2]);

        runners.register(Language::Lua, runner_answering(3));
        assert_eq!(run(&runners), vec![3, 2]);
    }
}
