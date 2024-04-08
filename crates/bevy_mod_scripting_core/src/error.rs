use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ScriptError {
    #[error("Runtime error in script `{script}` {msg}")]
    RuntimeError { script: String, msg: String },
    #[error("Failed to load script asset for `{script}` {msg}")]
    FailedToLoad { script: String, msg: String },
    #[error("Syntax error for script `{script}` {msg}")]
    SyntaxError { script: String, msg: String },
    #[error("Callback method `{callback}` invalid for script `{script}` {msg}")]
    InvalidCallback {
        script: String,
        callback: String,
        msg: String,
    },
    #[error("Failed to attach API for script `{script}` {msg}")]
    FailedToAttachAPI { script: String, msg: String },
    #[error("Failed to generate documentation `{0}`")]
    DocGenError(String),
    #[error("{0}")]
    Other(String),
}

impl ScriptError {
    /// Create new `ScriptError::Other` from another error
    pub fn new_other<T: std::error::Error>(other: T) -> Self {
        Self::Other(other.to_string())
    }
}

#[derive(Error, Debug, Clone)]
pub enum ReflectionError {
    #[error("Base reference `{base}` is invalid. {reason}")]
    InvalidBaseReference { base: String, reason: String },
    #[error("Cannot safely access `{base}`. {reason}")]
    InsufficientAccess { base: String, reason: String },
    #[error("Insuficient provenance error while accessing `{path}`. {msg}")]
    InsufficientProvenance { path: String, msg: String },
    #[error("Invalid reflection path: `{path}`. {msg}")]
    InvalidReflectionPath { path: String, msg: String },

    #[error("{0}")]
    Other(String),
}
