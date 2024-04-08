//! # Motivation
//!
//! Traits and structs needed to support the creation of bindings for scripting languages.
//! reflection gives us access to `dyn Reflect` objects via their type name,
//! Scripting languages only really support `Clone` objects so if we want to support references,
//! we need wrapper types which have owned and ref variants.
use parking_lot::RwLock;
use std::{any::TypeId, cell::RefCell, collections::HashMap, marker::PhantomData, sync::Arc};

use bevy::{
    ecs::{
        change_detection::MutUntyped,
        component::{Component, ComponentId},
        entity::Entity,
        system::Resource,
        world::{unsafe_world_cell::UnsafeWorldCell, Mut, World},
    },
    ptr::Ptr,
    reflect::{
        ParsedPath, Reflect, ReflectFromPtr, ReflectPath, ReflectPathError, TypeInfo, TypeRegistry,
    },
};

use crate::{allocator::AllocationId, error::ReflectionError};

/// A wrapper for a `dyn Reflect` struct, can either be owned or a reference
pub enum Wrapper {
    Owned(Arc<dyn Reflect>, AllocationId),
    Ref(ReflectReference),
}

/// Describes kinds of base value we are accessing via reflection
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ReflectAccessKind {
    ComponentOrResource,
    Owned,
}

/// Describes the id pointing to the base value we are accessing via reflection, for components and resources this is the ComponentId
/// for script owned values this is an allocationId, this is used to ensure we have permission to access the value.
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct ReflectAccessId {
    kind: ReflectAccessKind,
    id: usize,
}

pub struct WorldAccessGuard<'w> {
    cell: UnsafeWorldCell<'w>,
    // TODO: lotta indirection here, can we make this better?
    accesses: RefCell<HashMap<usize, Arc<RwLock<WorldAccess<'w>>>>>,
}

impl<'w> WorldAccessGuard<'w> {
    /// Creates a new [`WorldAccessGuard`] for the given mutable borrow of the world
    pub fn new(world: &'w mut World) -> Self {
        Self {
            cell: world.as_unsafe_world_cell(),
            accesses: Default::default(),
        }
    }

