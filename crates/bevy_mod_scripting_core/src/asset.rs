use crate::{error::ScriptError, script::ScriptId};
use bevy::{
    asset::{Asset, AssetId, AssetLoader},
    ecs::system::Resource,
    reflect::TypePath,
    utils::HashMap,
};
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

/// Represents a scripting language. Languages which compile into another language should use the target language as their language.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
    Rhai,
    Lua,
    Rune,
    External(Cow<'static, str>),
    /// Initial setting before being processed by the script synchronization systems
    Unset,
    /// Set if none of the asset path to language mappers match
    Unknown,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rhai => "Rhai".fmt(f),
            Language::Lua => "Lua".fmt(f),
            Language::Rune => "Rune".fmt(f),
            Language::External(cow) => cow.fmt(f),
            Language::Unset => "Unset".fmt(f),
            Language::Unknown => "Unknown".fmt(f),
        }
    }
}

/// Represents a script loaded into memory as an asset
#[derive(Asset, TypePath, Clone)]
pub struct ScriptAsset {
    pub content: Box<[u8]>,
    /// The virtual filesystem path of the asset, used to map to the script Id for asset backed scripts
    pub asset_path: PathBuf,
    pub language: Language,
}

pub struct ScriptAssetLoader {
    /// The file extensions this loader should handle
    pub extensions: &'static [&'static str],
    /// preprocessor to run on the script before saving the content to an asset
    pub preprocessor: Option<Box<dyn Fn(&mut [u8]) -> Result<(), ScriptError> + Send + Sync>>,
}

impl AssetLoader for ScriptAssetLoader {
    type Asset = ScriptAsset;

    type Settings = ();

    type Error = ScriptError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut content = Vec::new();
        reader
            .read_to_end(&mut content)
            .await
            .map_err(|e| ScriptError::new_external(e).with_context(load_context.asset_path()))?;
        if let Some(processor) = &self.preprocessor {
            processor(&mut content)?;
        }
        let asset = ScriptAsset {
            content: content.into_boxed_slice(),
            asset_path: load_context.path().to_owned(),
            language: Language::Unset,
        };
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        self.extensions
    }
}

#[derive(Clone, Resource)]
pub struct ScriptAssetSettings {
    pub script_id_mapper: AssetPathToScriptIdMapper,
    pub script_language_mappers: Vec<AssetPathToLanguageMapper>,
}

impl ScriptAssetSettings {
    pub fn select_script_language(&self, path: &Path) -> Language {
        for mapper in &self.script_language_mappers {
            let language = (mapper.map)(path);
            match language {
                Language::Unset | Language::Unknown => continue,
                _ => return language,
            }
        }

        Language::Unknown
    }
}

impl Default for ScriptAssetSettings {
    fn default() -> Self {
        Self {
            script_id_mapper: AssetPathToScriptIdMapper {
                map: (|path: &Path| path.to_string_lossy().into_owned().into()),
            },
            script_language_mappers: vec![AssetPathToLanguageMapper {
                map: (|_: &Path| Language::Unset),
            }],
        }
    }
}

/// Strategy for mapping asset paths to script ids, by default this is the identity function
#[derive(Clone, Copy)]
pub struct AssetPathToScriptIdMapper {
    pub map: fn(&Path) -> ScriptId,
}

#[derive(Clone, Copy)]
pub struct AssetPathToLanguageMapper {
    pub map: fn(&Path) -> Language,
}

/// A cache of asset id's to their script id's. Necessary since when we drop an asset we won't have the ability to get the path from the asset.
#[derive(Default, Debug, Resource)]
pub struct ScriptMetadataStore {
    pub map: HashMap<AssetId<ScriptAsset>, ScriptMetadata>,
}

#[derive(Debug, Clone)]
pub struct ScriptMetadata {
    pub script_id: ScriptId,
    pub language: Language,
}

impl ScriptMetadataStore {
    pub fn insert(&mut self, id: AssetId<ScriptAsset>, meta: ScriptMetadata) {
        self.map.insert(id, meta);
    }

    pub fn get(&self, id: AssetId<ScriptAsset>) -> Option<&ScriptMetadata> {
        self.map.get(&id)
    }

    pub fn remove(&mut self, id: AssetId<ScriptAsset>) -> Option<ScriptMetadata> {
        self.map.remove(&id)
    }
}
