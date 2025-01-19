use anyhow::{Context, *};
use clap::Parser;
use itertools::Itertools;
use json_comments::StripComments;
use log::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ffi::{OsStr, OsString},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};
use strum::{IntoEnumIterator, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    strum::EnumString,
    strum::EnumIter,
    strum::Display,
    strum::VariantNames,
    strum::VariantArray,
)]
#[strum(serialize_all = "snake_case")]
enum Feature {
    // Lua
    Lua51,
    Lua52,
    Lua53,
    Lua54,
    Luajit,
    Luajit52,
    Luau,
    BevyBindings,
    CoreFunctions,
    UnsafeLuaModules,
    MluaSerialize,
    MluaMacros,
    MluaAsync,
    // Rhai
    // Rhai,

    // Rune
    // Rune,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, strum::EnumIter)]
enum FeatureGroup {
    LuaExclusive,
    // RhaiExclusive,
    // RuneExclusive,
    ForExternalCrate,
    BMSFeature,
}

impl FeatureGroup {
    fn default_feature(self) -> Feature {
        match self {
            FeatureGroup::LuaExclusive => Feature::Lua54,
            // FeatureGroup::RhaiExclusive => Feature::Rhai,
            // FeatureGroup::RuneExclusive => Feature::Rune,
            _ => panic!("No default feature for non-exclusive group"),
        }
    }

    fn is_exclusive(self) -> bool {
        matches!(
            self,
            FeatureGroup::LuaExclusive // | FeatureGroup::RhaiExclusive | FeatureGroup::RuneExclusive
        )
    }
}

trait IntoFeatureGroup {
    fn to_feature_group(self) -> FeatureGroup;
}

impl IntoFeatureGroup for Feature {
    fn to_feature_group(self) -> FeatureGroup {
        match self {
            Feature::Lua51
            | Feature::Lua52
            | Feature::Lua53
            | Feature::Lua54
            | Feature::Luajit
            | Feature::Luajit52
            | Feature::Luau => FeatureGroup::LuaExclusive,
            // Feature::Rhai => FeatureGroup::RhaiExclusive,
            // Feature::Rune => FeatureGroup::RuneExclusive,
            Feature::MluaAsync
            | Feature::MluaMacros
            | Feature::MluaSerialize
            | Feature::UnsafeLuaModules => FeatureGroup::ForExternalCrate,
            Feature::BevyBindings | Feature::CoreFunctions => FeatureGroup::BMSFeature,
            // don't use wildcard here, we want to be explicit
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Features(HashSet<Feature>);

impl Default for Features {
    fn default() -> Self {
        // should be kept up to date with the default feature + lua54
        Features::new(vec![
            Feature::Lua54,
            Feature::CoreFunctions,
            Feature::BevyBindings,
        ])
    }
}

impl Features {
    fn new<I: IntoIterator<Item = Feature>>(features: I) -> Self {
        Self(features.into_iter().collect())
    }

    /// Returns all features except the exclusive ones which are not the default
    fn all_features() -> Self {
        // remove exclusive features which are not the default
        Self(
            <Feature as strum::VariantArray>::VARIANTS
                .iter()
                .filter(|f| {
                    let group = f.to_feature_group();
                    (!group.is_exclusive()) || (**f == group.default_feature())
                })
                .cloned()
                .collect(),
        )
    }

    fn non_exclusive_features() -> Self {
        Self(
            <Feature as strum::VariantArray>::VARIANTS
                .iter()
                .filter(|f| !f.to_feature_group().is_exclusive())
                .cloned()
                .collect(),
        )
    }

    fn to_cargo_args(&self) -> Vec<String> {
        if self.0.is_empty() {
            vec![]
        } else {
            vec![
                "--no-default-features".to_owned(),
                "--features".to_owned(),
                self.to_string(),
            ]
        }
    }

    fn to_placeholder() -> clap::builder::Str {
        format!("[{}]", Feature::VARIANTS.join("|")).into()
    }

    fn split_by_group(&self) -> HashMap<FeatureGroup, Vec<Feature>> {
        let mut groups = HashMap::new();
        for feature in &self.0 {
            let group = feature.to_feature_group();
            groups.entry(group).or_insert_with(Vec::new).push(*feature);
        }
        groups
    }
}

impl std::fmt::Display for Features {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, feature) in self.0.iter().sorted().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", feature)?;
        }
        std::result::Result::Ok(())
    }
}

impl From<String> for Features {
    fn from(s: String) -> Self {
        if s.is_empty() {
            return Self::new(vec![]);
        }

        let features = s
            .trim()
            .split(',')
            .map(|f| {
                Feature::from_str(f).unwrap_or_else(|_| {
                    eprintln!("Unknown feature: '{}'", f);
                    std::process::exit(1);
                })
            })
            .collect();
        Self(features)
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

        match self.subcmd {
            Xtasks::Macros { macro_name } => {
                cmd.arg("macros").arg(macro_name.as_ref());
            }
            Xtasks::Init => {
                cmd.arg("init");
            }
            Xtasks::Build => {
                cmd.arg("build");
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
            } => {
                cmd.arg("codegen")
                    .arg("--output-dir")
                    .arg(output_dir)
                    .arg("--bevy-features")
                    .arg(bevy_features.join(","));
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
                    self.global_args.features.to_string()
                }
            ),
            os: os.to_string(),
            generates_coverage: self.global_args.coverage,
        }
    }
}

