//! All script host related stuff

pub mod rhai_host;
pub mod rlua_host;

use anyhow::Result;
use bevy::{asset::Asset, ecs::system::SystemState, prelude::*};
pub use {crate::rhai_host::*, crate::rlua_host::*};

use std::{
    collections::{HashMap, HashSet},
    sync::atomic::{AtomicU32, Ordering},
};

/// All code assets share this common interface.
/// When adding a new code asset don't forget to implement asset loading
/// and inserting appropriate systems when registering with the app
pub trait CodeAsset: Asset {
    fn bytes(&self) -> &[u8];
}

/// Implementers can modify a script context in order to enable
/// API access. ScriptHosts call `attach_api` when creating scripts
pub trait APIProvider: 'static + Default {
    /// The type of script context this api provider handles
    type Ctx;

    /// provide the given script context with the API permamently
    fn attach_api(ctx: &mut Self::Ctx);
}

#[derive(Component)]
/// The component storing many scripts.
/// Scripts receive information about the entity they are attached to
/// Scripts have unique identifiers and hence multiple copies of the same script
/// can be attached to the same entity
pub struct ScriptCollection<T: Asset> {
    pub scripts: Vec<Script<T>>,
}

/// A resource storing the script contexts for each script instance.
/// The reason we need this is to split the world borrow in our handle event systems, but this
/// has the added benefit that users don't see the contexts at all, and we can provide
/// generic handling for each new/removed script in one place.
///
/// We keep this public for now since there is no API for communicating with scripts
/// outside of events. Later this might change.
pub struct ScriptContexts<H: ScriptHost> {
    /// holds script contexts for all scripts given their instance ids
    pub context_entities: HashMap<u32, (Entity, H::ScriptContext)>,
}

impl<H: ScriptHost> Default for ScriptContexts<H> {
    fn default() -> Self {
        Self {
            context_entities: Default::default(),
        }
    }
}

impl<H: ScriptHost> ScriptContexts<H> {
    pub fn script_owner(&self, script_id: u32) -> Option<Entity> {
        self.context_entities.get(&script_id).map(|(e, _c)| *e)
    }

    pub fn insert_context(&mut self, script_id: u32, entity: Entity, ctx: H::ScriptContext) {
        self.context_entities.insert(script_id, (entity, ctx));
    }

    pub fn remove_context(&mut self, script_id: u32) {
        self.context_entities.remove(&script_id);
    }
}

/// A struct defining an instance of a script asset.
/// Multiple instances of the same script can exist on the same entity
pub struct Script<T: Asset> {
    /// a strong handle to the script asset
    handle: Handle<T>,

    /// the name of the script, usually its file name + relative asset path
    name: String,

    /// uniquely identifies the script instance (scripts which use the same asset don't necessarily have the same ID)
    id: u32,
}

impl<T: Asset> Script<T> {
    /// creates a new script instance with the given name and asset handle
    /// automatically gives this script instance a unique ID.
    /// No two scripts instances ever share the same ID
    pub fn new<H: ScriptHost>(name: String, handle: Handle<T>) -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        Self {
            handle,
            name,
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    #[inline(always)]
    /// returns the name of the script
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline(always)]
    /// returns the asset handle which this script is executing
    pub fn handle(&self) -> &Handle<T> {
        &self.handle
    }

    #[inline(always)]
    /// returns the unique ID of this script instance
    pub fn id(&self) -> u32 {
        self.id
    }

    /// reloads the script by deleting the old context and inserting a new one
    /// if the script context never existed, it will after this call.
    pub(crate) fn reload_script<H: ScriptHost>(
        script: &Script<H::ScriptAsset>,
        script_assets: &Res<Assets<H::ScriptAsset>>,
        contexts: &mut ResMut<ScriptContexts<H>>,
    ) {
        // retrieve owning entity
        let entity = contexts.script_owner(script.id()).unwrap();

        // remove old context
        contexts.remove_context(script.id());

        // insert new re-loaded context
        Self::insert_new_script_context(script, entity, script_assets, contexts);
    }

    /// inserts a new script context for the given script
    pub(crate) fn insert_new_script_context<H: ScriptHost>(
        new_script: &Script<H::ScriptAsset>,
        entity: Entity,
        script_assets: &Res<Assets<H::ScriptAsset>>,
        contexts: &mut ResMut<ScriptContexts<H>>,
    ) {
        let script = match script_assets.get(&new_script.handle) {
            Some(s) => s,
            None => {
                warn!(
                    "Script asset missing: {}. Did you make sure the script asset is loaded?",
                    new_script.name
                );
                // TODO: deal with component, remove ? or make ctx Optional
                return;
            }
        };

        match H::load_script(script.bytes(), &new_script.name) {
            Ok(mut ctx) => {
                contexts.insert_context(new_script.id(), entity, ctx);
            }
            Err(e) => {
                warn! {"Error in loading script {}:\n{}", &new_script.name,e}
                // TODO: deal with component, remove ? or make ctx Optional
            }
        }
    }
}

/// system state for exclusive systems dealing with script events
pub(crate) struct CachedScriptEventState<'w, 's, S: Send + Sync + 'static> {
    event_state: SystemState<EventReader<'w, 's, S>>,
}

