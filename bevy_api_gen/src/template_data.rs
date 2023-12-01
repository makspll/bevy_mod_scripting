use std::{error::Error, fmt::Display};

use clap::Arg;
use indexmap::{IndexMap, IndexSet};
use rustdoc_types::{Crate, Item, ItemEnum, Type};
use sailfish::TemplateOnce;
use serde_derive::Serialize;

use crate::{Config, ItemData, TypeMeta, ValidType};

#[derive(TemplateOnce)]
#[template(path = "main.stpl", escape = false)]
pub struct TemplateData {
    pub items: IndexMap<String, ItemData>,
    pub primitives: IndexSet<String>,
}
