//! Lua integration for the bevy_mod_scripting system.
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use ::{
    bevy_app::Plugin,
    bevy_asset::Handle,
    bevy_ecs::{entity::Entity, world::World},
};
use bevy_app::App;
use bevy_asset::AssetPath;
use bevy_ecs::world::{Mut, WorldId};
use bevy_log::trace;
use bevy_mod_scripting_asset::{Language, ScriptAsset};
use bevy_mod_scripting_bindings::{
    InteropError, PartialReflectExt, ThreadWorldContainer, function::namespace::Namespace,
    globals::AppScriptGlobalsRegistry, script_value::ScriptValue,
};
use bevy_mod_scripting_core::{
    IntoScriptPluginParams, ScriptingPlugin,
    callbacks::ScriptCallbacks,
    config::{GetPluginThreadConfig, ScriptingPluginConfiguration},
    event::CallbackLabel,
    make_plugin_config_static,
    script::ContextPolicy,
};
use bevy_mod_scripting_script::ScriptAttachment;
use bindings::{
    reference::{LuaReflectReference, LuaStaticReflectReference},
    script_value::LuaScriptValue,
};
pub use mlua;
use mlua::{Function, IntoLua, Lua, MultiValue, Variadic};

/// Bindings for lua.
pub mod bindings;

make_plugin_config_static!(LuaScriptingPlugin);

/// A newtype around a lua context.
#[derive(Debug, Clone)]
pub struct LuaContext(Lua);

impl Deref for LuaContext {
    type Target = Lua;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoScriptPluginParams for LuaScriptingPlugin {
    type C = LuaContext;
    type R = ();
    const LANGUAGE: Language = Language::Lua;

    fn build_runtime() -> Self::R {}

    fn handler() -> bevy_mod_scripting_core::handler::HandlerFn<Self> {
        lua_handler
    }

    fn context_loader() -> bevy_mod_scripting_core::context::ContextLoadFn<Self> {
        lua_context_load
    }

    fn context_reloader() -> bevy_mod_scripting_core::context::ContextReloadFn<Self> {
        lua_context_reload
    }
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

fn register_plugin_globals(lua: &mut Lua) -> Result<(), mlua::Error> {
    lua.globals().set(
        "register_callback",
        lua.create_function(|_lua: &Lua, (callback, func): (String, Function)| {
            let thread_ctxt = ThreadWorldContainer
                .try_get_context()
                .map_err(mlua::Error::external)?;
            let world = thread_ctxt.world;
            let attachment = thread_ctxt.attachment;
            world
                .with_resource_mut(|res: Mut<ScriptCallbacks<LuaScriptingPlugin>>| {
                    let mut callbacks = res.callbacks.write();
                    callbacks.insert(
                        (attachment.clone(), callback),
                        Arc::new(
                            move |args: Vec<ScriptValue>,
                                  lua: &mut LuaContext,
                                  world_id: WorldId| {
                                let pre_handling_callbacks =
                                    LuaScriptingPlugin::readonly_configuration(world_id)
                                        .pre_handling_callbacks;

                                pre_handling_callbacks
                                    .iter()
                                    .try_for_each(|init| init(&attachment, lua))?;

                                let args = args
                                    .into_iter()
                                    .map(LuaScriptValue)
                                    .collect::<Variadic<_>>();

                                func.call::<LuaScriptValue>(args)
                                    .map_err(IntoInteropError::to_bms_error)
                                    .map(ScriptValue::from)
                            },
                        ),
                    )
                })
                .map_err(mlua::Error::external)?;
            Ok(())
        })?,
    )?;
    Ok(())
}

impl Default for LuaScriptingPlugin {
    fn default() -> Self {
        LuaScriptingPlugin {
            scripting_plugin: ScriptingPlugin {
                runtime_initializers: Vec::default(),
                supported_extensions: vec!["lua", "luau"],
                context_initializers: vec![
                    |_script_id, context| {
                        // set the world global
                        let globals = context.globals();

                        globals
                            .set(
                                "world",
                                LuaStaticReflectReference(std::any::TypeId::of::<World>()),
                            )
                            .map_err(IntoInteropError::to_bms_error)?;

                        register_plugin_globals(context).map_err(IntoInteropError::to_bms_error)?;

                        Ok(())
                    },
                    |_script_id, context| {
                        // set static globals
                        let world = ThreadWorldContainer.try_get_context()?.world;
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
                                        .set(key.to_string(), LuaScriptValue::from(global))
                                        .map_err(IntoInteropError::to_bms_error)?
                                }
                                None => {
                                    let ref_ = LuaStaticReflectReference(global.type_id);
                                    context
                                        .globals()
                                        .set(key.to_string(), ref_)
                                        .map_err(IntoInteropError::to_bms_error)?
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
                                .map_err(IntoInteropError::to_bms_error)?;
                        }

                        Ok(())
                    },
                ],
                context_pre_handling_initializers: vec![|context_key, context| {
                    // TODO: convert these to functions
                    let world = ThreadWorldContainer.try_get_context()?.world;
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
                            .map_err(IntoInteropError::to_bms_error)?;
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
                        .map_err(IntoInteropError::to_bms_error)?;

                    Ok(())
                }],
                language: Language::Lua,
                context_policy: ContextPolicy::default(),
                emit_responses: false,
                processing_pipeline_plugin: Default::default(),
            },
        }
    }
}

impl Plugin for LuaScriptingPlugin {
    fn build(&self, app: &mut App) {
        self.scripting_plugin.build(app);
    }

