pub mod assets;



use crate::{
    script_add_synchronizer, script_event_handler, script_hot_reload_handler,
    script_remove_synchronizer, APIProvider, CachedScriptEventState, ScriptContexts,
    ScriptHost,
};
use anyhow::{anyhow, Result};
use beau_collector::BeauCollector as _;
use bevy::{
    prelude::{
        AddAsset, ExclusiveSystemDescriptorCoercion, IntoExclusiveSystem, Mut, SystemSet, World,
    },
};
use rhai::*;
use std::marker::PhantomData;

pub use {assets::*};


pub struct RhaiScriptHost<A: FuncArgs + Send, API: APIProvider> {
    _ph: PhantomData<A>,
    _ph2: PhantomData<API>,
}

unsafe impl<A: FuncArgs + Send, API: APIProvider> Send for RhaiScriptHost<A, API> {}
unsafe impl<A: FuncArgs + Send, API: APIProvider> Sync for RhaiScriptHost<A, API> {}

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
}

impl<A: FuncArgs + Send + Clone + Sync + 'static, API: APIProvider<Ctx = RhaiContext>> ScriptHost
    for RhaiScriptHost<A, API>
{
    type ScriptContext = RhaiContext;
    type ScriptEvent = RhaiEvent<A>;
    type ScriptAsset = RhaiFile;
    type ScriptAPIProvider = API;

    fn register_with_app(app: &mut bevy::prelude::App, stage: impl bevy::prelude::StageLabel) {
        app.add_event::<Self::ScriptEvent>();
        app.add_asset::<RhaiFile>();
        app.init_asset_loader::<RhaiLoader>();
        app.init_resource::<CachedScriptEventState<Self::ScriptEvent>>();
        app.init_resource::<ScriptContexts<Self>>();

        app.add_system_set_to_stage(
            stage,
            SystemSet::new()
                .with_system(script_add_synchronizer::<Self>)
                .with_system(script_remove_synchronizer::<Self>)
                .with_system(script_hot_reload_handler::<Self>)
                .with_system(script_event_handler::<Self>.exclusive_system().at_end()),
        );
    }

    #[allow(deprecated)]
    fn load_script(path: &[u8], script_name: &str) -> anyhow::Result<Self::ScriptContext> {
        let mut engine = Engine::new();
        let mut scope = Scope::new();
        let ast = engine
            .compile(std::str::from_utf8(path)?)
            .map_err(|e| anyhow!("Error in script {}:\n{}", script_name, e.to_string()))?;

        // prevent shadowing of `state`,`world` and `entity` in variable in scripts
        engine.on_def_var(|_, info, _| {
            Ok(info.name != "state" && info.name != "world" && info.name != "entity")
        });

        // persistent state for scripts
        scope.push("state", Map::new());

        Ok(RhaiContext { engine, ast, scope })
    }

    fn handle_events(
        world: &mut bevy::prelude::World,
        events: &[Self::ScriptEvent],
    ) -> anyhow::Result<()> {
        // we need to do this since scripts need access to the world, but they also
        // live in it, hence we only store indices into a resource which can then be scoped
        // instead of storing contexts directly on the components
        world.resource_scope(|world, mut res: Mut<ScriptContexts<Self>>| {
            res.context_entities
                .values_mut()
                .flat_map(|(entity, ctx)| {
                    ctx.scope.set_value("world", world as *mut World as usize);
                    ctx.scope.set_value("entity", *entity);

                    events.iter().map(|event| {
                        ctx.engine
                            .call_fn(
                                &mut ctx.scope,
                                &ctx.ast,
                                &event.hook_name,
                                event.args.clone(),
                            )
                            .map_err(|e| anyhow!("{:?}", *e))
                    })
                })
                .bcollect()
        })
    }
}
