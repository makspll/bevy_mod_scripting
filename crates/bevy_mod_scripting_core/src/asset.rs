//! Systems and resources for handling script assets and events

use std::collections::VecDeque;

use crate::{
    IntoScriptPluginParams, ScriptComponent, ScriptingSystemSet,
    commands::{CreateOrUpdateScript, DeleteScript},
    event::ScriptEvent,
    script::{ContextKey, DisplayProxy, ScriptAttachment, ScriptContext},
};
use ::{
    bevy_app::{App, Last},
    bevy_asset::{AssetEvent, Assets, LoadState},
    bevy_log::{error, trace},
};
use bevy_asset::{AssetServer, Handle};
use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    schedule::IntoScheduleConfigs,
    system::{Commands, Local, Query, Res},
    world::WorldId,
};
use bevy_mod_scripting_asset::ScriptAsset;

/// The queue that evaluates scripts.
type ScriptQueue = VecDeque<ScriptAttachment>;

fn sync_assets(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    mut script_events: EventWriter<ScriptEvent>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Modified { id } => {
                script_events.write(ScriptEvent::Modified { script: *id });
            }
            AssetEvent::Added { id } => {
                script_events.write(ScriptEvent::Added { script: *id });
            }
            AssetEvent::Removed { id } => {
                script_events.write(ScriptEvent::Removed { script: *id });
            }
            _ => (),
        }
    }
}

/// Listens to [`ScriptEvent`] events and dispatches [`CreateOrUpdateScript`] and [`DeleteScript`] commands accordingly.
///
/// Allows for hot-reloading of scripts.
#[profiling::function]
fn handle_script_events<P: IntoScriptPluginParams>(
    mut events: EventReader<ScriptEvent>,
    script_assets: Res<Assets<ScriptAsset>>,
    scripts: Query<(Entity, &ScriptComponent)>,
    asset_server: Res<AssetServer>,
    mut script_queue: Local<ScriptQueue>,
    script_contexts: Res<ScriptContext<P>>,
    mut commands: Commands,
    world_id: WorldId,
) {
    for event in events.read() {
        trace!("{}: Received script event: {:?}", P::LANGUAGE, event);
        match event {
            ScriptEvent::Modified { script: id } => {
                if let Some(asset) = script_assets.get(*id) {
                    if asset.language != P::LANGUAGE {
                        continue;
                    }
                    // We need to reload the script for any context it's
                    // associated with. That could be static scripts, script
                    // components.

                    for (entity, script_component) in &scripts {
                        if let Some(handle) =
                            script_component.0.iter().find(|handle| handle.id() == *id)
                        {
                            commands.queue(
                                CreateOrUpdateScript::<P>::new(ScriptAttachment::EntityScript(
                                    entity,
                                    handle.clone(),
                                ))
                                .with_responses(P::readonly_configuration(world_id).emit_responses),
                            );
                        }
                    }

                    let handle = Handle::Weak(*id);
                    let attachment = ScriptAttachment::StaticScript(handle.clone());
                    let script_contexts = script_contexts.read();
                    for (resident, _) in script_contexts
                        .residents(&attachment)
                        .filter(|(r, _)| r.script() == handle && r.is_static())
                    {
                        // if the script does not have any associated entity it's static.
                        commands
                            .queue(CreateOrUpdateScript::<P>::new(resident).with_responses(
                                P::readonly_configuration(world_id).emit_responses,
                            ));
                    }
                }
            }
            ScriptEvent::Detached { key } => {
                commands.queue(
                    DeleteScript::<P>::new(key.clone())
                        .with_responses(P::readonly_configuration(world_id).emit_responses),
                );
            }
            ScriptEvent::Attached { key } => {
                script_queue.push_back(key.clone());
            }
            _ => (),
        }
    }

    // Evalute the scripts in the order they were attached.
    //
    // If a script is not loaded yet, we stop evaluation and try again on the
    // next call.
    while !script_queue.is_empty() {
        let mut script_failed = false;
        // NOTE: Maybe using pop_front_if once stabalized.
        let script_ready = script_queue
            .front()
            .map(|context_key| context_key.clone().into())
            .map(|context_key: ContextKey| {
                // If there is a script, wait for the script to load.
                context_key
                    .script
                    .as_ref()
                    .map(|script| {
                        script_assets.contains(script.id())
                            || match asset_server.load_state(script) {
                                LoadState::NotLoaded => false,
                                LoadState::Loading => false,
                                LoadState::Loaded => true,
                                LoadState::Failed(e) => {
                                    script_failed = true;
                                    error!(
                                        "Failed to load script {} for eval: {e}.",
                                        script.display()
                                    );
                                    true
                                }
                            }
                    })
                    .unwrap_or(true)
            })
            .unwrap_or(false);
        if !script_ready {
            // We can't evaluate it yet. It's still loading.
            break;
        }

        if let Some(context_key) = script_queue.pop_front() {
            if script_failed {
                continue;
            }

            let language = script_assets
                .get(&context_key.script())
                .map(|asset| asset.language.clone())
                .unwrap_or_default();

            if language == P::LANGUAGE {
                commands.queue(
                    CreateOrUpdateScript::<P>::new(context_key)
                        .with_responses(P::readonly_configuration(world_id).emit_responses),
                );
            }
        }
    }
}

/// Setup all the asset systems for the scripting plugin and the dependencies
#[profiling::function]
pub(crate) fn configure_asset_systems(app: &mut App) {
    // these should be in the same set as bevy's asset systems
    // currently this is in the PreUpdate set
    app.add_systems(
        Last,
        (sync_assets).in_set(ScriptingSystemSet::ScriptAssetDispatch),
    )
    .configure_sets(
        Last,
        (
            ScriptingSystemSet::ScriptAssetDispatch.after(bevy_asset::AssetEvents),
            ScriptingSystemSet::ScriptCommandDispatch
                .after(ScriptingSystemSet::ScriptAssetDispatch),
        ),
    );
}

/// Setup all the asset systems for the scripting plugin and the dependencies
#[profiling::function]
pub(crate) fn configure_asset_systems_for_plugin<P: IntoScriptPluginParams>(app: &mut App) {
    app.add_systems(
        Last,
        handle_script_events::<P>.in_set(ScriptingSystemSet::ScriptCommandDispatch),
    );
}
