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
use bevy_log::debug;
use bevy_mod_scripting_asset::ScriptAsset;
use bevy_mod_scripting_bindings::WorldGuard;
use bevy_mod_scripting_display::DisplayProxy;
use bevy_platform::collections::HashSet;
use parking_lot::Mutex;
use smallvec::SmallVec;

use crate::{
    IntoScriptPluginParams,
    context::ScriptingLoader,
    error::ScriptError,
    event::{
        ForPlugin, Recipients, ScriptAssetModifiedEvent, ScriptAttachedEvent, ScriptDetachedEvent,
        ScriptErrorEvent,
    },
    pipeline::hooks::{
        OnLoadedListener, OnReloadedListener, OnUnloadedForReloadListener,
        OnUnloadedForUnloadListener,
    },
    script::ScriptContext,
};

mod hooks;
mod machines;
mod start;
pub use {machines::*, start::*};

#[derive(SystemSet, Hash, Debug, Clone, Copy, PartialEq, Eq)]
/// System sets allowing for placing hooks at different stages in the loading/unloading process
pub enum PipelineSet {
    /// During this phase, various systems listen for events and filter through them to match what the pipeline expects
    ListeningPhase,
    /// During this phase we convert the filtered events to new machines
    MachineStartPhase,
}

/// A pipeline plugin which enables the loading and unloading of scripts in a highly modular way
pub struct ScriptLoadingPipeline<P: IntoScriptPluginParams> {
    /// by default the plugin will listen to [`crate::ScriptComponent`] attachments/detachments and synchronize scripts accordingly,
    /// you can opt out of this behavior by disabling this flag.
    pub script_component_triggers: bool,
    /// by default the plugin will listen to [`bevy_asset::AssetEvent<ScriptAsset>`] events, and trigger the pipeline on asset modifications.
    pub hot_loading_asset_triggers: bool,

    /// If true the [`crate::event::OnScriptLoaded`] callback will be triggered when loading scripts
    pub on_script_loaded_callback: bool,
    /// If true the [`crate::event::OnScriptReloaded`] callback will be triggered when loading scripts
    pub on_script_reloaded_callback: bool,
    /// If true the [`crate::event::OnScriptUnloaded`] callback will be triggered when loading scripts
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
                Some(LoadState::Loaded) | None => { // none in case this is added in memory and not through asset server
                    let strong = StrongScriptHandle::from_assets(handle, &mut self.assets);
                    if let Some(strong) = strong {
                        self.loaded_with_handles.push_front((e.clone(), strong));
                    }
                    false
                }
                Some(LoadState::Loading) => true,
                state => {

                    debug!(
                        "discarding script lifecycle triggers with handle: {} due to asset load state: {state:?}",
                        handle.display()
                    );
                    false
                }
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

        app.configure_sets(
            PostUpdate,
            PipelineSet::ListeningPhase.before(PipelineSet::MachineStartPhase),
        );

        // todo: nicer way to order these, ideall .chain() + conditional newtype?
        if self.script_component_triggers {
            app.add_systems(
                PostUpdate,
                filter_script_attachments::<P>
                    .in_set(PipelineSet::ListeningPhase)
                    .before(filter_script_modifications::<P>)
                    .before(filter_script_detachments::<P>),
            );
            app.add_systems(
                PostUpdate,
                filter_script_detachments::<P>
                    .in_set(PipelineSet::ListeningPhase)
                    .after(filter_script_attachments::<P>)
                    .before(filter_script_modifications::<P>),
            );
        }

        if self.hot_loading_asset_triggers {
            app.add_systems(
                PostUpdate,
                filter_script_modifications::<P>.in_set(PipelineSet::ListeningPhase),
            );
        }

        app.add_systems(
            PostUpdate,
            (
                process_attachments::<P>,
                process_detachments::<P>,
                process_asset_modifications::<P>,
            )
                .chain()
                .in_set(PipelineSet::MachineStartPhase),
        );

        app.add_systems(
            PostUpdate,
            automatic_pipeline_runner::<P>.after(PipelineSet::MachineStartPhase),
        );

        let mut schedule = Schedule::new(ScriptProcessingSchedule::<P>(Default::default()));

