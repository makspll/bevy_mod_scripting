pub mod assets;

use crate::{
    script_add_synchronizer, script_hot_reload_handler, script_remove_synchronizer, APIProvider,
    CachedScriptEventState, FlatScriptData, Recipients, Script, ScriptCollection, ScriptContexts,
    ScriptError, ScriptErrorEvent, ScriptEvent, ScriptHost, APIProviders,
};
use anyhow::Result;

use bevy::prelude::*;
use bevy_event_priority::AddPriorityEvent;
use tealr::mlu::mlua::{prelude::*,Function};

use std::marker::PhantomData;
use std::sync::Mutex;

pub use assets::*;

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
pub struct RLuaScriptHost<A: LuaArg> {
    _ph: PhantomData<A>,
}

unsafe impl<A: LuaArg> Send for RLuaScriptHost<A> {}
unsafe impl<A: LuaArg> Sync for RLuaScriptHost<A> {}

impl<A: LuaArg> ScriptHost for RLuaScriptHost<A> {
    type ScriptContext = Mutex<Lua>;
    type ScriptEvent = LuaEvent<A>;
    type ScriptAsset = LuaFile;

    fn register_with_app(app: &mut App, stage: impl StageLabel) {
        app.add_priority_event::<Self::ScriptEvent>()
            .add_asset::<LuaFile>()
            .init_asset_loader::<LuaLoader>()
            .init_resource::<CachedScriptEventState<Self>>()
            .init_resource::<ScriptContexts<Self::ScriptContext>>()
            .init_resource::<APIProviders<Self::ScriptContext>>()
            .register_type::<ScriptCollection<Self::ScriptAsset>>()
            .register_type::<Script<Self::ScriptAsset>>()
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

    fn load_script(script: &[u8], script_name: &str, providers: &APIProviders<Self::ScriptContext>) -> Result<Self::ScriptContext, ScriptError> {
        let lua = Lua::new();
        
        lua.load(script)
            .set_name(script_name)
            .map(|c| c.exec())
            .map_err(|_e| ScriptError::FailedToLoad {
                script: script_name.to_owned(),
            })??;

        let mut lua = Mutex::new(lua);

        providers.attach_all(&mut lua)?;

        Ok(lua)
    }

    fn handle_events<'a>(
        world: &mut World,
        events: &[Self::ScriptEvent],
        ctxs: impl Iterator<Item = (FlatScriptData<'a>, &'a mut Self::ScriptContext)>,
    ) {
        let world_ptr = world as *mut World as usize;

        world.resource_scope(
            |world, mut cached_state: Mut<CachedScriptEventState<Self>>| {
                let (_, mut error_wrt) = cached_state.event_state.get_mut(world);

                ctxs.for_each(|(fd, ctx)| {
                    let success = ctx
                        .get_mut()
                        .map_err(|e| ScriptError::Other(e.to_string()) )
                        .and_then(|ctx| {
                            let globals = ctx.globals();
                            globals.set("world", world_ptr)?;
                            globals.set("entity", fd.entity.to_bits())?;
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
                                    f = f.bind(a.to_lua(ctx)).map_err(|e| {
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

                            Ok(())
                        });

                    success
                        .map_err(|e| {
                            error!("{}", e);
                            error_wrt.send(ScriptErrorEvent { err: e })
                        })
                        .ok();
                });
            },
        );
    }
}

