use log::{debug, trace};
use rustc_errors::FatalError;
use rustc_hir::def_id::LOCAL_CRATE;

use crate::ALL_PASSES;

pub struct BevyAnalyzerCallbacks;

impl rustc_driver::Callbacks for BevyAnalyzerCallbacks {
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {}

    fn after_expansion<'tcx>(
        &mut self,
        compiler: &rustc_interface::interface::Compiler,
        queries: &'tcx rustc_interface::Queries<'tcx>,
    ) -> rustc_driver::Compilation {
        trace!("After expansion callback");
        let Ok(mut gcx) = queries.global_ctxt() else {
            FatalError.raise()
        };
        let sess = &compiler.sess;

        if sess.dcx().has_errors().is_some() {
            sess.dcx().fatal("compilation failed, aborting analysis.");
        }
        gcx.enter(|tcx| {
            let mut ctxt = crate::BevyCtxt::new(tcx);
            trace!("Running all passes");
            for p in ALL_PASSES {
                debug!(
                    "Running pass: '{}' on crate: '{}'",
                    p.name,
                    tcx.crate_name(LOCAL_CRATE)
                );
                tcx.sess.time(p.name, || (p.cb)(&mut ctxt));
            }
        });
        rustc_driver::Compilation::Continue
    }
}
