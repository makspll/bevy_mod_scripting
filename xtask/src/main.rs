use std::{
    collections::HashMap,
    ffi::OsString,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output},
    str::FromStr,
};

use anyhow::{Context, *};
use clap::Parser;
use itertools::Itertools;
use json_comments::StripComments;
use log::*;
use serde::{Deserialize, Serialize};
use strum::{IntoEnumIterator, VariantNames};
use xtask::{
    BindingCrate, Feature, FeatureGroup, Features, GlobalArgs, Meta, codegen_crate_dir,
    main_workspace_cargo_metadata, prepare_codegen, read_rust_toolchain, relative_workspace_dir,
    run_system_command, run_workspace_command, workspace_dir,
};

/// Enumerates the binaries available in the project and their paths
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::EnumString,
    strum::AsRefStr,
    strum::VariantNames,
)]
#[strum(serialize_all = "snake_case")]
enum Binary {
    MdbookPreprocessor,
}

impl Binary {
    pub fn path(self) -> PathBuf {
        PathBuf::from(match self {
            Binary::MdbookPreprocessor => "./crates/lad_backends/mdbook_lad_preprocessor/",
        })
    }

    pub fn to_placeholder() -> clap::builder::Str {
        format!("[{}]", Binary::VARIANTS.join("|")).into()
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum::EnumString,
    strum::AsRefStr,
    strum::Display,
    strum::VariantArray,
)]
enum CiOs {
    #[strum(serialize = "windows-latest")]
    Windows,
    #[strum(serialize = "macos-latest")]
    Macos,
    #[strum(serialize = "ubuntu-latest")]
    Ubuntu,
}

impl CiOs {
    fn is_main_os(&self) -> bool {
        matches!(self, CiOs::Ubuntu)
    }
}

#[derive(Debug, Clone, Parser)]
struct App {
    #[clap(subcommand)]
    subcmd: Xtasks,

    #[clap(flatten)]
    global_args: GlobalArgs,
}

impl App {
    fn into_command(self) -> Command {
        let mut cmd = Command::new("cargo");
        cmd.arg("xtask");

        if self.global_args.features != Features::default() {
            cmd.arg("--features")
                .arg(self.global_args.features.to_string());
        }

        if let Some(profile) = self.global_args.profile {
            cmd.arg("--profile").arg(profile);
        }

        if self.global_args.coverage {
            cmd.arg("--coverage");
        }

        if let Some(jobs) = self.global_args.jobs {
            cmd.arg("--jobs").arg(jobs.to_string());
        }

        match self.subcmd {
            Xtasks::Macros { macro_name } => {
                cmd.arg("macros").arg(macro_name.as_ref());
            }
            Xtasks::Init { dont_update_ide } => {
                let arg = cmd.arg("init");

                if dont_update_ide {
                    arg.arg("--dont-update-ide");
                }
            }
            Xtasks::Build { timings } => {
                cmd.arg("build");
                if timings {
                    cmd.arg("--timings");
                }
            }
            Xtasks::Check { ide_mode, kind } => {
                cmd.arg("check");
                if ide_mode {
                    cmd.arg("--ide-mode");
                }
                cmd.arg("--kind").arg(kind.as_ref());
            }
            Xtasks::Docs { open, no_rust_docs } => {
                cmd.arg("docs");
                if open {
                    cmd.arg("--open");
                }
                if no_rust_docs {
                    cmd.arg("--no-rust-docs");
                }
            }
            Xtasks::Test { name, package } => {
                cmd.arg("test");
                if let Some(name) = name {
                    cmd.arg("--name").arg(name);
                }
                if let Some(package) = package {
                    cmd.arg("--package").arg(package);
                }
            }
            Xtasks::CiCheck => {
                cmd.arg("ci-check");
            }
            Xtasks::CiMatrix => {
                cmd.arg("ci-matrix");
            }
            Xtasks::Codegen {
                output_dir,
                bevy_features,
                list_only,
            } => {
                let cmd = cmd
                    .arg("codegen")
                    .arg("--output-dir")
                    .arg(output_dir)
                    .arg("--bevy-features")
                    .arg(bevy_features.join(","));

                if list_only {
                    cmd.arg("--list-only");
                }
            }
            Xtasks::Example { example } => {
                cmd.arg("example").arg(example);
            }
            Xtasks::Install { binary } => {
                cmd.arg("install").arg(binary.as_ref());
            }
            Xtasks::Bencher { publish } => {
                cmd.arg("bencher");

                if publish {
                    cmd.arg("--publish");
                }
            }
            Xtasks::Bench {
                name,
                enable_profiling: profile,
            } => {
                cmd.arg("bench");

                if let Some(name) = name {
                    cmd.arg("--name").arg(name);
                }

                if profile {
                    cmd.arg("--profile");
                }
            }
        }

        cmd
    }

    pub(crate) fn into_command_string(self) -> OsString {
        let cmd = self.into_command();
        let program = cmd.get_program();
        let args = cmd.get_args();
        let len = args.len();
        let mut os_string = OsString::new();
        os_string.push(program);
        os_string.push(" ");
        for (i, arg) in args.enumerate() {
            os_string.push(arg);
            if i < len - 1 {
                os_string.push(" ");
            }
        }

        os_string
    }

