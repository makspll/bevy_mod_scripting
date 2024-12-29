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
    reflection_extensions::PartialReflectExt,
    script::ScriptId,
    AddContextInitializer, AddContextPreHandlingInitializer, ScriptingPlugin,
};
use bindings::{
    // providers::bevy_ecs::LuaEntity,
    // proxy::LuaProxied,
    reference::{LuaReflectReference, LuaStaticReflectReference},
    world::{GetWorld, LuaWorld},
};
pub use mlua;
use mlua::{Function, IntoLuaMulti, Lua};
pub mod bindings;
// use type_data::{
//     pre_register_common_containers, register_lua_values, ReflectLuaProxied, ReflectLuaValue,
// };

pub mod prelude {
    pub use crate::mlua::{self, prelude::*, Value};
}

pub trait LuaEventArg: Args + IntoLuaMulti {}
impl<T: Args + IntoLuaMulti> LuaEventArg for T {}

pub struct LuaScriptingPlugin<A: Args + IntoLuaMulti> {
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
        // register_lua_values(app);
        app.add_context_pre_handling_initializer::<()>(|script_id, entity, context: &mut Lua| {
            let world = context.get_world();
            context
                .globals()
                .set(
                    "entity",
                    LuaReflectReference(<Entity>::allocate(Box::new(entity), world)),
                )
                .map_err(ScriptError::from_mlua_error)?;
            context
                .globals()
                .set("script_id", script_id.clone())
                .map_err(ScriptError::from_mlua_error)?;
            Ok(())
        });
    }

    fn cleanup(&self, app: &mut App) {
        // find all registered types, and insert dummy for calls
        // let mut type_registry = app.world_mut().get_resource_or_init::<AppTypeRegistry>();
        // let mut type_registry = type_registry.read();

        // type_registry.iter().map(|t| {
        //     t.
        // })
        app.add_context_initializer::<()>(|_script_id, context: &mut Lua| {
            let world = context.get_world();
            let type_registry = world.type_registry();
            let type_registry = type_registry.read();

            for registration in type_registry.iter() {
                // only do this for non generic types
                // we don't want to see `Vec<Entity>:function()` in lua
                if !registration.type_info().generics().is_empty() {
                    continue;
                }

                if let Some(global_name) = registration.type_info().type_path_table().ident() {
                    let ref_ = LuaStaticReflectReference(registration.type_id());
                    context
                        .globals()
                        .set(global_name, ref_)
                        .map_err(ScriptError::from_mlua_error)?;
                }
            }
            Ok(())
        });

        // let mut type_registry = app.world_mut().get_resource_mut().unwrap();

        // we register up to two levels of nesting, if more are needed, the user will have to do this manually
        // pre_register_common_containers(&mut type_registry);
        // pre_register_common_containers(&mut type_registry);
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

        context
            .load(content)
            .exec()
            .map_err(ScriptError::from_mlua_error)?;
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
pub fn lua_handler<A: Args + IntoLuaMulti>(
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

        handler
            .call::<()>(args)
            .map_err(ScriptError::from_mlua_error)?;
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
        context
            .globals()
            .set("world", LuaReflectReference(ReflectReference::new_world()))
            .map_err(ScriptError::from_mlua_error)?;
        context.set_app_data(guard.clone());
        f(context)
    })
}
