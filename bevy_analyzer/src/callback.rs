use log::{debug, trace};
use rustc_errors::FatalError;

use crate::FIND_REFLECT_TYPES;

pub struct BevyAnalyzerCallbacks;

impl rustc_driver::Callbacks for BevyAnalyzerCallbacks {
    fn config(&mut self, _config: &mut rustc_interface::interface::Config) {}

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
            let passes = [FIND_REFLECT_TYPES];
            let mut ctxt = crate::BevyCtxt { tcx };
            trace!("Running all passes");
            for p in passes {
                trace!("Running pass: {}", p.name);
                tcx.sess.time(p.name, || (p.cb)(&mut ctxt));
            }
        });
        rustc_driver::Compilation::Continue
    }
}
