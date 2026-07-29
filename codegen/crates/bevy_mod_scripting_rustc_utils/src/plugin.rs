use bevy_mod_scripting_rustc_driver::{RustcPlugin, RustcPluginArgs, copy_command_without_args};
use cargo_metadata::camino::Utf8Path;
use clap::Parser;
use rustc_driver::{Callbacks, Compilation};
use rustc_hir::{
    def_id::{DefId, LOCAL_CRATE},
};
use rustc_span::Ident;

use crate::{
    args::Cli, emitter::{CaptureState}, query::{query_crate_non_trait_impls, type_implements_trait, typing_env_function_arg_in_impl},
};

pub struct RustcUtilsPlugin {}

pub struct RustcUtilsCallbacks {
    args: Cli,
}

impl Callbacks for RustcUtilsCallbacks {
    fn after_expansion(
        &mut self,
        compiler: &rustc_interface::interface::Compiler,
        tcx: rustc_middle::ty::TyCtxt<'_>,
    ) -> rustc_driver::Compilation {
        let sess = &compiler.sess;

        if sess.dcx().has_errors().is_some() {
            sess.dcx().fatal("compilation failed, aborting analysis.");
        }

        CaptureState::install(sess);

        match &self.args.command {
            crate::args::Command::ArgImplements(arg_implements_args) => {
                let trait_did = match tcx
                    .traits(LOCAL_CRATE)
                    .iter()
                    .find(|t| tcx.item_ident(**t).to_string() == arg_implements_args.trait_name)
                {
                    Some(trait_did) => *trait_did,
                    None => {
                        println!(
                            "no: trait '{}' not found in crate",
                            arg_implements_args.trait_name
                        );
                        return rustc_driver::Compilation::Continue;
                    }
                };

                let impl_did: Option<DefId> =
                    query_crate_non_trait_impls(tcx, |tcx, type_did, _impl_did| {
                        tcx.hir_ident(tcx.hir_expect_item(*type_did).hir_id())
                            .to_string()
                            == arg_implements_args.type_name
                    })
                    .map(|(_, i)| *i)
                    .collect::<Vec<_>>()
                    .first()
                    .copied();

                let impl_did = match impl_did {
                    Some(impl_did) => impl_did,
                    None => {
                        println!(
                            "no: impl block not found for type: {}",
                            arg_implements_args.type_name
                        );
                        return Compilation::Continue
                    }
                };

                let items = tcx.associated_items(impl_did);

                let func_did = items.find_by_ident_and_kind(tcx, Ident::from_str(&arg_implements_args.function), rustc_middle::ty::AssocTag::Fn, impl_did);

                let func_did = match func_did {
                    Some(func_did) => func_did.def_id,
                    None => {
                        println!("no: could not find function: {} on any impl block for type: {}", arg_implements_args.function, arg_implements_args.type_name);
                        return Compilation::Continue;
                    }
                };              

                let bound_sig = tcx.fn_sig(func_did);

 
                let (typing_env, fn_sig) =
                    typing_env_function_arg_in_impl(tcx, bound_sig, func_did, impl_did);


                let param = match tcx.fn_arg_idents(func_did).iter().zip(fn_sig.inputs()).find(|(param, _)| {
                    param.is_some_and(|p| p.to_string() == arg_implements_args.arg)
                }) {
                    Some((_,param)) => param,
                    None => {
                        println!("no: no parameter named {} found on function", arg_implements_args.arg);
                        return Compilation::Continue;
                    },
                };

                match type_implements_trait(tcx, typing_env, *param, trait_did) {
                    Ok(_) => println!("yes"),
                    Err(err) => {
                        println!("no: {err}")
                    },
                };

                }
            }
            rustc_driver::Compilation::Continue
        }
}
    


impl RustcPlugin for RustcUtilsPlugin {
    type Args = Cli;

    fn version(&self) -> std::borrow::Cow<'static, str> {
        env!("CARGO_PKG_VERSION").into()
    }

    fn driver_name(&self) -> std::borrow::Cow<'static, str> {
        "bms-rustc-utils-driver".into()
    }

    fn args(
        &self,
        _target_dir: &Utf8Path,
    ) -> bevy_mod_scripting_rustc_driver::RustcPluginArgs<Self::Args> {
        RustcPluginArgs {
            args: Cli::parse_from(std::env::args().skip(1)),
            filter: bevy_mod_scripting_rustc_driver::CrateFilter::AllCrates,
        }
    }

    fn run(self, compiler_args: Vec<String>, plugin_args: RustcPluginArgs<Self::Args>) {
        let mut callbacks = RustcUtilsCallbacks { args: plugin_args.args };

        rustc_driver_impl::run_compiler(&compiler_args, &mut callbacks);
    }
     fn modify_cargo(&self, cmd: &mut std::process::Command, _args: &Self::Args) {
        *cmd = copy_command_without_args(cmd, &["-q", "-v",]); //"--all", "--workspace"]);
        cmd.args(["--color", "always"]);
        // if !args.features.is_empty() {
        //     cmd.args(["--features", &args.features.join(",")]);
        // }

        // if args.no_default_features {
        //     cmd.arg("--no-default-features");
        // }

        // // make cargo chatty as well
        // if args.verbose.get_log_level_int() >= 3 {
            // cmd.arg("-q");
        // } else {
        //     cmd.arg("-q");
        // }

        // if let Some(crates) = WorkspaceMeta::from_env().include_crates {
        //     for c in crates {
        //         cmd.args(["-p", &c]);
        //     }
        // }

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
        log::debug!("Running cargo build command: \n{all_env} {bin_name} {args}",);
    }
}
