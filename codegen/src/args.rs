use cargo_metadata::camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Deserialize, Serialize)]
#[command(
    version,
    about,
    long_about,
    disable_help_flag = true,
    bin_name = "cargo bms-codegen",
    arg_required_else_help = true
)]
/// A Cargo plugin which can generate reflection powered wrappers for Bevy types,
/// list `Reflect` types in a workspace and perform arbitrary codegen using Tera templates.
///
/// Generally performs the following steps:
///
/// 1.Compiles crate as normal up untill the HIR stage
///
/// 2.Analyses the HIR for types which implement `Reflect`
///
/// 3.Gathers data about the sort of methods and fields available on the type together with their reflection `strategy` when used in other wrappers (Use wrapper, Do not use reflection, Skip, etc..)
///
/// 4.Generates a `meta` file which can be used when generating code for other crates to understand the reflection strategy of the types here
///
/// 5.Generates code using Tera templates given the data gathered from the analysis
pub struct Args {
    #[command(subcommand)]
    pub cmd: Command,

    #[command(flatten)]
    pub verbose: Verbosity,

    /// Features to enable when running cargo, requires workspace_root to be provided to work properly
    #[arg(
        global = true,
        short,
        long,
        default_value = "",
        use_value_delimiter = true,
        value_delimiter = ','
    )]
    pub features: Vec<String>,

    /// Disable default features when running cargo, requires workspace_root to be provided to work properly
    #[arg(global = true, long, default_value = "false")]
    pub no_default_features: bool,

    /// If provided will use the workspace root to calculate effective dependencies and only generate code for currently active features
    #[arg(
        global = true,
        long,
        default_value = "bevy",
        help = "Deprecated: root is now automatically detected from cargo metadata"
    )]
    pub workspace_root: Option<String>,

    /// additional template context in the form of json, provided to the templates under an 'args' key
    #[arg(global = true, long)]
    pub template_args: Option<String>,

    /// The path to the bevy_mod_scripting_bindings crate, used to bootstrap necessary traits
    #[arg(global = true, long, default_value = ".")]
    pub bms_bindings_path: Utf8PathBuf,

    /// Crates to exclude from code generation
    #[arg(
        global = true,
        short,
        long,
        default_value = "bevy_winit,bevy_window",
        use_value_delimiter = true,
        value_delimiter = ','
    )]
    pub exclude_crates: Option<Vec<String>>,
}

#[derive(clap::Args, Debug, Clone, Default, Serialize, Deserialize)]
#[command(about = None, long_about = None)]
pub struct Verbosity {
    #[arg(
        long,
        short = 'v',
        help = "Increase verbosity, can be used multiple times to increase verbosity further",
        action = clap::ArgAction::Count,
        global = true,
    )]
    pub verbose: u8,

    #[arg(
        long,
        short = 'q',
        help = "Decrease verbosity, can be used multiple times to decrease verbosity further",
        action = clap::ArgAction::Count,
        global = true,
        conflicts_with = "verbose",
    )]
    pub quiet: u8,
}

impl Verbosity {
    pub fn get_log_level_int(&self) -> i8 {
        (self.verbose as i8) - (self.quiet as i8)
    }
    pub fn get_log_level(&self) -> log::Level {
        match self.get_log_level_int() {
            0 => log::Level::Info,
            1 => log::Level::Debug,
            x if x >= 2 => log::Level::Trace,
            _ => log::Level::Error,
        }
    }

    pub fn get_rustlog_value(&self) -> String {
        let make_string = |level| {
            format!(
                "bms-codegen-driver={level},cargo-bms-codegen={level},bevy_mod_scripting_codegen={level},warning"
            )
        };

        match self.get_log_level_int() {
            0 => make_string("info"),
            1 => make_string("debug"),
            x if x >= 2 => make_string("trace"),
            _ => make_string("error"),
        }
    }
}

fn default_ignored_types() -> String {
    [
        "std::any::TypeId",
        "bevy_reflect::DynamicArray",
        "bevy_reflect::DynamicList",
        "bevy_reflect::DynamicMap",
        "bevy_reflect::DynamicStruct",
        "bevy_reflect::DynamicTuple",
        "bevy_reflect::DynamicTupleStruct",
        "bevy_reflect::DynamicEnum",
        "bevy_reflect::DynamicSet",
        "bevy_reflect::OsString", // TODO: once macros allow Vecs for primitives as args remove this from ignored types
    ]
    .join(",")
}

