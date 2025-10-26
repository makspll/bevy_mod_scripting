//! "Runtime" here refers to the execution evironment of scripts. This might be the VM executing bytecode or the interpreter executing source code.
//! The important thing is that there is only one runtime which is used to execute all scripts of a particular type or `context`.

use crate::{IntoScriptPluginParams, error::ScriptError};

/// A trait that all script runtimes must implement.
pub trait Runtime: Default + 'static + Send + Sync {}
impl<T: Default + 'static + Send + Sync> Runtime for T {}

/// A function that initializes a runtime.
pub type RuntimeInitializer<P> = fn(&<P as IntoScriptPluginParams>::R) -> Result<(), ScriptError>;
