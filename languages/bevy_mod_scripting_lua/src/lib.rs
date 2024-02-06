use crate::{
    assets::{LuaFile, LuaLoader},
    docs::LuaDocFragment,
};
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use bevy_mod_scripting_core::{prelude::*, systems::*, world::WorldPointerGuard};

use std::fmt;
use std::marker::PhantomData;
use std::sync::Mutex;
use tealr::mlu::mlua::{prelude::*, Function};

pub mod assets;
pub mod docs;
pub mod util;
use bevy_mod_scripting_core::event::write_error_event_with_world;
use bevy_mod_scripting_core::world::WorldPointer;
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

pub trait LuaArg: for<'lua> IntoLuaMulti<'lua> + Clone + Sync + Send + 'static {}

impl<T: for<'lua> IntoLuaMulti<'lua> + Clone + Sync + Send + 'static> LuaArg for T {}

#[derive(Clone, Event)]
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

#[derive(Resource)]
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

    fn register_with_app_in_set(app: &mut App, schedule: impl ScheduleLabel, set: impl SystemSet) {
        app.add_priority_event::<Self::ScriptEvent>()
            .add_asset::<LuaFile>()
            .init_asset_loader::<LuaLoader>()
            .init_resource::<CachedScriptState<Self>>()
            .init_resource::<CachedScriptLoadState<Self>>()
            .init_resource::<ScriptContexts<Self::ScriptContext>>()
            .init_resource::<APIProviders<Self>>()
            .register_type::<ScriptCollection<Self::ScriptAsset>>()
            .register_type::<Script<Self::ScriptAsset>>()
            .register_type::<Handle<LuaFile>>()
            // handle script insertions removal first
            // then update their contexts later on script asset changes
            .add_systems(
                schedule,
                (
                    script_add_synchronizer::<Self>,
                    script_remove_synchronizer::<Self>,
                    script_hot_reload_handler::<Self>,
                )
                    .chain()
                    .in_set(set),
            );
    }

    fn load_script(
        &mut self,
        world: WorldPointer,
        script: &[u8],
        script_data: &ScriptData,
        providers: &mut APIProviders<Self>,
    ) -> Result<Self::ScriptContext, ScriptError> {
        #[cfg(feature = "unsafe_lua_modules")]
        let lua = unsafe { Lua::unsafe_new() };
        #[cfg(not(feature = "unsafe_lua_modules"))]
        let lua = Lua::new();

        // init lua api before loading script
        let mut lua = Mutex::new(lua);

        providers
            .setup_runtime_all(world.clone(), script_data, &mut lua)
            .expect("Could not setup script runtime");

        providers.attach_all(&mut lua)?;

        // We do this twice to get around the issue of attach_all overriding values here for the sake of
        // documenting, TODO: this is messy, shouldn't be a problem but it's messy
        providers
            .setup_runtime_all(world.clone(), script_data, &mut lua)
            .expect("Could not setup script runtime");

        lua.get_mut()
            .map_err(|e| {
                write_error_event_with_world::<Self>(
                    world.clone(),
                    script_data.name.to_owned(),
                    e.to_string(),
                );
                ScriptError::FailedToLoad {
                    script: script_data.name.to_owned(),
                    msg: e.to_string(),
                }
            })?
            .load(script)
            .set_name(script_data.name)
            .exec()
            .map_err(|e| {
                write_error_event_with_world::<Self>(
                    world.clone(),
                    script_data.name.to_owned(),
                    e.to_string(),
                );
                ScriptError::FailedToLoad {
                    script: script_data.name.to_owned(),
                    msg: e.to_string(),
                }
            })?;

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
        &mut self,
        world: &mut World,
        events: &[Self::ScriptEvent],
        ctxs: impl Iterator<Item = (ScriptData<'a>, &'a mut Self::ScriptContext)>,
        providers: &mut APIProviders<Self>,
    ) {
        // safety:
        // - we have &mut World access
        // - we do not use the original reference again anywhere in this function
        let world = unsafe { WorldPointerGuard::new(world) };

        ctxs.for_each(|(script_data, ctx)| {
            providers
                .setup_runtime_all(world.clone(), &script_data, ctx)
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
                    write_error_event_with_world::<Self>(
                        world.clone(),
                        script_data.name.to_owned(),
                        error.to_string(),
                    );
                }
            }
        });
    }
}
