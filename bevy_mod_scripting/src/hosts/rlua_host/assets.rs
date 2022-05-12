use crate::CodeAsset;
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    reflect::TypeUuid,
};
use std::sync::Arc;

#[derive(Debug, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
/// A lua code file in bytes
pub struct LuaFile {
    pub bytes: Arc<[u8]>,
}

impl CodeAsset for LuaFile {
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Default)]
/// Asset loader for lua scripts
pub struct LuaLoader;

impl AssetLoader for LuaLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>> {
        load_context.set_default_asset(LoadedAsset::new(LuaFile {
            bytes: bytes.into(),
        }));
        Box::pin(async move { Ok(()) })
    }

    fn extensions(&self) -> &[&str] {
        &["lua"]
    }
}
