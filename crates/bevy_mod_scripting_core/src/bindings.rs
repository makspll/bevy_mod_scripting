//! Traits and structs needed to support the creation of bindings for scripting languages.

// reflection gives us access to `dyn Reflect` objects via their type name,
// Scripting languages only really support `Clone` objects so if we want to support references,
// we need wrapper types which have owned and ref variants.
// we need traits which let us go from &dyn Reflect to a wrapper type

use std::{
    any::TypeId,
    sync::{Arc, Mutex},
};

use bevy::{
    ecs::{
        change_detection::MutUntyped, entity::Entity, world::unsafe_world_cell::UnsafeWorldCell,
    },
    ptr::Ptr,
    reflect::{
        ParsedPath, Reflect, ReflectFromPtr, ReflectPath, ReflectPathError, TypeInfo, TypeRegistry,
    },
};

use crate::error::ReflectionError;

/// A wrapper for a `dyn Reflect` struct
pub enum Wrapper {
    Owned(Arc<Mutex<dyn Reflect>>),
    Ref(ReflectReference),
}

/// An accessor to a `dyn Reflect` struct, stores a base ID of the type and a reflection path
/// safe to build but to reflect on the value inside you need to ensure aliasing rules are upheld
#[derive(Debug)]
pub struct ReflectReference {
    pub base: ReflectBase,
    // TODO: experiment with Fixed capacity vec, boxed array etc, compromise between heap allocation and runtime cost
    // needs benchmarks first though
    /// The path from the top level type to the actual value we want to access
    pub reflect_path: Vec<ReflectionPathElem>,
}

impl ReflectReference {
    /// Retrieves a reference to the underlying `dyn Reflect` type valid for the 'w lifetime of the world cell
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing mut references to the same value exist at all at the same time
    /// - This includes the AppTypeRegistry resource if it is being borrowed from the world!
    pub unsafe fn reflect<'w>(
        &self,
        world: UnsafeWorldCell<'w>,
        type_registry: &TypeRegistry,
    ) -> Result<&'w dyn Reflect, ReflectionError> {
        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = type_registry
            .get_type_data(self.base.type_id)
            .expect("FromPtr is not registered for this type, cannot retrieve reflect reference");

        let ptr = self
            .base
            .base_id
            .get_ptr(self.base.type_id, world)
            .ok_or_else(|| ReflectionError::InvalidBaseReference {
                base: self.base.display_with_type_name(type_registry),
                reason: "Base reference is invalid, is the component/resource initialized? does the entity exist?".to_string(),
            })?;

        // (Ptr) Safety: we use the same type_id to both
        // 1) retrieve the ptr
        // 2) retrieve the ReflectFromPtr type data
        // (UnsafeWorldCell) Safety:
        // we already have access to &world so no &mut world exists
        debug_assert_eq!(
            from_ptr_data.type_id(),
            self.base.type_id,
            "Invariant violated"
        );
        let mut base = unsafe { from_ptr_data.as_reflect(ptr) };

        for elem in self.reflect_path.iter() {
            base = elem
                .reflect_element(base)
                .map_err(|e| ReflectionError::Other(e.to_string()))?;
        }

        Ok(base)
    }

    /// Retrieves mutable reference to the underlying `dyn Reflect` type valid for the 'w lifetime of the world cell
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing mut references to the same value exist at all at the same time
    /// - This includes the AppTypeRegistry resource if it is being borrowed from the world!
    pub unsafe fn reflect_mut<'w>(
        &self,
        world: UnsafeWorldCell<'w>,
        type_registry: &TypeRegistry,
    ) -> Result<&'w mut dyn Reflect, ReflectionError> {
        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = type_registry
            .get_type_data(self.base.type_id)
            .expect("FromPtr is not registered for this type, cannot retrieve reflect reference");

        let ptr = self
         .base
         .base_id
         .get_ptr_mut(self.base.type_id, world)
         .ok_or_else(|| ReflectionError::InvalidBaseReference {
             base: self.base.display_with_type_name(type_registry),
             reason: "Base reference is invalid, is the component/resource initialized? does the entity exist?".to_string(),
         })?
            .into_inner();

        // (Ptr) Safety: we use the same type_id to both
        // 1) retrieve the ptr
        // 2) retrieve the ReflectFromPtr type data
        // (UnsafeWorldCell) Safety:
        // we already have access to &world so no &mut world exists
        debug_assert_eq!(
            from_ptr_data.type_id(),
            self.base.type_id,
            "Invariant violated"
        );
        let mut base = unsafe { from_ptr_data.as_reflect_mut(ptr) };

        for elem in self.reflect_path.iter() {
            base = elem
                .reflect_element_mut(base)
                .map_err(|e| ReflectionError::Other(e.to_string()))?;
        }

        Ok(base)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ReflectBase {
    type_id: TypeId,
    base_id: ReflectId,
}

