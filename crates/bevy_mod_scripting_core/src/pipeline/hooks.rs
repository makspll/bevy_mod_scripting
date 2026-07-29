use bevy_ecs::{
    observer::On,
    system::{Commands, In},
    world::WorldId,
};
use bevy_mod_scripting_script::ScriptAttachment;

use crate::{
    commands::RunScriptCallback,
    event::{IntoCallbackLabel, OnScriptLoaded, OnScriptReloaded, OnScriptUnloaded},
};

use super::*;

pub(crate) fn clear_machine_data(
    attachment: In<ScriptAttachment>,
    mut datas: ResMut<ActiveMachinesData>,
) {
    datas.0.remove(&attachment.0);
}

pub(crate) fn process_machine_failure<P: IntoScriptPluginParams>(
    attachment: In<ScriptAttachment>,
    script_contexts: ResMut<ScriptContexts<P>>,
) {
    let mut script_contexts = script_contexts.write();

    // TODO: handle this
    let _ = script_contexts.mark_active_if_not_loading(&attachment);
}

pub fn on_script_loaded_pipeline_handler<P: IntoScriptPluginParams>(
    trigger: On<ContextAssigned<P>>,
    mut commands: Commands,
    world_id: WorldId,
) {
    let event = trigger.event();
    let emit_responses = P::readonly_configuration(world_id).emit_responses;
    bevy_log::debug!(
        "Running on_script_loaded hook for script: {}",
        event.attachment
    );
    commands.queue(
        RunScriptCallback::<P>::new(
            event.attachment.clone(),
            OnScriptLoaded::into_callback_label(),
            vec![],
            emit_responses,
        )
        .with_context_override(trigger.context.clone()),
    );
}

pub fn on_script_unloaded_for_unload_pipeline_handler<P: IntoScriptPluginParams>(
    trigger: On<UnloadingInitialized<P>>,
    mut commands: Commands,
    world_id: WorldId,
) {
    let event = trigger.event();
    let emit_responses = P::readonly_configuration(world_id).emit_responses;
    // let guard = WorldGuard::new_exclusive(world);
    bevy_log::debug!(
        "Running on_script_unloaded hook for script: {}, due to unload",
        event.attachment
    );
    commands.queue(
        RunScriptCallback::<P>::new(
            event.attachment.clone(),
            OnScriptUnloaded::into_callback_label(),
            vec![],
            emit_responses,
        )
        .with_context_override(trigger.existing_context.clone())
        .with_post_callback_handler(|world, attachment, response| {
            if let Ok(v) = response.as_ref() {
                bevy_log::debug!(
                    "on_script_unloaded hook for script: {} setting reload_state: {:?}",
                    attachment,
                    v
                );
                let mut data = world.get_resource_or_init::<ActiveMachinesData>();
                let data = data.0.entry(attachment).or_default();

                data.reload_state = v.clone();
            }
        }),
    );
}
// todo unify the two, they just need a context extractor trait
pub fn on_script_unloaded_for_reload_pipeline_handler<P: IntoScriptPluginParams>(
    trigger: On<ReloadingInitialized<P>>,
    mut commands: Commands,
    world_id: WorldId,
) {
    let event = trigger.event();
    let emit_responses = P::readonly_configuration(world_id).emit_responses;
    bevy_log::debug!(
        "Running on_script_unloaded hook for script: {}, due to reload",
        event.attachment
    );
    commands.queue(
        RunScriptCallback::<P>::new(
            event.attachment.clone(),
            OnScriptUnloaded::into_callback_label(),
            vec![],
            emit_responses,
        )
        .with_context_override(trigger.existing_context.clone())
        .with_post_callback_handler(|world, attachment, response| {
            if let Ok(v) = response.as_ref() {
                bevy_log::debug!(
                    "on_script_unloaded hook for script: {} setting reload_state: {:?}",
                    attachment,
                    v
                );
                let mut datas = world.get_resource_or_init::<ActiveMachinesData>();
                let data = datas.0.entry(attachment).or_default();

                data.reload_state = v.clone();
                bevy_log::debug!("Datas: {}", datas.0.len());
            }
        }),
    );
}

pub fn on_script_reloaded_pipeline_handler<P: IntoScriptPluginParams>(
    trigger: On<ContextAssigned<P>>,
    mut commands: Commands,
    world_id: WorldId,
    datas: Res<ActiveMachinesData>,
) {
    let event = trigger.event();

    let emit_responses = P::readonly_configuration(world_id).emit_responses;

    if event.is_new_context {
        return;
    }

    let unload_state = datas
        .0
        .get(&event.attachment)
        .map(|e| e.reload_state.clone())
        .unwrap_or_default();

    bevy_log::debug!(
        "Running on_script_reloaded hook for script: {}, with unload_state: {:?}, with data count: {}",
        event.attachment,
        unload_state,
        datas.0.len()
    );

    commands.queue(
        RunScriptCallback::<P>::new(
            event.attachment.clone(),
            OnScriptReloaded::into_callback_label(),
            vec![unload_state],
            emit_responses,
        )
        .with_context_override(trigger.context.clone()),
    )
}
