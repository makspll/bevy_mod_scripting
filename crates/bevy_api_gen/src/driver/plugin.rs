use std::{borrow::Cow, process::Command};

use cargo_metadata::camino::Utf8Path;
use serde::{Serialize, de::DeserializeOwned};

/// Specification of a set of crates.
pub enum CrateFilter {
    /// Every crate in the workspace and all transitive dependencies.
    AllCrates,
    /// Just crates in the workspace.
    OnlyWorkspace,
}

/// Arguments from your plugin to the rustc_plugin framework.
pub struct RustcPluginArgs<Args> {
    /// Whatever CLI arguments you want to pass along.
    pub args: Args,

    /// Which crates you want to run the plugin on.
    pub filter: CrateFilter,
}

/// Interface between your plugin and the rustc_plugin framework.
pub trait RustcPlugin: Sized {
    /// Command-line arguments passed by the user.
    type Args: Serialize + DeserializeOwned;

    /// Returns the version of your plugin.
    ///
    /// A sensible default is your plugin's Cargo version:
    ///
    /// ```ignore
    /// env!("CARGO_PKG_VERSION").into()
    /// ```
    fn version(&self) -> Cow<'static, str>;

    /// Returns the name of your driver binary as it's installed in the filesystem.
    ///
    /// Should be just the filename, not the full path.
    fn driver_name(&self) -> Cow<'static, str>;

    /// Parses and returns the CLI arguments for the plugin.
    fn args(&self, target_dir: &Utf8Path) -> RustcPluginArgs<Self::Args>;

    /// Optionally modify the `cargo` command that launches rustc.
    /// For example, you could pass a `--feature` flag here.
    fn modify_cargo(&self, _cargo: &mut Command, _args: &Self::Args) {}

    /// Executes the plugin with a set of compiler and plugin args.
    fn run(self, compiler_args: Vec<String>, plugin_args: Self::Args);
}

/// The name of the environment variable shared between the CLI and the driver.
/// Must not conflict with any other env var used by Cargo.
pub const PLUGIN_ARGS: &str = "PLUGIN_ARGS";
