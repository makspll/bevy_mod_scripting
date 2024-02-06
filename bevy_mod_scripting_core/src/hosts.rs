//! All script host related stuff
use bevy::{asset::Asset, ecs::schedule::ScheduleLabel, prelude::*};
use std::ops::DerefMut;
use std::{
    collections::HashMap,
    iter::once,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::world::WorldPointerGuard;
use crate::{
    asset::CodeAsset,
    docs::DocFragment,
    error::ScriptError,
    event::{ScriptEvent, ScriptLoaded},
    world::WorldPointer,
};

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
/// Data used to describe a script instance.
pub struct ScriptData<'a> {
    pub sid: u32,
    pub entity: Entity,
    pub name: &'a str,
}

impl Recipients {
    /// Returns true if the given script is a recipient
    pub fn is_recipient(&self, c: &ScriptData) -> bool {
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

/// A script host is the interface between your rust application
/// and the scripts in some interpreted language.
pub trait ScriptHost: Send + Sync + 'static + Default + Resource {
    /// the type of the persistent script context, representing the execution context of the script
    type ScriptContext: Send + Sync + 'static;
    /// the type of events picked up by lua callbacks
    type ScriptEvent: ScriptEvent;
    /// the type of asset representing the script files for this host
    type ScriptAsset: CodeAsset;
    /// the type representing the target of api providers, i.e. the
    /// script engine or the script context itself
    type APITarget: Send + Sync + 'static;
    /// the type of each doc fragment
    type DocTarget: DocFragment;

    /// Loads a script in byte array format, the script name can be used
    /// to send useful errors.
    fn load_script(
        &mut self,
        world_ptr: WorldPointer,
        script: &[u8],
        script_data: &ScriptData,
        providers: &mut APIProviders<Self>,
    ) -> Result<Self::ScriptContext, ScriptError>;

    /// Perform one-off initialization of scripts (happens for every new or re-loaded script)
    fn setup_script(
        &mut self,
        script_data: &ScriptData,
        ctx: &mut Self::ScriptContext,
        providers: &mut APIProviders<Self>,
    ) -> Result<(), ScriptError>;

    /// the main point of contact with the bevy world.
    /// Scripts are called with appropriate events in the event order
    fn handle_events<'a>(
        &mut self,
        world_ptr: &mut World,
        events: &[Self::ScriptEvent],
        ctxs: impl Iterator<Item = (ScriptData<'a>, &'a mut Self::ScriptContext)>,
        providers: &mut APIProviders<Self>,
    );

    /// Loads and runs script instantaneously without storing any script data into the world.
    /// The script id is set to `u32::MAX`.
    fn run_one_shot(
        &mut self,
        script: &[u8],
        script_name: &str,
        entity: Entity,
        world: &mut World,
        event: Self::ScriptEvent,
    ) -> Result<(), ScriptError> {
        let fd = ScriptData {
            name: script_name,
            sid: u32::MAX,
            entity,
        };

        let mut providers: APIProviders<Self> = world.remove_resource().unwrap();
        // safety:
        // - we have &mut World access
        // - we do not use the original reference again anywhere in this function after this was created
        let world = unsafe { WorldPointerGuard::new(world) };
        let mut ctx = self
            .load_script(world.clone(), script, &fd, &mut providers)
            .unwrap();
        self.setup_script(&fd, &mut ctx, &mut providers)?;
        let events = [event; 1];

        let mut world = world.write();

        self.handle_events(
            world.deref_mut(),
            &events,
            once((fd, &mut ctx)),
            &mut providers,
        );

        world.insert_resource(providers);

        Ok(())
    }

    /// Registers the script host with the given app, and attaches handlers to deal with spawning/removing scripts in the given System Set.
    ///
    /// Ideally place after any game logic which can spawn/remove/modify scripts to avoid frame lag. (typically `PostUpdate`)
    fn register_with_app(app: &mut App, schedule: impl ScheduleLabel) {
        #[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone, Copy)]
        struct DummySet;

        Self::register_with_app_in_set(app, schedule, DummySet);
    }

    /// Similar to `register_with_app` but allows you to specify a system set to add the handler to.
    fn register_with_app_in_set(app: &mut App, schedule: impl ScheduleLabel, set: impl SystemSet);
}

