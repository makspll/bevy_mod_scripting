#![doc=include_str!("../../readme.md")]

use bevy::{prelude::*, ecs::schedule::IntoRunCriteria};

pub mod hosts;
pub mod error;


pub use bevy_event_priority as events;
pub use {hosts::*,error::*};

#[derive(Default)]
/// Bevy plugin enabling run-time scripting
pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {}
}

/// An error coming from a script
pub struct ScriptErrorEvent {
    err : ScriptError
}

/// Trait for app builder notation
pub trait AddScriptHost {
    /// registers the given script host with your app
    fn add_script_host<T: ScriptHost, S: StageLabel>(&mut self, stage: S) -> &mut Self;
}

impl AddScriptHost for App {
    fn add_script_host<T: ScriptHost, S: StageLabel>(&mut self, stage: S) -> &mut Self {
        T::register_with_app(self, stage);
        self
    }
}

pub trait AddScriptHostHandler {
    /// Enables this script host to handle events with priorities in the range [0,min_prio] (inclusive),
    /// during the runtime of the given stage.
    ///
    /// Think of handler stages as a way to run certain types of events at various points in your engine.
    /// A good example of this is Unity [game loop's](https://docs.unity3d.com/Manual/ExecutionOrder.html) `onUpdate` and `onFixedUpdate`.
    /// FixedUpdate runs *before* any physics while Update runs after physics and input events.
    ///
    /// A similar setup can be achieved by using a separate stage before and after your physics,
    /// then assigning event priorities such that your events are forced to run at a particular stage, for example:
    ///
    /// PrePhysics: min_prio = 1
    /// PostPhysics: min_prio = 4
    ///
    /// | Priority | Handler     | Event        |
    /// | -------- | ----------- | ------------ |
    /// | 0        | PrePhysics  | Start        |
    /// | 1        | PrePhysics  | FixedUpdate  |
    /// | 2        | PostPhysics | OnCollision  |
    /// | 3        | PostPhysics | OnMouse      |
    /// | 4        | PostPhysics | Update       |
    ///
    /// The *frequency* of running these events, is controlled by your systems, if the event is not emitted, it cannot not handled.
    /// Of course there is nothing stopping your from emitting a single event type at varying priorities.
    fn add_script_handler_stage<T: ScriptHost, S: StageLabel, const MAX: u32, const MIN: u32>(
        &mut self,
        stage: S,
    ) -> &mut Self;

    /// Like `add_script_handler_stage` but with additional run criteria
    fn add_script_handler_stage_with_criteria<
        T: ScriptHost,
        S: StageLabel,
        M,
        C: IntoRunCriteria<M>,
        const MAX: u32,
        const MIN: u32,
    >(
        &mut self,
        stage: S,
        criteria: C,
    ) -> &mut Self;
}

impl AddScriptHostHandler for App {
    fn add_script_handler_stage<T: ScriptHost, S: StageLabel, const MAX: u32, const MIN: u32>(
        &mut self,
        stage: S,
    ) -> &mut Self {
        self.add_system_to_stage(
            stage,
            script_event_handler::<T, MAX, MIN>
                .exclusive_system()
                .at_end(),
        );
        self
    }

    fn add_script_handler_stage_with_criteria<
        T: ScriptHost,
        S: StageLabel,
        M,
        C: IntoRunCriteria<M>,
        const MAX: u32,
        const MIN: u32,
    >(
        &mut self,
        stage: S,
        criteria: C,
    ) -> &mut Self {
        self.add_system_to_stage(
            stage,
            script_event_handler::<T, MAX, MIN>
                .exclusive_system()
                .at_end()
                .with_run_criteria(criteria),
        );
        self
    }
}