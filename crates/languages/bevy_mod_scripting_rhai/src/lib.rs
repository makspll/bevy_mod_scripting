//! Rhai scripting language support for Bevy.

use std::{ops::Deref, str::Utf8Error, sync::Arc};

use crate::bindings::script_value::{FromDynamic, IntoDynamic};

use ::{
    bevy_app::Plugin,
    bevy_asset::Handle,
    bevy_ecs::{entity::Entity, world::World},
};
use bevy_app::App;
use bevy_ecs::world::{Mut, WorldId};
use bevy_log::trace;
use bevy_mod_scripting_asset::{Language, ScriptAsset};
use bevy_mod_scripting_bindings::{
    AppScriptGlobalsRegistry, InteropError, Namespace, PartialReflectExt, ScriptValue,
    ThreadWorldContainer,
};
use bevy_mod_scripting_core::{
    IntoScriptPluginParams, ScriptingPlugin,
    callbacks::ScriptCallbacks,
    config::{GetPluginThreadConfig, ScriptingPluginConfiguration},
    event::CallbackLabel,
    make_plugin_config_static,
    script::ContextPolicy,
};
use bevy_mod_scripting_display::DisplayProxy;
use bevy_mod_scripting_script::ScriptAttachment;
use bindings::reference::{ReservedKeyword, RhaiReflectReference, RhaiStaticReflectReference};
use parking_lot::RwLock;
pub use rhai;

use rhai::{AST, CallFnOptions, Dynamic, Engine, EvalAltResult, FnPtr, ParseError, Scope};
/// Bindings for rhai.
pub mod bindings;

/// The rhai runtime type.
pub type RhaiRuntime = RwLock<Engine>;

/// The rhai context type.
pub struct RhaiScriptContext {
    /// The AST of the script
    pub ast: AST,
    /// The scope of the script
    pub scope: Scope<'static>,
}

make_plugin_config_static!(RhaiScriptingPlugin);

impl IntoScriptPluginParams for RhaiScriptingPlugin {
    type C = RhaiScriptContext;
    type R = RhaiRuntime;

    const LANGUAGE: Language = Language::Rhai;

    fn build_runtime() -> Self::R {
        Engine::new().into()
    }

    fn handler() -> bevy_mod_scripting_core::handler::HandlerFn<Self> {
        rhai_callback_handler
    }

    fn context_loader() -> bevy_mod_scripting_core::context::ContextLoadFn<Self> {
        rhai_context_load
    }

    fn context_reloader() -> bevy_mod_scripting_core::context::ContextReloadFn<Self> {
        rhai_context_reload
    }
}

/// A trait for converting types into an [`EvalAltResult`]
pub trait IntoRhaiError {
    /// Converts the error into an [`InteropError`]
    fn into_rhai_error(self) -> Box<EvalAltResult>;
}

impl IntoRhaiError for InteropError {
    fn into_rhai_error(self) -> Box<EvalAltResult> {
        Box::new(rhai::EvalAltResult::ErrorSystem(
            "ScriptError".to_owned(),
            Box::new(self),
        ))
    }
}

/// A trait for converting types into an [`InteropError`]
pub trait IntoInteropError {
    /// Converts the error into an [`InteropError`]
    fn into_bms_error(self) -> InteropError;
}

impl IntoInteropError for Box<EvalAltResult> {
    fn into_bms_error(self) -> InteropError {
        match *self {
            rhai::EvalAltResult::ErrorSystem(message, error) => {
                if let Some(inner) = error.downcast_ref::<InteropError>() {
                    inner.clone()
                } else {
                    InteropError::external_boxed(error).with_context(message)
                }
            }
            _ => InteropError::external(self),
        }
    }
}

impl IntoInteropError for ParseError {
    fn into_bms_error(self) -> InteropError {
        InteropError::external(self)
    }
}

impl IntoInteropError for Utf8Error {
    fn into_bms_error(self) -> InteropError {
        InteropError::external(self)
    }
}
/// The rhai scripting plugin. Used to add rhai scripting to a bevy app within the context of the BMS framework.
pub struct RhaiScriptingPlugin {
    /// The internal scripting plugin
    pub scripting_plugin: ScriptingPlugin<RhaiScriptingPlugin>,
}

impl AsMut<ScriptingPlugin<Self>> for RhaiScriptingPlugin {
    fn as_mut(&mut self) -> &mut ScriptingPlugin<Self> {
        &mut self.scripting_plugin
    }
}