/// Implementors can modify a script context in order to enable
/// API access. ScriptHosts call `attach_api` when creating scripts
pub trait APIProvider: 'static + Send + Sync {
    /// the type of script engine/context the API is attached to, this must be the same as the APITarget of the ScriptHost meant to receive it.
    type APITarget: Send + Sync + 'static;
    /// The type of script context the APIProvider works with, must be the same as the ScriptContext of the target ScriptHost.
    type ScriptContext: Send + Sync + 'static;
    /// The type of documentation fragment produced by the APIProvider, must be the same as the DocTarget of the target ScriptHost.
    type DocTarget: DocFragment;

    /// provide the given script context with the API permamently.
    /// Depending on the host, API's may be attached on a per-script basis
    /// or on a per-engine basis. Rhai for example allows you to decouple the State of each script from the
    /// engine. For one-time setup use `Self::setup_script`
    fn attach_api(&mut self, api: &mut Self::APITarget) -> Result<(), ScriptError>;

    /// Hook executed every time a script is about to handle events, most notably used to "refresh" world pointers
    fn setup_script_runtime(
        &mut self,
        _world_ptr: WorldPointer,
        _script_data: &ScriptData,
        _ctx: &mut Self::ScriptContext,
    ) -> Result<(), ScriptError> {
        Ok(())
    }

    /// Setup meant to be executed once for every single script. Use this if you need to consistently setup scripts.
    /// For API's use `Self::attach_api` instead.
    fn setup_script(
        &mut self,
        _script_data: &ScriptData,
        _ctx: &mut Self::ScriptContext,
    ) -> Result<(), ScriptError> {
        Ok(())
    }

    /// Generate a piece of documentation to be merged with the other documentation fragments
    /// provided by other API providers
    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        None
    }

    /// Some providers might provide additional types which need to be registered
    /// with the reflection API to work.
    fn register_with_app(&self, _app: &mut App) {}
}

#[derive(Resource)]
/// Stores many API providers
pub struct APIProviders<T: ScriptHost> {
    pub providers: Vec<
        Box<
            dyn APIProvider<
                APITarget = T::APITarget,
                DocTarget = T::DocTarget,
                ScriptContext = T::ScriptContext,
            >,
        >,
    >,
}

impl<T: ScriptHost> Default for APIProviders<T> {
    fn default() -> Self {
        Self {
            providers: Default::default(),
        }
    }
}

impl<T: ScriptHost> APIProviders<T> {
    pub fn attach_all(&mut self, ctx: &mut T::APITarget) -> Result<(), ScriptError> {
        for p in self.providers.iter_mut() {
            p.attach_api(ctx)?;
        }

        Ok(())
    }

    pub fn setup_runtime_all(
        &mut self,
        world_ptr: WorldPointer,
        script_data: &ScriptData,
        ctx: &mut T::ScriptContext,
    ) -> Result<(), ScriptError> {
        for p in self.providers.iter_mut() {
            p.setup_script_runtime(world_ptr.clone(), script_data, ctx)?;
        }

        Ok(())
    }

    pub fn setup_all(
        &mut self,
        script_data: &ScriptData,
        ctx: &mut T::ScriptContext,
    ) -> Result<(), ScriptError> {
        for p in self.providers.iter_mut() {
            p.setup_script(script_data, ctx)?;
        }

        Ok(())
    }

    pub fn gen_all(&self) -> Result<(), ScriptError> {
        let mut d: Option<T::DocTarget> = None;
        for p in self.providers.iter() {
            if let Some(f) = p.get_doc_fragment() {
                if let Some(prev) = d {
                    d = Some(prev.merge(f))
                } else {
                    d = Some(f)
                }
            }
        }
        d.map(|d| d.gen_docs()).unwrap_or_else(|| Ok(()))
    }
}

/// A resource storing the script contexts for each script instance.
/// The reason we need this is to split the world borrow in our handle event systems, but this
/// has the added benefit that users don't see the contexts at all, and we can provide
/// generic handling for each new/removed script in one place.
///
/// We keep this public for now since there is no API for communicating with scripts
/// outside of events. Later this might change.
#[derive(Resource)]
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

    pub fn insert_context(&mut self, fd: ScriptData, ctx: Option<C>) {
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

    pub fn is_empty(&self) -> bool {
        self.context_entities.is_empty()
    }
}

