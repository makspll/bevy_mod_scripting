use super::*;
use crate::IntoScriptPluginParams;
use bevy::prelude::{default, Entity};
use bevy::{ecs::system::Resource};
use parking_lot::Mutex;
use std::{borrow::Cow, sync::Arc};

/// The key for a context.
#[derive(Debug, Hash, Clone, Default, PartialEq, Eq)]
pub struct ContextKey {
    /// Entity if there is one.
    pub entity: Option<Entity>,
    /// Script ID if there is one.
    pub script_id: Option<ScriptId>,
    /// Domain if there is one.
    pub domain: Option<Domain>,
}

impl ContextKey {
    /// Is the key empty?
    pub fn is_empty(&self) -> bool {
        self.entity.is_none() && self.script_id.is_none() && self.domain.is_none()
    }

    /// or with other context
    pub fn or(self, other: ContextKey) -> Self {
        Self {
            entity: self.entity.or(other.entity),
            script_id: self.script_id.or(other.script_id),
            domain: self.domain.or(other.domain),
        }
    }

    /// Returns true if self is a subset of other.
    ///
    /// Subset meaning if `self.entity` is `Some`, then other must be `Some` and
    /// equal.
    ///
    /// If `self.entity` is `None`, then other.entity can be anything.
    ///
    /// An empty [ContextKey] is a subset of any context key.
    pub fn is_subset(&self, other: &ContextKey) -> bool {
        self.entity.map(|a| other.entity.map(|b| a == b).unwrap_or(false)).unwrap_or(true)
            || self.script_id.map(|a| other.script_id.map(|b| a == b).unwrap_or(false)).unwrap_or(true)
            || self.domain.as_ref().map(|a| other.domain.as_ref().map(|b| a == b).unwrap_or(false)).unwrap_or(true)
    }
}

impl From<Entity> for ContextKey {
    fn from(entity: Entity) -> Self {
        Self {
            entity: Some(entity),
            ..default()
        }
    }
}

impl From<ScriptId> for ContextKey {
    fn from(script_id: ScriptId) -> Self {
        Self {
            script_id: Some(script_id),
            ..default()
        }
    }
}

impl From<Domain> for ContextKey {
    fn from(domain: Domain) -> Self {
        Self {
            domain: Some(domain),
            ..default()
        }
    }
}
