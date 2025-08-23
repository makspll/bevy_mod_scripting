//! "Runtime" here refers to the execution evironment of scripts. This might be the VM executing bytecode or the interpreter executing source code.
//! The important thing is that there is only one runtime which is used to execute all scripts of a particular type or `context`.

use ::bevy_ecs::{resource::Resource, system::Res, system::ResMut};

use crate::{IntoScriptPluginParams, error::ScriptError};

/// A trait that all script runtimes must implement.
pub trait Runtime: Default + 'static + Send + Sync {}
impl<T: Default + 'static + Send + Sync> Runtime for T {}

/// A function that initializes a runtime.
pub type RuntimeInitializer<P> = fn(&<P as IntoScriptPluginParams>::R) -> Result<(), ScriptError>;

#[derive(Resource)]
/// Resource storing settings for a scripting plugin regarding runtime initialization & configuration.
pub struct RuntimeSettings<P: IntoScriptPluginParams> {
    /// Initializers for the runtime. These are run when the runtime is initialized.
    pub initializers: Vec<RuntimeInitializer<P>>,
}

impl<P: IntoScriptPluginParams> Default for RuntimeSettings<P> {
    fn default() -> Self {
        Self {
            initializers: Default::default(),
        }
    }
}

impl<P: IntoScriptPluginParams> Clone for RuntimeSettings<P> {
    fn clone(&self) -> Self {
        Self {
            initializers: self.initializers.clone(),
        }
    }
}

/// Stores a particular runtime.
#[derive(Resource)]
pub struct RuntimeContainer<P: IntoScriptPluginParams> {
    /// The runtime contained within.
    pub runtime: P::R,
}

impl<P: IntoScriptPluginParams> Default for RuntimeContainer<P> {
    fn default() -> Self {
        Self {
            runtime: Default::default(),
        }
    }
}

#[profiling::function]
pub(crate) fn initialize_runtime<P: IntoScriptPluginParams>(
    runtime: ResMut<RuntimeContainer<P>>,
    settings: Res<RuntimeSettings<P>>,
) -> Result<(), ScriptError> {
    for initializer in settings.initializers.iter() {
        (initializer)(&runtime.runtime)?;
    }
    Ok(())
}
