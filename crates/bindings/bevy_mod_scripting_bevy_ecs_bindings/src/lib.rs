//! Script bindings for the bevy ECS: entities, names and hierarchy.
//!
//! Most of the ECS is available through `import_namespace("bms").world`, see the core bindings.

use std::sync::Arc;

use bevy_app::{App, Plugin};
// `script_bindings` expects these in scope
use bevy_ecs::{
    entity::Entity, hierarchy::ChildOf, name::Name, reflect::AppTypeRegistry, world::World,
};
use bevy_mod_scripting_bindings::{
    InteropError, ScriptQueryBuilder, ScriptTypeRegistration, WorldExtensions,
    function::{
        from::{M, R, Union, V},
        script_function::FunctionCallContext,
    },
};
use bevy_mod_scripting_derive::script_bindings;

/// Registers the ECS script bindings.
#[derive(Default)]
pub struct BevyEcsBindingsPlugin;

impl Plugin for BevyEcsBindingsPlugin {
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_ecs"],
            "ecs",
        );
    }

    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        register_entity_functions(world);
        register_name_functions(world);
        register_child_of_functions(world);
        register_world_ecs_functions(world);
    }
}

#[script_bindings(
    remote,
    name = "entity_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "ecs",
    core
)]
impl Entity {
    /// The index of the entity. Indices of despawned entities are reused, so compare whole entities to tell them apart.
    fn index(_self: V<Entity>) -> u32 {
        _self.index_u32()
    }

    /// How many times the index of this entity has been reused.
    fn generation(_self: V<Entity>) -> u32 {
        _self.generation().to_bits()
    }

    /// The entity as a single number, which can be turned back into the entity with [`from_bits`](Self::from_bits).
    ///
    /// Only meaningful while the app is running, don't save it.
    fn to_bits(_self: V<Entity>) -> u64 {
        _self.to_bits()
    }

    /// The entity from a number returned by [`to_bits`](Self::to_bits), or nil if the number is not a valid entity.
    fn from_bits(bits: u64) -> Option<V<Entity>> {
        Entity::try_from_bits(bits).map(V)
    }

    /// Whether two entities are the same.
    fn eq(_self: V<Entity>, other: V<Entity>) -> bool {
        *_self == *other
    }
}

#[script_bindings(
    remote,
    name = "name_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "ecs",
    core
)]
impl Name {
    /// Creates a new [`Name`](bevy_ecs::name::Name) component, used to identify entities.
    ///
    /// ```multi-lang-example,lua
    /// local Name = import_namespace("ecs").Name
    /// local name = Name.new("player")
    /// assert(name:as_str() == "player")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let Name = import_namespace("ecs").Name;
    /// let name = Name.new("player");
    /// assert!(name.as_str() == "player");
    /// ```
    fn new(name: String) -> V<Name> {
        V(Name::new(name))
    }

    /// The name as a string.
    fn as_str(_self: R<Name>) -> String {
        _self.as_str().to_owned()
    }

    /// Changes the name.
    fn set(mut _self: M<Name>, name: String) {
        _self.set(name);
    }

    /// Whether the name is the same as another name or a string.
    ///
    /// In Lua, `==` only compares a name with another name, use `name:eq("text")` for strings.
    fn eq(_self: R<Name>, other: Union<R<Name>, String>) -> bool {
        match other.into_left() {
            Ok(other) => *_self == *other,
            Err(other) => _self.as_str() == other,
        }
    }
}

#[script_bindings(
    remote,
    name = "child_of_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "ecs",
    core
)]
impl ChildOf {
    /// The parent entity.
    fn parent(_self: R<ChildOf>) -> V<Entity> {
        V(_self.parent())
    }
}

#[script_bindings(
    remote,
    name = "world_ecs_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    unregistered
)]
impl World {
    /// The first entity with the given [`Name`](bevy_ecs::name::Name), or nil if there is none.
    fn find_entity_by_name(
        ctxt: FunctionCallContext,
        name: String,
    ) -> Result<Option<V<Entity>>, InteropError> {
        let world = ctxt.world()?;
        let registration = {
            let registry = world.type_registry();
            let registry = registry.read();
            registry
                .get(std::any::TypeId::of::<Name>())
                .cloned()
                .ok_or_else(|| InteropError::str("the Name component is not registered"))?
        };
        let component = world
            .get_component_type(ScriptTypeRegistration::new(Arc::new(registration)))?
            .map_err(|_| InteropError::str("Name is not registered as a component"))?;
        let mut query = ScriptQueryBuilder::new();
        query.component(component);
        for result in world.query(query)? {
            let Some(Some(name_reference)) = result.components.first() else {
                continue;
            };
            let matches = name_reference.with_reflect(world.clone(), |value| {
                value
                    .try_downcast_ref::<Name>()
                    .is_some_and(|value| value.as_str() == name)
            })?;
            if matches {
                return Ok(Some(V(result.entity)));
            }
        }
        Ok(None)
    }
}
