//! Rhai scripting language support for Bevy.

use bevy::{
    app::Plugin,
    asset::AssetPath,
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
    reference::{ReservedKeyword, RhaiReflectReference, RhaiStaticReflectReference},
    script_value::{FromDynamic, IntoDynamic},
};
use rhai::{CallFnOptions, Dynamic, Engine, EvalAltResult, Scope, AST};

pub use rhai;
/// Bindings for rhai.
pub mod bindings;

/// The rhai runtime type.
pub type RhaiRuntime = Engine;

/// The rhai context type.
pub struct RhaiScriptContext {
    /// The AST of the script
    pub ast: AST,
    /// The scope of the script
    pub scope: Scope<'static>,
}

impl IntoScriptPluginParams for RhaiScriptingPlugin {
    type C = RhaiScriptContext;
    type R = RhaiRuntime;

    const LANGUAGE: Language = Language::Rhai;

    fn build_runtime() -> Self::R {
        RhaiRuntime::new()
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

impl Default for RhaiScriptingPlugin {
    fn default() -> Self {
        RhaiScriptingPlugin {
            scripting_plugin: ScriptingPlugin {
                context_assigner: Default::default(),
                runtime_settings: RuntimeSettings {
                    initializers: vec![|runtime: &mut Engine| {
                        runtime.build_type::<RhaiReflectReference>();
                        runtime.build_type::<RhaiStaticReflectReference>();
                        runtime.register_iterator_result::<RhaiReflectReference, _>();
                        Ok(())
                    }],
                },
                callback_handler: rhai_callback_handler,
                context_builder: ContextBuilder {
                    load: rhai_context_load,
                    reload: rhai_context_reload,
                },
                language_mapper: AssetPathToLanguageMapper {
                    map: rhai_language_mapper,
                },
                context_initializers: vec![
                    |_, context: &mut RhaiScriptContext| {
                        context.scope.set_or_push(
                            "world",
                            RhaiStaticReflectReference(std::any::TypeId::of::<World>()),
                        );
                        Ok(())
                    },
                    |_, context: &mut RhaiScriptContext| {
                        // initialize global functions
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
                                let ref_ = RhaiStaticReflectReference(registration.type_id());
                                context.scope.set_or_push(global_name, ref_);
                            }
                        }

                        let mut script_function_registry = world.script_function_registry();
                        let mut script_function_registry = script_function_registry.write();

                        // iterate all functions, and remap names with reserved keywords
                        let mut re_insertions = Vec::new();
                        for (key, function) in script_function_registry.iter_all() {
                            let name = key.name.clone();
                            if ReservedKeyword::is_reserved_keyword(&name) {
                                let new_name = format!("{}_", name);
                                let mut function = function.clone();
                                function.info.name = new_name.clone().into();
                                re_insertions.push((key.namespace, new_name, function.clone()));
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
                                ScriptValue::Function(function.clone()).into_dynamic()?,
                            );
                        }

                        Ok(())
                    },
                ],
                context_pre_handling_initializers: vec![|script, entity, context| {
                    let world = ThreadWorldContainer.try_get_world()?;
                    context.scope.set_or_push(
                        "entity",
                        RhaiReflectReference(<Entity>::allocate(Box::new(entity), world)),
                    );
                    context.scope.set_or_push("script_id", script.to_owned());
                    Ok(())
                }],
                supported_extensions: &["rhai"],
            },
        }
    }
}

fn rhai_language_mapper(path: &AssetPath) -> Language {
    match path.path().extension().and_then(|ext| ext.to_str()) {
        Some("rhai") => Language::Rhai,
        _ => Language::Unknown,
    }
}

impl Plugin for RhaiScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        self.scripting_plugin.build(app);
    }

    fn finish(&self, app: &mut bevy::app::App) {
        self.scripting_plugin.finish(app);
    }
}

/// Load a rhai context from a script.
pub fn rhai_context_load(
    script: &ScriptId,
    content: &[u8],
    initializers: &[ContextInitializer<RhaiScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<RhaiScriptingPlugin>],
    runtime: &mut RhaiRuntime,
) -> Result<RhaiScriptContext, ScriptError> {
    let mut ast = runtime.compile(std::str::from_utf8(content)?)?;
    ast.set_source(script.to_string());

    let mut context = RhaiScriptContext {
        ast,
        scope: Scope::new(),
    };
    initializers
        .iter()
        .try_for_each(|init| init(script, &mut context))?;

    pre_handling_initializers
        .iter()
        .try_for_each(|init| init(script, Entity::from_raw(0), &mut context))?;

    runtime.eval_ast_with_scope(&mut context.scope, &context.ast)?;
    // do not invoke top level statements after the first time we run the script
    context.ast.clear_statements();

    Ok(context)
}

/// Reload a rhai context from a script.
pub fn rhai_context_reload(
    script: &ScriptId,
    content: &[u8],
    context: &mut RhaiScriptContext,
    initializers: &[ContextInitializer<RhaiScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<RhaiScriptingPlugin>],
    runtime: &mut RhaiRuntime,
) -> Result<(), ScriptError> {
    *context = rhai_context_load(
        script,
        content,
        initializers,
        pre_handling_initializers,
        runtime,
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
/// The rhai callback handler.
pub fn rhai_callback_handler(
    args: Vec<ScriptValue>,
    entity: Entity,
    script_id: &ScriptId,
    callback: &CallbackLabel,
    context: &mut RhaiScriptContext,
    pre_handling_initializers: &[ContextPreHandlingInitializer<RhaiScriptingPlugin>],
    runtime: &mut RhaiRuntime,
) -> Result<ScriptValue, ScriptError> {
    pre_handling_initializers
        .iter()
        .try_for_each(|init| init(script_id, entity, context))?;

    // we want the call to be able to impact the scope
    let options = CallFnOptions::new().rewind_scope(false);
    let args = args
        .into_iter()
        .map(|v| v.into_dynamic())
        .collect::<Result<Vec<_>, _>>()?;

    bevy::log::trace!(
        "Calling callback {} in script {} with args: {:?}",
        callback,
        script_id,
        args
    );
    match runtime.call_fn_with_options::<Dynamic>(
        options,
        &mut context.scope,
        &context.ast,
        callback.as_ref(),
        args,
    ) {
        Ok(v) => Ok(ScriptValue::from_dynamic(v)?),
        Err(e) => {
            if let EvalAltResult::ErrorFunctionNotFound(_, _) = e.unwrap_inner() {
                bevy::log::trace!(
                    "Script {} is not subscribed to callback {} with the provided arguments.",
                    script_id,
                    callback
                );
                Ok(ScriptValue::Unit)
            } else {
                Err(ScriptError::from(e))
            }
        }
    }
}
