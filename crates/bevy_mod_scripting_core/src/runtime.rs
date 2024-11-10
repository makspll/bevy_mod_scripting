//! "Runtime" here refers to the execution evironment of scripts. This might be the VM executing bytecode or the interpreter executing source code.
//! The important thing is that there is only one runtime which is used to execute all scripts of a particular type or `context`.

use bevy::ecs::system::Resource;

pub trait Runtime: 'static {}
impl<T: 'static> Runtime for T {}

pub type RuntimeInitializer<R> = fn(&mut R);

#[derive(Clone, Resource)]
pub struct RuntimeSettings<R: Runtime> {
    pub initializers: Vec<RuntimeInitializer<R>>,
}

impl<R: Runtime> Default for RuntimeSettings<R> {
    fn default() -> Self {
        Self {
            initializers: Default::default(),
        }
    }
}

/// Stores a particular runtime.
#[derive(Resource)]
pub struct RuntimeContainer<R: Runtime> {
    pub runtime: Option<R>,
}

impl<T: Runtime> Default for RuntimeContainer<T> {
    fn default() -> Self {
        Self {
            runtime: Default::default(),
        }
    }
}
