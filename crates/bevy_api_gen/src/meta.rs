use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter, Write},
};

use cargo_metadata::camino::Utf8PathBuf;
use log::trace;
use rustc_hir::def_id::DefPathHash;
use serde::{Deserialize, Serialize};

use crate::WorkspaceMeta;

/// Similar to .rmeta files but for the code generator, each crate is analysed separately but we need to share some information
/// between crates to be able to properly identify links between crates
#[derive(Serialize, Deserialize, Clone)]
pub struct Meta {
    /// The local proxies generated after analysis
    pub(crate) proxies: Vec<ProxyMeta>,
    /// False if no files are going to be generated for this crate
    pub(crate) will_generate: bool,
}

impl Meta {
    /// Returns true if the crate generated a proxy with the given DefPathHash (for the ADT)
    pub(crate) fn contains_def_path_hash(&self, did: DefPathHash) -> bool {
        self.proxies.iter().any(|meta| {
            meta.stable_crate_id == did.stable_crate_id().as_u64()
                && meta.local_hash_id == did.local_hash().as_u64()
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct ProxyMeta {
    pub(crate) ident: String,
    pub(crate) stable_crate_id: u64,
    pub(crate) local_hash_id: u64,
}

/// Manages deserialisation and retrieval of meta files
pub struct MetaLoader {
    pub(crate) meta_dirs: Vec<Utf8PathBuf>,
    pub(crate) workspace_meta: WorkspaceMeta,
    cache: RefCell<HashMap<String, Meta>>,
}

impl MetaLoader {
    /// First meta dir is used as output
    pub fn new(meta_dirs: Vec<Utf8PathBuf>, workspace_meta: WorkspaceMeta) -> Self {
        Self {
            meta_dirs,
            cache: Default::default(),
            workspace_meta,
        }
    }

    /// Retrieves the meta for the provided crate, returns 'Some(meta)' if it exists and 'None' otherwise
    pub fn meta_for(&self, crate_name: &str) -> Option<Meta> {
        self.meta_for_retry(crate_name, 3)
    }

    fn meta_for_retry(&self, crate_name: &str, _try_attempts: usize) -> Option<Meta> {
        let meta = self
            .meta_dirs
            .iter()
            .find_map(|dir| self.meta_for_in_dir(crate_name, dir));

        let needs_meta = self
            .workspace_meta
            .is_workspace_and_included_crate(crate_name);

        if meta.is_none() {
            log::trace!(
                "Could not find meta for crate: `{}`, is_workspace_and_included: '{}'",
                crate_name,
                needs_meta
            )
        }
        if meta.is_none() && needs_meta {
            panic!("Could not find meta for workspace crate: {}", crate_name);
        };

        meta
    }

    fn meta_for_in_dir(&self, crate_name: &str, dir: &Utf8PathBuf) -> Option<Meta> {
        let cache = self.cache.borrow();
        if cache.contains_key(crate_name) {
            trace!("Loading meta from cache for: {}", crate_name);
            return cache.get(crate_name).cloned();
        } else {
            trace!("Loading meta from filesystem for: {}", crate_name);
            drop(cache);
            let mut cache = self.cache.borrow_mut();
            let meta =
                Self::opt_load_meta(dir.join(Self::crate_name_to_meta_filename(crate_name)))?;
            cache.insert(crate_name.to_owned(), meta.clone());
            Some(meta)
        }
    }

    fn opt_load_meta(path: Utf8PathBuf) -> Option<Meta> {
        if !path.exists() {
            trace!("Meta not found at: {}", path);
            return None;
        }
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).unwrap()
    }

    pub fn write_meta(&self, crate_name: &str, meta: &Meta) {
        let path = self
            .meta_dirs
            .first()
            .expect("No meta directory provided for output")
            .join(Self::crate_name_to_meta_filename(crate_name));

        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let file = File::create(&path).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, meta).unwrap();
        writer.flush().expect("Could not flush data to meta file");
    }

    fn crate_name_to_meta_filename(crate_name: &str) -> String {
        format!("{}.json", crate_name)
    }
}
