//! Script related types, functions and components

use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use crate::event::{ScriptAttachedEvent, ScriptDetachedEvent};

use ::{
    bevy_asset::{AssetId, Handle},
    bevy_ecs::{
        component::HookContext, entity::Entity, prelude::ReflectComponent, resource::Resource,
        world::DeferredWorld,
    },
    bevy_reflect::Reflect,
};

mod context_key;
mod script_context;
use bevy_ecs::component::Component;
use bevy_log::trace;
use bevy_mod_scripting_asset::ScriptAsset;
use bevy_mod_scripting_script::ScriptAttachment;
pub use context_key::*;
pub use script_context::*;

/// A unique identifier for a script, by default corresponds to the path of the asset excluding the asset source.
///
/// I.e. an asset with the path `path/to/asset.ext` will have the script id `path/to/asset.ext`
pub type ScriptId = AssetId<ScriptAsset>;

#[derive(Component, Reflect, Clone, Default, Debug)]
#[reflect(Component)]
#[component(on_remove=Self::on_remove, on_add=Self::on_add)]
/// A component which identifies the scripts existing on an entity.
///
/// Event handlers search for components with this component to figure out which scripts to run and on which entities.
pub struct ScriptComponent(pub Vec<Handle<ScriptAsset>>);

impl Deref for ScriptComponent {
    type Target = Vec<Handle<ScriptAsset>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ScriptComponent {
    /// Creates a new [`ScriptComponent`] with the given ScriptID's
    pub fn new<S: Into<Handle<ScriptAsset>>, I: IntoIterator<Item = S>>(components: I) -> Self {
        Self(components.into_iter().map(Into::into).collect())
    }

    fn get_context_keys_present(world: &DeferredWorld, entity: Entity) -> Vec<ScriptAttachment> {
        let entity_ref = world.entity(entity);
        let script_component = entity_ref.components::<&ScriptComponent>();
        let mut context_keys = Vec::new();
        for script in script_component.iter() {
            context_keys.push(ScriptAttachment::EntityScript(entity, script.clone()));
        }
        context_keys
    }

    /// the lifecycle hook called when a script component is removed from an entity, emits an appropriate event so we can handle
    /// the removal of the script.
    pub fn on_remove(mut world: DeferredWorld, context: HookContext) {
        let context_keys = Self::get_context_keys_present(&world, context.entity);
        trace!("on remove hook for script components: {context_keys:?}");
        world.send_event_batch(context_keys.into_iter().map(ScriptDetachedEvent));
    }

    /// the lifecycle hook called when a script component is added to an entity, emits an appropriate event so we can handle
    /// the addition of the script.
    pub fn on_add(mut world: DeferredWorld, context: HookContext) {
        let context_keys = Self::get_context_keys_present(&world, context.entity);
        trace!("on add hook for script components: {context_keys:?}");
        world.send_event_batch(context_keys.into_iter().map(ScriptAttachedEvent));
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::{event::Events, world::World};

    use super::*;

    #[test]
    fn test_component_add() {
        let mut world = World::new();
        world.init_resource::<Events<ScriptAttachedEvent>>();
        // spawn new script component
        let entity = world
            .spawn(ScriptComponent::new([Handle::Weak(AssetId::invalid())]))
            .id();

        // check that the event was sent
        let mut events = world.resource_mut::<Events<ScriptAttachedEvent>>();
        assert_eq!(
            ScriptAttachment::EntityScript(entity, Handle::Weak(AssetId::invalid())),
            events.drain().next().unwrap().0
        );
    }
}
