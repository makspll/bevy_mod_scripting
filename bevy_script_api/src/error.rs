use std::borrow::Cow;

use thiserror::Error;

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

#[cfg(feature = "lua")]
impl From<ReflectionError> for bevy_mod_scripting_lua::tealr::mlu::mlua::Error {
    fn from(e: ReflectionError) -> Self {
        bevy_mod_scripting_lua::tealr::mlu::mlua::Error::RuntimeError(e.to_string())
    }
}

impl From<ReflectionError> for Box<bevy_mod_scripting_rhai::rhai::EvalAltResult> {
    fn from(e: ReflectionError) -> Self {
        bevy_mod_scripting_rhai::rhai::EvalAltResult::ErrorRuntime(
            e.to_string().into(),
            bevy_mod_scripting_rhai::rhai::Position::NONE,
        )
        .into()
    }
}
