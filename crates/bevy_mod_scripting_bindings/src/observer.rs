//! Support for scripts observing events.

use crate::ScriptNamespaceAppExt;
use bevy_app::{App, Plugin};
use bevy_ecs::{
    lifecycle::{AddEvent, DespawnEvent, DiscardEvent, InsertEvent, RemoveEvent},
    reflect::ReflectEvent,
};

/// Makes the component lifecycle events observable by scripts, by giving them [`ReflectEvent`] type data.
pub struct ScriptObserverPlugin;

impl Plugin for ScriptObserverPlugin {
    fn build(&self, app: &mut App) {
        app.set_script_namespace::<crate::ScriptEventRegistration>("bevy_mod_scripting")
            .register_type::<AddEvent>()
            .register_type_data::<AddEvent, ReflectEvent>()
            .register_type::<InsertEvent>()
            .register_type_data::<InsertEvent, ReflectEvent>()
            .register_type::<DiscardEvent>()
            .register_type_data::<DiscardEvent, ReflectEvent>()
            .register_type::<RemoveEvent>()
            .register_type_data::<RemoveEvent, ReflectEvent>()
            .register_type::<DespawnEvent>()
            .register_type_data::<DespawnEvent, ReflectEvent>();
    }
}
