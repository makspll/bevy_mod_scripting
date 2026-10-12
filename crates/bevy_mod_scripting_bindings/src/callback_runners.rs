//! Type erased runners, letting language agnostic code run script callbacks in any scripting language.

use std::sync::Arc;

use bevy_ecs::resource::Resource;
use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_script::{Recipients, ScriptAttachment};
use bevy_mod_scripting_world::WorldGuard;
use parking_lot::RwLock;

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

/// The [`ScriptCallbackRunner`]s of every scripting plugin, registered by each plugin when it is built.
///
/// Available to script functions via the world guard, so they can run callbacks in scripts of any language,
/// even from script systems without access to the plugins' resources.
#[derive(Clone, Resource, Default)]
pub struct AppScriptCallbackRunners(Arc<RwLock<Vec<(Language, Arc<ScriptCallbackRunner>)>>>);

#[profiling::all_functions]
impl AppScriptCallbackRunners {
    /// Registers the runner for the given language, replacing the previous one if there was one.
    pub fn register(&self, language: Language, runner: Arc<ScriptCallbackRunner>) {
        let mut runners = self.0.write();
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
            .0
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
    fn runs_every_language_and_replaces_runner_of_same_language() {
        let runners = AppScriptCallbackRunners::default();
        runners.register(Language::Lua, runner_answering(1));
        runners.register(Language::Rhai, runner_answering(2));
        assert_eq!(run(&runners), vec![1, 2]);

        runners.register(Language::Lua, runner_answering(3));
        assert_eq!(run(&runners), vec![3, 2]);
    }
}
