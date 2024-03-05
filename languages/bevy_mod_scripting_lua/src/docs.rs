use std::{
    env,
    fs::{self, File},
    io::Write,
    process::Command,
};

//use bevy::asset::FileAssetIo;
use bevy::asset::io::file::FileAssetReader;
use bevy_mod_scripting_core::prelude::*;
use tealr::{TypeGenerator, TypeWalker};

pub type TypeWalkerBuilder = fn(TypeWalker) -> TypeWalker;

static DEFAULT_DOC_CONFIG: fn(&str) -> String = |s| {
    format!(
        r#"
{{
    "doc_template": "Builtin",
    "page_root": "",
    "store_in": "{s}",
    "name": "{s}",
    "type_def_files": {{
      "runner": "Builtin",
      "templates": {{
        "teal": {{
          "extension": ".d.tl",
          "template": "Teal"
        }}
      }}
    }}
  }}

"#
    )
};

#[cfg(feature = "teal")]
static DEFAULT_TEAL_CONFIG: &str = r#"
return {
    global_env_def="types/types",
    build_dir="build/"
}
"#;

struct Fragment {
    builder: TypeWalkerBuilder,
}

pub struct LuaDocFragment {
    name: &'static str,
    walker: Vec<Fragment>,
}

/// A piece of lua documentation,
/// Each piece is combined into one large documentation page, and also a single teal declaration file if the `teal` feature is enabled
impl LuaDocFragment {
    pub fn new(name: &'static str, f: TypeWalkerBuilder) -> Self {
        Self {
            name,
            walker: vec![Fragment { builder: f }],
        }
    }
}

impl DocFragment for LuaDocFragment {
    fn name(&self) -> &'static str {
        self.name
    }

    fn merge(mut self, o: Self) -> Self {
        self.walker.extend(o.walker);
        self
    }

    fn gen_docs(self) -> Result<(), ScriptError> {
        let script_asset_path = &FileAssetReader::get_base_path()
            .join("assets")
            .join("scripts");

        let script_doc_dir = &env::var("SCRIPT_DOC_DIR")
            .map(|v| v.into())
            .unwrap_or_else(|_e| script_asset_path.join("doc"));

        fs::create_dir_all(script_doc_dir)
            .expect("Could not create `.../assets/scripts/doc` directories");

        let docs_name = self.name().to_owned();

        // build the type walker
        let mut tw = self
            .walker
            .into_iter()
            .fold(TypeWalker::new(), |a, v| (v.builder)(a));

        // fixes bug in tealr which causes syntax errors in teal due to duplicate fields (from having both getters and setters)
        tw.given_types.iter_mut().for_each(|tg| {
            if let TypeGenerator::Record(rg) = tg {
                rg.fields.dedup_by(|a, b| a.name == b.name);
            }
        });

        // generate json file
        let json = serde_json::to_string_pretty(&tw)
            .map_err(|e| ScriptError::DocGenError(e.to_string()))?;

        // temporary fix for incompatibility in json formats
        // json.remove(json.len() - 1);
        // json.push_str(",\n\"tealr_version_used\": \"0.9.0-alpha3\",\n\"extra_page\": []\n}");

        let json_path = script_doc_dir.join(format!("{}.json", docs_name));

        File::create(json_path)
            .and_then(|mut file| {
                file.write_all(json.as_bytes())?;
                file.flush()
            })
            .map_err(|e| ScriptError::DocGenError(e.to_string()))?;

        // generate doc config files if they don't exist
        if !script_doc_dir.join("tealr_doc_gen_config.json").exists() {
            let config_path = script_doc_dir.join("tealr_doc_gen_config.json");
            File::create(config_path)
                .and_then(|mut file| file.write_all(DEFAULT_DOC_CONFIG(&docs_name).as_bytes()))
                .map_err(|e| ScriptError::DocGenError(e.to_string()))?;
        }

        // generate docs
        Command::new("tealr_doc_gen")
            .current_dir(script_doc_dir)
            .args(["run"])
            .status()
            .map_err(|e| ScriptError::DocGenError(e.to_string()))?;

        #[cfg(feature = "teal")]
        {
            // now manage the definition (d.tl) file
            let definition_directory = script_asset_path.join("types");
            fs::create_dir_all(&definition_directory).map_err(|e| {
                ScriptError::DocGenError(format!(
                    "Could not create `{}` directories: {e}",
                    &definition_directory.display()
                ))
            })?;

            let definition_file_path = script_doc_dir
                .join(&docs_name)
                .join("definitions")
                .join(docs_name + ".d.tl");
            let output_definition_file_path = script_asset_path.join("types").join("types.d.tl");
            fs::copy(&definition_file_path, &output_definition_file_path).map_err(|e| {
                ScriptError::DocGenError(format!(
                    "Could not copy definition file from `{}` to `{}`: {e}",
                    definition_file_path.display(),
                    output_definition_file_path.display()
                ))
            })?;

            // finally create a tlconfig.lua file if doesn't exist
            // we do this to avoid problems with varying teal configurations
            // keep em settings consistent everywhere
            let tl_config_path = script_asset_path.join("tlconfig.lua");
            if !tl_config_path.exists() {
                let mut tl_file = File::create(tl_config_path)
                    .map_err(|e| ScriptError::DocGenError(e.to_string()))?;
                tl_file
                    .write_all(DEFAULT_TEAL_CONFIG.as_bytes())
                    .map_err(|e| ScriptError::DocGenError(e.to_string()))?;
            }
        }
        Ok(())
    }
}