impl ReflectBase {
    pub fn type_name(&self, type_registry: &TypeRegistry) -> &'static str {
        type_registry
            .get_type_info(self.type_id)
            .map(TypeInfo::type_path)
            .unwrap_or("<Unregistered TypeId>")
    }

    pub fn display_with_type_name(&self, type_registry: &TypeRegistry) -> String {
        format!(
            "ReflectBase({}, {:?})",
            self.type_name(type_registry),
            self.base_id
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ReflectId {
    Component(Entity),
    Resource,
    // Bundle(BundleId),
}

impl ReflectId {
    /// Retrieves the pointer to the underlying `dyn Reflect` object valid for the 'w lifteime of the world cell
    ///
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing mutable references to the same value exist at the same time
    pub unsafe fn get_ptr(self, type_id: TypeId, world: UnsafeWorldCell<'_>) -> Option<Ptr<'_>> {
        match self {
            ReflectId::Component(entity) => {
                let component_id = world.components().get_id(type_id)?;
                // Safety: the caller ensures invariants hold
                world.get_entity(entity)?.get_by_id(component_id)
            }
            ReflectId::Resource => {
                let component_id = world.components().get_resource_id(type_id)?;

                // Safety: the caller ensures invariants hold
                world.get_resource_by_id(component_id)
            }
        }
    }

    /// Retrieves the pointer to the underlying `dyn Reflect` object valid for the 'w lifteime of the world cell
    ///
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing references to the same value exist at all at the same time
    pub unsafe fn get_ptr_mut(
        self,
        type_id: TypeId,
        world: UnsafeWorldCell<'_>,
    ) -> Option<MutUntyped<'_>> {
        match self {
            ReflectId::Component(entity) => {
                let component_id = world.components().get_id(type_id)?;
                // Safety: the caller ensures invariants hold
                world.get_entity(entity)?.get_mut_by_id(component_id)
            }
            ReflectId::Resource => {
                let component_id = world.components().get_id(type_id)?;

                // Safety: the caller ensures invariants hold
                world.get_resource_mut_by_id(component_id)
            }
        }
    }
}

/// An element in the reflection path, the base reference included
#[derive(Clone)]
pub enum ReflectionPathElem {
    /// A standard reflection path, i.e. `.field_name[vec_index]`, pre-parsed since we construct once potentially use many times
    Reflection(ParsedPath),
    /// a deferred reflection
    DeferredReflection(DeferredReflection),
}

impl std::fmt::Debug for ReflectionPathElem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reflection(arg0) => f.debug_tuple("Reflection").field(arg0).finish(),
            Self::DeferredReflection(_) => f.write_str("DeferredReflection"),
        }
    }
}

impl<'a> ReflectPath<'a> for &'a ReflectionPathElem {
    fn reflect_element<'r>(
        self,
        root: &'r dyn Reflect,
    ) -> Result<&'r dyn Reflect, ReflectPathError<'a>> {
        match self {
            ReflectionPathElem::Reflection(path) => path.reflect_element(root),
            ReflectionPathElem::DeferredReflection(f) => (f.get)(root),
        }
    }

    fn reflect_element_mut<'r>(
        self,
        root: &'r mut dyn Reflect,
    ) -> Result<&'r mut dyn Reflect, ReflectPathError<'a>> {
        match self {
            ReflectionPathElem::Reflection(path) => path.reflect_element_mut(root),
            ReflectionPathElem::DeferredReflection(defref) => (defref.get_mut)(root),
        }
    }
}

/// A ReflectPath which can perform arbitrary operations on the root object to produce a sub-reference
#[derive(Clone)]
pub struct DeferredReflection {
    get: Arc<dyn Fn(&dyn Reflect) -> Result<&dyn Reflect, ReflectPathError<'static>> + Send + Sync>,
    get_mut: Arc<
        dyn Fn(&mut dyn Reflect) -> Result<&mut dyn Reflect, ReflectPathError<'static>>
            + Send
            + Sync,
    >,
}

