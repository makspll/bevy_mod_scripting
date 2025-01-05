use crate::{
    bindings::script_value::ScriptValue, context::ContextPreHandlingInitializer,
    error::ScriptError, event::CallbackLabel, script::ScriptId, IntoScriptPluginParams,
};
use bevy::ecs::{entity::Entity, system::Resource, world::World};

pub trait Args: Clone + Send + Sync + 'static {}
impl<T: Clone + Send + Sync + 'static> Args for T {}

pub type HandlerFn<P> = fn(
    args: Vec<ScriptValue>,
    entity: Entity,
    script_id: &ScriptId,
    callback: &CallbackLabel,
    context: &mut <P as IntoScriptPluginParams>::C,
    pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
    runtime: &mut <P as IntoScriptPluginParams>::R,
    world: &mut World,
) -> Result<(), ScriptError>;

/// A resource that holds the settings for the callback handler for a specific combination of type parameters
#[derive(Resource)]
pub struct CallbackSettings<P: IntoScriptPluginParams> {
    pub callback_handler: Option<HandlerFn<P>>,
}

impl<P: IntoScriptPluginParams> Default for CallbackSettings<P> {
    fn default() -> Self {
        Self {
            callback_handler: None,
        }
    }
}
