//! "Runtime" here refers to the execution evironment of scripts. This might be the VM executing bytecode or the interpreter executing source code.
//! The important thing is that there is only one runtime which is used to execute all scripts of a particular type or `context`.

use crate::{error::ScriptError, IntoScriptPluginParams};
use bevy::{
    ecs::system::Resource,
    prelude::{NonSendMut, Res},
};

/// A trait that all script runtimes must implement.
pub trait Runtime: 'static {}
impl<T: 'static> Runtime for T {}

/// A function that initializes a runtime.
pub type RuntimeInitializer<P> =
    fn(&mut <P as IntoScriptPluginParams>::R) -> Result<(), ScriptError>;

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

pub(crate) fn initialize_runtime<P: IntoScriptPluginParams>(
    mut runtime: NonSendMut<RuntimeContainer<P>>,
    settings: Res<RuntimeSettings<P>>,
) -> Result<(), ScriptError> {
    for initializer in settings.initializers.iter() {
        (initializer)(&mut runtime.runtime)?;
    }
    Ok(())
}
