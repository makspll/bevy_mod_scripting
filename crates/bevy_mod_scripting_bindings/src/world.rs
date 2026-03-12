//! # Motivation
//!
//! Traits and structs needed to support the creation of bindings for scripting languages.
//! reflection gives us access to `dyn PartialReflect` objects via their type name,
//! Scripting languages only really support `Clone` objects so if we want to support references,
//! we need wrapper types which have owned and ref variants.

use super::{
    access_map::{
        AccessCount, AccessMapKey, AnyAccessMap, DynamicSystemMeta, ReflectAccessId,
        ReflectAccessKind, SubsetAccessMap,
    },
    with_global_access,
};
use crate::{with_access_read, with_access_write};
use ::{
    bevy_app::AppExit,
    bevy_ecs::{
        component::{Component, ComponentId},
        entity::Entity,
        prelude::Resource,
        system::Commands,
        world::{CommandQueue, Mut, World, unsafe_world_cell::UnsafeWorldCell},
    },
};
use bevy_ecs::{
    component::Mutable,
    hierarchy::{ChildOf, Children},
    system::Command,
    world::WorldId,
};
use bevy_mod_scripting_script::ScriptAttachment;
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    fmt::Debug,
    panic::Location,
    rc::Rc,
    sync::atomic::AtomicBool,
};

/// Prefer to directly using [`WorldAccessGuard`]. If the underlying type changes, this alias will be updated.
pub type WorldGuard<'w> = WorldAccessGuard<'w>;
/// Similar to [`WorldGuard`], but without the arc, use for when you don't need the outer Arc.
pub type WorldGuardRef<'w> = &'w WorldAccessGuard<'w>;

pub enum DynWorldAccessError {
    UnregisteredComponent(String),
    MissingWorld,
    CannotClaimAccess(ReflectAccessId, Option<Location<'static>>, String),
}

impl DynWorldAccessError {
    pub fn unregistered_component_or_resource_type(type_name: &str) -> Self {
        Self::UnregisteredComponent(type_name.to_string())
    }

    pub fn missing_world() -> Self {
        Self::MissingWorld
    }

