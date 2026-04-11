//! # Motivation
//!
//! Traits and structs needed to support the creation of bindings for scripting languages.
//! reflection gives us access to `dyn PartialReflect` objects via their type name,
//! Scripting languages only really support `Clone` objects so if we want to support references,
//! we need wrapper types which have owned and ref variants.

use crate::{AccessByteSet, AccessMap, WorldAccessRange};

use super::access_map::DynamicSystemMeta;
use ::bevy_ecs::{
    component::ComponentId,
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};
use bevy_ecs::{
    component::Component, reflect::AppTypeRegistry, resource::Resource, system::Command,
    world::WorldId,
};
use bevy_reflect::TypeRegistryArc;
use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell},
    fmt::Debug,
    rc::Rc,
    sync::atomic::AtomicBool,
};

/// Prefer to directly using [`WorldAccessGuard`]. If the underlying type changes, this alias will be updated.
pub type WorldGuard<'w> = WorldAccessGuard<'w>;
/// Similar to [`WorldGuard`], but without the arc, use for when you don't need the outer Arc.
pub type WorldGuardRef<'w> = &'w WorldAccessGuard<'w>;

/// A class of errors related to accessing the world with untyped acccess information
#[derive(Debug)]
pub enum DynWorldAccessError {
    /// World thread local was not set
    MissingWorld,
    /// Could not claim necessary access
    CannotClaimAccess(WorldAccessRange, String),
    /// Resource was not registered
    UnregisteredResource(TypeId),
    /// Component was not registered
    UnregisteredComponent(TypeId),
}

impl DynWorldAccessError {
    /// Creates [`DynWorldAccessError::MissingWorld`]
    pub fn missing_world() -> Self {
        Self::MissingWorld
    }

    /// Creates [`DynWorldAccessError::CannotClaimAccess`]
    pub fn cannot_claim_access(key: WorldAccessRange, msg: impl ToString) -> Self {
        Self::CannotClaimAccess(key, msg.to_string())
    }
}

/// Provides safe access to the world via [`AnyAccessMap`] permissions, which enforce aliasing rules at runtime in multi-thread environments
#[derive(Clone, Debug)]
pub struct WorldAccessGuard<'w> {
    /// The guard this guard pointer represents
    pub(crate) inner: Rc<WorldAccessGuardInner<'w>>,
    /// if true the guard is invalid and cannot be used, stored as a second pointer so that this validity can be
    /// stored separate from the contents of the guard
    invalid: Rc<AtomicBool>,
}
impl WorldAccessGuard<'_> {
    /// Returns the id of the world this guard provides access to
    pub fn id(&self) -> WorldId {
        self.inner.cell.id()
    }
}

/// A registry which is cached withing the [`WorldAccessGuard`] to avoid many access lookups.
///
/// Slots are reserved by each registry type, and the same slot must not ever be used by two registries of different types.
///
/// Allows us to decouple dependencies while retaining some caching benefits.
pub trait CachedRegistry: Any {
    /// The cache slot used by this registry.
    /// Must not be used by another registry to work correctly.
    const SLOT: usize;
}

/// Aliases the type used as the registry cache for the world guard.
pub type RegistryCache = [Rc<RefCell<dyn Any>>; 5];

/// Used to decrease the stack size of [`WorldAccessGuard`]
pub(crate) struct WorldAccessGuardInner<'w> {
    /// Safety: cannot be used unless the scope depth is less than the max valid scope
    cell: UnsafeWorldCell<'w>,
    // TODO: this is fairly hefty, explore sparse sets, bit fields etc
    pub(crate) accesses: AccessMap,
    /// Cached for convenience, since we need it for most operations, means we don't need to lock the type registry every time
    type_registry: TypeRegistryArc,

    cached_slots: RegistryCache,
    // /// The script allocator for the world
    // allocator: AppReflectAllocator,
    // /// The function registry for the world
    // function_registry: AppScriptFunctionRegistry,
    // /// The schedule registry for the world
    // schedule_registry: AppScheduleRegistry,
    // /// The registry of script registered components
    // script_component_registry: AppScriptComponentRegistry,
}

impl std::fmt::Debug for WorldAccessGuardInner<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorldAccessGuardInner").finish()
    }
}

#[profiling::all_functions]
impl WorldAccessGuard<'static> {
    /// Shortens the lifetime of the guard to the given lifetime.
    pub(crate) fn shorten_lifetime<'w>(self) -> WorldGuard<'w> {
        // Safety: todo
        unsafe { std::mem::transmute(self) }
    }
}

