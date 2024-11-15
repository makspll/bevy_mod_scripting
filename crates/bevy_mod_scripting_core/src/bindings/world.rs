//! # Motivation
//!
//! Traits and structs needed to support the creation of bindings for scripting languages.
//! reflection gives us access to `dyn PartialReflect` objects via their type name,
//! Scripting languages only really support `Clone` objects so if we want to support references,
//! we need wrapper types which have owned and ref variants.
use lockable::{LockableHashMap, SyncLimit};

use std::{
    any::TypeId,
    fmt::Debug,
    marker::PhantomData,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Weak,
    },
    time::Duration,
};

use bevy::{
    ecs::{
        component::{Component, ComponentId},
        entity::Entity,
        reflect::{AppTypeRegistry, ReflectComponent, ReflectFromWorld, ReflectResource},
        system::{Commands, Resource},
        world::{unsafe_world_cell::UnsafeWorldCell, CommandQueue, Mut, World},
    },
    hierarchy::{BuildChildren, Children, DespawnRecursiveExt, Parent},
    reflect::{std_traits::ReflectDefault, TypeRegistry},
};

use smallvec::SmallVec;

use crate::{
    bindings::ReflectAllocationId,
    prelude::{ReflectAllocator, ScriptError, ScriptResult},
};

use super::{
    proxy::{Proxy, Unproxy},
    ReflectBase, ReflectBaseType, ReflectReference, ScriptTypeRegistration,
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

impl From<ReflectAllocationId> for ReflectAccessId {
    fn from(value: ReflectAllocationId) -> Self {
        Self {
            kind: ReflectAccessKind::Allocation,
            id: value.id(),
        }
    }
}

/// While [`WorldAccessGuard`] prevents aliasing at runtime and also makes sure world exists at least as long as the guard itself,
/// borrows sadly do not persist the script-host boundary :(. That is to be expected, but instead we can make an abstraction which removes the lifetime parameter, making the outer type 'static,
/// while making sure the lifetime is still satisfied!
#[derive(Clone, Debug)]
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

pub(crate) const STALE_WORLD_MSG: &str = "Tried to access world via stale reference";
pub(crate) const CONCURRENT_WORLD_ACCESS_MSG: &str =
    "Something else is accessing the world right now!";
pub(crate) const CONCURRENT_ACCESS_MSG: &str =
    "Something else is accessing the resource/component/allocation right now!";

/// common world methods, see:
/// - [`crate::bindings::query`] for query related functionality
impl WorldCallbackAccess {
    pub fn spawn(&self) -> Entity {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.spawn()
    }

    // TODO: uses `String` for type_name to avoid lifetime issues with types proxying this via macros
    pub fn get_type_by_name(&self, type_name: String) -> Option<ScriptTypeRegistration> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.get_type_by_name(type_name)
    }

    pub fn add_default_component(
        &self,
        entity: Entity,
        registration: ScriptTypeRegistration,
    ) -> ScriptResult<()> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.add_default_component(entity, registration)
    }

    pub fn get_component(
        &self,
        entity: Entity,
        component_id: ComponentId,
    ) -> ScriptResult<Option<ReflectReference>> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.get_component(entity, component_id)
    }

    pub fn has_component(&self, entity: Entity, component_id: ComponentId) -> ScriptResult<bool> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.has_component(entity, component_id)
    }

    pub fn remove_component(
        &self,
        entity: Entity,
        registration: ScriptTypeRegistration,
    ) -> ScriptResult<()> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.remove_component(entity, registration)
    }

    pub fn get_resource(&self, resource_id: ComponentId) -> ScriptResult<Option<ReflectReference>> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.get_resource(resource_id)
    }

    pub fn remove_resource(&self, registration: ScriptTypeRegistration) -> ScriptResult<()> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.remove_resource(registration)
    }

    pub fn has_resource(&self, resource_id: ComponentId) -> bool {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.has_resource(resource_id)
    }

    pub fn has_entity(&self, entity: Entity) -> bool {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.has_entity(entity)
    }

    pub fn get_children(&self, entity: Entity) -> ScriptResult<Vec<Entity>> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.get_children(entity)
    }

    pub fn get_parent(&self, entity: Entity) -> ScriptResult<Option<Entity>> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.get_parent(entity)
    }

    pub fn push_children(&self, parent: Entity, children: &[Entity]) -> ScriptResult<()> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.push_children(parent, children)
    }

    pub fn remove_children(&self, parent: Entity, children: &[Entity]) -> ScriptResult<()> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.remove_children(parent, children)
    }

    pub fn insert_children(
        &self,
        parent: Entity,
        index: usize,
        children: &[Entity],
    ) -> ScriptResult<()> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.insert_children(parent, index, children)
    }

    pub fn despawn_recursive(&self, entity: Entity) -> ScriptResult<()> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.despawn_recursive(entity)
    }

    pub fn despawn(&self, entity: Entity) -> ScriptResult<()> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.despawn(entity)
    }

    pub fn despawn_descendants(&self, entity: Entity) -> ScriptResult<()> {
        let world = self.read().unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"));
        world.despawn_descendants(entity)
    }
}

/// Unit of world access
pub type WorldAccessUnit<'w> = WorldAccessWrite<'w>;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);
pub const DEFAULT_INTERVAL: Duration = Duration::from_millis(10);

/// Provides safe access to the world via [`WorldAccess`] permissions, which enforce aliasing rules at runtime in multi-thread environments
#[derive(Clone)]
pub struct WorldAccessGuard<'w> {
    cell: UnsafeWorldCell<'w>,
    // TODO: this is fairly hefty, explore other ways to hand out locks on WorldAccess
    accesses: Arc<LockableHashMap<ReflectAccessId, Option<WorldAccessUnit<'w>>>>,
    /// true if anybody has any access to the world
    accesses_count: Arc<AtomicUsize>,
    // TODO can we track code/stack locations of things holding onto theese locks for debugging?
}

