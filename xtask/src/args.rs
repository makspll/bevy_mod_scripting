use std::path::PathBuf;

use crate::features::Features;
use clap::Parser;

#[derive(Debug, Parser, Clone)]
pub struct GlobalArgs {
    #[clap(long, short, global = true, value_parser=clap::value_parser!(Features), value_name=Features::to_placeholder(), default_value=Features::default().to_string(),required = false)]
    pub features: Features,

    #[clap(
        long,
        global = true,
        default_value = "false",
        help = "Enable coverage collection for cargo commands"
    )]
    pub coverage: bool,

    #[clap(skip)]
    pub override_workspace_dir: Option<PathBuf>,

    #[clap(skip)]
    pub override_toolchain: Option<String>,

    #[clap(
        long,
        short,
        global = true,
        value_name = "PROFILE",
        help = "The cargo profile to use for commands that support it"
    )]
    pub profile: Option<String>,

    #[clap(
        long,
        global = true,
        value_name = "JOBS",
        help = "The number of parallel jobs to run at most"
    )]
    pub jobs: Option<usize>,
}

impl GlobalArgs {
    pub fn with_max_jobs(self, jobs: usize) -> Self {
        Self {
            jobs: Some(jobs),
            ..self
        }
    }

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
