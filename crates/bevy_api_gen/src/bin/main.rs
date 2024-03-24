use std::{
    collections::HashMap,
    env,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use bevy_api_gen::*;
use clap::Parser;
use log::{debug, info, trace};
use strum::VariantNames;
use tera::Context;

const BOOTSTRAP_DEPS: [&str; 2] = ["mlua", "bevy_reflect"];

fn main() {
    env_logger::init();
    debug!("CLI entrypoint");
    debug!("Creating bootstrapping crate");

    let metadata = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .other_options(["--all-features".to_string(), "--offline".to_string()])
        .exec()
        .unwrap();
    let plugin_subdir = format!("plugin-{}", env!("RUSTC_CHANNEL"));
    let target_dir = metadata.target_directory.join(plugin_subdir);
    env::set_var(TARGET_DIR_ENV_NAME, target_dir);

    // parse this here to early exit on wrong args
    let args = Args::parse_from(env::args().skip(1));

    if env::var("RUST_LOG").is_err() {
        env::set_var(
            "RUST_LOG",
            match (args.verbose.verbose as isize) - (args.verbose.quiet as isize) {
                0 => "info",
                1 => "debug",
                x if x >= 2 => "trace",
                _ => "error",
            },
        );
    }

    match args.cmd {
        bevy_api_gen::Command::Print { template } => {
            println!(
                "{}",
                TEMPLATE_DIR
                    .get_file(template.to_string())
                    .unwrap()
                    .contents_utf8()
                    .unwrap()
            );
            return;
        }
        bevy_api_gen::Command::ListTemplates => {
            for template in TemplateKind::VARIANTS {
                println!("{}", template);
            }
            return;
        }
        bevy_api_gen::Command::Collect {
            output,
            templates,
            api_name,
        } => {
            let tera = configure_tera("no_crate", &templates);
            info!("Collecting from: {}", output);
            if !output.is_dir() {
                panic!("Output is not a directory");
            }
            let crates = std::fs::read_dir(&output)
                .expect("Could not read output directory")
                .filter_map(|d| {
                    let entry = d.expect("Could not read entry in output directory");
                    let path = entry.path();
                    if path.extension().is_some_and(|ext| ext == "rs")
                        && path.file_stem().is_some_and(|s| s != "mod")
                    {
                        Some(path.file_stem().unwrap().to_owned())
                    } else {
                        None
                    }
                });
            let meta_loader = MetaLoader::new(vec![output.to_owned()]);
            let context = Collect {
                crates: crates
                    .map(|c| {
                        let name = c.to_str().unwrap().to_owned();
                        log::info!("Collecting crate: {}", name);
                        let meta = meta_loader
                            .meta_for(&name)
                            .expect("Could not find meta file for crate");
                        Crate { name, meta }
                    })
                    .collect(),
                api_name,
            };
            let context =
                Context::from_serialize(context).expect("Could not create template context");
            let file = File::create(output.join("mod.rs")).unwrap();
            tera.render_to(&TemplateKind::SharedModule.to_string(), &context, file)
                .expect("Failed to render mod.rs");
            log::info!("Succesfully generated mod.rs");
            return;
        }
        _ => {}
    }

    let temp_dir = tempdir::TempDir::new("bevy_api_gen_bootstrap")
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

    debug!("Running bevy_api_gen main cargo command");

    debug!("RUSTFLAGS={}", env::var("RUSTFLAGS").unwrap_or_default());

    rustc_plugin::cli_main(BevyAnalyzer);

    // just making sure the temp dir lives until everything is done
    drop(temp_dir);
}

/// Generate bootstrapping crate files
fn write_bootstrap_files(path: &Path) {
    // write manifest file 'Cargo.toml'
    let manifest_content = include_bytes!("../../Cargo.bootstrap.toml");
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