impl<'w> WorldAccessGuard<'w> {
    /// Creates a new [`WorldAccessGuard`] for the given mutable borrow of the world
    pub fn new(world: &'w mut World) -> Self {
        Self {
            cell: world.as_unsafe_world_cell(),
            accesses: Default::default(),
            accesses_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Purely debugging utility to list all accesses currently held.
    pub fn list_accesses(&self) -> Vec<ReflectAccessId> {
        let keys = self.accesses.keys_with_entries_or_locked();

        // return only those which have no value
        keys.into_iter()
            .filter(|k| {
                let val = self
                    .accesses
                    .blocking_lock(*k, SyncLimit::no_limit())
                    .unwrap();
                let val = val.value().unwrap();

                val.is_none()
            })
            .collect()
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

    /// Gets the component id of the given component or resource
    pub fn get_component_id(&self, id: TypeId) -> Option<ComponentId> {
        self.cell.components().get_id(id)
    }

    pub fn get_resource_id(&self, id: TypeId) -> Option<ComponentId> {
        self.cell.components().get_resource_id(id)
    }

    /// Checks nobody else is currently accessing the world, and if so locks access to it until
    /// [`release_whole_world_access`] is called.
    pub fn get_whole_world_access(&self) -> Option<&mut World> {
        if self.accesses_count.load(Ordering::Relaxed) == 0 {
            Some(unsafe { self.cell.world_mut() })
        } else {
            None
        }
    }

    /// Releases whole world access. Allowing others to access it.
    pub fn release_whole_world_access(&self, _world: &mut World) {
        // we do not need ot use the world reference, it's there as proof that the caller has claimed access before
        assert_eq!(self.accesses_count.load(Ordering::Relaxed), 1);
        self.accesses_count.fetch_sub(1, Ordering::Relaxed);
    }

    /// Tries to get access to the given reflect access id, if it's already given out returns `None`. If you want to wait for access, use [`WorldAccessGuard::get_access_timeout`] instead.
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
            self.accesses_count.fetch_add(1, Ordering::Relaxed);
            guard.take()
        } else {
            // somebody has access to this already, we cannot access at the moment
            None
        }
    }

    /// Blocking version of [`WorldAccessGuard::get_access`], waits for access to the given reflect access id. Will busy wait at the given intervals, untill the timeout is reached.
    /// If interval is zero this is equivalent to busy waiting.
    ///
    /// # Panic
    /// Will panic once access was not available after the timeout was reached
    pub fn get_access_timeout(
        &self,
        raid: ReflectAccessId,
        timeout: Duration,
        interval: Duration,
    ) -> Option<WorldAccessUnit<'w>> {
        let mut access = self.get_access(raid);
        let start = std::time::Instant::now();

        while access.is_none() {
            std::thread::sleep(interval);
            access = self.get_access(raid);
            if start.elapsed() > timeout {
                return None;
            }
        }
        access
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
        assert!(
            guard.is_none(),
            "Invariant violated, an access has been released by someone else already who shouldn't have been able to do so"
        );

        self.accesses_count.fetch_sub(1, Ordering::Relaxed);
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

    /// Similar to [`Self::get_component_access`] but typed, additionally panics if the component is not registered
    pub fn get_component_access_typed<T: Component>(&self) -> Option<WorldAccessUnit<'w>> {
        self.get_component_access(
            self.cell
                .components()
                .component_id::<T>()
                .unwrap_or_else(|| {
                    panic!("Component not registered: `{}`", std::any::type_name::<T>())
                }),
        )
    }

    /// Get access to the given component_id, this is the only way to access a component/resource safely (in the context of the world access guard)
    /// since you can only access this component_id through a RwLock, there is no way to break aliasing rules.
    /// Additionally the 'w lifetime prevents you from storing this access outside the lifetime of the underlying cell
    pub fn get_resource_access(&self, cid: ComponentId) -> Option<WorldAccessUnit<'w>> {
        self.get_component_access(cid)
    }

    /// Similar to [`Self::get_resource_access`] but typed, additionally panics if the resource is not registered
    pub fn get_resource_access_typed<T: Resource>(&self) -> Option<WorldAccessUnit<'w>> {
        self.get_resource_access(
            self.cell
                .components()
                .resource_id::<T>()
                .unwrap_or_else(|| {
                    panic!("Resource not registered: `{}`", std::any::type_name::<T>())
                }),
        )
    }

    /// Get access to the given allocation_id, this is the only way to access a script owned value safely (in the context of the world access guard)
    pub fn get_allocation_access(&self, id: ReflectAllocationId) -> Option<WorldAccessUnit<'w>> {
        let access_id = ReflectAccessId {
            kind: ReflectAccessKind::Allocation,
            id: id.id(),
        };
        self.get_access(access_id)
    }

    /// Provides access to a resource via callback. Panics if the resource does not exist or if waiting for access times out.
    pub fn with_resource<R: Resource, O, F: FnOnce(&Self, Mut<R>) -> O>(&self, f: F) -> O {
        let cid = self
            .cell
            .components()
            .resource_id::<R>()
            .unwrap_or_else(|| panic!("Resource not registered: `{}`", std::any::type_name::<R>()));

        let mut access = self
            .get_access_timeout(cid.into(), DEFAULT_TIMEOUT, DEFAULT_INTERVAL)
            .unwrap_or_else(|| {
                panic!(
                    "Timed out while waiting for access to resource: `{}`",
                    std::any::type_name::<R>()
                )
            });

        let resource = self
            .get_resource_with_access_mut::<R>(&mut access)
            .expect("invariant")
            .expect("invariant");
        let out = f(self, resource);
        self.release_access(access);
        out
    }

    /// Call a function on a type which can be proxied, first by unproxying the input with world access,
    /// then calling the function and finally proxying the output with the allocator.
    pub fn proxy_call<'i, O: Proxy, T: Unproxy, F: Fn(T::Output<'_>) -> O::Input<'i>>(
        &self,
        mut proxied_input: T,
        f: F,
    ) -> ScriptResult<O> {
        let type_registry =
            self.with_resource(|_, type_registry: Mut<AppTypeRegistry>| type_registry.clone());

        let type_registry = type_registry.read();
        let mut world_acceses = SmallVec::default();

        let unproxied_input = self.with_resource(|_, allocator: Mut<ReflectAllocator>| {
            proxied_input.collect_accesses(self, &mut world_acceses)?;
            unsafe {
                proxied_input.unproxy_with_world(self, &world_acceses, &type_registry, &allocator)
            }
        })?;

        let out = f(unproxied_input);

        let proxied_output = self.with_resource(|_, mut allocator: Mut<ReflectAllocator>| {
            O::proxy_with_allocator(out, &mut allocator)
        })?;

        // make sure to release all accesses
        world_acceses.drain(..).for_each(|a| {
            self.release_access(a);
        });

        Ok(proxied_output)
    }

    /// Get access to the given component, this is the only way to access a component/resource safely (in the context of the world access guard)
    pub fn get_component_with_access<T: Component>(
        &self,
        access: &WorldAccessWrite,
        entity: Entity,
    ) -> ScriptResult<Option<&T>> {
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
            Err(ScriptError::new_reflection_error(
                "Cannot read component, received invalid access".to_string(),
            ))
        }
    }

    /// Get access to the given component, this is the only way to access a component/resource safely (in the context of the world access guard)
    pub fn get_component_with_access_mut<T: Component>(
        &self,
        access: &mut WorldAccessWrite,
        entity: Entity,
    ) -> ScriptResult<Option<Mut<T>>> {
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
            Err(ScriptError::new_reflection_error(
                "Cannot write component, received invalid access".to_string(),
            ))
        }
    }

    /// Get access to the given resource
    pub fn get_resource_with_access<T: Resource>(
        &self,
        access: &WorldAccessWrite,
    ) -> ScriptResult<Option<&T>> {
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
            Err(ScriptError::new_reflection_error(
                "Cannot read resource, received invalid access".to_string(),
            ))
        }
    }

    /// Get access to the given resource, this is the only way to access a component/resource safely (in the context of the world access guard)
    pub fn get_resource_with_access_mut<T: Resource>(
        &self,
        access: &mut WorldAccessWrite,
    ) -> ScriptResult<Option<Mut<T>>> {
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
            Err(ScriptError::new_reflection_error(
                "Cannot write resource, received invalid access".to_string(),
            ))
        }
    }

    /// checks if a given entity exists and is valid
    pub fn is_valid_entity(&self, entity: Entity) -> bool {
        self.cell.get_entity(entity).is_some()
    }
}

