pub mod api;
pub mod assets;

use crate::{
    script_add_synchronizer, script_hot_reload_handler, script_remove_synchronizer, APIProvider,bevy_types::LuaBevyAPI,
    CachedScriptEventState, FlatScriptData, Recipients, Script, ScriptCollection, ScriptContexts,
    ScriptError, ScriptErrorEvent, ScriptEvent, ScriptHost,
};
use anyhow::Result;

use bevy::prelude::*;
use bevy_event_priority::AddPriorityEvent;
use parking_lot::RwLock;
use rlua::prelude::*;
use rlua::{Context, Function, Lua, ToLua, ToLuaMulti};

use std::fmt;
use std::marker::PhantomData;
use std::sync::{Mutex,Arc};

pub use {api::*, assets::*};

pub trait LuaArg: for<'lua> ToLua<'lua> + Clone + Sync + Send + 'static {}

impl<T: for<'lua> ToLua<'lua> + Clone + Sync + Send + 'static> LuaArg for T {}

#[derive(Clone)]
/// A Lua Hook. The result of creating this event will be
/// a call to the lua script with the hook_name and the given arguments
pub struct LuaEvent<A: LuaArg> {
    pub hook_name: String,
    pub args: Vec<A>,
    pub recipients: Recipients,
}

impl<A: LuaArg> fmt::Debug for LuaEvent<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LuaEvent")
            .field("hook_name", &self.hook_name)
            .field("recipients", &self.recipients)
            .finish()
    }
}

impl<A: LuaArg> ScriptEvent for LuaEvent<A> {
    fn recipients(&self) -> &crate::Recipients {
        &self.recipients
    }
}

/// Rlua script host, enables Lua scripting provided by the Rlua library.
/// Always provides two global variables to each script by default:
///     - `world` - a raw pointer to the `bevy::World` the script lives in
///     - `entity` - an `Entity::to_bits` representation of the entity the script is attached to
///
/// # Examples
///
/// You can use these variables in your APIProviders like so:
/// ```
///    use std::sync::Mutex;
///    use bevy::prelude::*;
///    use rlua::prelude::*;
///    use bevy_mod_scripting::{RLuaScriptHost, APIProvider};
///    
///    #[derive(Default)]
///    pub struct LuaAPIProvider {}
///
///    #[derive(Clone)]
///    pub struct MyLuaArg;
///
///    impl<'lua> ToLua<'lua> for MyLuaArg {
///        fn to_lua(self, _lua: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
///            Ok(rlua::Value::Nil)
///        }
///    }

///    /// the custom Lua api, world is provided via a global pointer,
///    /// and callbacks are defined only once at script creation
///    impl APIProvider for LuaAPIProvider {
///        type Ctx = Mutex<Lua>;
///        fn attach_api(ctx: &mut Self::Ctx) {
///            // callbacks can receive any `ToLuaMulti` arguments, here '()' and
///            // return any `FromLuaMulti` arguments, here a `usize`
///            // check the Rlua documentation for more details
///            RLuaScriptHost::<MyLuaArg,Self>::register_api_callback(
///                "your_callback",
///                |ctx, ()| {
///                    let globals = ctx.globals();
///
///                    // retrieve the world pointer
///                    let world_data: usize = globals.get("world").unwrap();
///                    let world: &mut World = unsafe { &mut *(world_data as *mut World) };
///                    
///                    // retrieve script entity
///                    let entity_id : u64 = globals.get("entity").unwrap();
///                    let entity : Entity = Entity::from_bits(entity_id);
///
///                    
///                    Ok(())
///                },
///                ctx,
///            ).unwrap();
///        }
///    }
/// ```
#[derive(Default)]
pub struct RLuaScriptHost<A: LuaArg, API: APIProvider> {
    _ph: PhantomData<API>,
    _ph2: PhantomData<A>,
}

impl<A: LuaArg, API: APIProvider<Ctx = Mutex<Lua>>> ScriptHost for RLuaScriptHost<A, API> {
    type ScriptContext = Mutex<Lua>;
    type ScriptEvent = LuaEvent<A>;
    type ScriptAsset = LuaFile;
    type BevyAPI = LuaBevyAPI;