fn register_plugin_globals(ctxt: &mut Engine) {
    let register_callback_fn = |callback: String, func: FnPtr| {
        let thread_ctxt = ThreadWorldContainer
            .try_get_context()
            .map_err(|e| Box::new(EvalAltResult::ErrorSystem("".to_string(), Box::new(e))))?;
        let world = thread_ctxt.world;
        let attachment = thread_ctxt.attachment;
        world
            .with_resource_mut(|res: Mut<ScriptCallbacks<RhaiScriptingPlugin>>| {
                let mut callbacks = res.callbacks.write();
                callbacks.insert(
                    (attachment.clone(), callback),
                    Arc::new(
                        move |args: Vec<ScriptValue>,
                              rhai: &mut RhaiScriptContext,
                              world_id: WorldId| {
                            let config = RhaiScriptingPlugin::readonly_configuration(world_id);
                            let pre_handling_callbacks = config.pre_handling_callbacks;
                            let runtime = config.runtime;
                            let runtime_guard = runtime.read();
                            pre_handling_callbacks
                                .iter()
                                .try_for_each(|init| init(&attachment, rhai))?;

                            let ret = func
                                .call::<Dynamic>(&runtime_guard, &rhai.ast, args)
                                .map_err(IntoInteropError::into_bms_error)?;
                            ScriptValue::from_dynamic(ret).map_err(IntoInteropError::into_bms_error)
                        },
                    ),
                )
            })
            .map_err(|e| Box::new(EvalAltResult::ErrorSystem("".to_string(), Box::new(e))))?;
        Ok::<_, Box<EvalAltResult>>(())
    };

    ctxt.register_fn("register_callback", register_callback_fn);
}

impl Default for RhaiScriptingPlugin {
    fn default() -> Self {
        RhaiScriptingPlugin {
            scripting_plugin: ScriptingPlugin {
                supported_extensions: vec!["rhai"],
                runtime_initializers: vec![|runtime| {
                    let mut engine = runtime.write();
                    engine.set_max_expr_depths(999, 999);
                    engine.build_type::<RhaiReflectReference>();
                    engine.build_type::<RhaiStaticReflectReference>();
                    engine.register_iterator_result::<RhaiReflectReference, _>();
                    register_plugin_globals(&mut engine);
                    Ok(())
                }],
                context_initializers: vec![
                    |_, context| {
                        context.scope.set_or_push(
                            "world",
                            RhaiStaticReflectReference(std::any::TypeId::of::<World>()),
                        );
                        Ok(())
                    },
                    |_, context| {
                        // initialize global functions
                        let world = ThreadWorldContainer.try_get_context()?.world;
                        let globals_registry =
                            world.with_resource(|r: &AppScriptGlobalsRegistry| r.clone())?;
                        let globals_registry = globals_registry.read();

                        for (key, global) in globals_registry.iter() {
                            match &global.maker {
                                Some(maker) => {
                                    let global = (maker)(world.clone())?;
                                    context.scope.set_or_push(
                                        key.to_string(),
                                        global
                                            .into_dynamic()
                                            .map_err(IntoInteropError::into_bms_error)?,
                                    );
                                }
                                None => {
                                    let ref_ = RhaiStaticReflectReference(global.type_id);
                                    context.scope.set_or_push(key.to_string(), ref_);
                                }
                            }
                        }

                        let mut script_function_registry = world.script_function_registry();
                        let mut script_function_registry = script_function_registry.write();

                        // iterate all functions, and remap names with reserved keywords
                        let mut re_insertions = Vec::new();
                        for (key, function) in script_function_registry.iter_all() {
                            let name = key.name.clone();
                            if ReservedKeyword::is_reserved_keyword(&name) {
                                let new_name = format!("{name}_");
                                let mut new_function = function.clone();
                                let new_info =
                                    function.info.deref().clone().with_name(new_name.clone());
                                new_function.info = new_info.into();
                                re_insertions.push((key.namespace, new_name, new_function));
                            }
                        }
                        for (namespace, name, func) in re_insertions {
                            script_function_registry.raw_insert(namespace, name, func);
                        }

                        // then go through functions in the global namespace and add them to the lua context

                        for (key, function) in script_function_registry
                            .iter_all()
                            .filter(|(k, _)| k.namespace == Namespace::Global)
                        {
                            context.scope.set_or_push(
                                key.name.clone(),
                                ScriptValue::Function(function.clone())
                                    .into_dynamic()
                                    .map_err(IntoInteropError::into_bms_error)?,
                            );
                        }

                        Ok(())
                    },
                ],
                context_pre_handling_initializers: vec![|context_key, context| {
                    let world = ThreadWorldContainer.try_get_context()?.world;

                    if let Some(entity) = context_key.entity() {
                        context.scope.set_or_push(
                            "entity",
                            RhaiReflectReference(<Entity>::allocate(
                                Box::new(entity),
                                world.clone(),
                            )),
                        );
                    }
                    context.scope.set_or_push(
                        "script_asset",
                        RhaiReflectReference(<Handle<ScriptAsset>>::allocate(
                            Box::new(context_key.script().clone()),
                            world,
                        )),
                    );

                    Ok(())
                }],
                // already supported by BMS core
                language: Language::Rhai,
                context_policy: ContextPolicy::default(),
                emit_responses: false,
                processing_pipeline_plugin: Default::default(),
            },
        }
    }
}

