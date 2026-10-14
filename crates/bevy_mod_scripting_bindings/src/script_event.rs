//! Events defined by scripts.
//!
//! All script events are one Rust event type, [`ScriptEvent`], told apart by name.
//! This keeps script events entirely safe code, at the cost of every script event observer
//! being visited when any script event is triggered (a string comparison each).

use std::sync::Arc;

use bevy_ecs::event::Event;
use bevy_reflect::Reflect;

use crate::script_value::ScriptValue;

/// The event triggered for every script event, see [`ScriptEventRegistration`].
#[derive(Event, Clone)]
pub struct ScriptEvent {
    /// The name the event was registered under
    pub name: Arc<str>,
    /// The data the event was triggered with
    pub payload: ScriptValue,
}

/// A handle to a script registered event type.
#[derive(Clone, Reflect, Debug)]
#[reflect(opaque)]
pub struct ScriptEventRegistration {
    /// The name the event was registered under
    pub name: Arc<str>,
}

impl ScriptEventRegistration {
    /// The script namespace the event was registered in, i.e. `script.game` for `script.game.Hit`.
    pub fn namespace(&self) -> &str {
        // names cannot contain dots, so the namespace is everything before the last one
        self.name
            .rsplit_once('.')
            .map_or("", |(namespace, _)| namespace)
    }

    /// The name of the event within its namespace, i.e. `Hit` for `script.game.Hit`.
    pub fn short_name(&self) -> &str {
        self.name
            .rsplit_once('.')
            .map_or(&self.name, |(_, name)| name)
    }
}