/// A struct defining an instance of a script asset.
/// Multiple instances of the same script can exist on the same entity
#[derive(Debug, Reflect)]
pub struct Script<T: Asset> {
    /// a strong handle to the script asset
    handle: Handle<T>,

    /// the name of the script, usually its file name + relative asset path
    name: String,

    /// uniquely identifies the script instance (scripts which use the same asset don't necessarily have the same ID)
    id: u32,
}

static COUNTER: AtomicU32 = AtomicU32::new(0);

impl<T: Asset> Script<T> {
    /// creates a new script instance with the given name and asset handle
    /// automatically gives this script instance a unique ID.
    /// No two scripts instances ever share the same ID
    pub fn new(name: String, handle: Handle<T>) -> Self {
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
        world: &mut World,
        host: &mut H,
        script: &Script<H::ScriptAsset>,
        script_assets: &Assets<H::ScriptAsset>,
        providers: &mut APIProviders<H>,
        contexts: &mut ScriptContexts<H::ScriptContext>,
    ) {
        debug!("reloading script {}", script.id);

        // retrieve owning entity
        if let Some(entity) = contexts.script_owner(script.id()) {
            // remove old context
            contexts.remove_context(script.id());
            // insert new re-loaded context
            Self::insert_new_script_context::<H>(
                world,
                host,
                script,
                entity,
                script_assets,
                providers,
                contexts,
            );
        } else {
            // remove old context
            contexts.remove_context(script.id());
        }
    }

    /// checks if a script has loaded, and if so loads (`ScriptHost::load_script`),
    /// sets up (`ScriptHost::setup_script`) and inserts its new context into the contexts resource
    /// otherwise inserts None. Sends ScriptLoaded event if the script was loaded
    pub(crate) fn insert_new_script_context<H: ScriptHost>(
        world: &mut World,
        host: &mut H,
        new_script: &Script<H::ScriptAsset>,
        entity: Entity,
        script_assets: &Assets<H::ScriptAsset>,
        providers: &mut APIProviders<H>,
        contexts: &mut ScriptContexts<H::ScriptContext>,
    ) {
        // safety:
        // - we have &mut World access
        // - we do not use the original reference again anywhere in this function
        let world = unsafe { WorldPointerGuard::new(world) };

        let fd = ScriptData {
            sid: new_script.id(),
            entity,
            name: new_script.name(),
        };

        let script = match script_assets.get(&new_script.handle) {
            Some(s) => s,
            None => {
                // not loaded yet
                debug!("Inserted script which hasn't loaded yet {:?}", fd);
                contexts.insert_context(fd, None);
                return;
            }
        };
        debug!("Inserted script {:?}", fd);

        match host.load_script(world.clone(), script.bytes(), &fd, providers) {
            Ok(mut ctx) => {
                host.setup_script(&fd, &mut ctx, providers)
                    .expect("Failed to setup script");
                contexts.insert_context(fd, Some(ctx));
                {
                    let mut world = world.write();
                    world.resource_scope(|_, mut event_writer: Mut<Events<ScriptLoaded>>| {
                        event_writer.send(ScriptLoaded {
                            sid: new_script.id(),
                        });
                    })
                }
            }
            Err(e) => {
                warn! {"Error in loading script {}:\n{}", &new_script.name,e}
                // this script will now never execute, unless manually reloaded
                // but contexts are left in a valid state
                contexts.insert_context(fd, None);
            }
        }
    }
}

/// Allows the script handles to be cloned along with the explicit bevy asset handle clone
impl<T: Asset> Clone for Script<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            name: self.name.clone(),
            id: self.id,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component, Default)]
/// The component storing many scripts.
/// Scripts receive information about the entity they are attached to
/// Scripts have unique identifiers and hence multiple copies of the same script
/// can be attached to the same entity
pub struct ScriptCollection<T: Asset> {
    pub scripts: Vec<Script<T>>,
}

impl<T: Asset> Clone for ScriptCollection<T> {
    fn clone(&self) -> Self {
        Self {
            scripts: self.scripts.clone(),
        }
    }
}

impl<T: Asset> Default for ScriptCollection<T> {
    fn default() -> Self {
        Self {
            scripts: Default::default(),
        }
    }
}
