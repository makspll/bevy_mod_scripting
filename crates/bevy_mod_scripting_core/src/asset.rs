use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use bevy::{
    asset::{Asset, AssetLoader, AsyncReadExt},
    ecs::system::Resource,
    reflect::TypePath,
    utils::BoxedFuture,
};

use crate::{prelude::ScriptError, script::ScriptId};

/// Represents a script loaded into memory as an asset
#[derive(Asset, TypePath, Clone)]
pub struct ScriptAsset {
    pub content: Box<[u8]>,
    /// The virtual filesystem path of the asset, used to map to the script Id for asset backed scripts
    pub asset_path: PathBuf,
    pub language: Cow<'static, str>,
}

pub struct ScriptAssetLoader {
    /// Used to set the language of the script
    pub language: Cow<'static, str>,
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
        reader.read_to_end(&mut content).await.map_err(|e| {
            ScriptError::new_lifecycle_error(e).with_context(load_context.asset_path())
        })?;
        if let Some(processor) = &self.preprocessor {
            processor(&mut content)?;
        }
        let asset = ScriptAsset {
            content: content.into_boxed_slice(),
            asset_path: load_context.path().to_owned(),
            language: self.language.clone(),
        };
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        self.extensions
    }
}

#[derive(Clone, Copy, Resource)]
pub struct ScriptAssetSettings {
    pub script_id_mapper: AssetPathToScriptIdMapper,
}

impl Default for ScriptAssetSettings {
    fn default() -> Self {
        Self {
            script_id_mapper: AssetPathToScriptIdMapper {
                map: (|path: &Path| path.to_string_lossy().into_owned().into()),
            },
        }
    }
}

/// Strategy for mapping asset paths to script ids, by default this is the identity function
#[derive(Clone, Copy)]
pub struct AssetPathToScriptIdMapper {
    pub map: fn(&Path) -> ScriptId,
}
