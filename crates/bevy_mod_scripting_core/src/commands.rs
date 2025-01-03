use std::{any::type_name, marker::PhantomData};

use bevy::{asset::Handle, ecs::world::Mut, log::debug, prelude::Command};

use crate::{
    asset::ScriptAsset,
    context::{Context, ContextLoadingSettings, ScriptContexts},
    prelude::{Runtime, RuntimeContainer},
    script::{Script, ScriptId, Scripts},
    systems::handle_script_errors,
    IntoScriptPluginParams,
};

pub struct DeleteScript<P: IntoScriptPluginParams> {
    pub id: ScriptId,
    // hack to make this Send, C does not need to be Send since it is not stored in the command
    pub _ph: PhantomData<fn(P::C, P::R)>,
}

impl<P: IntoScriptPluginParams> DeleteScript<P> {
    pub fn new(id: ScriptId) -> Self {
        Self {
            id,
            _ph: PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> Command for DeleteScript<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
        let settings = world
            .get_resource::<ContextLoadingSettings<P>>()
            .expect("No ScriptLoadingSettings resource found")
            .clone();

        world.resource_scope(|world, mut scripts: Mut<Scripts>| {
            if let Some(script) = scripts.scripts.remove(&self.id) {
                debug!("Deleting script with id: {}", self.id);
                let mut ctxts = world.get_non_send_resource_mut::<ScriptContexts<P>>();
                let ctxts = ctxts.as_deref_mut().unwrap();
                let assigner = settings
                    .assigner
                    .as_ref()
                    .expect("Could not find context assigner in settings");
                debug!("Removing script with id: {}", self.id);
                (assigner.remove)(script.context_id, &script, ctxts)
            } else {
                bevy::log::error!(
                    "Attempted to delete script with id: {} but it does not exist, doing nothing!",
                    self.id
                );
            }
        });

        world.insert_resource(settings);
    }
}

/// Creates new script with the given ID, if a script with the given ID already exists, this is treated as an update
///
/// If script comes from an asset, expects it to be loaded, otherwise this command will fail to process the script.
pub struct CreateOrUpdateScript<P: IntoScriptPluginParams> {
    id: ScriptId,
    content: Box<[u8]>,
    asset: Option<Handle<ScriptAsset>>,
    // Hack to make this Send, C does not need to be Send since it is not stored in the command
    _ph: std::marker::PhantomData<fn(P::R, P::C)>,
}

impl<P: IntoScriptPluginParams> CreateOrUpdateScript<P> {
    pub fn new(id: ScriptId, content: Box<[u8]>, asset: Option<Handle<ScriptAsset>>) -> Self {
        Self {
            id,
            content,
            asset,
            _ph: std::marker::PhantomData,
        }
    }
}

impl<P: IntoScriptPluginParams> Command for CreateOrUpdateScript<P> {
    fn apply(self, world: &mut bevy::prelude::World) {
        let settings = world
            .get_resource::<ContextLoadingSettings<P>>()
            .unwrap()
            .clone();
        let mut contexts = world
            .remove_non_send_resource::<ScriptContexts<P>>()
            .unwrap();
        let mut runtime = world
            .remove_non_send_resource::<RuntimeContainer<P>>()
            .unwrap();
        // assign context
        let assigner = settings.assigner.clone().expect("No context assigner set");
        let builder = settings.loader.clone().expect("No context loader set");

        world.resource_scope(|world, mut scripts: Mut<Scripts>| {

            // check if script already exists

            let mut script = scripts.scripts.get_mut(&self.id);
            let previous_context_id = script.as_ref().map(|s| s.context_id);
            debug!(
                "CreateOrUpdateScript command applying with to (script_id: {}, previous_context_id: {:?})",
                self.id, previous_context_id
            );

            // If None assign new context ID, otherwise assign the old one
            // If re-loading and different from the previous one, the old one will be removed
            let current_context_id = (assigner.assign)(script.as_deref(), &self.id, &self.content, &mut contexts);
            debug!("Context assigned: {:?}", current_context_id);

            let current_context_id = if let Some(id) = current_context_id {
                id
            } else {
                let ctxt = (builder.load)(&self.id, &self.content, &settings.context_initializers, &settings.context_pre_handling_initializers, world, &mut runtime.runtime);
                match ctxt {
                    Ok(ctxt) => contexts.insert(ctxt),
                    Err(e) => {
                        handle_script_errors(world, [e.with_context(format!("Loading context for script with id: {}. With runtime type: {} and context type: {}", self.id, type_name::<P::R>(), type_name::<P::C>()))].into_iter());
                        return;
                    }
                }
            };

            if let Some(previous) = previous_context_id {
                if previous != current_context_id {
                    debug!(
                        "Script is being moved to a new context with id: {}, removing up old context.",
                        current_context_id
                    );
                    script.as_deref_mut().unwrap().context_id = current_context_id;
                    (assigner.remove)(previous, script.unwrap(), &mut contexts);
                }
            }


            // now we can insert the actual script
            scripts.scripts.insert(
                self.id.clone(),
                Script {
                    id: self.id,
                    asset: self.asset,
                    context_id: current_context_id,
                },
            );
        });
        world.insert_resource(settings);
        world.insert_non_send_resource(runtime);
        world.insert_non_send_resource(contexts);
    }
}
