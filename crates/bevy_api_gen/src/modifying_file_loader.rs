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

                for crate_ in &["bevy_reflect", "mlua"] {
                    if !f.contains(&format!("extern crate {crate_}")) {
                        if f.contains(&format!("pub use {crate_}")) {
                            f.push_str(&format!(
                                "#[allow(unused_extern_crates)] pub extern crate {crate_};"
                            ));
                        } else {
                            // this causes issues in proc macros so let's make it private where we can
                            f.push_str(&format!(
                                "#[allow(unused_extern_crates)] extern crate {crate_};"
                            ));
                        }
                    }
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
