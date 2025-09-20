//! Scripting asset definitions

use bevy_asset::Asset;
use bevy_reflect::Reflect;

use crate::Language;

/// Represents a script loaded into memory as an asset
#[derive(Asset, Clone, Reflect, Default)]
#[reflect(opaque)]
pub struct ScriptAsset {
    /// The body of the script
    pub content: Box<[u8]>, // Any chance a Cow<'static, ?> could work here?
    /// The language of the script
    pub language: Language,
}

impl From<String> for ScriptAsset {
    fn from(s: String) -> ScriptAsset {
        ScriptAsset {
            content: s.into_bytes().into_boxed_slice(),
            language: Language::default(),
        }
    }
}

impl ScriptAsset {
    /// Create a new script asset with an unknown language.
    pub fn new(s: impl Into<String>) -> Self {
        s.into().into()
    }
}
