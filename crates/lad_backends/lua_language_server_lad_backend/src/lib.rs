//! Logic for generating Lua language server files from a LAD file.

use std::path::Path;

use anyhow::Context;

use crate::{convert::convert_ladfile_to_lua_declaration_files, templating::render_template};

mod convert;
mod keywords;
mod lua_declaration_file;
mod plugin;
mod templating;
pub use plugin::*;

/// Processess a LAD file and generates Lua language server files.
///
/// The builtins and the root namespace go to `file_name`, every other namespace to a file of its own next to it,
/// i.e. `bindings.math.d.lua` for `bindings.d.lua`. Point the language server at the whole directory.
pub fn generate_lua_language_server_files(
    ladfile: &ladfile::LadFile,
    output_dir: &Path,
    file_name: &Path,
) -> Result<(), anyhow::Error> {
    let file_name = file_name.to_string_lossy();
    for (namespace, declaration_file) in convert_ladfile_to_lua_declaration_files(ladfile)? {
        let output_path = output_dir.join(namespace_file_name(&file_name, &namespace));
        std::fs::create_dir_all(
            output_path
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Output path has no parent"))?,
        )
        .with_context(|| "failed to create output directories")?;
        let context = tera::Context::from_serialize(&declaration_file).with_context(|| {
            format!(
                "Failed to serialize LuaModule for template rendering: {}",
                output_path.display()
            )
        })?;

        let rendered = render_template("declaration_file.tera", &context)?;
        std::fs::write(&output_path, rendered).with_context(|| {
            format!(
                "Failed to write rendered template to file: {}",
                output_path.display()
            )
        })?;
    }
    Ok(())
}

/// The file the declarations of a namespace go to: `file_name` for the root namespace, otherwise the namespace inserted before its extensions,
/// i.e. `bindings.math.d.lua`.
pub fn namespace_file_name(file_name: &str, namespace: &str) -> String {
    if namespace.is_empty() {
        return file_name.to_owned();
    }
    match file_name.split_once('.') {
        Some((stem, extensions)) => format!("{stem}.{namespace}.{extensions}"),
        None => format!("{file_name}.{namespace}"),
    }
}
