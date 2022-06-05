pub mod assets;

use crate::{
    script_add_synchronizer, script_hot_reload_handler, script_remove_synchronizer, APIProvider,
    CachedScriptEventState, FlatScriptData, Recipients, ScriptContexts, ScriptError,
    ScriptErrorEvent, ScriptEvent, ScriptHost,
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

pub struct RhaiScriptHost<A: FuncArgs + Send, API: APIProvider> {
    _ph: PhantomData<A>,
    _ph2: PhantomData<API>,
}


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

#[derive(Default)]
pub struct DummyRhaiAPI;
impl APIProvider for DummyRhaiAPI {
    type Ctx = RhaiContext;

    fn attach_api(ctx: &mut Self::Ctx) {
        todo!()
    }
    
}


impl<A: FuncArgs + Send + Clone + Sync + 'static, API: RhaiAPIProvider<Ctx = RhaiContext>>
    ScriptHost for RhaiScriptHost<A, API>
{
    type ScriptContext = RhaiContext;
    type ScriptEvent = RhaiEvent<A>;
    type ScriptAsset = RhaiFile;
    type BevyAPI = DummyRhaiAPI;

    fn register_with_app(app: &mut bevy::prelude::App, stage: impl bevy::prelude::StageLabel) {
        app.add_priority_event::<Self::ScriptEvent>();
        app.add_asset::<RhaiFile>();
        app.init_asset_loader::<RhaiLoader>();
        app.init_resource::<CachedScriptEventState<Self>>();
        app.init_resource::<ScriptContexts<Self::ScriptContext>>();

        app.add_system_set_to_stage(
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
    fn load_script(path: &[u8], script_name: &str) -> Result<Self::ScriptContext, ScriptError> {
        let mut engine = Engine::new();

        API::setup_engine(&mut engine);

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

        API::attach_api(&mut ctx);

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
