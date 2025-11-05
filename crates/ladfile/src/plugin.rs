use std::{any::Any, error::Error, path::Path};

use crate::LadFile;

/// A trait implemented by ladfile post-processors.
pub trait LadFilePlugin: Any {
    /// A user friendly name of the pkugin
    fn name(&self) -> &'static str;
    /// Apply the ladfile plugin, given the specified output directory
    fn run(&self, ladfile: &LadFile, output_dir: &Path) -> Result<(), Box<dyn Error>>;
}
