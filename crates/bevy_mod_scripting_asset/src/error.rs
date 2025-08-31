//! Error definitions for the scripting asset pipeline.

use std::fmt::Display;

use bevy_asset::AssetPath;

#[derive(Debug)]
/// An error that can occur when loading or processing a script asset.
pub struct ScriptAssetError {
    pub(crate) phrase: &'static str,
    pub(crate) asset_path: Option<AssetPath<'static>>,
    pub(crate) inner: Box<dyn std::error::Error + Send + Sync + 'static>,
}
impl ScriptAssetError {
    /// Create a new script asset error
    pub fn new(
        phrase: &'static str,
        asset_path: Option<&AssetPath<'static>>,
        inner: Box<dyn std::error::Error + Send + Sync + 'static>,
    ) -> Self {
        Self {
            phrase,
            asset_path: asset_path.cloned(),
            inner,
        }
    }
}

impl Display for ScriptAssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = &self.asset_path {
            write!(
                f,
                "Error {}. while processing script asset '{}': {}",
                self.phrase, path, self.inner
            )
        } else {
            write!(
                f,
                "Error {}. while processing script asset: {}",
                self.phrase, self.inner
            )
        }
    }
}

impl std::error::Error for ScriptAssetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.inner.as_ref())
    }
}
