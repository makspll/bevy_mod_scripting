use std::{
    fs::{self, File},
    io::Write,
};

use rustc_hir::def_id::LOCAL_CRATE;

use crate::{Args, BevyCtxt, Meta, ProxyMeta};

/// Finds and caches relevant mlua traits, if they cannot be found throws an ICE
pub(crate) fn write_meta(ctxt: &mut BevyCtxt<'_>, args: &Args) -> bool {
    let output = match &args.cmd {
        crate::Command::Generate {
            output,
            meta_output,
            ..
        } => meta_output.as_ref().unwrap_or(output),
        _ => return true,
    };

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
    let meta = Meta { proxies };
    let meta_json = serde_json::to_string(&meta).expect("Meta serialization failed");
    let crate_name = tcx.crate_name(LOCAL_CRATE);

    fs::create_dir_all(output).unwrap();

    let mut file =
        File::create(output.join(format!(".{crate_name}")).with_extension("json")).unwrap();

    file.write_all(meta_json.as_bytes()).unwrap();

    true
}
