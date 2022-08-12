use crate::{
    assets::{RhaiFile, RhaiLoader},
    docs::RhaiDocFragment,
};
use bevy::prelude::*;
use bevy_mod_scripting_core::{prelude::*, systems::*, world::WorldPointer};
use rhai::*;
use std::marker::PhantomData;

pub mod assets;
pub mod docs;
pub use rhai;
pub mod prelude {
    pub use crate::{
        assets::{RhaiFile, RhaiLoader},
        docs::RhaiDocFragment,
        RhaiContext, RhaiEvent, RhaiScriptHost,
    };
    pub use rhai;
}

pub struct RhaiScriptHost<A: FuncArgs + Send> {
    pub engine: Engine,
    _ph: PhantomData<A>,
}

#[allow(deprecated)]
impl<A: FuncArgs + Send> Default for RhaiScriptHost<A> {
    fn default() -> Self {
        let mut e = Engine::new();
        // prevent shadowing of `state`,`world` and `entity` in variable in scripts
        e.on_def_var(|_, info, _| {
            Ok(info.name != "state" && info.name != "world" && info.name != "entity")
        });

        Self {
            engine: e,
            _ph: Default::default(),
        }
    }
}

pub struct RhaiContext {
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

impl<A: FuncArgs + Send + Clone + Sync + 'static> ScriptHost for RhaiScriptHost<A> {
    type ScriptContext = RhaiContext;
    type ScriptEvent = RhaiEvent<A>;
    type ScriptAsset = RhaiFile;
    type APITarget = Engine;
    type DocTarget = RhaiDocFragment;

    fn register_with_app(app: &mut bevy::prelude::App, stage: impl bevy::prelude::StageLabel) {
        app.add_priority_event::<Self::ScriptEvent>()
            .add_asset::<RhaiFile>()
            .init_asset_loader::<RhaiLoader>()
            .init_resource::<CachedScriptState<Self>>()
            .init_resource::<ScriptContexts<Self::ScriptContext>>()
            .init_resource::<APIProviders<Self>>()
            .register_type::<ScriptCollection<Self::ScriptAsset>>()
            .register_type::<Script<Self::ScriptAsset>>()
            .add_system_set_to_stage(
                stage,
                SystemSet::new()
                    .with_system(
                        script_add_synchronizer::<Self>.before(script_remove_synchronizer::<Self>),
                    )
                    .with_system(
                        script_remove_synchronizer::<Self>
                            .before(script_hot_reload_handler::<Self>),
                    )
                    .with_system(script_hot_reload_handler::<Self>)
                    .with_system(
                        script_setup_handler::<Self>
                            .exclusive_system()
                            .at_end()
                            .before(ScriptSystemLabel::EventHandling),
                    ),
            )
            // setup engine
            .add_startup_system(
                |mut providers: ResMut<APIProviders<Self>>, mut host: ResMut<Self>| {
                    providers
                        .attach_all(&mut host.engine)
                        .expect("Error in adding api's for rhai");
                },
            );
    }

    fn setup_script(
        &mut self,
        world_ptr: WorldPointer,
        script_data: &ScriptData,
        ctx: &mut Self::ScriptContext,
        providers: &mut APIProviders<Self>,
    ) -> Result<(), ScriptError> {
        providers.setup_all(world_ptr, script_data, ctx)
    }

    fn load_script(
        &mut self,
        path: &[u8],
        script_data: &ScriptData,
        _: &mut APIProviders<Self>,
    ) -> Result<Self::ScriptContext, ScriptError> {
        let mut scope = Scope::new();
        let mut ast = self
            .engine
            .compile(
                std::str::from_utf8(path).map_err(|_| ScriptError::FailedToLoad {
                    script: script_data.name.to_owned(),
                })?,
            )
            .map_err(|e| ScriptError::SyntaxError {
                script: script_data.name.to_owned(),
                msg: e.to_string(),
            })?;

        ast.set_source(script_data.name);

        // persistent state for scripts
        scope.push("state", Map::new());

        Ok(RhaiContext { ast, scope })
    }

    fn handle_events<'a>(
        &self,
        world_ptr: WorldPointer,
        events: &[Self::ScriptEvent],
        ctxs: impl Iterator<Item = (ScriptData<'a>, &'a mut Self::ScriptContext)>,
    ) {
        let mut world = world_ptr.write();
        let mut state: CachedScriptState<Self> = world.remove_resource().unwrap();

        // this is important, the scripts might have access to the world pointer
        // not unlocking this would prevent them from accessing the world
        drop(world);

        ctxs.for_each(|(fd, ctx)| {
            for event in events.iter() {
                // check if this script should handle this event
                if !event.recipients().is_recipient(&fd) {
                    return;
                };

                match self.engine.call_fn(
                    &mut ctx.scope,
                    &ctx.ast,
                    &event.hook_name,
                    event.args.clone(),
                ) {
                    Ok(v) => v,
                    Err(e) => {
                        let mut world = world_ptr.write();
                        let (_, mut error_wrt, _) = state.event_state.get_mut(&mut world);

                        let err = ScriptError::RuntimeError {
                            script: fd.name.to_string(),
                            msg: e.to_string(),
                        };
                        error!("{}", err);
                        error_wrt.send(ScriptErrorEvent { error: err });
                    }
                };
            }
        });

        let mut world = world_ptr.write();
        world.insert_resource(state);
    }
}
