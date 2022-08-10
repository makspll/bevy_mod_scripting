use crate::{
    assets::{LuaFile, LuaLoader},
    docs::LuaDocFragment,
};
use bevy::prelude::*;
use bevy_mod_scripting_core::{
    prelude::*, script_add_synchronizer, script_hot_reload_handler, script_remove_synchronizer,
    CachedScriptEventState,
};
use parking_lot::RwLock;
use std::fmt;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use tealr::mlu::mlua::{prelude::*, Function};

pub mod assets;
pub mod docs;
pub mod util;
pub use tealr;
pub mod prelude {
    pub use crate::{
        assets::{LuaFile, LuaLoader},
        docs::{LuaDocFragment, TypeWalkerBuilder},
        tealr::{
            self,
            mlu::{
                mlua::{self, prelude::*, Value},
                TealData,
            },
        },
        LuaEvent, LuaScriptHost,
    };
}

pub trait LuaArg: for<'lua> ToLuaMulti<'lua> + Clone + Sync + Send + 'static {}

impl<T: for<'lua> ToLuaMulti<'lua> + Clone + Sync + Send + 'static> LuaArg for T {}

#[derive(Clone)]
/// A Lua Hook. The result of creating this event will be
/// a call to the lua script with the hook_name and the given arguments
pub struct LuaEvent<A: LuaArg> {
    pub hook_name: String,
    pub args: A,
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

/// Mlua script host, enables Lua scripting provided by the mlua library.
/// Always provides two global variables to each script by default:
///     - `world` - a reference to the `bevy::ecs::World` the script lives in via [`LuaWorld`]
///     - `entity` - an `Entity::to_bits` representation of the entity the script is attached to
///     - `script` - an `LuaScriptData` object containing the unique id of this script
///
/// # Examples
///
/// You can use these variables in your APIProviders like so:
/// ```
///    use ::std::sync::Mutex;
///    use bevy::prelude::*;
///    use bevy_mod_scripting::{*, langs::mlu::{mlua,mlua::prelude::*}};
///    
///    #[derive(Default)]
///    pub struct LuaAPIProvider;
///
///    /// the custom Lua api, world is provided via a global pointer,
///    /// and callbacks are defined only once at script creation
///    impl APIProvider for LuaAPIProvider {
///        type APITarget = Mutex<Lua>;
///        type DocTarget = LuaDocFragment;
///        type ScriptContext = Mutex<Lua>;
///
///        fn attach_api(&mut self, ctx: &mut Self::APITarget) -> Result<(),ScriptError> {
///            // callbacks can receive any `ToLuaMulti` arguments, here '()' and
///            // return any `FromLuaMulti` arguments, here a `usize`
///            // check the Rlua documentation for more details
///            let ctx = ctx.lock().unwrap();
///
///            ctx.globals().set("your_callback", ctx.create_function(|ctx, ()| {
///                 let globals = ctx.globals();
///                 // retrieve the world pointer
///                 let world_data: usize = globals.get("world").unwrap();
///                 let world: &mut World = unsafe { &mut *(world_data as *mut World) };
///                 // retrieve script entity
///                 let entity_id : u64 = globals.get("entity").unwrap();
///                 let entity : Entity = Entity::from_bits(entity_id);
///
///                 Ok(())
///            })?)?;
///
///                    
///            Ok(())
///        }
///    }
/// ```
pub struct LuaScriptHost<A: LuaArg> {
    _ph: PhantomData<A>,
}

impl<A: LuaArg> Default for LuaScriptHost<A> {
    fn default() -> Self {
        Self {
            _ph: Default::default(),
        }
    }
}

impl<A: LuaArg> ScriptHost for LuaScriptHost<A> {
    type ScriptContext = Mutex<Lua>;
    type APITarget = Mutex<Lua>;
    type ScriptEvent = LuaEvent<A>;
    type ScriptAsset = LuaFile;
    type DocTarget = LuaDocFragment;

    fn register_with_app(app: &mut App, stage: impl StageLabel) {
        app.add_priority_event::<Self::ScriptEvent>()
            .add_asset::<LuaFile>()
            .init_asset_loader::<LuaLoader>()
            .init_resource::<CachedScriptEventState<Self>>()
            .init_resource::<ScriptContexts<Self::ScriptContext>>()
            .init_resource::<APIProviders<Self>>()
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

    fn load_script(
        &mut self,
        script: &[u8],
        script_data: &ScriptData,
        providers: &mut APIProviders<Self>,
    ) -> Result<Self::ScriptContext, ScriptError> {
        #[cfg(feature = "unsafe_lua_modules")]
        let lua = unsafe { Lua::unsafe_new() };
        #[cfg(not(feature = "unsafe_lua_modules"))]
        let lua = Lua::new();

        lua.load(script)
            .set_name(script_data.name)
            .and_then(|c| c.exec())
            .map_err(|_e| ScriptError::FailedToLoad {
                script: script_data.name.to_owned(),
            })?;

        let mut lua = Mutex::new(lua);

        providers.attach_all(&mut lua)?;
        providers.setup_all(script_data, &mut lua)?;
        Ok(lua)
    }

    fn handle_events<'a>(
        &self,
        world: &mut World,
        events: &[Self::ScriptEvent],
        ctxs: impl Iterator<Item = (ScriptData<'a>, &'a mut Self::ScriptContext)>,
    ) {
        world.resource_scope(
            |world_orig, mut cached_state: Mut<CachedScriptEventState<Self>>| {
                let world_arc = Arc::new(RwLock::new(std::mem::take(world_orig)));

                ctxs.for_each(|(fd, ctx)| {
                    let success = ctx
                        .get_mut()
                        .map_err(|e| ScriptError::Other(e.to_string()))
                        .and_then(|ctx| {
                            let globals = ctx.globals();
                            // globals.set("world", LuaWorld::new(Arc::downgrade(&world_arc)))?;
                            // globals.set("entity", LuaEntity::new(fd.entity))?;
                            // globals.set::<_, LuaScriptData>("script", (&fd).into())?;

                            // event order is preserved, but scripts can't rely on any temporal
                            // guarantees when it comes to other scripts callbacks,
                            // at least for now.
                            // we stop on the first error encountered
                            for event in events {
                                // check if this script should handle this event
                                if !event.recipients().is_recipient(&fd) {
                                    continue;
                                }

                                let f: Function = match globals.get(event.hook_name.clone()) {
                                    Ok(f) => f,
                                    Err(_) => continue, // not subscribed to this event
                                };

                                f.call::<_, ()>(event.args.clone()).map_err(|e| {
                                    ScriptError::RuntimeError {
                                        script: fd.name.to_owned(),
                                        msg: e.to_string(),
                                    }
                                })?
                            }

                            // we must clear the world in order to free the Arc pointer
                            Ok(())
                        });

                    success
                        .map_err(|e| {
                            let mut guard = world_arc.write();
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
