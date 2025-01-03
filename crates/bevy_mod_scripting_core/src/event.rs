use bevy::{ecs::entity::Entity, prelude::Event};

use crate::{error::ScriptError, handler::Args, prelude::ScriptValue, script::ScriptId};

/// An error coming from a script
#[derive(Debug, Event)]
pub struct ScriptErrorEvent {
    pub error: ScriptError,
}

/// A string which disallows common invalid characters in callback labels,
/// particularly at the start of the string
///
/// a valid callback label starts with a letter or underscore, and contains only ascii characters, as well as disallows some common keywords
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CallbackLabel(String);

impl CallbackLabel {
    fn filter_invalid(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut first = true;
        for char in s.chars() {
            if char == '_'
                || ((!first && char.is_ascii_alphanumeric()) || char.is_ascii_alphabetic())
            {
                out.push(char);
                first = false;
            } else {
                continue;
            }
        }
        if FORBIDDEN_KEYWORDS.contains(&s) {
            String::default()
        } else {
            out
        }
    }

    pub fn new_lossy(label: &str) -> Self {
        Self(Self::filter_invalid(label))
    }

    pub fn new(label: &str) -> Option<Self> {
        let new_lossy = Self::new_lossy(label);
        if new_lossy.0.len() != label.len() {
            None
        } else {
            Some(new_lossy)
        }
    }
}

#[macro_export]
macro_rules! callback_labels {
    ($($name:ident => $label:expr),*) => {
        pub enum CallbackLabels {
            $($name),*
        }

        impl IntoCallbackLabel for CallbackLabels {
            fn into_callback_label() -> CallbackLabel {
                match self {
                    $(CallbackLabels::$name => $label.into()),*
                }
            }
        }
    };
}

pub trait IntoCallbackLabel {
    fn into_callback_label() -> CallbackLabel;
}

impl From<&str> for CallbackLabel {
    fn from(s: &str) -> Self {
        Self::new_lossy(s)
    }
}
impl From<String> for CallbackLabel {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl AsRef<str> for CallbackLabel {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CallbackLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

/// Describes the designated recipients of a script event
#[derive(Clone, Debug)]
pub enum Recipients {
    /// The event needs to be handled by all scripts
    All,
    /// The event is to be handled by a specific script
    Script(ScriptId),
    /// The event is to be handled by all the scripts on the specified entity
    Entity(Entity),
}

/// A callback event meant to trigger a callback in a subset/set of scripts in the world with the given arguments
#[derive(Clone, Event, Debug)]
pub struct ScriptCallbackEvent {
    pub label: CallbackLabel,
    pub recipients: Recipients,
    pub args: Vec<ScriptValue>,
}

impl ScriptCallbackEvent {
    pub fn new<L: Into<CallbackLabel>>(
        label: L,
        args: Vec<ScriptValue>,
        recipients: Recipients,
    ) -> Self {
        Self {
            label: label.into(),
            args,
            recipients,
        }
    }

    pub fn new_for_all<L: Into<CallbackLabel>>(label: L, args: Vec<ScriptValue>) -> Self {
        Self::new(label, args, Recipients::All)
    }
}

static FORBIDDEN_KEYWORDS: [&str; 82] = [
    // Lua
    "and",
    "break",
    "do",
    "else",
    "elseif",
    "end",
    "false",
    "for",
    "function",
    "if",
    "in",
    "local",
    "nil",
    "not",
    "or",
    "repeat",
    "return",
    "then",
    "true",
    "until",
    "while",
    //  Rhai
    "true",
    "false",
    "let",
    "const",
    "is_shared",
    "if",
    "else",
    "switch",
    "do",
    "while",
    "loop",
    "until",
    "for",
    "in",
    "continue",
    "break",
    "fn",
    "private",
    "is_def_fn",
    "this",
    "return",
    "throw",
    "try",
    "catch",
    "import",
    "export",
    "as",
    "global",
    "Fn",
    "call",
    "curry",
    "type_of",
    "print",
    "debug",
    "eval",
    "is_def_var",
    "var",
    "static",
    "is",
    "goto",
    "match",
    "case",
    "public",
    "protected",
    "new",
    "use",
    "with",
    "module",
    "package",
    "super",
    "spawn",
    "thread",
    "go",
    "sync",
    "async",
    "await",
    "yield",
    "default",
    "void",
    "null",
    "nil",
];

#[cfg(test)]
mod test {
    use super::FORBIDDEN_KEYWORDS;

    #[test]
    fn test_invalid_strings() {
        FORBIDDEN_KEYWORDS.iter().for_each(|keyword| {
            assert_eq!(super::CallbackLabel::new(keyword), None);
        });
    }

    #[test]
    fn test_bad_chars() {
        let bad_chars = [
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '{', '}', '[', ']',
            '|', '\\', ':', ';', '"', '\'', '<', '>', ',', '.', '?', '/', '`', '~',
        ];
        bad_chars.iter().for_each(|char| {
            assert_eq!(super::CallbackLabel::new(&format!("bad{}", char)), None);
        });
    }

    #[test]
    fn bad_first_letter() {
        let bad_chars = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '@', '#', '$', '%', '^', '&', '*',
            '(', ')', '-', '+', '=', '{', '}', '[', ']', '|', '\\', ':', ';', '"', '\'', '<', '>',
            ',', '.', '?', '/', '`', '~',
        ];
        bad_chars.iter().for_each(|char| {
            assert_eq!(super::CallbackLabel::new(&format!("{}bad", char)), None);
        });
    }

    #[test]
    fn test_valid_idents() {
        let valid = ["h", "_v", "hello", "_2d", "heloo_2", "_1231412"];
        valid.iter().for_each(|ident| {
            assert!(super::CallbackLabel::new(ident).is_some());
            assert_eq!(super::CallbackLabel::new_lossy(ident).as_ref(), *ident);
        });
    }
}
