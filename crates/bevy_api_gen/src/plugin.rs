use cargo_metadata::camino::Utf8PathBuf;
use clap::{Parser, Subcommand, ValueEnum};
use log::debug;
use rustc_plugin::{CrateFilter, RustcPlugin, RustcPluginArgs, Utf8Path};
use serde::{Deserialize, Serialize};
use std::env;
use strum::{Display, EnumString, IntoStaticStr, VariantArray, VariantNames};

use crate::{modifying_file_loader::ModifyingFileLoader, BevyAnalyzerCallbacks};

pub struct BevyAnalyzer;

#[derive(Parser, Deserialize, Serialize)]
#[command(
    version,
    about,
    long_about,
    disable_help_flag = true,
    bin_name = "cargo bevy-api-gen",
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

pub const TARGET_DIR_ENV_NAME: &str = "PLUGIN_TARGET_DIR";
fn compute_default_dir() -> String {
    const DEFAULT_DIR: &str = "bevy_api_gen";
    //  this will be called a number of times, when it matters it won't default,
    // if the args are passed anywhere else this will act as dummy default
    Utf8PathBuf::from(std::env::var(TARGET_DIR_ENV_NAME).unwrap_or_default())
        .join(DEFAULT_DIR)
        .to_string()
}

#[derive(Subcommand, Deserialize, Serialize, strum::EnumIs)]
pub enum Command {
    /// Prints built-in templates to stdout
    ///
    /// See `List`` for a list of available templates
    Print {
        /// Specify the name of a template to print, can be used t
        #[arg(value_enum, value_name = "TEMPLATE")]
        template: TemplateKind,
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
        #[arg(short, long, value_name = "NAME", default_value = "BevyAPIProvider")]
        api_name: String,
    },
}

/// Describes the available templates and overrides
#[derive(
    Display,
    EnumString,
    VariantNames,
    VariantArray,
    IntoStaticStr,
    Clone,
    Copy,
    Serialize,
    Deserialize,
)]
pub enum TemplateKind {
    #[strum(to_string = "mod.tera")]
    SharedModule,
    #[strum(to_string = "crate.tera")]
    CrateArtifact,
    #[strum(to_string = "field.tera")]
    Field,
    #[strum(to_string = "function.tera")]
    Function,
    #[strum(to_string = "item.tera")]
    Item,
    #[strum(to_string = "header.tera")]
    Header,
    #[strum(to_string = "footer.tera")]
    Footer,
}

impl ValueEnum for TemplateKind {
    fn value_variants<'a>() -> &'a [Self] {
        <Self as VariantArray>::VARIANTS
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        let static_str: &'static str = self.into();
        Some(clap::builder::PossibleValue::new(static_str))
    }
}

impl RustcPlugin for BevyAnalyzer {
    type Args = Args;

    fn version(&self) -> std::borrow::Cow<'static, str> {
        env!("CARGO_PKG_VERSION").into()
    }

    fn driver_name(&self) -> std::borrow::Cow<'static, str> {
        "bevy-api-gen-driver".into()
    }

    fn args(&self, target_dir: &Utf8Path) -> rustc_plugin::RustcPluginArgs<Self::Args> {
        debug!("Target dir: {}", target_dir);

        RustcPluginArgs {
            args: Args::parse_from(std::env::args().skip(1)),
            filter: CrateFilter::AllCrates,
        }
    }

    fn run(
        self,
        compiler_args: Vec<String>,
        plugin_args: Self::Args,
    ) -> rustc_interface::interface::Result<()> {
        let mut callbacks = BevyAnalyzerCallbacks::new(plugin_args);
        let mut compiler = rustc_driver::RunCompiler::new(&compiler_args, &mut callbacks);
        compiler.set_file_loader(Some(Box::new(ModifyingFileLoader)));
        compiler.run()
    }
}
