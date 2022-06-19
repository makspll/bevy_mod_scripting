pub mod assets;

use crate::{
    script_add_synchronizer, script_hot_reload_handler, script_remove_synchronizer, APIProvider,
    CachedScriptEventState, FlatScriptData, Recipients, ScriptContexts, ScriptError,
    ScriptErrorEvent, ScriptEvent, ScriptHost, APIProviders, ScriptCollection, Script,
};
use bevy::prelude::{error, AddAsset, Mut, ParallelSystemDescriptorCoercion, SystemSet, World};
use bevy_event_priority::AddPriorityEvent;
use rhai::*;
use std::marker::PhantomData;

pub use assets::*;

/// More specific APIProvider implementation allowing more control over Rhai scripts
pub trait RhaiAPIProvider: APIProvider {
    /// set Rhai engine settings before scripts are compiled
    fn setup_engine(engine: &mut Engine);
}

pub struct RhaiScriptHost<A: FuncArgs + Send> {
    _ph: PhantomData<A>,
}

unsafe impl<A: FuncArgs + Send> Send for RhaiScriptHost<A> {}
unsafe impl<A: FuncArgs + Send> Sync for RhaiScriptHost<A> {}

pub struct RhaiContext {
    pub engine: Engine,
    pub ast: AST,
    pub scope: Scope<'static>,
}

#[derive(Clone)]
/// A Rhai Hook. The result of creating this event will be
/// a call to the lua script with the hook_name and the given arguments
pub struct RhaiEvent<A: FuncArgs + Clone + 'static> {
    pub hook_name: String,
    pub args: A,
    pub recipients: Recipients,
}

impl<A: FuncArgs + Clone + Send + Sync + 'static> ScriptEvent for RhaiEvent<A> {
    fn recipients(&self) -> &crate::Recipients {
        &self.recipients
    }
}

impl<A: FuncArgs + Send + Clone + Sync + 'static>
    ScriptHost for RhaiScriptHost<A>
{
    type ScriptContext = RhaiContext;
    type ScriptEvent = RhaiEvent<A>;
    type ScriptAsset = RhaiFile;

    fn register_with_app(app: &mut bevy::prelude::App, stage: impl bevy::prelude::StageLabel) {
        app.add_priority_event::<Self::ScriptEvent>()
            .add_asset::<RhaiFile>()
            .init_asset_loader::<RhaiLoader>()
            .init_resource::<CachedScriptEventState<Self>>()
            .init_resource::<ScriptContexts<Self::ScriptContext>>()
            .init_resource::<APIProviders<Self::ScriptContext>>()
            .register_type::<ScriptCollection<Self::ScriptAsset>>()
            .register_type::<Script<Self::ScriptAsset>>()
            .add_system_set_to_stage(
                stage,
                SystemSet::new()
                    .with_system(
                        script_add_synchronizer::<Self>.before(script_remove_synchronizer::<Self>),
                    )
                    .with_system(
                        script_remove_synchronizer::<Self>.before(script_hot_reload_handler::<Self>),
                    )
                    .with_system(script_hot_reload_handler::<Self>),
            );
    }

    #[allow(deprecated)]
    fn load_script(path: &[u8], script_name: &str, providers: &APIProviders<Self::ScriptContext>) -> Result<Self::ScriptContext, ScriptError> {
        let mut engine = Engine::new();

        // TODO: fix this API::setup_engine(&mut engine);

        let mut scope = Scope::new();
        let ast = engine
            .compile(
                std::str::from_utf8(path).map_err(|_| ScriptError::FailedToLoad {
                    script: script_name.to_owned(),
                })?,
            )
            .map_err(|e| ScriptError::SyntaxError {
                script: script_name.to_owned(),
                msg: e.to_string(),
            })?;

        // prevent shadowing of `state`,`world` and `entity` in variable in scripts
        engine.on_def_var(|_, info, _| {
            Ok(info.name != "state" && info.name != "world" && info.name != "entity")
        });

        // persistent state for scripts
        scope.push("state", Map::new());

        let mut ctx = RhaiContext { engine, ast, scope };

        providers.attach_all(&mut ctx)?;

        Ok(ctx)
    }

    fn handle_events<'a>(
        world: &mut World,
        events: &[Self::ScriptEvent],
        ctxs: impl Iterator<Item = (FlatScriptData<'a>, &'a mut Self::ScriptContext)>,
    ) {
        let world_ptr = world as *mut World as usize;
        world.resource_scope(
            |world, mut cached_state: Mut<CachedScriptEventState<Self>>| {
                let (_, mut error_wrt) = cached_state.event_state.get_mut(world);

                ctxs.for_each(|(fd, ctx)| {
                    ctx.scope.set_value("world", world_ptr);
                    ctx.scope.set_value("entity", fd.entity);
                    ctx.scope.set_value("script", fd.sid);

                    for event in events.iter() {
                        // check if this script should handle this event
                        if !event.recipients().is_recipient(&fd) {
                            return;
                        };

                        match ctx.engine.call_fn(
                            &mut ctx.scope,
                            &ctx.ast,
                            &event.hook_name,
                            event.args.clone(),
                        ) {
                            Ok(v) => v,
                            Err(e) => {
                                let err = ScriptError::RuntimeError {
                                    script: fd.name.to_string(),
                                    msg: e.to_string(),
                                };
                                error!("{}", err);
                                error_wrt.send(ScriptErrorEvent { err });
                            }
                        };
                    }
                })
            },
        );
    }
}
