use std::{cell::RefCell, collections::HashMap, fs::File, io::BufReader};

use cargo_metadata::camino::Utf8PathBuf;
use log::debug;
use rustc_hir::def_id::DefPathHash;
use serde::{Deserialize, Serialize};

/// Similar to .rmeta files but for the code generator, each crate is analysed separately but we need to share some information
/// between crates to be able to properly identify links between crates
#[derive(Serialize, Deserialize, Clone)]
pub struct Meta {
    /// The local proxies generated after analysis
    pub(crate) proxies: Vec<ProxyMeta>,
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
    cache: RefCell<HashMap<String, Meta>>,
}

impl MetaLoader {
    pub fn new(meta_dirs: Vec<Utf8PathBuf>) -> Self {
        Self {
            meta_dirs,
            cache: Default::default(),
        }
    }

    /// Retrieves the meta for the provided crate, returns 'Some(meta)' if it exists and 'None' otherwise
    pub fn meta_for(&self, crate_name: &str) -> Option<Meta> {
        self.meta_dirs
            .iter()
            .find_map(|dir| self.meta_for_in_dir(crate_name, dir))
    }

    fn meta_for_in_dir(&self, crate_name: &str, dir: &Utf8PathBuf) -> Option<Meta> {
        let cache = self.cache.borrow();
        if cache.contains_key(crate_name) {
            debug!("Loading meta from cache for: {}", crate_name);
            return cache.get(crate_name).cloned();
        } else {
            debug!("Loading meta from filesystem for: {}", crate_name);
            drop(cache);
            let mut cache = self.cache.borrow_mut();
            let meta = Self::opt_load_meta(dir.join(format!(".{crate_name}.json")))?;
            cache.insert(crate_name.to_owned(), meta.clone());
            Some(meta)
        }
    }

    fn opt_load_meta(path: Utf8PathBuf) -> Option<Meta> {
        if !path.exists() {
            debug!("Meta not found at: {}", path);
            return None;
        }
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).unwrap()
    }
}
