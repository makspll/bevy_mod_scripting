use rustc_ast::Mutability;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{
    self, AliasTy, GenericArg, GenericArgKind, GenericParamDef, GenericParamDefKind, Ty, TyCtxt,
    TyKind,
};

use crate::BmsTypes;
pub fn wrap_non_primtiives<'tcx>(
    ty: Ty<'tcx>,
    tcx: TyCtxt<'tcx>,
    wrapper_ids: &BmsTypes,
) -> Ty<'tcx> {
    wrap_type_signature(tcx, ty, wrapper_ids)
}

pub fn wrap_type_signature<'tcx>(
    tcx: TyCtxt<'tcx>,
    ty: Ty<'tcx>,
    bms_types: &BmsTypes,
) -> Ty<'tcx> {
    fn go<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>, bms_types: &BmsTypes) -> Ty<'tcx> {
        match ty.kind() {
            // ----- ADTs -----
            TyKind::Adt(adt_def, args) => {
                // if w.primitive_adts.contains(&adt_def.did()) {
                //     return ty;
                // }
                if ty.type_in_bms_wrapper(bms_types).is_some() {
                    return ty;
                }

                // generics preserved
                if !args.is_empty() {
                    let new_args =
                        tcx.mk_args_from_iter(args.iter().map(|a| go_generic(tcx, a, bms_types)));
                    return tcx.mk_ty_from_kind(TyKind::Adt(*adt_def, new_args));
                }

                // Non-generic ADT → wrap in Val<T>
                let new_args =
                    tcx.mk_args_from_iter(args.iter().map(|a| go_generic(tcx, a, bms_types)));
                let inner = tcx.mk_ty_from_kind(TyKind::Adt(*adt_def, new_args));

                wrap_val(tcx, inner, bms_types)
            }

            // ----- Alias types -----
            TyKind::Alias(kind, alias_ty) => {
                let new_args = tcx
                    .mk_args_from_iter(alias_ty.args.iter().map(|a| go_generic(tcx, a, bms_types)));
                let new_alias = AliasTy::new_from_args(tcx, alias_ty.def_id, new_args);

                // Do NOT wrap an alias — keep structure.
                tcx.mk_ty_from_kind(TyKind::Alias(*kind, new_alias))
            }

            // ----- References (&T, &mut T) -----
            TyKind::Ref(_region, inner, mutbl) => {
                let inner_wrapped = go(tcx, *inner, bms_types);

                match mutbl {
                    Mutability::Not => {
                        if matches!(inner_wrapped.kind(),
                            TyKind::Adt(adt, _) if adt.did() == bms_types.ref_wrapper
                        ) {
                            inner_wrapped
                        } else {
                            wrap_ref(tcx, inner_wrapped, bms_types)
                        }
                    }
                    Mutability::Mut => {
                        if matches!(inner_wrapped.kind(),
                            TyKind::Adt(adt, _) if adt.did() == bms_types.mut_wrapper
                        ) {
                            inner_wrapped
                        } else {
                            wrap_mut(tcx, inner_wrapped, bms_types)
                        }
                    }
                }
            }

            // ----- Arrays / slices -----
            TyKind::Array(elem, len) => {
                let new_elem = go(tcx, *elem, bms_types);
                tcx.mk_ty_from_kind(TyKind::Array(new_elem, *len))
            }

            TyKind::Slice(elem) => {
                let new_elem = go(tcx, *elem, bms_types);
                tcx.mk_ty_from_kind(TyKind::Slice(new_elem))
            }

            // ----- Everything else left unchanged -----
            _ => ty,
        }
    }

    fn go_generic<'tcx>(
        tcx: TyCtxt<'tcx>,
        arg: GenericArg<'tcx>,
        bms_types: &BmsTypes,
    ) -> GenericArg<'tcx> {
        match arg.kind() {
            GenericArgKind::Type(t) => go(tcx, t, bms_types).into(),
            _ => arg,
        }
    }

    /// Build GenericArgs for a wrapper ADT so the arg kinds match the wrapper's generics.
    /// The `inner` type will be placed into the first Type generic parameter encountered.
    /// Lifetime parameters will receive an erased region.
    fn make_wrapper_args<'tcx>(
        tcx: TyCtxt<'tcx>,
        wrapper_def_id: DefId,
        inner: Ty<'tcx>,
    ) -> ty::GenericArgsRef<'tcx> {
        let generics = tcx.generics_of(wrapper_def_id);

        // iterator producing GenericArg<'tcx>
        let args_iter = generics.own_params.iter().map(|param: &GenericParamDef| {
            match param.kind {
                GenericParamDefKind::Lifetime => {
                    // produce erased region GenericArg

                    let erased_region = tcx.lifetimes.re_erased; // adjust if your API differs
                    GenericArg::from(erased_region)
                }

                GenericParamDefKind::Type { .. } => GenericArg::from(inner),

                GenericParamDefKind::Const { .. } => {
                    // Most wrapper ADTs won't have const params; if they do, you must create a const GenericArg.
                    // For now, abort so the mismatch is explicit.
                    panic!(
                        "wrapper ADT has const generic param; not supported by make_wrapper_args"
                    );
                }
            }
        });

        tcx.mk_args_from_iter(args_iter)
    }

    fn wrap_val<'tcx>(tcx: TyCtxt<'tcx>, inner: Ty<'tcx>, bms_types: &BmsTypes) -> Ty<'tcx> {
        // guard double-wrap
        if inner.type_in_bms_wrapper(bms_types).is_some() {
            return inner;
        }

        let adt = tcx.adt_def(bms_types.val_wrapper);
        let args = make_wrapper_args(tcx, adt.did(), inner);
        tcx.mk_ty_from_kind(TyKind::Adt(adt, args))
    }

    fn wrap_ref<'tcx>(tcx: TyCtxt<'tcx>, inner: Ty<'tcx>, bms_types: &BmsTypes) -> Ty<'tcx> {
        let adt = tcx.adt_def(bms_types.ref_wrapper);
        let args = make_wrapper_args(tcx, adt.did(), inner);
        tcx.mk_ty_from_kind(TyKind::Adt(adt, args))
    }

    fn wrap_mut<'tcx>(tcx: TyCtxt<'tcx>, inner: Ty<'tcx>, bms_types: &BmsTypes) -> Ty<'tcx> {
        let adt = tcx.adt_def(bms_types.mut_wrapper);
        let args = make_wrapper_args(tcx, adt.did(), inner);
        tcx.mk_ty_from_kind(TyKind::Adt(adt, args))
    }

    go(tcx, ty, bms_types)
}

pub(crate) trait UnwrapWrapper<'tcx> {
    fn type_in_bms_wrapper(&self, ctxt: &BmsTypes) -> Option<Ty<'tcx>>;
}

impl<'tcx> UnwrapWrapper<'tcx> for Ty<'tcx> {
    fn type_in_bms_wrapper(&self, ctxt: &BmsTypes) -> Option<Ty<'tcx>> {
        if let TyKind::Adt(ty, args) = self.kind()
            && [ctxt.mut_wrapper, ctxt.ref_wrapper, ctxt.val_wrapper].contains(&ty.did())
            && let Some(inner) = args.iter().find_map(|a| a.as_type())
        {
            return Some(inner);
        }
        None
    }
}
