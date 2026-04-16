//! Script related types, functions and components

use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use crate::event::{ScriptAttachedEvent, ScriptDetachedEvent};

use ::{
    bevy_asset::Handle,
    bevy_ecs::{
        entity::Entity, prelude::ReflectComponent, resource::Resource, world::DeferredWorld,
    },
    bevy_reflect::Reflect,
};

mod context_key;
mod script_context;
use bevy_ecs::{
    component::Component,
    entity::EntityHashMap,
    lifecycle::HookContext,
    message::MessageWriter,
    query::Changed,
    system::{Query, ResMut},
    world::Ref,
};
use bevy_log::trace;
use bevy_mod_scripting_asset::ScriptAsset;
use bevy_mod_scripting_script::ScriptAttachment;
pub use context_key::*;
pub use script_context::*;

/// A unique identifier for a script, by default corresponds to the path of the asset excluding the asset source.
///
/// I.e. an asset with the path `path/to/asset.ext` will have the script id `path/to/asset.ext`
pub type ScriptId = Handle<ScriptAsset>;

#[derive(Component, Reflect, Clone, Default, Debug, PartialEq, Eq)]
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

        if let Some(mut cache) = world.get_resource_mut::<ScriptComponentsChangeCache>() {
            cache.last_values.remove(&context.entity);
        }

        world.write_message_batch(context_keys.into_iter().map(ScriptDetachedEvent));
    }

    /// the lifecycle hook called when a script component is added to an entity, emits an appropriate event so we can handle
    /// the addition of the script.
    pub fn on_add(mut world: DeferredWorld, context: HookContext) {
        let context_keys = Self::get_context_keys_present(&world, context.entity);
        trace!("on add hook for script components: {context_keys:?}");

        if let Some(mut cache) = world.get_resource_mut::<ScriptComponentsChangeCache>() {
            cache.last_values.insert(
                context.entity,
                context_keys.iter().map(|x| x.script().clone()).collect(),
            );
        }

        world.write_message_batch(context_keys.into_iter().map(ScriptAttachedEvent));
    }
}

/// Cache holding the last values of script components
/// Allows the calculation of what handles have been added or removed since last frame.
/// 
/// Any handles in this cache are removed immediately when they are removed via the component
#[derive(Resource, Default)]
pub struct ScriptComponentsChangeCache {
    last_values: EntityHashMap<HashSet<Handle<ScriptAsset>>>,
}

/// A system that handles pure modifications to a [`ScriptComponent`].
///
/// Other lifecycle events, such as addition and removal of these components are handled immediately via component hooks.
pub fn script_component_changed_handler(
    mut cache: ResMut<ScriptComponentsChangeCache>,
    changed: Query<(Entity, Ref<ScriptComponent>), Changed<ScriptComponent>>,
    mut attachment_messages: MessageWriter<ScriptAttachedEvent>,
    mut detachment_messages: MessageWriter<ScriptDetachedEvent>,
) {
    for (entity, current_value) in changed {
        if let Some(last_value) = cache.last_values.get_mut(&entity) {
            let mut any_change = false;

            // check removals
            for old in last_value.iter() {
                if !current_value.0.contains(old) {
                    any_change = true;
                    detachment_messages.write(ScriptDetachedEvent(ScriptAttachment::EntityScript(
                        entity,
                        old.clone(),
                    )));
                }
            }

            // check additions
            for new in current_value.0.iter() {
                if !last_value.contains(new) {
                    any_change = true;
                    attachment_messages.write(ScriptAttachedEvent(ScriptAttachment::EntityScript(
                        entity,
                        new.clone(),
                    )));
                }
            }

            if any_change {
                last_value.clear();
                last_value.extend(current_value.0.iter().cloned());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::{message::Messages, world::World};

    use super::*;

    #[test]
    fn test_component_add() {
        let mut world = World::new();
        world.init_resource::<Messages<ScriptAttachedEvent>>();
        // spawn new script component
        let entity = world.spawn(ScriptComponent::new([Handle::default()])).id();

        // check that the event was sent
        let mut events = world.resource_mut::<Messages<ScriptAttachedEvent>>();
        assert_eq!(
            ScriptAttachment::EntityScript(entity, Handle::default()),
            events.drain().next().unwrap().0
        );
    }
}
