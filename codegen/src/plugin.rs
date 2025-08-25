use std::env;

use cargo_metadata::camino::Utf8Path;
use clap::Parser;
use log::debug;

use crate::{
    BevyAnalyzerCallbacks, WorkspaceMeta,
    driver::{CrateFilter, RustcPluginArgs},
};

pub struct BevyAnalyzer;
impl crate::driver::RustcPlugin for BevyAnalyzer {
    type Args = crate::Args;

    fn version(&self) -> std::borrow::Cow<'static, str> {
        env!("CARGO_PKG_VERSION").into()
    }

    fn driver_name(&self) -> std::borrow::Cow<'static, str> {
        "bms-codegen-driver".into()
    }

    fn args(&self, target_dir: &Utf8Path) -> RustcPluginArgs<crate::Args> {
        debug!("Target dir: {target_dir}");

        RustcPluginArgs {
            args: crate::Args::parse_from(std::env::args().skip(1)),
            filter: CrateFilter::OnlyWorkspace,
        }
    }

    fn run(self, compiler_args: Vec<String>, plugin_args: Self::Args) {
        let mut callbacks = BevyAnalyzerCallbacks::new(plugin_args);

        rustc_driver_impl::run_compiler(&compiler_args, &mut callbacks);
        log::trace!("Finished compiling with plugin");
    }

    fn modify_cargo(&self, cmd: &mut std::process::Command, args: &Self::Args) {
        *cmd = copy_command_without_args(cmd, &["-q", "-v", "--all", "--workspace"]);
        cmd.args(["--color", "always"]);
        if !args.features.is_empty() {
            cmd.args(["--features", &args.features.join(",")]);
        }

        if args.no_default_features {
            cmd.arg("--no-default-features");
        }

        // make cargo chatty as well
        if args.verbose.get_log_level_int() >= 3 {
            cmd.arg("-v");
        } else {
            cmd.arg("-q");
        }

        if let Some(crates) = WorkspaceMeta::from_env().include_crates {
            for c in crates {
                cmd.args(["-p", &c]);
            }
        }

        let all_env = cmd
            .get_envs()
            .filter_map(|(key, val)| {
                val.map(|val| format!("{}={}", key.to_string_lossy(), val.to_string_lossy()))
            })
            .collect::<Vec<_>>()
            .join(" ");
        let bin_name = cmd.get_program().to_string_lossy();
        let args = cmd
            .get_args()
            .map(|a| a.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");
        log::debug!("Running: \n{all_env} {bin_name} {args}",);
    }
}

fn copy_command_without_args(
    cmd: &std::process::Command,
    arg_filter: &[&str],
) -> std::process::Command {
    let mut new_cmd = std::process::Command::new(cmd.get_program());
    new_cmd.args(
        cmd.get_args()
            .filter(|a| !arg_filter.iter().any(|f| f == a)),
    );
    new_cmd.envs(cmd.get_envs().filter_map(|(a, b)| b.map(|b| (a, b))));
    new_cmd
}
