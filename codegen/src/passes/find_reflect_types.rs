use log::{debug, info};
use rustc_hir::def_id::LOCAL_CRATE;

use crate::{Args, BevyCtxt, DEF_PATHS_REFLECT, ReflectType};

/// Finds all reflect types which we can wrap in the crate as well as sorts the final list.
pub(crate) fn find_reflect_types(ctxt: &mut BevyCtxt<'_>, args: &Args) -> bool {
    let tcx = &ctxt.tcx;
    let ignored_types = match &args.cmd {
        crate::Command::Generate { ignored_types, .. } => ignored_types,
        _ => return true,
    };

    for trait_did in tcx.all_local_trait_impls(()).keys() {
        // we want to find the canonical `Reflect` trait's implemenations across crates, so let's check all impls and choose those
        // whose def_path is equal to what we know the Reflect trait's is

        let def_path_str = tcx.def_path_str(trait_did);

        if !DEF_PATHS_REFLECT.contains(&def_path_str.as_str()) {
            continue;
        }

        debug!(
            "Found Reflect impls in crate: {}, with path: {}",
            tcx.crate_name(LOCAL_CRATE),
            def_path_str
        );

        // this returns non-local impls as well
        let reflect_trait_impls = tcx.trait_impls_of(trait_did);

        // blanket impls are implementations on generics directly, i.e. `impl From<T> for T`
        // non blanket impls may also contain generics but those will be contained within another type i.e. `impl Default for Vec<T>`
        // ignore anything with a generic, so blanket_impls are out for now
        // we also make sure to only work over types and impls directly in the local crate
        let reflect_adts_did = reflect_trait_impls
            .non_blanket_impls()
            .iter()
            .flat_map(|(self_ty, impl_dids)| impl_dids.iter().zip(std::iter::repeat(self_ty)))
            .filter_map(|(impl_did, self_ty)| {
                let generics = tcx.generics_of(*impl_did);
                (impl_did.is_local() &&
                // only non parametrized simple types are allowed, i.e. "MyStruct" is allowed but "MyStruct<T>" isn't
                    generics.count() == 0 &&
                    self_ty.def().is_some_and(|did| {
                            let short_form = format!("{}::{}",ctxt.tcx.crate_name(LOCAL_CRATE),ctxt.tcx.item_name(did));
                            if ignored_types.contains(&short_form) || ignored_types.contains(&tcx.def_path_str(did)) {                                info!("Ignoring type: {:?}", tcx.def_path_str(did));
                                return false;
                            };
                            let adt_generics = tcx.generics_of(did);
                            tcx.visibility(did).is_public() && adt_generics.count() == 0
                        }))
                .then(|| self_ty.def().unwrap())
            })
            .inspect(|impl_| debug!("On type: {:?}", tcx.item_name(*impl_)))
            .map(|did| (did, ReflectType::default()));

        ctxt.reflect_types.extend(reflect_adts_did);
    }

    ctxt.reflect_types
        .sort_by_cached_key(|did, _| tcx.item_name(*did));

    if args.cmd.is_list_types() {
        for did in ctxt.reflect_types.keys() {
            println!("{:?}", tcx.def_path_str(did));
        }
        return false;
    }

    true
}
