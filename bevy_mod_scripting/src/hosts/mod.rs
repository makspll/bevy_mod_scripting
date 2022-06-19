//! All script host related stuff

pub mod rhai_host;
pub mod rlua_host;

use bevy::{asset::Asset, ecs::system::SystemState, prelude::*, reflect::FromReflect};
use bevy_event_priority::PriorityEventReader;
pub use {crate::rhai_host::*, crate::rlua_host::*};

use std::{
    collections::{HashMap, HashSet},
    sync::atomic::{AtomicU32, Ordering},
};

use crate::{ScriptError, ScriptErrorEvent};

/// Describes the target set of scripts this event should
/// be handled by
#[derive(Clone, Debug)]
pub enum Recipients {
    /// Send to all scripts
    All,
    /// Send only to scripts on the given entity
    Entity(Entity),
    /// Send to script with the given ID
    ScriptID(u32),
    // Send to script with the given name
    ScriptName(String),
}

#[derive(Debug)]
pub struct FlatScriptData<'a> {
    sid: u32,
    entity: Entity,
    name: &'a str,
}

impl Recipients {
    /// Returns true if the given script should
    pub fn is_recipient(&self, c: &FlatScriptData) -> bool {
        match self {
            Recipients::All => true,
            Recipients::Entity(e) => e == &c.entity,
            Recipients::ScriptID(i) => i == &c.sid,
            Recipients::ScriptName(n) => n == c.name,
        }
    }
}

impl Default for Recipients {
    fn default() -> Self {
        Self::All
    }
}

pub trait ScriptEvent: Send + Sync + Clone + 'static {
    /// Retrieves the recipient scripts for this event
    fn recipients(&self) -> &Recipients;
}

/// A script host is the interface between your rust application
/// and the scripts in some interpreted language.
pub trait ScriptHost: Send + Sync + 'static + Default {
    /// the type of the persistent script context, representing the execution context of the script
    type ScriptContext: Send + Sync + 'static;
    /// the type of events picked up by lua callbacks
    type ScriptEvent: ScriptEvent;
    /// the type of asset representing the script files for this host
    type ScriptAsset: CodeAsset;
    /// the type representing the target of api providers, i.e. the 
    /// script engine or the thing which stores the API
    type APITarget;

    /// Loads a script in byte array format, the script name can be used
    /// to send useful errors.
    fn load_script(&mut self,script: &[u8], script_name: &str, providers: &APIProviders<Self::APITarget>) -> Result<Self::ScriptContext, ScriptError>;

    /// the main point of contact with the bevy world.
    /// Scripts are called with appropriate events in the event order
    fn handle_events<'a>(
        &self,
        world: &mut World,
        events: &[Self::ScriptEvent],
        ctxs: impl Iterator<Item = (FlatScriptData<'a>, &'a mut Self::ScriptContext)>,
    );

    /// Loads and runs script instantaneously without storing any script data into the world.
    /// The script receives the `world` global as normal, but `entity` is set to `u64::MAX`.
    /// The script id is set to `u32::MAX`.
    fn run_one_shot(&mut self, script: &[u8], script_name: &str, world: &mut World, event: Self::ScriptEvent) {
        
        let providers : &APIProviders<Self::APITarget> = world.resource();
        let mut ctx = self.load_script(script, script_name,providers).unwrap();

        let entity = Entity::from_bits(u64::MAX);

        let events = [event; 1];
        let ctx_iter = [(
            FlatScriptData {
                name: script_name,
                sid: u32::MAX,
                entity,
            },
            &mut ctx,
        ); 1]
            .into_iter();

        self.handle_events(world, &events, ctx_iter)
    }

    /// Registers the script host with the given app, and attaches handlers to deal with spawning/removing scripts at the given stage.
    ///
    /// Ideally place after any game logic which can spawn/remove/modify scripts to avoid frame lag. (typically `CoreStage::Post_Update`)
    fn register_with_app(app: &mut App, stage: impl StageLabel);
}

/// All code assets share this common interface.
/// When adding a new code asset don't forget to implement asset loading
/// and inserting appropriate systems when registering with the app
pub trait CodeAsset: Asset {
    fn bytes(&self) -> &[u8];
}

/// Implementers can modify a script context in order to enable
/// API access. ScriptHosts call `attach_api` when creating scripts
pub trait APIProvider: 'static + Send + Sync {
    type Target;

    /// provide the given script context with the API permamently
    fn attach_api(&self,ctx: &mut Self::Target) -> Result<(),ScriptError>;
}

/// Stores many API providers, used for 
pub struct APIProviders<T : 'static> {
    pub providers: Vec<Box<dyn APIProvider<Target=T>>>
}

