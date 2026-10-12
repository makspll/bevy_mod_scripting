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

    /// Returns true if the attachment is a static script.
    pub fn is_static(&self) -> bool {
        matches!(self, ScriptAttachment::StaticScript(_))
    }

    /// Returns true if the attachment is an entity script.
    pub fn is_entity_script(&self) -> bool {
        matches!(self, ScriptAttachment::EntityScript(_, _))
    }
}

/// Describes the designated recipients of a script event
#[derive(Clone, Debug, Reflect)]
pub enum Recipients {
    /// The event needs to be handled by all scripts, if multiple scripts share a context, the event will be sent once per script in the context.
    /// Scripts currently loading or unloading will NOT receive this event after/before they are loaded/unloaded.
    AllScripts,
    /// The event is to be handled by all unique contexts, i.e. if two scripts share the same context, the event will be sent only once per the context.
    /// Contexts currently loading or unloading will NOT receive this event after/before they are loaded/unloaded.
    AllContexts,
    /// The event is to be handled by a specific script-entity pair
    /// If the target context is in the process of loading, the callback will be re-queued untill the script is loaded, if the entity has been despawned the callback will not be executed
    ScriptEntity(Handle<ScriptAsset>, Entity),
    /// the event is to be handled by a specific static script
    /// If the target context is in the process of loading, the callback will be re-queued untill the script is loaded.
    StaticScript(Handle<ScriptAsset>),
}