    fn finish(&self, app: &mut App) {
        self.scripting_plugin.finish(app);
    }
}

fn load_lua_content_into_context(
    context: &mut LuaContext,
    context_key: &ScriptAttachment,
    content: &[u8],
    world_id: WorldId,
) -> Result<(), InteropError> {
    let config = LuaScriptingPlugin::readonly_configuration(world_id);
    let initializers = config.context_initialization_callbacks;
    let pre_handling_initializers = config.pre_handling_callbacks;
    initializers
        .iter()
        .try_for_each(|init| init(context_key, context))?;

    pre_handling_initializers
        .iter()
        .try_for_each(|init| init(context_key, context))?;

    context
        .load(content)
        .exec()
        .map_err(IntoInteropError::to_bms_error)?;

    Ok(())
}

/// App data which can be retrieved via [`mlua::Lua::app_data_ref`], containing some metadata about scripts present
#[derive(Default, Debug)]
pub struct LuaContextAppData {
    /// the asset path of the script loaded last if this is a shared context, or the only script if it's not.
    pub last_loaded_script_name: Option<AssetPath<'static>>,
}

#[profiling::function]
/// Load a lua context from a script
pub fn lua_context_load(
    context_key: &ScriptAttachment,
    content: &[u8],
    world_id: WorldId,
) -> Result<LuaContext, InteropError> {
    #[cfg(feature = "unsafe_lua_modules")]
    let mut context = LuaContext(unsafe { Lua::unsafe_new() });
    #[cfg(not(feature = "unsafe_lua_modules"))]
    let mut context = LuaContext(Lua::new());

    context.set_app_data(LuaContextAppData {
        last_loaded_script_name: context_key.script().path().cloned(),
    });

    load_lua_content_into_context(&mut context, context_key, content, world_id)?;
    Ok(context)
}

#[profiling::function]
/// Reload a lua context from a script
pub fn lua_context_reload(
    context_key: &ScriptAttachment,
    content: &[u8],
    old_ctxt: &mut LuaContext,
    world_id: WorldId,
) -> Result<(), InteropError> {
    load_lua_content_into_context(old_ctxt, context_key, content, world_id)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[profiling::function]
/// The lua handler for events
pub fn lua_handler(
    args: Vec<ScriptValue>,
    context_key: &ScriptAttachment,
    callback_label: &CallbackLabel,
    context: &mut LuaContext,
    world_id: WorldId,
) -> Result<ScriptValue, bevy_mod_scripting_bindings::InteropError> {
    let config = LuaScriptingPlugin::readonly_configuration(world_id);

    config
        .pre_handling_callbacks
        .iter()
        .try_for_each(|init| init(context_key, context))?;

    let handler: Function = match context.globals().raw_get(callback_label.as_ref()) {
        Ok(handler) => handler,
        // not subscribed to this event type
        Err(_) => {
            trace!(
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
            .collect::<Result<_, _>>()
            .map_err(IntoInteropError::to_bms_error)?,
    );

    let out = handler
        .call::<LuaScriptValue>(input)
        .map_err(IntoInteropError::to_bms_error)?;
    Ok(out.into())
}

/// A trait to convert between mlua::Error and InteropError
pub trait IntoInteropError {
    /// Convert into InteropError
    fn to_bms_error(self) -> InteropError;
}

impl IntoInteropError for mlua::Error {
    fn to_bms_error(self) -> InteropError {
        match self {
            mlua::Error::CallbackError { traceback, cause }
                if matches!(cause.as_ref(), mlua::Error::ExternalError(_)) =>
            {
                let inner = cause.deref().clone();
                inner.to_bms_error().with_context(traceback)
            }
            e => {
                if let Some(inner) = e.downcast_ref::<InteropError>() {
                    inner.clone()
                } else {
                    InteropError::external(e)
                }
            }
        }
    }
}

/// A trait to convert between InteropError and mlua::Error
pub trait IntoMluaError {
    /// Convert into mlua::Error
    fn to_lua_error(self) -> mlua::Error;
}

impl IntoMluaError for InteropError {
    fn to_lua_error(self) -> mlua::Error {
        mlua::Error::external(self)
    }
}
#[cfg(test)]
mod test {
    use ::bevy_asset::{AssetId, AssetIndex, Handle};
    use bevy_mod_scripting_asset::LanguageExtensions;
    use mlua::Value;

    use super::*;

    #[test]
    fn test_reload_doesnt_overwrite_old_context() {
        let lua = Lua::new();
        let mut old_ctxt = LuaContext(lua.clone());
        let handle = Handle::Weak(AssetId::from(AssetIndex::from_bits(0)));
        let context_key = ScriptAttachment::EntityScript(Entity::from_raw(1), handle);
        let world_id = WorldId::new().unwrap();
        LuaScriptingPlugin::set_world_local_config(
            world_id,
            ScriptingPluginConfiguration {
                pre_handling_callbacks: &[],
                context_initialization_callbacks: &[],
                emit_responses: false,
                runtime: &(),
                language_extensions: Box::leak(Box::new(LanguageExtensions::default())),
            },
        );
        lua_context_load(
            &context_key,
            "function hello_world_from_first_load()

            end"
            .as_bytes(),
            world_id,
        )
        .unwrap();

        lua_context_reload(
            &context_key,
            "function hello_world_from_second_load()

            end"
            .as_bytes(),
            &mut old_ctxt,
            world_id,
        )
        .unwrap();

        // assert both functions exist in globals
        let globals = lua.globals();
        assert!(globals.get::<Value>("hello_world_from_first_load").is_ok());
        assert!(globals.get::<Value>("hello_world_from_second_load").is_ok());
    }
}
