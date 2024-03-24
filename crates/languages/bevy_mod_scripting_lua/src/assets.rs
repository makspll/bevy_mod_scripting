use bevy::{
    asset::{AssetLoader, Error, LoadedAsset},
    reflect::{TypePath, TypeUuid},
};
use bevy_mod_scripting_core::asset::CodeAsset;

use std::sync::Arc;

#[derive(Debug, TypeUuid, TypePath)]
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
    ) -> bevy::asset::BoxedFuture<'a, Result<(), Error>> {
        match load_context.path().extension().map(|s| s.to_str().unwrap()) {
            #[cfg(all(feature = "teal", debug_assertions))]
            Some("tl") => {
                use bevy::asset::FileAssetIo;
                use std::fs;
                use std::path::PathBuf;
                use std::process::Command;

                let scripts_dir = &FileAssetIo::get_base_path().join("assets").join("scripts");

                let temp_file_path = &std::env::temp_dir().join("bevy_mod_scripting.temp.lua");

                // optionally put the output in the /build folder
                let build_dir_path: Option<PathBuf> =
                    if load_context.path().starts_with("scripts/build/") {
                        Some(
                            load_context
                                .path()
                                .strip_prefix("scripts/")
                                .unwrap()
                                .to_owned(),
                        )
                    } else if load_context.path().starts_with("scripts/") {
                        Some(
                            PathBuf::from("build/")
                                .join(load_context.path().strip_prefix("scripts/").unwrap()),
                        )
                    } else {
                        None
                    };

                let full_path = &FileAssetIo::get_base_path()
                    .join("assets")
                    .join(load_context.path());

                if let Ok(e) = Command::new("tl")
                    .args(["check", full_path.to_str().unwrap()])
                    .current_dir(scripts_dir)
                    .status()
                {
                    if !e.success() {
                        return Box::pin(async move {
                            Err(Error::msg(format!(
                                "Teal file `{}` has errors",
                                load_context.path().to_str().unwrap()
                            )))
                        });
                    }
                } else {
                    fs::remove_file(temp_file_path)
                        .expect("Something went wrong running `tl check`");
                    panic!("Something went wrong running `tl check`");
                }

                if let Ok(e) = Command::new("tl")
                    .args([
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
                            Err(Error::msg(format!(
                                "Teal file `{}` could not be compiled!",
                                load_context.path().to_str().unwrap()
                            )))
                        });
                    }
                } else {
                    fs::remove_file(temp_file_path).expect("Something went wrong running `tl gen`");
                    panic!("Something went wrong running `tl gen`")
                }

                if let Some(mut build_dir_path) = build_dir_path {
                    build_dir_path = scripts_dir.join(build_dir_path);
                    let _ = fs::create_dir_all(build_dir_path.parent().unwrap());
                    let _ = fs::copy(temp_file_path, build_dir_path.with_extension("lua"));
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
