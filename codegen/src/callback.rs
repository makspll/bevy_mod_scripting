use std::path::PathBuf;

use crate_feature_graph::WorkspaceGraph;
use log::{info, trace};
use rustc_hir::def_id::LOCAL_CRATE;
use tera::Context;

use crate::{
    ALL_PASSES, Args, TemplateKind, WorkspaceMeta, driver::WORKSPACE_GRAPH_FILE_ENV,
    modifying_file_loader::ModifyingFileLoader,
};

pub(crate) struct BevyAnalyzerCallbacks {
    args: Args,
}

impl BevyAnalyzerCallbacks {
    pub(crate) fn new(args: Args) -> Self {
        Self { args }
    }
}

impl rustc_driver::Callbacks for BevyAnalyzerCallbacks {
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
        if let crate::Command::Generate {
            output,
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
            meta_dirs.push(output.to_owned());
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

        info!("Using meta directories: {meta_dirs:?}");
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
            WorkspaceGraph::deserialize(&PathBuf::from(
                std::env::var(WORKSPACE_GRAPH_FILE_ENV).unwrap(),
            ))
            .unwrap(),
        );

        trace!("Running all passes");
        for p in ALL_PASSES {
            info!(
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

        rustc_driver::Compilation::Continue
    }

    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        config.file_loader = Some(Box::new(ModifyingFileLoader));
    }
}
