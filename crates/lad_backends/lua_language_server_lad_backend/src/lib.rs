//! Logic for generating Lua language server files from a LAD file.

use std::path::Path;

use anyhow::Context;

use crate::{convert::convert_ladfile_to_lua_declaration_file, templating::render_template};

mod convert;
mod keywords;
mod lua_declaration_file;
mod plugin;
mod templating;
pub use plugin::*;

/// Processess a LAD file and generates Lua language server files.
pub fn generate_lua_language_server_files(
    ladfile: &ladfile::LadFile,
    output_dir: &Path,
    file_name: &Path,
) -> Result<(), anyhow::Error> {
    let declaration_file = convert_ladfile_to_lua_declaration_file(ladfile)?;

    let output_path = output_dir.join(file_name);
    std::fs::create_dir_all(
        output_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Output path has no parent"))?,
    )
    .with_context(|| "failed to create output directories")?;
    let context = tera::Context::from_serialize(&declaration_file).with_context(|| {
        format!(
            "Failed to serialize LuaModule for template rendering: {}",
            file_name.as_os_str().display()
        )
    })?;

    let rendered = render_template("declaration_file.tera", &context)?;
    std::fs::write(&output_path, rendered).with_context(|| {
        format!(
            "Failed to write rendered template to file: {}",
            output_path.display()
        )
    })?;
    Ok(())
}
