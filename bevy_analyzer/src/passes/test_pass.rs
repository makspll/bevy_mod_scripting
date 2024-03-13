use log::debug;

use crate::BevyCtxt;

pub fn test_pass(ctxt: &mut BevyCtxt<'_>) {
    let tcx = &ctxt.tcx;
    for c in tcx.crates(()) {
        debug!("Crate: {c:?}, {}", tcx.crate_name(*c));
        for t in tcx.traits(*c) {
            let n = tcx.item_name(*t);
            debug!("Trait: {:?}", n);
            let impls = tcx.trait_impls_of(*t);

            // for blanket in impls.blanket_impls() {
            //     if blanket.krate == t.krate {
            //         continue;
            //     }
            //     debug!("Blanket impl in : {}", tcx.crate_name(blanket.krate))
            // }
            // for (ty, v) in impls.non_blanket_impls() {
            //     for v in v {
            //         if v.krate == t.krate {
            //             continue;
            //         }
            //         debug!("Impl in : {}", tcx.crate_name(v.krate));
            //     }
            // }
        }
    }
}