    pub(crate) fn into_ci_row(self, os: CiOs) -> CiMatrixRow {
        CiMatrixRow {
            command: self.clone().into_command_string().into_string().unwrap(),
            name: format!(
                "{}({}) - {}",
                self.subcmd.as_ref(),
                os,
                if self.global_args.features == Features::all_features() {
                    "all features".to_owned()
                } else {
                    self.global_args.features.display_no_default()
                }
            ),
            os: os.to_string(),
            generates_coverage: self.global_args.coverage,
            run_on_forks: !matches!(self.subcmd, Xtasks::Bencher { .. } | Xtasks::Bench { .. }),
            requires_gpu: matches!(self.subcmd, Xtasks::Docs { .. }),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    strum::EnumString,
    strum::VariantNames,
    strum::AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
enum CheckKind {
    #[default]
    All,
    Main,
    Codegen,
}

impl CheckKind {
    fn to_placeholder() -> clap::builder::Str {
        format!("[{}]", CheckKind::VARIANTS.join("|")).into()
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::EnumString,
    strum::AsRefStr,
    strum::VariantNames,
)]
#[strum(serialize_all = "snake_case")]
enum Macro {
    /// Integration tests for all script plugins
    ScriptTests,
}

impl Macro {
    pub fn to_placeholder() -> clap::builder::Str {
        format!("[{}]", Macro::VARIANTS.join("|")).into()
    }
}

fn fetch_default_bevy_features() -> String {
    let try_dirs = vec![".", "../"];
    let path = "codegen_bevy_features.txt";
    for dir in &try_dirs {
        let full_path = Path::new(dir).join(path);
        if full_path.exists() {
            return std::fs::read_to_string(&full_path)
                .with_context(|| format!("Failed to read default bevy features from {full_path:?}"))
                .unwrap();
        }
    }
    panic!("Failed to find {path} in any of the tried directories: {try_dirs:?}");
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, clap::Subcommand, strum::AsRefStr)]
#[clap(
    name = "xtask",
    bin_name = "cargo xtask",
    about = "A set of xtasks for managing the project. Run 'cargo xtask init' to get started."
)]
enum Xtasks {
    /// Set of tasks with predefined settings for running a combination of the other commands
    Macros {
        #[clap(
            required = true,
            value_parser=clap::value_parser!(Macro),
            value_name=Macro::to_placeholder(),
            help = "The macro to run"
        )]
        macro_name: Macro,
    },
    /// Performs first time local-development environment setup
    Init {
        /// Prevents updating the IDE settings, defaults to false
        #[clap(long, default_value = "false")]
        dont_update_ide: bool,
    },
    /// Build the main workspace only
    Build {
        /// Emit cargo build timings
        #[clap(
            long,
            default_value = "false",
            help = "Emit cargo build timings via --timinigs"
        )]
        timings: bool,
    },
    /// Build the main workspace, apply all prefferred lints
    Check {
        #[clap(
            long,
            default_value = "false",
            help = "Run in the expected format for rust-analyzer's override check command"
        )]
        ide_mode: bool,

        #[clap(
            long,
            default_value = "all",
            value_parser=clap::value_parser!(CheckKind),
            value_name=CheckKind::to_placeholder(),
            help = "The kind of check to perform",
        )]
        kind: CheckKind,
    },
    /// Run the example with the given name
    Example {
        /// The example to run
        example: String,
    },
    /// Installs a binary produced by the workspace
    Install {
        /// The binary to install
        #[clap(
            value_parser=clap::value_parser!(Binary),
            value_name=Binary::to_placeholder(),
        )]
        binary: Binary,
    },
    /// Build the rust crates.io docs as well as any other docs
    Docs {
        /// Open in browser
        /// This will open the generated docs in the default browser
        #[clap(long)]
        open: bool,

        /// Skip building rust docs
        #[clap(long)]
        no_rust_docs: bool,
    },
    /// Run code generation
    Codegen {
        /// output the generated code to the given directory
        #[clap(long, default_value = "./target/bindings_crates/")]
        output_dir: PathBuf,

        #[clap(
            long,
            default_value = fetch_default_bevy_features(),
            help = "The features to enable for the bevy crate"
        )]
        bevy_features: Vec<String>,

        /// If set will not generate but instead print out type and debug information from the bevy crate
        #[clap(long)]
        list_only: bool,
    },
    /// Build the main workspace, and then run all tests
    Test {
        /// Run tests containing the given name only
        #[clap(long)]
        name: Option<String>,

        /// Run tests in the given package only
        #[clap(long)]
        package: Option<String>,
    },
    /// Perform a full check as it would be done in CI, except not parallelised
    CiCheck,
    /// Generate a json job matrix, containing, describing maximally parallelised jobs for running in CI/CD.
    ///
    /// The format of the output is:
    ///
    /// [
    ///    {
    ///       "command": "the command to run"
    ///    }
    /// ]
    ///
    CiMatrix,
    /// Runs bencher in dry mode by default if not on the main branch
    /// To publish main branch defaults set publish mode to true
    Bencher {
        /// Publish the benchmarks when on main
        #[clap(long, default_value = "false", help = "Publish the benchmarks")]
        publish: bool,
    },
    /// Runs criterion benchmarks generates json required to be published by bencher and generates html performance report
    Bench {
        /// Whether or not to enable tracy profiling
        #[clap(long, default_value = "false", help = "Enable tracy profiling")]
        enable_profiling: bool,
        /// The name argument passed to `cargo bench`, can be used in combination with profile to selectively profile benchmarks
        #[clap(long, help = "The name argument passed to `cargo bench`")]
        name: Option<String>,
    },
}

#[derive(Serialize, Clone)]
struct CiMatrixRow {
    /// The command to run
    command: String,
    /// The display name of the job
    name: String,
    /// The os to run this on
    os: String,
    /// If this run produces lcov files
    generates_coverage: bool,
    /// If this step requires a gpu
    requires_gpu: bool,
    /// if it should run on fork PR's
    run_on_forks: bool,
}