impl Plugin for RhaiScriptingPlugin {
    fn build(&self, app: &mut App) {
        self.scripting_plugin.build(app);
    }

    fn finish(&self, app: &mut App) {
        self.scripting_plugin.finish(app);
    }
}

// NEW helper function to load content into an existing context without clearing previous definitions.
fn load_rhai_content_into_context(
    context: &mut RhaiScriptContext,
    context_key: &ScriptAttachment,
    content: &[u8],
    world_id: WorldId,
) -> Result<(), InteropError> {
    let config = RhaiScriptingPlugin::readonly_configuration(world_id);
    let initializers = config.context_initialization_callbacks;
    let pre_handling_initializers = config.pre_handling_callbacks;
    let runtime = config.runtime.read();

    context.ast = std::str::from_utf8(content)
        .map_err(IntoInteropError::into_bms_error)
        .and_then(|content| {
            runtime
                .compile(content)
                .map_err(IntoInteropError::into_bms_error)
        })?;
    context
        .ast
        .set_source(context_key.script().display().to_string());

    initializers
        .iter()
        .try_for_each(|init| init(context_key, context))?;
    pre_handling_initializers
        .iter()
        .try_for_each(|init| init(context_key, context))?;
    runtime
        .eval_ast_with_scope::<()>(&mut context.scope, &context.ast)
        .map_err(IntoInteropError::into_bms_error)?;

    context.ast.clear_statements();
    Ok(())
}

/// Load a rhai context from a script.
pub fn rhai_context_load(
    context_key: &ScriptAttachment,
    content: &[u8],
    world_id: WorldId,
) -> Result<RhaiScriptContext, InteropError> {
    let mut context = RhaiScriptContext {
        // Using an empty AST as a placeholder.
        ast: AST::empty(),
        scope: Scope::new(),
    };
    load_rhai_content_into_context(&mut context, context_key, content, world_id)?;
    Ok(context)
}

/// Reload a rhai context from a script. New content is appended to the existing context.
pub fn rhai_context_reload(
    context_key: &ScriptAttachment,
    content: &[u8],
    context: &mut RhaiScriptContext,
    world_id: WorldId,
) -> Result<(), InteropError> {
    load_rhai_content_into_context(context, context_key, content, world_id)
}

#[allow(clippy::too_many_arguments)]
/// The rhai callback handler.
pub fn rhai_callback_handler(
    args: Vec<ScriptValue>,
    context_key: &ScriptAttachment,
    callback: &CallbackLabel,
    context: &mut RhaiScriptContext,
    world_id: WorldId,
) -> Result<ScriptValue, InteropError> {
    let config = RhaiScriptingPlugin::readonly_configuration(world_id);
    let pre_handling_initializers = config.pre_handling_callbacks;

    pre_handling_initializers
        .iter()
        .try_for_each(|init| init(context_key, context))?;

    // we want the call to be able to impact the scope
    let options = CallFnOptions::new().rewind_scope(false);
    let args = args
        .into_iter()
        .map(|v| v.into_dynamic())
        .collect::<Result<Vec<_>, _>>()
        .map_err(IntoInteropError::into_bms_error)?;

    trace!(
        "Calling callback {} in context {} with args: {:?}",
        callback, context_key, args
    );
    let runtime = config.runtime.read();

    match runtime.call_fn_with_options::<Dynamic>(
        options,
        &mut context.scope,
        &context.ast,
        callback.as_ref(),
        args,
    ) {
        Ok(v) => Ok(ScriptValue::from_dynamic(v).map_err(IntoInteropError::into_bms_error)?),
        Err(e) => {
            if let EvalAltResult::ErrorFunctionNotFound(_, _) = e.unwrap_inner() {
                trace!(
                    "Context {} is not subscribed to callback {} with the provided arguments.",
                    context_key, callback
                );
                Ok(ScriptValue::Unit)
            } else {
                Err(e.into_bms_error())
            }
        }
    }
}
