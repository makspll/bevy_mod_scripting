use clap::Parser;
use log::debug;
use rustc_plugin::{CrateFilter, RustcPlugin, RustcPluginArgs, Utf8Path};

use std::env;

use crate::{modifying_file_loader::ModifyingFileLoader, BevyAnalyzerCallbacks};

pub struct BevyAnalyzer;
impl RustcPlugin for BevyAnalyzer {
    type Args = crate::Args;

    fn version(&self) -> std::borrow::Cow<'static, str> {
        env!("CARGO_PKG_VERSION").into()
    }

    fn driver_name(&self) -> std::borrow::Cow<'static, str> {
        "bevy-api-gen-driver".into()
    }

    fn args(&self, target_dir: &Utf8Path) -> rustc_plugin::RustcPluginArgs<Self::Args> {
        debug!("Target dir: {}", target_dir);

        RustcPluginArgs {
            args: crate::Args::parse_from(std::env::args().skip(1)),
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