    fn register_with_app(app: &mut App, stage: impl StageLabel) {
        app.add_priority_event::<Self::ScriptEvent>()
            .add_asset::<LuaFile>()
            .init_asset_loader::<LuaLoader>()
            .init_resource::<CachedScriptEventState<Self>>()
            .init_resource::<ScriptContexts<Self::ScriptContext>>()
            .register_type::<ScriptCollection<Self::ScriptAsset>>()
            .register_type::<Script<Self::ScriptAsset>>()
            .register_type::<Handle<LuaFile>>()
            .add_system_set_to_stage(
                stage,
                SystemSet::new()
                    // handle script insertions removal first
                    // then update their contexts later on script asset changes
                    .with_system(
                        script_add_synchronizer::<Self>.before(script_remove_synchronizer::<Self>),
                    )
                    .with_system(
                        script_remove_synchronizer::<Self>
                            .before(script_hot_reload_handler::<Self>),
                    )
                    .with_system(script_hot_reload_handler::<Self>),
            );
    }

    fn load_script(script: &[u8], script_name: &str) -> Result<Self::ScriptContext, ScriptError> {
        let lua = Lua::new();
        lua.context::<_, Result<(), ScriptError>>(|lua_ctx| {
            lua_ctx
                .load(script)
                .set_name(script_name)
                .map(|c| c.exec())
                .map_err(|_e| ScriptError::FailedToLoad {
                    script: script_name.to_owned(),
                })??;

            Ok(())
        })?;

        let mut lua = Mutex::new(lua);

        Self::BevyAPI::attach_api(&mut lua);
        API::attach_api(&mut lua);

        Ok(lua)
    }

    fn handle_events<'a>(
        world: &mut World,
        events: &[Self::ScriptEvent],
        ctxs: impl Iterator<Item = (FlatScriptData<'a>, &'a mut Self::ScriptContext)>,
    ) {

        world.resource_scope(
            |world_orig, mut cached_state: Mut<CachedScriptEventState<Self>>| {
                
                let world_arc = Arc::new(RwLock::new(std::mem::take(world_orig)));

                ctxs.for_each(|(fd, ctx)| {
                    let success = ctx
                        .get_mut()
                        .expect("Could not get lock on script context")
                        .context::<_, Result<(), ScriptError>>(|lua_ctx| {
                            let globals = lua_ctx.globals();
                            globals.set("world", LuaWorld(Arc::downgrade(&world_arc)))?;
                            globals.set("entity", LuaEntity::Owned(fd.entity))?;
                            globals.set("script", fd.sid)?;

                            // event order is preserved, but scripts can't rely on any temporal
                            // guarantees when it comes to other scripts callbacks,
                            // at least for now.
                            // we stop on the first error encountered
                            for event in events {
                                // check if this script should handle this event
                                if !event.recipients().is_recipient(&fd) {
                                    continue;
                                }

                                let mut f: Function = match globals.get(event.hook_name.clone()) {
                                    Ok(f) => f,
                                    Err(_) => continue, // not subscribed to this event
                                };

                                let ags = event.args.clone();
                                // bind arguments and catch any errors
                                for a in ags {
                                    f = f.bind(a.to_lua(lua_ctx)).map_err(|e| {
                                        ScriptError::InvalidCallback {
                                            script: fd.name.to_owned(),
                                            callback: event.hook_name.to_owned(),
                                            msg: e.to_string(),
                                        }
                                    })?
                                }

                                f.call::<(), ()>(())
                                    .map_err(|e| ScriptError::RuntimeError {
                                        script: fd.name.to_owned(),
                                        msg: e.to_string(),
                                    })?
                            }

                            // we must clear the world in order to free the Arc pointer
                            Ok(())
                        });
                    success
                        .map_err(|e| {
                            let mut guard = world_arc.write() ;
                            let (_, mut error_wrt) = cached_state.event_state.get_mut(&mut guard);

                            error!("{}", e);
                            error_wrt.send(ScriptErrorEvent { err: e })
                        })
                        .ok();
                });

                *world_orig = Arc::try_unwrap(world_arc).unwrap().into_inner();

            },
        );
    }
}

impl<A: LuaArg, API: APIProvider<Ctx = Mutex<Lua>>> RLuaScriptHost<A, API> {
    pub fn register_api_callback<F, Arg, R>(
        callback_fn_name: &str,
        callback: F,
        script: &<Self as ScriptHost>::ScriptContext,
    ) -> Result<(), ScriptError>
    where
        Arg: for<'lua> FromLuaMulti<'lua>,
        R: for<'lua> ToLuaMulti<'lua>,
        F: 'static + Send + for<'lua> Fn(Context<'lua>, Arg) -> Result<R, LuaError>,
    {
        script
            .lock()
            .expect("Could not get lock on script context")
            .context::<_, Result<(), ScriptError>>(|lua_ctx| {
                let f = lua_ctx.create_function(callback)?;
                lua_ctx.globals().set(callback_fn_name, f)?;

                Ok(())
            })
    }
}
