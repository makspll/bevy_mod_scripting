//! Everything to do with the script lifetime management pipeline

use std::{any::Any, collections::VecDeque, marker::PhantomData, sync::Arc};

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::{AssetServer, Assets, Handle, LoadState};
use bevy_ecs::{
    event::{Event, EventCursor, EventReader, EventWriter, Events},
    resource::Resource,
    schedule::{IntoScheduleConfigs, Schedule, ScheduleLabel, SystemSet},
    system::{Command, Local, Res, ResMut, SystemParam, SystemState},
    world::World,
};
use bevy_mod_scripting_asset::ScriptAsset;
use bevy_mod_scripting_bindings::WorldGuard;
use bevy_platform::collections::HashSet;
use itertools::{Either, Itertools};
use parking_lot::Mutex;
use smallvec::SmallVec;

use crate::{
    IntoScriptPluginParams,
    context::ScriptingLoader,
    error::ScriptError,
    event::ScriptErrorEvent,
    script::{ScriptAttachment, ScriptContext, ScriptId},
};

mod finish;
mod hooks;
mod insert;
mod machines;
mod start;
mod update;
pub use {finish::*, hooks::*, insert::*, machines::*, start::*, update::*};

#[derive(SystemSet, Hash, Debug, Clone, Copy, PartialEq, Eq)]
/// System sets allowing for placing hooks at different stages in the loading/unloading process
pub enum PipelineSet {
    /// During this phase, various systems listen for
    StartPhase,
    /// During this phase, we load and reload contexts for scripts
    ContextUpdatePhase,
    /// During this phase, we insert new contexts, and mark scripts as resident
    InsertionPhase,
    /// During this phase, we remove old contexts, and unmark scripts as resident,
    RemovalPhase,
    /// A phase in which we send machine final states
    CompletionPhase,
}

/// A pipeline plugin which enables the loading and unloading of scripts in a highly modular way
pub struct ScriptLoadingPipeline<P: IntoScriptPluginParams>(PhantomData<fn(P)>);

impl<P: IntoScriptPluginParams> Default for ScriptLoadingPipeline<P> {
    fn default() -> Self {
        Self(Default::default())
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

    fn add_state<E: Event>(&self, app: &mut App) -> &Self {
        app.add_event::<ForPlugin<E, P>>();
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
                        true
                    } else {
                        false
                    }
                }
                Some(LoadState::Loading) => true,
                Some(_) => false,
                None => false,
            }
        });

        // now return loaded with handles elements by draining
        self.loaded_with_handles.drain(..)
    }
}

// struct StateBuilder<'a, P: IntoScriptPluginParams, M, S> {
//     plugin: &'a ScriptLoadingPipeline<P>,
//     app: &'a mut App,
//     ph: PhantomData<fn(M, S)>,
// }

// impl<P: IntoScriptPluginParams, M, S> StateBuilder<'_, P, M, S> {
//     fn add_transition<CM, C: IntoScheduleConfigs<ScheduleSystem, CM>>(self, systems: C) -> Self {
//         self.app.add_systems(systems.run_if());
//         self
//     }
// }

