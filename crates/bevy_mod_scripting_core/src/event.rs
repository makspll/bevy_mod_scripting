//! Event handlers and event types for scripting.

use std::{marker::PhantomData, sync::Arc};

use ::{bevy_asset::Handle, bevy_ecs::entity::Entity, bevy_reflect::Reflect};
use bevy_ecs::event::Event;
use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::ScriptValue;
use bevy_mod_scripting_script::ScriptAttachment;
use parking_lot::Mutex;

use crate::{
    IntoScriptPluginParams,
    error::ScriptError,
    script::{ScriptContext, ScriptId},
};

/// An error coming from a script
#[derive(Debug, Event)]
pub struct ScriptErrorEvent {
    /// The script that caused the error
    pub error: ScriptError,
}

impl ScriptErrorEvent {
    /// Creates a new script error event from a script error
    pub fn new(error: ScriptError) -> Self {
        Self { error }
    }
}

/// Emitted when a script is attached.
#[derive(Event, Clone, Debug)]
pub struct ScriptAttachedEvent(pub ScriptAttachment);

/// Emitted when a script is detached.
#[derive(Event, Clone, Debug)]
pub struct ScriptDetachedEvent(pub ScriptAttachment);

/// Emitted when a script asset is modified and all its attachments require re-loading
#[derive(Event, Clone, Debug)]
pub struct ScriptAssetModifiedEvent(pub ScriptId);

#[derive(Event)]
/// Wrapper around a script event making it available to read by a specific plugin only
pub struct ForPlugin<T, P: IntoScriptPluginParams>(T, PhantomData<fn(P)>);

impl<T: std::fmt::Debug, P: IntoScriptPluginParams> std::fmt::Debug for ForPlugin<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ForPlugin").field(&self.0).finish()
    }
}

impl<T, P: IntoScriptPluginParams> From<T> for ForPlugin<T, P> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Clone, P: IntoScriptPluginParams> Clone for ForPlugin<T, P> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}

impl<T, P: IntoScriptPluginParams> ForPlugin<T, P> {
    /// Creates a new wrapper for the specific plugin
    pub fn new(event: T) -> Self {
        Self(event, Default::default())
    }

    /// Retrieves the inner event
    pub fn event(&self) -> &T {
        &self.0
    }

    /// Retrieves the inner event mutably
    pub fn event_mut(&mut self) -> &mut T {
        &mut self.0
    }

    /// Unpacks the inner event
    pub fn inner(self) -> T {
        self.0
    }
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
    /// The event needs to be handled by all scripts, if multiple scripts share a context, the event will be sent once per script in the context.
    AllScripts,
    /// The event is to be handled by all unique contexts, i.e. if two scripts share the same context, the event will be sent only once per the context.
    AllContexts,
    /// The event is to be handled by a specific script-entity pair
    ScriptEntity(ScriptId, Entity),
    /// the event is to be handled by a specific static script
    StaticScript(ScriptId),
}

impl Recipients {
    /// Retrieves all the recipients of the event based on existing scripts
    pub fn get_recipients<P: IntoScriptPluginParams>(
        &self,
        script_context: ScriptContext<P>,
    ) -> Vec<(ScriptAttachment, Arc<Mutex<P::C>>)> {
        let script_context = script_context.read();
        match self {
            Recipients::AllScripts => script_context.all_residents().collect(),
            Recipients::AllContexts => script_context.first_resident_from_each_context().collect(),
            Recipients::ScriptEntity(script, entity) => {
                let attachment = ScriptAttachment::EntityScript(*entity, Handle::Weak(*script));
                script_context
                    .get_context(&attachment)
                    .into_iter()
                    .map(|entry| (attachment.clone(), entry))
                    .collect()
            }
            Recipients::StaticScript(script) => {
                let attachment = ScriptAttachment::StaticScript(Handle::Weak(*script));
                script_context
                    .get_context(&attachment)
                    .into_iter()
                    .map(|entry| (attachment.clone(), entry))
                    .collect()
            }
        }
    }
}

