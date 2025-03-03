//! Lua integration for the bevy_mod_scripting system.
use bevy::{
    app::Plugin,
    asset::AssetPath,
    ecs::{entity::Entity, world::World},
};
use bevy_mod_scripting_core::{
    asset::{AssetPathToLanguageMapper, Language},
    bindings::{
        function::namespace::Namespace, globals::AppScriptGlobalsRegistry,
        script_value::ScriptValue, ThreadWorldContainer, WorldContainer,
    },
    context::{ContextBuilder, ContextInitializer, ContextPreHandlingInitializer},
    error::ScriptError,
    event::CallbackLabel,
    reflection_extensions::PartialReflectExt,
    runtime::RuntimeSettings,
    script::ScriptId,
    IntoScriptPluginParams, ScriptingPlugin,
};
use bindings::{
    reference::{LuaReflectReference, LuaStaticReflectReference},
    script_value::LuaScriptValue,
};
pub use mlua;
use mlua::{Function, IntoLua, Lua, MultiValue};

/// Bindings for lua.
pub mod bindings;

impl IntoScriptPluginParams for LuaScriptingPlugin {
    type C = Lua;
    type R = ();
    const LANGUAGE: Language = Language::Lua;

    fn build_runtime() -> Self::R {}
}

// necessary for automatic config goodies
impl AsMut<ScriptingPlugin<Self>> for LuaScriptingPlugin {
    fn as_mut(&mut self) -> &mut ScriptingPlugin<LuaScriptingPlugin> {
        &mut self.scripting_plugin
    }
}

/// The lua scripting plugin. Used to add lua scripting to a bevy app within the context of the BMS framework.
pub struct LuaScriptingPlugin {
    /// The internal scripting plugin
    pub scripting_plugin: ScriptingPlugin<Self>,
}

impl Default for LuaScriptingPlugin {
    fn default() -> Self {
        LuaScriptingPlugin {
            scripting_plugin: ScriptingPlugin {
                context_assigner: Default::default(),
                runtime_settings: RuntimeSettings::default(),
                callback_handler: lua_handler,
                context_builder: ContextBuilder::<LuaScriptingPlugin> {
                    load: lua_context_load,
                    reload: lua_context_reload,
                },
                language_mapper: AssetPathToLanguageMapper {
                    map: lua_language_mapper,
                },
                context_initializers: vec![
                    |_script_id, context| {
                        // set the world global
                        context
                            .globals()
                            .set(
                                "world",
                                LuaStaticReflectReference(std::any::TypeId::of::<World>()),
                            )
                            .map_err(ScriptError::from_mlua_error)?;

                        Ok(())
                    },
                    |_script_id, context: &mut Lua| {
                        // set static globals
                        let world = ThreadWorldContainer.try_get_world()?;
                        let globals_registry =
                            world.with_resource(|r: &AppScriptGlobalsRegistry| r.clone())?;
                        let globals_registry = globals_registry.read();

                        for (key, global) in globals_registry.iter() {
                            match &global.maker {
                                Some(maker) => {
                                    // non-static global
                                    let global = (maker)(world.clone())?;
                                    context
                                        .globals()
                                        .set(key.to_string(), LuaScriptValue::from(global))?
                                }
                                None => {
                                    let ref_ = LuaStaticReflectReference(global.type_id);
                                    context.globals().set(key.to_string(), ref_)?
                                }
                            }
                        }

                        // go through functions in the global namespace and add them to the lua context
                        let script_function_registry = world.script_function_registry();
                        let script_function_registry = script_function_registry.read();

                        for (key, function) in script_function_registry
                            .iter_all()
                            .filter(|(k, _)| k.namespace == Namespace::Global)
                        {
                            context
                                .globals()
                                .set(
                                    key.name.to_string(),
                                    LuaScriptValue::from(ScriptValue::Function(function.clone())),
                                )
                                .map_err(ScriptError::from_mlua_error)?;
                        }

                        Ok(())
                    },
                ],
                context_pre_handling_initializers: vec![|script_id, entity, context| {
                    let world = ThreadWorldContainer.try_get_world()?;
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
                }],
                supported_extensions: &["lua"],
            },
        }
    }
}
#[profiling::function]
fn lua_language_mapper(path: &AssetPath) -> Language {
    match path.path().extension().and_then(|ext| ext.to_str()) {
        Some("lua") => Language::Lua,
        _ => Language::Unknown,
    }
}

