use anyhow::Error;
use bevy::{
    asset::{io::Reader, Asset, AssetLoader},
    reflect::TypePath,
};
use bevy_mod_scripting_core::prelude::*;

#[derive(Asset, TypePath, Debug)]
/// A loaded rune file in bytes.
pub struct RuneFile {
    /// File content in bytes.
    pub bytes: Vec<u8>,
}

impl CodeAsset for RuneFile {
    fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

#[derive(Default)]
/// Enables loading Rune scripts from `.rune` and `.rn` files.
pub struct RuneLoader;

impl AssetLoader for RuneLoader {
    type Asset = RuneFile;
    type Settings = ();
    type Error = Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> std::result::Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(RuneFile { bytes })
    }

    fn extensions(&self) -> &[&str] {
        &["rune", "rn"]
    }
}
