use super::*;

/// Inserts residents into contexts, and or the contexts themselves if they aren't already there
pub fn insert_residents<P: IntoScriptPluginParams>(
    machines: StateMachine<Machine<Loading, ContextAssigned<P>>, P>,
    contexts: Res<ScriptContext<P>>,
    mut errors: EventWriter<ScriptErrorEvent>,
) {
    let residents_to_insert = machines.iter_cloned().into_iter().map(|machine| {
        let attachment = machine.attachment;
        let context = machine.state.context;
        (attachment, context)
    });

    let mut contexts_guard = contexts.write();
    let (_, failures): (Vec<_>, Vec<_>) = contexts_guard
        .insert_batch(residents_to_insert)
        .partition_result();

    if !failures.is_empty() {
        let failures = failures
            .into_iter()
            .map(|(attachment, _)| ScriptErrorEvent {
                error: ScriptError::new_boxed_without_type_info(
                    format!("failed to insert context for, or mark attachment as resident: {attachment}. No context policy could be matched. The context will be discarded").into(),
                ),
            });
        errors.write_batch(failures);
    }
}
