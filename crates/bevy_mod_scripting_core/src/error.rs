//! Errors that can occur when interacting with the scripting system

use std::{
    borrow::Cow,
    fmt::{Debug, Display, Write},
    ops::Deref,
    str::Utf8Error,
    sync::Arc,
};

use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::InteropError;

use ::bevy_reflect::Reflect;
use bevy_mod_scripting_display::{DebugWithTypeInfo, DisplayWithTypeInfo, WithTypeInfo};

/// An error with an optional script Context
#[derive(Debug, Clone, Reflect)]
#[reflect(opaque)]
pub struct ScriptError(pub Arc<ScriptErrorInner>);

impl DisplayWithTypeInfo for ScriptError {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn bevy_mod_scripting_display::GetTypeInfo>,
    ) -> std::fmt::Result {
        self.0.display_with_type_info(f, type_info_provider)
    }
}

impl std::error::Error for ScriptError {}

impl Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ScriptError {
    type Target = ScriptErrorInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
/// The reason for a scripting error
pub enum Reason {
    /// An error implementing the standard error interface with no need for type info
    WithoutTypeInfo(Arc<dyn std::error::Error + Send + Sync + 'static>),
    /// An error which can make use of the `GetTypeInfo` retrieval to print itself better.
    WithTypeInfo(Arc<dyn DisplayWithTypeInfo + Send + Sync + 'static>),
}

impl DebugWithTypeInfo for Reason {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn bevy_mod_scripting_display::GetTypeInfo>,
    ) -> std::fmt::Result {
        match self {
            Reason::WithoutTypeInfo(err) => write!(f, "{err}"),
            Reason::WithTypeInfo(err) => err.display_with_type_info(f, type_info_provider),
        }
    }
}

impl Debug for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&WithTypeInfo::new(self), f)
    }
}

impl DisplayWithTypeInfo for Reason {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn bevy_mod_scripting_display::GetTypeInfo>,
    ) -> std::fmt::Result {
        match self {
            Reason::WithoutTypeInfo(err) => write!(f, "{err}"),
            Reason::WithTypeInfo(err) => err.display_with_type_info(f, type_info_provider),
        }
    }
}

#[derive(Clone)]
/// A piece context added to a script error.
pub enum Context {
    /// A piece of string context
    String(Cow<'static, str>),
    /// A piece of type information dependent context
    WithTypeInfo(
        Option<Cow<'static, str>>,
        Arc<dyn DisplayWithTypeInfo + Send + Sync + 'static>,
    ),
}

impl DebugWithTypeInfo for Context {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn bevy_mod_scripting_display::GetTypeInfo>,
    ) -> std::fmt::Result {
        self.display_with_type_info(f, type_info_provider)
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&WithTypeInfo::new(self), f)
    }
}

impl DisplayWithTypeInfo for Context {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn bevy_mod_scripting_display::GetTypeInfo>,
    ) -> std::fmt::Result {
        match self {
            Context::String(cow) => f.write_str(cow),
            Context::WithTypeInfo(prefix, display_with_type_info) => {
                if let Some(prefix) = prefix {
                    f.write_str(prefix)?;
                }
                display_with_type_info.display_with_type_info(f, type_info_provider)
            }
        }
    }
}

/// The innards are separated to reduce the size of this error
#[derive(Debug, Clone)]
pub struct ScriptErrorInner {
    /// The script that caused the error
    pub script: Option<String>,
    /// The context in which the error occurred
    pub context: Vec<Context>,
    /// The error that occurred
    pub reason: Reason,
    /// The language in whose context the error happened
    pub language: Language,
}

impl DisplayWithTypeInfo for ScriptErrorInner {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn bevy_mod_scripting_display::GetTypeInfo>,
    ) -> std::fmt::Result {
        f.write_str("Error ")?;
        if let Some(script) = &self.script {
            f.write_str("in script: '")?;
            f.write_str(script)?;
            f.write_str("' ")?;
        }
        f.write_str(":\n")?;

        self.reason.display_with_type_info(f, type_info_provider)?;

        if !self.context.is_empty() {
            f.write_str("\nContext:\n")?;
            let mut first = true;
            for c in &self.context {
                if !first {
                    f.write_char('\n')?;
                }
                first = false;
                c.display_with_type_info(f, type_info_provider)?;
            }
        }

        f.write_str("\nLanguage: ")?;
        f.write_str(&Cow::<'static, str>::from(&self.language))
    }
}

impl Display for ScriptErrorInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&WithTypeInfo::new(self), f)
    }
}

impl ScriptError {
    /// Creates a new script error with an external error
    pub fn new_without_type_info(reason: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::new_boxed_without_type_info(Box::new(reason))
    }

