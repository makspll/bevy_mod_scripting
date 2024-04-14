use thiserror::Error;

use crate::{allocator::ReflectAllocationId, bindings::ReflectReference};

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
#[derive(Debug)]
pub struct ScriptError {
    pub script: Option<String>,
    pub additional_message: Option<String>,
    pub reason: Box<dyn std::error::Error + Send + Sync + 'static>,
}

impl ScriptError {
    pub fn new(reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self {
            script: None,
            reason,
            additional_message: None,
        }
    }

    pub fn new_with_context<T: ToString>(
        script: T,
        reason: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        Self {
            script: Some(script.to_string()),
            reason,
            additional_message: None,
        }
    }

    pub fn with_context<T: ToString>(self, script: T) -> Self {
        Self {
            script: Some(script.to_string()),
            reason: self.reason,
            additional_message: self.additional_message,
        }
    }

    pub fn with_msg<T: ToString>(self, msg: T) -> Self {
        Self {
            script: self.script,
            reason: self.reason,
            additional_message: Some(msg.to_string()),
        }
    }
}

impl<T: std::error::Error + Send + Sync + 'static> From<T> for ScriptError {
    fn from(value: T) -> Self {
        Self::new(Box::new(value))
    }
}

impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(script) = &self.script {
            write!(f, "Script `{}` failed: {}", script, self.reason)
        } else {
            write!(f, "Script failed: {}", self.reason)
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum ReflectionError {
    #[error("Base reference `{base}` is invalid. {reason}")]
    InvalidBaseReference { base: String, reason: String },
    #[error("Cannot safely access `{base}`. {reason}")]
    InsufficientAccess { base: String, reason: String },

    #[error("Cannot downcast reference: {reference:?} to: {to}")]
    CannotDowncast {
        reference: ReflectReference,
        to: String,
    },
    #[error("Could not dereference script allocation with ID: {id}. {reason}")]
    AllocationError {
        id: ReflectAllocationId,
        reason: String,
    },
    #[error("{0}")]
    Other(String),
}
