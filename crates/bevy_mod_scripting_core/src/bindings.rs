//! Traits and structs needed to support the creation of bindings for scripting languages.

// reflection gives us access to `dyn Reflect` objects via their type name,
// Scripting languages only really support `Clone` objects so if we want to support references,
// we need wrapper types which have owned and ref variants.
// we need traits which let us go from &dyn Reflect to a wrapper type

use std::{
    any::{Any, TypeId},
    sync::{Arc, Mutex},
};

use bevy::{
    ecs::{component::ComponentId, entity::Entity, world::World},
    ptr::Ptr,
    reflect::{ParsedPath, Reflect, ReflectFromPtr, ReflectPath, ReflectPathError, TypeRegistry},
};

use crate::error::ReflectionError;

/// A wrapper for a `dyn Reflect` struct
pub enum Wrapper {
    Owned(Arc<Mutex<dyn Reflect>>),
    Ref(ReflectReference),
}

/// An accessor to a `dyn Reflect` struct, stores a base ID of the type and a reflection path
pub struct ReflectReference {
    base: ReflectId,
    // TODO: experiment with Fixed capacity vec, boxed array etc, compromise between heap allocation and runtime cost
    // needs benchmarks first though
    /// The path from the top level type to the actual value we want to access
    reflect_path: Vec<ReflectionPathElem<'static>>,
}

impl ReflectReference {
    /// Retrieves the underlying `dyn Reflect` object given a world
    /// - If the reference is to a component and the entity no longer exists, returns `None`
    /// - If the resource or component ID's are invalid will panic
    /// - In the future if the underlying reference is NOT to a Component or Resource ReflectId this will return None, i.e. for Bundles
    pub fn reflect<'w>(
        &self,
        world: &'w World,
        type_registry: &TypeRegistry,
    ) -> Result<&'w dyn Reflect, ReflectionError> {
        let type_id = self.base.type_id(world);

        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = type_registry
            .get_type_data(type_id)
            .expect("FromPtr is not registered for this type, cannot retrieve reflect reference");

        let ptr = self.base.get_ptr(world)?;

        // Safety: we use the same type_id to both
        // 1) retrieve the ptr
        // 2) retrieve the ReflectFromPtr type data
        // so the as_reflect implementation should match and this is sound
        debug_assert_eq!(from_ptr_data.type_id(), type_id, "Invariant violated");
        let mut base = unsafe { from_ptr_data.as_reflect(ptr) };

        for elem in self.reflect_path.iter().skip(1) {
            base = elem
                .reflect_element(base)
                .map_err(|e| ReflectionError::Other(e.to_string()))?;
        }

        Ok(base)
    }

    pub fn reflect_mut<'w>(
        &self,
        world: &'w mut World,
        type_registry: &TypeRegistry,
    ) -> Result<&'w mut dyn Reflect, ReflectionError> {
        let type_id = self.base.type_id(world);

        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = type_registry
            .get_type_data(type_id)
            .expect("FromPtr is not registered for this type, cannot retrieve reflect reference");

        let ptr = self.base.get_ptr(world)?;

        // Safety: we use the same type_id to both
        // 1) retrieve the ptr
        // 2) retrieve the ReflectFromPtr type data
        // so the as_reflect implementation should match and this is sound
        debug_assert_eq!(from_ptr_data.type_id(), type_id, "Invariant violated");
        let mut base = unsafe { from_ptr_data.as_reflect_mut(ptr) };

        for elem in self.reflect_path.iter().skip(1) {
            base = elem
                .reflect_mut(base)
                .map_err(|e| ReflectionError::Other(e.to_string()))?;
        }

        Ok(base)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ReflectId {
    Component(ComponentId, Entity),
    Resource(ComponentId),
    // Bundle(BundleId),
}

impl ReflectId {
    pub fn type_id(self, world: &World) -> TypeId {
        match self {
            ReflectId::Component(id, _) | ReflectId::Resource(id) => world
                .components()
                .get_info(id)
                .expect("Invalid component id")
                .type_id()
                .expect("Expected rust type"),
        }
    }

