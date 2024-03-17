use std::{
    collections::HashMap,
    env,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use bevy_analyzer::*;
use log::{debug, trace};
use rustc_plugin::RustcPlugin;

const BOOTSTRAP_DEPS: [&str; 2] = ["mlua", "bevy_reflect"];

fn main() {
    env_logger::init();
    debug!("CLI entrypoint");
    debug!("Creating bootstrapping crate");

    let temp_dir = tempdir::TempDir::new("bevy_analyzer_bootstrap")
        .expect("Error occured when trying to acquire temp file");

    debug!("Temporary directory: {}", &temp_dir.path().display());

    write_bootstrap_files(temp_dir.path());

    debug!("Building bootstrapping crate");

    let mut cmd = Command::new("cargo")
        .current_dir(&temp_dir)
        .stdout(Stdio::piped())
        .args(["build", "--message-format=json"])
        .spawn()
        .unwrap();

    let reader = std::io::BufReader::new(cmd.stdout.take().unwrap());

    let mut bootstrap_rlibs = HashMap::with_capacity(BOOTSTRAP_DEPS.len());
    for msg in cargo_metadata::Message::parse_stream(reader) {
        if let cargo_metadata::Message::CompilerArtifact(artifact) = msg.unwrap() {
            trace!(
                "produced artifacts for: {}, at: {:?}",
                artifact.package_id,
                artifact.filenames
            );

            for artifact in artifact.filenames.into_iter() {
                let file_name = artifact.file_name().unwrap_or_default();
                let lib_name = file_name.split('-').next().unwrap().strip_prefix("lib");

                if let Some(lib_name) = lib_name {
                    if BOOTSTRAP_DEPS.contains(&lib_name)
                        && artifact.extension().is_some_and(|ext| ext == "rlib")
                    {
                        bootstrap_rlibs.insert(lib_name.to_owned(), artifact);
                    }
                }
            }
        }
    }

    if !cmd.wait().unwrap().success() {
        panic!("Building bootstrap crate returned a failure status code");
    };

    if bootstrap_rlibs.len() == BOOTSTRAP_DEPS.len() {
        let extern_args = bootstrap_rlibs
            .iter()
            .map(|(key, val)| format!("--extern {key}={val}",))
            .collect::<Vec<_>>()
            .join(" ");

        debug!("bootstrap paths: {bootstrap_rlibs:?}");
        env::set_var(
            "RUSTFLAGS",
            format!(
                "{} {} -L dependency={}",
                env::var("RUSTFLAGS").unwrap_or("".to_owned()),
                extern_args,
                bootstrap_rlibs.iter().next().unwrap().1.parent().unwrap()
            ),
        );
    } else {
        panic!("Could not find 'libmlua' artifact among bootstrap crate artifacts, stopping.");
    }

    debug!("Running bevy_analyzer main cargo command");

    debug!("RUSTFLAGS={}", env::var("RUSTFLAGS").unwrap_or_default());

    rustc_plugin::cli_main(BevyAnalyzer);

    // just making sure the temp dir lives until everything is done
    drop(temp_dir);
}

/// Generate bootstrapping crate files
fn write_bootstrap_files(path: &Path) {
    // write manifest file 'Cargo.toml'
    let manifest_content = include_bytes!("../Cargo.bootstrap.toml");
    let manifest_path = path.join("Cargo.toml");

    let mut file = File::create(manifest_path)
        .expect("Could not create manifest file for bootstrapping crate.");
    file.write_all(manifest_content)
        .expect("Failed writing to manifest file for bootstrapping crate");

    // write dummy main function

    let mut main_file_path = path.join("src");

    create_dir_all(&main_file_path).unwrap();

    main_file_path.push("main.rs");

    let mut file = File::create(&main_file_path).unwrap();
    file.write_all(b"fn main(){}").unwrap();
}