impl Xtasks {
    fn run(self, app_settings: GlobalArgs) -> Result<String> {
        if app_settings.coverage {
            Self::set_cargo_coverage_settings();
        }

        match self {
            Xtasks::Build { timings } => Self::build(timings, app_settings),
            Xtasks::Check { ide_mode, kind } => Self::check(app_settings, ide_mode, kind),
            Xtasks::Docs { open, no_rust_docs } => Self::docs(app_settings, open, no_rust_docs),
            Xtasks::Test { name, package } => Self::test(app_settings, package, name),
            Xtasks::CiCheck => Self::cicd(app_settings),
            Xtasks::Init { dont_update_ide } => Self::init(app_settings, dont_update_ide),
            Xtasks::Example { example } => Self::example(app_settings, example),
            Xtasks::Macros { macro_name } => match macro_name {
                Macro::ScriptTests => {
                    let mut settings = app_settings.clone();
                    settings.features = Features::all_features();
                    Self::test(settings, Some("script_test".to_owned()), None)
                }
            },
            Xtasks::CiMatrix => {
                let mut output = Self::ci_matrix(app_settings)?;
                output.sort_by(|e1, e2| e1.subcmd.cmp(&e2.subcmd));
                let mut rows = Vec::default();
                for os in <CiOs as strum::VariantArray>::VARIANTS {
                    for row in output.iter() {
                        let step_should_run_on_main_os =
                            matches!(row.subcmd, Xtasks::Build { .. } | Xtasks::Docs { .. });
                        let is_coverage_step = row.global_args.coverage;

                        if !os.is_main_os() && step_should_run_on_main_os {
                            continue;
                        }

                        // we only need one source of coverage + windows is slow with this setting
                        let row = if !os.is_main_os() && is_coverage_step {
                            let new_args = row.global_args.clone().without_coverage();
                            App {
                                global_args: new_args,
                                ..row.clone()
                            }
                            .into_ci_row(*os)
                        } else {
                            row.clone().into_ci_row(*os)
                        };

                        rows.push(row);
                    }
                }

                let json = serde_json::to_string_pretty(&rows)?;
                return Ok(json);
            }
            Xtasks::Codegen {
                output_dir,
                bevy_features,
                list_only,
            } => {
                if list_only {
                    Self::codegen_list(app_settings, output_dir, bevy_features)
                } else {
                    Self::codegen(app_settings, output_dir, bevy_features)
                }
            }
            Xtasks::Install { binary } => Self::install(app_settings, binary),
            Xtasks::Bencher { publish } => Self::bencher(app_settings, publish),
            Xtasks::Bench {
                name,
                enable_profiling,
            } => {
                let _ = Self::bench(app_settings, enable_profiling, name, false)?;
                Ok(())
            }
        }?;

        Ok("".into())
    }

    fn append_rustflags(flag: &str) {
        let rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
        let mut flags = rustflags.split(' ').collect::<Vec<_>>();
        flags.push(flag);
        let flags = flags.join(" ");
        unsafe { std::env::set_var("RUSTFLAGS", flags) };
    }

    fn build(timings: bool, app_settings: GlobalArgs) -> Result<()> {
        // build workspace using the given features
        let mut args = vec!["--all-targets", "--examples"];
        if timings {
            args.push("--timings");
        }

        run_workspace_command(
            &app_settings,
            "build",
            "Failed to build workspace",
            args,
            None,
            false,
        )?;
        Ok(())
    }

    fn check_main_workspace(app_settings: GlobalArgs, ide_mode: bool) -> Result<()> {
        // start with cargo clippy
        let mut clippy_args = vec![];
        if ide_mode {
            clippy_args.push("--message-format=json");
        }

        clippy_args.extend(["--all-targets", "--examples"]);

        let keep_going = std::env::var(XTASK_KEEP_GOING).is_ok();
        if !keep_going {
            clippy_args.extend(vec!["--", "-D", "warnings"]);
        }

        run_workspace_command(
            &app_settings,
            "clippy",
            "Failed to run clippy",
            clippy_args,
            None,
            false,
        )?;

        if ide_mode {
            return Ok(());
        }

        // run cargo fmt checks
        run_workspace_command(
            &app_settings,
            "fmt",
            "Failed to run cargo fmt",
            vec!["--all", "--", "--check"],
            None,
            false,
        )?;

        Ok(())
    }

    fn check_codegen_workspace(app_settings: GlobalArgs, ide_mode: bool) -> Result<()> {
        let toolchain = read_rust_toolchain(&codegen_crate_dir(&app_settings)?);

        // set the working directory to the codegen crate
        let app_settings = app_settings
            .clone()
            .with_workspace_dir(codegen_crate_dir(&app_settings)?)
            .with_toolchain(toolchain)
            .with_features(Features::new(vec![])); // TODO: support features for codegen crate if any

        let mut clippy_args = vec![];
        if ide_mode {
            clippy_args.push("--message-format=json");
        }

        let keep_going = std::env::var(XTASK_KEEP_GOING).is_ok();
        if !keep_going {
            clippy_args.extend(vec!["--all-targets", "--", "-D", "warnings"]);
        }

        run_workspace_command(
            &app_settings,
            "clippy",
            "Failed to run clippy on codegen crate",
            clippy_args,
            None,
            false,
        )?;

        // TODO: for now do nothing, it's difficult to get rust analyzer to accept the nightly version

        Ok(())
    }

    fn codegen_list(
        app_settings: GlobalArgs,
        output_dir: PathBuf,
        bevy_features: Vec<String>,
    ) -> Result<()> {
        let settings = prepare_codegen(app_settings, Some(output_dir), bevy_features)?;
        run_workspace_command(
            &settings.bevy_repo_app_settings,
            "bms-codegen",
            "Failed to run bms-codegen generate",
            vec![
                "list-types",
                "--bms-bindings-path",
                settings.bms_bindings_path.to_str().unwrap(),
                "--output",
                settings.output_dir.to_str().unwrap(),
                "--template-args",
                settings.template_args.as_str(),
                "--features",
                settings.bevy_features.join(",").as_str(),
                "-v",
            ],
            Some(&settings.bevy_dir),
            false,
        )?;
        Ok(())
    }

