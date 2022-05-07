pub mod rlua_host;
use std::{collections::HashMap};
use bevy::{prelude::{World, App, StageLabel, RemovedComponents, ResMut, Res, Assets, warn, Query, Added, FromWorld, EventReader, Handle, Component, Mut}, asset::Asset, ecs::system::SystemState};
pub use {rlua_host::*};
use anyhow::{Result};

pub trait AddScriptHost {
    fn add_script_host<T : ScriptHost>(&mut self) -> &mut Self; 
}


pub trait CodeAsset : Asset {
    fn bytes<'a>(&'a self) -> &'a [u8];
}

pub trait APIProvider : 'static + Default {
    type Ctx;
    fn attach_api(ctx : &Self::Ctx);
}


#[derive(Component)]
pub struct Script<T : Asset> {
    pub handle : Handle<T>,
    pub name : String,
}

#[derive(Default)]
pub struct ScriptContexts<H : ScriptHost> {
    pub contexts : HashMap<String,H::ScriptContext>
}




pub struct CachedScriptEventState<'w, 's, S : Send + Sync + 'static> 
{
    event_state: SystemState<EventReader<'w, 's, S>>
}

impl <'w,'s, S : Send + Sync + 'static>FromWorld for CachedScriptEventState<'w,'s,S>{
    fn from_world(world: &mut World) -> Self {
        Self {
            event_state: SystemState::new(world)
        }
    }
}

pub fn script_add_synchronizer<H : ScriptHost + 'static>(
    mut query: Query<(&Script<H::ScriptAssetType>,Added<Script<H::ScriptAssetType>>)>,
    mut contexts: ResMut<ScriptContexts<H>>,
    mut script_assets : Res<Assets<H::ScriptAssetType>>
){

    query.for_each(|(new_script,added)| {

        
        let script = match script_assets.get(&new_script.handle) {
            Some(s) => s,
            None => {warn!("Failed to load script: {}", new_script.name); return;},
        };

        match H::load_script(&script.bytes(), &new_script.name){
            Ok(ctx) => {
                H::ScriptAPIProvider::attach_api(&ctx);

                contexts.contexts.insert(new_script.name.clone(),ctx);

            },
            Err(e) => {
                warn!{"Failed to load script: {}", new_script.name}
                // TODO: deal with component, remove ? or make ctx Optional
            },
        }

    })
}

pub fn script_remove_synchronizer<H : ScriptHost + 'static>(
    mut query: RemovedComponents<Script<H::ScriptAssetType>>,
    mut contexts: ResMut<ScriptContexts<H>>,
    mut script_assets : Res<Assets<H::ScriptAssetType>>
){


    // TODO : this functionality
}

pub fn script_event_handler<H: ScriptHost>  (
    world : &mut World,
)
{

    world.resource_scope(|world, mut cached_state: Mut<CachedScriptEventState<H::ScriptEventType>>| {
        
        // we need to clone the events otherwise we cannot perform the subsequent query for scripts
        // assumption is that events are few, so this shouldn't be much of a problem
        let events = cached_state.event_state
                .get_mut(world)
                .iter()
                .map(|v| v.clone())
                .collect();


        match H::handle_events(world,&events){
            Ok(_) => return,
            Err(e) => warn!("{}",e),
        }
    });

}

pub trait ScriptHost : Send + Sync{
    type ScriptContext : Send + Sync + 'static;
    type ScriptEventType : Send + Sync + Clone + 'static;
    type ScriptAssetType : CodeAsset;
    type ScriptAPIProvider : APIProvider<Ctx=Self::ScriptContext>;

    fn load_script(path : &[u8], script_name : &str) -> Result<Self::ScriptContext>;
    fn handle_events(world : &mut World, events : &Vec<Self::ScriptEventType>) -> Result<()>;

    /// registers the script host with the given app, and stage.
    /// all script events generated will be handled at the end of this stage. Ideally place after update 
    fn register_with_app(app : &mut App, stage : impl StageLabel);
}   