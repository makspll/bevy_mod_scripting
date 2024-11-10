//! Everything to do with the way scripts and their contexts are stored and handled.

use std::{borrow::Cow, collections::HashMap, ops::Deref};

use bevy::{asset::Handle, ecs::system::Resource, reflect::Reflect};

use crate::{asset::ScriptAsset, context::ContextId};

pub type ScriptId = Cow<'static, str>;

#[derive(bevy::ecs::component::Component, Reflect)]
pub struct ScriptComponent(pub Vec<ScriptId>);

impl Deref for ScriptComponent {
    type Target = Vec<ScriptId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ScriptComponent {
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
    pub id: ScriptId,
    /// the asset holding the content of the script if it comes from an asset
    pub asset: Option<Handle<ScriptAsset>>,
    /// The id of the context this script is currently assigned to
    pub context_id: ContextId,
}
