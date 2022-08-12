use std::borrow::Cow;

use bevy::prelude::Entity;

use crate::{error::ScriptError, hosts::Recipients};

/// An error coming from a script
#[derive(Debug)]
pub struct ScriptErrorEvent {
    pub error: ScriptError,
}

/// An event emitted when a script was loaded or re-loaded (with a hot-reload),
/// guaranteed to be sent for every script at least once and immediately after it's loaded.
#[derive(Clone, Debug)]
pub struct ScriptLoaded {
    pub sid: u32,
}

/// A trait for events to be handled by scripts
pub trait ScriptEvent: Send + Sync + Clone + 'static {
    /// Retrieves the recipient scripts for this event
    fn recipients(&self) -> &Recipients;
}
