//! Everything to do with the script lifetime management pipeline

use std::{any::Any, collections::VecDeque, marker::PhantomData, sync::Arc};

use bevy_app::{App, Plugin, PreUpdate};
use bevy_asset::{AssetServer, Assets, Handle, LoadState};
use bevy_ecs::{
    message::{Message, MessageReader},
    observer::On,
    resource::Resource,
    schedule::{IntoScheduleConfigs, Schedule, ScheduleLabel, SystemSet},
    system::{Command, Commands, IntoSystem, Local, Res, ResMut, SystemParam},
    world::{World, WorldId},
};
use bevy_log::debug;
use bevy_mod_scripting_asset::{Language, ScriptAsset};
use bevy_mod_scripting_display::DisplayProxy;
use parking_lot::Mutex;

use crate::{
    IntoScriptPluginParams, ScriptingSystemSet,
    context::ScriptingLoader,
    error::ScriptError,
    event::{
        GetRecipients, Recipients, ScriptAssetModifiedMessage, ScriptAttachedMessage,
        ScriptDetachedMessage, ScriptErrorMessage,
    },
    pipeline::hooks::{
        clear_machine_data, on_script_unloaded_for_reload_pipeline_handler,
        on_script_unloaded_for_unload_pipeline_handler, process_machine_failure,
        queue_on_script_loaded, queue_on_script_reloaded,
    },
    script::ScriptContexts,
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

    /// The maximum amount of state machine ticks (loading steps) run each frame, unlimited if not set (the default).
    /// A tick can run script code (i.e. top level code or lifecycle callbacks), so this bounds the amount of steps, not their duration.
    ///
    /// This counts ticks rather than wall clock time so that the frame in which a script loads is deterministic.
    pub tick_budget: Option<usize>,
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
            tick_budget: self.tick_budget,
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
            tick_budget: None,
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

/// A trait describing things containing script handles
pub trait GetScriptHandle {
    /// Retrieve the contained script handle
    fn get_script_handle(&self) -> Handle<ScriptAsset>;
}

/// An event queued in [`LoadedWithHandles`].
#[derive(Clone)]
pub enum HeldEvent<T, D> {
    /// Waits for its script asset to finish loading.
    Load(T),
    /// Released as soon as all the events before it are released.
    Pass(D),
}

/// An event released by [`LoadedWithHandles`].
pub enum ReleasedEvent<T, D> {
    /// An event whose script asset finished loading, with a strong handle to it and its language.
    Loaded(T, StrongScriptHandle, Language),
    /// An event which waited for the events before it.
    Passed(D),
}

#[derive(SystemParam)]
/// A system param which operates over types implementing [`GetScriptHandle`].
/// Captures incoming "handle" like types `T`, and waits until their asset is in a final state before proceeding, if that final state
/// is loaded, will also guarantee a strong handle, otherwise the whole thing is skipped.
///
/// Events of type `D` don't need their asset, but are queued with the `T` events, so they never overtake one which is still loading.
///
/// Think of this as a proxy for "baby'ing" asset handles
pub struct LoadedWithHandles<'w, 's, T: GetScriptHandle + Message + Clone, D: Message + Clone> {
    /// The script assets resource. To allow systems needing both resources to access them in one place
    pub assets: ResMut<'w, Assets<ScriptAsset>>,
    asset_server: Res<'w, AssetServer>,
    fresh_events: MessageReader<'w, 's, T>,
    fresh_pass_events: MessageReader<'w, 's, D>,
    loaded_with_handles: Local<'s, VecDeque<ReleasedEvent<T, D>>>,
    loading: Local<'s, VecDeque<HeldEvent<T, D>>>,
}

impl<T: GetScriptHandle + Message + Clone, D: Message + Clone> LoadedWithHandles<'_, '_, T, D> {
    /// Retrieves all of the events of type `T`, which have finished loading and have a strong handle,
    /// the rest will be discarded, together with the `D` events queued between them.
    ///
    /// Events are released strictly in the order they arrived, an event whose asset is still loading holds back all the events behind it.
    /// This way the order in which scripts enter the pipeline does not depend on asset IO timing.
    /// Within a frame, the `T` events are queued before the `D` events.
    ///
    /// This uses a [`EventReader<T>`] underneath, meaning if you don't call this method once every frame (or every other frame).
    /// You may miss events.
    pub fn get_loaded(&mut self) -> impl Iterator<Item = ReleasedEvent<T, D>> {
        // first get all of the fresh_events
        self.loading
            .extend(self.fresh_events.read().cloned().map(HeldEvent::Load));
        self.loading
            .extend(self.fresh_pass_events.read().cloned().map(HeldEvent::Pass));
        // now process the loading queue in order, until we hit an event which is still loading
        while let Some(front) = self.loading.front() {
            let handle = match front {
                HeldEvent::Load(e) => e.get_script_handle(),
                HeldEvent::Pass(_) => {
                    if let Some(HeldEvent::Pass(e)) = self.loading.pop_front() {
                        self.loaded_with_handles.push_back(ReleasedEvent::Passed(e));
                    }
                    continue;
                }
            };
            match self.asset_server.get_load_state(&handle) {
                Some(LoadState::Loading) => break,
                Some(LoadState::Loaded) | None => {
                    // none in case this is added in memory and not through asset server
                    let Some(HeldEvent::Load(e)) = self.loading.pop_front() else {
                        break;
                    };
                    let strong = StrongScriptHandle::from_assets(handle, &mut self.assets);
                    if let Some(strong) = strong {
                        let lang = strong.get(&self.assets).language.clone();
                        self.loaded_with_handles
                            .push_back(ReleasedEvent::Loaded(e, strong, lang));
                    }
                }
                state => {
                    self.loading.pop_front();
                    debug!(
                        "discarding script lifecycle triggers with handle: {} due to asset load state: {state:?}",
                        handle.display()
                    );
                }
            }
        }

        // now return loaded with handles elements by draining
        self.loaded_with_handles.drain(..)
    }
}

impl<P: IntoScriptPluginParams> Plugin for ScriptLoadingPipeline<P> {
    fn build(&self, app: &mut App) {
        app.add_message::<ScriptAttachedMessage>()
            .add_message::<ScriptDetachedMessage>()
            .add_message::<ScriptAssetModifiedMessage>();

        app.init_resource::<ActiveMachinesData>();

        // bevy does not guarantee the order of observers watching the same event (it depends on their entity ids),
        // so a single observer runs `on_script_loaded` and then `on_script_reloaded`, as documented
        let (loaded, reloaded) = (
            self.on_script_loaded_callback,
            self.on_script_reloaded_callback,
        );
        if loaded || reloaded {
            app.add_observer(
                move |trigger: On<ContextAssigned<P>>,
                      mut commands: Commands,
                      world_id: WorldId,
                      datas: Res<ActiveMachinesData>| {
                    if loaded {
                        queue_on_script_loaded(trigger.event(), &mut commands, world_id);
                    }
                    if reloaded {
                        queue_on_script_reloaded(trigger.event(), &mut commands, world_id, &datas);
                    }
                },
            );
        }

        // these are the only triggers on their events
        if self.on_script_unloaded_callback {
            app.add_observer(on_script_unloaded_for_unload_pipeline_handler::<P>);
            app.add_observer(on_script_unloaded_for_reload_pipeline_handler::<P>);
        }

        // clear hanging machine data
        app.add_observer(
            (|trigger: On<LoadingCompleted>| trigger.0.clone()).pipe(clear_machine_data),
        );
        app.add_observer(
            (|trigger: On<UnloadingCompleted>| trigger.0.clone()).pipe(clear_machine_data),
        );
        app.add_observer(
            (|trigger: On<ProcessInterrupted>| trigger.0.clone()).pipe(clear_machine_data),
        );
        // failed machines shouldn't lead to locking out scripts
        app.add_observer(
            (|trigger: On<ProcessInterrupted>| trigger.0.clone())
                .pipe(process_machine_failure::<P>),
        );

        let mut active_machines = app.world_mut().get_resource_or_init::<ActiveMachines<P>>();

        active_machines.budget = self.tick_budget;

        app.configure_sets(
            PreUpdate,
            PipelineSet::ListeningPhase
                .after(bevy_asset::AssetTrackingSystems)
                .after(ScriptingSystemSet::SyncScriptingComponents) // we want to pick up changes immediately
                .before(PipelineSet::MachineStartPhase),
        );

        let sync_components = self.script_component_triggers;
        let hot_loads = self.hot_loading_asset_triggers;
        let input_system = move || FilterAssetEventsSettings {
            sync_components,
            hot_loads,
        };
        app.add_systems(
            PreUpdate,
            (input_system.pipe(filter_asset_events::<P>)).in_set(PipelineSet::ListeningPhase),
        );

        app.add_systems(
            PreUpdate,
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
    override_budget: Option<usize>,
    _ph: PhantomData<fn(P)>,
}

impl<P> RunProcessingPipelineOnce<P> {
    /// Creates a new [`RunProcessingPipelineOnce`] command for the given plugin.
    /// The budget override can be used to make sure all scripts in the pipeline get fully processed in one go
    pub fn new(override_budget: Option<usize>) -> Self {
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

    type Out = ();
}

/// A system which runs [`RunProcessingPipelineOnce`] command for the plugin only if there are active machines
pub fn automatic_pipeline_runner<P: IntoScriptPluginParams>(world: &mut World) {
    if world
        .get_resource::<ActiveMachines<P>>()
        .is_some_and(|machines| machines.processing_and_queued_machines() > 0)
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
            let has_active =
                machines.is_some_and(|machines| machines.processing_and_queued_machines() > 0);
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
    contexts: Res<'w, ScriptContexts<P>>,
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
    pub fn processing_script_count(&self) -> usize {
        self.machines.processing_and_queued_machines()
    }

    /// Returns true if no scripts are currently being processed or queued for processing.
    /// If new scripts are queued during processing, processing is not completed until those finish too.
    pub fn is_processing_completed(&self) -> bool {
        self.processing_script_count() == 0
    }

    /// Returns the number of scripts currently existing in contexts.
    /// This corresponds to [`Recipients::AllScripts`], i.e. it counts 'residents' within contexts as a script
    pub fn loaded_script_count(&self) -> usize {
        Recipients::AllScripts
            .get_recipients(self.contexts.clone())
            .len()
    }

    /// returns a number between 0.0 and 1.0 to represent the current script pipeline progress,
    /// 0.0 representing no progress made, and 1.0 all processing completed, together with the numbers used for the fraction loaded and total.
    pub fn progress_fraction(&self) -> (f32, usize, usize) {
        let fraction = self.loaded_script_count();
        let total = self.processing_script_count() + fraction;
        if total == 0 {
            return (0.0, 0, 0);
        }
        (fraction as f32 / total as f32, fraction, total)
    }
}

#[cfg(test)]
mod test {
    use bevy_asset::{AssetApp, AssetPlugin};
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
        app.add_message::<ScriptAttachedMessage>()
            .add_message::<ScriptDetachedMessage>();
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<ScriptAsset>();
        app.finish();

        let world = app.world_mut();
        let mut system_state = SystemState::<
            LoadedWithHandles<ScriptAttachedMessage, ScriptDetachedMessage>,
        >::from_world(world);
        // start empty
        {
            let mut state = system_state.get_mut(world).unwrap();
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
        let handle_invalid = Handle::default();
        world.write_message(ScriptAttachedMessage(ScriptAttachment::StaticScript(
            handle,
        )));
        world.write_message(ScriptAttachedMessage(ScriptAttachment::StaticScript(
            handle_invalid,
        )));

        // expect one loading, holding back the invalid one behind it
        {
            let mut state = system_state.get_mut(world).unwrap();
            let loaded = state.get_loaded().collect::<Vec<_>>();
            assert!(loaded.is_empty());
            assert_eq!(state.loading.len(), 2);
        }

        // now on next call the old ones don't persist
        {
            let mut state = system_state.get_mut(world).unwrap();
            let loaded = state.get_loaded().collect::<Vec<_>>();
            assert!(loaded.is_empty())
        }
    }

    make_test_plugin!(crate);

    #[test]
    fn test_detachment_waits_for_loading_attachment() {
        let mut app = App::default();
        app.add_message::<ScriptAttachedMessage>()
            .add_message::<ScriptDetachedMessage>();
        app.add_plugins(AssetPlugin::default());
        app.init_asset::<ScriptAsset>();
        app.init_resource::<ActiveMachines<TestPlugin>>();
        app.finish();

        let world = app.world_mut();
        // stays loading until the asset systems run
        let handle = world.resource::<AssetServer>().add(ScriptAsset {
            content: "asd".to_string().into_boxed_str().into_boxed_bytes(),
            language: Language::Lua,
        });
        let attachment = ScriptAttachment::StaticScript(handle);
        world.write_message(ScriptAttachedMessage(attachment.clone()));
        world.write_message(ScriptDetachedMessage(attachment));

        let system = world.register_system(filter_asset_events::<TestPlugin>);
        let settings = FilterAssetEventsSettings {
            sync_components: true,
            hot_loads: false,
        };
        world.run_system_with(system, settings).unwrap();

        // the detachment must not overtake the attachment which is still loading
        let machines = world.resource::<ActiveMachines<TestPlugin>>();
        assert_eq!(machines.queued_machines(), 0);
    }

    #[test]
    fn test_run_override_is_undid() {
        let mut app = App::default();
        app.add_message::<ScriptAttachedMessage>();

        app.add_plugins((AssetPlugin::default(), TestPlugin::default()));
        app.init_asset::<ScriptAsset>();
        let mut machines = ActiveMachines::<TestPlugin>::default();
        machines.budget = None;
        app.insert_resource(machines);
        app.finish();

        let world = app.world_mut();
        RunProcessingPipelineOnce::<TestPlugin>::new(Some(1)).apply(world);

        let machines = world.get_resource::<ActiveMachines<TestPlugin>>().unwrap();
        assert_eq!(machines.budget, None);
    }
}
