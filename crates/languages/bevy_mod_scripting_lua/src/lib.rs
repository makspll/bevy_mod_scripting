//! Lua integration for the bevy_mod_scripting system.
use bevy::{
    app::Plugin,
    asset::Handle,
    ecs::{entity::Entity, world::World},
};
use bevy_mod_scripting_core::{
    asset::{Language, ScriptAsset},
    bindings::{
        function::namespace::Namespace, globals::AppScriptGlobalsRegistry,
        script_value::ScriptValue, ThreadWorldContainer, WorldContainer,
    },
    context::{ContextBuilder, ContextInitializer, ContextPreHandlingInitializer},
    error::ScriptError,
    event::CallbackLabel,
    reflection_extensions::PartialReflectExt,
    runtime::RuntimeSettings,
    script::{ContextPolicy, ScriptAttachment},
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
                runtime_settings: RuntimeSettings::default(),
                callback_handler: lua_handler,
                context_builder: ContextBuilder::<LuaScriptingPlugin> {
                    load: lua_context_load,
                    reload: lua_context_reload,
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
                    |_script_id, context| {
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
                context_pre_handling_initializers: vec![|context_key, context| {
                    // TODO: convert these to functions
                    let world = ThreadWorldContainer.try_get_world()?;
                    if let Some(entity) = context_key.entity() {
                        context
                            .globals()
                            .set(
                                "entity",
                                LuaReflectReference(<Entity>::allocate(
                                    Box::new(entity),
                                    world.clone(),
                                )),
                            )
                            .map_err(ScriptError::from_mlua_error)?;
                    }
                    context
                        .globals()
                        .set(
                            "script_asset",
                            LuaReflectReference(<Handle<ScriptAsset>>::allocate(
                                Box::new(context_key.script()),
                                world,
                            )),
                        )
                        .map_err(ScriptError::from_mlua_error)?;

                    Ok(())
                }],
                language: Language::Lua,
                context_policy: ContextPolicy::default(),
                emit_responses: false,
            },
        }
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
    context_key: &ScriptAttachment,
    content: &[u8],
    initializers: &[ContextInitializer<LuaScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<LuaScriptingPlugin>],
) -> Result<(), ScriptError> {
    initializers
        .iter()
        .try_for_each(|init| init(context_key, context))?;

    pre_handling_initializers
        .iter()
        .try_for_each(|init| init(context_key, context))?;

    context
        .load(content)
        .exec()
        .map_err(ScriptError::from_mlua_error)?;

    Ok(())
}

#[profiling::function]
/// Load a lua context from a script
pub fn lua_context_load(
    context_key: &ScriptAttachment,
    content: &[u8],
    initializers: &[ContextInitializer<LuaScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<LuaScriptingPlugin>],
    _: &(),
) -> Result<Lua, ScriptError> {
    #[cfg(feature = "unsafe_lua_modules")]
    let mut context = unsafe { Lua::unsafe_new() };
    #[cfg(not(feature = "unsafe_lua_modules"))]
    let mut context = Lua::new();

    load_lua_content_into_context(
        &mut context,
        context_key,
        content,
        initializers,
        pre_handling_initializers,
    )?;
    Ok(context)
}

#[profiling::function]
/// Reload a lua context from a script
pub fn lua_context_reload(
    context_key: &ScriptAttachment,
    content: &[u8],
    old_ctxt: &mut Lua,
    initializers: &[ContextInitializer<LuaScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<LuaScriptingPlugin>],
    _: &(),
) -> Result<(), ScriptError> {
    load_lua_content_into_context(
        old_ctxt,
        context_key,
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
    context_key: &ScriptAttachment,
    callback_label: &CallbackLabel,
    context: &mut Lua,
    pre_handling_initializers: &[ContextPreHandlingInitializer<LuaScriptingPlugin>],
    _: &(),
) -> Result<ScriptValue, bevy_mod_scripting_core::error::ScriptError> {
    pre_handling_initializers
        .iter()
        .try_for_each(|init| init(context_key, context))?;

    let handler: Function = match context.globals().raw_get(callback_label.as_ref()) {
        Ok(handler) => handler,
        // not subscribed to this event type
        Err(_) => {
            bevy::log::trace!(
                "Context {} is not subscribed to callback {}",
                context_key,
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
    use bevy::{
        asset::{AssetId, AssetIndex},
        prelude::Handle,
    };
    use mlua::Value;

    use super::*;

    #[test]
    fn test_reload_doesnt_overwrite_old_context() {
        let lua = Lua::new();
        let initializers = vec![];
        let pre_handling_initializers = vec![];
        let mut old_ctxt = lua.clone();
        let handle = Handle::Weak(AssetId::from(AssetIndex::from_bits(0)));
        let context_key = ScriptAttachment::EntityScript(Entity::from_raw(1), handle);

        lua_context_load(
            &context_key,
            "function hello_world_from_first_load()

            end"
            .as_bytes(),
            &initializers,
            &pre_handling_initializers,
            &(),
        )
        .unwrap();

        lua_context_reload(
            &context_key,
            "function hello_world_from_second_load()

            end"
            .as_bytes(),
            &mut old_ctxt,
            &initializers,
            &pre_handling_initializers,
            &(),
        )
        .unwrap();

        // assert both functions exist in globals
        let globals = lua.globals();
        assert!(globals.get::<Value>("hello_world_from_first_load").is_ok());
        assert!(globals.get::<Value>("hello_world_from_second_load").is_ok());
    }
}
