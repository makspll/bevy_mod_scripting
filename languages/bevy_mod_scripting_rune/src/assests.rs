use bevy::{
    asset::{AssetLoader, LoadedAsset},
    reflect::{TypePath, TypeUuid},
};
use bevy_mod_scripting_core::prelude::*;
use std::sync::Arc;

#[derive(Debug, TypeUuid, TypePath)]
#[uuid = "e4f7d00d-5acd-45fb-a29c-6472718771fc"]
/// A loaded rune file in bytes.
pub struct RuneFile {
    /// File content in bytes.
    pub bytes: Arc<[u8]>,
}

impl CodeAsset for RuneFile {
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Default)]
/// Enables loading Rune scripts from `.rune` and `.rn` files.
pub struct RuneLoader;

impl AssetLoader for RuneLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            load_context.set_default_asset(LoadedAsset::new(RuneFile {
                bytes: bytes.into(),
            }));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["rune", "rn"]
    }
}