impl <T : 'static>Default for APIProviders<T> {
    fn default() -> Self {
        Self { providers: Default::default() }
    }
}

impl <T : 'static>APIProviders<T> {
    pub fn attach_all(&self,ctx: &mut T) -> Result<(),ScriptError>{
        for p in self.providers.iter() {
            p.attach_api(ctx)?;
        }

        Ok(())
    }
}

#[derive(Component, Debug, FromReflect, Reflect)]
#[reflect(Component)]
/// The component storing many scripts.
/// Scripts receive information about the entity they are attached to
/// Scripts have unique identifiers and hence multiple copies of the same script
/// can be attached to the same entity
pub struct ScriptCollection<T: Asset> {
    pub scripts: Vec<Script<T>>,
}

impl<T: Asset> Default for ScriptCollection<T> {
    fn default() -> Self {
        Self {
            scripts: Default::default(),
        }
    }
}

/// A resource storing the script contexts for each script instance.
/// The reason we need this is to split the world borrow in our handle event systems, but this
/// has the added benefit that users don't see the contexts at all, and we can provide
/// generic handling for each new/removed script in one place.
///
/// We keep this public for now since there is no API for communicating with scripts
/// outside of events. Later this might change.
pub struct ScriptContexts<C> {
    /// holds script contexts for all scripts given their instance ids.
    /// This also stores contexts which are not fully loaded hence the Option
    pub context_entities: HashMap<u32, (Entity, Option<C>, String)>,
}

impl<C> Default for ScriptContexts<C> {
    fn default() -> Self {
        Self {
            context_entities: Default::default(),
        }
    }
}

impl<C> ScriptContexts<C> {
    pub fn script_owner(&self, script_id: u32) -> Option<Entity> {
        self.context_entities.get(&script_id).map(|(e, _c, _n)| *e)
    }

    pub fn insert_context(&mut self, fd: FlatScriptData, ctx: Option<C>) {
        self.context_entities
            .insert(fd.sid, (fd.entity, ctx, fd.name.to_owned()));
    }

    pub fn remove_context(&mut self, script_id: u32) {
        self.context_entities.remove(&script_id);
    }

    pub fn has_context(&self, script_id: u32) -> bool {
        self.context_entities
            .get(&script_id)
            .map_or(false, |(_, c, _)| c.is_some())
    }
}

/// A struct defining an instance of a script asset.
/// Multiple instances of the same script can exist on the same entity
#[derive(Debug, Reflect, FromReflect)]
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
        host: &mut H,
        script: &Script<H::ScriptAsset>,
        script_assets: &Assets<H::ScriptAsset>,
        providers: &APIProviders<H::APITarget>,
        contexts: &mut ScriptContexts<H::ScriptContext>,
    ) {
        // retrieve owning entity
        let entity = contexts.script_owner(script.id()).unwrap();

        // remove old context
        contexts.remove_context(script.id());

        // insert new re-loaded context
        Self::insert_new_script_context::<H>(host,script, entity, script_assets, providers, contexts);
    }

    /// inserts a new script context for the given script
    pub(crate) fn insert_new_script_context<H: ScriptHost>(
        host: &mut H,
        new_script: &Script<H::ScriptAsset>,
        entity: Entity,
        script_assets: &Assets<H::ScriptAsset>,
        providers: &APIProviders<H::APITarget>,
        contexts: &mut ScriptContexts<H::ScriptContext>,
    ) {
        let fd = FlatScriptData {
            sid: new_script.id(),
            entity,
            name: new_script.name(),
        };

        let script = match script_assets.get(&new_script.handle) {
            Some(s) => s,
            None => {
                // not loaded yet
                contexts.insert_context(fd, None);
                return;
            }
        };

        match host.load_script(script.bytes(), &new_script.name,providers) {
            Ok(ctx) => {
                contexts.insert_context(fd, Some(ctx));
            }
            Err(e) => {
                warn! {"Error in loading script {}:\n{}", &new_script.name,e}
                // this script will now never execute, unless manually reloaded
                // but contexts are left in a valid state
                contexts.insert_context(fd, None)
            }
        }
    }
}

/// system state for exclusive systems dealing with script events
pub(crate) struct CachedScriptEventState<'w, 's, H: ScriptHost> {
    event_state: SystemState<(
        PriorityEventReader<'w, 's, H::ScriptEvent>,
        EventWriter<'w, 's, ScriptErrorEvent>,
    )>,
}

impl<'w, 's, H: ScriptHost> FromWorld for CachedScriptEventState<'w, 's, H> {
    fn from_world(world: &mut World) -> Self {
        Self {
            event_state: SystemState::new(world),
        }
    }
}

