//! Everything to do with the script lifetime management pipeline

use std::{any::Any, collections::VecDeque, marker::PhantomData, sync::Arc, time::Duration};

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::{AssetServer, Assets, Handle, LoadState};
use bevy_ecs::{
    event::{Event, EventCursor, EventReader, EventWriter, Events},
    resource::Resource,
    schedule::{IntoScheduleConfigs, Schedule, ScheduleLabel, SystemSet},
    system::{Command, Local, Res, ResMut, SystemParam},
    world::World,
};
use bevy_mod_scripting_asset::ScriptAsset;
use bevy_mod_scripting_bindings::WorldGuard;
use bevy_platform::collections::HashSet;
use parking_lot::Mutex;
use smallvec::SmallVec;

use crate::{
    IntoScriptPluginParams,
    context::ScriptingLoader,
    error::ScriptError,
    event::{
        ForPlugin, ScriptAssetModifiedEvent, ScriptAttachedEvent, ScriptDetachedEvent,
        ScriptErrorEvent,
    },
    pipeline::hooks::{
        OnLoadedListener, OnReloadedListener, OnUnloadedForReloadListener,
        OnUnloadedForUnloadListener,
    },
    script::{ScriptAttachment, ScriptContext, ScriptId},
};

mod hooks;
mod machines;
mod start;
pub use {machines::*, start::*};

#[derive(SystemSet, Hash, Debug, Clone, Copy, PartialEq, Eq)]
/// System sets allowing for placing hooks at different stages in the loading/unloading process
pub enum PipelineSet {
    /// During this phase, various systems listen for
    InitializePhase,
    /// The phase during which we tick all state machines
    TickMachinesPhase,
}

/// A pipeline plugin which enables the loading and unloading of scripts in a highly modular way
pub struct ScriptLoadingPipeline<P: IntoScriptPluginParams> {
    /// by default the plugin will listen to [`ScriptComponent`] attachments/detachments and synchronize scripts accordingly,
    /// you can opt out of this behavior by disabling this flag.
    pub script_component_triggers: bool,
    /// by default the plugin will listen to [`AssetEvent<ScriptAsset>`] events, and trigger the pipeline on asset modifications.
    pub hot_loading_asset_triggers: bool,

    /// If true the [`OnScriptLoaded`] callback will be triggered when loading scripts
    pub on_script_loaded_callback: bool,
    /// If true the [`OnScriptReloaded`] callback will be triggered when loading scripts
    pub on_script_reloaded_callback: bool,
    /// If true the [`OnScriptUnloaded`] callback will be triggered when loading scripts
    pub on_script_unloaded_callback: bool,
    _ph: PhantomData<fn(P)>,

    /// The budget in wall clock time, for loading scripts each frame, if not set, will default to one 60FPS frametime.
    /// The executor will try its best to keep loading time up within this budget. This cannot be guaranteed as not all operations are
    /// granular enough (for example script execution)
    pub time_budget: Option<Duration>,
}

impl<P: IntoScriptPluginParams> Clone for ScriptLoadingPipeline<P> {
    fn clone(&self) -> Self {
        Self {
            script_component_triggers: self.script_component_triggers,
            hot_loading_asset_triggers: self.hot_loading_asset_triggers,
            on_script_loaded_callback: self.on_script_loaded_callback,
            on_script_reloaded_callback: self.on_script_reloaded_callback,
            on_script_unloaded_callback: self.on_script_unloaded_callback,
            _ph: self._ph,
            time_budget: self.time_budget,
        }
    }
}

impl<P: IntoScriptPluginParams> Default for ScriptLoadingPipeline<P> {
    fn default() -> Self {
        Self {
            _ph: PhantomData,
            script_component_triggers: true,
            hot_loading_asset_triggers: true,
            on_script_loaded_callback: true,
            on_script_reloaded_callback: true,
            on_script_unloaded_callback: true,
            time_budget: Some(Duration::from_millis(16)),
        }
    }
}

#[derive(ScheduleLabel, Copy)]
/// Schedule dedicated to script processing
pub struct ScriptProcessingSchedule<P: IntoScriptPluginParams>(PhantomData<fn(P)>);

impl<P: IntoScriptPluginParams> Default for ScriptProcessingSchedule<P> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<P: IntoScriptPluginParams> PartialEq for ScriptProcessingSchedule<P> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl<P: IntoScriptPluginParams> Eq for ScriptProcessingSchedule<P> {}

impl<P: IntoScriptPluginParams> std::hash::Hash for ScriptProcessingSchedule<P> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        0.hash(state)
    }
}

