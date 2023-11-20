use std::{error::Error, fmt::Display};

use indexmap::IndexMap;
use rustdoc_types::Type;
use sailfish::TemplateOnce;

use crate::{ArgType, Config, ItemData};

/// Struct representing an argument to a function, or a field declaration, Anything with a name and a type reall
#[derive(Debug)]
pub struct NameType {
    pub name: String,
    pub type_: ArgType,
    pub is_proxied: bool,
}

impl TryFrom<((String, Type), &Config)> for NameType {
    type Error = Box<dyn Error>;

    fn try_from(((name, type_), config): ((String, Type), &Config)) -> Result<Self, Self::Error> {
        let type_: ArgType = (name == "self", &type_).try_into()?;

        let is_primitive = type_
            .base_ident()
            .is_some_and(|ident| config.primitives.contains(ident));

        let is_proxied = type_.is_contextual()
            || (!is_primitive
                && type_
                    .base_ident()
                    .is_some_and(|ident| config.types.contains_key(ident)));
        if !is_primitive && !is_proxied {
            return Err(format!(
                "Type is neither a wrapped type in the config or an allowed primitive: `{type_:?}`"
            )
            .into());
        }

        Ok(Self {
            name,
            type_,
            is_proxied,
        })
    }
}

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