/// Impl block for higher level world methods
impl<'w> WorldAccessGuard<'w> {
    pub fn spawn(&self) -> Entity {
        if let Some(world) = self.get_whole_world_access() {
            world.spawn_empty().id()
        } else {
            panic!("{CONCURRENT_WORLD_ACCESS_MSG}")
        }
    }

    pub fn get_type_by_name(&self, type_name: String) -> Option<ScriptTypeRegistration> {
        self.with_resource(|world, registry: Mut<AppTypeRegistry>| {
            let registry = registry.read();
            registry
                .get_with_short_type_path(&type_name)
                .or_else(|| registry.get_with_type_path(&type_name))
                .map(|registration| {
                    ScriptTypeRegistration::new(
                        Arc::new(registration.clone()),
                        world.get_component_id(registration.type_id()),
                        world.get_resource_id(registration.type_id()),
                    )
                })
        })
    }

    pub fn add_default_component(
        &self,
        entity: Entity,
        registration: ScriptTypeRegistration,
    ) -> ScriptResult<()> {
        let component_data = registration.registration.data::<ReflectComponent>().ok_or_else(|| ScriptError::new_runtime_error(format!(
            "Cannot add default component since type: `{}`, Does not have ReflectComponent data registered.",
            registration.registration.type_info().type_path()
        )))?;

        // we look for ReflectDefault or ReflectFromWorld data then a ReflectComponent data
        let instance = if let Some(default_td) = registration.registration.data::<ReflectDefault>()
        {
            default_td.default()
        } else if let Some(from_world_td) = registration.registration.data::<ReflectFromWorld>() {
            if let Some(world) = self.get_whole_world_access() {
                from_world_td.from_world(world)
            } else {
                panic!("{CONCURRENT_WORLD_ACCESS_MSG}")
            }
        } else {
            return Err(ScriptError::new_runtime_error(format!(
                "Cannot add default component since type: `{}`, Does not have ReflectDefault or ReflectFromWorld data registered.",
                registration.registration.type_info().type_path()
            )));
        };

        //  TODO: this shouldn't need entire world access it feels
        if let Some(world) = self.get_whole_world_access() {
            let app_registry = world
                .remove_resource::<AppTypeRegistry>()
                .unwrap_or_else(|| panic!("Missing type registry"));

            let mut entity = world.get_entity_mut(entity).map_err(|e| {
                ScriptError::new_runtime_error(format!(
                    "Could not retrieve entity: {:?}. {e}",
                    entity
                ))
            })?;
            {
                let registry = app_registry.read();
                component_data.insert(&mut entity, instance.as_partial_reflect(), &registry);
            }
            world.insert_resource(app_registry);
        } else {
            panic!("{CONCURRENT_WORLD_ACCESS_MSG}")
        }

        Ok(())
    }
    pub fn get_component(
        &self,
        entity: Entity,
        component_id: ComponentId,
    ) -> ScriptResult<Option<ReflectReference>> {
        let entity = self.cell.get_entity(entity).ok_or_else(|| {
            ScriptError::new_runtime_error(format!("Entity does not exist: {:?}", entity))
        })?;

        let component_info = self
            .cell
            .components()
            .get_info(component_id)
            .ok_or_else(|| {
                ScriptError::new_runtime_error(format!(
                    "Component does not exist: {:?}",
                    component_id
                ))
            })?;

        if entity.contains_id(component_id) {
            Ok(Some(ReflectReference {
                base: ReflectBaseType {
                    type_id: component_info
                        .type_id()
                        .expect("Component does not have type id"),
                    base_id: ReflectBase::Component(entity.id(), component_id),
                },
                reflect_path: Default::default(),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn has_component(&self, entity: Entity, component_id: ComponentId) -> ScriptResult<bool> {
        let entity = self.cell.get_entity(entity).ok_or_else(|| {
            ScriptError::new_runtime_error(format!("Entity does not exist: {:?}", entity))
        })?;

        Ok(entity.contains_id(component_id))
    }

    pub fn remove_component(
        &self,
        entity: Entity,
        registration: ScriptTypeRegistration,
    ) -> ScriptResult<()> {
        let component_data = registration.registration.data::<ReflectComponent>().ok_or_else(|| ScriptError::new_runtime_error(format!(
            "Cannot remove component since type: `{}`, Does not have ReflectComponent data registered.",
            registration.registration.type_info().type_path()
        )))?;

        //  TODO: this shouldn't need entire world access it feels
        if let Some(world) = self.get_whole_world_access() {
            let mut entity = world.get_entity_mut(entity).map_err(|e| {
                ScriptError::new_runtime_error(format!(
                    "Could not retrieve entity: {:?}. {e}",
                    entity
                ))
            })?;

            component_data.remove(&mut entity);
        } else {
            panic!("{CONCURRENT_WORLD_ACCESS_MSG}")
        }
        Ok(())
    }

    pub fn get_resource(&self, resource_id: ComponentId) -> ScriptResult<Option<ReflectReference>> {
        let component_info = match self.cell.components().get_info(resource_id) {
            Some(info) => info,
            None => return Ok(None),
        };

        Ok(Some(ReflectReference {
            base: ReflectBaseType {
                type_id: component_info
                    .type_id()
                    .expect("Resource does not have type id"),
                base_id: ReflectBase::Resource(resource_id),
            },
            reflect_path: Default::default(),
        }))
    }

    pub fn remove_resource(&self, registration: ScriptTypeRegistration) -> ScriptResult<()> {
        let component_data = registration.registration.data::<ReflectResource>().ok_or_else(|| ScriptError::new_runtime_error(format!(
            "Cannot remove resource since type: `{}`, Does not have ReflectResource data registered.",
            registration.registration.type_info().type_path()
        )))?;

        //  TODO: this shouldn't need entire world access it feels
        if let Some(world) = self.get_whole_world_access() {
            component_data.remove(world);
        } else {
            panic!("{CONCURRENT_WORLD_ACCESS_MSG}")
        }
        Ok(())
    }

    pub fn has_resource(&self, resource_id: ComponentId) -> bool {
        // Safety: we are not reading the value at all
        let res_ptr = unsafe { self.cell.get_resource_by_id(resource_id) };
        res_ptr.is_some()
    }

    pub fn has_entity(&self, entity: Entity) -> bool {
        self.is_valid_entity(entity)
    }

    pub fn get_children(&self, entity: Entity) -> ScriptResult<Vec<Entity>> {
        if !self.is_valid_entity(entity) {
            return Err(ScriptError::new_runtime_error(format!(
                "Entity does not exist or is not valid: {:?}",
                entity
            )));
        }

        let access = self
            .get_component_access_typed::<Children>()
            .unwrap_or_else(|| panic!("{CONCURRENT_ACCESS_MSG}"));

        Ok(self
            .get_component_with_access::<Children>(&access, entity)?
            .map(|c| c.to_vec())
            .unwrap_or_default())
    }

    pub fn get_parent(&self, entity: Entity) -> ScriptResult<Option<Entity>> {
        if !self.is_valid_entity(entity) {
            return Err(ScriptError::new_runtime_error(format!(
                "Entity does not exist or is not valid: {:?}",
                entity
            )));
        }

        let access = self
            .get_component_access_typed::<Parent>()
            .unwrap_or_else(|| panic!("{CONCURRENT_ACCESS_MSG}"));

        Ok(self
            .get_component_with_access::<Parent>(&access, entity)?
            .map(|c| c.get()))
    }

    pub fn push_children(&self, parent: Entity, children: &[Entity]) -> ScriptResult<()> {
        // verify entities exist
        if !self.is_valid_entity(parent) {
            return Err(ScriptError::new_runtime_error(format!(
                "The parent Entity does not exist or is not valid: {:?}",
                parent
            )));
        }
        for c in children {
            if !self.is_valid_entity(*c) {
                return Err(ScriptError::new_runtime_error(format!(
                    "the Entity does not exist or is not valid: {:?}",
                    c
                )));
            }
        }

        if let Some(world) = self.get_whole_world_access() {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).add_children(children);
            queue.apply(world);
        } else {
            panic!("{CONCURRENT_WORLD_ACCESS_MSG}")
        }

        Ok(())
    }

    pub fn remove_children(&self, parent: Entity, children: &[Entity]) -> ScriptResult<()> {
        if !self.is_valid_entity(parent) {
            return Err(ScriptError::new_runtime_error(format!(
                "The parent Entity does not exist or is not valid: {:?}",
                parent
            )));
        }

        for c in children {
            if !self.is_valid_entity(*c) {
                return Err(ScriptError::new_runtime_error(format!(
                    "the Entity does not exist or is not valid: {:?}",
                    c
                )));
            }
        }

        if let Some(world) = self.get_whole_world_access() {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).remove_children(children);
            queue.apply(world);
        } else {
            panic!("{CONCURRENT_WORLD_ACCESS_MSG}")
        }

        Ok(())
    }

    pub fn insert_children(
        &self,
        parent: Entity,
        index: usize,
        children: &[Entity],
    ) -> ScriptResult<()> {
        if !self.is_valid_entity(parent) {
            return Err(ScriptError::new_runtime_error(format!(
                "parent Entity does not exist or is not valid: {:?}",
                parent
            )));
        }

        for c in children {
            if !self.is_valid_entity(*c) {
                return Err(ScriptError::new_runtime_error(format!(
                    "the Entity does not exist or is not valid: {:?}",
                    c
                )));
            }
        }

        if let Some(world) = self.get_whole_world_access() {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).insert_children(index, children);
            queue.apply(world);
        } else {
            panic!("{CONCURRENT_WORLD_ACCESS_MSG}")
        }

        Ok(())
    }

    pub fn despawn_recursive(&self, parent: Entity) -> ScriptResult<()> {
        if !self.is_valid_entity(parent) {
            return Err(ScriptError::new_runtime_error(format!(
                "parent Entity does not exist or is not valid: {:?}",
                parent
            )));
        }

        if let Some(world) = self.get_whole_world_access() {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).despawn_recursive();
            queue.apply(world);
        } else {
            panic!("{CONCURRENT_WORLD_ACCESS_MSG}")
        }

        Ok(())
    }

    pub fn despawn(&self, entity: Entity) -> ScriptResult<()> {
        if !self.is_valid_entity(entity) {
            return Err(ScriptError::new_runtime_error(format!(
                "Entity does not exist or is not valid: {:?}",
                entity
            )));
        }

        if let Some(world) = self.get_whole_world_access() {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(entity).despawn();
            queue.apply(world);
        } else {
            panic!("{CONCURRENT_WORLD_ACCESS_MSG}")
        }

        Ok(())
    }

    pub fn despawn_descendants(&self, parent: Entity) -> ScriptResult<()> {
        if !self.is_valid_entity(parent) {
            return Err(ScriptError::new_runtime_error(format!(
                "parent Entity does not exist or is not valid: {:?}",
                parent
            )));
        }

        if let Some(world) = self.get_whole_world_access() {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).despawn_descendants();
            queue.apply(world);
        } else {
            panic!("{CONCURRENT_WORLD_ACCESS_MSG}")
        }

        Ok(())
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

    /// Prints the type of access this [`WorldAccessWrite`] entails, enriched with type information from the registry
    pub fn to_enriched_str(
        &self,
        registry: &TypeRegistry,
        allocator: Option<&ReflectAllocator>,
        cell: UnsafeWorldCell,
    ) -> String {
        let (base_type, type_id) = match self.raid.kind {
            ReflectAccessKind::ComponentOrResource => {
                let type_id = cell
                    .components()
                    .get_info(ComponentId::new(self.raid.id))
                    .and_then(|info| info.type_id());

                ("Component/Resource", type_id)
            }
            ReflectAccessKind::Allocation => {
                let type_id = allocator
                    .and_then(|allocator| allocator.get_type_id(ReflectAllocationId(self.raid.id)));
                ("Allocation", type_id)
            }
        };

        type_id
            .and_then(|type_id| registry.get_type_info(type_id))
            .map(|info| format!("{base_type}<{}>", info.type_path()))
            .unwrap_or(format!("{:?}", self.raid))
    }
}

// pub struct

#[cfg(test)]
mod test {
    use crate::bindings::ScriptTypeRegistration;
    use crate::prelude::ScriptErrorKind;
    use bevy::ecs::system::Commands;
    use bevy::hierarchy::BuildChildren;
    use bevy::reflect::{ParsedPath, Reflect};