impl<P: IntoScriptPluginParams> Clone for ScriptProcessingSchedule<P> {
    fn clone(&self) -> Self {
        ScriptProcessingSchedule::default()
    }
}

impl<P: IntoScriptPluginParams> std::fmt::Debug for ScriptProcessingSchedule<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!(
            "ScriptProcessingSchedule<{}>",
            std::any::type_name::<P>()
        ))
        .finish()
    }
}

impl<P: IntoScriptPluginParams> ScriptLoadingPipeline<P> {
    fn add_plugin_event<E: Event>(&self, app: &mut App) -> &Self {
        app.add_event::<E>().add_event::<ForPlugin<E, P>>();
        self
    }
}

/// A trait describing things containing script handles
pub trait GetScriptHandle {
    /// Retrieve the contained script handle
    fn get_script_handle(&self) -> Handle<ScriptAsset>;
}

#[derive(SystemParam)]
/// A system param which operates over types implementing [`GetScriptHandle`].
/// Captures incoming "handle" like types, and waits until their asset is in a final state before proceeding, if that final state
/// is loaded, will also guarantee a strong handle, otherwise the whole thing is skipped.
///
/// Think of this as a proxy for "baby'ing" asset handles
pub struct LoadedWithHandles<'w, 's, T: GetScriptHandle + Event + Clone> {
    assets: ResMut<'w, Assets<ScriptAsset>>,
    asset_server: Res<'w, AssetServer>,
    fresh_events: EventReader<'w, 's, T>,
    loaded_with_handles: Local<'s, VecDeque<(T, StrongScriptHandle)>>,
    loading: Local<'s, VecDeque<T>>,
}

impl<T: GetScriptHandle + Event + Clone> LoadedWithHandles<'_, '_, T> {
    /// Retrieves all of the events of type `T`, which have finished loading and have a strong handle,
    /// the rest will be discarded.
    ///
    /// This uses a [`EventReader<T>`] underneath, meaning if you don't call this method once every frame (or every other frame).
    /// You may miss events.
    pub fn get_loaded(&mut self) -> impl Iterator<Item = (T, StrongScriptHandle)> {
        // first get all of the fresh_events
        self.loading.extend(self.fresh_events.read().cloned());
        // now process the loading queue
        self.loading.retain(|e| {
            let handle = e.get_script_handle();
            match self.asset_server.get_load_state(&handle) {
                Some(LoadState::Loaded) => {
                    let strong = StrongScriptHandle::from_assets(handle, &mut self.assets);
                    if let Some(strong) = strong {
                        self.loaded_with_handles.push_front((e.clone(), strong));
                    }
                    false
                }
                Some(LoadState::Loading) => true,
                _ => false,
            }
        });

        // now return loaded with handles elements by draining
        self.loaded_with_handles.drain(..)
    }
}

impl<P: IntoScriptPluginParams> Plugin for ScriptLoadingPipeline<P> {
    fn build(&self, app: &mut App) {
        self.add_plugin_event::<ScriptAttachedEvent>(app)
            .add_plugin_event::<ScriptDetachedEvent>(app)
            .add_plugin_event::<ScriptAssetModifiedEvent>(app);

        let mut active_machines = app.world_mut().get_resource_or_init::<ActiveMachines<P>>();
        if self.on_script_loaded_callback {
            active_machines.push_listener::<ContextAssigned<P>>(OnLoadedListener);
        }

        if self.on_script_reloaded_callback {
            active_machines.push_listener::<ContextAssigned<P>>(OnReloadedListener);
        }

        if self.on_script_unloaded_callback {
            active_machines.push_listener::<ReloadingInitialized<P>>(OnUnloadedForReloadListener);
            active_machines.push_listener::<UnloadingInitialized<P>>(OnUnloadedForUnloadListener);
        }

        active_machines.budget = self.time_budget;

        app.init_resource::<RequestProcessingPipelineRun<P>>();

        if self.script_component_triggers {
            app.add_systems(
                PostUpdate,
                (
                    filter_script_attachments::<P>,
                    filter_script_detachments::<P>,
                )
                    .before(PipelineSet::InitializePhase),
            );
        }

        if self.hot_loading_asset_triggers {
            app.add_systems(
                PostUpdate,
                (filter_script_modifications::<P>,).before(PipelineSet::InitializePhase),
            );
        }

        app.add_systems(
            PostUpdate,
            automatic_pipeline_runner::<P>.after(PipelineSet::InitializePhase),
        );

        let mut schedule = Schedule::new(ScriptProcessingSchedule::<P>(Default::default()));
        schedule
            .configure_sets(PipelineSet::InitializePhase.before(PipelineSet::TickMachinesPhase));

        schedule.add_systems(
            (
                process_attachments::<P>,
                process_detachments::<P>,
                process_asset_modifications::<P>,
            )
                .in_set(PipelineSet::InitializePhase),
        );

        schedule.add_systems((machine_ticker::<P>).in_set(PipelineSet::TickMachinesPhase));

        app.add_schedule(schedule);
    }
}