    pub fn get_ptr(self, world: &World) -> Result<Ptr, ReflectionError> {
        match self {
            ReflectId::Component(id, entity) => {
                world
                    .get_by_id(entity, id)
                    .ok_or_else(|| ReflectionError::InvalidBaseReference {
                        base: format!("{:?}", self),
                        reason: "Entity no longer exists, or componentId is no longer valid"
                            .to_string(),
                    })
            }
            ReflectId::Resource(id) => {
                world
                    .get_resource_by_id(id)
                    .ok_or_else(|| ReflectionError::InvalidBaseReference {
                        base: format!("{:?}", self),
                        reason: "Resource no longer exists, or componentId is no longer valid"
                            .to_string(),
                    })
            }
        }
    }
}

/// An element in the reflection path, the base reference included
#[derive(Clone)]
pub enum ReflectionPathElem<'a> {
    /// A standard reflection path, i.e. `.field_name[vec_index]`, pre-parsed since we construct once potentially use many times
    Reflection(ParsedPath),
    /// a deferred reflection
    DeferredReflection(DeferredReflection<'a>),
}

impl std::fmt::Debug for ReflectionPathElem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reflection(arg0) => f.debug_tuple("Reflection").field(arg0).finish(),
            Self::DeferredReflection(_) => f.write_str("DeferredReflection"),
        }
    }
}

impl<'a> ReflectPath<'a> for ReflectionPathElem<'a> {
    fn reflect_element(self, root: &dyn Reflect) -> Result<&dyn Reflect, ReflectPathError<'a>> {
        match self {
            ReflectionPathElem::Reflection(path) => path.reflect_element(root),
            ReflectionPathElem::DeferredReflection(f) => (f.get)(root),
        }
    }

    fn reflect_element_mut(
        self,
        root: &mut dyn Reflect,
    ) -> Result<&mut dyn Reflect, ReflectPathError<'a>> {
        match self {
            ReflectionPathElem::Reflection(path) => path.reflect_element_mut(root),
            ReflectionPathElem::DeferredReflection(defref) => (defref.get_mut)(root),
        }
    }
}

/// A ReflectPath which can perform arbitrary operations on the root object to produce a sub-reference
#[derive(Clone)]
pub struct DeferredReflection<'a> {
    get: Arc<dyn Fn(&dyn Reflect) -> Result<&dyn Reflect, ReflectPathError<'a>> + Send + Sync>,
    get_mut: Arc<
        dyn Fn(&mut dyn Reflect) -> Result<&mut dyn Reflect, ReflectPathError<'a>> + Send + Sync,
    >,
}

#[cfg(test)]
mod test {
    use bevy::ecs::{component::Component, system::Resource};

    use super::*;

    #[derive(Component, Reflect)]
    struct TestComponent {
        strings: Vec<String>,
    }

    #[derive(Resource, Reflect, Default)]
    struct TestResource {
        bytes: Vec<u8>,
    }

    #[test]
    fn test_reflect_reference() {
        let mut world = World::default();
        let component_id = world.init_component::<TestComponent>();
        let resource_id = world.init_resource::<TestResource>();

        let type_registry = TypeRegistry::new();

        let entity = world
            .spawn(TestComponent {
                strings: vec![String::from("hello")],
            })
            .id();

        world.insert_resource(TestResource { bytes: vec![42] });

        let component_reflect_ref = ReflectReference {
            base: ReflectId::Component(component_id, entity),
            reflect_path: vec![ReflectionPathElem::Reflection(
                ParsedPath::parse_static(".strings[0]").unwrap(),
            )],
        };

        assert_eq!(
            component_reflect_ref
                .reflect(&world, &type_registry)
                .unwrap()
                .downcast_ref(),
            Some(&String::from("hello"))
        );

        let resource_reflect_ref = ReflectReference {
            base: ReflectId::Resource(resource_id),
            reflect_path: vec![ReflectionPathElem::Reflection(
                ParsedPath::parse_static(".bytes[0]").unwrap(),
            )],
        };

        assert_eq!(
            resource_reflect_ref
                .reflect(&world, &type_registry)
                .unwrap()
                .downcast_ref(),
            Some(&42)
        );
    }
}
