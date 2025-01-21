use bevy::{
    app::Plugin,
    ecs::{entity::Entity, world::World},
};
use bevy_mod_scripting_core::{
    asset::{AssetPathToLanguageMapper, Language},
    bindings::{script_value::ScriptValue, ThreadWorldContainer, WorldContainer},
    context::{ContextBuilder, ContextInitializer, ContextPreHandlingInitializer},
    error::ScriptError,
    event::CallbackLabel,
    reflection_extensions::PartialReflectExt,
    runtime::RuntimeSettings,
    script::ScriptId,
    IntoScriptPluginParams, ScriptingPlugin,
};
use bindings::reference::{RhaiReflectReference, RhaiStaticReflectReference};
use rhai::{CallFnOptions, Engine, FnPtr, Scope, AST};

pub use rhai;
pub mod bindings;

pub type RhaiRuntime = Engine;

pub struct RhaiScriptContext {
    pub ast: AST,
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

pub struct RhaiScriptingPlugin {
    pub scripting_plugin: ScriptingPlugin<RhaiScriptingPlugin>,
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
                context_initializers: vec![|_script_id: _, context: &mut RhaiScriptContext| {
                    context.scope.set_or_push(
                        "world",
                        RhaiStaticReflectReference(std::any::TypeId::of::<World>()),
                    );
                    Ok(())
                }],
                context_pre_handling_initializers: vec![|script, entity, context| {
                    let world = ThreadWorldContainer.try_get_world()?;
                    context.scope.set_or_push(
                        "entity",
                        RhaiReflectReference(<Entity>::allocate(Box::new(entity), world)),
                    );
                    context.scope.set_or_push("script_id", script.to_owned());
                    Ok(())
                }],
            },
        }
    }
}

fn rhai_language_mapper(path: &std::path::Path) -> Language {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("rhai") => Language::Rhai,
        _ => Language::Unknown,
    }
}

impl Plugin for RhaiScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        self.scripting_plugin.build(app);
    }

    fn cleanup(&self, _app: &mut bevy::prelude::App) {
        // let mut runtime = app
        //     .world_mut()
        //     .get_non_send_resource_mut::<RuntimeContainer<Self>>()
        //     .expect("Rhai runtime not found");
        // let engine = &mut runtime.runtime;
        // let function_registry = app
        //     .world_mut()
        //     .get_resource_or_init::<AppScriptFunctionRegistry>();

        // let function_registry = function_registry.read();

        // for (k, func) in function_registry.iter_all() {
        //     let rhai_func = to_rhai_fn(func.clone());
        //     // engine.register_fn("func", rhai_func);
        // }
    }
}

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

    if context
        .scope
        .get_value::<FnPtr>(callback.as_ref())
        .is_none()
    {
        // not subscribed to this handler
        return Ok(ScriptValue::Unit);
    };

    // we want the call to be able to impact the scope
    let options = CallFnOptions::new().rewind_scope(false);
    let out = runtime.call_fn_with_options::<ScriptValue>(
        options,
        &mut context.scope,
        &context.ast,
        callback.as_ref(),
        args,
    )?;
    Ok(out)
}
