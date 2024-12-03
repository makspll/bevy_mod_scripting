use bevy::ecs::{entity::Entity, system::Resource, world::World};

use crate::{
    context::{Context, ContextPreHandlingInitializer},
    event::CallbackLabel,
    prelude::ScriptError,
    runtime::Runtime,
    script::ScriptId,
};

pub trait Args: Clone + Send + Sync + 'static {}
impl<T: Clone + Send + Sync + 'static> Args for T {}

pub type HandlerFn<A, C, R> = fn(
    args: A,
    entity: Entity,
    script_id: &ScriptId,
    callback: &CallbackLabel,
    context: &mut C,
    pre_handling_initializers: &[ContextPreHandlingInitializer<C>],
    runtime: &mut R,
    world: &mut World,
) -> Result<(), ScriptError>;

/// A resource that holds the settings for the callback handler for a specific combination of type parameters
#[derive(Resource)]
pub struct CallbackSettings<A: Args, C: Context, R: Runtime> {
    pub callback_handler: Option<HandlerFn<A, C, R>>,
}

impl<A: Args, C: Context, R: Runtime> Default for CallbackSettings<A, C, R> {
    fn default() -> Self {
        Self {
            callback_handler: None,
        }
    }
}
