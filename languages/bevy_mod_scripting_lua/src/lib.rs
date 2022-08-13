use crate::{
    assets::{LuaFile, LuaLoader},
    docs::LuaDocFragment,
};
use bevy::prelude::*;
use bevy_mod_scripting_core::{prelude::*, systems::*, world::WorldPointer};
use parking_lot::RwLock;
use std::fmt;
use std::marker::PhantomData;
use std::sync::Mutex;
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

/// Lua script host, enables Lua scripting.
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
            .init_resource::<CachedScriptState<Self>>()
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
        Ok(lua)
    }

    fn setup_script(
        &mut self,
        script_data: &ScriptData,
        ctx: &mut Self::ScriptContext,
        providers: &mut APIProviders<Self>,
    ) -> Result<(), ScriptError> {
        // safety: this is fine, world will only ever be accessed
        providers.setup_all(script_data, ctx)
    }

    fn handle_events<'a>(
        &self,
        world: &mut World,
        events: &[Self::ScriptEvent],
        ctxs: impl Iterator<Item = (ScriptData<'a>, &'a mut Self::ScriptContext)>,
        providers: &mut APIProviders<Self>,
    ) {
        // safety:
        // - we have &mut World access
        // - we do not use world_ptr after using the world reference which it's derived from
        let world_ptr = unsafe { WorldPointer::new(world) };

        ctxs.for_each(|(script_data, ctx)| {
            providers
                .setup_runtime_all(world_ptr.clone(), &script_data, ctx)
                .expect("Could not setup script runtime");

            let ctx = ctx.get_mut().expect("Poison error in context");

            // event order is preserved, but scripts can't rely on any temporal
            // guarantees when it comes to other scripts callbacks,
            // at least for now.
            let globals = ctx.globals();

            for event in events {
                // check if this script should handle this event
                if !event.recipients().is_recipient(&script_data) {
                    continue;
                }

                let f: Function = match globals.raw_get(event.hook_name.clone()) {
                    Ok(f) => f,
                    Err(_) => continue, // not subscribed to this event
                };

                if let Err(error) = f.call::<_, ()>(event.args.clone()) {
                    let error = ScriptError::RuntimeError {
                        script: script_data.name.to_owned(),
                        msg: error.to_string(),
                    };

                    let mut world = world_ptr.write();
                    let mut state: CachedScriptState<Self> = world.remove_resource().unwrap();

                    let (_, mut error_wrt, _) = state.event_state.get_mut(&mut world);

                    error!("{}", error);
                    error_wrt.send(ScriptErrorEvent { error });
                    world.insert_resource(state);
                }
            }
        });
    }
}