/// System which ticks machines within the given time budget
pub fn machine_ticker<P: IntoScriptPluginParams>(world: &mut World) {
    if let Some(mut machines) = world.remove_resource::<ActiveMachines<P>>() {
        machines.tick_machines(world);
        world.insert_resource(machines)
    }
}
/// A command which triggers the script processing pipeline to run once,
/// causing outstanding attachment events to be processed
pub struct RunProcessingPipelineOnce<P>(PhantomData<fn(P)>);

impl<P> RunProcessingPipelineOnce<P> {
    /// Creates a new [`RunProcessingPipelineOnce`] command for the given plugin
    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<P> Default for RunProcessingPipelineOnce<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: IntoScriptPluginParams> Command for RunProcessingPipelineOnce<P> {
    fn apply(self, world: &mut World) {
        world.run_schedule(ScriptProcessingSchedule::<P>::default());
    }
}

#[derive(Resource)]
/// A resource marker used to notify the processing pipeline to run
pub struct RequestProcessingPipelineRun<P>(PhantomData<fn(P)>, bool);

impl<P> RequestProcessingPipelineRun<P> {
    /// Creates a default [`RequestProcessingPipelineRun`] for the plugi
    pub fn new() -> Self {
        Self(Default::default(), false)
    }

    /// Returns true if a processing pipeline run was requested, and sets the flag to false again.
    pub fn get_and_unset(&mut self) -> bool {
        let requested = self.1;
        self.1 = false;
        requested
    }

    /// requests a run this frame
    pub fn request_run(&mut self) {
        self.1 = true;
    }
}

impl<P> Default for RequestProcessingPipelineRun<P> {
    fn default() -> Self {
        Self::new()
    }
}

/// A system which runs [`RunProcessingPipelineOnce`] command for the plugin only if [`RequestProcessingPipelineRun`] resource had [`RequestProcessingPipelineRun::request_run`] called on it
pub fn automatic_pipeline_runner<P: IntoScriptPluginParams>(world: &mut World) {
    let mut res = world.get_resource_or_init::<RequestProcessingPipelineRun<P>>();
    if res.get_and_unset() {
        RunProcessingPipelineOnce::<P>::new().apply(world);
    }
}

#[cfg(test)]
mod test {
    use bevy_asset::{AssetApp, AssetId, AssetPlugin};
    use bevy_ecs::{system::SystemState, world::FromWorld};
    use bevy_mod_scripting_asset::Language;

    use super::*;
    #[test]
    fn test_system_params() {
        let mut app = App::default();
        app.add_event::<ScriptAttachedEvent>();
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<ScriptAsset>();
        app.finish();

        let world = app.world_mut();
        let mut system_state =
            SystemState::<LoadedWithHandles<ScriptAttachedEvent>>::from_world(world);
        // start empty
        {
            let mut state = system_state.get_mut(world);
            let loaded = state.get_loaded().collect::<Vec<_>>();
            assert!(loaded.is_empty())
        }

        // send event with loading asset
        // let assets = world.get_resource_mut::<Assets<ScriptAsset>>().unwrap();
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let asset = ScriptAsset {
            content: "asd".to_string().into_boxed_str().into_boxed_bytes(),
            language: Language::Lua,
        };
        let handle = asset_server.add(asset);
        let handle_invalid = Handle::Weak(AssetId::invalid());
        world.send_event(ScriptAttachedEvent(ScriptAttachment::StaticScript(handle)));
        world.send_event(ScriptAttachedEvent(ScriptAttachment::StaticScript(
            handle_invalid,
        )));

        // expect one loading, one invalid
        {
            let mut state = system_state.get_mut(world);
            let loaded = state.get_loaded().collect::<Vec<_>>();
            assert!(loaded.is_empty());
            assert_eq!(state.loading.len(), 1);
        }

        // now on next call the old ones don't persist
        {
            let mut state = system_state.get_mut(world);
            let loaded = state.get_loaded().collect::<Vec<_>>();
            assert!(loaded.is_empty())
        }
    }
}
