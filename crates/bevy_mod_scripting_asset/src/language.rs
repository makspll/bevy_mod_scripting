//! Defines supported scripting languages and their file extensions.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Represents a scripting language. Languages which compile into another language should use the target language as their language.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum Language {
    /// The Rhai scripting language
    Rhai,
    /// The Lua scripting language
    Lua,
    /// The Rune scripting language
    Rune,
    /// An external scripting language
    External(Cow<'static, str>),
    /// Set if none of the asset path to language mappers match
    #[default]
    Unknown,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rhai => "Rhai".fmt(f),
            Language::Lua => "Lua".fmt(f),
            Language::Rune => "Rune".fmt(f),
            Language::External(cow) => cow.fmt(f),
            Language::Unknown => "Unknown".fmt(f),
        }
    }
}

/// Collect the language extensions supported during initialization.
///
/// NOTE: This resource is removed after plugin setup.
#[derive(Debug)]
pub struct LanguageExtensions(Vec<&'static str>, Vec<Language>);

impl LanguageExtensions {
    /// Create a new language extensions mapping from an iterator of (extension, language) pairs.
    pub fn new(iter: impl IntoIterator<Item = (&'static str, Language)>) -> Self {
        let (extensions, languages): (Vec<&'static str>, Vec<Language>) = iter.into_iter().unzip();
        Self(extensions, languages)
    }

    /// Retrieves the language for the given file extension, if it exists.
    pub fn get(&self, extension: &str) -> Option<&Language> {
        self.0
            .iter()
            .position(|&ext| ext.eq_ignore_ascii_case(extension))
            .and_then(|index| self.1.get(index))
    }

    /// Inserts a new (extension, language) pair into the mapping.
    pub fn insert(&mut self, extension: &'static str, language: Language) {
        if let Some(pos) = self
            .0
            .iter()
            .position(|&ext| ext.eq_ignore_ascii_case(extension))
        {
            self.1[pos] = language;
        } else {
            self.0.push(extension);
            self.1.push(language);
        }
    }

    /// Returns a slice of all supported file extensions.
    pub fn extensions(&self) -> &[&str] {
        self.0.as_slice()
    }
}

impl Default for LanguageExtensions {
    fn default() -> Self {
        LanguageExtensions::new([
            ("lua", Language::Lua),
            ("luau", Language::Lua),
            ("rhai", Language::Rhai),
            ("rn", Language::Rune),
        ])
    }
}