    fn codegen(
        app_settings: GlobalArgs,
        output_dir: PathBuf,
        bevy_features: Vec<String>,
    ) -> Result<()> {
        let settings = prepare_codegen(app_settings, Some(output_dir), bevy_features)?;

        run_workspace_command(
            &settings.bevy_repo_app_settings,
            "bms-codegen",
            "Failed to run bms-codegen generate",
            vec![
                "generate",
                "--bms-bindings-path",
                settings.bms_bindings_path.to_str().unwrap(),
                "--output",
                settings.output_dir.to_str().unwrap(),
                "--template-args",
                settings.template_args.as_str(),
                "--features",
                settings.bevy_features.join(",").as_str(),
                "-v",
            ],
            Some(&settings.bevy_dir),
            false,
        )?;

        // collect
        run_workspace_command(
            &settings.bevy_repo_app_settings,
            "bms-codegen",
            "Failed to run bms-codegen generate",
            vec![
                "collect",
                "--bms-bindings-path",
                settings.bms_bindings_path.to_str().unwrap(),
                "--output",
                settings.output_dir.to_str().unwrap(),
                "--template-args",
                settings.template_args.as_str(),
                "-v",
            ],
            Some(&settings.bevy_dir),
            false,
        )?;

        // now expand the macros and replace the files in place
        // by running cargo expand --features crate_name and capturing the output

        let generated_crates =
            (std::fs::read_dir(&settings.output_dir)?).collect::<Result<Vec<_>, _>>()?;
        let crate_names = generated_crates
            .iter()
            .filter(|s| {
                s.path().is_file()
                    && s.path().file_name().is_some_and(|name| {
                        name != "mod.rs" && name.to_string_lossy().ends_with(".rs")
                    })
            })
            .map(|s| s.path().file_stem().unwrap().to_str().unwrap().to_owned());

        for entry in crate_names {
            // finally, generate the bindings crate code and move the code in there

            // get the version from the bevy workspace manifest
            let manifest = main_workspace_cargo_metadata()?;
            let version = manifest
                .packages
                .iter()
                .find_map(|p| {
                    if p.name.to_string() == "bevy_mod_scripting" {
                        Some(p.version.to_string())
                    } else {
                        None
                    }
                })
                .expect("Could not find bevy_mod_scripting package in metadata");

            // find features in the corresponding meta file <crate>.json under "features" key
            let meta_path = settings.output_dir.join(format!("{entry}.json"));
            let meta: Meta = serde_json::from_reader(
                std::fs::File::open(&meta_path)
                    .with_context(|| format!("opening meta file {meta_path:?}"))?,
            )?;

            let krate = BindingCrate::new(
                &entry,
                &version,
                meta.features,
                meta.version,
                meta.dependencies,
            );
            let path = relative_workspace_dir(
                &settings.main_workspace_app_settings,
                format!("crates/bindings/{entry}_bms_bindings/"),
            )?;
            krate.generate_in_dir(&path)?;
            info!("Wrote bindings crate to {path:?}");

            // copy the generated file to the bindings crate src/lib.rs
            let dest_path = path.join("src/lib.rs");
            // make dirs
            std::fs::create_dir_all(dest_path.parent().unwrap()).with_context(|| {
                format!(
                    "creating parent directory for bindings crate lib.rs: {:?}",
                    dest_path.parent().unwrap()
                )
            })?;
            std::fs::copy(settings.output_dir.join(format!("{entry}.rs")), &dest_path)
                .with_context(|| {
                    format!("copying generated binding file to bindings crate: {dest_path:?}")
                })?;

            // finally expand the macros inside

            let args = vec![String::from("expand")];
            let expand_cmd = run_system_command(
                &settings.main_workspace_app_settings,
                "cargo",
                "pre-expanding generated code",
                args,
                Some(&path),
                true,
            )?;

            let output = String::from_utf8(expand_cmd.stdout)?;

            let output = output.replacen("#![feature(prelude_import)]", "", 1);
            let output = output.replacen("#[prelude_import]", "", 1);
            let output = output.replacen("use std::prelude::rust_2024::*;", "", 1);
            let output = output.replacen("#[macro_use]\nextern crate std;", "", 1);

            std::fs::write(&dest_path, output)
                .with_context(|| format!("writing expanded code to {dest_path:?}"))?;
            info!("Wrote expanded code to {path:?}");
        }

        Ok(())
    }

    fn check(app_settings: GlobalArgs, ide_mode: bool, kind: CheckKind) -> Result<()> {
        if ide_mode && kind == CheckKind::All {
            bail!(
                "Ide mode should not be used with 'all' check kind, each workspace needs to have each own individual check, for toolchains to be properly supported"
            );
        }

        match kind {
            CheckKind::All => {
                let err_main = Self::check_main_workspace(app_settings.clone(), ide_mode);
                let err_codegen = Self::check_codegen_workspace(app_settings.clone(), ide_mode);

                err_main?;
                err_codegen?;
            }
            CheckKind::Main => {
                Self::check_main_workspace(app_settings, ide_mode)?;
            }
            CheckKind::Codegen => {
                Self::check_codegen_workspace(app_settings, ide_mode)?;
            }
        }
        Ok(())
    }

    fn docs(mut app_settings: GlobalArgs, open: bool, no_rust_docs: bool) -> Result<()> {
        // find [package.metadata."docs.rs"] key in Cargo.toml
        info!("installing mdbook ladfile preprocessor binary");
        Self::install(app_settings.clone(), Binary::MdbookPreprocessor)?;

        info!("Running docgen example to generate ladfiles");
        Self::example(app_settings.clone(), "docgen".to_owned())?;

        // copy the `<workspace>/assets/bindings.lad.json` file to it's path in the book
        let ladfile_path =
            relative_workspace_dir(&app_settings, "assets/definitions/bindings.lad.json")?;
        let destination_path =
            relative_workspace_dir(&app_settings, "docs/src/ladfiles/bindings.lad.json")?;

        info!("Copying generated ladfile from: {ladfile_path:?} to: {destination_path:?}");
        std::fs::create_dir_all(destination_path.parent().unwrap())?;
        std::fs::copy(ladfile_path, destination_path)
            .with_context(|| "copying generated ladfile")?;

        if !no_rust_docs {
            info!("Building rust docs");
            let metadata = main_workspace_cargo_metadata()?;

            let package = metadata
                .packages
                .iter()
                .find(|p| p.name.as_str() == "bevy_mod_scripting")
                .expect("Could not find bevy_mod_scripting package in metadata");

            info!("Building with root package: {}", package.name);

            let docs_rs = package
                .metadata
                .get("docs.rs")
                .expect("no docs.rs metadata");

            let features = docs_rs
                .as_object()
                .expect("docs.rs metadata is not an object")
                .get("features")
                .expect("no 'features' in docs.rs metadata");

            info!("Using docs.rs metadata: {docs_rs:?}");
            let string_list = features
                .as_array()
                .expect("docs.rs metadata is not an array")
                .iter()
                .map(|v| v.as_str().expect("docs.rs metadata is not a string"))
                .map(|s| Feature::from_str(s).expect("invalid feature"))
                .collect::<Vec<_>>();

            // include default features
            let default_features = Features::default();
            let mut features = Features::new(string_list);
            features.0.extend(default_features.0);

            app_settings.features = features;

            let mut args = Vec::default();
            args.push("--all");
            if open {
                args.push("--open");
            }
            run_workspace_command(
                &app_settings,
                "doc",
                "Failed to build crates.io docs",
                args,
                None,
                false,
            )?;
        }

        // build mdbook
        info!("Building mdbook docs");
        let args = if !open { vec!["build"] } else { vec!["serve"] };

        run_system_command(
            &app_settings,
            "mdbook",
            "Failed to build or serve mdbook docs",
            args,
            Some(Path::new("docs")),
            false,
        )?;

        Ok(())
    }

