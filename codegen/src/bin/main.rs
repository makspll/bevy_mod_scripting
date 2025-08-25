#![feature(rustc_private)]
use std::{
    collections::HashMap,
    env,
    fs::{File, create_dir_all},
    io::{BufRead, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use bevy_mod_scripting_codegen::{driver::*, *};
use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use crate_feature_graph::{Workspace, WorkspaceGraph};
use log::{debug, error, info};
use strum::VariantNames;
use tera::Context;

const BOOTSTRAP_DEPS: [&str; 2] = ["bevy_reflect", "bevy_mod_scripting_core"];

fn main() {
    // parse this here to early exit on wrong args
    let args = Args::parse_from(env::args().skip(1));

    if env::var("RUST_LOG").is_err() {
        unsafe { env::set_var("RUST_LOG", args.verbose.get_rustlog_value()) };
    }
    pretty_env_logger::init();

    info!("Using RUST_LOG: {:?}", env::var("RUST_LOG"));

    info!("Computing crate metadata");
    let metadata = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .other_options(["--all-features".to_string(), "--offline".to_string()])
        .exec()
        .unwrap();

    let crates = metadata
        .workspace_packages()
        .iter()
        .map(|p| p.name.to_owned())
        .collect::<Vec<_>>();

    info!("Computing active features");
    let include_crates = if args.cmd.is_generate() {
        let workspace = Workspace::from(&metadata);
        let mut graph = WorkspaceGraph::from(workspace);
        info!("Using workspace graph: \n{}", graph.to_dot());

        info!(
            "Computing all transitive dependencies for enabled top-level features: {}. using default features: {}",
            args.features.join(","),
            !args.no_default_features
        );

        graph.calculate_enabled_features_and_dependencies_parse(args.features, None);

        let mut dependencies = graph
            .workspace
            .workspace_crates
            .iter()
            .filter(|krate| krate.is_enabled.unwrap_or(false))
            .map(|krate| krate.name.to_string())
            .collect::<Vec<_>>();

        // log all dependencies
        debug!("Enabled dependencies: {}", dependencies.join(","));

        if let Some(excluded_crates) = &args.exclude_crates {
            dependencies.retain(|c| !excluded_crates.contains(c));
            info!("Excluding crates: {excluded_crates:?}");
        }

        let graph_path =
            PathBuf::from(fetch_target_directory(&metadata).join("workspace_graph.dot"));
        graph.serialize(&graph_path).unwrap();
        info!("Serialized workspace graph to: {}", graph_path.display());
        unsafe { std::env::set_var(WORKSPACE_GRAPH_FILE_ENV, graph_path) };

        Some(dependencies)
    } else {
        None
    };

    let plugin_subdir = format!("plugin-{}", env!("RUSTC_CHANNEL"));
    let plugin_target_dir = metadata.target_directory.join(plugin_subdir);

    info!("Computing workspace metadata");

    // inform the deps about the workspace crates, this is going to be useful when working with meta files as we will be able to
    // know when to panic if a crate is not found
    // it's also useful to pass around the output directory for our Args default values to be able to compute them
    let workspace_meta = WorkspaceMeta {
        crates,
        plugin_target_dir: plugin_target_dir.clone(),
        include_crates,
    };
    workspace_meta.set_env();

    match args.cmd {
        bevy_mod_scripting_codegen::Command::Print { template } => {
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
        bevy_mod_scripting_codegen::Command::ListTemplates => {
            for template in TemplateKind::VARIANTS {
                println!("{template}");
            }
            return;
        }
        bevy_mod_scripting_codegen::Command::Collect {
            output,
            templates,
            api_name,
        } => {
            let tera = configure_tera("no_crate", &templates);
            info!("Collecting from: {output}");
            if !output.is_dir() {
                panic!("Output is not a directory");
            }

            let meta_loader = MetaLoader::new(vec![output.to_owned()], workspace_meta);
            let mut crates = meta_loader
                .iter_meta()
                .filter_map(|m| {
                    log::debug!(
                        "Processing crate: {}, will generate: {}",
                        m.crate_name(),
                        m.will_generate()
                    );
                    m.will_generate().then_some(Crate {
                        name: m.crate_name().to_owned(),
                        meta: m,
                    })
                })
                .collect::<Vec<_>>();

            crates.sort_by(|a, b| a.name.cmp(&b.name));

            let json = serde_json::to_string_pretty(
                &crates.iter().map(|c| c.name.clone()).collect::<Vec<_>>(),
            )
            .unwrap();

            let collect = Collect { crates, api_name };

            let mut context =
                Context::from_serialize(collect).expect("Could not create template context");

            extend_context_with_args(args.template_args.as_deref(), &mut context);

            let mut file = File::create(output.join("mod.rs")).unwrap();
            tera.render_to(&TemplateKind::SharedModule.to_string(), &context, &mut file)
                .expect("Failed to render mod.rs");
            file.flush().unwrap();
            log::info!("Succesfully generated mod.rs");

            // put json of Collect context into stdout
            std::io::stdout()
                .write_all(json.as_bytes())
                .expect("Failed to write Collect context to stdout");
            return;
        }
        _ => {}
    }

    let temp_dir = find_bootstrap_dir();

    debug!("Bootstrap directory: {}", &temp_dir.as_path().display());

    write_bootstrap_files(args.bms_core_path, temp_dir.as_path());

    let bootstrap_rlibs = build_bootstrap(temp_dir.as_path(), &plugin_target_dir.join("bootstrap"));

    if bootstrap_rlibs.len() == BOOTSTRAP_DEPS.len() {
        let extern_args = bootstrap_rlibs
            .iter()
            .map(|(key, val)| format!("--extern {key}={val}",))
            .collect::<Vec<_>>()
            .join(" ");

        debug!("bootstrap paths: {bootstrap_rlibs:?}");
        unsafe {
            env::set_var(
                "RUSTFLAGS",
                format!(
                    "{} {} -L dependency={}",
                    env::var("RUSTFLAGS").unwrap_or("".to_owned()),
                    extern_args,
                    bootstrap_rlibs.iter().next().unwrap().1.parent().unwrap()
                ),
            )
        };
    } else {
        panic!("Could not find 'libmlua' artifact among bootstrap crate artifacts, stopping.");
    }

    debug!("Running bevy_api_gen main cargo command");

    debug!("RUSTFLAGS={}", env::var("RUSTFLAGS").unwrap_or_default());

    // disable incremental compilation
    unsafe { env::set_var("CARGO_INCREMENTAL", "0") };

    driver::cli_main(
        BevyAnalyzer,
        workspace_meta.include_crates.unwrap_or_default(),
        &metadata,
    );

    // just making sure the temp dir lives until everything is done
    drop(temp_dir);
}

/// Build bootstrap files if they don't exist
/// use cached ones otherwise
fn build_bootstrap(
    temp_dir: &Path,
    cache_dir: &Utf8Path,
) -> HashMap<String, cargo_metadata::camino::Utf8PathBuf> {
    debug!("Building bootstrapping crate");

    // first check cache
    if cache_dir.exists() {
        let mut bootstrap_rlibs = HashMap::with_capacity(BOOTSTRAP_DEPS.len());
        for entry in std::fs::read_dir(cache_dir).unwrap() {
            let entry = entry.unwrap();
            let artifact = entry.path();
            process_artifact(artifact.try_into().unwrap(), &mut bootstrap_rlibs);
        }
        return bootstrap_rlibs;
    }

    // run build command
    let mut cmd = Command::new("cargo")
        .current_dir(temp_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(["build", "--message-format=json"])
        .spawn()
        .unwrap();

    info!(
        "cd {} && cargo build --message-format=json",
        temp_dir.display()
    );

    let reader = std::io::BufReader::new(cmd.stdout.take().unwrap());
    let err_reader = std::io::BufReader::new(cmd.stderr.take().unwrap());

    std::fs::create_dir_all(cache_dir).unwrap();

    let mut bootstrap_rlibs = HashMap::with_capacity(BOOTSTRAP_DEPS.len());
    for msg in cargo_metadata::Message::parse_stream(reader) {
        let msg = msg.unwrap();
        if let cargo_metadata::Message::CompilerArtifact(artifact) = msg {
            for artifact in artifact.filenames.into_iter() {
                process_artifact(artifact, &mut bootstrap_rlibs);
            }
        } else {
            match msg {
                cargo_metadata::Message::BuildFinished(finished) => {
                    if !finished.success {
                        error!("Bootstrapping crate failed to build artifact");
                    }
                }
                cargo_metadata::Message::TextLine(t) => {
                    info!("{t}");
                }
                cargo_metadata::Message::CompilerMessage(msg) => {
                    info!("{msg}");
                }
                _ => {}
            }
        }
    }
    for msg in err_reader.lines() {
        if let Ok(line) = msg {
            info!("{line}");
        } else {
            panic!("Failed to read cargo stderr");
        }
    }

    // cache bootstrap artifacts
    if let Some(artifact) = bootstrap_rlibs.values().next() {
        let deps_dir = artifact.parent().unwrap();

        for dir in std::fs::read_dir(deps_dir).unwrap() {
            let dir = dir.unwrap();
            let path = dir.path();

            let dest = cache_dir.join(path.file_name().unwrap().to_str().unwrap());
            std::fs::copy(path, dest).unwrap();
        }
    }
    match cmd.wait() {
        Ok(status) => {
            if !status.success() {
                panic!("Building bootstrap crate returned a failure status code");
            }
        }
        Err(e) => {
            panic!("Failed to wait on cargo build process: {e}");
        }
    }

    bootstrap_rlibs
}

/// Process artifact and add it to the bootstrap rlibs if it's is for a bootstrap dependency and an rlib
fn process_artifact(
    artifact: cargo_metadata::camino::Utf8PathBuf,
    bootstrap_rlibs: &mut HashMap<String, cargo_metadata::camino::Utf8PathBuf>,
) {
    let file_name = artifact.file_name().unwrap_or_default();
    let lib_name = file_name.split('-').next().unwrap().strip_prefix("lib");

    if let Some(lib_name) = lib_name
        && BOOTSTRAP_DEPS.contains(&lib_name)
        && artifact.extension().is_some_and(|ext| ext == "rlib")
    {
        bootstrap_rlibs.insert(lib_name.to_owned(), artifact);
    }
}

/// finds best location for bootstrapping crate
/// this will be the nearest target/bms_codegen_bootstrap directory
fn find_bootstrap_dir() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    loop {
        if path.join("target").exists() {
            break;
        } else if let Some(parent) = path.parent() {
            path = parent.to_path_buf();
        } else {
            panic!("Could not find `target` directory");
        }
    }

    path.push("target");
    path.push("bms_codegen_bootstrap");

    // create all the directories
    create_dir_all(&path).unwrap();
    path
}

/// Generate bootstrapping crate files
fn write_bootstrap_files(bms_core_path: Utf8PathBuf, path: &Path) {
    const BMS_CORE_PATH_PLACEHOLDER: &str = "{{BMS_CORE_PATH}}";

    // write manifest file 'Cargo.toml'
    let mut manifest_content =
        String::from_utf8(include_bytes!("../../Cargo.bootstrap.toml").to_vec())
            .expect("Could not read manifest template as utf8");

    manifest_content = manifest_content.replace(BMS_CORE_PATH_PLACEHOLDER, bms_core_path.as_str());

    let manifest_path = path.join("Cargo.toml");

    let mut file = File::create(manifest_path)
        .expect("Could not create manifest file for bootstrapping crate.");
    file.write_all(manifest_content.as_bytes())
        .expect("Failed writing to manifest file for bootstrapping crate");

    // write dummy main function

    let mut main_file_path = path.join("src");

    create_dir_all(&main_file_path).unwrap();

    main_file_path.push("main.rs");

    let mut file = File::create(&main_file_path).unwrap();
    file.write_all(b"fn main(){}").unwrap();
}