#[derive(Debug, Parser, Clone)]
struct GlobalArgs {
    #[clap(long, short, global = true, value_parser=clap::value_parser!(Features), value_name=Features::to_placeholder(), default_value=Features::default().to_string(),required = false)]
    features: Features,

    #[clap(
        long,
        global = true,
        default_value = "false",
        help = "Enable coverage collection for cargo commands"
    )]
    coverage: bool,

    #[clap(skip)]
    override_workspace_dir: Option<PathBuf>,

    #[clap(skip)]
    override_toolchain: Option<String>,

    #[clap(
        long,
        short,
        global = true,
        value_name = "PROFILE",
        help = "The cargo profile to use for commands that support it"
    )]
    profile: Option<String>,
}

impl GlobalArgs {
    pub fn with_coverage(self) -> Self {
        Self {
            coverage: true,
            ..self
        }
    }

    pub fn without_coverage(self) -> Self {
        Self {
            coverage: false,
            ..self
        }
    }

    pub fn with_workspace_dir(self, dir: PathBuf) -> Self {
        Self {
            override_workspace_dir: Some(dir),
            ..self
        }
    }

    pub fn with_toolchain(self, toolchain: String) -> Self {
        Self {
            override_toolchain: Some(toolchain),
            ..self
        }
    }

    pub fn with_features(self, features: Features) -> Self {
        Self { features, ..self }
    }

    pub fn with_profile(self, profile: Option<String>) -> Self {
        Self { profile, ..self }
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
#[derive(Debug, Clone, serde::Serialize)]
struct CodegenTemplateArgs {
    self_is_bms_lua: bool,
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
    Init,
    /// Build the main workspace only
    Build,
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
        #[clap(
            long,
            default_value = "./crates/bevy_mod_scripting_functions/src/bevy_bindings/"
        )]
        output_dir: PathBuf,

        #[clap(
            long,
            default_value = "bevy_asset,bevy_animation,bevy_core_pipeline,bevy_ui,bevy_pbr,bevy_render,bevy_text,bevy_sprite,file_watcher,multi_threaded",
            help = "The features to enable for the bevy crate"
        )]
        bevy_features: Vec<String>,
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
}

impl Xtasks {
    fn run(self, app_settings: GlobalArgs) -> Result<String> {
        if app_settings.coverage {
            Self::set_cargo_coverage_settings();
        }

        match self {
            Xtasks::Build => Self::build(app_settings),
            Xtasks::Check { ide_mode, kind } => Self::check(app_settings, ide_mode, kind),
            Xtasks::Docs { open, no_rust_docs } => Self::docs(app_settings, open, no_rust_docs),
            Xtasks::Test { name, package } => Self::test(app_settings, name, package),
            Xtasks::CiCheck => Self::cicd(app_settings),
            Xtasks::Init => Self::init(app_settings),
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
                            matches!(row.subcmd, Xtasks::Build | Xtasks::Docs { .. });
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
            } => Self::codegen(app_settings, output_dir, bevy_features),
        }?;