#[derive(Subcommand, Deserialize, Serialize, strum::EnumIs)]
pub enum Command {
    /// Prints built-in templates to stdout
    ///
    /// See `List`` for a list of available templates
    Print {
        /// Specify the name of a template to print, can be used t
        #[arg(value_enum, value_name = "TEMPLATE")]
        template: crate::TemplateKind,
    },
    /// Lists all available reflect types in the crate
    ListTypes,
    /// Lists all available built-in templates, each one can be overriden by providing a file with the same name in the `templates` directory
    ListTemplates,
    /// Crawls current workspace/crate and generates wrappers for Reflect types using templates
    Generate {
        /// the output and artifact directory
        #[arg(short, long, default_value = compute_default_dir(), value_name = "DIR")]
        output: Utf8PathBuf,

        /// The directory in which to look for templates, if unspecified will use built-in templates
        #[arg(short, long, value_name = "DIR")]
        templates: Option<Utf8PathBuf>,

        /// If true will include private types in the generated code, by default only publically accessibl types are included
        #[arg(long, default_value = "false")]
        include_private: bool,

        /// Optional additional directories to be added to the meta file search path.
        ///
        /// You can specify multiple directories by providing multiple meta arguments.
        ///
        /// Meta files are produced as part of the codegen and contain information about the generated wrappers from each crate.
        /// If you reference types from other crates, make sure you add the directory of the output directory of the other crate here, for the types to be
        /// properly resolved.
        #[arg(short, long, value_name = "DIR")]
        meta: Option<Vec<Utf8PathBuf>>,

        /// Optional directory to write meta files to, if different from output, will also add this directory to the search path for meta files.
        ///
        /// Handy in a multi-crate environment, if you specify the same directory here for all your crates and run codegen in dependency order, you will
        /// be able to resolve all types from all crates.
        #[arg(short, long, value_name = "DIR")]
        meta_output: Option<Utf8PathBuf>,

        /// If specified will only generate the template data in json format for each crate and not generate any code.
        ///
        /// The data returned is the same as the one provided to the templates.
        #[arg(long, action)]
        template_data_only: bool,

        #[arg(
            long,
            default_value = default_ignored_types(),
            use_value_delimiter = true,
            value_delimiter = ','
        )]
        ignored_types: Vec<String>,
    },
    // /// Final step, once you generate all the crate files you would like to have in your module, you can run this command to
    // /// generate a `mod.rs` file using the `collect.rs` template, which will be provided with all the generated filenames and can 'collect' all the items as it wishes
    Collect {
        /// the output and artifact directory
        #[arg(short, long, default_value = compute_default_dir(), value_name = "DIR")]
        output: Utf8PathBuf,

        /// The directory in which to look for templates, if unspecified will use built-in templates
        #[arg(short, long, value_name = "DIR")]
        templates: Option<Utf8PathBuf>,

        /// The name of the API, this will be passed to the `collect.rs` template, which by default will be used as the APIProvider name and the
        /// title of the documentation.
        #[arg(
            short,
            long,
            value_name = "NAME",
            default_value = "LuaBevyScriptingPlugin"
        )]
        api_name: String,
    },
}

pub(crate) fn compute_default_dir() -> String {
    WorkspaceMeta::from_env().plugin_target_dir.to_string()
}

/// Utility for storing and retrieving workspace meta information in env vars
#[derive(Default, Clone)]
pub struct WorkspaceMeta {
    /// the crates in the workspace
    pub crates: Vec<String>,
    pub plugin_target_dir: Utf8PathBuf,
    pub include_crates: Option<Vec<String>>,
}

impl WorkspaceMeta {
    const CRATES_ENV_NAME: &'static str = "WORKSPACE_CRATES_META";
    const PLUGIN_DIR_NAME: &'static str = "WORKSPACE_PLUGIN_DIR_META";
    const INCLUDE_CRATES_ENV_NAME: &'static str = "WORKSPACE_OPT_INCLUDE_CRATES_META";

    /// Returns true if the given crate is in the workspace and if the plugin will run on it
    pub fn is_workspace_and_included_crate(&self, crate_name: &str) -> bool {
        self.include_crates
            .as_ref()
            .map(|include_crates| include_crates.contains(&crate_name.to_owned()))
            .unwrap_or(true)
            && self.crates.contains(&crate_name.to_owned())
    }

    /// Will populate the meta from the environment variables, if empty will use defaults
    pub fn from_env() -> Self {
        Self {
            crates: std::env::var(Self::CRATES_ENV_NAME)
                .unwrap_or_default()
                .split(',')
                .map(|s| s.to_owned())
                .collect(),
            plugin_target_dir: std::env::var(Self::PLUGIN_DIR_NAME)
                .unwrap_or_default()
                .into(),
            include_crates: std::env::var(Self::INCLUDE_CRATES_ENV_NAME)
                .ok()
                .map(|s| s.split(',').map(|s| s.to_owned()).collect()),
        }
    }

    pub fn set_env(&self) {
        unsafe { std::env::set_var(Self::CRATES_ENV_NAME, self.crates.join(",")) };
        unsafe { std::env::set_var(Self::PLUGIN_DIR_NAME, &self.plugin_target_dir) };
        if let Some(include_crates) = &self.include_crates {
            unsafe { std::env::set_var(Self::INCLUDE_CRATES_ENV_NAME, include_crates.join(",")) };
        }
    }
}
