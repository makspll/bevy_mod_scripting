//! A loader pipeline for script assets

use bevy_asset::AssetLoader;
use bevy_log::warn;
use serde::{Deserialize, Serialize};

use crate::{Language, LanguageExtensions, ScriptAsset, ScriptAssetError};

/// Script settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScriptSettings {
    /// Define the language for a script or use the extension if None.
    pub language: Option<Language>,
}

/// A loader for script assets
pub struct ScriptAssetLoader {
    /// The file extensions this loader should handle
    language_extensions: &'static LanguageExtensions,
    /// preprocessor to run on the script before saving the content to an asset
    pub preprocessor: Option<Box<dyn Fn(&mut [u8]) -> Result<(), ScriptAssetError> + Send + Sync>>,
}

impl ScriptAssetLoader {
    /// Create a new script asset loader for the given extensions.
    pub fn new(language_extensions: &'static LanguageExtensions) -> Self {
        Self {
            language_extensions,
            preprocessor: None,
        }
    }

    /// Add a preprocessor
    pub fn with_preprocessor(
        mut self,
        preprocessor: Box<dyn Fn(&mut [u8]) -> Result<(), ScriptAssetError> + Send + Sync>,
    ) -> Self {
        self.preprocessor = Some(preprocessor);
        self
    }
}

impl AssetLoader for ScriptAssetLoader {
    type Asset = ScriptAsset;

    type Settings = ScriptSettings;

    type Error = ScriptAssetError;

    async fn load(
        &self,
        reader: &mut dyn bevy_asset::io::Reader,
        settings: &Self::Settings,
        load_context: &mut bevy_asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut content = Vec::new();
        reader.read_to_end(&mut content).await.map_err(|e| {
            ScriptAssetError::new(
                "reading from disk",
                Some(load_context.asset_path()),
                Box::new(e),
            )
        })?;
        if let Some(processor) = &self.preprocessor {
            processor(&mut content)?;
        }
        let language = settings.language.clone().unwrap_or_else(|| {
            let ext = load_context
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or_default();
            self.language_extensions
                .get(ext)
                .cloned()
                .unwrap_or_else(|| {
                    warn!("Unknown language for {:?}", load_context.path().display());
                    Language::Unknown
                })
        });
        // if language == Language::Lua && cfg!(not(feature = "mlua")) {
        //     warn_once!(
        //         "Script {:?} is a Lua script but the {:?} feature is not enabled; the script will not be evaluated.",
        //         load_context.path().display(),
        //         "mlua"
        //     );
        // }
        // if language == Language::Rhai && cfg!(not(feature = "rhai")) {
        //     warn_once!(
        //         "Script {:?} is a Rhai script but the {:?} feature is not enabled; the script will not be evaluated.",
        //         load_context.path().display(),
        //         "rhai"
        //     );
        // }
        let asset = ScriptAsset {
            content: content.into_boxed_slice(),
            language,
        };
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        self.language_extensions.extensions()
    }
}