    /// Creates a new script error with an external error
    pub fn new_boxed_without_type_info(
        reason: Box<dyn std::error::Error + Send + Sync + 'static>,
    ) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: None,
            reason: Reason::WithoutTypeInfo(Arc::from(reason)),
            context: Default::default(),
            language: Default::default(),
        }))
    }

    /// Creates a new script error with a reason that has type info
    pub fn new_with_type_info(reason: impl DisplayWithTypeInfo + Send + Sync + 'static) -> Self {
        Self::new_boxed_with_type_info(Box::new(reason))
    }

    /// Creates a new script error with a reason that has type info
    pub fn new_boxed_with_type_info(
        reason: Box<dyn DisplayWithTypeInfo + Send + Sync + 'static>,
    ) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: None,
            reason: Reason::WithTypeInfo(Arc::from(reason)),
            context: Default::default(),
            language: Default::default(),
        }))
    }

    /// Creates a new script error with a reason
    pub fn with_script<S: ToString>(self, script: S) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: Some(script.to_string()),
            context: self.0.context.clone(),
            reason: self.0.reason.clone(),
            language: self.0.language.clone(),
        }))
    }

    /// Sets the language context on the error
    pub fn with_language(self, language: Language) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: self.0.script.clone(),
            context: self.0.context.clone(),
            reason: self.0.reason.clone(),
            language,
        }))
    }

    /// Adds string context to the error
    pub fn with_context(self, context: impl Into<Cow<'static, str>>) -> Self {
        let script = self.0.script.clone();
        let reason = self.0.reason.clone();
        let mut new_ctxt = self.0.context.clone();
        new_ctxt.push(Context::String(context.into()));

        Self(Arc::new(ScriptErrorInner {
            language: self.language.clone(),
            script,
            context: new_ctxt,
            reason,
        }))
    }

    /// Adds context which requires type information to be printed
    pub fn with_type_info_context(
        self,
        prefix: Option<impl Into<Cow<'static, str>>>,
        context: impl DisplayWithTypeInfo + Send + Sync + 'static,
    ) -> Self {
        let script = self.0.script.clone();
        let reason = self.0.reason.clone();
        let mut new_ctxt = self.0.context.clone();
        new_ctxt.push(Context::WithTypeInfo(
            prefix.map(Into::into),
            Arc::new(context),
        ));

        Self(Arc::new(ScriptErrorInner {
            language: self.language.clone(),
            script,
            context: new_ctxt,
            reason,
        }))
    }

    /// Adds context which requires type information to be printed in boxed form.
    pub fn with_type_info_context_boxed(
        self,
        prefix: Option<impl Into<Cow<'static, str>>>,
        context: Box<dyn DisplayWithTypeInfo + Send + Sync + 'static>,
    ) -> Self {
        let script = self.0.script.clone();
        let reason = self.0.reason.clone();
        let mut new_ctxt = self.0.context.clone();
        new_ctxt.push(Context::WithTypeInfo(
            prefix.map(Into::into),
            Arc::from(context),
        ));

        Self(Arc::new(ScriptErrorInner {
            language: self.language.clone(),
            script,
            context: new_ctxt,
            reason,
        }))
    }
}

#[derive(Clone, Debug, PartialEq)]
/// An error thrown when a resource is missing
pub struct MissingResourceError(&'static str);

impl MissingResourceError {
    /// Creates a new missing resource error
    pub fn new<R>() -> Self {
        Self(std::any::type_name::<R>())
    }
}

impl Display for MissingResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Missing resource: {}. Was the plugin initialized correctly?",
            self.0
        )
    }
}

impl std::error::Error for MissingResourceError {}

impl From<InteropError> for ScriptError {
    fn from(val: InteropError) -> Self {
        let (ctxt, err) = val.unwrap_context();
        let mut err = ScriptError::new_with_type_info(err);
        for ctxt in ctxt {
            err = err.with_context(ctxt);
        }
        err
    }
}

impl From<Utf8Error> for ScriptError {
    fn from(val: Utf8Error) -> Self {
        ScriptError::new_without_type_info(val)
    }
}

/// Utility trait for flattening errors
pub trait FlattenError<O, E> {
    /// Flattens the error into a single error type
    fn flatten_interop_error(self) -> Result<O, E>;
}

impl<O> FlattenError<O, InteropError> for Result<Result<O, InteropError>, InteropError> {
    fn flatten_interop_error(self) -> Result<O, InteropError> {
        match self {
            Ok(Ok(o)) => Ok(o),
            Ok(Err(e)) => Err(e),
            Err(e) => Err(e),
        }
    }
}
