pub mod assets;
pub mod docs;
pub mod type_data;
pub mod util;
use bevy::{
    app::{App, Plugin, Startup},
    ecs::{entity::Entity, world::World},
    prelude::{AppTypeRegistry, Mut},
    reflect::{impl_reflect, FromType, GetTypeRegistration, PartialReflect, Reflect, TypePath},
};
use bevy_mod_scripting_core::{
    bindings::{ReflectAllocator, ReflectReference, WorldCallbackAccess},
    context::{ContextBuilder, ContextInitializer, ContextPreHandlingInitializer},
    error::ScriptError,
    event::CallbackLabel,
    handler::Args,
    script::ScriptId,
    AddContextPreHandlingInitializer, ScriptingPlugin,
};
use bindings::{
    providers::bevy_ecs::LuaEntity,
    proxy::LuaProxied,
    world::{GetWorld, LuaWorld},
};
pub use tealr;
pub mod bindings;
use tealr::mlu::mlua::{FromLua, Function, IntoLua, IntoLuaMulti, Lua, Value};
use type_data::{
    pre_register_common_containers, register_lua_values, ReflectLuaProxied, ReflectLuaValue,
};

pub mod prelude {
    pub use crate::tealr::{
        self,
        mlu::{
            mlua::{self, prelude::*, Value},
            TealData,
        },
    };
}

pub trait LuaEventArg: Args + for<'l> IntoLuaMulti<'l> {}
impl<T: Args + for<'l> IntoLuaMulti<'l>> LuaEventArg for T {}

pub struct LuaScriptingPlugin<A: Args + for<'l> IntoLuaMulti<'l>> {
    pub scripting_plugin: ScriptingPlugin<A, Lua, ()>,
}

impl<A: LuaEventArg> Default for LuaScriptingPlugin<A> {
    fn default() -> Self {
        LuaScriptingPlugin {
            scripting_plugin: ScriptingPlugin {
                context_assigner: None,
                runtime_builder: Default::default,
                runtime_settings: None,
                callback_handler: Some(lua_handler::<A>),
                context_builder: Some(ContextBuilder::<Lua, ()> {
                    load: lua_context_load,
                    reload: lua_context_reload,
                }),
            },
        }
    }
}

impl<A: LuaEventArg> Plugin for LuaScriptingPlugin<A> {
    fn build(&self, app: &mut bevy::prelude::App) {
        self.scripting_plugin.build(app);
        register_lua_values(app);
        app.add_context_pre_handling_initializer::<()>(|script_id, entity, context: &mut Lua| {
            let world = context.get_world();
            let lua_entity = world.with_resource::<ReflectAllocator, _, _>(|_, mut allocator| {
                let reflect_reference = ReflectReference::new_allocated(entity, &mut allocator);
                <Entity as LuaProxied>::Proxy::from(reflect_reference)
            });

            context.globals().set("script_id", script_id.to_owned())?;
            context.globals().set("entity", lua_entity)?;
            Ok(())
        });
    }

    fn cleanup(&self, app: &mut App) {
        let mut type_registry = app.world_mut().get_resource_mut().unwrap();
        pre_register_common_containers(&mut type_registry);
    }
}

pub fn lua_context_load(
    script_id: &ScriptId,
    content: &[u8],
    initializers: &[ContextInitializer<Lua>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<Lua>],
    world: &mut World,
    _: &mut (),
) -> Result<Lua, ScriptError> {
    #[cfg(feature = "unsafe_lua_modules")]
    let context = unsafe { Lua::unsafe_new() };
    #[cfg(not(feature = "unsafe_lua_modules"))]
    let mut context = Lua::new();

    with_world(world, &mut context, |context| {
        initializers
            .iter()
            .try_for_each(|init| init(script_id, context))?;

        pre_handling_initializers
            .iter()
            .try_for_each(|init| init(script_id, Entity::from_raw(0), context))?;

        context.load(content).exec()?;
        Ok(())
    })?;

    Ok(context)
}

pub fn lua_context_reload(
    script: &ScriptId,
    content: &[u8],
    old_ctxt: &mut Lua,
    initializers: &[ContextInitializer<Lua>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<Lua>],
    world: &mut World,
    _: &mut (),
) -> Result<(), ScriptError> {
    *old_ctxt = lua_context_load(
        script,
        content,
        initializers,
        pre_handling_initializers,
        world,
        &mut (),
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn lua_handler<A: Args + for<'l> IntoLuaMulti<'l>>(
    args: A,
    entity: bevy::ecs::entity::Entity,
    script_id: &ScriptId,
    callback_label: &CallbackLabel,
    context: &mut Lua,
    pre_handling_initializers: &[ContextPreHandlingInitializer<Lua>],
    _: &mut (),
    world: &mut bevy::ecs::world::World,
) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
    with_world(world, context, |context| {
        pre_handling_initializers
            .iter()
            .try_for_each(|init| init(script_id, entity, context))?;

        let handler: Function = match context.globals().raw_get(callback_label.as_ref()) {
            Ok(handler) => handler,
            // not subscribed to this event type
            Err(_) => return Ok(()),
        };

        handler.call::<_, ()>(args)?;
        Ok(())
    })
}

/// Safely scopes world access for a lua context to the given closure's scope
pub fn with_world<F: FnOnce(&mut Lua) -> Result<(), ScriptError>>(
    world: &mut World,
    context: &mut Lua,
    f: F,
) -> Result<(), ScriptError> {
    WorldCallbackAccess::with_callback_access(world, |guard| {
        context.globals().set("world", LuaWorld(guard.clone()))?;
        f(context)
    })
}