    fn bench(
        app_settings: GlobalArgs,
        profile: bool,
        name: Option<String>,
        capture_streams_in_output: bool,
    ) -> Result<Output> {
        log::info!("Profiling enabled: {profile}");

        let mut features = Features::default();

        if profile {
            unsafe { std::env::set_var("ENABLE_PROFILING", "1") };
            // features.push(Feature::BevyTracy);
            features.0.insert(Feature::ProfileWithTracy);
        } else {
            unsafe { std::env::set_var("RUST_LOG", "bevy_mod_scripting=error") };
        }

        let args = if let Some(name) = name {
            vec!["--".to_owned(), name]
        } else {
            vec![]
        };

        let output = run_workspace_command(
            // run with just lua54
            &app_settings.with_features(features),
            "bench",
            "Failed to run benchmarks",
            args,
            None,
            capture_streams_in_output,
        )
        .with_context(|| "when executing criterion benchmarks")?;

        Ok(output)
    }

    fn bencher(app_settings: GlobalArgs, publish: bool) -> Result<()> {
        // // first of all figure out which branch we're on
        // // run // git rev-parse --abbrev-ref HEAD
        let workspace_dir = workspace_dir(&app_settings).unwrap();
        let command = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(workspace_dir.clone())
            .output()
            .with_context(|| "Trying to figure out which branch we're on in benchmarking")?;
        let branch = String::from_utf8(command.stdout)?.trim().replace("\n", "");

        let is_main = branch.trim() == "main";

        // figure out if we're running in github actions
        let github_token = std::env::var("GITHUB_TOKEN").ok();

        // get testbed
        // we want this to be a combination of
        // is_github_ci?
        // OS
        // machine id

        let os = std::env::consts::OS;

        let testbed = format!(
            "{os}{}",
            if github_token.is_some() {
                "-gha"
            } else {
                Default::default()
            }
        );

        // also figure out if we're on a fork

        let token = std::env::var("BENCHER_API_TOKEN").ok();

        // first of all run bench, and save output to a file

        let result = Self::bench(app_settings, false, None, true)?;
        let bench_file_path = PathBuf::from("./bencher_output.txt");
        let mut file = std::fs::File::create(&bench_file_path)?;
        file.write_all(&result.stdout)?;

        let mut bencher_cmd = Command::new("bencher");
        bencher_cmd
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .arg("run")
            .args(["--project", "bms"])
            .args(["--branch", &branch])
            .args(["--token", &token.unwrap_or_default()])
            .args(["--testbed", &testbed])
            // .args(["--build-time"])
            .args(["--threshold-measure", "latency"])
            .args(["--threshold-test", "t_test"])
            .args(["--threshold-max-sample-size", "10"])
            .args(["--threshold-upper-boundary", "0.99"])
            .args(["--thresholds-reset"]);

        if let Some(token) = &github_token {
            bencher_cmd.args(["--github-actions", token]);
        }

        if !is_main || !publish {
            bencher_cmd.args(["--dry-run"]);
        }

        bencher_cmd
            .args(["--adapter", "rust_criterion"])
            .arg("--file")
            .arg(bench_file_path);

        log::info!("Running bencher command: {bencher_cmd:?}");

        let out = bencher_cmd
            .output()
            .with_context(|| "Could not trigger bencher command")?;
        if !out.status.success() {
            bail!("Failed to run bencher: {:?}", out);
        }

        // if we're on linux and publishing and on main synch graphs
        if os == "linux" && is_main && publish && github_token.is_some() {
            Self::synch_bencher_graphs()?;
        }

        Ok(())
    }