    /// Retrieves the underlying unsafe world cell, with no additional guarantees of safety
    /// proceed with caution and only use this if you understand what you're doing
    pub fn as_unsafe_world_cell(&self) -> UnsafeWorldCell<'w> {
        self.cell
    }

    /// Retrieves the underlying read only unsafe world cell, with no additional guarantees of safety
    /// proceed with caution and only use this if you understand what you're doing
    pub fn as_unsafe_world_cell_readonly(&self) -> UnsafeWorldCell<'w> {
        self.cell
    }

    fn make_access_if_not_exists(&self, id: ReflectAccessId) {
        self.accesses
            .borrow_mut()
            .entry(id.id)
            .or_insert_with(|| Arc::new(RwLock::new(WorldAccess(id, PhantomData))));
    }

    pub fn get_access(&self, raid: ReflectAccessId) -> Arc<RwLock<WorldAccess<'w>>> {
        self.make_access_if_not_exists(raid);
        let locks = self.accesses.borrow();
        let val = locks.get(&raid.id).expect("access not present");
        val.clone()
    }

    /// Get access to the given component_id, this is the only way to access a component/resource safely (in the context of the world access guard)
    /// since you can only access this component_id through a RwLock, there is no way to break aliasing rules.
    /// Additionally the 'w lifetime prevents you from storing this access outside the lifetime of the underlying cell
    pub fn get_component_access(&self, cid: ComponentId) -> Arc<RwLock<WorldAccess<'w>>> {
        let access_id = ReflectAccessId {
            kind: ReflectAccessKind::ComponentOrResource,
            id: cid.index(),
        };
        self.get_access(access_id)
    }

    /// Get access to the given component_id, this is the only way to access a component/resource safely (in the context of the world access guard)
    /// since you can only access this component_id through a RwLock, there is no way to break aliasing rules.
    /// Additionally the 'w lifetime prevents you from storing this access outside the lifetime of the underlying cell
    pub fn get_resource_access(&self, cid: ComponentId) -> Arc<RwLock<WorldAccess<'w>>> {
        self.get_component_access(cid)
    }

    pub fn get_owned_access(&self, id: AllocationId) -> Arc<RwLock<WorldAccess<'w>>> {
        let access_id = ReflectAccessId {
            kind: ReflectAccessKind::Owned,
            id,
        };
        self.get_access(access_id)
    }

    pub fn get_component<T: Component>(
        &self,
        access: &WorldAccess,
        entity: Entity,
    ) -> Result<Option<&T>, ReflectionError> {
        let component_id = match self.cell.components().component_id::<T>() {
            Some(id) => id,
            None => return Ok(None),
        };

        if access.0
            == (ReflectAccessId {
                kind: ReflectAccessKind::ComponentOrResource,
                id: component_id.index(),
            })
        {
            // Safety: we have the correct access id
            return unsafe { Ok(self.cell.get_entity(entity).and_then(|e| e.get::<T>())) };
        } else {
            Err(ReflectionError::InsufficientAccess {
                base: format!("Component<{}>", std::any::type_name::<T>()),
                reason: format!(
                    "Invalid access, instead got permission to read: {}",
                    self.cell
                        .components()
                        .get_info(ComponentId::new(access.0.id))
                        .map(|info| info.name())
                        .unwrap_or("<Unknown Component>")
                ),
            })
        }
    }

    pub fn get_component_mut<T: Component>(
        &self,
        access: &mut WorldAccess,
        entity: Entity,
    ) -> Result<Option<Mut<T>>, ReflectionError> {
        let component_id = match self.cell.components().component_id::<T>() {
            Some(id) => id,
            None => return Ok(None),
        };

        if access.0
            == (ReflectAccessId {
                kind: ReflectAccessKind::ComponentOrResource,
                id: component_id.index(),
            })
        {
            // Safety: we have the correct access id
            return unsafe { Ok(self.cell.get_entity(entity).and_then(|e| e.get_mut::<T>())) };
        } else {
            Err(ReflectionError::InsufficientAccess {
                base: format!("Component<{}>", std::any::type_name::<T>()),
                reason: format!(
                    "Invalid access, instead got permission to read: {}",
                    self.cell
                        .components()
                        .get_info(ComponentId::new(access.0.id))
                        .map(|info| info.name())
                        .unwrap_or("<Unknown Component>")
                ),
            })
        }
    }

    pub fn get_resource<T: Resource>(
        &self,
        access: &WorldAccess,
    ) -> Result<Option<&T>, ReflectionError> {
        let resource_id = match self.cell.components().resource_id::<T>() {
            Some(id) => id,
            None => return Ok(None),
        };

        if access.0
            == (ReflectAccessId {
                kind: ReflectAccessKind::ComponentOrResource,
                id: resource_id.index(),
            })
        {
            // Safety: we have the correct access id
            return unsafe { Ok(self.cell.get_resource::<T>()) };
        } else {
            Err(ReflectionError::InsufficientAccess {
                base: format!("Resource<{}>", std::any::type_name::<T>()),
                reason: format!(
                    "Invalid access, instead got permission to read: {}",
                    self.cell
                        .components()
                        .get_info(ComponentId::new(access.0.id))
                        .map(|info| info.name())
                        .unwrap_or("<Unknown Component>")
                ),
            })
        }
    }

    pub fn get_resource_mut<T: Resource>(
        &self,
        access: &mut WorldAccess,
    ) -> Result<Option<Mut<T>>, ReflectionError> {
        let resource_id = match self.cell.components().resource_id::<T>() {
            Some(id) => id,
            None => return Ok(None),
        };

        if access.0
            == (ReflectAccessId {
                kind: ReflectAccessKind::ComponentOrResource,
                id: resource_id.index(),
            })
        {
            // Safety: we have the correct access id
            return unsafe { Ok(self.cell.get_resource_mut::<T>()) };
        } else {
            Err(ReflectionError::InsufficientAccess {
                base: format!("Resource<{}>", std::any::type_name::<T>()),
                reason: format!(
                    "Invalid access, instead got permission to read: {}",
                    self.cell
                        .components()
                        .get_info(ComponentId::new(access.0.id))
                        .map(|info| info.name())
                        .unwrap_or("<Unknown Component>")
                ),
            })
        }
    }
}

/// Having this is permission to access the contained [`ReflectAccessId`], there is no way to access anything safely through a [`WorldAccessGuard`]
/// without having a [`WorldAccess`] instance for that particular [`ReflectAccessId`].
///
/// If you do own a [`WorldAccess`] for some [`ReflectAccessId`], you can read and write to it safely.
/// If you only have an immutable borrow of [`WorldAccess`] you can only read it safely.
/// If you have a mutable borrow of [`WorldAccess`] you can read and write to it safely.
pub struct WorldAccess<'a>(pub ReflectAccessId, PhantomData<&'a usize>);

