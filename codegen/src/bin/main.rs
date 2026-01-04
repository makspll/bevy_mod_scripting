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
use cargo_metadata::camino::Utf8PathBuf;
use clap::Parser;
use crate_feature_graph::{Crate, Workspace, WorkspaceGraph};
use log::{debug, error};
use strum::VariantNames;
use tera::Context;

const BOOTSTRAP_DEPS: [&str; 2] = ["bevy_reflect", "bevy_mod_scripting_bindings"];
// const INJECT_DEPS: [&str; 1] = ["bevy_mod_scripting_bindings"];

fn main() {
    // parse this here to early exit on wrong args
    let args = Args::parse_from(env::args().skip(1));

    if env::var("RUST_LOG").is_err() {
        unsafe { env::set_var("RUST_LOG", args.verbose.get_rustlog_value()) };
    }
    pretty_env_logger::init();

    debug!("Using RUST_LOG: {:?}", env::var("RUST_LOG"));

    debug!(
        "MSRV target: {}",
        args.mrsv_target()
            .map(|t| t.to_string())
            .unwrap_or(String::from("unset"))
    );

    debug!("Computing crate metadata");
    let metadata = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .other_options(["--all-features".to_string(), "--offline".to_string()])
        .exec()
        .unwrap();

    let root_crate = &metadata.root_package().unwrap().name;

    let crates = metadata
        .workspace_packages()
        .iter()
        .map(|p| p.name.to_owned())
        .collect::<Vec<_>>();

    debug!("Computing active features");
    let include_crates = if !args.cmd.is_collect() {
        let workspace = Workspace::from(&metadata);
        let mut graph = WorkspaceGraph::from(workspace);
        debug!("Using workspace graph: \n{}", graph.to_dot());

        debug!(
            "Computing all transitive dependencies for enabled top-level features: {}. using default features: {}",
            args.features.join(","),
            !args.no_default_features
        );

        graph.calculate_enabled_features_and_dependencies_parse(args.features.clone(), None);

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
            debug!("Excluding crates: {excluded_crates:?}");
        }

        let graph_path =
            PathBuf::from(fetch_target_directory(&metadata).join("workspace_graph.dot"));
        graph.serialize(&graph_path).unwrap();
        debug!("Serialized workspace graph to: {}", graph_path.display());
        unsafe { std::env::set_var(WORKSPACE_GRAPH_FILE_ENV, graph_path) };

        Some(dependencies)
    } else {
        None
    };

    let plugin_subdir = format!("plugin-{}", env!("RUSTC_CHANNEL"));
    let plugin_target_dir = metadata.target_directory.join(plugin_subdir);

    debug!("Computing workspace metadata");

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
            debug!("Collecting from: {output}");
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
            log::debug!("Succesfully generated mod.rs");

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

    write_bootstrap_files(
        args.bms_bindings_path,
        root_crate,
        &PathBuf::from("."),
        args.features.iter().map(|s| s.as_str()),
        temp_dir.as_path(),
    );

    let bootstrap_rlibs = build_bootstrap(temp_dir.as_path());

    debug!("Running bevy_api_gen main cargo command");

    // disable incremental compilation
    unsafe { env::set_var("CARGO_INCREMENTAL", "0") };

    let analyzer = BevyAnalyzer {
        args: crate::Args::parse_from(std::env::args().skip(1)),
        payload: Payload {
            bootstrap_rlibs: bootstrap_rlibs
                .into_iter()
                .map(|(a, b)| (a, b.to_string()))
                .collect(),
            include_crates: workspace_meta.include_crates.unwrap_or_default(),
            bootstrap_deps_path: temp_dir
                .join("target")
                .join("debug")
                .join("deps")
                .to_str()
                .unwrap()
                .to_string(),
        },
    };

    driver::cli_main(analyzer, &metadata);

    // just making sure the temp dir lives until everything is done
    drop(temp_dir);
}

