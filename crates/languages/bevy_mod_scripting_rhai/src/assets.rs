use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, LoadContext},
    reflect::TypePath,
};

use bevy_mod_scripting_core::prelude::*;

#[derive(Asset, Debug, TypePath)]
/// A rhai code file in bytes
pub struct RhaiFile {
    pub bytes: Vec<u8>,
}

impl CodeAsset for RhaiFile {
    fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

#[derive(Default)]
/// Asset loader for lua scripts
pub struct RhaiLoader;

impl AssetLoader for RhaiLoader {
    type Asset = RhaiFile;
    type Settings = ();
    type Error = anyhow::Error;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _: &'a Self::Settings,
        _: &'a mut LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            Ok(RhaiFile { bytes })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["rhai"]
    }
}