    pub fn cannot_claim_access(
        id: ReflectAccessId,
        location: Option<Location<'static>>,
        msg: impl ToString,
    ) -> Self {
        Self::CannotClaimAccess(id, location, msg.to_string())
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
    const SLOT: usize;
}

/// Used to decrease the stack size of [`WorldAccessGuard`]
pub(crate) struct WorldAccessGuardInner<'w> {
    /// Safety: cannot be used unless the scope depth is less than the max valid scope
    cell: UnsafeWorldCell<'w>,
    // TODO: this is fairly hefty, explore sparse sets, bit fields etc
    pub(crate) accesses: AnyAccessMap,
    cached_slots: [Rc<dyn Any>; 5],
    // /// Cached for convenience, since we need it for most operations, means we don't need to lock the type registry every time
    // type_registry: TypeRegistryArc,
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
        cached_slots: [Rc<dyn Any>; 5],
        f: impl FnOnce(WorldGuard<'static>) -> O,
    ) -> O {
        let guard = WorldAccessGuard::new_exclusive(world, cached_slots);
        // safety: we invalidate the guard after the closure is called, meaning the world cannot be accessed at all after the 'w lifetime ends
        let static_guard: WorldAccessGuard<'static> = unsafe { std::mem::transmute(guard) };
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
    /// either by being converted to mutable or non mutable reference, this guard will be unsafe.
    pub unsafe fn new_non_exclusive(
        world: UnsafeWorldCell<'w>,
        subset: impl IntoIterator<Item = ReflectAccessId>,
        registry_cache: [Rc<dyn Any>; 5],
    ) -> Self {
        Self {
            inner: Rc::new(WorldAccessGuardInner {
                cell: world,
                accesses: AnyAccessMap::SubsetAccessMap(SubsetAccessMap::new(
                    subset,
                    // allocations live beyond the world, and can be safely accessed
                    |id| ReflectAccessId::from_index(id).kind == ReflectAccessKind::Allocation,
                )),
                cached_slots: registry_cache,
            }),
            invalid: Rc::new(false.into()),
        }
    }

    /// Creates a new [`WorldAccessGuard`] for the given mutable borrow of the world.
    ///
    /// If these resources do not exist, they will be initialized.
    pub fn new_exclusive(world: &'w mut World, registry_cache: [Rc<dyn Any>; 5]) -> Self {
        Self {
            inner: Rc::new(WorldAccessGuardInner {
                cell: world.as_unsafe_world_cell(),
                accesses: AnyAccessMap::UnlimitedAccessMap(Default::default()),
                cached_slots: registry_cache,
            }),
            invalid: Rc::new(false.into()),
        }
    }

    /// Queues a command to the world, which will be executed later.
    pub(crate) fn queue(&self, command: impl Command) -> Result<(), DynWorldAccessError> {
        self.with_global_access(|w| {
            w.commands().queue(command);
        })
    }

    /// Runs a closure within an isolated access scope, releasing leftover accesses, should only be used in a single-threaded context.
    ///
    /// Safety:
    /// - The caller must ensure it's safe to release any potentially locked accesses.
    pub(crate) unsafe fn with_access_scope<O, F: FnOnce() -> O>(
        &self,
        f: F,
    ) -> Result<O, DynWorldAccessError> {
        Ok(self.inner.accesses.with_scope(f))
    }

    /// Purely debugging utility to list all accesses currently held.
    pub fn list_accesses(&self) -> Vec<(ReflectAccessId, AccessCount)> {
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

    /// A utility for running a closure with scoped read access to the given id
    pub fn with_read_access<T: Into<ReflectAccessId>, O, F: FnOnce(&Self) -> O>(
        &self,
        id: T,
        closure: F,
    ) -> Result<O, ()> {
        let id = id.into();
        if self.claim_read_access(id) {
            let out = Ok(closure(self));
            // Safety: just claimed this access
            unsafe { self.release_access(id) };
            out
        } else {
            Err(())
        }
    }

    /// A utility for running a closure with scoped write access to the given id
    pub fn with_write_access<T: Into<ReflectAccessId>, O, F: FnOnce(&Self) -> O>(
        &self,
        id: T,
        closure: F,
    ) -> Result<O, ()> {
        let id = id.into();
        if self.claim_write_access(id) {
            let out = Ok(closure(self));
            // Safety: just claimed this access
            unsafe { self.release_access(id) };
            out
        } else {
            Err(())
        }
    }

    /// Get the location of the given access
    pub fn get_access_location(
        &self,
        raid: ReflectAccessId,
    ) -> Option<std::panic::Location<'static>> {
        self.inner.accesses.access_location(raid)
    }

    #[track_caller]
    /// Claims read access to the given type.
    pub fn claim_read_access(&self, raid: ReflectAccessId) -> bool {
        self.inner.accesses.claim_read_access(raid)
    }

    #[track_caller]
    /// Claims write access to the given type.
    pub fn claim_write_access(&self, raid: ReflectAccessId) -> bool {
        self.inner.accesses.claim_write_access(raid)
    }

    /// Releases read or write access to the given type.
    ///
    /// # Safety
    /// - This can only be called safely after all references to the type created using the access have been dropped
    /// - You can only call this if you previously called one of: [`WorldAccessGuard::claim_read_access`] or [`WorldAccessGuard::claim_write_access`]
    /// - The number of claim and release calls for the same id must always match
    pub unsafe fn release_access(&self, raid: ReflectAccessId) {
        self.inner.accesses.release_access(raid)
    }

    /// Claims global access to the world
    pub fn claim_global_access(&self) -> bool {
        self.inner.accesses.claim_global_access()
    }

    /// Releases global access to the world
    ///
    /// # Safety
    /// - This can only be called safely after all references created using the access have been dropped
    pub unsafe fn release_global_access(&self) {
        self.inner.accesses.release_global_access()
    }

    /// Claims access to the world for the duration of the closure, allowing for global access to the world.
    #[track_caller]
    pub fn with_global_access<F: FnOnce(&mut World) -> O, O>(
        &self,
        f: F,
    ) -> Result<O, DynWorldAccessError> {
        with_global_access!(
            &self.inner.accesses,
            "Could not claim exclusive world access",
            {
                // safety: we have global access for the duration of the closure
                let world = unsafe { self.as_unsafe_world_cell()?.world_mut() };
                Ok(f(world))
            }
        )?
    }

    /// Safely accesses the resource by claiming and releasing access to it.
    ///
    /// # Panics
    /// - if the resource does not exist
    pub fn with_resource<F, R, O>(&self, f: F) -> Result<O, DynWorldAccessError>
    where
        R: Resource,
        F: FnOnce(&R) -> O,
    {
        let cell = self.as_unsafe_world_cell()?;
        let access_id = ReflectAccessId::for_resource::<R>(&cell)?;

        with_access_read!(
            &self.inner.accesses,
            access_id,
            format!("Could not access resource: {}", std::any::type_name::<R>()),
            {
                // Safety: we have acquired access for the duration of the closure
                f(unsafe {
                    cell.get_resource::<R>().ok_or_else(|| {
                        DynWorldAccessError::unregistered_component_or_resource_type(
                            std::any::type_name::<R>(),
                        )
                    })?
                })
            }
        )
    }

    /// Safely accesses the resource by claiming and releasing access to it.
    ///
    /// # Panics
    /// - if the resource does not exist
    pub fn with_resource_mut<F, R, O>(&self, f: F) -> Result<O, DynWorldAccessError>
    where
        R: Resource,
        F: FnOnce(Mut<R>) -> O,
    {
        let cell = self.as_unsafe_world_cell()?;
        let access_id = ReflectAccessId::for_resource::<R>(&cell)?;
        with_access_write!(
            &self.inner.accesses,
            access_id,
            format!("Could not access resource: {}", std::any::type_name::<R>()),
            {
                // Safety: we have acquired access for the duration of the closure
                f(unsafe {
                    cell.get_resource_mut::<R>().ok_or_else(|| {
                        DynWorldAccessError::unregistered_component_or_resource_type(
                            std::any::type_name::<R>(),
                        )
                    })?
                })
            }
        )
    }

    /// Safely accesses the component by claiming and releasing access to it.
    pub fn with_component<F, T, O>(&self, entity: Entity, f: F) -> Result<O, DynWorldAccessError>
    where
        T: Component,
        F: FnOnce(Option<&T>) -> O,
    {
        let cell = self.as_unsafe_world_cell()?;
        let access_id = ReflectAccessId::for_component::<T>(&cell)?;
        with_access_read!(
            &self.inner.accesses,
            access_id,
            format!("Could not access component: {}", std::any::type_name::<T>()),
            {
                // Safety: we have acquired access for the duration of the closure
                f(unsafe { cell.get_entity(entity).map(|e| e.get::<T>()) }
                    .ok()
                    .unwrap_or(None))
            }
        )
    }

    /// Safely accesses the component by claiming and releasing access to it.
    pub fn with_component_mut<F, T, O>(
        &self,
        entity: Entity,
        f: F,
    ) -> Result<O, DynWorldAccessError>
    where
        T: Component<Mutability = Mutable>,
        F: FnOnce(Option<Mut<T>>) -> O,
    {
        let cell = self.as_unsafe_world_cell()?;
        let access_id = ReflectAccessId::for_component::<T>(&cell)?;

        with_access_write!(
            &self.inner.accesses,
            access_id,
            format!("Could not access component: {}", std::any::type_name::<T>()),
            {
                // Safety: we have acquired access for the duration of the closure
                f(unsafe { cell.get_entity(entity).map(|e| e.get_mut::<T>()) }
                    .ok()
                    .unwrap_or(None))
            }
        )
    }

    /// Safey modify or insert a component by claiming and releasing global access.
    pub fn with_or_insert_component_mut<F, T, O>(
        &self,
        entity: Entity,
        f: F,
    ) -> Result<O, DynWorldAccessError>
    where
        T: Component<Mutability = Mutable> + Default,
        F: FnOnce(&mut T) -> O,
    {
        self.with_global_access(|world| match world.get_mut::<T>(entity) {
            Some(mut component) => f(&mut component),
            None => {
                let mut component = T::default();
                let mut commands = world.commands();
                let result = f(&mut component);
                commands.entity(entity).insert(component);
                result
            }
        })
    }
}

/// Impl block for higher level world methods
#[profiling::all_functions]
impl WorldAccessGuard<'_> {
    /// If a registry has been initialized in this world guard, downcasts it to its original type and returns
    /// a reference to it
    pub fn get_cached_registry<T: CachedRegistry>(&self) -> Option<&T> {
        let idx = T::SLOT;
        self.inner.cached_slots[idx].downcast_ref()
    }
}

/// A world container that stores the world in a thread local
pub struct ThreadWorldContainer;

#[derive(Clone)]
/// Context passed down indirectly to script related functions, used to avoid prop drilling problems.
pub struct ThreadScriptContext<'l> {
    /// The world pointer
    pub world: WorldGuard<'l>,
    /// The currently active script attachment
    pub attachment: ScriptAttachment,
}

