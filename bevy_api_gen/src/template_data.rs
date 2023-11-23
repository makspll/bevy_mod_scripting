use std::{error::Error, fmt::Display};

use clap::Arg;
use indexmap::IndexMap;
use rustdoc_types::{Crate, Item, ItemEnum, Type};
use sailfish::TemplateOnce;

use crate::{Config, ItemData, TypeMeta, ValidType};

#[derive(Clone)]
pub struct ImportPath {
    pub components: Vec<String>,
}

impl From<Vec<String>> for ImportPath {
    fn from(value: Vec<String>) -> Self {
        Self { components: value }
    }
}

impl Display for ImportPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.components.join("::").as_str())
    }
}

impl ImportPath {
    pub fn replace_prefix(&self, prefix: &str, replacement: &str) -> Self {
        let mut components = self.components.clone();
        if let Some(first) = components.first_mut() {
            if let Some(stripped) = first.strip_prefix(prefix) {
                *first = replacement.to_owned() + stripped;
            }
        }
        Self { components }
    }
}

#[derive(TemplateOnce)]
#[template(path = "main.stpl", escape = false)]
pub struct TemplateData {
    pub items: IndexMap<String, ItemData>,
}
