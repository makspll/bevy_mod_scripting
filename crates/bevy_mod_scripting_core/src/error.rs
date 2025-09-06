//! Errors that can occur when interacting with the scripting system

use std::{
    fmt::{Debug, Display},
    ops::Deref,
    str::Utf8Error,
    sync::Arc,
};

use bevy_mod_scripting_bindings::InteropError;

use ::bevy_reflect::Reflect;

/// An error with an optional script Context
#[derive(Debug, Clone, Reflect)]
#[reflect(opaque)]
pub struct ScriptError(pub Arc<ScriptErrorInner>);

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

/// The innards are separated to reduce the size of this error
#[derive(Debug, Clone)]
pub struct ScriptErrorInner {
    /// The script that caused the error
    pub script: Option<String>,
    /// The context in which the error occurred
    pub context: String,
    /// The error that occurred
    pub reason: Arc<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl Display for ScriptErrorInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(script) = &self.script {
            write!(
                f,
                "Script Error in script '{}': {}\nContext: {}",
                script, self.reason, self.context
            )
        } else {
            write!(
                f,
                "Script Error: {}\nContext: {}",
                self.reason, self.context
            )
        }
    }
}

impl ScriptError {
    /// Creates a new script error with an external error
    pub fn new(reason: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::new_boxed(Box::new(reason))
    }

    /// Creates a new script error with an external error
    pub fn new_boxed(reason: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: None,
            reason: Arc::new(reason),
            context: Default::default(),
        }))
    }

    /// Creates a new script error with a reason
    pub fn with_script<S: ToString>(self, script: S) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: Some(script.to_string()),
            context: self.0.context.clone(),
            reason: self.0.reason.clone(),
        }))
    }

    /// Adds context to the error
    pub fn with_context<S: ToString>(self, context: S) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: self.0.script.clone(),
            context: format!("{}\n{}", self.0.context, context.to_string()),
            reason: self.0.reason.clone(),
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
        let mut err = ScriptError::new(err);
        for ctxt in ctxt {
            err = err.with_context(ctxt);
        }
        err
    }
}

impl From<Utf8Error> for ScriptError {
    fn from(val: Utf8Error) -> Self {
        ScriptError::new(val)
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