#[profiling::all_functions]
impl<'w> WorldAccessGuard<'w> {
    /// Retrieves the cached type registry from this instantiation of the guard
    pub fn type_registry(&self) -> &TypeRegistryArc {
        &self.inner.type_registry
    }

    #[track_caller]
    /// Claims read access to the given type.
    pub fn claim_read_access(&self, raid: impl Into<WorldAccessRange>) -> bool {
        self.inner.accesses.claim_read_access(raid)
    }

    #[track_caller]
    /// Claims write access to the given type.
    pub fn claim_write_access(&self, raid: impl Into<WorldAccessRange>) -> bool {
        self.inner.accesses.claim_write_access(raid)
    }

    /// Releases read or write access to the given type.
    ///
    /// # Safety
    /// - This can only be called safely after all references to the type created using the access have been dropped
    /// - You can only call this if you previously called one of: [`WorldAccessGuard::claim_read_access`] or [`WorldAccessGuard::claim_write_access`]
    /// - The number of claim and release calls for the same id must always match
    pub unsafe fn release_access(&self, raid: impl Into<WorldAccessRange>) {
        self.inner.accesses.release_access(raid)
    }

    /// Procures and releases the given access key allowing safe read access to world resources
    /// requiring that key within the given closure
    #[track_caller]
    pub fn with_read_access<F: FnOnce() -> O, O>(
        &self,
        key: impl Into<WorldAccessRange>,
        f: F,
    ) -> Result<O, DynWorldAccessError> {
        let key = key.into();
        if !self.inner.accesses.claim_read_access(key) {
            Err(DynWorldAccessError::cannot_claim_access(
                key,
                "Could not claim read access",
            ))
        } else {
            let res = f();
            // Safety: we have claimed read access to this key
            unsafe { self.release_access(key) };
            Ok(res)
        }
    }

    /// Procures and releases the given access key allowing safe read access to world resources
    /// requiring that key within the given closure.
    ///
    /// This is a version of [`WorldAccessGuard::with_read_access`] which flattens errors using into implementations
    #[track_caller]
    pub fn with_read_access_and_then<E, F: FnOnce() -> Result<O, E>, O>(
        &self,
        key: impl Into<WorldAccessRange>,
        f: F,
    ) -> Result<O, E>
    where
        DynWorldAccessError: Into<E>,
    {
        let key = key.into();
        if !self.inner.accesses.claim_read_access(key) {
            Err(DynWorldAccessError::cannot_claim_access(key, "Could not claim read access").into())
        } else {
            let res = f()?;
            // Safety: we have claimed read access to this key
            unsafe { self.release_access(key) };
            Ok(res)
        }
    }

    /// Procures and releases the given access key allowing safe write access to world resources
    /// requiring that key within the given closure
    #[track_caller]
    pub fn with_write_access<F: FnOnce() -> O, O>(
        &self,
        key: impl Into<WorldAccessRange>,
        f: F,
    ) -> Result<O, DynWorldAccessError> {
        let key = key.into();
        if !self.inner.accesses.claim_write_access(key) {
            Err(DynWorldAccessError::cannot_claim_access(
                key,
                "Could not claim write access",
            ))
        } else {
            let res = f();
            // Safety: we have claimed read access to this key
            unsafe { self.release_access(key) };
            Ok(res)
        }
    }

    /// Procures and releases the given access key allowing safe write access to world resources
    /// requiring that key within the given closure
    ///
    /// This is a version of [`WorldAccessGuard::with_write_access`] which flattens errors using into implementations.
    #[track_caller]
    pub fn with_write_access_and_then<E, F: FnOnce() -> Result<O, E>, O>(
        &self,
        key: impl Into<WorldAccessRange>,
        f: F,
    ) -> Result<O, E>
    where
        DynWorldAccessError: Into<E>,
    {
        let key = key.into();
        if !self.inner.accesses.claim_write_access(key) {
            Err(
                DynWorldAccessError::cannot_claim_access(key, "Could not claim write access")
                    .into(),
            )
        } else {
            let res = f()?;
            // Safety: we have claimed read access to this key
            unsafe { self.release_access(key) };
            Ok(res)
        }
    }

    /// Procures and releases the given access key allowing safe read access to world resources
    /// requiring that key within the given closure
    #[track_caller]
    pub fn with_world_access<F: FnOnce(&World) -> O, O>(
        &self,
        f: F,
    ) -> Result<O, DynWorldAccessError> {
        self.with_read_access(WorldAccessRange::Global, || {
            let cell = self.as_unsafe_world_cell()?;
            // Safety: we have exclusive access
            Ok(f(unsafe { cell.world() }))
        })?
    }

