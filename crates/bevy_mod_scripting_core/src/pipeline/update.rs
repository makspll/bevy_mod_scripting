use super::*;

/// Progresses loading state machines from [`LoadingInitializedState`] to [`ContextAssigned`]
pub fn assign_contexts_for_new_scripts<P: IntoScriptPluginParams>(
    world: &mut World,
    early_state: &mut SystemState<(
        Res<Assets<ScriptAsset>>,
        StateMachine<Machine<Loading, LoadingInitialized>, P>,
    )>,
    next_state: &mut SystemState<StateMachine<Machine<Loading, ContextAssigned<P>>, P>>,
    error_state: &mut SystemState<EventWriter<ScriptErrorEvent>>,
) {
    let (assets, mut machines) = early_state.get_mut(world);
    let machines = machines
        .drain()
        .map(|machine| (machine.state.source.get(&assets), machine))
        .collect::<Vec<_>>();

    let guard = WorldGuard::new_exclusive(world);

    let (successes, failures): (Vec<_>, Vec<_>) = machines
        .into_iter()
        .map(|(asset, machine)| {
            let attachment = &machine.attachment;
            let ctxt = P::load(attachment, &asset.content, guard.clone());
            ctxt.map(|ctxt| machine.assign_new_context::<P>(Arc::new(Mutex::new(ctxt))))
                .map_err(|e| ScriptErrorEvent {
                    error: ScriptError::from(e),
                })
        })
        .partition_result();

    if !successes.is_empty() {
        let mut writer = next_state.get_mut(world);
        writer.write_batch(successes);
    }

    if !failures.is_empty() {
        let mut writer = error_state.get_mut(world);
        writer.write_batch(failures);
    }
}

/// Progresses loading state machines from [`ReloadingInitializedState`] to [`ContextAssigned`]
pub fn reload_existing_contexts<P: IntoScriptPluginParams>(
    world: &mut World,
    early_state: &mut SystemState<(
        Res<Assets<ScriptAsset>>,
        StateMachine<Machine<Loading, ReloadingInitialized<P>>, P>,
    )>,
    next_state: &mut SystemState<StateMachine<Machine<Loading, ContextAssigned<P>>, P>>,
    error_state: &mut SystemState<EventWriter<ScriptErrorEvent>>,
) {
    let (assets, mut machines) = early_state.get_mut(world);
    let machines = machines
        .drain()
        .map(|machine| (machine.state.source.get(&assets), machine))
        .collect::<Vec<_>>();

    let guard = WorldGuard::new_exclusive(world);

    let (successes, failures): (Vec<_>, Vec<_>) = machines
        .into_iter()
        .map(|(asset, machine)| {
            let attachment = &machine.attachment;
            let previous_context = &machine.state.existing_context;
            let mut previous_context_guard = previous_context.lock();
            let ctxt = P::reload(
                attachment,
                &asset.content,
                &mut previous_context_guard,
                guard.clone(),
            );

            drop(previous_context_guard);

            ctxt.map(|_| machine.assign_reloaded_context())
                .map_err(|e| ScriptErrorEvent {
                    error: ScriptError::from(e),
                })
        })
        .partition_result();

    if !successes.is_empty() {
        let mut writer = next_state.get_mut(world);
        writer.write_batch(successes);
    }

    if !failures.is_empty() {
        let mut writer = error_state.get_mut(world);
        writer.write_batch(failures);
    }
}

/// Processes [`UnloadingInitialized`] states into [`ContextRemoved`] and [`ResidentRemoved`] states
pub fn remove_residents_or_remove_contexts<P: IntoScriptPluginParams>(
    mut machines: StateMachine<Machine<Unloading, UnloadingInitialized<P>>, P>,
    mut context_removed_machines: StateMachine<Machine<Unloading, ContextRemoved<P>>, P>,
    mut resident_removed_machines: StateMachine<Machine<Unloading, ResidentRemoved<P>>, P>,
    contexts: Res<ScriptContext<P>>,
) {
    let machines = machines.drain();
    let mut contexts_guard = contexts.write();

    let (context_removed, resident_removed): (Vec<_>, Vec<_>) = machines
        .map(|machine| {
            let residents_len = contexts_guard.residents_len(&machine.attachment);
            if residents_len == 1 {
                contexts_guard.remove(&machine.attachment);
                Ok(machine.remove_context())
            } else {
                contexts_guard.remove_resident(&machine.attachment);
                Err(machine.remove_resident())
            }
        })
        .partition_result();

    context_removed_machines.write_batch(context_removed);
    resident_removed_machines.write_batch(resident_removed);
}

/// Processes [`ContextRemoved`] and [`ResidentRemoved`] states into [`UnloadingCompleted`]
pub fn complete_unloading<P: IntoScriptPluginParams>(
    mut context_removed_machines: StateMachine<Machine<Unloading, ContextRemoved<P>>, P>,
    mut resident_removed_machines: StateMachine<Machine<Unloading, ResidentRemoved<P>>, P>,
    mut completed_machines: StateMachine<Machine<Unloading, UnloadingCompleted>, P>,
) {
    completed_machines.write_batch(
        context_removed_machines
            .drain()
            .map(|m| m.complete_unloading()),
    );
    completed_machines.write_batch(
        resident_removed_machines
            .drain()
            .map(|m| m.complete_unloading()),
    );
}
