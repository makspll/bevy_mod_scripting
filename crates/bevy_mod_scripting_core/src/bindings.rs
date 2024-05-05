//! # Motivation
//!
//! Traits and structs needed to support the creation of bindings for scripting languages.
//! reflection gives us access to `dyn Reflect` objects via their type name,
//! Scripting languages only really support `Clone` objects so if we want to support references,
//! we need wrapper types which have owned and ref variants.
use lockable::LockableHashMap;

use std::{
    any::TypeId,
    cell::UnsafeCell,
    fmt::Debug,
    marker::PhantomData,
    sync::{Arc, Weak},
    time::Duration,
};

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
    utils::smallvec::SmallVec,
};

use crate::{
    allocator::{ReflectAllocation, ReflectAllocationId},
    error::ReflectionError,
    prelude::ReflectAllocator,
    proxy::{Proxy, Unproxy},
};

/// Describes kinds of base value we are accessing via reflection
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum ReflectAccessKind {
    ComponentOrResource,
    Allocation,
}

/// Describes the id pointing to the base value we are accessing via reflection, for components and resources this is the ComponentId
/// for script owned values this is an allocationId, this is used to ensure we have permission to access the value.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct ReflectAccessId {
    kind: ReflectAccessKind,
    id: usize,
}

impl From<ComponentId> for ReflectAccessId {
    fn from(value: ComponentId) -> Self {
        Self {
            kind: ReflectAccessKind::ComponentOrResource,
            id: value.index(),
        }
    }
}

impl ReflectAccessId {
    pub fn new_component_or_resource_id(id: ComponentId) -> Self {
        Self {
            kind: ReflectAccessKind::ComponentOrResource,
            id: id.index(),
        }
    }

    pub fn new_owned_id(id: ReflectAllocationId) -> Self {
        Self {
            kind: ReflectAccessKind::Allocation,
            id: id.id(),
        }
    }
}
/// While [`WorldAccessGuard`] prevents aliasing at runtime and also makes sure world exists at least as long as the guard itself,
/// borrows sadly do not persist the script-host boundary :(. That is to be expected, but instead we can make an abstraction which removes the lifetime parameter, making the outer type 'static,
/// while making sure the lifetime is still satisfied!
#[derive(Clone)]
pub struct WorldCallbackAccess(Weak<WorldAccessGuard<'static>>);

impl WorldCallbackAccess {
    /// Wraps a callback which requires access to the world in a 'static way via [`WorldCallbackAccess`].
    pub fn with_callback_access<T>(
        world: &mut World,
        callback: impl FnOnce(&WorldCallbackAccess) -> T,
    ) -> T {
        // - the world cannot be dropped before the world drops since we have mutable reference to it in this entire function
        // - nothing can alias inappropriately WorldAccessGuard since it's only instance is behind the raw Arc
        let world_guard = Arc::new(WorldAccessGuard::new(world));
        let world_guard = unsafe { WorldCallbackAccess::new(Arc::downgrade(&world_guard)) };

        callback(&world_guard)
    }

    /// Creates a new [`WorldCallbackAccess`] with an erased lifetime.
    ///
    /// # Safety
    /// - The caller must ensure the [`WorldAccessGuard`] must not outlive the 'w lifetime
    /// - In practice this means that between the moment the original Arc is dropped, the lifetime 'w must be valid
    /// - I.e. you *must* drop the original [`Arc<WorldAccessGuard>`] before the original 'w scope ends
    pub unsafe fn new<'w>(world: Weak<WorldAccessGuard<'w>>) -> Self {
        // Safety: the caller ensures `WorldAccessGuard` does not outlive the original lifetime 'w

        let world = unsafe {
            std::mem::transmute::<Weak<WorldAccessGuard<'w>>, Weak<WorldAccessGuard<'static>>>(
                world,
            )
        };

        Self(world)
    }

    /// Attempts to read the world access guard, if it still exists
    pub fn read(&self) -> Option<Arc<WorldAccessGuard<'static>>> {
        self.0.upgrade()
    }
}

/// Unit of world access
pub type WorldAccessUnit<'w> = WorldAccessWrite<'w>;