    /// Procures and releases the given access key allowing safe write access to world resources
    /// requiring that key within the given closure
    #[track_caller]
    pub fn with_world_mut_access<F: FnOnce(&mut World) -> O, O>(
        &self,
        f: F,
    ) -> Result<O, DynWorldAccessError> {
        self.with_write_access(WorldAccessRange::Global, || {
            let cell = self.as_unsafe_world_cell()?;
            // Safety: we have exclusive access
            Ok(f(unsafe { cell.world_mut() }))
        })?
    }

    /// Procures and releases the given access key allowing safe write access to world resources
    /// requiring that key within the given closure
    ///
    /// This is a version of [`WorldAccessGuard::with_world_mut_access`] which flattens errors using into implementations.
    #[track_caller]
    pub fn with_world_mut_access_and_then<E, F: FnOnce(&mut World) -> Result<O, E>, O>(
        &self,
        f: F,
    ) -> Result<O, E>
    where
        DynWorldAccessError: Into<E>,
    {
        let cell = self.as_unsafe_world_cell().map_err(Into::into)?;
        self.with_write_access_and_then(WorldAccessRange::Global, || {
            // Safety: we have exclusive access
            f(unsafe { cell.world_mut() })
        })
    }

    /// Procures and releases the given access key allowing safe read access to world resources
    /// requiring that key within the given closure
    ///
    /// This is a version of [`WorldAccessGuard::with_world_access`] which flattens errors using into implementations.
    #[track_caller]
    pub fn with_world_access_and_then<E, F: FnOnce(&World) -> Result<O, E>, O>(
        &self,
        f: F,
    ) -> Result<O, E>
    where
        DynWorldAccessError: Into<E>,
    {
        let cell = self.as_unsafe_world_cell().map_err(Into::into)?;
        self.with_write_access_and_then(WorldAccessRange::Global, || {
            // Safety: we have exclusive access
            f(unsafe { cell.world_mut() })
        })
    }

