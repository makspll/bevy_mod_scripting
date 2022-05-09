pub mod rlua_host;
use anyhow::Result;
use bevy::{asset::Asset, ecs::system::SystemState, prelude::*};
pub use rlua_host::*;
use std::{collections::{HashMap, HashSet}, sync::atomic::{AtomicU32, Ordering}};

pub trait AddScriptHost {
    fn add_script_host<T: ScriptHost>(&mut self) -> &mut Self;
}

pub trait CodeAsset: Asset {
    fn bytes(&self) -> &[u8];
}

/// Implementers can modify a script context in order to enable
/// API access. ScriptHosts call `attach_api` when creating scripts
pub trait APIProvider: 'static + Default {
    type Ctx;
    fn attach_api(ctx: &Self::Ctx);
}

#[derive(Component)]
pub struct ScriptCollection<T: Asset> {
    pub scripts: Vec<Script<T>>,
}

#[derive(Default)]
pub struct ScriptContexts<H: ScriptHost> {
    /// holds script contexts for all scripts given their instance ids
    pub contexts: HashMap<u32, H::ScriptContext>,
}


/// A struct defining an instance of a script asset
pub struct Script<T: Asset> {

    /// a strong handle to the script asset 
    handle: Handle<T>,

    /// the name of the script, usually its file name + relative asset path
    name: String,

    /// uniquely identifies the script instance (scripts which use the same asset don't necessarily have the same ID)
    id: u32,
}

impl<T: Asset> Script<T> {
    pub fn new<H: ScriptHost>(name: String, handle: Handle<T>) -> Self {
        static COUNTER:AtomicU32 = AtomicU32::new(0);
        Self { handle, name, id:COUNTER.fetch_add(1,Ordering::Relaxed)}
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn handle(&self) -> &Handle<T> {
        &self.handle
    }
    pub fn id(&self) -> u32 {
        self.id
    }

    fn reload_script<H: ScriptHost> (
        script: &Script<H::ScriptAssetType>,
        script_assets: &Res<Assets<H::ScriptAssetType>>,
        contexts : &mut ResMut<ScriptContexts<H>>
    ){
        // remove old context 
        contexts.contexts.remove(&script.id);

        // insert new re-loaded context
        Self::insert_new_script_context(script,script_assets,contexts)
    }

    fn insert_new_script_context<H: ScriptHost>(
        new_script: &Script<H::ScriptAssetType>,
        script_assets: &Res<Assets<H::ScriptAssetType>>,
        contexts: &mut ResMut<ScriptContexts<H>>,
    ) {
        let script = match script_assets.get(&new_script.handle) {
            Some(s) => s,
            None => {
                warn!("Failed to load script: {}", new_script.name);
                return;
            }
        };

        match H::load_script(script.bytes(), &new_script.name) {
            Ok(ctx) => {
                // allow plugging in an API
                H::ScriptAPIProvider::attach_api(&ctx);

                contexts.contexts.insert(new_script.id(), ctx);
            }
            Err(_e) => {
                warn! {"Failed to load script: {}", &new_script.name}
                // TODO: deal with component, remove ? or make ctx Optional
            }
        }
    }
}


pub struct CachedScriptEventState<'w, 's, S: Send + Sync + 'static> {
    event_state: SystemState<EventReader<'w, 's, S>>,
}

impl<'w, 's, S: Send + Sync + 'static> FromWorld for CachedScriptEventState<'w, 's, S> {
    fn from_world(world: &mut World) -> Self {
        Self {
            event_state: SystemState::new(world),
        }
    }
}

pub fn script_add_synchronizer<H: ScriptHost + 'static>(
    query: Query<
        (
            &ScriptCollection<H::ScriptAssetType>,
            ChangeTrackers<ScriptCollection<H::ScriptAssetType>>,
        ),
        Changed<ScriptCollection<H::ScriptAssetType>>,
    >,
    mut contexts: ResMut<ScriptContexts<H>>,
    script_assets: Res<Assets<H::ScriptAssetType>>,
) {
    query.for_each(|(new_scripts, tracker)| {
        if tracker.is_added() {
            new_scripts.scripts.iter().for_each(|new_script| {
                Script::<H::ScriptAssetType>::insert_new_script_context(
                    new_script,
                    &script_assets,
                    &mut contexts,
                )
            })
        } else {
            // changed but structure already exists in contexts
            // find out what's changed
            // we only care about added or removed scripts here
            // if the script asset gets changed we deal with that elsewhere

            let context_ids = contexts.contexts.keys().cloned().collect::<HashSet<u32>>();
            let script_ids = new_scripts.scripts.iter()
                .map(|s| s.id())
                .collect::<HashSet<u32>>();

            let removed_scripts = context_ids.difference(&script_ids);
            let added_scripts = script_ids.difference(&context_ids);

            for r in removed_scripts {
                contexts.contexts.remove(r);
            }

            for a in added_scripts {
                let script = new_scripts.scripts
                    .iter()
                    .find(|e| &e.id == a).unwrap();

                Script::<H::ScriptAssetType>::insert_new_script_context(
                    script,
                    &script_assets,
                    &mut contexts,
                )
            }
        }
    })
}

pub fn script_remove_synchronizer<H: ScriptHost>(
    query: RemovedComponents<ScriptCollection<H::ScriptAssetType>>,
    mut contexts: ResMut<ScriptContexts<H>>,
) {
    let ctxs = &mut contexts.contexts;

    query.iter().for_each(|v| {
        // we know that this entity used to have a script component
        // ergo a script context must exist in ctxs, remove
        // all scripts on the entity
        ctxs.remove(&v.id());
    })
}

pub fn script_hot_reload_handler<H: ScriptHost>(
    mut events : EventReader<AssetEvent<H::ScriptAssetType>>,
    scripts : Query<&ScriptCollection<H::ScriptAssetType>>,
    script_assets: Res<Assets<H::ScriptAssetType>>,
    mut contexts: ResMut<ScriptContexts<H>>
) {
    for e in events.iter() {
        match e {
            AssetEvent::Modified { handle } => {
                // find script using this handle by handle id 
                'outer : for scripts in scripts.iter() {
                    for script in &scripts.scripts {
                        if &script.handle == handle{
                            // reload the script 
                            Script::<H::ScriptAssetType>::reload_script(&script,&script_assets,&mut contexts);
                            
                            // update other changed assets
                            break 'outer
                        }
                    }
                }
            },
            _ => continue,
        }
    }
}

pub fn script_event_handler<H: ScriptHost>(world: &mut World) {
    world.resource_scope(
        |world, mut cached_state: Mut<CachedScriptEventState<H::ScriptEventType>>| {
            // we need to clone the events otherwise we cannot perform the subsequent query for scripts
            // assumption is that events are few, so this shouldn't be much of a problem
            let events: Vec<<H as ScriptHost>::ScriptEventType> = cached_state
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

pub trait ScriptHost: Send + Sync + 'static {
    type ScriptContext: Send + Sync + 'static;
    type ScriptEventType: Send + Sync + Clone + 'static;
    type ScriptAssetType: CodeAsset;
    type ScriptAPIProvider: APIProvider<Ctx = Self::ScriptContext>;

    fn load_script(path: &[u8], script_name: &str) -> Result<Self::ScriptContext>;
    fn handle_events(world: &mut World, events: &[Self::ScriptEventType]) -> Result<()>;

    /// registers the script host with the given app, and stage.
    /// all script events generated will be handled at the end of this stage. Ideally place after update
    fn register_with_app(app: &mut App, stage: impl StageLabel);
}
