use std::fmt;

use bevy_ecs::entity::Entity;
use bevy_mod_scripting_display::DisplayProxy;

use super::*;
use crate::ScriptAsset;

impl From<ScriptAttachment> for ContextKey {
    fn from(val: ScriptAttachment) -> Self {
        match val {
            ScriptAttachment::EntityScript(entity, script) => ContextKey {
                entity: Some(entity),
                script: Some(script),
            },
            ScriptAttachment::StaticScript(script) => ContextKey {
                entity: None,
                script: Some(script),
            },
        }
    }
}

/// The key for a context. The context key is used for:
/// - Identifying the script itself, uniquely.
/// - later on it's mapped to a context, which will determine how the scripts are grouped in execution environments.
#[derive(Debug, Hash, Clone, Default, PartialEq, Eq, Reflect)]
pub struct ContextKey {
    /// Entity if there is one.
    pub entity: Option<Entity>,
    /// Script ID if there is one.
    /// Can be empty if the script is not driven by an asset.
    pub script: Option<Handle<ScriptAsset>>,
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
        if empty {
            write!(f, "empty")?;
        }
        Ok(())
    }
}

impl ContextKey {
    /// Creates an invalid context key, which should never exist.
    pub const INVALID: Self = Self {
        entity: Some(Entity::PLACEHOLDER),
        script: Some(Handle::Weak(AssetId::invalid())),
    };

    /// Creates a shared context key, which is used for shared contexts
    pub const SHARED: Self = {
        Self {
            entity: None,
            script: None,
        }
    };

    /// Is the key empty?
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// or with other context
    pub fn or(self, other: ContextKey) -> Self {
        Self {
            entity: self.entity.or(other.entity),
            script: self.script.or(other.script),
        }
    }

    /// If a script handle is present and is strong, convert it to a weak
    /// handle.
    pub fn into_weak(mut self) -> Self {
        if let Some(script) = &self.script
            && script.is_strong()
        {
            self.script = Some(script.clone_weak());
        }

        self
    }
}
