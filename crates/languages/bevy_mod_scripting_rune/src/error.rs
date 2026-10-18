//! Conversions between Rune and BMS errors.
use bevy_mod_scripting_bindings::InteropError;
use rune::runtime::{VmError, VmResult};

/// An error which came out of Rune, kept as its message.
#[derive(Debug, Clone)]
pub struct RuneError(pub String);

impl std::fmt::Display for RuneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for RuneError {}

/// A trait to convert Rune errors into [`InteropError`]
pub trait IntoInteropError {
    /// Convert into [`InteropError`]
    fn to_bms_error(self) -> InteropError;
}

impl IntoInteropError for VmError {
    fn to_bms_error(self) -> InteropError {
        InteropError::external(RuneError(self.to_string()))
    }
}

impl IntoInteropError for rune::alloc::Error {
    fn to_bms_error(self) -> InteropError {
        InteropError::external(RuneError(self.to_string()))
    }
}

impl IntoInteropError for rune::runtime::RuntimeError {
    fn to_bms_error(self) -> InteropError {
        InteropError::external(RuneError(self.to_string()))
    }
}

/// Turns a BMS result into one Rune can return from a native function, an error aborts the running script.
pub fn vm<T>(result: Result<T, InteropError>) -> VmResult<T> {
    match result {
        Ok(value) => VmResult::Ok(value),
        Err(error) => VmResult::err(VmError::panic(error.to_string())),
    }
}