        schedule.add_systems(machine_ticker::<P>);

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
pub struct RunProcessingPipelineOnce<P> {
    override_budget: Option<Duration>,
    _ph: PhantomData<fn(P)>,
}

impl<P> RunProcessingPipelineOnce<P> {
    /// Creates a new [`RunProcessingPipelineOnce`] command for the given plugin.
    /// The budget override can be used to make sure all scripts in the pipeline get fully processed in one go
    pub fn new(override_budget: Option<Duration>) -> Self {
        Self {
            override_budget,
            _ph: PhantomData,
        }
    }
}

impl<P> Default for RunProcessingPipelineOnce<P> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl<P: IntoScriptPluginParams> Command for RunProcessingPipelineOnce<P> {
    fn apply(self, world: &mut World) {
        let mut last_setting = None;
        if let Some(override_budget) = self.override_budget {
            let mut machines = world.get_resource_or_init::<ActiveMachines<P>>();
            last_setting = Some(machines.budget);
            machines.budget = Some(override_budget)
        }
        world.run_schedule(ScriptProcessingSchedule::<P>::default());
        if let Some(last_setting) = last_setting {
            let mut machines = world.get_resource_or_init::<ActiveMachines<P>>();
            machines.budget = last_setting;
        }
    }
}

/// A system which runs [`RunProcessingPipelineOnce`] command for the plugin only if there are active machines
pub fn automatic_pipeline_runner<P: IntoScriptPluginParams>(world: &mut World) {
    if world
        .get_resource::<ActiveMachines<P>>()
        .is_some_and(|machines| machines.active_machines() > 0)
    {
        RunProcessingPipelineOnce::<P>::new(None).apply(world);
    }
}

/// A helper trait for App implementations
pub trait PipelineRun {
    /// Runs update's until all scripts are processed and no more machines are running
    fn update_until_all_scripts_processed<P: IntoScriptPluginParams>(&mut self);
}

impl PipelineRun for App {
    fn update_until_all_scripts_processed<P: IntoScriptPluginParams>(&mut self) {
        loop {
            let world = self.world_mut();
            let machines = world.get_resource::<ActiveMachines<P>>();
            let has_active = machines.is_some_and(|machines| machines.active_machines() > 0);
            if !has_active {
                break;
            }
            self.update();
        }
    }
}

#[derive(SystemParam)]
/// System parameter composing resources related to script loading, exposing utility methods for checking on your script pipeline status
pub struct ScriptPipelineState<'w, P: IntoScriptPluginParams> {
    contexts: Res<'w, ScriptContext<P>>,
    machines: Res<'w, ActiveMachines<P>>,
}

impl<'w, P: IntoScriptPluginParams> ScriptPipelineState<'w, P> {
    /// Returns the handle to the currently processing script, if the handle came from an asset server and a path,
    /// it can be used to display the currently loading script
    pub fn currently_loading_script(&self) -> Option<Handle<ScriptAsset>> {
        self.machines
            .current_machine()
            .map(|machine| machine.context.attachment.script())
    }

    /// Returns the number of scripts currently being processed,
    /// this includes loads, reloads and removals, when this is zero, no processing is happening at the moment
    pub fn num_processing_scripts(&self) -> usize {
        self.machines.active_machines()
    }

    /// returns true if the current processing batch is completed,
    /// a batch is completed when the last active processing machine is finished.
    /// If new machines are added during the processing of a batch, that batch is "extended".
    pub fn processing_batch_completed(&self) -> bool {
        self.num_processing_scripts() == 0
    }

    /// Returns the number of scripts currently existing in contexts.
    /// This corresponds to [`Recipients::AllScripts`], i.e. it counts 'residents' within contexts as a script
    pub fn num_loaded_scripts(&self) -> usize {
        Recipients::AllScripts
            .get_recipients(self.contexts.clone())
            .len()
    }

    /// returns a number between 0 and 100.0 to represent the current script pipeline progress,
    /// 0 representing no progress made, and 100 all processing completed, together with the numbers used for the fraction loaded and total.
    pub fn progress(&self) -> (f32, usize, usize) {
        let fraction = self.num_loaded_scripts();
        let total = self.num_processing_scripts() + fraction;
        if total == 0 {
            return (0.0, 0, 0);
        }
        ((fraction as f32 / total as f32) * 100.0, fraction, total)
    }
}

#[cfg(test)]
mod test {
    use bevy_asset::{AssetApp, AssetId, AssetPlugin};
    use bevy_ecs::{entity::Entity, system::SystemState, world::FromWorld};
    use bevy_mod_scripting_asset::Language;
    use bevy_mod_scripting_bindings::ScriptValue;
    use bevy_mod_scripting_script::ScriptAttachment;
    use test_utils::make_test_plugin;

    use crate::config::{GetPluginThreadConfig, ScriptingPluginConfiguration};

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

    make_test_plugin!(crate);

    #[test]
    fn test_run_override_is_undid() {
        let mut app = App::default();
        app.add_event::<ScriptAttachedEvent>();

        app.add_plugins((AssetPlugin::default(), TestPlugin::default()));
        app.init_asset::<ScriptAsset>();
        let mut machines = ActiveMachines::<TestPlugin>::default();
        machines.budget = None;
        app.insert_resource(machines);
        app.finish();

        let world = app.world_mut();
        RunProcessingPipelineOnce::<TestPlugin>::new(Some(Duration::from_secs(1))).apply(world);

        let machines = world.get_resource::<ActiveMachines<TestPlugin>>().unwrap();
        assert_eq!(machines.budget, None);
    }
}
