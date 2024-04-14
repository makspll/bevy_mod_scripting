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
                runtime_builder: Some(RhaiRuntime::new),
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
            .try_for_each(|init| init(script, &mut context))?;

        pre_handling_initializers
            .iter()
            .try_for_each(|init| init(script, Entity::from_raw(0), &mut context))?;

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
// use crate::{
//     assets::{RhaiFile, RhaiLoader},
//     docs::RhaiDocFragment,
// };
// use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
// use bevy_mod_scripting_core::{prelude::*, systems::*};
// use rhai::*;
// use std::marker::PhantomData;

// pub mod assets;
// pub mod docs;
// pub use rhai;
// pub mod prelude {
//     pub use crate::{
//         assets::{RhaiFile, RhaiLoader},
//         docs::RhaiDocFragment,
//         RhaiContext, RhaiEvent, RhaiScriptHost,
//     };
//     pub use rhai;
//     pub use rhai::{RhaiRuntime, FuncArgs};
// }

// #[derive(Resource)]
// pub struct RhaiScriptHost<A: FuncArgs + Send> {
//     pub RhaiRuntime: RhaiRuntime,
//     _ph: PhantomData<A>,
// }

// #[allow(deprecated)]
// impl<A: FuncArgs + Send> Default for RhaiScriptHost<A> {
//     fn default() -> Self {
//         let mut e = RhaiRuntime::new();
//         // prevent shadowing of `state`,`world` and `entity` in variable in scripts
//         e.on_def_var(|_, info, _| {
//             Ok(info.name() != "state" && info.name() != "world" && info.name() != "entity")
//         });

//         Self {
//             RhaiRuntime: e,
//             _ph: Default::default(),
//         }
//     }
// }

// pub struct RhaiContext {
//     pub ast: AST,
//     pub scope: Scope<'static>,
// }

// #[derive(Clone, Event)]
// /// A Rhai Hook. The result of creating this event will be
// /// a call to the lua script with the hook_name and the given arguments
// pub struct RhaiEvent<A: FuncArgs + Clone + 'static> {
//     pub hook_name: String,
//     pub args: A,
//     pub recipients: Recipients,
// }

// impl<A: FuncArgs + Clone + Send + Sync + 'static> ScriptEvent for RhaiEvent<A> {
//     fn recipients(&self) -> &crate::Recipients {
//         &self.recipients
//     }
// }

// impl<A: FuncArgs + Send + Clone + Sync + 'static> ScriptHost for RhaiScriptHost<A> {
//     type ScriptContext = RhaiContext;
//     type ScriptEvent = RhaiEvent<A>;
//     type ScriptAsset = RhaiFile;
//     type APITarget = RhaiRuntime;
//     type DocTarget = RhaiDocFragment;

//     fn register_with_app_in_set(
//         app: &mut bevy::prelude::App,
//         schedule: impl ScheduleLabel,
//         set: impl SystemSet,
//     ) {
//         app.add_priority_event::<Self::ScriptEvent>()
//             .init_asset::<RhaiFile>()
//             .init_asset_loader::<RhaiLoader>()
//             .init_resource::<CachedScriptState<Self>>()
//             .init_resource::<ScriptContexts<Self::ScriptContext>>()
//             .init_resource::<APIProviders<Self>>()
//             .register_type::<ScriptCollection<Self::ScriptAsset>>()
//             .register_type::<Script<Self::ScriptAsset>>()
//             .register_type::<Handle<RhaiFile>>()
//             .add_systems(
//                 schedule,
//                 (
//                     script_add_synchronizer::<Self>,
//                     script_remove_synchronizer::<Self>,
//                     script_hot_reload_handler::<Self>,
//                 )
//                     .chain()
//                     .in_set(set),
//             )
//             // setup RhaiRuntime
//             .add_systems(
//                 Startup,
//                 |mut providers: ResMut<APIProviders<Self>>, mut host: ResMut<Self>| {
//                     providers
//                         .attach_all(&mut host.RhaiRuntime)
//                         .expect("Error in adding api's for rhai");
//                 },
//             );
//     }

//     fn setup_script(
//         &mut self,
//         script_data: &ScriptData,
//         ctx: &mut Self::ScriptContext,
//         providers: &mut APIProviders<Self>,
//     ) -> Result<(), ScriptError> {
//         providers.setup_all(script_data, ctx)
//     }

//     fn load_script(
//         &mut self,
//         script: &[u8],
//         script_data: &ScriptData,
//         _: &mut APIProviders<Self>,
//     ) -> Result<Self::ScriptContext, ScriptError> {
//         let mut scope = Scope::new();
//         let mut ast = self
//             .RhaiRuntime
//             .compile(
//                 std::str::from_utf8(script).map_err(|e| ScriptError::FailedToLoad {
//                     script: script_data.name.to_owned(),
//                     msg: e.to_string(),
//                 })?,
//             )
//             .map_err(|e| ScriptError::SyntaxError {
//                 script: script_data.name.to_owned(),
//                 msg: e.to_string(),
//             })?;

//         ast.set_source(script_data.name);

//         // persistent state for scripts
//         scope.push("state", Map::new());

//         Ok(RhaiContext { ast, scope })
//     }

//     fn handle_events<'a>(
//         &mut self,
//         world: &mut World,
//         events: &[Self::ScriptEvent],
//         ctxs: impl Iterator<Item = (ScriptData<'a>, &'a mut Self::ScriptContext)>,
//         _providers: &mut APIProviders<Self>,
//     ) {
//         ctxs.for_each(|(fd, ctx)| {
//             for event in events.iter() {
//                 // check if this script should handle this event
//                 if !event.recipients().is_recipient(&fd) {
//                     continue;
//                 };

//                 match self.RhaiRuntime.call_fn(
//                     &mut ctx.scope,
//                     &ctx.ast,
//                     &event.hook_name,
//                     event.args.clone(),
//                 ) {
//                     Ok(v) => v,
//                     Err(e) => {
//                         let mut state: CachedScriptState<Self> = world.remove_resource().unwrap();

//                         match *e {
//                             EvalAltResult::ErrorFunctionNotFound(..) => {}
//                             _ => {
//                                 let (_, mut error_wrt, _) = state.event_state.get_mut(world);

//                                 let error = ScriptError::RuntimeError {
//                                     script: fd.name.to_string(),
//                                     msg: e.to_string(),
//                                 };
//                                 error!("{}", error);
//                                 error_wrt.send(ScriptErrorEvent { error });
//                             }
//                         }

//                         world.insert_resource(state);
//                     }
//                 };
//             }

//             // executing this at the end here means we execute global statements exactly once
//             // all this method call does is set a variable on the AST to NONE so should not affect performance
//             ctx.ast.clear_statements();
//         });
//     }
// }
