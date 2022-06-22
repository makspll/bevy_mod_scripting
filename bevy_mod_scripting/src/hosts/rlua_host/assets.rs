use crate::CodeAsset;
use bevy::{
    asset::{AssetLoader, LoadedAsset, FileAssetIo},
    reflect::TypeUuid,
};
use tealr::embed_compiler;
use std::{sync::Arc, process::Command, fs};

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


// #[cfg(feature="teal")]
static teal_compiler : fn(&str) -> String = embed_compiler!("v0.9.0");

#[derive(Default)]
/// Asset loader for lua scripts
pub struct LuaLoader;

impl AssetLoader for LuaLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>> {

        match load_context.path().extension().map(|s| s.to_str().unwrap()) {
            // #[cfg(feature="teal")]
            Some("tl") => {
                
                let full_path = &FileAssetIo::get_root_path()
                    .join("assets")
                    .join(load_context.path());

                // TODO: go through string and remove instances of require asking for .t.dl scripts

                let command = Command::new("tl")
                    .args(&[
                        "check",
                        // "-I",
                        // path.as_os_str(),
                        full_path.to_str().unwrap(),
                    ])
                    .status()
                    .expect("Invalid .tl file");

                let command = Command::new("tl")
                    .args(&[
                        "gen",
                        // "-I",
                        // path.as_os_str(),
                        full_path.to_str().unwrap(),
                        "-o",
                        "temp.lua"
                    ])
                    .status()
                    .expect("Could not generate lua file");
                
                let lua_code = fs::read_to_string("temp.lua").expect("Could not find output lua file");
                fs::remove_file("temp.lua");

                load_context.set_default_asset(LoadedAsset::new(LuaFile {
                    bytes: lua_code.as_bytes().into(),
                }));
            },
            _ => {
                load_context.set_default_asset(LoadedAsset::new(LuaFile {
                    bytes: bytes.into(),
                }));
            },
        }

        Box::pin(async move { Ok(()) })
    }

    #[cfg(feature="teal")]
    fn extensions(&self) -> &[&str] {
        &["lua","tl"]
    }
    #[cfg(not(feature="teal"))]
    fn extensions(&self) -> &[&str] {
        &["lua"]
    }

}