// pub struct

/// An accessor to a `dyn Reflect` struct, stores a base ID of the type and a reflection path
/// safe to build but to reflect on the value inside you need to ensure aliasing rules are upheld
#[derive(Debug)]
pub struct ReflectReference {
    pub base: ReflectBaseType,
    // TODO: experiment with Fixed capacity vec, boxed array etc, compromise between heap allocation and runtime cost
    // needs benchmarks first though
    /// The path from the top level type to the actual value we want to access
    pub reflect_path: Vec<ReflectionPathElem>,
}

// just a dummy standin for unregistered types
struct UnregisteredType;

impl ReflectReference {
    /// Retrieves a reference to the underlying `dyn Reflect` type valid for the 'w lifetime of the world cell.
    /// If the underlying componentId is not the same as the one we have access to, an error is returned.
    pub fn reflect<'w>(
        &'w self,
        world: UnsafeWorldCell<'w>,
        access: &WorldAccess<'w>,
        type_registry: &TypeRegistry,
    ) -> Result<&'w dyn Reflect, ReflectionError> {
        if access.0 == self.base.base_id.get_reflect_access_id() {
            // Safety: since we have read access to the underlying componentId we can safely access the component
            return unsafe { self.reflect_unsafe(world, type_registry) };
        }
        Err(ReflectionError::InsufficientAccess {
            base: self.base.display_with_type_name(type_registry),
            reason: format!(
                "Invalid access, instead got permission to read: {}",
                ReflectBaseType {
                    type_id: world
                        .components()
                        .get_info(ComponentId::new(access.0.id))
                        .and_then(|c| c.type_id())
                        .unwrap_or(TypeId::of::<UnregisteredType>()),
                    base_id: self.base.base_id.clone()
                }
                .display_with_type_name(type_registry),
            )
            .to_owned(),
        })
    }

    /// Retrieves a reference to the underlying `dyn Reflect` type valid for the 'w lifetime of the world cell.
    /// If the underlying componentId is not the same as the one we have access to, an error is returned.
    ///
    /// If we are accessing a component or resource, it's marked as changed
    pub fn reflect_mut<'w>(
        &'w self,
        world: UnsafeWorldCell<'w>,
        access: &mut WorldAccess<'w>,
        type_registry: &TypeRegistry,
    ) -> Result<&'w mut dyn Reflect, ReflectionError> {
        if access.0 == self.base.base_id.get_reflect_access_id() {
            // Safety: since we have write access to the underlying reflect access id we can safely access the component
            return unsafe { self.reflect_mut_unsafe(world, type_registry) };
        }

        Err(ReflectionError::InsufficientAccess {
            base: self.base.display_with_type_name(type_registry),
            reason: format!(
                "Invalid access, instead got permission to mutate: {}",
                ReflectBaseType {
                    type_id: world
                        .components()
                        .get_info(ComponentId::new(access.0.id))
                        .and_then(|c| c.type_id())
                        .unwrap_or(TypeId::of::<UnregisteredType>()),
                    base_id: self.base.base_id.clone()
                }
                .display_with_type_name(type_registry),
            )
            .to_owned(),
        })
    }

    /// Retrieves a reference to the underlying `dyn Reflect` type valid for the 'w lifetime of the world cell
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing mut references to the same value exist at all at the same time
    pub unsafe fn reflect_unsafe<'w>(
        &'w self,
        world: UnsafeWorldCell<'w>,
        type_registry: &TypeRegistry,
    ) -> Result<&'w dyn Reflect, ReflectionError> {
        if let ReflectBase::Owned(weak, _) = &self.base.base_id {
            // safety:
            return Ok(weak.as_ref());
        };
        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = type_registry
            .get_type_data(self.base.type_id)
            .expect("FromPtr is not registered for this type, cannot retrieve reflect reference");

        let ptr = self
            .base
            .base_id
            .clone()
            .into_ptr(world)
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
    pub unsafe fn reflect_mut_unsafe<'w>(
        &'w self,
        world: UnsafeWorldCell<'w>,
        type_registry: &TypeRegistry,
    ) -> Result<&'w mut dyn Reflect, ReflectionError> {
        if let ReflectBase::Owned(weak, _) = &self.base.base_id {
            // Safety: caller promises this is fine :)
            return Ok(unsafe { &mut *(Arc::as_ptr(weak) as *mut _) });
        };

        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = type_registry
            .get_type_data(self.base.type_id)
            .expect("FromPtr is not registered for this type, cannot retrieve reflect reference");

        let ptr = self
         .base
         .base_id
         .clone()
         .into_ptr_mut(world)
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