impl Plugin for LuaScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        self.scripting_plugin.build(app);
    }

    fn finish(&self, app: &mut bevy::app::App) {
        self.scripting_plugin.finish(app);
    }
}

fn load_lua_content_into_context(
    context: &mut Lua,
    script_id: &ScriptId,
    content: &[u8],
    initializers: &[ContextInitializer<LuaScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<LuaScriptingPlugin>],
) -> Result<(), ScriptError> {
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
}

#[profiling::function]
/// Load a lua context from a script
pub fn lua_context_load(
    script_id: &ScriptId,
    content: &[u8],
    initializers: &[ContextInitializer<LuaScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<LuaScriptingPlugin>],
    _: &mut (),
) -> Result<Lua, ScriptError> {
    #[cfg(feature = "unsafe_lua_modules")]
    let mut context = unsafe { Lua::unsafe_new() };
    #[cfg(not(feature = "unsafe_lua_modules"))]
    let mut context = Lua::new();

    load_lua_content_into_context(
        &mut context,
        script_id,
        content,
        initializers,
        pre_handling_initializers,
    )?;
    Ok(context)
}

#[profiling::function]
/// Reload a lua context from a script
pub fn lua_context_reload(
    script: &ScriptId,
    content: &[u8],
    old_ctxt: &mut Lua,
    initializers: &[ContextInitializer<LuaScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<LuaScriptingPlugin>],
    _: &mut (),
) -> Result<(), ScriptError> {
    load_lua_content_into_context(
        old_ctxt,
        script,
        content,
        initializers,
        pre_handling_initializers,
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[profiling::function]
/// The lua handler for events
pub fn lua_handler(
    args: Vec<ScriptValue>,
    entity: bevy::ecs::entity::Entity,
    script_id: &ScriptId,
    callback_label: &CallbackLabel,
    context: &mut Lua,
    pre_handling_initializers: &[ContextPreHandlingInitializer<LuaScriptingPlugin>],
    _: &mut (),
) -> Result<ScriptValue, bevy_mod_scripting_core::error::ScriptError> {
    pre_handling_initializers
        .iter()
        .try_for_each(|init| init(script_id, entity, context))?;

    let handler: Function = match context.globals().raw_get(callback_label.as_ref()) {
        Ok(handler) => handler,
        // not subscribed to this event type
        Err(_) => {
            bevy::log::trace!(
                "Script {} is not subscribed to callback {}",
                script_id,
                callback_label.as_ref()
            );
            return Ok(ScriptValue::Unit);
        }
    };

    let input = MultiValue::from_vec(
        args.into_iter()
            .map(|arg| LuaScriptValue::from(arg).into_lua(context))
            .collect::<Result<_, _>>()?,
    );

    let out = handler.call::<LuaScriptValue>(input)?;
    Ok(out.into())
}

#[cfg(test)]
mod test {
    use mlua::Value;

    use super::*;

    #[test]
    fn test_reload_doesnt_overwrite_old_context() {
        let lua = Lua::new();
        let script_id = ScriptId::from("asd.lua");
        let initializers = vec![];
        let pre_handling_initializers = vec![];
        let mut old_ctxt = lua.clone();

        lua_context_load(
            &script_id,
            "function hello_world_from_first_load()
            
            end"
            .as_bytes(),
            &initializers,
            &pre_handling_initializers,
            &mut (),
        )
        .unwrap();

        lua_context_reload(
            &script_id,
            "function hello_world_from_second_load()
            
            end"
            .as_bytes(),
            &mut old_ctxt,
            &initializers,
            &pre_handling_initializers,
            &mut (),
        )
        .unwrap();

        // assert both functions exist in globals
        let globals = lua.globals();
        assert!(globals.get::<Value>("hello_world_from_first_load").is_ok());
        assert!(globals.get::<Value>("hello_world_from_second_load").is_ok());
    }
}
