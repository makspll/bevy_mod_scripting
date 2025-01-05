use anyhow::*;
use clap::Parser;
use log::*;
use std::{process::Command, str::FromStr};
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

#[derive(Debug, Clone)]
struct Features(Vec<Feature>);

impl Features {
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
    /// Build the main workspace, and then run all tests
    Test,
    /// Perform a full check as it would be done in CI
    CiCheck,
}

impl Xtasks {
    fn run(self, features: Features) -> Result<(), Error> {
        match self {
            Xtasks::Build => Self::build(features),
            Xtasks::Check => self.check(),
            Xtasks::Test => self.test(),
            Xtasks::CiCheck => self.cicd(),
            Xtasks::Init => self.init(),
        }
    }

    fn run_workspace_command(
        command: &str,
        context: &str,
        features: Features,
    ) -> Result<(), anyhow::Error> {
        info!("Running workspace command: {}", command);

        let mut args = vec![];
        args.push(command.to_owned());
        args.push("--workspace".to_owned());
        args.extend(features.to_cargo_args());

        let mut cmd = Command::new("cargo");
        cmd.args(args)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());

        info!("Using command: {:?}", cmd);

        let output = cmd.output().with_context(|| context.to_owned())?;
        match output.status.code() {
            Some(0) => Ok(()),
            _ => bail!(
                "{} failed with exit code: {}",
                context,
                output.status.code().unwrap_or(-1)
            ),
        }
    }

    fn build(features: Features) -> Result<(), anyhow::Error> {
        // build workspace using the given features
        Self::run_workspace_command("build", "Failed to build workspace", features)?;
        Ok(())
    }

    fn check(self) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn test(self) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn cicd(self) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn init(self) -> Result<(), anyhow::Error> {
        todo!()
    }
}

fn try_main() -> Result<(), anyhow::Error> {
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