#[cfg(test)]
mod test {
    use std::sync::RwLock;

    use bevy::{
        ecs::{component::Component, reflect::AppTypeRegistry, system::Resource, world::World},
        reflect::TypeRegistryArc,
    };

    use super::*;

    #[derive(Component, Reflect)]
    struct TestComponent {
        strings: Vec<String>,
    }

    #[derive(Resource, Reflect, Default)]
    struct TestResource {
        bytes: Vec<u8>,
    }

    fn setup_world() -> (World, Arc<RwLock<TypeRegistry>>) {
        let mut world = World::default();
        world.init_component::<TestComponent>();
        world.init_resource::<TestResource>();

        let mut type_registry = TypeRegistry::new();
        type_registry.register::<TestComponent>();
        type_registry.register::<TestResource>();

        let type_registry = Arc::new(RwLock::new(type_registry));

        world.insert_resource(AppTypeRegistry(TypeRegistryArc {
            internal: type_registry.clone(),
        }));

        (world, type_registry)
    }

    #[test]
    fn test_parsed_path() {
        let (mut world, type_registry) = setup_world();
        let entity = world
            .spawn(TestComponent {
                strings: vec![String::from("hello")],
            })
            .id();

        world.insert_resource(TestResource { bytes: vec![42] });

        let component_reflect_ref = ReflectReference {
            base: ReflectBase {
                base_id: ReflectId::Component(entity),
                type_id: TypeId::of::<TestComponent>(),
            },
            reflect_path: vec![ReflectionPathElem::Reflection(
                ParsedPath::parse_static(".strings[0]").unwrap(),
            )],
        };

        let type_registry = &type_registry.read().unwrap();
        // Safety:
        // - we have unique access to world, and nothing else is accessing it
        // - we are not accessing type registry via the cell
        unsafe {
            assert_eq!(
                component_reflect_ref
                    .reflect(world.as_unsafe_world_cell_readonly(), type_registry)
                    .unwrap()
                    .downcast_ref::<String>(),
                Some(&String::from("hello"))
            );
        }

        let resource_reflect_ref = ReflectReference {
            base: ReflectBase {
                base_id: ReflectId::Resource,
                type_id: TypeId::of::<TestResource>(),
            },
            reflect_path: vec![ReflectionPathElem::Reflection(
                ParsedPath::parse_static(".bytes[0]").unwrap(),
            )],
        };

        // Safety:
        // - we have unique access to world, and nothing else is accessing it
        // - we are not accessing type registry via the cell
        unsafe {
            assert_eq!(
                resource_reflect_ref
                    .reflect(world.as_unsafe_world_cell(), type_registry)
                    .unwrap()
                    .downcast_ref(),
                Some(&42u8)
            );
        }
    }

    #[test]
    fn test_parsed_and_deferred_path() {
        let (mut world, type_registry) = setup_world();
        let entity = world
            .spawn(TestComponent {
                strings: vec![String::from("hello")],
            })
            .id();

        world.insert_resource(TestResource { bytes: vec![42] });

        let component_reflect_ref = ReflectReference {
            base: ReflectBase {
                base_id: ReflectId::Component(entity),
                type_id: TypeId::of::<TestComponent>(),
            },
            reflect_path: vec![
                ReflectionPathElem::Reflection(ParsedPath::parse_static(".strings").unwrap()),
                ReflectionPathElem::DeferredReflection(DeferredReflection {
                    get: Arc::new(|root| {
                        let strings = root.downcast_ref::<Vec<String>>().unwrap();
                        Ok(strings.first().unwrap())
                    }),
                    get_mut: Arc::new(|root| {
                        let strings = root.downcast_mut::<Vec<String>>().unwrap();
                        Ok(strings.first_mut().unwrap())
                    }),
                }),
            ],
        };

        let type_registry = &type_registry.read().unwrap();
        // Safety:
        // - we have unique access to world, and nothing else is accessing it
        // - we are not accessing type registry via the cell
        // - we drop the mutable access before the immutable access
        unsafe {
            *component_reflect_ref
                .reflect_mut(world.as_unsafe_world_cell(), type_registry)
                .unwrap()
                .downcast_mut::<String>()
                .unwrap() = String::from("world");

            assert_eq!(
                component_reflect_ref
                    .reflect(world.as_unsafe_world_cell_readonly(), type_registry)
                    .unwrap()
                    .downcast_ref::<String>(),
                Some(&String::from("world"))
            );
        }
    }
}
