//! Contains definitions for script related structures

use bevy_asset::Handle;
use bevy_ecs::entity::Entity;
use bevy_mod_scripting_asset::ScriptAsset;
use bevy_mod_scripting_display::DisplayProxy;
use bevy_reflect::Reflect;
use std::fmt;

/// Specifies a unique attachment of a script. These attachments are mapped to [`bevy_mod_scripting_core::ContextKey`]'s depending on the context policy used.
#[derive(Debug, Hash, Clone, PartialEq, Eq, Reflect)]
pub enum ScriptAttachment {
    /// a script attached to an entity, with an optional domain. By default selecting a domain will put the context of this script on a per-domain basis.
    EntityScript(Entity, Handle<ScriptAsset>),
    /// a static script, with an optional domain. By default selecting a domain will put the context of this script on a per-domain basis.
    StaticScript(Handle<ScriptAsset>),
}

impl std::fmt::Display for ScriptAttachment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptAttachment::EntityScript(entity, script) => {
                write!(
                    f,
                    "EntityScript(entity: {}, script: {})",
                    entity,
                    script.display(),
                )
            }
            ScriptAttachment::StaticScript(script) => {
                write!(f, "StaticScript(script: {})", script.display())
            }
        }
    }
}

impl ScriptAttachment {
    /// Returns the script handle.
    pub fn script(&self) -> Handle<ScriptAsset> {
        match self {
            ScriptAttachment::EntityScript(_, script) => script.clone(),
            ScriptAttachment::StaticScript(script) => script.clone(),
        }
    }

    /// Returns a mutable reference to the underlying script handle.
    pub fn script_mut(&mut self) -> &mut Handle<ScriptAsset> {
        match self {
            ScriptAttachment::EntityScript(_, script) => script,
            ScriptAttachment::StaticScript(script) => script,
        }
    }

    /// Returns the entity if it exists.
    pub fn entity(&self) -> Option<Entity> {
        match self {
            ScriptAttachment::EntityScript(entity, _) => Some(*entity),
            ScriptAttachment::StaticScript(_) => None,
        }
    }

    /// Downcasts any script handles into weak handles.
    pub fn into_weak(self) -> Self {
        match self {
            ScriptAttachment::EntityScript(entity, script) => {
                ScriptAttachment::EntityScript(entity, script.clone_weak())
            }
            ScriptAttachment::StaticScript(script) => {
                ScriptAttachment::StaticScript(script.clone_weak())
            }
        }
    }

    /// Returns true if the attachment is a static script.
    pub fn is_static(&self) -> bool {
        matches!(self, ScriptAttachment::StaticScript(_))
    }

    /// Returns true if the attachment is an entity script.
    pub fn is_entity_script(&self) -> bool {
        matches!(self, ScriptAttachment::EntityScript(_, _))
    }
}
