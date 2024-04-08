use crate::{
    event::ScriptErrorEvent,
    hosts::{APIProvider, APIProviders, ScriptHost},
};
use allocator::ReflectAllocator;
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use event::ScriptLoaded;
use systems::{garbage_collector, script_event_handler};

pub mod allocator;
pub mod asset;
pub mod bindings;
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
        app.add_event::<ScriptErrorEvent>()
            .init_resource::<ReflectAllocator>()
            .add_systems(PostUpdate, garbage_collector);
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
    ///
    /// Note: any systems which need to run the same frame a script is loaded must run after this set.
    fn add_script_host<T: ScriptHost>(&mut self, schedule: impl ScheduleLabel) -> &mut Self;

    /// Similar to `add_script_host` but allows you to specify a system set to add the script host to.
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

    fn add_script_host<T>(&mut self, schedule: impl ScheduleLabel) -> &mut Self
    where
        T: ScriptHost,
    {
        T::register_with_app(self, schedule);
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
    /// Note: this is identical to adding the script_event_handler system manually, so if you require more complex setup, you can use the following:
    /// ```rust,ignore
    /// self.add_systems(
    ///     MySchedule,
    ///     script_event_handler::<T, MAX, MIN>
    /// );
    /// ```
    ///
    /// Think of event handler systems as event sinks, which collect and "unpack" the instructions in each event every frame.
    /// Because events are also prioritised, you can enforce a particular order of execution for your events (within each frame)
    /// regardless of where they were fired from.
    ///
    /// A good example of this is Unity [game loop's](https://docs.unity3d.com/Manual/ExecutionOrder.html) `onUpdate` and `onFixedUpdate`.
    /// FixedUpdate runs *before* any physics while Update runs after physics and input events.
    ///
    /// In this crate you can achieve this by using a separate system set before and after your physics,
    /// then assigning event priorities such that your events are forced to run at the points you want them to, for example:
    ///
    /// PrePhysics priority range [0,1]
    /// PostPhysics priority range [2,4]
    ///
    /// | Priority | Handler     | Event         |
    /// | -------- | ----------- | ------------  |
    /// | 0        | PrePhysics  | Start       0 |
    /// | 1        | PrePhysics  | FixedUpdate 1 |
    /// | 2        | PostPhysics | OnCollision 2 |
    /// | 3        | PostPhysics | OnMouse     3 |
    /// | 4        | PostPhysics | Update      4 |
    ///
    /// Note: in this example, if your FixedUpdate event is fired *after* the handler system set has run, it will be discarded (since other handlers discard events of higher priority).
    fn add_script_handler<T: ScriptHost, const MAX: u32, const MIN: u32>(
        &mut self,
        schedule: impl ScheduleLabel,
    ) -> &mut Self;

    /// The same as `add_script_handler` but allows you to specify a system set to add the handler to.
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

    fn add_script_handler<T: ScriptHost, const MAX: u32, const MIN: u32>(
        &mut self,
        schedule: impl ScheduleLabel,
    ) -> &mut Self {
        self.add_systems(schedule, script_event_handler::<T, MAX, MIN>);
        self
    }
}
