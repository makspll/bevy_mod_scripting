use super::*;
use crate::ScriptAsset;
use bevy::prelude::{default, Entity};
use std::fmt;

/// Specifies a unique attachment of a script. These attachments are mapped to [`ContextKey`]'s depending on the context policy used.
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum ScriptAttachment {
    /// a script attached to an entity, with an optional domain.
    EntityScript(Entity, Handle<ScriptAsset>, Option<Domain>),
    /// a static script, with an optional domain.
    StaticScript(Handle<ScriptAsset>, Option<Domain>),
    /// External context key, comes from outside the BMS integration, can be combined with a custom policy for custom behavior.
    Other(ContextKey),
}

impl std::fmt::Display for ScriptAttachment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptAttachment::EntityScript(entity, script, domain) => {
                write!(
                    f,
                    "EntityScript(entity: {}, script: {}, domain: {:?})",
                    entity,
                    script.display(),
                    domain
                )
            }
            ScriptAttachment::StaticScript(script, domain) => {
                write!(
                    f,
                    "StaticScript(script: {}, domain: {:?})",
                    script.display(),
                    domain
                )
            }
            ScriptAttachment::Other(context_key) => write!(f, "Other({context_key})"),
        }
    }
}

impl ScriptAttachment {
    /// Returns the script handle if it exists.
    pub fn script(&self) -> Option<Handle<ScriptAsset>> {
        match self {
            ScriptAttachment::EntityScript(_, script, _) => Some(script.clone()),
            ScriptAttachment::StaticScript(script, _) => Some(script.clone()),
            ScriptAttachment::Other(context_key) => context_key.script.clone(),
        }
    }

    /// Returns the entity if it exists.
    pub fn entity(&self) -> Option<Entity> {
        match self {
            ScriptAttachment::EntityScript(entity, _, _) => Some(*entity),
            ScriptAttachment::StaticScript(_, _) => None,
            ScriptAttachment::Other(context_key) => context_key.entity,
        }
    }
}

impl From<ScriptAttachment> for ContextKey {
    fn from(val: ScriptAttachment) -> Self {
        match val {
            ScriptAttachment::EntityScript(entity, script, domain) => ContextKey {
                entity: Some(entity),
                script: Some(script),
                domain,
            },
            ScriptAttachment::StaticScript(script, domain) => ContextKey {
                entity: None,
                script: Some(script),
                domain,
            },
            ScriptAttachment::Other(context_key) => context_key,
        }
    }
}

/// The key for a context. The context key is used for:
/// - Identifying the script itself, uniquely.
/// - later on it's mapped to a context, which will determine how the scripts are grouped in execution environments.
#[derive(Debug, Hash, Clone, Default, PartialEq, Eq)]
pub struct ContextKey {
    /// Entity if there is one.
    pub(crate) entity: Option<Entity>,
    /// Script ID if there is one.
    /// Can be empty if the script is not driven by an asset.
    pub(crate) script: Option<Handle<ScriptAsset>>,
    /// Domain if there is one.
    pub(crate) domain: Option<Domain>,
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
        if let Some(domain) = &self.domain {
            write!(f, "domain {domain:?}")?;
            empty = false;
        }
        if empty {
            write!(f, "empty")?;
        }
        Ok(())
    }
}

impl ContextKey {
    /// Creates an invalid context key, which should never exist.
    pub const INVALID: Self = Self {
        entity: Some(Entity::from_raw(0)),
        script: Some(Handle::Weak(AssetId::invalid())),
        domain: None,
    };

    /// Is the key empty?
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// or with other context
    pub fn or(self, other: ContextKey) -> Self {
        Self {
            entity: self.entity.or(other.entity),
            script: self.script,
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
        self.entity
            .map(|a| other.entity.map(|b| a == b).unwrap_or(false))
            .unwrap_or(true)
            || self.script == other.script
            || self
                .domain
                .as_ref()
                .map(|a| other.domain.as_ref().map(|b| a == b).unwrap_or(false))
                .unwrap_or(true)
    }

    /// If a script handle is present and is strong, convert it to a weak
    /// handle.
    pub fn into_weak(mut self) -> Self {
        if let Some(script) = &self.script {
            if script.is_strong() {
                self.script = Some(script.clone_weak());
            }
        }
        self
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