    fn synch_bencher_graphs() -> Result<()> {
        // first run `bencher benchmark list bms
        // this produces list of objects each containing a `uuid` and `name`

        let parse_list_of_dicts = |bytes: Vec<u8>| {
            if bytes.is_empty() {
                bail!("Empty input");
            }
            serde_json::from_slice::<Vec<HashMap<String, serde_json::Value>>>(&bytes)
                .map(|map| {
                    map.into_iter()
                        .map(|map| {
                            map.into_iter()
                                .map(|(k, v)| (k, v.as_str().unwrap_or_default().to_string()))
                                .collect::<HashMap<_, _>>()
                        })
                        .collect::<Vec<_>>()
                })
                .with_context(|| "Could not parse bencher output")
        };

        let token = std::env::var("BENCHER_API_TOKEN").ok();
        let mut bencher_cmd = Command::new("bencher");
        let benchmarks = bencher_cmd
            .stdout(std::process::Stdio::piped())
            .arg("benchmark")
            .args(["list", "bms"])
            .args(["--per-page", "255"])
            .args(["--token", &token.clone().unwrap_or_default()])
            .output()
            .with_context(|| "Could not list benchmarks")?;
        if !benchmarks.status.success() {
            bail!("Failed to list benchmarks: {:?}", benchmarks);
        }

        // parse teh name and uuid pairs
        let benchmarks = parse_list_of_dicts(benchmarks.stdout)
            .with_context(|| "Reading benchmarks")?
            .into_iter()
            .map(|p| {
                let name = p.get("name").expect("no name in project");
                let uuid = p.get("uuid").expect("no uuid in project");
                (name.clone(), uuid.clone())
            })
            .collect::<Vec<_>>();

        // delete all plots using bencher plot list bms to get "uuid's"
        // then bencher plot delete bms <uuid>

        let bencher_cmd = Command::new("bencher")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .args(["plot", "list", "bms"])
            .args(["--per-page", "255"])
            .args(["--token", &token.clone().unwrap_or_default()])
            .output()
            .with_context(|| "Could not list plots")?;

        if !bencher_cmd.status.success() {
            bail!("Failed to list plots: {:?}", bencher_cmd);
        }

        let plots = parse_list_of_dicts(bencher_cmd.stdout)
            .with_context(|| "reading plots")?
            .into_iter()
            .map(|p| {
                log::info!("Plot to delete: {p:?}");
                let uuid = p.get("uuid").expect("no uuid in plot");
                uuid.clone()
            })
            .collect::<Vec<_>>();

        for uuid in plots {
            let bencher_cmd = Command::new("bencher")
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .args(["plot", "delete", "bms", &uuid])
                .args(["--token", &token.clone().unwrap_or_default()])
                .output()
                .with_context(|| "Could not delete plot")?;

            if !bencher_cmd.status.success() {
                bail!("Failed to delete plot: {:?}", bencher_cmd);
            }
        }

        const MAIN_BRANCH_UUID: &str = "1d70a4e3-d416-43fc-91bd-4b1c8f9e9580";
        const LATENCY_MEASURE_UUID: &str = "6820b034-5163-4cdd-95f5-5640dd0ff298";
        const LINUX_GHA_TESTBED: &str = "467e8580-a67a-435e-a602-b167541f332c";
        const MACOS_GHA_TESTBAD: &str = "f8aab940-27d2-4b52-93df-4518fe68abfb";
        const WINDOWS_GHA_TESTBED: &str = "be8ff546-31d3-40c4-aacc-763e5e8a09c4";

        let testbeds = [
            ("linux-gha", LINUX_GHA_TESTBED),
            ("macos-gha", MACOS_GHA_TESTBAD),
            ("windows-gha", WINDOWS_GHA_TESTBED),
        ];

        let group_to_benchmark_map: HashMap<_, Vec<_>> =
            benchmarks
                .iter()
                .fold(HashMap::new(), |mut acc, (name, uuid)| {
                    let group = name.split('/').next().unwrap_or_default();
                    acc.entry(group.to_owned()).or_default().push(uuid.clone());
                    acc
                });

        // create plot using
        // bencher plot create --x-axis date_time --branches main --testbeds <uuids> --benchmarks <uuids> --measures latency

        for (group, uuids) in group_to_benchmark_map.iter().sorted() {
            for (testbed_name, testbed_uuid) in testbeds.iter() {
                let without_gha = testbed_name.replace("-gha", "");
                let plot_name = format!("{without_gha} {group}");

                let window_months = 12;
                let window_seconds = window_months * 30 * 24 * 60 * 60;
                let mut bencher_cmd = Command::new("bencher");
                bencher_cmd
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .args(["plot", "create", "bms"])
                    .args(["--title", &plot_name])
                    .args(["--x-axis", "version"])
                    .args(["--window", &window_seconds.to_string()])
                    .args(["--branches", MAIN_BRANCH_UUID])
                    .args(["--testbeds", testbed_uuid])
                    .args(["--measures", LATENCY_MEASURE_UUID])
                    .args(["--token", &token.clone().unwrap_or_default()]);

                for benchmark_uuid in uuids {
                    bencher_cmd.arg("--benchmarks").arg(benchmark_uuid);
                }

                let bencher_cmd = bencher_cmd
                    .output()
                    .with_context(|| "Could not create plot")?;

                if !bencher_cmd.status.success() {
                    bail!("Failed to create plot: {:?}", bencher_cmd);
                }
            }
        }

        Ok(())
    }

