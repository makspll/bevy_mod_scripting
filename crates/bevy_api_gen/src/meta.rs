use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter, Write},
};

use cargo_metadata::camino::Utf8PathBuf;
use chrono::NaiveDateTime;
use log::trace;
use rustc_hir::def_id::DefPathHash;
use serde::{Deserialize, Serialize, Serializer};

use crate::WorkspaceMeta;

/// The version of the meta file format, we can use this to show errors if incompatibile formats are used
/// or to convert older formats to newer ones automatically
pub(crate) const META_VERSION: &str = "1";

/// Similar to .rmeta files but for the code generator, each crate is analysed separately but we need to share some information
/// between crates to be able to properly identify links between crates
#[derive(Serialize, Deserialize, Clone)]
pub struct Meta {
    pub(crate) crate_name: String,
    /// The local proxies generated after analysis
    pub(crate) proxies: Vec<ProxyMeta>,
    /// False if no files are going to be generated for this crate
    pub(crate) will_generate: bool,
    pub(crate) meta_version: String,
    #[serde(
        serialize_with = "serialize_timestamp",
        deserialize_with = "deserialize_timestamp"
    )]
    pub(crate) timestamp: NaiveDateTime,
}

fn serialize_timestamp<S: Serializer>(
    timestamp: &NaiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    // format as date and time
    serializer.serialize_str(&timestamp.format("%Y-%m-%d %H:%M:%S").to_string())
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
}

impl Meta {
    /// Returns true if the crate generated a proxy with the given DefPathHash (for the ADT)
    pub fn contains_def_path_hash(&self, did: DefPathHash) -> bool {
        self.proxies.iter().any(|meta| {
            meta.stable_crate_id == did.stable_crate_id().as_u64()
                && meta.local_hash_id == did.local_hash().as_u64()
        })
    }

    pub fn will_generate(&self) -> bool {
        self.will_generate
    }

    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ProxyMeta {
    pub(crate) ident: String,
    pub(crate) stable_crate_id: u64,
    pub(crate) local_hash_id: u64,
}

/// Manages deserialisation and retrieval of meta files
pub struct MetaLoader {
    pub meta_dirs: Vec<Utf8PathBuf>,
    pub workspace_meta: WorkspaceMeta,
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

    /// iterates over the meta files in the meta directories and returns an iterator over the meta files
    pub fn iter_meta(&self) -> impl Iterator<Item = Meta> + '_ {
        self.meta_dirs.iter().flat_map(|dir| {
            dir.read_dir()
                .unwrap_or_else(|_| panic!("Could not read meta directory: {}", dir))
                .filter_map(|entry| {
                    let entry = entry.unwrap();
                    if entry.path().extension().is_some_and(|ext| ext == "json") {
                        return Self::opt_load_meta(
                            Utf8PathBuf::from_path_buf(entry.path())
                                .expect("Invalid meta file in meta directory"),
                        );
                    }
                    None
                })
        })
    }

    /// Retrieves the meta for the provided crate, returns 'Some(meta)' if it exists and 'None' otherwise
    pub fn meta_for(&self, crate_name: &str) -> Option<Meta> {
        self.meta_for_retry(crate_name, 3)
    }

    /// Searches the given meta sources in order for the provided DefPathHash, once a meta file containing this hash is found
    /// the search stops and returns true, if no meta file is found containing the hash, false is returned
    ///
    /// if a curr_source argument is provided, the search will skip this source as it is assumed that the current crate is still being compiled and not meta file for it exists yet
    pub fn one_of_meta_files_contains(
        &self,
        meta_sources: &[&str],
        curr_source: Option<&str>,
        target_def_path_hash: DefPathHash,
    ) -> bool {
        let meta = match meta_sources
            .iter()
            .filter(|s| curr_source.is_none() || curr_source.is_some_and(|cs| cs != **s))
            .find_map(|s| self.meta_for(s))
        {
            Some(meta) => meta,
            None => return false, // TODO: is it possible we get false negatives here ? perhaps due to parallel compilation ? or possibly because of dependency order
        };

        meta.contains_def_path_hash(target_def_path_hash)
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
                "Could not find meta file for crate: `{}`, is_workspace_and_included: '{}'",
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
            cache.get(crate_name).cloned()
        } else {
            drop(cache);
            let mut cache = self.cache.borrow_mut();
            let dir = dir.join(Self::crate_name_to_meta_filename(crate_name));
            trace!(
                "Attempting to load meta from filesystem for crate: {}, at: {}",
                crate_name,
                dir
            );
            let meta = Self::opt_load_meta(dir)?;
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
        serde_json::to_writer_pretty(&mut writer, meta).unwrap();
        writer.flush().expect("Could not flush data to meta file");
    }

    fn crate_name_to_meta_filename(crate_name: &str) -> String {
        format!("{}.json", crate_name)
    }
}
