use bevy::{
    app::Plugin,
    ecs::{entity::Entity, world::World},
};
use bevy_mod_scripting_core::{
    asset::{AssetPathToLanguageMapper, Language},
    bindings::{
        function::namespace::Namespace, script_value::ScriptValue, ThreadWorldContainer,
        WorldContainer,
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

pub struct LuaScriptingPlugin {
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
                        let type_registry = world.type_registry();
                        let type_registry = type_registry.read();

                        for registration in type_registry.iter() {
                            // only do this for non generic types
                            // we don't want to see `Vec<Entity>:function()` in lua
                            if !registration.type_info().generics().is_empty() {
                                continue;
                            }

                            if let Some(global_name) =
                                registration.type_info().type_path_table().ident()
                            {
                                let ref_ = LuaStaticReflectReference(registration.type_id());
                                context
                                    .globals()
                                    .set(global_name, ref_)
                                    .map_err(ScriptError::from_mlua_error)?;
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
            },
        }
    }
}
#[profiling::function]
fn lua_language_mapper(path: &std::path::Path) -> Language {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("lua") => Language::Lua,
        _ => Language::Unknown,
    }
}

impl Plugin for LuaScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        self.scripting_plugin.build(app);
    }
}
#[profiling::function]
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

    initializers
        .iter()
        .try_for_each(|init| init(script_id, &mut context))?;

    pre_handling_initializers
        .iter()
        .try_for_each(|init| init(script_id, Entity::from_raw(0), &mut context))?;

    context
        .load(content)
        .exec()
        .map_err(ScriptError::from_mlua_error)?;

    Ok(context)
}
#[profiling::function]
pub fn lua_context_reload(
    script: &ScriptId,
    content: &[u8],
    old_ctxt: &mut Lua,
    initializers: &[ContextInitializer<LuaScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<LuaScriptingPlugin>],
    _: &mut (),
) -> Result<(), ScriptError> {
    *old_ctxt = lua_context_load(
        script,
        content,
        initializers,
        pre_handling_initializers,
        &mut (),
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[profiling::function]
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