/// Provides safe access to the world via [`WorldAccess`] permissions, which enforce aliasing rules at runtime in multi-thread environments
#[derive(Clone)]
pub struct WorldAccessGuard<'w> {
    cell: UnsafeWorldCell<'w>,
    // TODO: this is fairly hefty, explore other ways to hand out locks on WorldAccess
    accesses: Arc<LockableHashMap<ReflectAccessId, Option<WorldAccessUnit<'w>>>>,
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

    /// Tries to get access to the given reflect access id, if it's already returns `None`. If you want to wait for access, use [`WorldAccessGuard::get_access_timeout`] instead.
    /// Remember to release this access once done with [`WorldAccessGuard::release_access`] or nobody else will be able to access this id!.
    ///
    /// Although forgetting to release access is safe, it's frankly quite rude and can lead to deadlocks.
    pub fn get_access(&self, raid: ReflectAccessId) -> Option<WorldAccessUnit<'w>> {
        let mut guard = self
            .accesses
            .blocking_lock(raid, lockable::SyncLimit::no_limit())
            .unwrap();
        let guard = guard.value_or_insert_with(|| {
            Some(WorldAccessWrite {
                raid,
                _ph: PhantomData,
            })
        });

        if guard.is_some() {
            guard.take()
        } else {
            // somebody has access to this already, we cannot access at the moment
            None
        }
    }

    /// Blocking version of [`WorldAccessGuard::get_access`], waits for access to the given reflect access id. Will busy wait at the given intervals, untill the timeout is reached.
    ///
    /// # Panic
    /// Will panic once access was not available after the timeout was reached
    pub fn get_access_timeout(
        &self,
        raid: ReflectAccessId,
        timeout: Duration,
        interval: Duration,
    ) -> WorldAccessUnit<'w> {
        let mut access = self.get_access(raid);
        let start = std::time::Instant::now();

        while access.is_none() {
            std::thread::sleep(interval);
            access = self.get_access(raid);
            if start.elapsed() > timeout {
                panic!("Timeout reached while waiting for access to {:?}", raid);
            }
        }
        access.unwrap()
    }

    /// Releases access to the given reflect access id
    pub fn release_access(&self, access: WorldAccessUnit<'w>) {
        let mut guard = self
            .accesses
            .blocking_lock(access.raid, lockable::SyncLimit::no_limit())
            .unwrap();

        let guard = guard
            .value_mut()
            .expect("Invariant violated, access should exist");

        // should not be possible, we are the only ones who can instantiate WorldAccessUnit
        debug_assert!(
            guard.is_none(),
            "Invariant violated, an access has been released by someone else already who shouldn't have been able to do so"
        );
        *guard = Some(access);
    }

    /// Get access to the given component_id, this is the only way to access a component/resource safely (in the context of the world access guard)
    /// since you can only access this component_id through a RwLock, there is no way to break aliasing rules.
    /// Additionally the 'w lifetime prevents you from storing this access outside the lifetime of the underlying cell
    pub fn get_component_access(&self, cid: ComponentId) -> Option<WorldAccessUnit<'w>> {
        let access_id = ReflectAccessId {
            kind: ReflectAccessKind::ComponentOrResource,
            id: cid.index(),
        };
        self.get_access(access_id)
    }

    /// Get access to the given component_id, this is the only way to access a component/resource safely (in the context of the world access guard)
    /// since you can only access this component_id through a RwLock, there is no way to break aliasing rules.
    /// Additionally the 'w lifetime prevents you from storing this access outside the lifetime of the underlying cell
    pub fn get_resource_access(&self, cid: ComponentId) -> Option<WorldAccessUnit<'w>> {
        self.get_component_access(cid)
    }

    /// Get access to the given allocation_id, this is the only way to access a script owned value safely (in the context of the world access guard)
    pub fn get_allocation_access(&self, id: ReflectAllocationId) -> Option<WorldAccessUnit<'w>> {
        let access_id = ReflectAccessId {
            kind: ReflectAccessKind::Allocation,
            id: id.id(),
        };
        self.get_access(access_id)
    }

    /// Call a function on a type which can be proxied, first by unproxying the input with world access,
    /// then calling the function and finally proxying the output with the allocator.
    pub fn proxy_call<'i, O: Proxy, T: Unproxy, F: Fn(T::Output<'_>) -> O::Input<'i>>(
        &self,
        mut proxied_input: T,
        f: F,
    ) -> Result<O, ReflectionError> {
        let cell = self.as_unsafe_world_cell();
        let allocator_resource_id = cell
            .components()
            .resource_id::<ReflectAllocator>()
            .expect("Reflect Allocator wasn't initialized");
        let type_registry_resource_id = cell
            .components()
            .resource_id::<bevy::ecs::reflect::AppTypeRegistry>()
            .expect("Type Registry wasn't initialized");
        let mut allocator_access = self
            .get_access(allocator_resource_id.into())
            .expect("Deadlock while accessing allocator");
        let type_registry_access = self
            .get_access(type_registry_resource_id.into())
            .expect("Deadlock while accessing type registry");
        let mut allocator = self
            .get_resource_mut::<ReflectAllocator>(&mut allocator_access)
            .unwrap()
            .unwrap();
        let type_registry = self
            .get_resource::<bevy::ecs::reflect::AppTypeRegistry>(&type_registry_access)
            .unwrap()
            .unwrap();
        let type_registry = type_registry.read();
        let mut world_acceses = SmallVec::default();

        proxied_input.collect_accesses(self, &mut world_acceses)?;
        let input = unsafe {
            proxied_input.unproxy_with_world(self, &world_acceses, &type_registry, &allocator)?
        };
        let out = f(input);

        O::proxy_with_allocator(out, &mut allocator)
    }

    /// Get access to the given component, this is the only way to access a component/resource safely (in the context of the world access guard)
    pub fn get_component<T: Component>(
        &self,
        access: &WorldAccessWrite,
        entity: Entity,
    ) -> Result<Option<&T>, ReflectionError> {
        let component_id = match self.cell.components().component_id::<T>() {
            Some(id) => id,
            None => return Ok(None),
        };

        if access.can_read(ReflectAccessId {
            kind: ReflectAccessKind::ComponentOrResource,
            id: component_id.index(),
        }) {
            // Safety: we have the correct access id
            return unsafe { Ok(self.cell.get_entity(entity).and_then(|e| e.get::<T>())) };
        } else {
            Err(ReflectionError::InsufficientAccess {
                base: format!("Component<{}>", std::any::type_name::<T>()),
                reason: format!(
                    "Invalid access, instead got permission to read: {}",
                    self.cell
                        .components()
                        .get_info(ComponentId::new(access.raid.id))
                        .map(|info| info.name())
                        .unwrap_or("<Unknown Component>")
                ),
            })
        }
    }

    /// Get access to the given component, this is the only way to access a component/resource safely (in the context of the world access guard)
    pub fn get_component_mut<T: Component>(
        &self,
        access: &mut WorldAccessWrite,
        entity: Entity,
    ) -> Result<Option<Mut<T>>, ReflectionError> {
        let component_id = match self.cell.components().component_id::<T>() {
            Some(id) => id,
            None => return Ok(None),
        };

        if access.can_write(ReflectAccessId {
            kind: ReflectAccessKind::ComponentOrResource,
            id: component_id.index(),
        }) {
            // Safety: we have the correct access id
            return unsafe { Ok(self.cell.get_entity(entity).and_then(|e| e.get_mut::<T>())) };
        } else {
            Err(ReflectionError::InsufficientAccess {
                base: format!("Component<{}>", std::any::type_name::<T>()),
                reason: format!(
                    "Invalid access, instead got permission to read: {}",
                    self.cell
                        .components()
                        .get_info(ComponentId::new(access.raid.id))
                        .map(|info| info.name())
                        .unwrap_or("<Unknown Component>")
                ),
            })
        }
    }

    /// Get access to the given resource
    pub fn get_resource<T: Resource>(
        &self,
        access: &WorldAccessWrite,
    ) -> Result<Option<&T>, ReflectionError> {
        let resource_id = match self.cell.components().resource_id::<T>() {
            Some(id) => id,
            None => return Ok(None),
        };

        if access.can_read(ReflectAccessId {
            kind: ReflectAccessKind::ComponentOrResource,
            id: resource_id.index(),
        }) {
            // Safety: we have the correct access id
            return unsafe { Ok(self.cell.get_resource::<T>()) };
        } else {
            Err(ReflectionError::InsufficientAccess {
                base: format!("Resource<{}>", std::any::type_name::<T>()),
                reason: format!(
                    "Invalid access, instead got permission to read: {}",
                    self.cell
                        .components()
                        .get_info(ComponentId::new(access.raid.id))
                        .map(|info| info.name())
                        .unwrap_or("<Unknown Component>")
                ),
            })
        }
    }

    /// Get access to the given resource, this is the only way to access a component/resource safely (in the context of the world access guard)
    pub fn get_resource_mut<T: Resource>(
        &self,
        access: &mut WorldAccessWrite,
    ) -> Result<Option<Mut<T>>, ReflectionError> {
        let resource_id = match self.cell.components().resource_id::<T>() {
            Some(id) => id,
            None => return Ok(None),
        };

        if access.can_write(ReflectAccessId {
            kind: ReflectAccessKind::ComponentOrResource,
            id: resource_id.index(),
        }) {
            // Safety: we have the correct access id
            return unsafe { Ok(self.cell.get_resource_mut::<T>()) };
        } else {
            Err(ReflectionError::InsufficientAccess {
                base: format!("Resource<{}>", std::any::type_name::<T>()),
                reason: format!(
                    "Invalid access, instead got permission to read: {}",
                    self.cell
                        .components()
                        .get_info(ComponentId::new(access.raid.id))
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
#[derive(Debug)]
pub struct WorldAccessWrite<'a> {
    pub raid: ReflectAccessId,
    pub(self) _ph: PhantomData<&'a usize>,
}

impl<'w> WorldAccessWrite<'w> {
    pub fn can_read(&self, raid: ReflectAccessId) -> bool {
        self.raid == raid
    }

    #[inline]
    pub fn can_write(&self, raid: ReflectAccessId) -> bool {
        self.can_read(raid)
    }
}

// pub struct

/// An accessor to a `dyn Reflect` struct, stores a base ID of the type and a reflection path
/// safe to build but to reflect on the value inside you need to ensure aliasing rules are upheld
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn new_allocated<T: Reflect>(
        value: T,
        allocator: &mut ReflectAllocator,
    ) -> ReflectReference {
        let id = allocator.allocate(ReflectAllocation::new(Arc::new(UnsafeCell::new(value))));
        ReflectReference {
            base: ReflectBaseType {
                type_id: TypeId::of::<T>(),
                base_id: ReflectBase::Owned(id),
            },
            reflect_path: Vec::default(),
        }
    }

    /// Returns `Ok(())` if the given access is sufficient to read the value or an appropriate error otherwise
    pub fn expect_read_access<'w>(
        &self,
        access: &WorldAccessWrite<'w>,
        type_registry: &TypeRegistry,
        world: UnsafeWorldCell<'w>,
    ) -> Result<(), ReflectionError> {
        if !access.can_read(self.base.base_id.get_reflect_access_id()) {
            Err(ReflectionError::InsufficientAccess {
                base: self.base.display_with_type_name(type_registry),
                reason: format!(
                    "Invalid access, instead got permission to read: {}",
                    ReflectBaseType {
                        type_id: world
                            .components()
                            .get_info(ComponentId::new(access.raid.id))
                            .and_then(|c| c.type_id())
                            .unwrap_or(TypeId::of::<UnregisteredType>()),
                        base_id: self.base.base_id.clone()
                    }
                    .display_with_type_name(type_registry),
                )
                .to_owned(),
            })
        } else {
            Ok(())
        }
    }

    /// Returns `Ok(())` if the given access is sufficient to write to the value or an appropriate error otherwise
    /// Note that this is not sufficient for write access, you also need to ensure the [`WorldAccessWrite`] won't be used to access the same value mutably elsewhere,
    /// if you have a `&mut WorldAccessWrite` you can guarantee this statically. This function just checks that the access itself is for the right base with write access
    pub fn expect_write_access<'w>(
        &self,
        access: &WorldAccessWrite<'w>,
        type_registry: &TypeRegistry,
        world: UnsafeWorldCell<'w>,
    ) -> Result<(), ReflectionError> {
        if !access.can_read(self.base.base_id.get_reflect_access_id()) {
            Err(ReflectionError::InsufficientAccess {
                base: self.base.display_with_type_name(type_registry),
                reason: format!(
                    "Invalid access, instead got permission to write to: {}",
                    ReflectBaseType {
                        type_id: world
                            .components()
                            .get_info(ComponentId::new(access.raid.id))
                            .and_then(|c| c.type_id())
                            .unwrap_or(TypeId::of::<UnregisteredType>()),
                        base_id: self.base.base_id.clone()
                    }
                    .display_with_type_name(type_registry),
                )
                .to_owned(),
            })
        } else {
            Ok(())
        }
    }

    /// Retrieves a reference to the underlying `dyn Reflect` type valid for the 'w lifetime of the world cell.
    /// If the underlying componentId is not the same as the one we have access to, an error is returned.
    pub fn reflect<'w, 'c>(
        &self,
        world: UnsafeWorldCell<'w>,
        access: &'c WorldAccessWrite<'w>,
        type_registry: &TypeRegistry,
        allocator: Option<&'c ReflectAllocator>,
    ) -> Result<&'c dyn Reflect, ReflectionError> {
        self.expect_read_access(access, type_registry, world)?;
        // Safety: since we have read access to the underlying componentId we can safely access the component
        // and we can return a reference tied to its lifetime, which will prevent invalid aliasing
        return unsafe { self.reflect_unsafe(world, type_registry, allocator) };
    }

    /// Retrieves a reference to the underlying `dyn Reflect` type valid for the 'w lifetime of the world cell.
    /// If the underlying componentId is not the same as the one we have access to, an error is returned.
    ///
    /// If we are accessing a component or resource, it's marked as changed
    pub fn reflect_mut<'w, 'c>(
        &self,
        world: UnsafeWorldCell<'w>,
        access: &'c mut WorldAccessWrite<'w>,
        type_registry: &TypeRegistry,
        allocator: Option<&'c ReflectAllocator>,
    ) -> Result<&'c mut dyn Reflect, ReflectionError> {
        self.expect_write_access(access, type_registry, world)?;
        // Safety: since we have write access to the underlying reflect access id we can safely access the component
        // and we can return a reference tied to its lifetime, which will prevent invalid aliasing
        return unsafe { self.reflect_mut_unsafe(world, type_registry, allocator) };
    }

    /// Retrieves a reference to the underlying `dyn Reflect` type valid for the 'w lifetime of the world cell
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing mut references to the same value exist at all at the same time
    pub unsafe fn reflect_unsafe<'w>(
        &self,
        world: UnsafeWorldCell<'w>,
        type_registry: &TypeRegistry,
        allocator: Option<&'w ReflectAllocator>,
    ) -> Result<&'w dyn Reflect, ReflectionError> {
        if let ReflectBase::Owned(id) = &self.base.base_id {
            let allocator = allocator.ok_or_else(|| ReflectionError::AllocationError {
                id: *id,
                reason: "Allocator missing".to_owned(),
            })?;
            let arc = allocator
                .get(*id)
                .ok_or_else(|| ReflectionError::AllocationError {
                    id: *id,
                    reason: "Allocation was deallocated before it was accessed".to_owned(),
                })?;

            // safety: caller promises it's fine :)
            return Ok(unsafe { &*arc.get_ptr() });
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
    /// - The caller must ensure no other references to the same value exist at all at the same time (even if you have the correct access)
    pub unsafe fn reflect_mut_unsafe<'w>(
        &self,
        world: UnsafeWorldCell<'w>,
        type_registry: &TypeRegistry,
        allocator: Option<&'w ReflectAllocator>,
    ) -> Result<&'w mut dyn Reflect, ReflectionError> {
        if let ReflectBase::Owned(id) = &self.base.base_id {
            let allocator = allocator.ok_or_else(|| ReflectionError::AllocationError {
                id: *id,
                reason: "Allocator missing".to_owned(),
            })?;
            let arc = allocator
                .get_mut(*id)
                .ok_or_else(|| ReflectionError::AllocationError {
                    id: *id,
                    reason: "Allocation was deallocated before it was accessed".to_owned(),
                })?;
            // Safety: caller promises this is fine :)
            return Ok(unsafe { &mut *arc.get_ptr() });
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReflectBaseType {
    pub type_id: TypeId,
    pub base_id: ReflectBase,
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReflectBase {
    Component(Entity, ComponentId),
    Resource(ComponentId),
    Owned(ReflectAllocationId),
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
            ReflectBase::Owned(id) => ReflectAccessId {
                kind: ReflectAccessKind::Allocation,
                id: id.id(),
            },
        }
    }
}

