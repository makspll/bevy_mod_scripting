//! Event handlers and event types for scripting.

use crate::{
    bindings::script_value::ScriptValue,
    error::ScriptError,
    script::{ContextKey, Domain, ScriptId},
};
use bevy::{ecs::entity::Entity, prelude::Event, reflect::Reflect};

/// An error coming from a script
#[derive(Debug, Event)]
pub struct ScriptErrorEvent {
    /// The script that caused the error
    pub error: ScriptError,
}

/// A string which disallows common invalid characters in callback labels,
/// particularly at the start of the string
///
/// a valid callback label starts with a letter or underscore, and contains only ascii characters, as well as disallows some common keywords
#[derive(Reflect, Clone, PartialEq, Eq, Hash, Debug)]
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

    /// Creates a new callback label, filtering out invalid characters
    pub fn new_lossy(label: &str) -> Self {
        Self(Self::filter_invalid(label))
    }

    /// Creates a new callback label, returning None if the label is invalid
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
/// Creates a set of callback labels
macro_rules! callback_labels {
    ($($(#[doc = $doc:expr])* $name:ident => $label:expr),* $(,)?) => {

        $(
            $(#[doc = $doc])*
            #[doc = "A callback label for the event: "]
            #[doc = stringify!($label)]
            pub struct $name;
            impl $crate::event::IntoCallbackLabel for $name {
                fn into_callback_label() -> $crate::event::CallbackLabel {
                    $label.into()
                }
            }
        )*
    };
}

callback_labels!(
    /// Fired when a script is successfully loaded
    OnScriptLoaded => "on_script_loaded",
    /// Fired when a script is unloaded before a reload, if a value is returned, it will be passed to the `on_script_reloaded` callback
    OnScriptUnloaded => "on_script_unloaded",
    /// Fired when a script is reloaded (loaded after being unloaded)
    /// This callback receives the value returned by the `on_script_unloaded` callback if any were returned
    OnScriptReloaded => "on_script_reloaded",
);

/// A trait for types that can be converted into a callback label
pub trait IntoCallbackLabel {
    /// Converts the type into a callback label
    fn into_callback_label() -> CallbackLabel;
}

impl<T: IntoCallbackLabel> From<T> for CallbackLabel {
    fn from(_: T) -> Self {
        T::into_callback_label()
    }
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
    /// The event is to be handled by a specific domain
    Domain(Domain),
    /// The event is to be handled by all the scripts of one language
    Language(crate::asset::Language),
}

/// A callback event meant to trigger a callback in a subset/set of scripts in the world with the given arguments
#[derive(Clone, Event, Debug)]
#[non_exhaustive]
pub struct ScriptCallbackEvent {
    /// The label of the callback
    pub label: CallbackLabel,
    /// The recipients of the callback
    pub recipients: Recipients,
    /// The arguments to the callback
    pub args: Vec<ScriptValue>,
    /// Whether the callback should emit a response event
    pub trigger_response: bool,
}

impl ScriptCallbackEvent {
    /// Creates a new callback event with the given label, arguments and recipients
    pub fn new<L: Into<CallbackLabel>>(
        label: L,
        args: Vec<ScriptValue>,
        recipients: Recipients,
    ) -> Self {
        Self {
            label: label.into(),
            args,
            recipients,
            trigger_response: false,
        }
    }

    /// Marks this event as expecting a response.
    ///
    /// When set, an `ScriptCallbackResponse` event will be emitted when the callback is completed with the result of the callback IF the callback was executed.
    pub fn with_response(mut self) -> Self {
        self.trigger_response = true;
        self
    }

    /// Creates a new callback event with the given label, arguments and all scripts as recipients
    pub fn new_for_all<L: Into<CallbackLabel>>(label: L, args: Vec<ScriptValue>) -> Self {
        Self::new(label, args, Recipients::All)
    }
}

/// Event published when a script completes a callback and a response is requested.
#[derive(Clone, Event, Debug)]
#[non_exhaustive]
pub struct ScriptCallbackResponseEvent {
    /// the label of the callback
    pub label: CallbackLabel,
    /// the key to the context that replied
    pub context_key: ContextKey,
    /// the response received
    pub response: Result<ScriptValue, ScriptError>,
}

impl ScriptCallbackResponseEvent {
    /// Creates a new callback response event with the given label, script, and response.
    pub fn new<L: Into<CallbackLabel>>(
        label: L,
        context_key: impl Into<ContextKey>,
        response: Result<ScriptValue, ScriptError>,
    ) -> Self {
        Self {
            label: label.into(),
            context_key: context_key.into(),
            response,
        }
    }

    /// Return the source entity for the callback if there was any.
    pub fn source_entity(&self) -> Option<Entity> {
        self.context_key.entity
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
            assert_eq!(super::CallbackLabel::new(&format!("bad{char}")), None);
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
            assert_eq!(super::CallbackLabel::new(&format!("{char}bad")), None);
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
