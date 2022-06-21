use std::{string::FromUtf8Error, env, path::PathBuf, process::Command, fs::{File, self}, io::Write};

use tealr::TypeWalker;

use crate::{DocFragment, ScriptError};


struct Fragment{
    walker : TypeWalker,
    is_global: bool,
    outer_name: String,
}

impl Fragment {
    fn generate(self) -> Result<String,FromUtf8Error>{
        self.walker.generate(&self.outer_name, self.is_global)
    }
}

pub struct LuaDocFragment {
    walker: Vec<Fragment>
}

impl LuaDocFragment{
    pub fn new(walker : TypeWalker, is_global: bool, outer_name: String) -> Self{
        Self {
            walker: vec![Fragment{ walker, is_global, outer_name }],
        }
    }
}

impl DocFragment for LuaDocFragment {
    fn merge(mut self, o : Self) -> Self {
        self.walker.extend(o.walker.into_iter());
        self
    }

    fn gen_docs(self) -> Result<(),ScriptError>{
        // asset path: FileAssetIo::get_root_path
        self.walker.into_iter().for_each(|v|{

            let json = serde_json::to_string_pretty(&v.walker).expect("Failed to serialize TypeWalker");
            let tdl_file = v.generate().unwrap();

            let target_dir = env::var("TARGET_DIR").expect("TARGET_DIR is not set");
            let root_dir = &PathBuf::from(target_dir).join("script_docs");
            let temp_dir = &root_dir.join("temp.json");
            fs::create_dir_all(root_dir).expect("Could not create script_docs directory");

            let mut json_file = File::create(&temp_dir).expect("Could not create temporary json file");
            json_file.write_all(json.as_bytes()).expect("Failed to write json to temporary file");

            Command::new("tealr_doc_gen")
                .args(["--json",
                    temp_dir.to_str().unwrap(),
                    "--name",
                    "LuaDoc",
                    // "--build_folder",
                    // fs::canonicalize(&root_dir).unwrap().to_str().unwrap()
                ])
                .status()
                .expect("Failed to generate documentation");

        });

        Ok(())
    }
}