/// An element in the reflection path, the base reference included
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ReflectionPathElem {
    /// A standard reflection path, i.e. `.field_name[vec_index]`, pre-parsed since we construct once potentially use many times
    Reflection(ParsedPath),
    /// a deferred reflection
    DeferredReflection(DeferredReflection),
}

impl ReflectionPathElem {
    pub fn new_reflection<I: Into<ParsedPath>>(path: I) -> Self {
        Self::Reflection(path.into())
    }

    pub fn new_deferred<I: Into<DeferredReflection>>(defref: I) -> Self {
        Self::DeferredReflection(defref.into())
    }
}

impl<A: 'static, B: 'static> From<(A, B)> for DeferredReflection
where
    A: Fn(&dyn Reflect) -> Result<&dyn Reflect, ReflectPathError<'static>> + Send + Sync,
    B: Fn(&mut dyn Reflect) -> Result<&mut dyn Reflect, ReflectPathError<'static>> + Send + Sync,
{
    fn from((get, get_mut): (A, B)) -> Self {
        Self {
            get: Arc::new(get),
            get_mut: Arc::new(get_mut),
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
    pub get:
        Arc<dyn Fn(&dyn Reflect) -> Result<&dyn Reflect, ReflectPathError<'static>> + Send + Sync>,
    pub get_mut: Arc<
        dyn Fn(&mut dyn Reflect) -> Result<&mut dyn Reflect, ReflectPathError<'static>>
            + Send
            + Sync,
    >,
}

impl Debug for DeferredReflection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DeferredReflection")
    }
}

