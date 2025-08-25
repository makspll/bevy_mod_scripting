use crate_feature_graph::CrateName;
use rustc_hir::def_id::LOCAL_CRATE;

use crate::{Args, BevyCtxt, Dependency, META_VERSION, Meta, ProxyMeta};

/// Finds and caches relevant traits, if they cannot be found throws an ICE
pub(crate) fn write_meta(ctxt: &mut BevyCtxt<'_>, _args: &Args) -> bool {
    let tcx = &ctxt.tcx;

    let mut proxies = Vec::with_capacity(ctxt.reflect_types.len());
    for proxy in ctxt.reflect_types.keys() {
        let def_path_hash = tcx.def_path_hash(*proxy);
        proxies.push(ProxyMeta {
            ident: tcx.item_name(*proxy).to_ident_string(),
            stable_crate_id: def_path_hash.stable_crate_id().as_u64(),
            local_hash_id: def_path_hash.local_hash().as_u64(),
        });
    }
    let will_generate = !proxies.is_empty();
    let crate_name = tcx.crate_name(LOCAL_CRATE).as_str().to_string();
    let matching_crate = ctxt
        .workspace
        .workspace
        .find_crate_opt(&CrateName::new(crate_name.clone()))
        .unwrap();
    let features = matching_crate
        .active_features
        .as_ref()
        .cloned()
        .unwrap_or_default();
    let version = matching_crate.version.clone();

    let dependencies = matching_crate
        .active_dependency_features
        .as_ref()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|(k, features)| {
            match matching_crate
                .dependencies
                .iter()
                .find(|d| d.name == k)
                .or_else(|| {
                    matching_crate
                        .optional_dependencies
                        .iter()
                        .find(|d| d.name == k)
                }) {
                Some(d) => (
                    k.to_string(),
                    Dependency {
                        version: d.version.to_string(),
                        features: features.iter().map(|f| f.to_string()).collect(),
                    },
                ),
                None => todo!(),
            }
        });

    let meta = Meta {
        crate_name,
        proxies,
        will_generate,
        meta_version: META_VERSION.to_string(),
        timestamp: chrono::Local::now().naive_local(),
        features: features.iter().map(|f| f.to_string()).collect(),
        version: version.to_string(),
        dependencies: dependencies.collect(),
    };

    ctxt.meta_loader
        .write_meta(ctxt.tcx.crate_name(LOCAL_CRATE).as_str(), &meta);

    if !will_generate {
        return false;
    }

    true
}
