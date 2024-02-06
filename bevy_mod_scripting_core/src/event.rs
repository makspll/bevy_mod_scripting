use bevy::log::error;
use bevy::prelude::Event;

use crate::hosts::ScriptHost;
use crate::systems::CachedScriptState;
use crate::world::WorldPointer;
use crate::{error::ScriptError, hosts::Recipients};

/// An error coming from a script
#[derive(Debug, Event)]
pub struct ScriptErrorEvent {
    pub error: ScriptError,
}

/// An event emitted when a script was loaded or re-loaded (with a hot-reload),
/// guaranteed to be sent for every script at least once and immediately after it's loaded.
#[derive(Clone, Debug, Event)]
pub struct ScriptLoaded {
    pub sid: u32,
}

/// A trait for events to be handled by scripts
pub trait ScriptEvent: Send + Sync + Clone + Event + 'static {
    /// Retrieves the recipient scripts for this event
    fn recipients(&self) -> &Recipients;
}

pub fn write_error_event_with_world<H: ScriptHost>(
    world: WorldPointer,
    script_name: String,
    error_text: String,
) {
    let mut world = world.write();
    let mut state: CachedScriptState<H> = world.remove_resource().unwrap();

    let (_, mut error_wrt, _) = state.event_state.get_mut(&mut world);

    let error = ScriptError::RuntimeError {
        script: script_name,
        msg: error_text,
    };

    error!("{}", error);
    error_wrt.send(ScriptErrorEvent { error });
    world.insert_resource(state);
}
