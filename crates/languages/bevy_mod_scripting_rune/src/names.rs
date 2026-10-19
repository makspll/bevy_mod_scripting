//! Collects the names scripts can use after a `.`, which Rune needs registered up front, see [`crate::types`].
use std::collections::BTreeSet;

use crate::types::intern;

/// Collects every identifier in the text of a script, which includes every name it can access with the `.` syntax.
///
/// Identifiers in comments and strings are collected too, registering a name no script uses is harmless.
/// Whether a member exists is checked when it is accessed.
pub fn source_names(text: &str) -> BTreeSet<&'static str> {
    text.split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|word| word.starts_with(|c: char| c.is_alphabetic() || c == '_'))
        .map(intern)
        .collect()
}
