use super::*;
use crate::ScriptAsset;
use bevy::prelude::{default, Entity};
use std::fmt;

/// The key for a context.
#[derive(Debug, Hash, Clone, Default, PartialEq, Eq)]
pub struct ContextKey {
    /// Entity if there is one.
    pub entity: Option<Entity>,
    /// Script ID if there is one.
    pub script: Option<Handle<ScriptAsset>>,
    /// Domain if there is one.
    pub domain: Option<Domain>,
}

impl fmt::Display for ContextKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "context ")?;
        let mut empty = true;
        if let Some(script_id) = &self.script {
            write!(f, "script {}", script_id.display())?;
            empty = false;
        }
        if let Some(id) = self.entity {
            write!(f, "entity {id}")?;
            empty = false;
        }
        if let Some(domain) = self.domain {
            write!(f, "domain {domain:?}")?;
            empty = false;
        }
        if empty {
            write!(f, "empty")?;
        }
        Ok(())
    }
}

// /// A context key reference.
// #[derive(Debug, PartialEq, Eq, Hash)]
// pub struct ContextKeyRef<'a> {
//     pub entity: Option<Entity>,
//     pub script_id: Option<&'a Handle<ScriptAsset>>,
//     pub domain: Option<&'a Domain>,
// }

impl ContextKey {
    /// Is the key empty?
    pub fn is_empty(&self) -> bool {
        self.entity.is_none() && self.script.is_none() && self.domain.is_none()
    }

    /// or with other context
    pub fn or(self, other: ContextKey) -> Self {
        Self {
            entity: self.entity.or(other.entity),
            script: self.script.or(other.script),
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
            || self.script.as_ref().map(|a| other.script.as_ref().map(|b| a == b).unwrap_or(false)).unwrap_or(true)
            || self.domain.as_ref().map(|a| other.domain.as_ref().map(|b| a == b).unwrap_or(false)).unwrap_or(true)
    }

    /// If a script handle is present and is strong, convert it to a weak
    /// handle.
    pub fn into_weak(mut self) -> Self {
        if let Some(script) = self.script.as_mut() {
            if script.is_strong() {
                *script = Handle::Weak(script.id());
            }
        }
        self
    }

    // pub fn as_ref(&self) -> ContextKeyRef<'_> {
    //     ContextKeyRef {
    //         entity: self.entity,
    //         script: self.script.as_ref().map(Handle::Weak),
    //         domain: self.domain.as_ref(),
    //     }
    // }
}

// impl<'a> Borrow<ContextKeyRef<'a>> for ContextKey {
//     #[inline]
//     fn borrow(&self) -> &ContextKeyRef<'a> {
//         self.as_ref()
//     }
// }

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
            script: Some(Handle::Weak(script_id)),
            ..default()
        }
    }
}

impl From<Handle<ScriptAsset>> for ContextKey {
    fn from(handle: Handle<ScriptAsset>) -> Self {
        Self {
            script: Some(handle),
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
