use crate::{
    event::ScriptErrorEvent,
    hosts::{APIProvider, APIProviders, ScriptHost},
};
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use event::ScriptLoaded;
use systems::script_event_handler;

pub mod asset;
pub mod docs;
pub mod error;
pub mod event;
pub mod hosts;
pub mod systems;
pub mod world;
pub mod prelude {
    // general
    pub use {
        crate::asset::CodeAsset,
        crate::docs::DocFragment,
        crate::error::ScriptError,
        crate::event::{ScriptErrorEvent, ScriptEvent},
        crate::hosts::{
            APIProvider, APIProviders, Recipients, Script, ScriptCollection, ScriptContexts,
            ScriptData, ScriptHost,
        },
        crate::systems::script_event_handler,
        crate::{
            AddScriptApiProvider, AddScriptHost, AddScriptHostHandler, GenDocumentation,
            ScriptingPlugin,
        },
        bevy_event_priority::{
            AddPriorityEvent, PriorityEvent, PriorityEventReader, PriorityEventWriter,
            PriorityEvents, PriorityIterator,
        },
    };
}
pub use bevy_event_priority as events;

#[derive(Default)]
/// Bevy plugin enabling run-time scripting
pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ScriptErrorEvent>();
    }
}

pub trait GenDocumentation {
    fn update_documentation<T: ScriptHost>(&mut self) -> &mut Self;
}

impl GenDocumentation for App {
    /// Updates/Generates documentation and any other artifacts required for script API's. Disabled in optimized builds unless `doc_always` feature is enabled.
    fn update_documentation<T: ScriptHost>(&mut self) -> &mut Self {
        #[cfg(any(debug_assertions, feature = "doc_always"))]
        {
            info!("Generating documentation");
            let w = &mut self.world;
            let providers: &APIProviders<T> = w.resource();
            if let Err(e) = providers.gen_all() {
                error!("{}", e);
            }
            info!("Documentation generated");
        }

        self
    }
}

/// Trait for app builder notation
pub trait AddScriptHost {
    /// registers the given script host with your app,
    /// the given system set will contain systems handling script loading, re-loading, removal etc.
    /// This system set will also send events related to the script lifecycle.
    /// Any systems which need to run the same frame a script is loaded must run after this set.
    fn add_script_host_to_set<T: ScriptHost>(
        &mut self,
        schedule: impl ScheduleLabel,
        set: impl SystemSet,
    ) -> &mut Self;
}

impl AddScriptHost for App {
    fn add_script_host_to_set<T>(
        &mut self,
        schedule: impl ScheduleLabel,
        set: impl SystemSet,
    ) -> &mut Self
    where
        T: ScriptHost,
    {
        T::register_with_app_in_set(self, schedule, set);
        self.init_resource::<T>();
        self.add_event::<ScriptLoaded>();
        self
    }
}

pub trait AddScriptApiProvider {
    fn add_api_provider<T: ScriptHost>(
        &mut self,
        provider: Box<
            dyn APIProvider<
                APITarget = T::APITarget,
                DocTarget = T::DocTarget,
                ScriptContext = T::ScriptContext,
            >,
        >,
    ) -> &mut Self;
}

impl AddScriptApiProvider for App {
    fn add_api_provider<T: ScriptHost>(
        &mut self,
        provider: Box<
            dyn APIProvider<
                APITarget = T::APITarget,
                DocTarget = T::DocTarget,
                ScriptContext = T::ScriptContext,
            >,
        >,
    ) -> &mut Self {
        provider.register_with_app(self);
        let w = &mut self.world;
        let providers: &mut APIProviders<T> = &mut w.resource_mut();
        providers.providers.push(provider);
        self
    }
}

pub trait AddScriptHostHandler {
    /// Enables this script host to handle events with priorities in the range [0,min_prio] (inclusive),
    /// during from within the given set.
    ///
    /// Note: this is identical to adding the script_event_handler system manually, so if you require setting schedules etc, you should use that directly.
    /// ```rust,ignore
    /// self.add_systems(
    ///     MySchedule,
    ///     script_event_handler::<T, MAX, MIN>.in_set(set)
    /// );
    /// ```
    /// Think of handler system sets as a way to run certain types of events at specific points in your engine.
    /// A good example of this is Unity [game loop's](https://docs.unity3d.com/Manual/ExecutionOrder.html) `onUpdate` and `onFixedUpdate`.
    /// FixedUpdate runs *before* any physics while Update runs after physics and input events.
    ///
    /// A similar setup can be achieved by using a separate system set before and after your physics,
    /// then assigning event priorities such that your events are forced to run at a particular system set, for example:
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
    /// The *frequency* of running these events, is controlled by your systems, if the event is not emitted, it cannot be handled.
    /// Of course there is nothing stopping your from emitting a single event type at varying priorities.
    fn add_script_handler_to_set<T: ScriptHost, const MAX: u32, const MIN: u32>(
        &mut self,
        schedule: impl ScheduleLabel,
        set: impl SystemSet,
    ) -> &mut Self;
}

impl AddScriptHostHandler for App {
    fn add_script_handler_to_set<T: ScriptHost, const MAX: u32, const MIN: u32>(
        &mut self,
        schedule: impl ScheduleLabel,
        set: impl SystemSet,
    ) -> &mut Self {
        self.add_systems(schedule, script_event_handler::<T, MAX, MIN>.in_set(set));
        self
    }
}