#[derive(Clone, Debug)]
pub struct ReflectBaseType {
    type_id: TypeId,
    base_id: ReflectBase,
}

impl ReflectBaseType {
    pub fn type_name(type_id: TypeId, type_registry: &TypeRegistry) -> &'static str {
        type_registry
            .get_type_info(type_id)
            .map(TypeInfo::type_path)
            .unwrap_or("<Unregistered TypeId>")
    }

    pub fn display_with_type_name(&self, type_registry: &TypeRegistry) -> String {
        format!(
            "ReflectBase({}, {:?})",
            Self::type_name(self.type_id, type_registry),
            self.base_id
        )
    }
}

/// The Id of the kind of reflection base being pointed to
#[derive(Clone, Debug)]
pub enum ReflectBase {
    Component(Entity, ComponentId),
    Resource(ComponentId),
    Owned(Arc<dyn Reflect>, AllocationId),
}

impl ReflectBase {
    /// Retrieves the pointer to the underlying `dyn Reflect` object valid for the 'w lifteime of the world cell
    ///
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing mutable references to the same value exist at the same time
    pub unsafe fn into_ptr(self, world: UnsafeWorldCell<'_>) -> Option<Ptr<'_>> {
        match self {
            ReflectBase::Component(entity, component_id) => {
                // Safety: the caller ensures invariants hold
                world.get_entity(entity)?.get_by_id(component_id)
            }
            ReflectBase::Resource(component_id) => {
                // Safety: the caller ensures invariants hold
                world.get_resource_by_id(component_id)
            }
            _ => None,
        }
    }

    /// Retrieves the pointer to the underlying `dyn Reflect` object valid for the 'w lifteime of the world cell
    ///
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing references to the same value exist at all at the same time
    pub unsafe fn into_ptr_mut(self, world: UnsafeWorldCell<'_>) -> Option<MutUntyped<'_>> {
        match self {
            ReflectBase::Component(entity, component_id) => {
                // Safety: the caller ensures invariants hold
                world.get_entity(entity)?.get_mut_by_id(component_id)
            }
            ReflectBase::Resource(component_id) => {
                // Safety: the caller ensures invariants hold
                world.get_resource_mut_by_id(component_id)
            }
            _ => None,
        }
    }

    pub fn get_reflect_access_id(&self) -> ReflectAccessId {
        match self {
            ReflectBase::Component(_, cid) | ReflectBase::Resource(cid) => ReflectAccessId {
                kind: ReflectAccessKind::ComponentOrResource,
                id: cid.index(),
            },
            ReflectBase::Owned(_, id) => ReflectAccessId {
                kind: ReflectAccessKind::Owned,
                id: *id,
            },
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

    use crate::allocator::ReflectAllocator;

    use super::*;
    use bevy::ecs::{component::Component, system::Resource, world::World};

    #[derive(Component, Reflect, PartialEq, Eq, Debug)]
    struct TestComponent {
        strings: Vec<String>,
    }

    #[derive(Resource, Reflect, Default, PartialEq, Eq, Debug)]
    struct TestResource {
        bytes: Vec<u8>,
    }

    fn setup_world() -> (World, TypeRegistry, ComponentId, ComponentId) {
        let mut world = World::default();
        let component_id = world.init_component::<TestComponent>();
        let resource_id = world.init_resource::<TestResource>();

        let mut type_registry = TypeRegistry::new();
        type_registry.register::<TestComponent>();
        type_registry.register::<TestResource>();

        (world, type_registry, component_id, resource_id)
    }

    #[test]
    fn test_component_access() {
        let (mut world, type_registry, component_id, _) = setup_world();
        let entity = world
            .spawn(TestComponent {
                strings: vec![String::from("hello")],
            })
            .id();

        let world = WorldAccessGuard::new(&mut world);

        let component_reflect_ref = ReflectReference {
            base: ReflectBaseType {
                base_id: ReflectBase::Component(entity, component_id),
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

        let component_access = world.get_component_access(component_id);

        *component_reflect_ref
            .reflect_mut(
                world.as_unsafe_world_cell(),
                &mut component_access.write(),
                &type_registry,
            )
            .unwrap()
            .downcast_mut::<String>()
            .unwrap() = "world".to_owned();

        assert_eq!(
            world
                .get_component::<TestComponent>(&component_access.read(), entity)
                .unwrap()
                .unwrap(),
            &TestComponent {
                strings: vec![String::from("world")]
            }
        );

        *world
            .get_component_mut::<TestComponent>(&mut component_access.write(), entity)
            .unwrap()
            .unwrap()
            .as_mut() = TestComponent {
            strings: vec![String::from("typed_world")],
        };

        assert_eq!(
            component_reflect_ref
                .reflect(
                    world.as_unsafe_world_cell(),
                    &component_access.read(),
                    &type_registry
                )
                .unwrap()
                .downcast_ref::<String>()
                .unwrap(),
            &"typed_world".to_owned()
        );
    }

    #[test]
    fn test_resource_access() {
        let (mut world, type_registry, _, resource_id) = setup_world();

        world.insert_resource(TestResource { bytes: vec![42u8] });
        let world = WorldAccessGuard::new(&mut world);

        let resource_reflect_ref = ReflectReference {
            base: ReflectBaseType {
                base_id: ReflectBase::Resource(resource_id),
                type_id: TypeId::of::<TestResource>(),
            },
            reflect_path: vec![
                ReflectionPathElem::Reflection(ParsedPath::parse_static(".bytes").unwrap()),
                ReflectionPathElem::DeferredReflection(DeferredReflection {
                    get: Arc::new(|root| {
                        let strings = root.downcast_ref::<Vec<u8>>().unwrap();
                        Ok(strings.first().unwrap())
                    }),
                    get_mut: Arc::new(|root| {
                        let strings = root.downcast_mut::<Vec<u8>>().unwrap();
                        Ok(strings.first_mut().unwrap())
                    }),
                }),
            ],
        };

        let resource_access = world.get_resource_access(resource_id);

        *resource_reflect_ref
            .reflect_mut(
                world.as_unsafe_world_cell(),
                &mut resource_access.write(),
                &type_registry,
            )
            .unwrap()
            .downcast_mut::<u8>()
            .unwrap() = 42u8;

        assert_eq!(
            world
                .get_resource::<TestResource>(&resource_access.read())
                .unwrap()
                .unwrap(),
            &TestResource { bytes: vec![42u8] }
        );

        *world
            .get_resource_mut::<TestResource>(&mut resource_access.write())
            .unwrap()
            .unwrap()
            .as_mut() = TestResource { bytes: vec![69u8] };

        assert_eq!(
            resource_reflect_ref
                .reflect(
                    world.as_unsafe_world_cell(),
                    &resource_access.read(),
                    &type_registry
                )
                .unwrap()
                .downcast_ref::<u8>()
                .unwrap(),
            &69u8
        );
    }

    #[test]
    fn test_script_alloc_access() {
        let (mut world, type_registry, _, _) = setup_world();

        let world = WorldAccessGuard::new(&mut world);
        let mut script_allocator = ReflectAllocator::default();
        let allocation_id = script_allocator.allocate(Arc::new("hello".to_string()));

        let owned_reflect_ref = ReflectReference {
            base: ReflectBaseType {
                base_id: ReflectBase::Owned(
                    script_allocator
                        .allocations
                        .get(&allocation_id)
                        .unwrap()
                        .clone(),
                    allocation_id,
                ),
                type_id: TypeId::of::<String>(),
            },
            reflect_path: vec![],
        };

        let owned_access = world.get_owned_access(allocation_id);

        assert_eq!(
            owned_reflect_ref
                .reflect(
                    world.as_unsafe_world_cell_readonly(),
                    &owned_access.read(),
                    &type_registry,
                )
                .unwrap()
                .downcast_ref::<String>(),
            Some(&String::from("hello"))
        );

        let onwed_access_read = owned_access.read();
        assert!(
            world.get_owned_access(allocation_id).try_write().is_none(),
            "Mutable borrow allowed while immutable borrow exists"
        );
        drop(onwed_access_read)
    }

    #[test]
    fn test_invalid_runtime_access() {
        let mut world = World::new();
        let world = WorldAccessGuard::new(&mut world);
        let access = world.get_component_access(ComponentId::new(0));
        let access2 = world.get_component_access(ComponentId::new(0));
        let access = access.read();
        assert!(
            access2.try_write().is_none(),
            "Immutable and Mutable borrow allowed at the same time"
        );
        drop(access);
    }
}
