use std::ops::Deref;

use bevy_mod_scripting_bindings::ScriptValue;

use crate::{
    commands::RunScriptCallback,
    event::{IntoCallbackLabel, OnScriptLoaded, OnScriptReloaded, OnScriptUnloaded},
};

use super::*;

const UNLOADED_SCRIPT_STATE_KEY: &str = "state";

/// System which plugs into the loading pipeline and runs the core [`OnScriptLoaded`] script callback after script contexts have finished being loaded or reloaded.
///
/// An error in this callback does not interrupt loading
pub fn run_on_script_loaded_hooks<P: IntoScriptPluginParams>(
    world: &mut World,
    machine_state: &mut SystemState<StateMachine<Machine<Loading, ContextAssigned<P>>, P>>,
    error_state: &mut SystemState<EventWriter<ScriptErrorEvent>>,
) {
    let spliced_machines = machine_state.get_mut(world).iter_cloned();

    let world_id = world.id();
    let emit_responses = P::readonly_configuration(world_id).emit_responses;
    let guard = WorldGuard::new_exclusive(world);

    let (_, failures): (Vec<_>, Vec<_>) = spliced_machines
        .into_iter()
        .map(|machine| {
            RunScriptCallback::<P>::new(
                machine.attachment.clone(),
                OnScriptLoaded::into_callback_label(),
                vec![],
                emit_responses,
            )
            .run_with_context(guard.clone(), machine.state.context.clone())
            .map_err(ScriptErrorEvent::new)
        })
        .partition_result();
    drop(guard);

    if !failures.is_empty() {
        let mut errors = error_state.get_mut(world);
        errors.write_batch(failures);
    }
}

/// System which plugs into the loading pipeline and runs the core [`OnScriptUnloaded`] script callback before script contexts have been reloaded.
///
/// An error in this callback does not interrupt loading
pub fn run_on_script_unloaded_hooks<P: IntoScriptPluginParams>(
    world: &mut World,
    machine_state: &mut SystemState<(
        StateMachine<Machine<Loading, ReloadingInitialized<P>>, P>,
        StateMachine<Machine<Unloading, UnloadingInitialized<P>>, P>,
    )>,
    error_state: &mut SystemState<EventWriter<ScriptErrorEvent>>,
) {
    let (mut reloading_machines, mut unloading_machines) = machine_state.get_mut(world);

    let reloading_machines = reloading_machines.drain().collect::<Vec<_>>();
    let unloading_machines = unloading_machines.drain().collect::<Vec<_>>();

    let world_id = world.id();
    let emit_responses = P::readonly_configuration(world_id).emit_responses;
    let guard = WorldGuard::new_exclusive(world);

    let mut failures = vec![];

    let reloads_to_send = reloading_machines
        .into_iter()
        .map(|machine| {
            let v = match RunScriptCallback::<P>::new(
                machine.attachment.clone(),
                OnScriptUnloaded::into_callback_label(),
                vec![],
                emit_responses,
            )
            .run_with_context(guard.clone(), machine.state.existing_context.clone())
            {
                Ok(v) => v,
                Err(e) => {
                    failures.push(ScriptErrorEvent::new(e));
                    ScriptValue::Unit
                }
            };
            machine.with_blackboard_insert(UNLOADED_SCRIPT_STATE_KEY, v)
        })
        .collect::<Vec<_>>();

    let unloads_to_send = unloading_machines
        .into_iter()
        .map(|machine| {
            let v = match RunScriptCallback::<P>::new(
                machine.attachment.clone(),
                OnScriptUnloaded::into_callback_label(),
                vec![],
                emit_responses,
            )
            .run_with_context(guard.clone(), machine.state.existing_context.clone())
            {
                Ok(v) => v,
                Err(e) => {
                    failures.push(ScriptErrorEvent::new(e));
                    ScriptValue::Unit
                }
            };
            machine.with_blackboard_insert(UNLOADED_SCRIPT_STATE_KEY, v)
        })
        .collect::<Vec<_>>();
    drop(guard);

    if !reloads_to_send.is_empty() || !unloads_to_send.is_empty() {
        let (mut forwarded_reload_machines, mut forwarded_unload_machines) =
            machine_state.get_mut(world);
        forwarded_reload_machines.write_batch(reloads_to_send);
        forwarded_unload_machines.write_batch(unloads_to_send);
    }

    if !failures.is_empty() {
        let mut errors = error_state.get_mut(world);
        errors.write_batch(failures);
    }
}

/// System which plugs into the loading pipeline and runs the core [`OnScriptReloaded`] script callback after script contexts have been reloaded
///
/// An error in this callback does not interrupt loading
pub fn run_on_script_reloaded_hooks<P: IntoScriptPluginParams>(
    world: &mut World,
    machine_state: &mut SystemState<StateMachine<Machine<Loading, ContextAssigned<P>>, P>>,
    error_state: &mut SystemState<EventWriter<ScriptErrorEvent>>,
) {
    let spliced_machines = machine_state.get_mut(world).iter_cloned();

    let world_id = world.id();
    let emit_responses = P::readonly_configuration(world_id).emit_responses;
    let guard = WorldGuard::new_exclusive(world);

    let (_, failures): (Vec<_>, Vec<_>) = spliced_machines
        .into_iter()
        .map(|machine| {
            if machine.state.is_new_context {
                return Ok(ScriptValue::Unit);
            }

            let unload_state = machine.get_blackboard_key::<ScriptValue>(UNLOADED_SCRIPT_STATE_KEY);
            let unload_state = unload_state
                .map(|v| v.deref().clone())
                .unwrap_or(ScriptValue::Unit);

            RunScriptCallback::<P>::new(
                machine.attachment.clone(),
                OnScriptReloaded::into_callback_label(),
                vec![unload_state],
                emit_responses,
            )
            .run_with_context(guard.clone(), machine.state.context.clone())
            .map_err(ScriptErrorEvent::new)
        })
        .partition_result();
    drop(guard);

    if !failures.is_empty() {
        let mut errors = error_state.get_mut(world);
        errors.write_batch(failures);
    }
}
