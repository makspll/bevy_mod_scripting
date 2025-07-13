//! Language Agnostic Declaration (LAD) file format post processor for generating Lua language server files for the bevy_mod_scripting crate.

use std::path::PathBuf;

use clap::Parser;
use lua_language_server_lad_backend::generate_lua_language_server_files;

#[derive(Debug, clap::Parser)]
/// Command line arguments for the Lua Language Server LAD backend.
pub struct Args {
    /// Input LAD file path
    #[clap(short, long, help = "LAD json input file")]
    pub input: String,

    /// Output directory for the generated Lua language server files
    #[clap(
        short,
        long,
        help = "Output directory for the generated Lua language server files, will generate multiple files"
    )]
    pub output: PathBuf,
}
fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    // Initialize the logger
    env_logger::init();

    // Log the input and output paths
    log::info!("Input LAD file: {}", args.input);
    log::info!("Output directory: {:?}", args.output);

    // Load the LAD file
    let file = std::fs::read_to_string(&args.input)
        .map_err(|e| anyhow::anyhow!("Failed to read LAD file {}: {}", args.input, e))?;
    let ladfile = ladfile::parse_lad_file(&file)?;

    generate_lua_language_server_files(ladfile, &args.output)?;
    Ok(())
}
