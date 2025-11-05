use std::{
    error::Error,
    path::{Path, PathBuf},
};

use ladfile::LadFilePlugin;

use crate::generate_lua_language_server_files;

/// A plugin which generates lua language definition files to a specified directory when run
#[derive(Clone)]
pub struct LuaLanguageServerLadPlugin {
    /// The filename of the generated definition file
    pub filename: PathBuf,
}

impl Default for LuaLanguageServerLadPlugin {
    fn default() -> Self {
        Self {
            filename: PathBuf::from("bindings.lua"),
        }
    }
}

impl LadFilePlugin for LuaLanguageServerLadPlugin {
    fn run(&self, ladfile: &ladfile::LadFile, path: &Path) -> Result<(), Box<dyn Error>> {
        generate_lua_language_server_files(ladfile, path, &self.filename)
            .map_err(|e| e.into_boxed_dyn_error() as Box<dyn Error>)
    }

    fn name(&self) -> &'static str {
        "Lua language server definition file generator"
    }
}
