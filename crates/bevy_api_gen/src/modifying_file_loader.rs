use std::io;

use log::trace;
use rustc_data_structures::sync::{AtomicBool, Lrc, Ordering};
use rustc_span::source_map::{FileLoader, RealFileLoader};

/// Injects extern statements into the first loaded file (crate root)
#[derive(Default)]
pub(crate) struct ModifyingFileLoader;
static LOADED: AtomicBool = AtomicBool::new(false);

impl FileLoader for ModifyingFileLoader {
    fn file_exists(&self, path: &std::path::Path) -> bool {
        RealFileLoader.file_exists(path)
    }

    fn read_file(&self, path: &std::path::Path) -> std::io::Result<String> {
        if !LOADED.fetch_or(true, Ordering::SeqCst) {
            trace!(
                "Injecting in-memory extern statements into: {}",
                path.to_str().unwrap()
            );
            RealFileLoader.read_file(path).map(|mut f| {
                // we make it pub so in case we are re-exporting this crate we won't run into private re-export issues
                if !f.contains("pub extern crate mlua") {
                    f.push_str("#[allow(unused_extern_crates)] extern crate mlua;");
                }
                if !f.contains("pub extern crate bevy_reflect") {
                    f.push_str("#[allow(unused_extern_crates)] extern crate bevy_reflect;");
                }
                f
            })
        } else {
            RealFileLoader.read_file(path)
        }
    }

    fn read_binary_file(&self, path: &std::path::Path) -> io::Result<Lrc<[u8]>> {
        RealFileLoader.read_binary_file(path)
    }
}