impl PartialEq for DeferredReflection {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.get, &other.get) && Arc::ptr_eq(&self.get_mut, &other.get_mut)
    }
}

impl Eq for DeferredReflection {}

#[cfg(test)]
mod test {

    use std::cell::UnsafeCell;

    use crate::allocator::{ReflectAllocation, ReflectAllocator};

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

    // #[test]
    // fn doesnt_compile() {
    //     let (mut world, type_registry, c_id, _) = setup_world();

    //     let world = WorldAccessGuard::new(&mut world);

    //     let reflect_ref = ReflectReference {
    //         base: ReflectBaseType {
    //             base_id: ReflectBase::Component(Entity::from_raw(0), c_id),
    //             type_id: TypeId::of::<TestComponent>(),
    //         },
    //         reflect_path: vec![],
    //     };

    //     let mut access = WorldAccess {
    //         raid: ReflectAccessId {
    //             kind: ReflectAccessKind::ComponentOrResource,
    //             id: c_id.index(),
    //         },
    //         write: true,
    //         _ph: PhantomData,
    //     };

    //     let read = reflect_ref
    //         .reflect(world.as_unsafe_world_cell(), &access, None, &type_registry)
    //         .unwrap();

    //     // This shouldn't compile! borrow checker should prevent this
    //     let read_mut = reflect_ref
    //         .reflect_mut(
    //             world.as_unsafe_world_cell(),
    //             &mut access,
    //             None,
    //             &type_registry,
    //         )
    //         .unwrap();