/// Handles creating contexts for new/modified scripts
/// Scripts are likely not loaded instantly at this point, so most of the time
/// this system simply inserts an empty context
pub(crate) fn script_add_synchronizer<H: ScriptHost + 'static>(
    query: Query<
        (
            Entity,
            &ScriptCollection<H::ScriptAsset>,
            ChangeTrackers<ScriptCollection<H::ScriptAsset>>,
        ),
        Changed<ScriptCollection<H::ScriptAsset>>,
    >,
    mut host: ResMut<H>,
    providers: Res<APIProviders<H::APITarget>>,
    script_assets: Res<Assets<H::ScriptAsset>>,
    mut contexts: ResMut<ScriptContexts<H::ScriptContext>>,
) {
    query.for_each(|(entity, new_scripts, tracker)| {
        if tracker.is_added() {
            new_scripts.scripts.iter().for_each(|new_script| {
                Script::<H::ScriptAsset>::insert_new_script_context::<H>(
                    &mut host,
                    new_script,
                    entity,
                    &script_assets,
                    &providers,
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
                .iter()
                .filter_map(|(sid, (e, _, _))| if *e == entity { Some(sid) } else { None })
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
                Script::<H::ScriptAsset>::insert_new_script_context::<H>(
                    &mut host,
                    script,
                    entity,
                    &script_assets,
                    &providers,
                    &mut contexts,
                )
            }
        }
    })
}

/// Handles the removal of script components and their contexts
pub(crate) fn script_remove_synchronizer<H: ScriptHost>(
    query: RemovedComponents<ScriptCollection<H::ScriptAsset>>,
    mut contexts: ResMut<ScriptContexts<H::ScriptContext>>,
) {
    query.iter().for_each(|v| {
        // we know that this entity used to have a script component
        // ergo a script context must exist in ctxts, remove all scripts on the entity
        contexts.remove_context(v.id());
    })
}

/// Reloads hot-reloaded scripts, or loads missing contexts for scripts which were added but not loaded
pub(crate) fn script_hot_reload_handler<H: ScriptHost>(
    mut events: EventReader<AssetEvent<H::ScriptAsset>>,
    mut host: ResMut<H>,
    scripts: Query<&ScriptCollection<H::ScriptAsset>>,
    script_assets: Res<Assets<H::ScriptAsset>>,
    providers: Res<APIProviders<H::APITarget>>,
    mut contexts: ResMut<ScriptContexts<H::ScriptContext>>,
) {
    for e in events.iter() {
        let (handle, created) = match e {
            AssetEvent::Modified { handle } => (handle, false),
            AssetEvent::Created { handle } => (handle, true),
            _ => continue,
        };

        // find script using this handle by handle id
        // whether this script was modified or created
        // if a script exists with this handle, we should reload it to load in a new context
        // which at this point will be either None or Some(outdated context)
        // both ways are fine
        for scripts in scripts.iter() {
            for script in &scripts.scripts {
                // the script could have well loaded in the same frame that it was added
                // in that case it will have a context attached and we do not want to reload it
                if &script.handle == handle && !(contexts.has_context(script.id()) && created) {
                    Script::<H::ScriptAsset>::reload_script::<H>(
                        &mut host,
                        script,
                        &script_assets,
                        &providers,
                        &mut contexts,
                    );
                }
            }
        }
    }
}

/// Lets the script host handle all script events
pub(crate) fn script_event_handler<H: ScriptHost, const MAX: u32, const MIN: u32>(
    world: &mut World,
) {
    // we need to collect the events to drop the borrow of the world
    let events = world.resource_scope(|world, mut cached_state: Mut<CachedScriptEventState<H>>| {
        let (mut cached_state, _) = cached_state.event_state.get_mut(world);
        cached_state
            .iter_prio_range(MAX, MIN)
            .collect::<Vec<H::ScriptEvent>>()
    });

    // we need a resource scope to be able to simultaneously access the contexts as well
    // as provide world access to scripts
    // afaik there is not really a better way to do this in bevy just now
    world.resource_scope(|world, mut ctxts: Mut<ScriptContexts<H::ScriptContext>>| {
        let ctx_iter =
            ctxts
                .as_mut()
                .context_entities
                .iter_mut()
                .filter_map(|(sid, (entity, o, name))| {
                    let ctx = match o {
                        Some(v) => v,
                        None => return None,
                    };

                    Some((
                        FlatScriptData {
                            sid: *sid,
                            entity: *entity,
                            name,
                        },
                        ctx,
                    ))
                });
        world.resource_scope(|world, host: Mut<H>|
            host.handle_events(world, &events, ctx_iter)
        );
    });
}
