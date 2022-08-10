use std::{
    env,
    fs::{self, File},
    io::Write,
    process::Command,
};

use bevy::asset::FileAssetIo;
use tealr::TypeWalker;

use bevy_mod_scripting_core::{DocFragment, ScriptError};

pub type TypeWalkerBuilder = fn(TypeWalker) -> TypeWalker;

struct Fragment {
    builder: TypeWalkerBuilder,
}

pub struct LuaDocFragment {
    walker: Vec<Fragment>,
}

/// A piece of lua documentation,
/// Each piece is combined into one large documentation page, and also a single teal declaration file if the `teal` feature is enabled
impl LuaDocFragment {
    pub fn new(f: TypeWalkerBuilder) -> Self {
        Self {
            walker: vec![Fragment { builder: f }],
        }
    }
}

impl DocFragment for LuaDocFragment {
    fn merge(mut self, o: Self) -> Self {
        self.walker.extend(o.walker.into_iter());
        self
    }

    fn gen_docs(self) -> Result<(), ScriptError> {
        let script_asset_path = &FileAssetIo::get_base_path().join("assets").join("scripts");

        let script_doc_dir = &env::var("SCRIPT_DOC_DIR")
            .map(|v| v.into())
            .unwrap_or_else(|_e| script_asset_path.join("doc"));

        fs::create_dir_all(script_doc_dir)
            .expect("Could not create `.../assets/scripts/doc` directories");

        // build the type walker
        let tw = self
            .walker
            .into_iter()
            .fold(TypeWalker::new(), |a, v| (v.builder)(a));

        // generate temporary json file
        let json = serde_json::to_string_pretty(&tw)
            .map_err(|e| ScriptError::DocGenError(e.to_string()))?;
        let temp_dir = &std::env::temp_dir().join("bevy_mod_scripting.temp.json");

        let mut json_file =
            File::create(temp_dir).map_err(|e| ScriptError::DocGenError(e.to_string()))?;

        json_file
            .write_all(json.as_bytes())
            .map_err(|e| ScriptError::DocGenError(e.to_string()))?;
        json_file.flush().unwrap();

        Command::new("tealr_doc_gen")
            .args([
                "--json",
                temp_dir.to_str().unwrap(),
                "--name",
                "LuaApi",
                "--build_folder",
                fs::canonicalize(script_doc_dir).unwrap().to_str().unwrap(),
            ])
            .status()
            .map_err(|e| ScriptError::DocGenError(e.to_string()))?;

        fs::remove_file(&temp_dir).unwrap();
        #[cfg(feature = "teal")]
        {
            // now generate teal declaration (d.tl) file

            let script_types_dir = &script_asset_path.join("types");
            fs::create_dir_all(script_types_dir)
                .expect("Could not create `.../assets/scripts/types` directories");

            let decl_path = &script_types_dir.join("types.d.tl");
            // generate declaration file
            let decl_file_contents = tw.generate("types", true).unwrap();

            let mut decl_file =
                File::create(decl_path).map_err(|e| ScriptError::DocGenError(e.to_string()))?;

            decl_file
                .write_all(decl_file_contents.as_bytes())
                .expect("Failed to write to declaration file");
            decl_file.flush().unwrap();

            // finally create a tlconfig.lua file if doesn't exist
            // we do this to avoid problems with varying teal configurations
            // keep em settings consistent everywhere
            let tl_config_path = script_asset_path.join("tlconfig.lua");
            if !tl_config_path.exists() {
                let mut tl_file = File::create(tl_config_path)
                    .map_err(|e| ScriptError::DocGenError(e.to_string()))?;
                tl_file
                    .write_all(
                        r#"
return {
    global_env_def="types/types",
    build_dir="build/"
}
"#
                        .as_bytes(),
                    )
                    .map_err(|e| ScriptError::DocGenError(e.to_string()))?;
            }
        }
        Ok(())
    }
}
