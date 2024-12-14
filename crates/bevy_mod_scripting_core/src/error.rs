use std::{
    any::TypeId,
    borrow::Cow,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use bevy::reflect::{ApplyError, Reflect};
use thiserror::Error;

use crate::{bindings::ReflectAllocationId, bindings::ReflectReference};

pub type ScriptResult<T> = Result<T, ScriptError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScriptErrorKind {
    /// Any other error, default for script errors generated via Into conversions
    Other,
    /// Errors specifically to do with reflecting & reading/writing stuff from the world
    ReflectionError,
    /// Erorrs to do with invalid script API usage, invalid arguments etc.
    RuntimeError,
    /// Errors to do with the script lifecycle, loading, unloading etc.
    Lifecycle,
}

impl std::fmt::Display for ScriptErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptErrorKind::ReflectionError => f.write_str("Reflection Error"),
            ScriptErrorKind::Lifecycle => f.write_str("Script Lifecycle Error"),
            ScriptErrorKind::Other => f.write_str("Error"),
            ScriptErrorKind::RuntimeError => f.write_str("Runtime Error"),
        };
        Ok(())
    }
}

#[derive(Error, Debug)]
pub struct ScriptErrorWrapper(ScriptError);

impl std::fmt::Display for ScriptErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ScriptError> for Box<dyn std::error::Error + Send + Sync + 'static> {
    fn from(val: ScriptError) -> Self {
        ScriptErrorWrapper(val).into()
    }
}
/// An error with an optional script Context
#[derive(Debug, Clone, PartialEq, Reflect)]
#[reflect(opaque)]
pub struct ScriptError(pub Arc<ScriptErrorInner>);

impl Deref for ScriptError {
    type Target = ScriptErrorInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The innards are separated to reduce the size of this error
#[derive(Debug)]
pub struct ScriptErrorInner {
    pub script: Option<String>,
    pub kind: ScriptErrorKind,
    pub context: String,
    pub reason: Arc<dyn std::error::Error + Send + Sync>,
}

impl PartialEq for ScriptErrorInner {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.context == other.context
            && self.reason.to_string() == other.reason.to_string()
    }
}

impl ScriptError {
    pub fn new_reflection_error<E: Into<Box<dyn std::error::Error + Send + Sync>>>(
        reason: E,
    ) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: None,
            kind: ScriptErrorKind::ReflectionError,
            reason: Arc::from(reason.into()),
            context: Default::default(),
        }))
    }

    pub fn new_generic_error<E: Into<Box<dyn std::error::Error + Send + Sync>>>(reason: E) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: None,
            kind: ScriptErrorKind::Other,
            reason: Arc::from(reason.into()),
            context: Default::default(),
        }))
    }

    pub fn new_lifecycle_error<E: Into<Box<dyn std::error::Error + Send + Sync>>>(
        reason: E,
    ) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: None,
            kind: ScriptErrorKind::Lifecycle,
            reason: Arc::from(reason.into()),
            context: Default::default(),
        }))
    }

    pub fn new_runtime_error<E: Into<Box<dyn std::error::Error + Send + Sync>>>(reason: E) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: None,
            kind: ScriptErrorKind::RuntimeError,
            reason: Arc::from(reason.into()),
            context: Default::default(),
        }))
    }

    pub fn with_context<S: ToString>(self, context: S) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: self.0.script.clone(),
            kind: self.0.kind,
            context: context.to_string(),
            reason: self.0.reason.clone(),
        }))
    }

    pub fn with_appended_context<S: ToString>(self, context: S) -> Self {
        Self(Arc::new(ScriptErrorInner {
            script: self.0.script.clone(),
            kind: self.0.kind,
            context: format!("{}. {}", self.0.context, context.to_string()),
            reason: self.0.reason.clone(),
        }))
    }
}

impl<T: std::error::Error + Send + Sync + 'static> From<T> for ScriptError {
    fn from(value: T) -> Self {
        Self::new_generic_error(value)
    }
}

impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(script) = &self.0.script {
            write!(
                f,
                "{} in script `{}`: {}",
                self.0.kind, script, self.0.reason
            )
        } else {
            write!(f, "{}: {}", self.0.kind, self.0.reason)
        }
    }
}

#[cfg(feature = "mlua_impls")]
impl From<ScriptError> for mlua::Error {
    fn from(value: ScriptError) -> Self {
        mlua::Error::external(value)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ValueConversionError {
    #[error("Expected type: {expected_type}, got: {actual_type:?}")]
    TypeMismatch {
        expected_type: Cow<'static, str>,
        actual_type: Option<Cow<'static, str>>,
    },
    #[error("Could not index script value {base:?} using index {index}. Reason: {reason:?}")]
    InvalidIndex {
        index: Cow<'static, str>,
        base: Option<Cow<'static, str>>,
        reason: Option<Cow<'static, str>>,
    },
    #[error("Type was not registed with the type registry: {type_id:?}. Could not convert.")]
    MissingTypeInformation { type_id: TypeId },
}

#[derive(thiserror::Error, Debug)]
pub enum FunctionError {
    #[error("Function {function_name:?} not found in type registry for type {type_:?}")]
    FunctionNotFound {
        function_name: Cow<'static, str>,
        type_: Option<Cow<'static, str>>,
    },
}

#[derive(thiserror::Error, Debug)]
pub enum ReflectReferenceError {
    #[error("Reference could not be reflected, due to missing component, entity or resource. {reason:?}")]
    InvalidBaseReference { reason: Cow<'static, str> },
}
