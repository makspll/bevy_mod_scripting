use bevy::{
    app::{App, Plugin},
    ecs::{entity::Entity, world::World},
};
use bevy_mod_scripting_core::{
    asset::{AssetPathToLanguageMapper, Language},
    bindings::{script_value::ScriptValue, WorldCallbackAccess},
    context::{ContextBuilder, ContextInitializer, ContextPreHandlingInitializer},
    error::ScriptError,
    event::CallbackLabel,
    reflection_extensions::PartialReflectExt,
    script::ScriptId,
    AddContextInitializer, AddContextPreHandlingInitializer, IntoScriptPluginParams,
    ScriptingPlugin,
};
use bindings::{
    reference::{LuaReflectReference, LuaStaticReflectReference},
    script_value::LuaScriptValue,
    world::GetWorld,
};
pub use mlua;
use mlua::{Function, IntoLua, Lua, MultiValue};
pub mod bindings;

impl IntoScriptPluginParams for LuaScriptingPlugin {
    type C = Lua;
    type R = ();
    const LANGUAGE: Language = Language::Lua;
}

pub struct LuaScriptingPlugin {
    pub scripting_plugin: ScriptingPlugin<Self>,
}

impl Default for LuaScriptingPlugin {
    fn default() -> Self {
        LuaScriptingPlugin {
            scripting_plugin: ScriptingPlugin {
                context_assigner: None,
                runtime_builder: Default::default,
                runtime_settings: None,
                callback_handler: Some(lua_handler),
                context_builder: Some(ContextBuilder::<LuaScriptingPlugin> {
                    load: lua_context_load,
                    reload: lua_context_reload,
                }),
                language_mapper: Some(AssetPathToLanguageMapper {
                    map: lua_language_mapper,
                }),
                context_initializers: Default::default(),
                context_pre_handling_initializers: Default::default(),
            },
        }
    }
}

fn lua_language_mapper(path: &std::path::Path) -> Language {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("lua") => Language::Lua,
        _ => Language::Unknown,
    }
}

impl Plugin for LuaScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        self.scripting_plugin.build(app);
        // register_lua_values(app);
        app.add_context_pre_handling_initializer::<LuaScriptingPlugin>(
            |script_id, entity, context: &mut Lua| {
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
                    .set("script_id", script_id)
                    .map_err(ScriptError::from_mlua_error)?;
                Ok(())
            },
        );
    }

    fn cleanup(&self, app: &mut App) {
        // find all registered types, and insert dummy for calls

        app.add_context_initializer::<LuaScriptingPlugin>(|_script_id, context: &mut Lua| {
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
    }
}

pub fn lua_context_load(
    script_id: &ScriptId,
    content: &[u8],
    initializers: &[ContextInitializer<LuaScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<LuaScriptingPlugin>],
    world: &mut World,
    _: &mut (),
) -> Result<Lua, ScriptError> {
    #[cfg(feature = "unsafe_lua_modules")]
    let mut context = unsafe { Lua::unsafe_new() };
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
    initializers: &[ContextInitializer<LuaScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<LuaScriptingPlugin>],
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
pub fn lua_handler(
    args: Vec<ScriptValue>,
    entity: bevy::ecs::entity::Entity,
    script_id: &ScriptId,
    callback_label: &CallbackLabel,
    context: &mut Lua,
    pre_handling_initializers: &[ContextPreHandlingInitializer<LuaScriptingPlugin>],
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

        let input = MultiValue::from_vec(
            args.into_iter()
                .map(|arg| LuaScriptValue::from(arg).into_lua(context))
                .collect::<Result<_, _>>()?,
        );

        handler.call::<()>(input)?;
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
            .set(
                "world",
                LuaStaticReflectReference(std::any::TypeId::of::<World>()),
            )
            .map_err(ScriptError::from_mlua_error)?;
        context.set_app_data(guard.clone());
        f(context)
    })
}
