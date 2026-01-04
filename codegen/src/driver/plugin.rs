use std::{borrow::Cow, process::Command};

use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub enum RunMode {
    NormalRustc,
    Plugin,
    Skip,
}

/// Interface between your plugin and the rustc_plugin framework.
pub trait RustcPlugin: Sized + Serialize + DeserializeOwned {
    /// Returns the version of your plugin.
    ///
    /// A sensible default is your plugin's Cargo version:
    ///
    /// ```ignore
    /// env!("CARGO_PKG_VERSION").into()
    /// ```
    fn version() -> Cow<'static, str>;

    /// Returns the name of your driver binary as it's installed in the filesystem.
    ///
    /// Should be just the filename, not the full path.
    fn driver_name(&self) -> Cow<'static, str>;

    /// Optionally modify the `cargo` command that launches rustc.
    /// For example, you could pass a `--feature` flag here.
    fn modify_cargo(&self, _cargo: &mut Command) {}

    fn modify_rustc(&self, _compiler_args: &mut Vec<String>) {}

    /// Executes the plugin with a set of compiler and plugin args.
    fn run(self, crate_name: &str, compiler_args: Vec<String>, is_not_build_invocation: bool);

    /// To be used after the PAYLOAD is initialized
    fn initialize_from_env() -> Self {
        serde_json::from_str(&std::env::var(PLUGIN_PAYLOAD).unwrap()).unwrap()
    }

    /// persists the plugin in the process environment for children to be able to call [`Self::initialize_from_env`]
    fn serialize_to_env(&self) {
        unsafe { std::env::set_var(PLUGIN_PAYLOAD, serde_json::to_string(&self).unwrap()) }
    }
}

pub const PLUGIN_PAYLOAD: &str = "PLUGIN_PAYLOAD";
