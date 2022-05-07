use std::ffi::c_void;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::{MutexGuard, Mutex, Arc};

use bevy::asset::{AssetLoader, LoadedAsset};
use bevy::ecs::world::WorldCell;
use bevy::prelude::{World, info, App, IntoExclusiveSystem, Mut, CoreStage, ParallelSystemDescriptorCoercion, ExclusiveSystemDescriptorCoercion, StageLabel, SystemSet};
use bevy::reflect::TypeUuid;
use once_cell::sync::{OnceCell, Lazy};
use rlua::{Lua, Function, MultiValue, ToLua, Context, ToLuaMulti};
use rlua::prelude::*;
use crate::{ScriptHost, CachedScriptEventState, script_event_handler, ScriptContexts, CodeAsset, script_add_synchronizer, APIProvider};
use anyhow::{anyhow,Result};
use beau_collector::BeauCollector as _;




#[derive(Debug, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct LuaFile {
    pub bytes: Arc<[u8]>,
}

impl CodeAsset for LuaFile {
    fn bytes<'a>(&'a self) -> &'a [u8] {
        &self.bytes
    }
}


#[derive(Default)]
pub struct LuaLoader;

impl AssetLoader for LuaLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>> {
        load_context.set_default_asset(LoadedAsset::new(LuaFile {
            bytes: bytes.into(),
        }));
        Box::pin(async move { Ok(()) })
    }

    fn extensions(&self) -> &[&str] {
        &["lua"]
    }
}



/// defines a value allowed to be passed as lua script arguments for callbacks
#[derive(Clone)]
pub enum LuaCallbackArgument {
    Integer(usize)
}


impl <'lua>ToLua<'lua> for LuaCallbackArgument {
    fn to_lua(self, lua: Context<'lua>) -> LuaResult<LuaValue<'lua>> {
        match self {
            LuaCallbackArgument::Integer(i) => i.to_lua(lua),
        }
    }
}
#[derive(Clone)]
pub struct LuaEvent{
    pub hook_name: String,
    pub args: Vec<LuaCallbackArgument>,
}



#[derive(Default)]
pub struct RLuaScriptHost<A : APIProvider>{
    _ph : PhantomData<A>
}


unsafe impl <A : APIProvider> Send for RLuaScriptHost<A>{}
unsafe impl <A : APIProvider> Sync for RLuaScriptHost<A>{}




impl <A: APIProvider<Ctx=Mutex<Lua>>>ScriptHost for RLuaScriptHost<A>{
    type ScriptContext = Mutex<Lua>;
    type ScriptEventType = LuaEvent;
    type ScriptAssetType = LuaFile;
    type ScriptAPIProvider = A;

    fn register_with_app(app : &mut App, stage : impl StageLabel){
        app.add_event::<LuaEvent>();
        app.init_resource::<CachedScriptEventState<Self::ScriptEventType>>();
        app.init_resource::<ScriptContexts<Self>>();


        app.add_system_set_to_stage(stage,
            SystemSet::new()
                .with_system(script_add_synchronizer::<Self>)
                .with_system(script_event_handler::<Self>.exclusive_system()
                    .at_end())
            );

    }


    fn load_script(script : &[u8], script_name : &str) -> Result<Self::ScriptContext> {
        let lua = Lua::new();
        lua.context::<_,Result<()>>(|lua_ctx| {


            lua_ctx
                .load(script)
                .set_name(script_name)
                .map(|c| c.exec())
                .map_err(|_e| anyhow!("Error loading script {}",script_name))?;

            Ok(())
        })?;

        Ok(Mutex::new(lua))
    }


    fn handle_events(world : &mut World, events : &Vec<Self::ScriptEventType>) -> Result<()>{

        // we need to do this since scripts need access to the world, but they also 
        // live in it, hence we only store indices into a resource which can then be scoped
        // instead of storing contexts directly on the components
        world.resource_scope(|world, res:  Mut<ScriptContexts<Self>>|{
            res.contexts.iter().map(|(script_name,ctx)| {
                let lua_ctx = ctx.lock().unwrap();

                lua_ctx.context::<_,Result<()>>(|lua_ctx|{
                    let globals = lua_ctx.globals();
                    lua_ctx.scope::<_,Result<()>>(|lua_scope| {
                        
                        globals
                            .set("world", 
                            LuaLightUserData(world as *mut World as *mut c_void))?;
                        
                        
                        // event order is preserved, but scripts can't rely on any temporal
                        // guarantees when it comes to other scripts callbacks,
                        // at least for now
                        for event in events.into_iter() {
                            let f : Function = match globals.get(event.hook_name.clone()) {
                                Ok(f) => f,
                                Err(_) => continue, // not subscribed to this event
                            };
                    
                            f.call::<MultiValue,()>(event.args.clone().to_lua_multi(lua_ctx)?)
                                .map_err(|e| anyhow!("Runtime LUA error: {}",e))?;
                        };

                        Ok(())
                    })
                })
            }).bcollect()             
        })      
    }


}


impl <API : APIProvider<Ctx=Mutex<Lua>>>RLuaScriptHost<API> {
    pub fn register_api_callback<F,A,R>(callback_fn_name : &str, callback : F, script : &<Self as ScriptHost>::ScriptContext) where 
        A: for<'lua> FromLuaMulti<'lua>,
        R: for<'lua> ToLuaMulti<'lua>,
        F: 'static + Send + for <'lua> Fn(Context<'lua>, A) -> Result<R,LuaError>
    
    {
        script.lock().unwrap().context(|lua_ctx| {
            let f = lua_ctx.create_function(callback).unwrap();
            lua_ctx.globals().set(callback_fn_name,f);

        });
    }
}

// pub trait RLuaAPIProvider : rlua::UserData {
//     fn get_world(& self) -> &World; 
// }

// pub struct DefaultRluaAPI<'a> {
//     world : &'a World
// }

// impl RLuaAPIProvider for DefaultRluaAPI<'_> {
//     fn get_world(&self) -> &World {
//         self.world
//     }
// }


// pub trait Registrar {
//     fn add_methods<'lua,'a, T: LuaUserDataMethods<'lua,DefaultRluaAPI<'a>>>(methods : *mut T);
// }


// type APIFunction<'lua> = fn(&);

// pub static RLUA_API: Lazy<Mutex<dyn Registrar>> = Lazy::new (|| {

// });

// impl rlua::UserData for DefaultRluaAPI<'_> {
//     fn add_methods<'lua, T: LuaUserDataMethods<'lua, Self>>(methods: &mut T) {
//         methods.add_method("test", |lua_ctx, world, ()| {
//             info!("test called yippe!");

//             Ok(5)
//         })
//     }
// }


