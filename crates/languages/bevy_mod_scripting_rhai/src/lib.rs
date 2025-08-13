//! Rhai scripting language support for Bevy.

use std::ops::Deref;

use bevy::{
    app::Plugin,
    ecs::{entity::Entity, world::World},
};
use bevy_mod_scripting_core::{
    asset::Language,
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
    reference::{ReservedKeyword, RhaiReflectReference, RhaiStaticReflectReference},
    script_value::{FromDynamic, IntoDynamic},
};
use parking_lot::RwLock;
pub use rhai;
use rhai::{CallFnOptions, Dynamic, Engine, EvalAltResult, Scope, AST};
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

impl IntoScriptPluginParams for RhaiScriptingPlugin {
    type C = RhaiScriptContext;
    type R = RhaiRuntime;

    const LANGUAGE: Language = Language::Rhai;

    fn build_runtime() -> Self::R {
        Engine::new().into()
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
                context_assignment_strategy: Default::default(),
                runtime_settings: RuntimeSettings {
                    initializers: vec![|runtime| {
                        let mut engine = runtime.write();
                        engine.set_max_expr_depths(999, 999);
                        engine.build_type::<RhaiReflectReference>();
                        engine.build_type::<RhaiStaticReflectReference>();
                        engine.register_iterator_result::<RhaiReflectReference, _>();
                        Ok(())
                    }],
                },
                callback_handler: rhai_callback_handler,
                context_builder: ContextBuilder {
                    load: rhai_context_load,
                    reload: rhai_context_reload,
                },
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
                        let world = ThreadWorldContainer.try_get_world()?;
                        let globals_registry =
                            world.with_resource(|r: &AppScriptGlobalsRegistry| r.clone())?;
                        let globals_registry = globals_registry.read();

                        for (key, global) in globals_registry.iter() {
                            match &global.maker {
                                Some(maker) => {
                                    let global = (maker)(world.clone())?;
                                    context
                                        .scope
                                        .set_or_push(key.to_string(), global.into_dynamic()?);
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
                // already supported by BMS core
                additional_supported_extensions: &[],
                language: Language::Rhai,
            },
        }
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

// NEW helper function to load content into an existing context without clearing previous definitions.
fn load_rhai_content_into_context(
    context: &mut RhaiScriptContext,
    script: &ScriptId,
    content: &[u8],
    initializers: &[ContextInitializer<RhaiScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<RhaiScriptingPlugin>],
    runtime: &RhaiRuntime,
) -> Result<(), ScriptError> {
    let runtime = runtime.read();

    context.ast = runtime.compile(std::str::from_utf8(content)?)?;
    context.ast.set_source(script.to_string());

    initializers
        .iter()
        .try_for_each(|init| init(script, context))?;
    pre_handling_initializers
        .iter()
        .try_for_each(|init| init(script, Entity::from_raw(0), context))?;
    runtime.eval_ast_with_scope(&mut context.scope, &context.ast)?;

    context.ast.clear_statements();
    Ok(())
}

/// Load a rhai context from a script.
pub fn rhai_context_load(
    script: &ScriptId,
    content: &[u8],
    initializers: &[ContextInitializer<RhaiScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<RhaiScriptingPlugin>],
    runtime: &RhaiRuntime,
) -> Result<RhaiScriptContext, ScriptError> {
    let mut context = RhaiScriptContext {
        // Using an empty AST as a placeholder.
        ast: AST::empty(),
        scope: Scope::new(),
    };
    load_rhai_content_into_context(
        &mut context,
        script,
        content,
        initializers,
        pre_handling_initializers,
        runtime,
    )?;
    Ok(context)
}

/// Reload a rhai context from a script. New content is appended to the existing context.
pub fn rhai_context_reload(
    script: &ScriptId,
    content: &[u8],
    context: &mut RhaiScriptContext,
    initializers: &[ContextInitializer<RhaiScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<RhaiScriptingPlugin>],
    runtime: &RhaiRuntime,
) -> Result<(), ScriptError> {
    load_rhai_content_into_context(
        context,
        script,
        content,
        initializers,
        pre_handling_initializers,
        runtime,
    )
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
    runtime: &RhaiRuntime,
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
    let runtime = runtime.read();

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reload_doesnt_overwrite_old_context() {
        let runtime = RhaiRuntime::new(Engine::new());
        let script_id = ScriptId::from("asd.rhai");
        let initializers: Vec<ContextInitializer<RhaiScriptingPlugin>> = vec![];
        let pre_handling_initializers: Vec<ContextPreHandlingInitializer<RhaiScriptingPlugin>> =
            vec![];

        // Load first content defining a function that returns 42.
        let mut context = rhai_context_load(
            &script_id,
            b"let hello = 2;",
            &initializers,
            &pre_handling_initializers,
            &runtime,
        )
        .unwrap();

        // Reload with additional content defining a second function that returns 24.
        rhai_context_reload(
            &script_id,
            b"let hello2 = 3",
            &mut context,
            &initializers,
            &pre_handling_initializers,
            &runtime,
        )
        .unwrap();

        // get first var
        let hello = context.scope.get_value::<i64>("hello").unwrap();
        assert_eq!(hello, 2);
        // get second var
        let hello2 = context.scope.get_value::<i64>("hello2").unwrap();
        assert_eq!(hello2, 3);
    }
}
