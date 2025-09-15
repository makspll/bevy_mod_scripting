use super::*;

/// Send completion state machine events for scripts which have now been added to the
/// contexts
pub fn complete_loading<P: IntoScriptPluginParams>(
    mut machines: StateMachine<Machine<Loading, ContextAssigned<P>>, P>,
    mut next_machines: StateMachine<Machine<Loading, LoadingCompleted>, P>,
) {
    next_machines.write_batch(machines.drain().map(Machine::complete_loading))
}
