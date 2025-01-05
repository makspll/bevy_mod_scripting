use anyhow::*;
use clap::Parser;
use itertools::Itertools;
use log::*;
use std::{collections::HashMap, ffi::OsStr, path::Path, process::Command, str::FromStr};
use strum::VariantNames;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    strum::EnumString,
    strum::EnumIter,
    strum::Display,
    strum::VariantNames,
    strum::VariantArray,
)]
#[strum(serialize_all = "snake_case")]
enum Feature {
    // Lua
    Lua,
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
    Rhai,

    // Rune
    Rune,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum FeatureGroup {
    LuaExclusive,
    RhaiExclusive,
    RuneExclusive,
    NonExclusiveOther,
}

impl FeatureGroup {
    fn default_feature(self) -> Feature {
        match self {
            FeatureGroup::LuaExclusive => Feature::Lua54,
            FeatureGroup::RhaiExclusive => Feature::Rhai,
            FeatureGroup::RuneExclusive => Feature::Rune,
            FeatureGroup::NonExclusiveOther => panic!("No default feature for non-exclusive group"),
        }
    }
}

trait IntoFeatureGroup {
    fn to_feature_group(self) -> FeatureGroup;
}

impl IntoFeatureGroup for Feature {
    fn to_feature_group(self) -> FeatureGroup {
        match self {
            Feature::Lua
            | Feature::Lua51
            | Feature::Lua52
            | Feature::Lua53
            | Feature::Lua54
            | Feature::Luajit
            | Feature::Luajit52
            | Feature::Luau => FeatureGroup::LuaExclusive,
            Feature::Rhai => FeatureGroup::RhaiExclusive,
            Feature::Rune => FeatureGroup::RuneExclusive,
            _ => FeatureGroup::NonExclusiveOther,
        }
    }
}

#[derive(Debug, Clone)]
struct Features(Vec<Feature>);

impl Features {
    /// Returns all features except the exclusive ones which are not the default
    fn all_features() -> Self {
        // remove exclusive features which are not the default
        Self(
            <Feature as strum::VariantArray>::VARIANTS
                .iter()
                .filter(|f| {
                    let group = f.to_feature_group();
                    (group == FeatureGroup::NonExclusiveOther) || (**f == group.default_feature())
                })
                .cloned()
                .collect(),
        )
    }

