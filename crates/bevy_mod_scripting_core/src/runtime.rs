//! "Runtime" here refers to the execution evironment of scripts. This might be the VM executing bytecode or the interpreter executing source code.
//! The important thing is that there is only one runtime which is used to execute all scripts of a particular type or `context`.

use crate::{error::ScriptError, IntoScriptPluginParams};
use bevy::{
    ecs::system::Resource,
    prelude::{NonSendMut, Res},
};

pub trait Runtime: 'static {}
impl<T: 'static> Runtime for T {}

pub type RuntimeInitializer<P> =
    fn(&mut <P as IntoScriptPluginParams>::R) -> Result<(), ScriptError>;

#[derive(Resource)]
pub struct RuntimeSettings<P: IntoScriptPluginParams> {
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
    pub runtime: P::R,
}

pub fn initialize_runtime<P: IntoScriptPluginParams>(
    mut runtime: NonSendMut<RuntimeContainer<P>>,
    settings: Res<RuntimeSettings<P>>,
) -> Result<(), ScriptError> {
    for initializer in settings.initializers.iter() {
        (initializer)(&mut runtime.runtime)?;
    }
    Ok(())
}
