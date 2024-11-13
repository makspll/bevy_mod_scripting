use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use bevy::reflect::Reflect;
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
#[derive(Debug, Clone)]
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
                "Encountered {} error in script `{}`: {}",
                self.0.kind, script, self.0.reason
            )
        } else {
            write!(
                f,
                "Encountered {} error in a script: {}",
                self.0.kind, self.0.reason
            )
        }
    }
}

// #[derive(Error, Debug, Clone)]
// pub enum ReflectionError {
//     #[error("Base reference `{base}` is invalid. {reason}")]
//     InvalidBaseReference { base: String, reason: String },
//     #[error("Cannot safely access `{base}`. {reason}")]
//     InsufficientAccess { base: String, reason: String },
//     #[error("Tried to access `{base:?}` with insufficient provenance. {reason}")]
//     InsufficientProvenance {
//         base: ReflectReference,
//         reason: String,
//     },
//     #[error("Cannot downcast reference: {reference:?} to: {to}")]
//     CannotDowncast {
//         reference: ReflectReference,
//         to: String,
//     },
//     #[error("Could not assign `{rhs}` to `{lhs:?}`. {reason}")]
//     InvalidAssignment {
//         lhs: ReflectReference,
//         rhs: String,
//         reason: String,
//     },
//     #[error("Failed to build concrete type from &Reflect type: `{ref_}`. Does this type have a FromReflect type data?")]
//     FromReflectFailure { ref_: String },
//     #[error("Could not dereference script allocation with ID: {id}. {reason}")]
//     AllocationError {
//         id: ReflectAllocationId,
//         reason: String,
//     },
//     #[error("Attempted to access world via stale world reference. Did you store a reference to a world across a frame boundary?")]
//     StaleWorldAccess,

//     #[error("{0}")]
//     Other(String),
// }
