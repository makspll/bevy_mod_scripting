use rustc_middle::ty::TyCtxt;

pub struct BevyCtxt<'tcx> {
    pub tcx: TyCtxt<'tcx>,
}