    use super::*;
    use std::any::TypeId;

    use crate::{
        bindings::ReflectAllocator,
        bindings::{
            DeferredReflection, ReflectBase, ReflectBaseType, ReflectReference, ReflectionPathElem,
        },
    };

    use bevy::ecs::world::World;
    use test_utils::test_data::{
        setup_world, CompWithFromWorld, GetTestComponentId, TestComponent, TestResource,
    };

    fn init_world() -> World {
        setup_world(|w, _| {
            w.init_resource::<ReflectAllocator>();
        })
    }

    /// Tests that the given ref_ can be accessed and the value is as expected and access is released correctly (not for allocated values)
    fn assert_access_yields<
        O: Reflect + PartialEq + Debug,
        F: FnOnce(&mut World) -> ReflectReference,
        G: FnOnce(&WorldAccessGuard),
    >(
        init: F,
        post_check: G,
        expected: O,
    ) {
        let mut world = init_world();
        let ref_ = init(&mut world);

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let world = world.read().unwrap();

            // test read
            world.with_resource(|world, allocator: Mut<ReflectAllocator>| {
                world.with_resource(|world, type_registry: Mut<AppTypeRegistry>| {
                    let type_registry = type_registry.read();
                    ref_.with_reflect(world, &type_registry, Some(&allocator), |reflect| {
                        let orig = reflect.try_downcast_ref::<O>();

                        let orig = match orig {
                            Some(v) => v,
                            None => {
                                panic!(
                                    "Could not downcast value {reflect:?} to {}",
                                    std::any::type_name::<O>()
                                )
                            }
                        };

                        assert_eq!(orig, &expected);
                    })
                })
            });

            assert!(
                world
                    .get_component_access(TestComponent::test_component_id())
                    .is_some(),
                "access to component was not release correctly"
            );

            assert!(
                world
                    .get_resource_access(TestResource::test_component_id())
                    .is_some(),
                "access to component was not release correctly"
            );

            post_check(&world);
        });
    }

    /// Tests that setting to the expected value works as well as follow up reads give the expected value
    fn assert_set_then_get_yields<
        O: Reflect + PartialEq + Debug + Clone,
        F: FnOnce(&mut World) -> ReflectReference,
        G: FnOnce(&WorldAccessGuard),
    >(
        init: F,
        post_check: G,
        expected: O,
    ) {
        let mut world = init_world();
        let ref_ = init(&mut world);
        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let world = world.read().unwrap();
            // test set
            world.with_resource(|world, allocator: Mut<ReflectAllocator>| {
                world.with_resource(|world, type_registry: Mut<AppTypeRegistry>| {
                    let type_registry = type_registry.read();
                    ref_.with_reflect_mut(world, &type_registry, Some(&allocator), |reflect| {
                        let orig = reflect.try_downcast_mut::<O>();

                        let orig = match orig {
                            Some(v) => v,
                            None => {
                                panic!(
                                    "Could not downcast value {reflect:?} to {}",
                                    std::any::type_name::<O>()
                                )
                            }
                        };

                        *orig = expected.clone();
                    })
                })
            });

            // test read
            world.with_resource(|world, allocator: Mut<ReflectAllocator>| {
                world.with_resource(|world, type_registry: Mut<AppTypeRegistry>| {
                    let type_registry = type_registry.read();
                    ref_.with_reflect(world, &type_registry, Some(&allocator), |reflect| {
                        let orig = reflect.try_downcast_ref::<O>();

                        let orig = match orig {
                            Some(v) => v,
                            None => {
                                panic!(
                                    "Could not downcast value {reflect:?} to {}",
                                    std::any::type_name::<O>()
                                )
                            }
                        };

                        assert_eq!(orig, &expected);
                    })
                })
            });
            post_check(&world);
        });
    }

    #[test]
    fn test_component_access() {
        let init = |world: &mut World| {
            let entity = world
                .spawn(TestComponent {
                    strings: vec![String::from("initial")],
                })
                .id();

            ReflectReference {
                base: ReflectBaseType {
                    base_id: ReflectBase::Component(entity, TestComponent::test_component_id()),
                    type_id: TypeId::of::<TestComponent>(),
                },
                reflect_path: vec![
                    ReflectionPathElem::Reflection(ParsedPath::parse_static(".strings").unwrap()),
                    ReflectionPathElem::DeferredReflection(DeferredReflection {
                        get: Arc::new(|root| {
                            let strings = root.try_downcast_ref::<Vec<String>>().unwrap();
                            Ok(strings.first().unwrap())
                        }),
                        get_mut: Arc::new(|root| {
                            let strings = root.try_downcast_mut::<Vec<String>>().unwrap();
                            Ok(strings.first_mut().unwrap())
                        }),
                    }),
                ],
            }
        };

        assert_access_yields(init, |_| {}, String::from("initial"));
        assert_set_then_get_yields(init, |_| {}, String::from("set"));
    }

    #[test]
    fn test_resource_access() {
        let init = |world: &mut World| {
            world.insert_resource(TestResource { bytes: vec![42u8] });

            ReflectReference {
                base: ReflectBaseType {
                    base_id: ReflectBase::Resource(TestResource::test_component_id()),
                    type_id: TypeId::of::<TestResource>(),
                },
                reflect_path: vec![
                    ReflectionPathElem::Reflection(ParsedPath::parse_static(".bytes").unwrap()),
                    ReflectionPathElem::DeferredReflection(DeferredReflection {
                        get: Arc::new(|root| {
                            let strings = root.try_downcast_ref::<Vec<u8>>().unwrap();
                            Ok(strings.first().unwrap())
                        }),
                        get_mut: Arc::new(|root| {
                            let strings = root.try_downcast_mut::<Vec<u8>>().unwrap();
                            Ok(strings.first_mut().unwrap())
                        }),
                    }),
                ],
            }
        };
        assert_access_yields(init, |_| {}, 42u8);
        assert_set_then_get_yields(init, |_| {}, 69u8);
    }

    #[test]
    fn test_script_alloc_access() {
        let init = |world: &mut World| {
            let mut script_allocator = ReflectAllocator::default();
            let mut ref_ = ReflectReference::new_allocated(
                TestComponent {
                    strings: vec![String::from("initial")],
                },
                &mut script_allocator,
            );
            ref_.index_path(ParsedPath::parse_static(".strings").unwrap());
            ref_.index_path(DeferredReflection {
                get: Arc::new(|root| {
                    let strings = root.try_downcast_ref::<Vec<String>>().unwrap();
                    Ok(strings.first().unwrap())
                }),
                get_mut: Arc::new(|root| {
                    let strings = root.try_downcast_mut::<Vec<String>>().unwrap();
                    Ok(strings.first_mut().unwrap())
                }),
            });
            world.insert_resource(script_allocator);
            ref_
        };
        let post_check = |world: &WorldAccessGuard| {
            assert!(
                world
                    .get_allocation_access(ReflectAllocationId(0))
                    .is_some(),
                "allocation access was not released correctly"
            );
        };
        assert_access_yields(init, post_check, String::from("initial"));
        assert_set_then_get_yields(init, post_check, String::from("set"));
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

    #[test]
    fn test_count_updated_correctly() {
        let mut world = World::new();
        let guard = WorldAccessGuard::new(&mut world);
        let access = guard.get_access(ComponentId::new(0).into()).unwrap();
        assert_eq!(1, guard.accesses_count.load(Ordering::Relaxed));
        guard.release_access(access);
        assert_eq!(0, guard.accesses_count.load(Ordering::Relaxed));
    }

    fn get_reg(world: &WorldCallbackAccess, name: &str) -> ScriptTypeRegistration {
        world
            .get_type_by_name(name.to_owned())
            .expect("Type not found")
    }

    fn test_comp_reg(world: &WorldCallbackAccess) -> ScriptTypeRegistration {
        world
            .get_type_by_name("TestComponent".to_owned())
            .expect("Component not found")
    }

    fn test_resource_reg(world: &WorldCallbackAccess) -> ScriptTypeRegistration {
        world
            .get_type_by_name("TestResource".to_owned())
            .expect("Resource not found")
    }

    #[test]
    fn test_get_type_by_name() {
        let mut world = init_world();
        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let comp_reg = world.get_type_by_name("TestComponent".to_owned()).unwrap();
            let resource_reg = world.get_type_by_name("TestResource".to_owned()).unwrap();

            assert_eq!(
                comp_reg.registration.type_info().type_id(),
                std::any::TypeId::of::<TestComponent>()
            );
            assert_eq!(
                resource_reg.registration.type_info().type_id(),
                std::any::TypeId::of::<TestResource>()
            );
        });
    }

    #[test]
    fn test_get_type_by_name_invalid() {
        let mut world = init_world();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let comp_reg = world.get_type_by_name("x".to_owned());
            let resource_reg = world.get_type_by_name("z".to_owned());

            assert!(comp_reg.is_none());
            assert!(resource_reg.is_none());
        });
    }

    #[test]
    fn test_add_default_component_from_world() {
        let mut world = init_world();

        let entity = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let comp_reg = get_reg(world, "CompWithFromWorld");
            world.add_default_component(entity, comp_reg).unwrap()
        });

        assert_eq!(
            world.get_entity(entity).unwrap().get::<CompWithFromWorld>(),
            Some(&CompWithFromWorld(String::from("FromWorld")))
        );
    }

    #[test]
    fn test_add_default_component_default() {
        let mut world = init_world();

        let entity = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let comp_reg = get_reg(world, "CompWithFromWorld");
            world.add_default_component(entity, comp_reg).unwrap()
        });

        assert_eq!(
            world.get_entity(entity).unwrap().get::<CompWithFromWorld>(),
            Some(&CompWithFromWorld(String::from("Default")))
        );
    }

    #[test]
    fn test_add_default_component_missing_from_world_and_default() {
        let mut world = init_world();

        let entity = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let comp_reg = get_reg(world, "CompWithFromWorld");
            match world.add_default_component(entity, comp_reg.clone()) {
                Ok(_) => {
                    panic!("Expected error")
                }
                Err(ScriptError(inner)) => {
                    assert_eq!(inner.kind, ScriptErrorKind::RuntimeError);
                    assert_eq!(inner.reason.to_string(), format!("Cannot add default component since type: `{}`, Does not have ReflectDefault or ReflectFromWorld data registered.", comp_reg.registration.type_info().type_path()));
                }
            }
        });
    }

    #[test]
    fn test_add_default_component_missing_component_data() {
        let mut world = init_world();

        let entity = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let comp_reg = get_reg(world, "CompWithFromWorld");
            match world.add_default_component(entity, comp_reg.clone()) {
                Ok(_) => {
                    panic!("Expected error")
                }
                Err(ScriptError(inner)) => {
                    assert_eq!(inner.kind, ScriptErrorKind::RuntimeError);
                    assert_eq!(inner.reason.to_string(), format!("Cannot add default component since type: `{}`, Does not have ReflectComponent data registered.", comp_reg.registration.type_info().type_path()));
                }
            }
        });
    }

    #[test]
    fn test_get_component_existing() {
        let mut world = init_world();

        let entity = world.spawn(TestComponent { strings: vec![] }).id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let comp_ref = world
                .get_component(entity, TestComponent::test_component_id())
                .unwrap()
                .unwrap();
            assert_eq!(
                comp_ref,
                ReflectReference {
                    base: ReflectBaseType {
                        type_id: std::any::TypeId::of::<TestComponent>(),
                        base_id: ReflectBase::Component(entity, TestComponent::test_component_id()),
                    },
                    reflect_path: Default::default(),
                }
            );
        });
    }

    #[test]
    fn test_get_component_missing() {
        let mut world = init_world();

        let entity = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let comp_ref = world
                .get_component(entity, TestComponent::test_component_id())
                .unwrap();
            assert_eq!(comp_ref, None);
        });
    }

    #[test]
    fn test_get_component_missing_entity() {
        let mut world = init_world();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let comp_ref =
                world.get_component(Entity::from_raw(0), TestComponent::test_component_id());
            match comp_ref {
                Ok(_) => {
                    panic!("Expected error")
                }
                Err(e) => {
                    assert_eq!(e.kind, ScriptErrorKind::RuntimeError);
                    assert_eq!(e.reason.to_string(), "Entity does not exist: 0v1");
                }
            }
        });
    }

    #[test]
    fn test_get_component_unregistered_component() {
        let mut world = init_world();

        let entity = world.spawn_empty().id();
        let fake_id = ComponentId::new(999);

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let comp_ref = world.get_component(entity, fake_id);
            match comp_ref {
                Ok(_) => {
                    panic!("Expected error")
                }
                Err(e) => {
                    assert_eq!(e.kind, ScriptErrorKind::RuntimeError);
                    assert_eq!(
                        e.reason.to_string(),
                        format!("Component does not exist: {fake_id:?}"),
                    );
                }
            }
        });
    }

    #[test]
    fn test_remove_component() {
        let mut world = init_world();

        let entity = world
            .spawn(TestComponent {
                strings: vec![String::from("strings")],
            })
            .id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world
                .remove_component(entity, test_comp_reg(world))
                .unwrap();
        });

        assert_eq!(
            world.get_entity(entity).unwrap().get::<TestComponent>(),
            None
        );
    }

    #[test]
    fn test_remove_component_empty_idempotent() {
        let mut world = init_world();

        let entity = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world
                .remove_component(entity, test_comp_reg(world))
                .unwrap();
        });

        assert_eq!(
            world.get_entity(entity).unwrap().get::<TestComponent>(),
            None
        );
    }

    #[test]
    fn test_remove_component_missing_comp_registration() {
        let mut world = init_world();

        let entity = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let result = world.remove_component(entity, test_resource_reg(world));
            match result {
                Ok(_) => {
                    panic!("Expected error")
                }
                Err(e) => {
                    assert_eq!(e.kind, ScriptErrorKind::RuntimeError);
                    assert_eq!(
                        e.reason.to_string(),
                        format!("Cannot remove component since type: `{}`, Does not have ReflectComponent data registered.", test_resource_reg(world).registration.type_info().type_path())
                    );
                }
            }
        });

        assert_eq!(
            world.get_entity(entity).unwrap().get::<TestComponent>(),
            None
        );
    }

    #[test]
    fn test_remove_component_missing_entity() {
        let mut world = init_world();

        let fake_entity = Entity::from_raw(0);

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let result = world.remove_component(fake_entity, test_comp_reg(world));
            match result {
                Ok(_) => {
                    panic!("Expected error")
                }
                Err(e) => {
                    assert_eq!(e.kind, ScriptErrorKind::RuntimeError);
                    assert_eq!(e.reason.to_string(), "Entity does not exist: 0v1");
                }
            }
        });
    }

    #[test]
    fn test_get_resource_existing() {
        let mut world = init_world();

        world.insert_resource(TestResource { bytes: vec![1] });

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let comp_ref = world
                .get_resource(TestResource::test_component_id())
                .unwrap()
                .unwrap();
            assert_eq!(
                comp_ref,
                ReflectReference {
                    base: ReflectBaseType {
                        type_id: std::any::TypeId::of::<TestResource>(),
                        base_id: ReflectBase::Resource(TestResource::test_component_id()),
                    },
                    reflect_path: Default::default(),
                }
            );
        });
    }

    #[test]
    fn test_get_resource_non_existing() {
        let mut world = init_world();

        let fake_comp = ComponentId::new(999);
        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let comp_ref = world.get_resource(fake_comp);
            match comp_ref {
                Ok(None) => {}
                e => {
                    panic!("Expected ok with none, got: {:?}", e);
                }
            }
        });
    }

    #[test]
    fn test_remove_resource() {
        let mut world = init_world();

        world.insert_resource(TestResource { bytes: vec![1] });

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world.remove_resource(test_resource_reg(world)).unwrap();
        });

        assert_eq!(world.get_resource::<TestResource>(), None);
    }

    #[test]
    fn test_remove_resource_missing_idempotent() {
        let mut world = init_world();

        world.remove_resource::<TestResource>();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world.remove_resource(test_resource_reg(world)).unwrap();
        });

        assert_eq!(world.get_resource::<TestResource>(), None);
    }

    #[test]
    fn test_remove_resource_missing_resource_registration() {
        let mut world = init_world();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            match world.remove_resource(test_comp_reg(world)) {
                Ok(_) => panic!("Expected error"),
                Err(e) => {
                    assert_eq!(e.kind, ScriptErrorKind::RuntimeError);
                    assert_eq!(e.reason.to_string(), format!("Cannot remove resource since type: `{}`, Does not have ReflectResource data registered.", test_comp_reg(world).registration.type_info().type_path()));
                }
            }
        });
    }

    #[test]
    fn test_has_resource_existing() {
        let mut world = init_world();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            assert!(world.has_resource(TestResource::test_component_id()));
        });
    }

    #[test]
    fn test_has_resource_missing() {
        let mut world = init_world();

        world.remove_resource::<TestResource>();
        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            assert!(world.has_resource(TestResource::test_component_id()));
        });
    }

    #[test]
    fn test_get_children_1_child() {
        let mut world = init_world();

        let parent = world.spawn_empty().id();
        let child = world.spawn_empty().id();
        let mut cmnds = CommandQueue::default();
        let mut cmnd = Commands::new(&mut cmnds, &world);
        cmnd.entity(parent).add_children(&[child]);
        cmnds.apply(&mut world);

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let children = world.get_children(parent).unwrap();
            assert_eq!(children.len(), 1);
            assert_eq!(children[0], child);
        });
    }

    #[test]
    #[should_panic(
        expected = "Component not registered: `bevy_hierarchy::components::children::Children`"
    )]
    fn test_get_children_children_component_unregistered() {
        let mut world = init_world();

        let parent = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world.get_children(parent).unwrap();
        });
    }

    #[test]
    fn test_get_children_no_children() {
        let mut world = init_world();

        world.register_component::<Children>();
        let parent = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let children = world.get_children(parent).unwrap();
            assert_eq!(children.len(), 0);
        });
    }

    #[test]
    fn test_get_parent_1_parent() {
        let mut world = init_world();

        let parent = world.spawn_empty().id();
        let child = world.spawn_empty().id();
        let mut cmnds = CommandQueue::default();
        let mut cmnd = Commands::new(&mut cmnds, &world);
        cmnd.entity(parent).add_children(&[child]);
        cmnds.apply(&mut world);

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let found_parent = world.get_parent(child).unwrap();
            assert_eq!(found_parent, Some(parent));
        });
    }

    #[test]
    fn test_get_parent_no_parent() {
        let mut world = init_world();

        world.register_component::<Parent>();

        let child = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            let found_parent = world.get_parent(child).unwrap();
            assert_eq!(found_parent, None);
        });
    }

    #[test]
    #[should_panic(
        expected = "Component not registered: `bevy_hierarchy::components::parent::Parent`"
    )]
    fn test_get_parent_parent_component_unregistered() {
        let mut world = init_world();

        let child = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world.get_parent(child).unwrap();
        });
    }

    #[test]
    fn test_push_children_empty_entity() {
        let mut world = init_world();

        let parent = world.spawn_empty().id();
        let child = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world.push_children(parent, &[child]).unwrap();
        });

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0], child);
    }

    #[test]
    fn test_push_children_entity_with_1_child() {
        let mut world = init_world();

        let parent = world.spawn_empty().id();
        let child = world.spawn_empty().id();
        let mut cmnds = CommandQueue::default();
        let mut cmnd = Commands::new(&mut cmnds, &world);
        cmnd.entity(parent).add_children(&[child]);
        cmnds.apply(&mut world);

        let child_2 = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world.push_children(parent, &[child_2]).unwrap();
        });

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0], child);
        assert_eq!(children[1], child_2);
    }

    #[test]
    fn test_remove_children_entity_with_1_child() {
        let mut world = init_world();

        let parent = world.spawn_empty().id();
        let child = world.spawn_empty().id();
        let mut cmnds = CommandQueue::default();
        let mut cmnd = Commands::new(&mut cmnds, &world);
        cmnd.entity(parent).add_children(&[child]);
        cmnds.apply(&mut world);

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world.remove_children(parent, &[child]).unwrap();
        });

        let children = world.get::<Children>(parent);
        assert!(children.is_none());
    }

    #[test]
    fn test_remove_children_remove_half_children() {
        let mut world = init_world();

        let parent = world.spawn_empty().id();
        let child = world.spawn_empty().id();
        let child_2 = world.spawn_empty().id();
        let mut cmnds = CommandQueue::default();
        let mut cmnd = Commands::new(&mut cmnds, &world);
        cmnd.entity(parent).add_children(&[child, child_2]);
        cmnds.apply(&mut world);

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world.remove_children(parent, &[child]).unwrap();
        });

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0], child_2);
    }

    #[test]
    fn test_insert_children_empty() {
        let mut world = init_world();

        let parent = world.spawn_empty().id();
        let child = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world.insert_children(parent, 0, &[child]).unwrap();
        });

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0], child);
    }

    #[test]
    fn test_insert_children_middle() {
        let mut world = init_world();

        let parent = world.spawn_empty().id();
        let child = world.spawn_empty().id();
        let child_2 = world.spawn_empty().id();
        let mut cmnds = CommandQueue::default();
        let mut cmnd = Commands::new(&mut cmnds, &world);
        cmnd.entity(parent).add_children(&[child, child_2]);
        cmnds.apply(&mut world);

        let child_to_insert = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world
                .insert_children(parent, 1, &[child_to_insert])
                .unwrap();
        });

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 3);
        assert_eq!(children[0], child);
        assert_eq!(children[1], child_to_insert);
        assert_eq!(children[2], child_2);
    }

    #[test]
    fn test_despawn_entity() {
        let mut world = init_world();

        let entity = world.spawn_empty().id();

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world.despawn(entity).unwrap();
        });

        assert!(world.get_entity(entity).is_err());
    }

    #[test]
    fn test_despawn_recursive() {
        let mut world = init_world();

        let parent = world.spawn_empty().id();
        let child = world.spawn_empty().id();
        let mut cmnds = CommandQueue::default();
        let mut cmnd = Commands::new(&mut cmnds, &world);
        cmnd.entity(parent).add_children(&[child]);
        cmnds.apply(&mut world);

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world.despawn_recursive(parent).unwrap();
        });

        assert!(world.get_entity(parent).is_err());
        assert!(world.get_entity(child).is_err());
    }

    #[test]
    fn test_despawn_descendants() {
        let mut world = init_world();

        let parent = world.spawn_empty().id();
        let child = world.spawn_empty().id();
        let mut cmnds = CommandQueue::default();
        let mut cmnd = Commands::new(&mut cmnds, &world);
        cmnd.entity(parent).add_children(&[child]);
        cmnds.apply(&mut world);

        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            world.despawn_descendants(parent).unwrap();
        });

        assert!(world.get_entity(parent).is_ok());
        assert!(world.get_entity(child).is_err());
    }
}