thread_local! {
    static WORLD_CALLBACK_ACCESS: RefCell<Option<ThreadScriptContext<'static>>> = const { RefCell::new(None) };
}
#[profiling::all_functions]
impl ThreadWorldContainer {
    /// Tries to set the thread context to the given value
    pub fn set_context(
        &mut self,
        world: ThreadScriptContext<'static>,
    ) -> Result<(), DynWorldAccessError> {
        WORLD_CALLBACK_ACCESS.with(|w| {
            w.replace(Some(world));
        });
        Ok(())
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
                attachment: v.attachment,
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
            array::from_fn(|_| Rc::new(TestRegistry) as Rc<dyn Any>),
        );
        let scoped_world = world.scope();

        // can use scoped & normal worlds
        scoped_world.spawn().unwrap();
        world.spawn().unwrap();
        pretty_assertions::assert_eq!(scoped_world.is_valid(), true);
        pretty_assertions::assert_eq!(world.is_valid(), true);

        scoped_world.invalidate();

        // can only use normal world
        pretty_assertions::assert_eq!(scoped_world.is_valid(), false);
        pretty_assertions::assert_eq!(world.is_valid(), true);
        world.spawn().unwrap();
    }

    #[test]
    fn with_existing_static_guard_does_not_invalidate_original() {
        let mut world = World::new();
        let world = WorldAccessGuard::new_exclusive(
            &mut world,
            array::from_fn(|_| Rc::new(TestRegistry) as Rc<dyn Any>),
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
            array::from_fn(|_| Rc::new(TestRegistry) as Rc<dyn Any>),
        );

        // within the access scope, no extra accesses are claimed
        let result = unsafe { guard.with_access_scope(|| 100) };
        assert_eq!(result.unwrap(), 100);
    }
}
