use std::{borrow::Cow, fmt::Debug};
use tealr::mlu::mlua;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("Runtime error in script `{script}` {msg}")]
    RuntimeError { script: String, msg: String },
    #[error("Failed to load script asset for `{script}`")]
    FailedToLoad { script: String },
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

#[derive(Error, Debug)]
pub enum ReflectionError {
    #[error("Base reference `{base}` is invalid. {reason}")]
    InvalidBaseReference { base: String, reason: String },
    #[error("Insuficient provenance error while accessing `{path}`. {msg}")]
    InsufficientProvenance { path: String, msg: String },
    #[error("Invalid reflection path: `{path}`. {msg}")]
    InvalidReflectionPath { path: String, msg: String },
    #[error("Cannot downcast from `{from}` to `{to}`")]
    CannotDowncast {
        from: Cow<'static, str>,
        to: Cow<'static, str>,
    },
    #[error("{0}")]
    Other(String),
}

// impl Into<mlua::Error> for ReflectionError {
//     fn into(self) -> mlua::Error {
//         mlua::Error::RuntimeError(self.to_string())
//     }
// }

impl From<ReflectionError> for mlua::Error {
    fn from(e: ReflectionError) -> Self {
        Self::RuntimeError(e.to_string())
    }
}

impl From<mlua::Error> for ScriptError {
    fn from(e: mlua::Error) -> Self {
        Self::Other(e.to_string())
    }
}
