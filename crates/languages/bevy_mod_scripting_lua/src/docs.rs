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
    "is_global": true,
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

pub struct LuaDocumentationFragment {
    name: &'static str,
    walker: Vec<Fragment>,
}

/// A piece of lua documentation,
/// Each piece is combined into one large documentation page, and also a single teal declaration file if the `teal` feature is enabled
impl LuaDocumentationFragment {
    pub fn new(name: &'static str, f: TypeWalkerBuilder) -> Self {
        Self {
            name,
            walker: vec![Fragment { builder: f }],
        }
    }
}

impl DocumentationFragment for LuaDocumentationFragment {
    fn name(&self) -> &'static str {
        self.name
    }

    fn merge(mut self, o: Self) -> Self {
        self.walker.extend(o.walker);
        self
    }

    fn gen_docs(self) -> Result<(), Box<dyn std::error::Error>> {
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
                rg.fields
                    .sort_by(|f1, f2| f1.name.deref().cmp(f2.name.deref()));
                rg.fields.dedup_by(|a, b| a.name == b.name);
                rg.static_fields
                    .sort_by(|f1, f2| f1.name.deref().cmp(f2.name.deref()));
                rg.static_fields.dedup_by(|a, b| a.name == b.name);
                for field in rg.fields.iter_mut().chain(rg.static_fields.iter_mut()) {
                    escape_name(&mut field.name);
                }
                for func in rg
                    .functions
                    .iter_mut()
                    .chain(rg.mut_functions.iter_mut())
                    .chain(rg.methods.iter_mut())
                    .chain(rg.mut_methods.iter_mut())
                {
                    escape_name(&mut func.name);
                }
            }
        });

        // generate json file
        let json = serde_json::to_string_pretty(&tw)?;

        let json_path = script_doc_dir.join(format!("{}.json", docs_name));

        (File::create(json_path).and_then(|mut file| {
            file.write_all(json.as_bytes())?;
            file.flush()
        }))?;
        // generate doc config files if they don't exist
        if !script_doc_dir.join("tealr_doc_gen_config.json").exists() {
            let config_path = script_doc_dir.join("tealr_doc_gen_config.json");
            (File::create(config_path)
                .and_then(|mut file| file.write_all(DEFAULT_DOC_CONFIG(&docs_name).as_bytes())))?
        }

        // generate docs
        (Command::new("tealr_doc_gen")
            .current_dir(script_doc_dir)
            .args(["run"])
            .status())?;

        #[cfg(feature = "teal")]
        {
            // now manage the definition (d.tl) file
            let definition_directory = script_asset_path.join("types");
            (fs::create_dir_all(&definition_directory))?;

            let definition_file_path = script_doc_dir
                .join(&docs_name)
                .join("definitions")
                .join(docs_name + ".d.tl");
            let output_definition_file_path = script_asset_path.join("types").join("types.d.tl");
            (fs::copy(&definition_file_path, &output_definition_file_path))?;

            // finally create a tlconfig.lua file if doesn't exist
            // we do this to avoid problems with varying teal configurations
            // keep em settings consistent everywhere
            let tl_config_path = script_asset_path.join("tlconfig.lua");
            if !tl_config_path.exists() {
                let mut tl_file = (File::create(tl_config_path))?;
                (tl_file.write_all(DEFAULT_TEAL_CONFIG.as_bytes()))?;
            }
        }
        Ok(())
    }
}

/// Escapes a name of a table field, if that table field is a reserved keyword.
///
/// ## Background
///
/// String keys in a Lua table are allowed to be anything, even reserved
/// keywords. By default when tealr generates the type definition for a table
/// field, the string it generates is `{name} : {type}`. This causes a syntax
/// error when writing a bare keyword, since `nil : {type}` is considered trying
/// to add a type to the *value* nil (which is invalid).
///
/// To get around this tealr allows us to escape table fields using the
/// `["{name}"] : {value}` syntax. This function detects if a name is one of the
/// Lua reserved words and fixes it if so.
fn escape_name(raw: &mut NameContainer) {
    // List of Lua reserved keywords
    const KEYWORD_FIELDS: &[&str] = &[
        "false", "true", "nil", // Values
        "and", "not", "or", // Operators
        "if", "then", "else", "elseif", "end", // If-Else
        "for", "in", "break", "do", "repeat", "until", "while", // Loops
        "function", "return", // Funcs
        "local",  // Declarations
        "record", // Teal extra
    ];
    let Ok(name) = str::from_utf8(raw) else {
        return;
    };
    if KEYWORD_FIELDS.contains(&name) {
        let mapped = format!("[\"{name}\"]");
        *raw = NameContainer::from(Cow::Owned(mapped));
    }
}