        Ok("".into())
    }

    /// Reads the metadata from the main workspace
    fn main_workspace_cargo_metadata() -> Result<cargo_metadata::Metadata> {
        let cargo_manifest_path = std::env::var("MAIN_CARGO_MANIFEST_PATH").unwrap();

        let mut cmd = cargo_metadata::MetadataCommand::new();
        cmd.manifest_path(cargo_manifest_path);
        let out = cmd.exec()?;
        Ok(out)
    }

    fn workspace_dir(app_settings: &GlobalArgs) -> Result<std::path::PathBuf> {
        if let Some(dir) = &app_settings.override_workspace_dir {
            return Ok(dir.into());
        }

        let metadata = Self::main_workspace_cargo_metadata()?;
        let workspace_root = metadata.workspace_root;
        Ok(workspace_root.into())
    }

    fn codegen_crate_dir(app_settings: &GlobalArgs) -> Result<std::path::PathBuf> {
        let workspace_dir = Self::workspace_dir(app_settings)?;
        Ok(workspace_dir.join("crates").join("bevy_api_gen"))
    }

    fn relative_workspace_dir<P: AsRef<Path>>(
        app_settings: &GlobalArgs,
        dir: P,
    ) -> Result<std::path::PathBuf> {
        let workspace_dir = Self::workspace_dir(app_settings)?;
        Ok(workspace_dir.join(dir))
    }

    fn append_rustflags(flag: &str) {
        let rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
        let mut flags = rustflags.split(' ').collect::<Vec<_>>();
        flags.push(flag);
        let flags = flags.join(" ");
        std::env::set_var("RUSTFLAGS", flags);
    }

    fn run_system_command<I: IntoIterator<Item = impl AsRef<OsStr>>>(
        app_settings: &GlobalArgs,
        command: &str,
        context: &str,
        add_args: I,
        dir: Option<&Path>,
    ) -> Result<()> {
        info!("Running system command: {}", command);

        let working_dir = match dir {
            Some(d) => Self::relative_workspace_dir(app_settings, d)?,
            None => Self::workspace_dir(app_settings)?,
        };

        let mut cmd = Command::new(command);
        cmd.args(add_args)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .current_dir(working_dir);

        info!("Using command: {:?}", cmd);

        let output = cmd.output();
        info!("Command output: {:?}", output);
        let output = output.with_context(|| context.to_owned())?;
        match output.status.code() {
            Some(0) => Ok(()),
            _ => bail!(
                "{} failed with exit code: {}",
                context,
                output.status.code().unwrap_or(-1)
            ),
        }
    }

    fn run_workspace_command<I: IntoIterator<Item = impl AsRef<OsStr>>>(
        app_settings: &GlobalArgs,
        command: &str,
        context: &str,
        add_args: I,
        dir: Option<&Path>,
    ) -> Result<()> {
        let coverage_mode = app_settings
            .coverage
            .then_some("with coverage")
            .unwrap_or_default();

        info!("Running workspace command {coverage_mode}: {command}");

        let mut args = vec![];

        if let Some(ref toolchain) = app_settings.override_toolchain {
            args.push(format!("+{}", toolchain));
        }

        args.push(command.to_owned());

        if command != "fmt" && command != "bevy-api-gen" {
            // fmt doesn't care about features, workspaces or profiles

            args.push("--workspace".to_owned());

            if let Some(profile) = app_settings.profile.as_ref() {
                let use_profile = if profile == "ephemeral-build" && app_settings.coverage {
                    // use special profile for coverage as it needs debug information
                    // but also don't want it too slow
                    "ephemeral-coverage"
                } else {
                    profile
                };

                if !app_settings.coverage {
                    args.push("--profile".to_owned());
                    args.push(use_profile.to_owned());
                }
            }

            args.extend(app_settings.features.to_cargo_args());
        }

        args.extend(add_args.into_iter().map(|s| {
            s.as_ref()
                .to_str()
                .expect("invalid command argument")
                .to_owned()
        }));

        let working_dir = match dir {
            Some(d) => Self::relative_workspace_dir(app_settings, d)?,
            None => Self::workspace_dir(app_settings)?,
        };

        let mut cmd = Command::new("cargo");
        cmd.args(args)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .current_dir(working_dir);

        info!("Using command: {:?}", cmd);

        let output = cmd.output().with_context(|| context.to_owned())?;
        match output.status.code() {
            Some(0) => Ok(()),
            _ => bail!(
                "{} failed with exit code: {}. Features: {}",
                context,
                output.status.code().unwrap_or(-1),
                app_settings.features
            ),
        }
    }

    fn build(app_settings: GlobalArgs) -> Result<()> {
        // build workspace using the given features
        Self::run_workspace_command(
            &app_settings,
            "build",
            "Failed to build workspace",
            vec!["--all-targets"],
            None,
        )?;
        Ok(())
    }

    fn check_main_workspace(app_settings: GlobalArgs, ide_mode: bool) -> Result<()> {
        // start with cargo clippy
        let mut clippy_args = vec![];
        if ide_mode {
            clippy_args.push("--message-format=json");
        }
        clippy_args.extend(vec!["--all-targets", "--", "-D", "warnings"]);

        Self::run_workspace_command(
            &app_settings,
            "clippy",
            "Failed to run clippy",
            clippy_args,
            None,
        )?;

        if ide_mode {
            return Ok(());
        }

        // run cargo fmt checks
        Self::run_workspace_command(
            &app_settings,
            "fmt",
            "Failed to run cargo fmt",
            vec!["--all", "--", "--check"],
            None,
        )?;

        Ok(())
    }

    fn check_codegen_crate(app_settings: GlobalArgs, ide_mode: bool) -> Result<()> {
        let toolchain = Self::read_rust_toolchain(&Self::codegen_crate_dir(&app_settings)?);

        // set the working directory to the codegen crate
        let app_settings = app_settings
            .clone()
            .with_workspace_dir(Self::codegen_crate_dir(&app_settings)?)
            .with_toolchain(toolchain)
            .with_features(Features::new(vec![])); // TODO: support features for codegen crate if any

        let mut clippy_args = vec![];
        if ide_mode {
            clippy_args.push("--message-format=json");
        }
        clippy_args.extend(vec!["--all-targets", "--", "-D", "warnings"]);

        Self::run_workspace_command(
            &app_settings,
            "clippy",
            "Failed to run clippy on codegen crate",
            clippy_args,
            None,
        )?;

        // TODO: for now do nothing, it's difficult to get rust analyzer to accept the nightly version

        Ok(())
    }

    /// reads rust-toolchain.toml file at the given directory and returns the toolchain string
    fn read_rust_toolchain(path: &Path) -> String {
        let rust_toolchain_path = path.join("rust-toolchain.toml");

        let rust_toolchain = std::fs::read_to_string(rust_toolchain_path)
            .expect("Could not read rust_toolchain.toml");

        // parse the toml file
        let toml: toml::Value =
            toml::from_str(&rust_toolchain).expect("Could not parse rust_toolchain.toml");

        let toolchain = toml
            .get("toolchain")
            .expect("Could not find toolchain in rust_toolchain.toml");
        let channel = toolchain
            .get("channel")
            .expect("Could not find channel in rust_toolchain.toml");
        let channel = channel
            .as_str()
            .expect("Channel in rust_toolchain.toml is not a string");

        // set the toolchain
        channel.to_string()
    }

    fn codegen(
        app_settings: GlobalArgs,
        output_dir: PathBuf,
        bevy_features: Vec<String>,
    ) -> Result<()> {
        let main_workspace_app_settings = app_settings;
        let output_dir = Self::relative_workspace_dir(&main_workspace_app_settings, output_dir)?;
        let bevy_dir =
            Self::relative_workspace_dir(&main_workspace_app_settings, "target/codegen/bevy")?;
        let bevy_target_dir = bevy_dir.join("target");
        // clear the bevy target dir if it exists
        info!("Clearing bevy target dir: {:?}", bevy_target_dir);
        if bevy_target_dir.exists() {
            std::fs::remove_dir_all(&bevy_target_dir)?;
        }

        let api_gen_dir = Self::codegen_crate_dir(&main_workspace_app_settings)?;
        let codegen_app_settings = main_workspace_app_settings
            .clone()
            .with_workspace_dir(api_gen_dir.clone());

        let bevy_repo_app_settings = main_workspace_app_settings
            .clone()
            .with_workspace_dir(bevy_dir.clone())
            .with_toolchain(Self::read_rust_toolchain(&api_gen_dir));

        // run cargo install
        Self::run_system_command(
            &codegen_app_settings,
            "cargo",
            "Failed to install bevy_api_gen",
            vec!["install", "--path", "."],
            None,
        )?;

        let metadata = Self::main_workspace_cargo_metadata()?;
        let bevy_version = metadata
            .packages
            .iter()
            .find(|p| p.name == "bevy")
            .expect("Could not find bevy package in metadata")
            .version
            .clone();
        // create directories if they don't already exist
        std::fs::create_dir_all(&bevy_dir)?;
        std::fs::create_dir_all(&output_dir)?;

        // git clone bevy repo
        let _ = Self::run_system_command(
            &bevy_repo_app_settings,
            "git",
            "Failed to clone bevy repo",
            vec![
                "clone",
                "https://github.com/bevyengine/bevy",
                "--branch",
                format!("v{}", bevy_version).as_str(),
                "--depth",
                "1",
                ".",
            ],
            None,
        );

        // fetch the tags
        Self::run_system_command(
            &bevy_repo_app_settings,
            "git",
            "Failed to fetch bevy tags",
            vec!["fetch", "--tags"],
            Some(&bevy_dir),
        )?;

        // checkout the version tag
        Self::run_system_command(
            &bevy_repo_app_settings,
            "git",
            "Failed to checkout bevy tag",
            vec!["checkout", format!("v{}", bevy_version).as_str()],
            Some(&bevy_dir),
        )?;

        // run bevy_api_gen
        let template_args = CodegenTemplateArgs {
            self_is_bms_lua: true,
        };

        let template_args = serde_json::to_string(&template_args)?;
        let bms_core_path = Self::workspace_dir(&main_workspace_app_settings)?
            .join("crates/bevy_mod_scripting_core")
            .to_path_buf();

        Self::run_workspace_command(
            &bevy_repo_app_settings,
            "bevy-api-gen",
            "Failed to run bevy-api-gen generate",
            vec![
                "generate",
                "--bms-core-path",
                bms_core_path.to_str().unwrap(),
                "--output",
                output_dir.to_str().unwrap(),
                "--template-args",
                template_args.as_str(),
                "--features",
                bevy_features.join(",").as_str(),
                "-v",
            ],
            Some(&bevy_dir),
        )?;

        // collect
        Self::run_workspace_command(
            &bevy_repo_app_settings,
            "bevy-api-gen",
            "Failed to run bevy-api-gen generate",
            vec![
                "collect",
                "--bms-core-path",
                bms_core_path.to_str().unwrap(),
                "--output",
                output_dir.to_str().unwrap(),
                "--template-args",
                template_args.as_str(),
                "-v",
            ],
            Some(&bevy_dir),
        )?;

        Ok(())
    }

    fn check(app_settings: GlobalArgs, ide_mode: bool, kind: CheckKind) -> Result<()> {
        match kind {
            CheckKind::All => {
                let err_main = Self::check_main_workspace(app_settings.clone(), ide_mode);
                let err_codegen = Self::check_codegen_crate(app_settings.clone(), ide_mode);

                err_main?;
                err_codegen?;
            }
            CheckKind::Main => {
                Self::check_main_workspace(app_settings, ide_mode)?;
            }
            CheckKind::Codegen => {
                Self::check_codegen_crate(app_settings, ide_mode)?;
            }
        }
        Ok(())
    }

    fn docs(mut app_settings: GlobalArgs, open: bool, no_rust_docs: bool) -> Result<()> {
        // find [package.metadata."docs.rs"] key in Cargo.toml
        if !no_rust_docs {
            info!("Building rust docs");
            let metadata = Self::main_workspace_cargo_metadata()?;

            let package = metadata
                .packages
                .iter()
                .find(|p| p.name == "bevy_mod_scripting")
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

            info!("Using docs.rs metadata: {:?}", docs_rs);
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
            Self::run_workspace_command(
                &app_settings,
                "doc",
                "Failed to build crates.io docs",
                args,
                None,
            )?;
        }

        // build mdbook
        info!("Building mdbook docs");
        let args = if !open { vec!["build"] } else { vec!["serve"] };

        Self::run_system_command(
            &app_settings,
            "mdbook",
            "Failed to build or serve mdbook docs",
            args,
            Some(Path::new("docs")),
        )?;

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

        std::env::set_var("LLVM_PROFILE_FILE", coverage_file);
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

        Self::run_workspace_command(
            &app_settings,
            "test",
            "Failed to run tests",
            vec!["--exclude", "xtask"],
            None,
        )?;

        // generate coverage report and lcov file
        if app_settings.coverage {
            Self::run_system_command(
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
            )?;

            Self::run_system_command(
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
        info!("Powerset: {:?}", powersets);

        let default_args = app_settings
            .clone()
            .with_features(Features::all_features())
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

            output.push(App {
                global_args: default_args.clone().with_features(feature_set.clone()),
                subcmd: Xtasks::Build,
            })
        }

        // also run a all features + each exclusive feature by itself
        for feature in available_features
            .0
            .iter()
            .filter(|f| f.to_feature_group().is_exclusive())
        {
            // run with all features
            let mut features = Features::non_exclusive_features();
            features.0.insert(*feature);

            // don't include if we already ran this combination
            if powersets.iter().any(|f| f == &features) {
                continue;
            }

            output.push(App {
                global_args: default_args.clone().with_features(features),
                subcmd: Xtasks::Build,
            });
        }

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

        // and finally run tests with coverage
        output.push(App {
            global_args: default_args.clone().with_coverage(),
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

    fn init(app_settings: GlobalArgs) -> Result<()> {
        // install cargo mdbook
        Self::run_system_command(
            &app_settings,
            "cargo",
            "Failed to install mdbook",
            vec!["install", "mdbook"],
            None,
        )?;

        // install grcov
        Self::run_system_command(
            &app_settings,
            "cargo",
            "Failed to install grcov",
            vec!["install", "grcov"],
            None,
        )?;

        // install nightly toolchaing for bevy api gen
        let toolchain = Self::read_rust_toolchain(&Self::codegen_crate_dir(&app_settings)?);
        Self::run_system_command(
            &app_settings,
            "rustup",
            "Failed to install nightly toolchain",
            vec!["toolchain", "install", toolchain.as_str()],
            None,
        )?;

        let rustup_components_args = [
            "component",
            "add",
            "rust-src",
            "rustc-dev",
            "clippy",
            "llvm-tools-preview",
        ];

        // install components for the stable and nightly toolchains
        Self::run_system_command(
            &app_settings,
            "rustup",
            "Failed to install rust components",
            rustup_components_args,
            None,
        )?;

        // add components on nightly toolchain
        Self::run_system_command(
            &app_settings,
            "rustup",
            "Failed to install nightly components",
            rustup_components_args
                .iter()
                .chain(["--toolchain", toolchain.as_str()].iter()),
            Some(Path::new(".")),
        )?;

        // create .vscode settings
        // read from templates at compile time
        let vscode_settings = include_str!("../templates/settings.json.tera");
        let mut tera = tera::Tera::default();
        let mut context = tera::Context::new();
        let workspace_dir = Self::workspace_dir(&app_settings)?;
        let json_workspace_dir = serde_json::to_string(&workspace_dir)?; // make sure this works as a json string
        context.insert("dir", &json_workspace_dir.trim_matches('\"'));

        let templated_settings = tera.render_str(vscode_settings, &context)?;
        let templated_settings_json = Self::read_json_with_comments(templated_settings.as_bytes())
            .with_context(|| "reading templated vscode settings")?;
        let vscode_dir = Self::relative_workspace_dir(&app_settings, ".vscode")?;
        std::fs::create_dir_all(&vscode_dir)?;
        let vscode_settings_path = vscode_dir.join("settings.json");

        // if the file already exists, merge the settings otherwise create it
        info!(
            "Merging vscode settings at {:?}. With overrides generated by template.",
            vscode_settings_path
        );
        if vscode_settings_path.exists() {
            let existing_settings = std::fs::read_to_string(&vscode_settings_path)?;
            let mut existing_settings = Self::read_json_with_comments(existing_settings.as_bytes())
                .with_context(|| "reading existing vscode settings file")?;
            Self::merge_json(templated_settings_json, &mut existing_settings);
            let merged_settings = serde_json::to_string_pretty(&existing_settings)?;
            std::fs::write(&vscode_settings_path, merged_settings)?;
        } else {
            std::fs::write(&vscode_settings_path, templated_settings)?;
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
                info!("Replacing json key: {} with value: {}", key, value);
                target.insert(key.clone(), value.clone());
            }
        } else {
            warn!("Could not merge json, overrides and target are not objects");
        }
    }
}

/// Because we are likely already runnnig in the context of a cargo invocation,
/// some environment variables may be set that we don't want to inherit.
/// Set them to MAIN_CARGO_<VARIABLE> so that we can reference them but so they dont get inherited by further cargo commands
fn pop_cargo_env() -> Result<()> {
    let env = std::env::vars().collect::<Vec<_>>();
    for (key, value) in env.iter() {
        if key.starts_with("CARGO_") {
            let new_key = format!("MAIN_{}", key);
            std::env::set_var(new_key, value);
            std::env::remove_var(key);
        }
    }

    // unset some other variables
    let remove_vars = ["RUSTUP_TOOLCHAIN"];
    for var in remove_vars.iter() {
        std::env::remove_var(var);
    }

    Ok(())
}

fn try_main() -> Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();
    pop_cargo_env()?;
    let args = App::try_parse()?;
    let out = args.subcmd.run(args.global_args)?;
    // push any output to stdout
    if !out.is_empty() {
        std::io::stdout().write_all(out.as_bytes())?;
        println!();
    }
    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}