    //     drop(read);
    //     drop(read_mut);
    // }

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

        let mut component_access = world.get_component_access(component_id).unwrap();
        *component_reflect_ref
            .reflect_mut(
                world.as_unsafe_world_cell(),
                &mut component_access,
                &type_registry,
                None,
            )
            .unwrap()
            .downcast_mut::<String>()
            .unwrap() = "world".to_owned();

        assert_eq!(
            world
                .get_component::<TestComponent>(&component_access, entity)
                .unwrap()
                .unwrap(),
            &TestComponent {
                strings: vec![String::from("world")]
            }
        );

        *world
            .get_component_mut::<TestComponent>(&mut component_access, entity)
            .unwrap()
            .unwrap()
            .as_mut() = TestComponent {
            strings: vec![String::from("typed_world")],
        };

        assert_eq!(
            component_reflect_ref
                .reflect(
                    world.as_unsafe_world_cell(),
                    &component_access,
                    &type_registry,
                    None,
                )
                .unwrap()
                .downcast_ref::<String>()
                .unwrap(),
            &"typed_world".to_owned()
        );

        world.release_access(component_access);

        assert!(
            world.get_component_access(component_id).is_some(),
            "access was not release correctly"
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

        let mut resource_access = world.get_resource_access(resource_id).unwrap();

        *resource_reflect_ref
            .reflect_mut(
                world.as_unsafe_world_cell(),
                &mut resource_access,
                &type_registry,
                None,
            )
            .unwrap()
            .downcast_mut::<u8>()
            .unwrap() = 42u8;

        assert_eq!(
            world
                .get_resource::<TestResource>(&resource_access)
                .unwrap()
                .unwrap(),
            &TestResource { bytes: vec![42u8] }
        );

        *world
            .get_resource_mut::<TestResource>(&mut resource_access)
            .unwrap()
            .unwrap()
            .as_mut() = TestResource { bytes: vec![69u8] };

        assert_eq!(
            resource_reflect_ref
                .reflect(
                    world.as_unsafe_world_cell(),
                    &resource_access,
                    &type_registry,
                    None,
                )
                .unwrap()
                .downcast_ref::<u8>()
                .unwrap(),
            &69u8
        );

        world.release_access(resource_access);
        assert!(
            world.get_resource_access(resource_id).is_some(),
            "access was not release correctly"
        );
    }