/// Build bootstrap files if they don't exist
/// use cached ones otherwise
fn build_bootstrap(temp_dir: &Path) -> HashMap<String, cargo_metadata::camino::Utf8PathBuf> {
    debug!("Building bootstrapping crate");

    // run build command
    let mut cmd = Command::new("cargo")
        .current_dir(temp_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(["build", "--message-format=json"])
        .spawn()
        .unwrap();

    debug!(
        "cd {} && cargo build --message-format=json",
        temp_dir.display()
    );

    let reader = std::io::BufReader::new(cmd.stdout.take().unwrap());
    let err_reader = std::io::BufReader::new(cmd.stderr.take().unwrap());

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
                    debug!("{t}");
                }
                cargo_metadata::Message::CompilerMessage(msg) => {
                    debug!("{msg}");
                }
                _ => {}
            }
        }
    }

    let fail = match cmd.wait() {
        Ok(status) => {
            if !status.success() {
                true
                // panic!("Building bootstrap crate returned a failure status code, {}",);
            } else {
                false
            }
        }
        Err(_) => {
            // panic!("Failed to wait on cargo build process: {e}");
            true
        }
    };

    for msg in err_reader.lines() {
        if let Ok(line) = msg {
            if fail {
                println!("{line}");
            }
        } else {
            panic!("Failed to read cargo stderr");
        }
    }

    if fail {
        panic!("failed to build bootstrap crate");
    }

    let include_list: [&'static str; 13] = [
        "bevy_reflect",
        "bevy_mod_scripting_bindings",
        "bevy_ptr",
        "bevy_utils",
        "bevy_reflect_derive",
        "bevy_platform",
        "bevy_utils",
        "serde",
        "serde_core",
        "glam",
        "uuid",
        "smallvec",
        "hashbrown",
    ];
    // for c in exclude_list {
    //     bootstrap_rlibs.remove(c);
    // }
    bootstrap_rlibs.retain(|k, _| include_list.contains(&k.as_str()));

    bootstrap_rlibs
}

const LINKABLE_EXTENSIONS: [&str; 3] = ["rlib", "so", "dll"];

/// Process artifact and add it to the bootstrap rlibs if it's is for a bootstrap dependency and an rlib
fn process_artifact(
    artifact: cargo_metadata::camino::Utf8PathBuf,
    bootstrap_rlibs: &mut HashMap<String, cargo_metadata::camino::Utf8PathBuf>,
) {
    let file_name = artifact.file_name().unwrap_or_default();
    let lib_name = file_name.split('-').next().unwrap().strip_prefix("lib");

    if let Some(lib_name) = lib_name
        && artifact
            .extension()
            .is_some_and(|ext| LINKABLE_EXTENSIONS.contains(&ext))
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

impl CrateRef {
    pub fn into_toml_key_val(self) -> String {
        match self {
            CrateRef::Path(p) => format!("path = \"{p}\""),
            CrateRef::Version(v) => format!("version = \"{v}\""),
        }
    }
}

/// Generate bootstrapping crate files
fn write_bootstrap_files<'a>(
    bms_bindings_path: Utf8PathBuf,
    dependencies: HashMap<Crate>,
    path: &Path,
) {
    // const BMS_BINDINGS_PATH_PLACEHOLDER: &str = "{{BMS_BINDINGS_PATH}}";
    const DEPENDENCIES_PATH_PLACEHOLDER: &str = "{{DEPENDENCIES}}";

    // write manifest file 'Cargo.toml'
    let mut manifest_content =
        String::from_utf8(include_bytes!("../../Cargo.bootstrap.toml").to_vec())
            .expect("Could not read manifest template as utf8");

    let dependencies = dependencies
        .into_iter()
        .map(|krate, (version, features)| format!("{} = {{ {}}}", krate.into_toml_key_val(),));

    manifest_content = manifest_content
        .replace(DEPENDENCIES_PATH_PLACEHOLDER, bms_bindings_path.as_str())
        .replace(
            ANALYSED_CRATE_PATH_PLACEHOLDER,
            &format!(
                "{analysed_crate_name} = {{ path = \"{}\", features = [{}]}}",
                analysed_crate_path.to_string_lossy(),
                analysed_crate_features
                    .map(|f| format!("\"{f}\""))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        );

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
