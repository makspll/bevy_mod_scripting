use std::borrow::Cow;

use indexmap::{IndexMap, IndexSet};
use rustdoc_types::{Crate, Impl, Item};
use serde_derive::Serialize;

use crate::NewtypeConfig;

/// Data describing a macro instance to be generated
#[derive(Serialize, Debug, Clone)]
pub struct TypeMeta<'a> {
    pub wrapped_type: &'a String,
    pub path_components: Cow<'a, [String]>,
    pub source: &'a Crate,
    pub config: &'a NewtypeConfig,
    pub item: &'a Item,
    /// The items coming from all trait implementations
    pub impl_items: IndexMap<&'a str, Vec<(&'a Impl, &'a Item)>>,
    pub implemented_traits: IndexSet<String>,
    pub self_impl: Option<&'a Impl>,
    pub crates: &'a [Crate],
    /// If this type has some things which are "static" this is set to true later
    pub has_global_methods: bool,
}
