use bevy::{
    app::Plugin,
    ecs::{entity::Entity, world::World},
};
use bevy_mod_scripting_core::{
    asset::{AssetPathToLanguageMapper, Language},
    bindings::{
        script_value::ScriptValue, ThreadWorldContainer, WorldCallbackAccess, WorldContainer,
    },
    context::{ContextBuilder, ContextInitializer, ContextPreHandlingInitializer},
    error::ScriptError,
    event::CallbackLabel,
    script::ScriptId,
    IntoScriptPluginParams, ScriptingPlugin,
};
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
                runtime_settings: None,
                callback_handler: Some(rhai_callback_handler),
                context_assigner: None,
                context_builder: Some(ContextBuilder {
                    load: rhai_context_load,
                    reload: rhai_context_reload,
                }),
                language_mapper: Some(AssetPathToLanguageMapper {
                    map: rhai_language_mapper,
                }),
                context_initializers: Default::default(),
                context_pre_handling_initializers: vec![|_script, _entity, context| {
                    let world = ThreadWorldContainer.get_world();
                    context.scope.push("world", world);
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
}

pub fn rhai_context_load(
    script: &ScriptId,
    content: &[u8],
    initializers: &[ContextInitializer<RhaiScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<RhaiScriptingPlugin>],
    world: &mut World,
    runtime: &mut RhaiRuntime,
) -> Result<RhaiScriptContext, ScriptError> {
    let mut ast = runtime.compile(std::str::from_utf8(content)?)?;
    ast.set_source(script.to_string());

    let mut context = RhaiScriptContext {
        ast,
        scope: Scope::new(),
    };
    with_world(world, &mut context, |context| {
        initializers
            .iter()
            .try_for_each(|init| init(script, context))?;

        pre_handling_initializers
            .iter()
            .try_for_each(|init| init(script, Entity::from_raw(0), context))?;

        runtime.eval_ast_with_scope(&mut context.scope, &context.ast)?;
        // do not invoke top level statements after the first time we run the script
        context.ast.clear_statements();

        Ok(())
    })?;
    Ok(context)
}

pub fn rhai_context_reload(
    script: &ScriptId,
    content: &[u8],
    context: &mut RhaiScriptContext,
    initializers: &[ContextInitializer<RhaiScriptingPlugin>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<RhaiScriptingPlugin>],
    world: &mut World,
    runtime: &mut RhaiRuntime,
) -> Result<(), ScriptError> {
    *context = rhai_context_load(
        script,
        content,
        initializers,
        pre_handling_initializers,
        world,
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
    world: &mut World,
) -> Result<ScriptValue, ScriptError> {
    with_world(world, context, |context| {
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
    })
}

pub fn with_world<O, F: FnOnce(&mut RhaiScriptContext) -> Result<O, ScriptError>>(
    world: &mut World,
    context: &mut RhaiScriptContext,
    f: F,
) -> Result<O, ScriptError> {
    WorldCallbackAccess::with_callback_access(world, |guard| {
        ThreadWorldContainer.set_world(guard.clone())?;
        f(context)
    })
}
