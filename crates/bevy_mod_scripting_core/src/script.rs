//! Script related types, functions and components

use crate::{asset::ScriptAsset, context::ContextId};
use bevy::{asset::Handle, ecs::system::Resource, reflect::Reflect};
use std::{borrow::Cow, collections::HashMap, ops::Deref};

/// A unique identifier for a script, by default corresponds to the path of the asset excluding the asset source.
///
/// I.e. an asset with the path `path/to/asset.ext` will have the script id `path/to/asset.ext`
pub type ScriptId = Cow<'static, str>;

#[derive(bevy::ecs::component::Component, Reflect, Clone)]

/// A component which identifies the scripts existing on an entity.
///
/// Event handlers search for components with this component to figure out which scripts to run and on which entities.
pub struct ScriptComponent(pub Vec<ScriptId>);

impl Deref for ScriptComponent {
    type Target = Vec<ScriptId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ScriptComponent {
    /// Creates a new [`ScriptComponent`] with the given ScriptID's
    pub fn new(components: Vec<ScriptId>) -> Self {
        Self(components)
    }
}

/// All the scripts which are currently loaded or loading and their mapping to contexts
#[derive(Resource, Default, Clone)]
pub struct Scripts {
    pub(crate) scripts: HashMap<ScriptId, Script>,
}

/// A script
#[derive(Clone)]
pub struct Script {
    /// The id of the script
    pub id: ScriptId,
    /// the asset holding the content of the script if it comes from an asset
    pub asset: Option<Handle<ScriptAsset>>,
    /// The id of the context this script is currently assigned to
    pub context_id: ContextId,
}