    #[test]
    fn test_script_alloc_access() {
        let (mut world, type_registry, _, _) = setup_world();

        let world = WorldAccessGuard::new(&mut world);
        let mut script_allocator = ReflectAllocator::default();
        let allocation_id = script_allocator.allocate(ReflectAllocation::new(Arc::new(
            UnsafeCell::new("hello".to_string()),
        )));

        let owned_reflect_ref = ReflectReference {
            base: ReflectBaseType {
                base_id: ReflectBase::Owned(allocation_id),
                type_id: TypeId::of::<String>(),
            },
            reflect_path: vec![],
        };

        let allocation_access = world.get_allocation_access(allocation_id).unwrap();

        assert_eq!(
            owned_reflect_ref
                .reflect(
                    world.as_unsafe_world_cell_readonly(),
                    &allocation_access,
                    &type_registry,
                    Some(&script_allocator),
                )
                .unwrap()
                .downcast_ref::<String>(),
            Some(&String::from("hello"))
        );

        assert!(
            world.get_allocation_access(allocation_id).is_none(),
            "Multiple accesses to same base ID exist, safety violation"
        );

        world.release_access(allocation_access);
        assert!(
            world.get_allocation_access(allocation_id).is_some(),
            "access was not release correctly"
        );
    }

    #[test]
    #[allow(clippy::drop_non_drop)]
    fn test_invalid_runtime_access() {
        let mut world = World::new();
        let world = WorldAccessGuard::new(&mut world);
        let access = world.get_component_access(ComponentId::new(0));
        assert!(
            world.get_component_access(ComponentId::new(0)).is_none(),
            "access was allowed to alias"
        );
        drop(access);
    }

    #[test]
    #[should_panic]
    fn test_double_release_panics() {
        let mut world = World::new();
        let world = WorldAccessGuard::new(&mut world);
        let access = world.get_component_access(ComponentId::new(0)).unwrap();
        world.release_access(access);
        // This won't be possible in client code
        world.release_access(WorldAccessWrite {
            raid: ReflectAccessId {
                kind: ReflectAccessKind::ComponentOrResource,
                id: 0,
            },
            _ph: PhantomData,
        });
    }
}
