use std::fmt::Debug;
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

impl From<mlua::Error> for ScriptError {
    fn from(e: mlua::Error) -> Self {
        Self::Other(e.to_string())
    }
}
