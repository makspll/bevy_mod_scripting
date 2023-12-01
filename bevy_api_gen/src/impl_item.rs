use rustdoc_types::{Impl, Item};
use serde_derive::Serialize;

use crate::{ImportPath, ValidType};

/// Represents an item coming from an impl, either from a trait or from the pure impl blocks
#[derive(Debug, Clone, Serialize)]
pub struct ImplItem<'a> {
    pub impl_: &'a Impl,
    pub item: &'a Item,
    pub trait_import_path: Option<ImportPath>,
    /// If true the impl item is implemented on another primitive type, and self types must be adjusted
    pub foreign: bool,
}
