use std::{env, io, str::FromStr};

use log::{debug, LevelFilter};
use rustc_plugin::{CrateFilter, RustcPlugin, RustcPluginArgs, Utf8Path};
use rustc_session::config::Input;

use crate::{modifying_file_loader::ModifyingFileLoader, BevyAnalyzerCallbacks};

pub struct BevyAnalyzer;

impl RustcPlugin for BevyAnalyzer {
    type Args = ();

    fn version(&self) -> std::borrow::Cow<'static, str> {
        env!("CARGO_PKG_VERSION").into()
    }

    fn driver_name(&self) -> std::borrow::Cow<'static, str> {
        "bevy-analyzer-driver".into()
    }

    fn args(&self, target_dir: &Utf8Path) -> rustc_plugin::RustcPluginArgs<Self::Args> {
        // TODO: integrate with clap for args
        RustcPluginArgs {
            args: (),
            filter: CrateFilter::AllCrates,
        }
    }

    fn run(
        self,
        compiler_args: Vec<String>,
        plugin_args: Self::Args,
    ) -> rustc_interface::interface::Result<()> {
        let mut callbacks = BevyAnalyzerCallbacks;
        let mut compiler = rustc_driver::RunCompiler::new(&compiler_args, &mut callbacks);
        compiler.set_file_loader(Some(Box::new(ModifyingFileLoader)));
        compiler.run()
    }
}
