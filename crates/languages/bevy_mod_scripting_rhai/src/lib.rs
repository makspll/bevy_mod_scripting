use bevy::{
    app::Plugin,
    ecs::{entity::Entity, world::World},
};
use bevy_mod_scripting_core::{
    bindings::WorldCallbackAccess,
    context::{ContextAssigner, ContextBuilder, ContextInitializer, ContextPreHandlingInitializer},
    error::ScriptError,
    event::CallbackLabel,
    handler::Args,
    script::ScriptId,
    ScriptingPlugin,
};
use rhai::{CallFnOptions, Engine, FnPtr, FuncArgs, Scope, AST};

pub use rhai;
pub mod prelude {
    pub use rhai;
    pub use rhai::FuncArgs;
}

pub trait RhaiEventArg: Args + FuncArgs {}
impl<T: Args + FuncArgs> RhaiEventArg for T {}

pub type RhaiRuntime = Engine;

pub struct RhaiScriptContext {
    pub ast: AST,
    pub scope: Scope<'static>,
}

pub struct RhaiScriptingPlugin<A: RhaiEventArg> {
    pub scripting_plugin: ScriptingPlugin<A, RhaiScriptContext, RhaiRuntime>,
}

impl<A: RhaiEventArg> Default for RhaiScriptingPlugin<A> {
    fn default() -> Self {
        RhaiScriptingPlugin {
            scripting_plugin: ScriptingPlugin {
                runtime_builder: RhaiRuntime::new,
                runtime_settings: None,
                callback_handler: Some(rhai_callback_handler::<A>),
                context_assigner: None,
                context_builder: Some(ContextBuilder {
                    load: rhai_context_load,
                    reload: rhai_context_reload,
                }),
            },
        }
    }
}

impl<A: RhaiEventArg> Plugin for RhaiScriptingPlugin<A> {
    fn build(&self, app: &mut bevy::prelude::App) {
        self.scripting_plugin.build(app);
    }
}

pub fn rhai_context_load(
    script: &ScriptId,
    content: &[u8],
    initializers: &[ContextInitializer<RhaiScriptContext>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<RhaiScriptContext>],
    world: &mut World,
    runtime: &mut RhaiRuntime,
) -> Result<RhaiScriptContext, ScriptError> {
    let mut ast = runtime.compile(std::str::from_utf8(content)?)?;
    ast.set_source(script.to_string());

    let mut context = RhaiScriptContext {
        ast,
        scope: Scope::new(),
    };
    with_world(world, &mut context, |mut context| {
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
    initializers: &[ContextInitializer<RhaiScriptContext>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<RhaiScriptContext>],
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
pub fn rhai_callback_handler<A: RhaiEventArg>(
    args: A,
    entity: Entity,
    script_id: &ScriptId,
    callback: &CallbackLabel,
    context: &mut RhaiScriptContext,
    pre_handling_initializers: &[ContextPreHandlingInitializer<RhaiScriptContext>],
    runtime: &mut RhaiRuntime,
    world: &mut World,
) -> Result<(), ScriptError> {
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
            return Ok(());
        };

        // we want the call to be able to impact the scope
        let options = CallFnOptions::new().rewind_scope(false);
        runtime.call_fn_with_options(
            options,
            &mut context.scope,
            &context.ast,
            callback.as_ref(),
            args,
        )?;
        Ok(())
    })
}

pub fn with_world<F: FnOnce(&mut RhaiScriptContext) -> Result<(), ScriptError>>(
    world: &mut World,
    context: &mut RhaiScriptContext,
    f: F,
) -> Result<(), ScriptError> {
    WorldCallbackAccess::with_callback_access(world, |guard| {
        context.scope.push("world", guard.clone());
        f(context)
    })
}
