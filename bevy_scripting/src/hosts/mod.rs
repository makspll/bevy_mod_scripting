pub mod rlua_host;
use anyhow::Result;
use bevy::{
    asset::Asset,
    ecs::system::SystemState,
    prelude::{
        warn, Added, App, Assets, Component, EventReader, FromWorld, Handle, Mut, Query,
        RemovedComponents, Res, ResMut, StageLabel, World, Entity,
    },
};
pub use rlua_host::*;
use std::collections::HashMap;

pub trait AddScriptHost {
    fn add_script_host<T: ScriptHost>(&mut self) -> &mut Self;
}

pub trait CodeAsset: Asset {
    fn bytes(&self) -> &[u8];
}

pub trait APIProvider: 'static + Default {
    type Ctx;
    fn attach_api(ctx: &Self::Ctx);
}

#[derive(Component)]
pub struct Script<T: Asset> {
    pub handle: Handle<T>,
    pub name: String,
}

#[derive(Default)]
pub struct ScriptContexts<H: ScriptHost> {
    /// holds script contexts for all scripts
    /// and keeps track of which entities they're attached to
    pub contexts: HashMap<u32,HashMap<String, H::ScriptContext>>,
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
        (Entity,&Script<H::ScriptAssetType>),
        Added<Script<H::ScriptAssetType>>,
    >,
    mut contexts: ResMut<ScriptContexts<H>>,
    script_assets: Res<Assets<H::ScriptAssetType>>,
) {

    query.for_each(|(entity,new_script)| {
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
                
                let name_map= contexts.contexts
                    .entry(entity.id())
                    .or_default();
                
                // if the script already exists on an entity, panic
                // not allowed at least for now
                if name_map.contains_key(&new_script.name){
                    panic!("Attempted to attach script: {} to entity which already has another copy of this script attached", new_script.name);
                } 

                name_map.insert(new_script.name.clone(),ctx);
                    
            }
            Err(_e) => {
                warn! {"Failed to load script: {}", new_script.name}
                // TODO: deal with component, remove ? or make ctx Optional
            }
        }
    })
}

pub fn script_remove_synchronizer<H: ScriptHost + 'static>(
    query: RemovedComponents<Script<H::ScriptAssetType>>,
    mut contexts: ResMut<ScriptContexts<H>>,
) {

    let ctxs = &mut contexts.contexts;
    query.iter().for_each(|v| {
        // we know that this entity used to have a script component
        // ergo a script context must exist in ctxs, remove it 
        ctxs.remove(&v.id());

    })
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

pub trait ScriptHost: Send + Sync {
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