    /// Returns true if the guard is valid, false if it is invalid
    fn is_valid(&self) -> bool {
        !self.invalid.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Invalidates the world access guard, making it and any guards derived from this one unusable.
    pub fn invalidate(&self) {
        self.invalid
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Safely allows access to the world for the duration of the closure via a static [`WorldAccessGuard`].
    ///
    /// The guard is invalidated at the end of the closure, meaning the world cannot be accessed at all after the closure ends.
    pub fn with_static_guard<O>(
        world: &'w mut World,
        cached_slots: RegistryCache,
        f: impl FnOnce(WorldGuard<'static>) -> O,
    ) -> O {
        let guard = WorldAccessGuard::new_exclusive(world, cached_slots);
        // safety: we invalidate the guard after the closure is called, meaning the world cannot be accessed at all after the 'w lifetime ends
        let static_guard: WorldAccessGuard<'static> = unsafe { std::mem::transmute(guard) };
        ThreadWorldContainer.set_context(ThreadScriptContext {
            world: static_guard.clone(),
        });
        let o = f(static_guard.clone());

        static_guard.invalidate();
        o
    }

    /// Safely allows access to the world for the duration of the closure via a static [`WorldAccessGuard`] using a previously lifetimed world guard.
    /// Will invalidate the static guard at the end but not the original.
    pub fn with_existing_static_guard<O>(
        guard: WorldAccessGuard<'w>,
        f: impl FnOnce(WorldGuard<'static>) -> O,
    ) -> O {
        // safety: we invalidate the guard after the closure is called, meaning the world cannot be accessed at all after the 'w lifetime ends, from the static guard
        // i.e. even if somebody squirells it away, it will be useless.
        let static_guard: WorldAccessGuard<'static> = unsafe { std::mem::transmute(guard.scope()) };
        ThreadWorldContainer.set_context(ThreadScriptContext {
            world: static_guard.clone(),
        });
        let o = f(static_guard.clone());
        static_guard.invalidate();
        o
    }

    /// Creates a new [`WorldAccessGuard`] from a possibly non-exclusive access to the world.
    ///
    /// It requires specyfing the exact accesses that are allowed to be given out by the guard.
    /// Those accesses need to be safe to be given out to the script, as the guard will assume that it is safe to give them out in any way.
    ///
    /// # Safety
    /// - The caller must ensure that the accesses in subset are not aliased by any other access
    /// - If an access is allowed in this subset, but alised by someone else,
    pub unsafe fn new_non_exclusive(
        world: UnsafeWorldCell<'w>,
        subset: AccessByteSet,
        type_registry: TypeRegistryArc,
        registry_cache: RegistryCache,
    ) -> Self {
        Self {
            inner: Rc::new(WorldAccessGuardInner {
                cell: world,
                accesses: AccessMap::new_subset(subset),
                type_registry,
                cached_slots: registry_cache,
            }),
            invalid: Rc::new(false.into()),
        }
    }

    /// Creates a new [`WorldAccessGuard`] for the given mutable borrow of the world.
    ///
    /// If these resources do not exist, they will be initialized.
    pub fn new_exclusive(world: &'w mut World, registry_cache: RegistryCache) -> Self {
        let type_registry = world.get_resource_or_init::<AppTypeRegistry>().0.clone();
        let map = AccessMap::new();
        Self {
            inner: Rc::new(WorldAccessGuardInner {
                cell: world.as_unsafe_world_cell(),
                accesses: map,
                cached_slots: registry_cache,
                type_registry,
            }),
            invalid: Rc::new(false.into()),
        }
    }

    /// Queues a command to the world, which will be executed later.
    ///
    /// Requires exclusive world access.
    pub(crate) fn queue(&self, command: impl Command) -> Result<(), DynWorldAccessError> {
        self.with_world_mut_access(|w| {
            w.commands().queue(command);
        })
    }

    /// Runs a closure within an isolated access scope, releasing leftover accesses, should only be used in a single-threaded context.
    ///
    /// Safety:
    /// - The caller must ensure it's safe to release any potentially locked accesses.
    pub unsafe fn with_access_scope<O, F: FnOnce() -> O>(
        &self,
        f: F,
    ) -> Result<O, DynWorldAccessError> {
        Ok(self.inner.accesses.with_scope(f))
    }

    /// Gets the component id of the given component or resource
    pub fn get_component_id(&self, id: TypeId) -> Result<Option<ComponentId>, DynWorldAccessError> {
        Ok(self
            .as_unsafe_world_cell_readonly()?
            .components()
            .get_id(id))
    }

    /// Gets the resource id of the given component or resource
    pub fn get_resource_id(&self, id: TypeId) -> Result<Option<ComponentId>, DynWorldAccessError> {
        Ok(self
            .as_unsafe_world_cell_readonly()?
            .components()
            .get_resource_id(id))
    }

    fn resource_component_id<R: Resource>(&self) -> Result<ComponentId, DynWorldAccessError> {
        self.as_unsafe_world_cell()?
            .components()
            .resource_id::<R>()
            .ok_or_else(|| DynWorldAccessError::UnregisteredResource(TypeId::of::<R>()))
    }

    fn component_component_id<R: Component>(&self) -> Result<ComponentId, DynWorldAccessError> {
        self.as_unsafe_world_cell()?
            .components()
            .component_id::<R>()
            .ok_or_else(|| DynWorldAccessError::UnregisteredComponent(TypeId::of::<R>()))
    }

    /// creates a new guard derived from this one, which if invalidated, will not invalidate the original
    fn scope(&self) -> Self {
        let mut new_guard = self.clone();
        new_guard.invalid = Rc::new(
            new_guard
                .invalid
                .load(std::sync::atomic::Ordering::Relaxed)
                .into(),
        );
        new_guard
    }

    /// Retrieves the underlying unsafe world cell, with no additional guarantees of safety
    /// proceed with caution and only use this if you understand what you're doing
    pub fn as_unsafe_world_cell(&self) -> Result<UnsafeWorldCell<'w>, DynWorldAccessError> {
        if !self.is_valid() {
            return Err(DynWorldAccessError::missing_world());
        }

        Ok(self.inner.cell)
    }

    /// Retrieves the underlying read only unsafe world cell, with no additional guarantees of safety
    /// proceed with caution and only use this if you understand what you're doing
    pub fn as_unsafe_world_cell_readonly(
        &self,
    ) -> Result<UnsafeWorldCell<'w>, DynWorldAccessError> {
        if !self.is_valid() {
            return Err(DynWorldAccessError::missing_world());
        }

        Ok(self.inner.cell)
    }

    /// Purely debugging utility to list all accesses currently held.
    pub fn list_accesses(&self) -> Vec<(WorldAccessRange, bool)> {
        self.inner.accesses.list_accesses()
    }

    /// Should only really be used for testing purposes
    pub unsafe fn release_all_accesses(&self) {
        self.inner.accesses.release_all_accesses();
    }

    /// Returns the number of accesses currently held.
    pub fn access_len(&self) -> usize {
        self.inner.accesses.count_accesses()
    }
}

/// Impl block for higher level world methods
#[profiling::all_functions]
impl WorldAccessGuard<'_> {
    /// If a registry has been initialized in this world guard, downcasts it to its original type and returns
    /// a reference to it
    pub fn get_cached_registry<'a, T: CachedRegistry>(&'a self) -> Option<Ref<T, 'a>> {
        let idx = T::SLOT;
        Ref::filter_map(self.inner.cached_slots[idx].borrow(), |r| r.downcast_ref()).ok()
    }

    /// If a registry has been initialized in this world guard, downcasts it to its original type and returns
    /// a reference to it
    pub fn set_cached_registry<T: CachedRegistry>(&self, registry: T) {
        let idx = T::SLOT;
        let mut mutt = RefCell::borrow_mut(&self.inner.cached_slots[idx]);

        #[allow(
            clippy::unwrap_used,
            reason = "internal domain boundary, enforced at creation of the guard"
        )]
        let mutt = mutt.downcast_mut().unwrap();
        *mutt = registry;
    }
}

