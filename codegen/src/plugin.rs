use std::{collections::HashMap, env, path::PathBuf};

use crate_feature_graph::WorkspaceGraph;
use log::{debug, trace};
use regex::Regex;
use rustc_hir::def_id::LOCAL_CRATE;
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::{
    ALL_PASSES, Args, TemplateKind, WORKSPACE_GRAPH_FILE_ENV, WorkspaceMeta,
    modifying_file_loader::ModifyingFileLoader,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Payload {
    /// The rmeta for the bootstrapped reflect crate
    pub bootstrap_deps_path: String,
    pub bootstrap_rlibs: HashMap<String, String>,
    pub include_crates: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BevyAnalyzer {
    pub payload: Payload,
    pub args: Args,
}

struct DefaultCallbacks;
impl rustc_driver::Callbacks for DefaultCallbacks {}

impl crate::driver::RustcPlugin for BevyAnalyzer {
    fn version() -> std::borrow::Cow<'static, str> {
        env!("CARGO_PKG_VERSION").into()
    }

    fn driver_name(&self) -> std::borrow::Cow<'static, str> {
        "bms-codegen-driver".into()
    }

    fn run(
        mut self,
        crate_name: &str,
        mut compiler_args: Vec<String>,
        is_not_build_invocation: bool,
    ) {
        // bootstrapped crates already have artifacts ready to inject
        // we don't want to build these, it will only generate errors
        let bootstrapped_crate = self.payload.bootstrap_rlibs.contains_key(crate_name);
        if bootstrapped_crate {
            return;
        }

        let workspace_crate = self
            .payload
            .include_crates
            .contains(&crate_name.to_string());

        self.inject_bootstrapped_artifacts(&mut compiler_args);

        // let active_features: Vec<String> = compiler_args
        //     .windows(2)
        //     .filter_map(|w| {
        //         if w[0] == "--cfg" {
        //             let arg = &w[1];

        //             // Match --cfg feature="foo"
        //             if let Some(feat) = arg.strip_prefix("feature=") {
        //                 // Trim quotes
        //                 return Some(feat.trim_matches('"').to_string());
        //             }
        //         }
        //         None
        //     })
        //     .collect();

        // log::trace!("Rustc received featuress: {}", active_features.join(", "));

        // non-workspace crates can be run as normal without any analysis
        // we don't care about these for now, but still need to modify externs to avoid errors, they need to compile fine
        if is_not_build_invocation || !workspace_crate {
            // for things like --print from cargo
            log::info!("Analysing: {crate_name}");
            rustc_driver_impl::run_compiler(&compiler_args, &mut DefaultCallbacks);
        } else {
            log::info!("Building: {crate_name}");
            rustc_driver_impl::run_compiler(&compiler_args, &mut self);
        }
    }

    fn modify_cargo(&self, cmd: &mut std::process::Command) {
        *cmd = copy_command_without_args(cmd, &["-q", "-v", "--all", "--workspace"]);
        cmd.args(["--color", "always"]);
        if !self.args.features.is_empty() {
            log::info!(
                "Running cargo with features: {}",
                self.args.features.join(",")
            );
            cmd.args(["--features", &self.args.features.join(",")]);
        }

        if self.args.no_default_features {
            cmd.arg("--no-default-features");
        }

        // make cargo chatty as well
        if self.args.verbose.get_log_level_int() >= 3 {
            cmd.arg("-vv");
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
        log::debug!("Running cargo build command: \n{all_env} {bin_name} {args}",);
    }

    fn initialize_from_env() -> Self {
        serde_json::from_str(&std::env::var(crate::PLUGIN_PAYLOAD).unwrap()).unwrap()
    }

    fn serialize_to_env(&self) {
        unsafe { std::env::set_var(crate::PLUGIN_PAYLOAD, serde_json::to_string(&self).unwrap()) }
    }
}

impl BevyAnalyzer {
    fn inject_bootstrapped_artifacts(&self, compiler_args: &mut Vec<String>) {
        log::info!("args: {compiler_args:?}");
        // we now modify externs before running anything via rustc
        let regex =
            Regex::new(r"(?P<full>(?P<path>(?:\./|/)?[A-Za-z0-9_\-./]*)?(?P<filename>lib(?P<crate>[A-Za-z0-9_]+)-(?P<hash>[A-Fa-f0-9]+)\.(rmeta|rlib)))").unwrap();

        // inject deps directory as linking dir
        compiler_args.extend([
            String::from("-L"),
            format!("dependency={}", self.payload.bootstrap_deps_path),
        ]);

        // replace existing externs, and if none are found
        // inject
        for (krate, rlib) in &self.payload.bootstrap_rlibs {
            let mut replaced = false;
            for arg in compiler_args.iter_mut() {
                for matches in regex.captures_iter(&arg.to_string()) {
                    let rlib_krate = &matches["crate"];
                    if rlib_krate == krate {
                        let found_rlib = &matches["full"];
                        log::info!("replacing rlib {found_rlib} with {rlib} for crate {krate}");
                        *arg = arg.replace(found_rlib, rlib);
                        replaced = true;
                    }
                }
            }
            if !replaced {
                let new_args = [String::from("--extern"), format!("{krate}={rlib}")];
                log::info!("injecting rlib for {krate}, injecting: {new_args:?}");
                compiler_args.extend(new_args);
            }
        }
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

impl rustc_driver::Callbacks for BevyAnalyzer {
    fn after_expansion(
        &mut self,
        compiler: &rustc_interface::interface::Compiler,
        tcx: rustc_middle::ty::TyCtxt<'_>,
    ) -> rustc_driver::Compilation {
        trace!("After expansion callback");

        let sess = &compiler.sess;

        if sess.dcx().has_errors().is_some() {
            sess.dcx().fatal("compilation failed, aborting analysis.");
        }

        let mut meta_dirs = Vec::default();
        let mut templates_dir = None;
        // add all relevant meta dirs to the context
        meta_dirs.push(self.args.output.to_owned());
        if let crate::Command::Generate {
            meta,
            meta_output,
            templates,
            ..
        } = &self.args.cmd
        {
            templates.clone_into(&mut templates_dir);
            if let Some(meta_output) = meta_output {
                meta_dirs.push(meta_output.to_owned())
            };
            meta.iter()
                .flatten()
                .for_each(|m| meta_dirs.push(m.to_owned()));
        };

        let include_private = match &self.args.cmd {
            crate::Command::Generate {
                include_private, ..
            } => *include_private,
            _ => false,
        };

        // tera environment for import processor
        let tera = crate::configure_tera(tcx.crate_name(LOCAL_CRATE).as_str(), &templates_dir);

        debug!("Using meta directories: {meta_dirs:?}");

        let mut graph = WorkspaceGraph::deserialize(&PathBuf::from(
            std::env::var(WORKSPACE_GRAPH_FILE_ENV).unwrap(),
        ))
        .unwrap();
        graph.stable_sort();

        let mut ctxt = crate::BevyCtxt::new(
            tcx,
            &meta_dirs,
            WorkspaceMeta::from_env(),
            include_private,
            Some(Box::new(move |import_path| {
                let mut ctxt = Context::new();
                ctxt.insert("import", import_path);
                tera.render(&TemplateKind::ImportProcessor.to_string(), &ctxt)
                    .unwrap()
            })),
            graph,
        );

        trace!("Running all passes");
        for p in ALL_PASSES {
            debug!(
                "{}, in crate: {}",
                p.description,
                tcx.crate_name(LOCAL_CRATE),
            );
            let continue_ = tcx.sess.time(p.name, || (p.cb)(&mut ctxt, &self.args));
            if !continue_ {
                break;
            }
            trace!("Finished pass, continuing");
        }

        log::info!(
            "Analyzed crate: {} (found {} types)",
            tcx.crate_name(LOCAL_CRATE),
            ctxt.reflect_types.len(),
        );

        rustc_driver::Compilation::Continue
    }

    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        config.file_loader = Some(Box::new(ModifyingFileLoader));
    }
}