impl<P: IntoScriptPluginParams> Plugin for ScriptLoadingPipeline<P> {
    fn build(&self, app: &mut App) {
        self.add_plugin_event::<ScriptAttachedEvent>(app)
            .add_plugin_event::<ScriptDetachedEvent>(app)
            .add_plugin_event::<ScriptAssetModifiedEvent>(app);

        self.add_state::<Machine<Loading, LoadingInitialized>>(app)
            .add_state::<Machine<Loading, ReloadingInitialized<P>>>(app)
            .add_state::<Machine<Loading, ContextAssigned<P>>>(app)
            .add_state::<Machine<Loading, LoadingCompleted>>(app);

        self.add_state::<Machine<Unloading, UnloadingInitialized<P>>>(app)
            .add_state::<Machine<Unloading, ResidentRemoved<P>>>(app)
            .add_state::<Machine<Unloading, ContextRemoved<P>>>(app)
            .add_state::<Machine<Unloading, UnloadingCompleted>>(app);

        app.init_resource::<RequestProcessingPipelineRun<P>>();
        app.add_systems(
            PostUpdate,
            (
                filter_script_attachments::<P>,
                filter_script_detachments::<P>,
                filter_script_modifications::<P>,
            )
                .before(PipelineSet::StartPhase),
        );

        app.add_systems(
            PostUpdate,
            automatic_pipeline_runner::<P>.after(PipelineSet::StartPhase),
        );

        let mut schedule = Schedule::new(ScriptProcessingSchedule::<P>(Default::default()));
        schedule.configure_sets((
            PipelineSet::StartPhase.before(PipelineSet::ContextUpdatePhase),
            PipelineSet::ContextUpdatePhase.before(PipelineSet::InsertionPhase),
            PipelineSet::InsertionPhase.before(PipelineSet::RemovalPhase),
            PipelineSet::RemovalPhase.before(PipelineSet::CompletionPhase),
        ));

        schedule.add_systems(
            (
                process_attachments::<P>,
                process_detachments::<P>,
                process_asset_modifications::<P>,
            )
                .in_set(PipelineSet::StartPhase),
        );

        // -- on script unloaded (reload + removal)
        schedule.add_systems(
            (run_on_script_unloaded_hooks::<P>)
                .after(PipelineSet::StartPhase)
                .before(PipelineSet::ContextUpdatePhase),
        );
        // --

        schedule.add_systems(
            (
                assign_contexts_for_new_scripts::<P>,
                reload_existing_contexts::<P>,
            )
                .in_set(PipelineSet::ContextUpdatePhase),
        );

        // -- on script loaded
        schedule.add_systems(
            (run_on_script_loaded_hooks::<P>)
                .after(PipelineSet::ContextUpdatePhase)
                .before(PipelineSet::InsertionPhase),
        );
        // --

        // -- on script reloaded
        schedule.add_systems(
            (run_on_script_reloaded_hooks::<P>)
                .after(run_on_script_loaded_hooks::<P>)
                .before(PipelineSet::InsertionPhase),
        );
        // --

        schedule.add_systems((insert_residents::<P>).in_set(PipelineSet::InsertionPhase));

        schedule.add_systems(
            (remove_residents_or_remove_contexts::<P>).in_set(PipelineSet::RemovalPhase),
        );

        schedule.add_systems(
            (complete_loading::<P>, complete_unloading::<P>).in_set(PipelineSet::CompletionPhase),
        );

        app.add_schedule(schedule);
    }
}

/// A run condition which checks that states of the given kind are present, only runs the system if so
pub fn states_outstanding<M: Send + Sync + 'static, P: IntoScriptPluginParams>(
    machine: StateMachine<M, P>,
) -> bool {
    machine.machines_outstanding() > 0
}

/// A run condition which checks that states of the given kind are present, only runs the system if so
pub fn events_outstanding<M: Send + Sync + 'static, P: IntoScriptPluginParams>(
    machine: StateMachine<M, P>,
) -> bool {
    machine.machines_outstanding() > 0
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

/// A system which runs [`RunProcessingPipelineOnce`] command for the plugin only if [`RequestProcessingPipelineRun`] resource had [`RequestProcessingPipelineRun::request_run`] was called on it
pub fn automatic_pipeline_runner<P: IntoScriptPluginParams>(
    world: &mut World,
    // mut res: ResMut<RequestProcessingPipelineRun<P>>,
) {
    let mut res = world.get_resource_or_init::<RequestProcessingPipelineRun<P>>();
    if res.get_and_unset() {
        RunProcessingPipelineOnce::<P>::new().apply(world);
    }
}

/// Command which emits a [`ScriptAttachedEvent`] and then runs the processing pipeline to immediately process it.
/// The end result is equivalent to attaching a script component or adding a static script and waiting for the normal pipeline to process it.
pub struct AttachScript<P: IntoScriptPluginParams>(pub ForPlugin<ScriptAttachedEvent, P>);

impl<P: IntoScriptPluginParams> AttachScript<P> {
    /// Creates a new [`AttachScript`] command, which will create the given attachment, run expected callbacks, and
    pub fn new(attachment: ScriptAttachment) -> Self {
        Self(ForPlugin::new(ScriptAttachedEvent(attachment)))
    }
}

/// Command which emits a [`ScriptDetachedEvent`] and then runs the processing pipeline to immediately process it.
/// The end result is equivalent to detaching a script component or removing a static script and waiting for the normal pipeline to process it.
pub struct DetachScript<P: IntoScriptPluginParams>(pub ForPlugin<ScriptDetachedEvent, P>);

impl<P: IntoScriptPluginParams> DetachScript<P> {
    /// Creates a new [`DetachScript`] command, which will create the given attachment, run all expected callbacks, and delete contexts if necessary.
    pub fn new(attachment: ScriptAttachment) -> Self {
        Self(ForPlugin::new(ScriptDetachedEvent(attachment)))
    }
}

impl<P: IntoScriptPluginParams> Command for AttachScript<P> {
    fn apply(self, world: &mut World) {
        world.send_event(self.0);
        RunProcessingPipelineOnce::<P>::new().apply(world)
    }
}

impl<P: IntoScriptPluginParams> Command for DetachScript<P> {
    fn apply(self, world: &mut World) {
        world.send_event(self.0);
        RunProcessingPipelineOnce::<P>::new().apply(world)
    }
}