/// A world container that stores the world in a thread local
pub struct ThreadWorldContainer;

#[derive(Clone)]
/// Context passed down indirectly to script related functions, used to avoid prop drilling problems.
pub struct ThreadScriptContext<'l> {
    /// The world pointer
    pub world: WorldGuard<'l>,
    // /// The currently active script attachment
    // pub attachment: ScriptAttachment,
}

thread_local! {
    static WORLD_CALLBACK_ACCESS: RefCell<Option<ThreadScriptContext<'static>>> = const { RefCell::new(None) };
}
#[profiling::all_functions]
impl ThreadWorldContainer {
    /// Sets the thread context to the given value
    pub fn set_context(&mut self, world: ThreadScriptContext<'static>) {
        WORLD_CALLBACK_ACCESS.with(|w| {
            w.replace(Some(world));
        });
    }

    /// Tries to get the world from the container
    pub fn try_get_context<'l>(&self) -> Result<ThreadScriptContext<'l>, DynWorldAccessError> {
        WORLD_CALLBACK_ACCESS
            .with(|w| {
                w.borrow()
                    .clone()
                    .ok_or_else(DynWorldAccessError::missing_world)
            })
            .map(|v| ThreadScriptContext {
                world: v.world.shorten_lifetime(),
                // attachment: v.attachment,
            })
    }
}

#[cfg(test)]
mod test {
    use std::array;

    use super::*;

    #[derive(Default)]
    struct TestRegistry;
    impl CachedRegistry for TestRegistry {
        const SLOT: usize = 0;
    }

    #[test]
    fn test_scoped_handle_invalidate_doesnt_invalidate_parent() {
        let mut world = World::new();
        let world = WorldAccessGuard::new_exclusive(
            &mut world,
            array::from_fn(|_| Rc::new(RefCell::new(TestRegistry)) as Rc<RefCell<dyn Any>>),
        );
        let scoped_world = world.scope();

        // can use scoped & normal worlds
        assert!(scoped_world.is_valid());
        assert!(world.is_valid());
        pretty_assertions::assert_eq!(scoped_world.is_valid(), true);
        pretty_assertions::assert_eq!(world.is_valid(), true);

        scoped_world.invalidate();

        // can only use normal world
        pretty_assertions::assert_eq!(scoped_world.is_valid(), false);
        pretty_assertions::assert_eq!(world.is_valid(), true);
        assert!(world.is_valid());
    }

    #[test]
    fn with_existing_static_guard_does_not_invalidate_original() {
        let mut world = World::new();
        let world = WorldAccessGuard::new_exclusive(
            &mut world,
            array::from_fn(|_| Rc::new(RefCell::new(TestRegistry)) as Rc<RefCell<dyn Any>>),
        );

        let mut sneaky_clone = None;
        WorldAccessGuard::with_existing_static_guard(world.clone(), |g| {
            pretty_assertions::assert_eq!(g.is_valid(), true);
            sneaky_clone = Some(g.clone());
        });
        pretty_assertions::assert_eq!(world.is_valid(), true, "original world was invalidated");
        pretty_assertions::assert_eq!(
            sneaky_clone.map(|c| c.is_valid()),
            Some(false),
            "scoped world was not invalidated"
        );
    }

    #[test]
    fn test_with_access_scope_success() {
        let mut world = World::new();
        let guard = WorldAccessGuard::new_exclusive(
            &mut world,
            array::from_fn(|_| Rc::new(RefCell::new(TestRegistry)) as Rc<RefCell<dyn Any>>),
        );

        // within the access scope, no extra accesses are claimed
        let result = unsafe { guard.with_access_scope(|| 100) };
        assert_eq!(result.unwrap(), 100);
    }
}