    fn set_cargo_coverage_settings() {
        // This makes local dev hell
        // std::env::set_var("CARGO_INCREMENTAL", "0");
        Self::append_rustflags("-Cinstrument-coverage");

        let target_dir =
            std::env::var("MAIN_CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_owned());
        let coverage_dir = std::path::PathBuf::from(target_dir).join("coverage");
        let coverage_file = coverage_dir.join("cargo-test-%p-%m.profraw");

        unsafe { std::env::set_var("LLVM_PROFILE_FILE", coverage_file) };
    }

    fn test(app_settings: GlobalArgs, package: Option<String>, name: Option<String>) -> Result<()> {
        // run cargo test with instrumentation
        let mut test_args = vec![];
        if let Some(package) = package {
            test_args.push("--package".to_owned());
            test_args.push(package);
        }

        if let Some(name) = name {
            test_args.push(name);
        }

        test_args.push("--exclude".to_owned());
        test_args.push("xtask".to_owned());

        if std::env::var("BLESS_MODE").is_ok() {
            test_args.push("--no-fail-fast".to_owned())
        }

        run_workspace_command(
            &app_settings,
            "test",
            "Failed to run tests",
            test_args,
            None,
            false,
        )?;

        // generate coverage report and lcov file
        if app_settings.coverage {
            run_system_command(
                &app_settings,
                "grcov",
                "Generating html coverage report",
                vec![
                    ".",
                    "--binary-path",
                    "./target/debug/deps/",
                    "-s",
                    ".",
                    "-t",
                    "html",
                    "--branch",
                    "--ignore-not-existing",
                    "--ignore",
                    "**/bevy_bindings/**",
                    "-o",
                    "target/coverage/html",
                ],
                None,
                false,
            )?;

            run_system_command(
                &app_settings,
                "grcov",
                "Failed to generate coverage report",
                vec![
                    ".",
                    "--binary-path",
                    "./target/debug/deps/",
                    "-s",
                    ".",
                    "-t",
                    "lcov",
                    "--branch",
                    "--ignore-not-existing",
                    "--ignore",
                    "**/bevy_bindings/**",
                    "-o",
                    "target/coverage/lcov.info",
                ],
                None,
                false,
            )?;
        }
        Ok(())
    }

    fn ci_matrix(app_settings: GlobalArgs) -> Result<Vec<App>> {
        // split up the tests we want to run into different jobs
        // everything that can be parallelised should be
        // each row in the output will correspond to a separate job in the matrix.

        let mut output = vec![];

        // first of all we run a powerset of check commands with various features enabled. all of which can be run in parallel
        let available_features =
            Features::new(<Feature as strum::VariantArray>::VARIANTS.iter().cloned());

        let grouped = available_features.split_by_group();

        let features_to_combine = grouped
            .get(&FeatureGroup::BMSFeature)
            .expect("no bms features were found at all, bms definitely has feature flags");

        // run powerset with all language features enabled without mutually exclusive
        let mut powersets = features_to_combine
            .iter()
            .cloned()
            .powerset()
            .map(Features::new)
            .collect::<Vec<_>>();

        // start with longest to compile all first
        powersets.reverse();
        info!("Powerset: {powersets:?}");

        let default_args = app_settings
            .clone()
            .with_features(Features::all_features().without(Feature::ProfileWithTracy))
            .with_profile(
                app_settings
                    .profile
                    .clone()
                    .or(Some("ephemeral-build".to_owned())),
            );

        for feature_set in powersets.iter_mut() {
            // choose language features
            for exclusive_category in FeatureGroup::iter().filter(|g| g.is_exclusive()) {
                feature_set.0.insert(exclusive_category.default_feature());
            }

            // include all non-bms features
            if let Some(f) = grouped.get(&FeatureGroup::ForExternalCrate) {
                feature_set.0.extend(f.iter().cloned());
            }

            // include all features which are excluded from powersetting
            if let Some(f) = grouped.get(&FeatureGroup::BMSFeatureNotInPowerset) {
                feature_set.0.extend(f.iter().cloned());
            }

            // replace args with powerset
            output.push(App {
                global_args: default_args.clone().with_features(feature_set.clone()),
                subcmd: Xtasks::Build { timings: false },
            })
        }

        log::info!("Powerset command combinations: {output:?}");

        // next run a full lint check with all features
        output.push(App {
            global_args: default_args.clone(),
            subcmd: Xtasks::Check {
                ide_mode: false,
                kind: CheckKind::All,
            },
        });

        // then run docs
        output.push(App {
            global_args: default_args.clone(),
            subcmd: Xtasks::Docs {
                open: false,
                no_rust_docs: false,
            },
        });

        // also run a benchmark
        // on non-main branches this will just dry run
        output.push(App {
            global_args: default_args.clone(),
            subcmd: Xtasks::Bencher { publish: true },
        });

        // and finally run tests with coverage
        output.push(App {
            global_args: default_args
                .clone()
                .with_coverage()
                // github actions has been throwing a lot of OOM SIGTERM's lately
                .with_max_jobs(2),
            subcmd: Xtasks::Test {
                name: None,
                package: None,
            },
        });

        Ok(output)
    }

    fn cicd(app_settings: GlobalArgs) -> Result<()> {
        // get the ci matrix
        let matrix = Self::ci_matrix(app_settings.clone())?;
        let length = matrix.len();
        for (i, app) in matrix.into_iter().enumerate() {
            info!("Running CI job {}/{}. {:?}", i + 1, length, app.subcmd);
            app.subcmd.run(app_settings.clone())?;
        }

        Ok(())
    }

    fn init(app_settings: GlobalArgs, dont_update_ide: bool) -> Result<()> {
        // install alsa et al
        if cfg!(target_os = "linux") {
            let sudo = if !is_root::is_root() { "sudo" } else { "" };
            let install_cmd = format!(
                "{sudo} apt-get update && {sudo} apt-get install --no-install-recommends -y libasound2-dev libudev-dev libwayland-dev"
            );
            run_system_command(
                &app_settings,
                "sh",
                "Failed to install Linux dependencies",
                vec!["-c", install_cmd.as_str()],
                None,
                false,
            )?;
        }

        // install bencher
        // linux curl --proto '=https' --tlsv1.2 -sSfL https://bencher.dev/download/install-cli.sh | sh
        // windows irm https://bencher.dev/download/install-cli.ps1 | iex
        run_system_command(
            &app_settings,
            "cargo",
            "Failed to install bencher",
            vec![
                "install",
                "--git",
                "https://github.com/bencherdev/bencher",
                "--branch",
                "main",
                "--locked",
                "--force",
                "bencher_cli",
            ],
            None,
            false,
        )?;
        // install cargo mdbook
        run_system_command(
            &app_settings,
            "cargo",
            "Failed to install mdbook",
            vec!["install", "mdbook"],
            None,
            false,
        )?;

        // install mdbook-mermaid
        run_system_command(
            &app_settings,
            "cargo",
            "Failed to install mdbook",
            vec!["install", "mdbook-mermaid"],
            None,
            false,
        )?;

        // install grcov
        run_system_command(
            &app_settings,
            "cargo",
            "Failed to install grcov",
            vec!["install", "grcov"],
            None,
            false,
        )?;

        // install cargo expand
        run_system_command(
            &app_settings,
            "cargo",
            "Failed to install cargo expand",
            vec!["install", "cargo-expand"],
            None,
            false,
        )?;

        // install nightly toolchaing for bevy api gen
        let toolchain = read_rust_toolchain(&codegen_crate_dir(&app_settings)?);
        run_system_command(
            &app_settings,
            "rustup",
            "Failed to install nightly toolchain",
            vec!["toolchain", "install", toolchain.as_str()],
            None,
            false,
        )?;

        let rustup_components_args = [
            "component",
            "add",
            "rust-src",
            "rustc-dev",
            "clippy",
            "llvm-tools-preview",
            "rustfmt",
        ];

        // install components for the stable and nightly toolchains
        run_system_command(
            &app_settings,
            "rustup",
            "Failed to install rust components",
            rustup_components_args,
            None,
            false,
        )?;

        // add components on nightly toolchain
        run_system_command(
            &app_settings,
            "rustup",
            "Failed to install nightly components",
            rustup_components_args
                .iter()
                .chain(["--toolchain", toolchain.as_str()].iter()),
            Some(Path::new(".")),
            false,
        )?;

        // create .vscode settings
        // read from templates at compile time
        if !dont_update_ide {
            let vscode_settings = include_str!("../templates/settings.json.tera");
            let mut tera = tera::Tera::default();
            let mut context = tera::Context::new();
            let workspace_dir = workspace_dir(&app_settings)?;
            let json_workspace_dir = serde_json::to_string(&workspace_dir)?; // make sure this works as a json string
            context.insert("dir", &json_workspace_dir.trim_matches('\"'));

            let templated_settings = tera.render_str(vscode_settings, &context)?;
            let templated_settings_json =
                Self::read_json_with_comments(templated_settings.as_bytes())
                    .with_context(|| "reading templated vscode settings")?;
            let vscode_dir = relative_workspace_dir(&app_settings, ".vscode")?;
            std::fs::create_dir_all(&vscode_dir)?;
            let vscode_settings_path = vscode_dir.join("settings.json");

            // if the file already exists, merge the settings otherwise create it
            info!(
                "Merging vscode settings at {vscode_settings_path:?}. With overrides generated by template."
            );
            if vscode_settings_path.exists() {
                let existing_settings = std::fs::read_to_string(&vscode_settings_path)?;
                let mut existing_settings =
                    Self::read_json_with_comments(existing_settings.as_bytes())
                        .with_context(|| "reading existing vscode settings file")?;
                Self::merge_json(templated_settings_json, &mut existing_settings);
                let merged_settings = serde_json::to_string_pretty(&existing_settings)?;
                std::fs::write(&vscode_settings_path, merged_settings)?;
            } else {
                std::fs::write(&vscode_settings_path, templated_settings)?;
            }
        }

        Ok(())
    }

    fn read_json_with_comments(bytes: &[u8]) -> Result<serde_json::Value> {
        let stripped = StripComments::new(bytes);
        let mut reader = serde_json::Deserializer::from_reader(stripped);
        let value = serde_json::Value::deserialize(&mut reader)
            .with_context(|| format!("deserializing json:\n{}", String::from_utf8_lossy(bytes)))?;
        Ok(value)
    }

    /// Override the target json file with some overrides. Will replace values if they already exist, or insert them otherwise.
    fn merge_json(overrides: serde_json::Value, target: &mut serde_json::Value) {
        if let (serde_json::Value::Object(overrides), serde_json::Value::Object(target)) =
            (overrides, target)
        {
            for (key, value) in overrides {
                // simply replace
                info!("Replacing json key: {key} with value: {value}");
                target.insert(key.clone(), value.clone());
            }
        } else {
            warn!("Could not merge json, overrides and target are not objects");
        }
    }

    fn example(app_settings: GlobalArgs, example: String) -> std::result::Result<(), Error> {
        // find the required features for the example named this in the cargo.toml of the main workspace
        // the keys look like
        // [[example]]
        // name = "docgen"
        // path = "examples/docgen.rs"
        // required-features = []

        // let metadata = main_workspace_cargo_metadata()?;
        // let metadata = &metadata.root_package().unwrap().targets;
        // println!("{metadata:#?}");

        // run the example
        run_workspace_command(
            &app_settings,
            "run",
            "Failed to run example",
            vec!["--example", example.as_str()],
            None,
            false,
        )?;

        Ok(())
    }

    fn install(app_settings: GlobalArgs, binary: Binary) -> std::result::Result<(), Error> {
        // run cargo install --path
        let binary_path = relative_workspace_dir(&app_settings, binary.path())?;
        run_system_command(
            &app_settings,
            "cargo",
            "Failed to install binary",
            vec!["install", "--path", binary_path.to_str().unwrap()],
            None,
            false,
        )?;

        Ok(())
    }
}

/// Because we are likely already runnnig in the context of a cargo invocation,
/// some environment variables may be set that we don't want to inherit.
/// Set them to MAIN_CARGO_<VARIABLE> so that we can reference them but so they dont get inherited by further cargo commands
fn pop_cargo_env() -> Result<()> {
    let env = std::env::vars().collect::<Vec<_>>();
    // RUSTUP TOOLCHAIN exclude is a temporary fix, it might make deving the api codegen crate not work
    let exclude_list = ["CARGO_HOME"];
    let include_list = []; //"LD_LIBRARY_PATH"

    for (key, value) in env.iter() {
        let key_str = &(key.as_str());
        if (include_list.contains(key_str) || key.starts_with("CARGO_"))
            && !exclude_list.contains(key_str)
        {
            let new_key = format!("MAIN_{key}");
            unsafe { std::env::set_var(new_key, value) };
            unsafe { std::env::remove_var(key) };
        }
    }

    // unset some other variables
    let remove_vars = ["RUSTUP_TOOLCHAIN"];
    for var in remove_vars.iter() {
        if exclude_list.contains(var) {
            continue;
        }
        unsafe { std::env::remove_var(var) };
    }

    Ok(())
}

fn try_main() -> Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();

    pop_cargo_env()?;

    let args = App::try_parse()?;
    info!(
        "Default toolchain: {:?}",
        args.global_args.override_toolchain
    );

    let out = args.subcmd.run(args.global_args)?;
    // push any output to stdout
    if !out.is_empty() {
        std::io::stdout().write_all(out.as_bytes())?;
        println!();
    }
    Ok(())
}

const XTASK_KEEP_GOING: &str = "XTASK_KEEP_GOING";

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
}