    fn to_cargo_args(&self) -> Vec<String> {
        if self.0.is_empty() {
            vec![]
        } else {
            vec!["--features".to_owned(), self.to_string()]
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
        for (i, feature) in self.0.iter().enumerate() {
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
            return Self(vec![]);
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

#[derive(Debug, Parser)]
struct App {
    #[clap(long, short, global = true, value_parser=clap::value_parser!(Features), value_name=Features::to_placeholder(), default_value="lua54",required = false)]
    features: Features,

    #[clap(subcommand)]
    subcmd: Xtasks,
}

#[derive(Debug, clap::Subcommand)]
#[clap(
    name = "xtask",
    bin_name = "cargo xtask",
    about = "A set of xtasks for managing the project. Run 'cargo xtask init' to get started."
)]
enum Xtasks {
    /// Performs first time local-development environment setup
    Init,
    /// Build the main workspace only
    Build,
    /// Build the main workspace, apply all prefferred lints
    Check,
    /// Build the rust crates.io docs as well as any other docs
    Docs {
        /// Open in browser
        /// This will open the generated docs in the default browser
        #[clap(long, short)]
        open: bool,
    },
    /// Build the main workspace, and then run all tests
    Test,
    /// Perform a full check as it would be done in CI
    CiCheck,
}

impl Xtasks {
    fn run(self, features: Features) -> Result<()> {
        match self {
            Xtasks::Build => Self::build(features),
            Xtasks::Check => Self::check(features),
            Xtasks::Docs { open } => Self::docs(open),
            Xtasks::Test => Self::test(features),
            Xtasks::CiCheck => Self::cicd(),
            Xtasks::Init => Self::init(),
        }
    }

    fn cargo_metadata() -> Result<cargo_metadata::Metadata> {
        let cargo_manifest_path = std::env::var("CARGO_MANIFEST_PATH").unwrap();

        let mut cmd = cargo_metadata::MetadataCommand::new();
        cmd.manifest_path(cargo_manifest_path);
        let out = cmd.exec()?;
        Ok(out)
    }

    fn workspace_dir() -> Result<std::path::PathBuf> {
        let metadata = Self::cargo_metadata()?;
        let workspace_root = metadata.workspace_root;
        Ok(workspace_root.into())
    }

    fn relative_workspace_dir<P: AsRef<Path>>(dir: P) -> Result<std::path::PathBuf> {
        let workspace_dir = Self::workspace_dir()?;
        Ok(workspace_dir.join(dir))
    }

    fn run_system_command<I: IntoIterator<Item = impl AsRef<OsStr>>>(
        command: &str,
        context: &str,
        add_args: I,
        dir: Option<&Path>,
    ) -> Result<()> {
        info!("Running system command: {}", command);

        let working_dir = match dir {
            Some(d) => Self::relative_workspace_dir(d)?,
            None => Self::workspace_dir()?,
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
        command: &str,
        context: &str,
        features: Features,
        add_args: I,
        dir: Option<&Path>,
    ) -> Result<()> {
        info!("Running workspace command: {}", command);

        let mut args = vec![];
        args.push(command.to_owned());
        args.push("--workspace".to_owned());
        args.extend(features.to_cargo_args());
        args.extend(add_args.into_iter().map(|s| {
            s.as_ref()
                .to_str()
                .expect("invalid command argument")
                .to_owned()
        }));

        let working_dir = match dir {
            Some(d) => Self::relative_workspace_dir(d)?,
            None => Self::workspace_dir()?,
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
                "{} failed with exit code: {}. Features: {features}",
                context,
                output.status.code().unwrap_or(-1)
            ),
        }
    }

    fn build(features: Features) -> Result<()> {
        // build workspace using the given features
        Self::run_workspace_command(
            "build",
            "Failed to build workspace",
            features,
            vec!["--all-targets"],
            None,
        )?;
        Ok(())
    }

    fn check(features: Features) -> Result<()> {
        // start with cargo clippy
        Self::run_workspace_command(
            "clippy",
            "Failed to run clippy",
            features,
            vec!["--all-targets", "--", "-D", "warnings"],
            None,
        )?;

        // run cargo fmt checks
        Self::run_system_command(
            "cargo",
            "Failed to run cargo fmt",
            vec!["fmt", "--all", "--", "--check"],
            None,
        )?;

        Ok(())
    }

    fn docs(open: bool) -> Result<()> {
        // find [package.metadata."docs.rs"] key in Cargo.toml
        let metadata = Self::cargo_metadata()?;

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

        let features = Features(string_list);

        let mut args = Vec::default();
        args.push("--all");
        if open {
            args.push("--open");
        }
        Self::run_workspace_command(
            "doc",
            "Failed to build crates.io docs",
            features.clone(),
            args,
            None,
        )?;

        // build mdbook
        let args = if open { vec!["build"] } else { vec!["serve"] };

        Self::run_system_command(
            "mdbook",
            "Failed to build or serve mdbook docs",
            args,
            Some(Path::new("docs")),
        )?;

        Ok(())
    }

    fn test(features: Features) -> Result<()> {
        // run cargo test with instrumentation
        std::env::set_var("CARGO_INCREMENTAL", "0");
        std::env::set_var("RUSTFLAGS", "-Cinstrument-coverage");
        let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_owned());
        let coverage_dir = std::path::PathBuf::from(target_dir).join("coverage");
        let coverage_file = coverage_dir.join("cargo-test-%p-%m.profraw");

        // clear coverage directory
        assert!(coverage_dir != std::path::Path::new("/"));
        std::fs::remove_dir_all(coverage_dir)?;

        std::env::set_var("LLVM_PROFILE_FILE", coverage_file);

        Self::run_workspace_command(
            "test",
            "Failed to run tests",
            features,
            vec!["--exclude", "xtask"],
            None,
        )?;

        // generate coverage report and lcov file
        Self::run_system_command(
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
                "../*",
                "--ignore",
                "/*",
                "-o",
                "target/coverage/html",
            ],
            None,
        )?;

        Self::run_system_command(
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
                "../*",
                "--ignore",
                "/*",
                "-o",
                "target/coverage/lcov.info",
            ],
            None,
        )
    }

    fn cicd() -> Result<()> {
        // setup the CI environment
        Self::init()?;

        // run everything with the ephemereal profile
        // first check everything compiles with every combination of features apart from mutually exclusive ones
        let all_features = Features(<Feature as strum::VariantArray>::VARIANTS.into());

        let grouped = all_features.split_by_group();

        let non_exclusive = grouped
            .get(&FeatureGroup::NonExclusiveOther)
            .unwrap_or(&vec![])
            .clone();

        // run powerset with all language features enabled without mutually exclusive
        let powersets = non_exclusive.iter().cloned().powerset().collect::<Vec<_>>();
        info!("Powerset: {:?}", powersets);
        let length = powersets.len();

        for (i, mut feature_set) in powersets.into_iter().map(Features).enumerate() {
            info!(
                "Running check {}/{length} with features: {}",
                i + 1,
                feature_set
            );
            // choose language features
            for category in [
                FeatureGroup::LuaExclusive,
                FeatureGroup::RhaiExclusive,
                FeatureGroup::RuneExclusive,
            ] {
                feature_set.0.push(category.default_feature());
            }

            Self::build(feature_set)?;
        }

        // run lints
        let all_features = Features::all_features();
        Self::check(all_features.clone())?;

        // run docs
        Self::docs(false)?;

        // run tests
        Self::test(all_features)?;

        Ok(())
    }

    fn init() -> Result<()> {
        // install cargo mdbook
        Self::run_system_command(
            "cargo",
            "Failed to install mdbook",
            vec!["install", "mdbook"],
            None,
        )?;

        // install grcov
        Self::run_system_command(
            "cargo",
            "Failed to install grcov",
            vec!["install", "grcov"],
            None,
        )?;

        // install llvm-tools and clippy
        Self::run_system_command(
            "rustup",
            "Failed to install rust components",
            vec![
                "component",
                "add",
                "rust-src",
                "rustc-dev",
                "clippy",
                "llvm-tools-preview",
            ],
            None,
        )?;

        Ok(())
    }
}

fn try_main() -> Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();
    let args = App::try_parse()?;
    args.subcmd.run(args.features)
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