impl<'w, 's, S: Send + Sync + 'static> FromWorld for CachedScriptEventState<'w, 's, S> {
    fn from_world(world: &mut World) -> Self {
        Self {
            event_state: SystemState::new(world),
        }
    }
}

/// Handles creating contexts for new/modified scripts
pub(crate) fn script_add_synchronizer<H: ScriptHost + 'static>(
    query: Query<
        (
            Entity,
            &ScriptCollection<H::ScriptAsset>,
            ChangeTrackers<ScriptCollection<H::ScriptAsset>>,
        ),
        Changed<ScriptCollection<H::ScriptAsset>>,
    >,
    mut contexts: ResMut<ScriptContexts<H>>,
    script_assets: Res<Assets<H::ScriptAsset>>,
) {
    query.for_each(|(entity, new_scripts, tracker)| {
        if tracker.is_added() {
            new_scripts.scripts.iter().for_each(|new_script| {
                Script::<H::ScriptAsset>::insert_new_script_context(
                    new_script,
                    entity,
                    &script_assets,
                    &mut contexts,
                )
            })
        } else {
            // changed but structure already exists in contexts
            // find out what's changed
            // we only care about added or removed scripts here
            // if the script asset gets changed we deal with that elsewhere

            let context_ids = contexts
                .context_entities
                .keys()
                .cloned()
                .collect::<HashSet<u32>>();
            let script_ids = new_scripts
                .scripts
                .iter()
                .map(|s| s.id())
                .collect::<HashSet<u32>>();

            let removed_scripts = context_ids.difference(&script_ids);
            let added_scripts = script_ids.difference(&context_ids);

            for r in removed_scripts {
                contexts.remove_context(*r);
            }

            for a in added_scripts {
                let script = new_scripts.scripts.iter().find(|e| &e.id == a).unwrap();

                Script::<H::ScriptAsset>::insert_new_script_context(
                    script,
                    entity,
                    &script_assets,
                    &mut contexts,
                )
            }
        }
    })
}

/// Handles the removal of script components and their contexts
pub(crate) fn script_remove_synchronizer<H: ScriptHost>(
    query: RemovedComponents<ScriptCollection<H::ScriptAsset>>,
    mut contexts: ResMut<ScriptContexts<H>>,
) {
    query.iter().for_each(|v| {
        // we know that this entity used to have a script component
        // ergo a script context must exist in ctxs, remove
        // all scripts on the entity
        contexts.remove_context(v.id());
    })
}

/// Reloads hot-reloaded scripts
pub(crate) fn script_hot_reload_handler<H: ScriptHost>(
    mut events: EventReader<AssetEvent<H::ScriptAsset>>,
    scripts: Query<&ScriptCollection<H::ScriptAsset>>,
    script_assets: Res<Assets<H::ScriptAsset>>,
    mut contexts: ResMut<ScriptContexts<H>>,
) {
    for e in events.iter() {
        match e {
            AssetEvent::Modified { handle } => {
                // find script using this handle by handle id
                for scripts in scripts.iter() {
                    for script in &scripts.scripts {
                        if &script.handle == handle {
                            Script::<H::ScriptAsset>::reload_script(
                                script,
                                &script_assets,
                                &mut contexts,
                            );
                        }
                    }
                }
            }
            _ => continue,
        }
    }
}

/// Lets the script host handle all script events
pub(crate) fn script_event_handler<H: ScriptHost>(world: &mut World) {
    world.resource_scope(
        |world, mut cached_state: Mut<CachedScriptEventState<H::ScriptEvent>>| {
            // we need to clone the events otherwise we cannot perform the subsequent query for scripts
            // assumption is that events are few, so this shouldn't be much of a problem
            let events: Vec<<H as ScriptHost>::ScriptEvent> = cached_state
                .event_state
                .get_mut(world)
                .iter()
                .cloned()
                .collect();

            match H::handle_events(world, &events) {
                Ok(_) => {}
                Err(e) => warn!("{}", e),
            }
        },
    );
}

/// A script host is the interface between your rust application
/// and the scripts in some interpreted language.
pub trait ScriptHost: Send + Sync + 'static {
    /// the type of the persistent script context, representing the execution context of the script
    type ScriptContext: Send + Sync + 'static;
    /// the type of events picked up by lua callbacks
    type ScriptEvent: Send + Sync + Clone + 'static;
    /// the type of asset representing the script files for this host
    type ScriptAsset: CodeAsset;

    /// Loads a script in byte array format, the script name can be used
    /// to send useful errors.
    fn load_script(path: &[u8], script_name: &str) -> Result<Self::ScriptContext>;

    /// the main point of contact with the bevy world.
    /// Scripts are called with appropriate events in the event order
    fn handle_events(world: &mut World, events: &[Self::ScriptEvent]) -> Result<()>;

    /// Registers the script host with the given app, and stage.
    /// all script events generated will be handled at the end of this stage. Ideally place after any game logic
    /// which can spawn/remove/modify scripts to avoid frame lag. (typically `CoreStage::Post_Update`)
    fn register_with_app(app: &mut App, stage: impl StageLabel);
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
