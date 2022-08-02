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
        match load_context.path().extension().map(|s| s.to_str().unwrap()) {
            
            #[cfg(all(feature = "teal", debug_assertions))]
            Some("tl") => {

                use anyhow::anyhow;
                use bevy::asset::FileAssetIo;
                use std::fs;
                use std::process::Command;

                let scripts_dir = &FileAssetIo::get_base_path().join("assets").join("scripts");

                let temp_file_path = &std::env::temp_dir().join("bevy_mod_scripting.temp.lua");

                let full_path = &FileAssetIo::get_base_path()
                    .join("assets")
                    .join(load_context.path());

                if let Ok(e) = Command::new("tl")
                    .args(&["check", full_path.to_str().unwrap()])
                    .current_dir(scripts_dir)
                    .status()
                {
                    if !e.success() {
                        return Box::pin(async move {
                            Err(anyhow!(
                                "Teal file `{}` has errors",
                                load_context.path().to_str().unwrap()
                            ))
                        });
                    }
                } else {
                    fs::remove_file(temp_file_path)
                        .expect("Something went wrong running `tl check`");
                    panic!("Something went wrong running `tl check`");
                }

                if let Ok(e) = Command::new("tl")
                    .args(&[
                        "gen",
                        full_path.to_str().unwrap(),
                        "-o",
                        temp_file_path.to_str().unwrap(),
                    ])
                    .current_dir(scripts_dir)
                    .status()
                {
                    if !e.success() {
                        return Box::pin(async move {
                            Err(anyhow!(
                                "Teal file `{}` could not be compiled!",
                                load_context.path().to_str().unwrap()
                            ))
                        });
                    }
                } else {
                    fs::remove_file(temp_file_path).expect("Something went wrong running `tl gen`");
                    panic!("Something went wrong running `tl gen`")
                }

                let lua_code =
                    fs::read_to_string(temp_file_path).expect("Could not find output lua file");
                fs::remove_file(temp_file_path).unwrap();

                load_context.set_default_asset(LoadedAsset::new(LuaFile {
                    bytes: lua_code.as_bytes().into(),
                }));
            }
            _ => {
                load_context.set_default_asset(LoadedAsset::new(LuaFile {
                    bytes: bytes.into(),
                }));
            }
        }

        Box::pin(async move { Ok(()) })
    }

    #[cfg(feature = "teal")]
    fn extensions(&self) -> &[&str] {
        &["lua", "tl"]
    }
    #[cfg(not(feature = "teal"))]
    fn extensions(&self) -> &[&str] {
        &["lua"]
    }
}