/// A callback event meant to trigger a callback in a subset/set of scripts in the world with the given arguments
#[derive(Clone, Event, Debug)]
#[non_exhaustive]
pub struct ScriptCallbackEvent {
    /// The label of the callback
    pub label: CallbackLabel,
    /// The recipients of the callback
    pub recipients: Recipients,
    /// The language of the callback, if unspecified will apply to all languages
    pub language: Option<Language>,
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
        language: Option<Language>,
    ) -> Self {
        Self {
            label: label.into(),
            language,
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

    /// Creates a new callback event with the given label, arguments and all scripts and languages as recipients
    pub fn new_for_all_scripts<L: Into<CallbackLabel>>(label: L, args: Vec<ScriptValue>) -> Self {
        Self::new(label, args, Recipients::AllScripts, None)
    }

    /// Creates a new callback event with the given label, arguments and all contexts (which can contain multiple scripts) and languages as recipients
    pub fn new_for_all_contexts<L: Into<CallbackLabel>>(label: L, args: Vec<ScriptValue>) -> Self {
        Self::new(label, args, Recipients::AllContexts, None)
    }
}

/// Event published when a script completes a callback and a response is requested.
#[derive(Clone, Event, Debug)]
#[non_exhaustive]
pub struct ScriptCallbackResponseEvent {
    /// the label of the callback
    pub label: CallbackLabel,
    /// the language of the callback that replied
    pub language: Language,
    /// the key to the context that replied
    pub context_key: ScriptAttachment,
    /// the response received
    pub response: Result<ScriptValue, ScriptError>,
}

impl ScriptCallbackResponseEvent {
    /// Creates a new callback response event with the given label, script, and response.
    pub fn new<L: Into<CallbackLabel>>(
        label: L,
        context_key: ScriptAttachment,
        response: Result<ScriptValue, ScriptError>,
        language: Language,
    ) -> Self {
        Self {
            label: label.into(),
            context_key,
            response,
            language,
        }
    }

    /// Return the source entity for the callback if there was any.
    pub fn source_entity(&self) -> Option<Entity> {
        self.context_key.entity()
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
    use super::*;
    use std::sync::Arc;

    use ::{
        bevy_app::{App, Plugin},
        bevy_asset::{AssetId, AssetIndex, Handle},
        bevy_ecs::entity::Entity,
    };
    use parking_lot::Mutex;
    use test_utils::make_test_plugin;

    use super::FORBIDDEN_KEYWORDS;
    use crate::{
        config::{GetPluginThreadConfig, ScriptingPluginConfiguration},
        event::Recipients,
        script::{ContextPolicy, ScriptContext},
    };

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

    make_test_plugin!(crate);

    /// make the following arrangement:
    /// use AssetId's to identify residents
    /// ContextA:
    /// - EntityScriptA (Entity::from_raw(0), AssetId::from_bits(0))
    /// - EntityScriptB (Entity::from_raw(0), AssetId::from_bits(1))
    ///
    /// ContextB:
    /// - EntityScriptC (Entity::from_raw(1), AssetId::from_bits(2))
    /// - EntityScriptD (Entity::from_raw(1), AssetId::from_bits(3))
    ///
    /// ContextC:
    /// - StaticScriptA (AssetId::from_bits(4))
    ///
    /// ContextD:
    /// - StaticScriptB (AssetId::from_bits(5))
    fn make_test_contexts() -> ScriptContext<TestPlugin> {
        let policy = ContextPolicy::per_entity();
        let script_context = ScriptContext::<TestPlugin>::new(policy);
        let mut script_context_guard = script_context.write();
        let context_a = Arc::new(Mutex::new(TestContext {
            invocations: vec![ScriptValue::String("a".to_string().into())],
        }));
        let context_b = Arc::new(Mutex::new(TestContext {
            invocations: vec![ScriptValue::String("b".to_string().into())],
        }));
        let context_c = Arc::new(Mutex::new(TestContext {
            invocations: vec![ScriptValue::String("c".to_string().into())],
        }));
        let context_d = Arc::new(Mutex::new(TestContext {
            invocations: vec![ScriptValue::String("d".to_string().into())],
        }));

        let entity_script_a = Handle::Weak(AssetId::from(AssetIndex::from_bits(0)));
        let entity_script_b = Handle::Weak(AssetId::from(AssetIndex::from_bits(1)));
        let entity_script_c = Handle::Weak(AssetId::from(AssetIndex::from_bits(2)));
        let entity_script_d = Handle::Weak(AssetId::from(AssetIndex::from_bits(3)));

        let static_script_a = Handle::Weak(AssetId::from(AssetIndex::from_bits(4)));
        let static_script_b = Handle::Weak(AssetId::from(AssetIndex::from_bits(5)));

        script_context_guard
            .insert(
                ScriptAttachment::EntityScript(Entity::from_raw(0), entity_script_a),
                context_a,
            )
            .unwrap();

        script_context_guard
            .insert_resident(ScriptAttachment::EntityScript(
                Entity::from_raw(0),
                entity_script_b,
            ))
            .unwrap();

        script_context_guard
            .insert(
                ScriptAttachment::EntityScript(Entity::from_raw(1), entity_script_c),
                context_b,
            )
            .unwrap();
        script_context_guard
            .insert_resident(ScriptAttachment::EntityScript(
                Entity::from_raw(1),
                entity_script_d,
            ))
            .unwrap();

        script_context_guard
            .insert(ScriptAttachment::StaticScript(static_script_a), context_c)
            .unwrap();

        script_context_guard
            .insert(ScriptAttachment::StaticScript(static_script_b), context_d)
            .unwrap();

        drop(script_context_guard);
        script_context
    }

    fn recipients_to_asset_ids(
        recipients: &[(ScriptAttachment, Arc<Mutex<TestContext>>)],
    ) -> Vec<(usize, String)> {
        recipients
            .iter()
            .map(|(attachment, context)| {
                if let AssetId::Index { index, .. } = attachment.script().id() {
                    let locked = context.lock();
                    let first_invocation_string =
                        if let Some(ScriptValue::String(s)) = locked.invocations.first() {
                            s.clone()
                        } else {
                            panic!("Expected first invocation to be a string")
                        };
                    (
                        index.to_bits() as usize,
                        first_invocation_string.to_string(),
                    )
                } else {
                    panic!(
                        "Expected AssetId::Index, got {:?}",
                        attachment.script().id()
                    )
                }
            })
            .collect()
    }

    #[test]
    fn test_all_scripts_recipients() {
        let script_context = make_test_contexts();
        let recipients = Recipients::AllScripts.get_recipients(script_context);
        assert_eq!(recipients.len(), 6);
        let mut id_context_pairs = recipients_to_asset_ids(&recipients);

        id_context_pairs.sort_by_key(|(id, _)| *id);

        assert_eq!(
            id_context_pairs,
            vec![
                (0, "a".to_string()),
                (1, "a".to_string()),
                (2, "b".to_string()),
                (3, "b".to_string()),
                (4, "c".to_string()),
                (5, "d".to_string()),
            ]
        );
    }

    #[test]
    fn test_all_contexts_recipients() {
        let script_context = make_test_contexts();
        let recipients = Recipients::AllContexts.get_recipients(script_context);
        assert_eq!(recipients.len(), 4);
        let mut id_context_pairs = recipients_to_asset_ids(&recipients);
        id_context_pairs.sort_by_key(|(id, _)| *id);

        // expect one of 0,1 for context a and one of 2,3 for context b
        // and 4 for context c and 5 for context d

        // we can't just use equality here because the order of contexts is not guaranteed
        assert!(
            id_context_pairs.contains(&(0, "a".to_string()))
                || id_context_pairs.contains(&(1, "a".to_string()))
        );
        assert!(
            id_context_pairs.contains(&(2, "b".to_string()))
                || id_context_pairs.contains(&(3, "b".to_string()))
        );
        assert!(id_context_pairs.contains(&(4, "c".to_string())));
        assert!(id_context_pairs.contains(&(5, "d".to_string())));
    }

    #[test]
    fn test_script_entity_recipients() {
        let script_context = make_test_contexts();
        let recipients =
            Recipients::ScriptEntity(AssetId::from(AssetIndex::from_bits(0)), Entity::from_raw(0))
                .get_recipients(script_context);

        assert_eq!(recipients.len(), 1);
        let id_context_pairs = recipients_to_asset_ids(&recipients);
        assert_eq!(id_context_pairs, vec![(0, "a".to_string())]);
    }

    #[test]
    fn test_static_script_recipients() {
        let script_context = make_test_contexts();
        let recipients = Recipients::StaticScript(AssetId::from(AssetIndex::from_bits(4)))
            .get_recipients(script_context);

        assert_eq!(recipients.len(), 1);
        let id_context_pairs = recipients_to_asset_ids(&recipients);
        assert_eq!(id_context_pairs, vec![(4, "c".to_string())]);
    }
}
