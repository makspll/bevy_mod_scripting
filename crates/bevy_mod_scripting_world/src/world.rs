//! # Motivation
//!
//! Traits and structs needed to support the creation of bindings for scripting languages.
//! reflection gives us access to `dyn PartialReflect` objects via their type name,
//! Scripting languages only really support `Clone` objects so if we want to support references,
//! we need wrapper types which have owned and ref variants.

use crate::{AccessByteSet, AccessMap, ArchetypeFilter, WorldAccessRange};

use ::bevy_ecs::{
    component::ComponentId,
    entity::Entity,
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};
use bevy_ecs::{
    component::Component, reflect::AppTypeRegistry, resource::Resource, system::Command,
    world::WorldId,
};
use bevy_reflect::TypeRegistryArc;
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    cell::{Cell, Ref, RefCell},
    fmt::Debug,
    rc::Rc,
    sync::Arc,
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
    invalid: Rc<Cell<bool>>,
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
pub type RegistryCache = [Rc<RefCell<dyn Any>>; 6];

/// A scope in which values converted from scripts may borrow from the world, see [`WorldAccessGuard::with_call_scope`].
///
/// Accesses claimed through the scope are released when it ends, which is why borrows from it can't outlive it.
///
/// A scope made by [`CallScope::with_lazy`] looks its world up the first time it's needed, so calls whose arguments never
/// touch the world (numbers, native values) don't.
pub struct CallScope<'w> {
    guard: std::cell::OnceCell<WorldGuard<'w>>,
    /// The claims made through this scope, released when it's dropped
    claims: RefCell<SmallVec<[WorldAccessRange; 4]>>,
    /// The copies of values outside the world made through this scope, see [`Self::copy_value`], written back and
    /// dropped when it's dropped
    copies: RefCell<SmallVec<[ValueCopy; 4]>>,
    /// Room for the first copies, the others are allocated
    slots: std::cell::UnsafeCell<[std::mem::MaybeUninit<CopySlot>; COPY_SLOTS]>,
    slots_used: std::cell::Cell<usize>,
}

/// How many copies a [`CallScope`] holds without allocating.
const COPY_SLOTS: usize = 4;

/// Room for a copy of most values (vectors, quaternions, matrices, transforms).
#[repr(C, align(16))]
struct CopySlot([std::mem::MaybeUninit<u8>; 64]);

/// A copy of a value, see [`CallScope::copy_value`].
struct ValueCopy {
    copy: *mut u8,
    layout: std::alloc::Layout,
    /// The slot of the scope the copy is in, if it wasn't allocated. Dropping the scope finds the copy by it: pointers
    /// into the scope made earlier are no longer valid then
    slot: Option<usize>,
    /// Where the copy is written back to, for values the call may change
    origin: Option<*mut u8>,
    drop: Option<unsafe fn(*mut u8)>,
}

impl<'w> CallScope<'w> {
    fn new(guard: Option<WorldGuard<'w>>) -> Self {
        Self {
            guard: guard.map(std::cell::OnceCell::from).unwrap_or_default(),
            claims: Default::default(),
            copies: Default::default(),
            slots: std::cell::UnsafeCell::new(
                [const { std::mem::MaybeUninit::uninit() }; COPY_SLOTS],
            ),
            slots_used: Default::default(),
        }
    }

    /// Copies a value outside the world, i.e. one stored by a script language, and returns the copy, which lives as long
    /// as this scope and is never aliased. Scripts can write such a value at any time, also while the call runs (a
    /// callback), so calls never hold references to them. With `write_back` the copy replaces the value when the scope
    /// is dropped (the last one wins if a value is copied twice).
    ///
    /// `clone` makes the copy (the bytes are copied without one), `drop` drops a value of the type, if it needs to be.
    ///
    /// `None` if the copy can't be allocated.
    ///
    /// # Safety
    /// - `value` is a valid value of `layout`, for reads and, with `write_back`, for writes, until the scope is dropped.
    ///   Without `clone` it needn't be aligned.
    /// - `clone` writes a valid copy of its first argument to its second, `drop` drops a value of the type.
    pub unsafe fn copy_value(
        &self,
        value: *mut u8,
        layout: std::alloc::Layout,
        clone: Option<unsafe fn(*const u8, *mut u8)>,
        drop: Option<unsafe fn(*mut u8)>,
        write_back: bool,
    ) -> Option<*mut u8> {
        if layout.size() == 0 {
            // nothing to copy or write back, any aligned pointer is a value
            return Some(layout.align() as *mut u8);
        }
        let used = self.slots_used.get();
        let fits =
            layout.size() <= size_of::<CopySlot>() && layout.align() <= align_of::<CopySlot>();
        let (copy, slot) = if fits && used < COPY_SLOTS {
            self.slots_used.set(used + 1);
            // Safety: inside the slots, each slot is handed out once
            (
                unsafe { (self.slots.get() as *mut CopySlot).add(used) } as *mut u8,
                Some(used),
            )
        } else {
            // Safety: the layout isn't empty
            let copy = unsafe { std::alloc::alloc(layout) };
            if copy.is_null() {
                return None;
            }
            (copy, None)
        };
        // Safety: the caller's, the copy is new memory of the layout
        unsafe {
            match clone {
                Some(clone) => clone(value, copy),
                None => std::ptr::copy_nonoverlapping(value, copy, layout.size()),
            }
        }
        self.copies.borrow_mut().push(ValueCopy {
            copy,
            layout,
            slot,
            origin: write_back.then_some(value),
            drop,
        });
        Some(copy)
    }

    /// Runs `f` in a new scope of the running script's world ([`ThreadWorldContainer`]), looked up the first time the
    /// scope needs it, releasing the accesses claimed in it once `f` returns or panics.
    ///
    /// Without a running script, the scope's world is an error once it's needed (and claims fail).
    pub fn with_lazy<O>(f: impl FnOnce(&CallScope<'w>) -> O) -> O {
        // dropped also when `f` panics
        let scope = CallScope::new(None);
        f(&scope)
    }

    /// The world, looked up for a lazy scope, see [`Self::with_lazy`].
    fn guard(&self) -> Result<&WorldGuard<'w>, DynWorldAccessError> {
        if let Some(guard) = self.guard.get() {
            return Ok(guard);
        }
        let world = ThreadWorldContainer::try_get_context()?.world;
        Ok(self.guard.get_or_init(|| world))
    }

    /// The world guard this scope belongs to, an error for a lazy scope without a running script, see [`Self::with_lazy`].
    pub fn world(&self) -> Result<WorldGuard<'w>, DynWorldAccessError> {
        self.guard().cloned()
    }

    /// Claims read access for a borrow that lives as long as this scope.
    #[track_caller]
    pub fn claim_read_access(&self, key: impl Into<WorldAccessRange>) -> bool {
        self.claim(key.into(), false)
    }

    /// Claims write access for a borrow that lives as long as this scope, see [`Self::claim_read_access`].
    #[track_caller]
    pub fn claim_write_access(&self, key: impl Into<WorldAccessRange>) -> bool {
        self.claim(key.into(), true)
    }

    #[track_caller]
    fn claim(&self, key: WorldAccessRange, write: bool) -> bool {
        // nothing can be claimed without a world
        let Ok(guard) = self.guard() else {
            return false;
        };
        let claimed = if write {
            guard.claim_write_access(key.clone())
        } else {
            guard.claim_read_access(key.clone())
        };
        if claimed {
            self.claims.borrow_mut().push(key);
        }
        claimed
    }
}

impl Drop for CallScope<'_> {
    fn drop(&mut self) {
        // claims are only made with a world
        if let Some(guard) = self.guard.get() {
            for key in self.claims.get_mut().drain(..).rev() {
                // Safety: the borrows made under the claim can't outlive the scope
                unsafe { guard.release_access(key) }
            }
        }
        let slots = self.slots.get() as *mut CopySlot;
        for copy in self.copies.get_mut().drain(..) {
            // Safety: `copy_value` made the copy with this layout, in this slot if it has one; its caller keeps the
            // origin valid until now
            unsafe {
                let value = match copy.slot {
                    Some(slot) => slots.add(slot) as *mut u8,
                    None => copy.copy,
                };
                match (copy.origin, copy.drop) {
                    // the copy replaces the value: the old one is dropped, the copy moved
                    (Some(origin), drop) => {
                        if let Some(drop) = drop {
                            drop(origin);
                        }
                        std::ptr::copy_nonoverlapping(value, origin, copy.layout.size());
                    }
                    (None, Some(drop)) => drop(value),
                    (None, None) => {}
                }
                if copy.slot.is_none() {
                    std::alloc::dealloc(copy.copy, copy.layout);
                }
            }
        }
    }
}

/// Invalidates a static guard when dropped, also when its closure panics
struct InvalidateOnDrop(WorldGuard<'static>);

impl Drop for InvalidateOnDrop {
    fn drop(&mut self) {
        self.0.invalidate();
    }
}

/// Used to decrease the stack size of [`WorldAccessGuard`]
pub(crate) struct WorldAccessGuardInner<'w> {
    /// Safety: cannot be used unless the scope depth is less than the max valid scope
    cell: UnsafeWorldCell<'w>,
    // TODO: this is fairly hefty, explore sparse sets, bit fields etc
    pub(crate) accesses: AccessMap,
    /// Per-entity restrictions on top of `accesses`, set for non-exclusive systems with filtered queries
    archetype_filter: Option<Arc<ArchetypeFilter>>,
    /// Cached for convenience, since we need it for most operations, means we don't need to lock the type registry every time
    type_registry: TypeRegistryArc,

    cached_slots: RegistryCache,
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
        // the guard is covariant in its lifetime
        self
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

    /// Claims the given access key, runs `f` and releases the claim, also when `f` panics.
    #[track_caller]
    fn with_access<O>(
        &self,
        key: WorldAccessRange,
        write: bool,
        f: impl FnOnce() -> O,
    ) -> Result<O, DynWorldAccessError> {
        /// Releases the claim when dropped
        struct Release<'a, 'w>(&'a WorldAccessGuard<'w>, Option<WorldAccessRange>);
        impl Drop for Release<'_, '_> {
            fn drop(&mut self) {
                if let Some(key) = self.1.take() {
                    // Safety: the claim was made in `with_access` and the borrows made under it ended with `f`
                    unsafe { self.0.release_access(key) }
                }
            }
        }

        let claimed = if write {
            self.inner.accesses.claim_write_access(key.clone())
        } else {
            self.inner.accesses.claim_read_access(key.clone())
        };
        if !claimed {
            let msg = if write {
                "Could not claim write access"
            } else {
                "Could not claim read access"
            };
            return Err(DynWorldAccessError::cannot_claim_access(key, msg));
        }
        let _release = Release(self, Some(key));
        Ok(f())
    }

    /// Procures and releases the given access key allowing safe read access to world resources
    /// requiring that key within the given closure
    #[track_caller]
    pub fn with_read_access<F: FnOnce() -> O, O>(
        &self,
        key: impl Into<WorldAccessRange>,
        f: F,
    ) -> Result<O, DynWorldAccessError> {
        self.with_access(key.into(), false, f)
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
        self.with_access(key.into(), false, f).map_err(Into::into)?
    }

    /// Procures and releases the given access key allowing safe write access to world resources
    /// requiring that key within the given closure
    #[track_caller]
    pub fn with_write_access<F: FnOnce() -> O, O>(
        &self,
        key: impl Into<WorldAccessRange>,
        f: F,
    ) -> Result<O, DynWorldAccessError> {
        self.with_access(key.into(), true, f)
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
        self.with_access(key.into(), true, f).map_err(Into::into)?
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
            // the world may change structurally: views are stale
            crate::bump_structural_epoch();
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
        self.with_write_access_and_then(WorldAccessRange::Global, || {
            let cell = self.as_unsafe_world_cell().map_err(Into::into)?;
            // the world may change structurally: views are stale
            crate::bump_structural_epoch();
            // Safety: we have exclusive access
            f(unsafe { cell.world_mut() })
        })
    }

    /// Returns true if the guard is valid, false if it is invalid
    fn is_valid(&self) -> bool {
        !self.invalid.get()
    }

    /// Invalidates the world access guard, making it and any guards derived from this one unusable.
    pub fn invalidate(&self) {
        self.invalid.set(true);
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
        let _invalidate = InvalidateOnDrop(static_guard.clone());
        let _restore = ThreadWorldContainer::replace_context(ThreadScriptContext {
            world: static_guard.clone(),
        });
        f(static_guard)
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
        let _invalidate = InvalidateOnDrop(static_guard.clone());
        let _restore = ThreadWorldContainer::replace_context(ThreadScriptContext {
            world: static_guard.clone(),
        });
        f(static_guard)
    }

    /// Creates a new [`WorldAccessGuard`] from a possibly non-exclusive access to the world.
    ///
    /// It requires specyfing the exact accesses that are allowed to be given out by the guard.
    /// Those accesses need to be safe to be given out to the script, as the guard will assume that it is safe to give them out in any way.
    ///
    /// The `archetype_filter` further restricts component accesses to entities in the given archetypes, see [`ArchetypeFilter`].
    ///
    /// # Safety
    /// - The caller must ensure that the accesses in subset (narrowed by the archetype filter) are not aliased by any other access
    /// - If an access is allowed in this subset, but alised by someone else,
    pub unsafe fn new_non_exclusive(
        world: UnsafeWorldCell<'w>,
        subset: AccessByteSet,
        archetype_filter: Option<Arc<ArchetypeFilter>>,
        type_registry: TypeRegistryArc,
        registry_cache: RegistryCache,
    ) -> Self {
        Self {
            inner: Rc::new(WorldAccessGuardInner {
                cell: world,
                accesses: AccessMap::new_subset(subset),
                archetype_filter,
                type_registry,
                cached_slots: registry_cache,
            }),
            invalid: Rc::new(false.into()),
        }
    }

    /// Checks the guard's [`ArchetypeFilter`] allows reading, or if `write` writing, the component on the given entity.
    ///
    /// Must be checked before handing out references to components, on top of claiming the component's access.
    /// Missing entities pass, as there is nothing to alias.
    #[inline]
    pub fn check_archetype_access(
        &self,
        entity: Entity,
        component_id: ComponentId,
        write: bool,
    ) -> Result<(), DynWorldAccessError> {
        let Some(filter) = &self.inner.archetype_filter else {
            return Ok(());
        };
        // only reads entity metadata, which cannot change while a non-exclusive system runs
        let Ok(entity_cell) = self.as_unsafe_world_cell()?.get_entity(entity) else {
            return Ok(());
        };
        if filter.is_allowed(component_id, entity_cell.archetype().id(), write) {
            Ok(())
        } else {
            let access = if write { "writing" } else { "accessing" };
            Err(DynWorldAccessError::cannot_claim_access(
                component_id.into(),
                format!(
                    "entity {entity} is not matched by any query of this system {access} the component"
                ),
            ))
        }
    }

    /// The address of the component `component_id` of `entity`, or of the resource `component_id` if `entity` is
    /// `None`, for a view of the context of `epoch` (see [`crate::RowView`]). `None` if there is none.
    ///
    /// Access is checked like for a reference (claimed and released, and the archetype filter): for writing if `write`
    /// and the guard allows it, else for reading, when the view is read-only (no changed tick). Writing views of
    /// mutable columns mark them changed ([`AtomicTick`](bevy_ecs::change_detection::AtomicTick) summary).
    ///
    /// # Safety
    /// The address may be used only by code which, before each use, checks both epochs of the view are unchanged
    /// (see [`crate::bump_structural_epoch`], [`crate::ViewEpoch`]), which reads it as the component's type, writes only
    /// fields where every bit pattern is a valid value (or which it writes valid values to), writes only if the view
    /// has a changed tick, and then writes `this_run` (see [`crate::ViewEpoch::this_run_ptr`]) to it.
    pub unsafe fn row_view(
        &self,
        epoch: &crate::ViewEpoch,
        entity: Option<Entity>,
        component_id: ComponentId,
        write: bool,
    ) -> Result<Option<crate::RowView>, DynWorldAccessError> {
        let cell = self.as_unsafe_world_cell()?;
        let Some(info) = cell.components().get_info(component_id) else {
            return Ok(None);
        };
        let entity = match entity {
            Some(entity) => {
                // a write view where the system may write, else a read-only one
                let write = write && info.mutable();
                if write
                    && self
                        .check_archetype_access(entity, component_id, true)
                        .is_ok()
                {
                    return self.claimed_row_view(epoch, entity, component_id, info, true);
                }
                self.check_archetype_access(entity, component_id, false)?;
                return self.claimed_row_view(epoch, entity, component_id, info, false);
            }
            // Safety: only the id of the resource's entity is read
            None => match unsafe { cell.resource_entities() }.get(component_id) {
                Some(entity) => entity,
                None => return Ok(None),
            },
        };
        let write = write && info.mutable();
        self.claimed_row_view(epoch, entity, component_id, info, write)
    }

    /// [`Self::row_view`] once the archetype filter allows it: claims access (for writing if `write`, falling back to
    /// reading) and finds the address.
    fn claimed_row_view(
        &self,
        epoch: &crate::ViewEpoch,
        entity: Entity,
        component_id: ComponentId,
        info: &bevy_ecs::component::ComponentInfo,
        write: bool,
    ) -> Result<Option<crate::RowView>, DynWorldAccessError> {
        use bevy_ecs::component::StorageType;
        let cell = self.as_unsafe_world_cell()?;
        let find = |write: bool| {
            let Ok(entity_cell) = cell.get_entity(entity) else {
                return None;
            };
            let location = entity_cell.location();
            // Safety: access to the component is claimed for this closure, the storages are only read
            let storages = unsafe { cell.storages() };
            let (data, changed, summary) = match info.storage_type() {
                StorageType::Table => {
                    let table = storages.tables.get(location.table_id)?;
                    let changed = table.get_changed_tick(component_id, location.table_row)?;
                    // Safety: the row is in the table, `get_changed_tick` checked it
                    let data = unsafe { table.get_component(component_id, location.table_row)? };
                    (data, changed, table.get_summary_tick(component_id))
                }
                StorageType::SparseSet => {
                    let (data, ticks) = storages
                        .sparse_sets
                        .get(component_id)?
                        .get_with_ticks(entity)?;
                    (data, ticks.changed, ticks.summary_tick)
                }
            };
            let this_run = cell.change_tick();
            if write {
                epoch.set_this_run(this_run.get());
                if let Some(summary) = summary {
                    summary.set(this_run);
                }
            }
            Some(crate::RowView {
                // Safety: the claimed access is exclusive for writes (as Bevy's own `get_mut`); a read-only view is never
                // written through
                data: unsafe { data.assert_unique() }.as_ptr(),
                changed_tick: if write {
                    changed.get().cast()
                } else {
                    std::ptr::null_mut()
                },
                epoch: epoch.get(),
                structural: crate::structural_epoch_value(),
            })
        };
        if write && let Ok(view) = self.with_access(component_id.into(), true, || find(true)) {
            return Ok(view);
        }
        self.with_access(component_id.into(), false, || find(false))
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
                archetype_filter: None,
                cached_slots: registry_cache,
                type_registry,
            }),
            invalid: Rc::new(false.into()),
        }
    }

    /// Whether this guard was created from exclusive access to the world, i.e. nothing else can be running in parallel with it.
    ///
    /// Guards for systems running in parallel with others, such as non-exclusive script systems, only give access to a subset of the world.
    pub fn is_exclusive(&self) -> bool {
        self.inner.accesses.is_exclusive()
    }

    /// Queues a command to the world, which will be executed later.
    ///
    /// Requires exclusive world access.
    pub(crate) fn queue(&self, command: impl Command) -> Result<(), DynWorldAccessError> {
        self.with_world_mut_access(|w| {
            w.commands().queue(command);
        })
    }

    /// Runs `f` in a new [`CallScope`], releasing the accesses claimed in it once `f` returns or panics.
    ///
    /// Values converted from scripts borrow from the world through the scope, so they can't outlive it:
    ///
    /// ```
    /// # use bevy_mod_scripting_world::WorldGuard;
    /// fn f(guard: WorldGuard) -> WorldGuard { guard.with_call_scope(|scope| scope.world().unwrap()) }
    /// ```
    ///
    /// ```compile_fail
    /// # use bevy_mod_scripting_world::WorldGuard;
    /// fn f(guard: WorldGuard) { let _ = guard.with_call_scope(|scope| scope); }
    /// ```
    pub fn with_call_scope<O>(&self, f: impl FnOnce(&CallScope<'w>) -> O) -> O {
        // dropped also when `f` panics
        let scope = CallScope::new(Some(self.clone()));
        f(&scope)
    }

    /// Gets the component id of the given component or resource
    pub fn get_component_id(&self, id: TypeId) -> Result<Option<ComponentId>, DynWorldAccessError> {
        Ok(self.as_unsafe_world_cell()?.components().get_id(id))
    }

    /// Gets the resource id of the given component or resource
    pub fn get_resource_id(&self, id: TypeId) -> Result<Option<ComponentId>, DynWorldAccessError> {
        let cell = self.as_unsafe_world_cell()?;
        Ok(cell
            .components()
            .get_id(id)
            // Safety: we are not updating the cache
            .and_then(|id| {
                unsafe { cell.resource_entities() }
                    .get(id)
                    .is_some()
                    .then_some(id)
            }))
    }

    fn resource_component_id<R: Resource>(&self) -> Result<ComponentId, DynWorldAccessError> {
        self.as_unsafe_world_cell()?
            .components()
            .get_id(TypeId::of::<R>())
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
        new_guard.invalid = Rc::new(Cell::new(new_guard.invalid.get()));
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

    /// Purely debugging utility to list all accesses currently held.
    pub fn list_accesses(&self) -> Vec<(WorldAccessRange, bool)> {
        self.inner.accesses.list_accesses()
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

    /// Replaces the registry cached in this world guard with the given one, returning the previous value.
    pub fn replace_cached_registry<T: CachedRegistry>(&self, registry: T) -> T {
        let idx = T::SLOT;
        let mut mutt = RefCell::borrow_mut(&self.inner.cached_slots[idx]);

        #[allow(
            clippy::unwrap_used,
            reason = "internal domain boundary, enforced at creation of the guard"
        )]
        let mutt = mutt.downcast_mut().unwrap();
        std::mem::replace(mutt, registry)
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

/// Restores the previous [`ThreadWorldContainer`] context when dropped, see [`ThreadWorldContainer::replace_context`].
pub struct RestoreThreadContext(Option<ThreadScriptContext<'static>>);

impl Drop for RestoreThreadContext {
    fn drop(&mut self) {
        let previous = self.0.take();
        WORLD_CALLBACK_ACCESS.with(|w| {
            w.replace(previous);
        });
    }
}

thread_local! {
    static WORLD_CALLBACK_ACCESS: RefCell<Option<ThreadScriptContext<'static>>> = const { RefCell::new(None) };
}
#[profiling::all_functions]
impl ThreadWorldContainer {
    /// Sets the thread context to the given value until the returned guard is dropped, at which point the previous context is restored.
    ///
    /// This keeps nested static guards (e.g. a callback running another callback) from leaving the outer callback with an invalidated world.
    #[must_use = "the previous context is restored as soon as the guard is dropped"]
    pub fn replace_context(world: ThreadScriptContext<'static>) -> RestoreThreadContext {
        RestoreThreadContext(WORLD_CALLBACK_ACCESS.with(|w| w.replace(Some(world))))
    }

    /// Tries to get the world from the container
    pub fn try_get_context<'l>() -> Result<ThreadScriptContext<'l>, DynWorldAccessError> {
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

        // also when the closure panics
        let sneaky_clone = RefCell::new(None);
        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            WorldAccessGuard::with_existing_static_guard(world.clone(), |g| {
                *sneaky_clone.borrow_mut() = Some(g.clone());
                panic!("callback failed");
            })
        }));
        assert!(panicked.is_err());
        pretty_assertions::assert_eq!(
            sneaky_clone.into_inner().map(|c| c.is_valid()),
            Some(false),
            "scoped world was not invalidated after a panic"
        );
    }

    #[test]
    fn nested_static_guard_restores_outer_thread_context() {
        let mut world = World::new();
        let world = WorldAccessGuard::new_exclusive(
            &mut world,
            array::from_fn(|_| Rc::new(RefCell::new(TestRegistry)) as Rc<RefCell<dyn Any>>),
        );

        let thread_world_is_valid = || {
            ThreadWorldContainer::try_get_context()
                .map(|c| c.world.is_valid())
                .unwrap_or(false)
        };

        WorldAccessGuard::with_existing_static_guard(world.clone(), |outer| {
            WorldAccessGuard::with_existing_static_guard(outer.clone(), |_| {
                assert!(thread_world_is_valid());
            });
            assert!(
                thread_world_is_valid(),
                "outer thread context was left pointing at the invalidated inner guard"
            );
        });
    }

    #[test]
    fn test_with_access_scope_success() {
        let mut world = World::new();
        let guard = WorldAccessGuard::new_exclusive(
            &mut world,
            array::from_fn(|_| Rc::new(RefCell::new(TestRegistry)) as Rc<RefCell<dyn Any>>),
        );

        // within the access scope, no extra accesses are claimed
        let result = guard.with_call_scope(|_| 100);
        assert_eq!(result, 100);

        let key = || WorldAccessRange::ComponentOrResource(ComponentId::new(0));
        guard.with_call_scope(|outer| {
            assert!(outer.claim_read_access(key()));
            guard.with_call_scope(|inner| {
                // each scope only releases its own claims
                assert!(outer.claim_read_access(key()));
                assert!(inner.claim_read_access(key()));
            });
            assert_eq!(guard.access_len(), 2);
        });
        // claims are released when their scope ends
        assert_eq!(guard.access_len(), 0);

        // a lazy scope looks its world up when needed: without a running script there's none, nothing is claimed
        CallScope::with_lazy(|scope| {
            assert!(scope.world().is_err());
            assert!(!scope.claim_read_access(key()));
        });
        // with one, claims are made and released like in other scopes
        let mut running = World::new();
        let slots = array::from_fn(|_| Rc::new(RefCell::new(TestRegistry)) as Rc<RefCell<dyn Any>>);
        WorldAccessGuard::with_static_guard(&mut running, slots, |running| {
            CallScope::with_lazy(|scope| {
                assert!(scope.claim_write_access(key()));
                assert_eq!(running.access_len(), 1);
            });
            assert_eq!(running.access_len(), 0);
        });

        // also when it panics
        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            guard.with_call_scope(|scope| {
                assert!(scope.claim_write_access(key()));
                panic!("in call")
            })
        }));
        assert!(panicked.is_err());
        assert_eq!(guard.access_len(), 0);

        // failing closures release their claim too
        let fail = || Err::<(), _>(DynWorldAccessError::missing_world());
        assert!(guard.with_read_access_and_then(key(), fail).is_err());
        assert!(guard.with_write_access_and_then(key(), fail).is_err());
        assert_eq!(guard.access_len(), 0);

        // and so do panicking ones, outside of a call scope
        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            guard.with_write_access(key(), || panic!("in closure"))
        }));
        assert!(panicked.is_err());
        assert_eq!(guard.access_len(), 0);
    }

    #[test]
    fn row_views_write_in_place_and_go_stale() {
        use bevy_ecs::{prelude::*, query::Changed};

        #[derive(Component)]
        struct Position(f32);
        #[derive(Component)]
        #[component(immutable)]
        struct Fixed(f32);

        let mut world = World::new();
        let a = world.spawn((Position(1.0), Fixed(2.0))).id();
        let b = world.spawn((Position(3.0), Fixed(4.0))).id();
        let position = world.register_component::<Position>();
        let fixed = world.register_component::<Fixed>();
        let mut changed = world.query_filtered::<Entity, Changed<Position>>();
        changed.iter(&world).count();
        world.increment_change_tick();

        let epoch = crate::ViewEpoch::default();
        let guard = WorldAccessGuard::new_exclusive(
            &mut world,
            array::from_fn(|_| Rc::new(RefCell::new(TestRegistry)) as Rc<RefCell<dyn Any>>),
        );
        // Safety: written as a `Position` (an `f32` at offset 0), with its tick, while both epochs are unchanged
        unsafe {
            let view = guard
                .row_view(&epoch, Some(b), position, true)
                .unwrap()
                .unwrap();
            assert!(!view.changed_tick.is_null(), "a write view");
            *(view.data as *mut f32) = 5.0;
            *view.changed_tick = *epoch.this_run_ptr();
            assert_eq!(
                (view.epoch, view.structural),
                (epoch.get(), crate::structural_epoch_value())
            );

            // immutable components are read-only
            let view = guard
                .row_view(&epoch, Some(a), fixed, true)
                .unwrap()
                .unwrap();
            assert!(view.changed_tick.is_null(), "a read-only view");
            assert_eq!(*(view.data as *const f32), 2.0);

            // claims are released after making a view
            assert_eq!(guard.access_len(), 0);

            // structural changes and entering script code make views stale
            let structural = crate::structural_epoch_value();
            guard.with_world_mut_access(|w| w.despawn(a)).unwrap();
            assert_ne!(crate::structural_epoch_value(), structural);
            let before = epoch.get();
            epoch.enter(|| assert_ne!(epoch.get(), before));
            assert_ne!(epoch.get(), before);
            assert!(
                guard
                    .row_view(&epoch, Some(a), position, true)
                    .unwrap()
                    .is_none(),
                "gone"
            );
        }
        drop(guard);
        assert_eq!(world.get::<Position>(b).unwrap().0, 5.0);
        assert_eq!(world.get::<Fixed>(b).unwrap().0, 4.0);
        // the tick write is seen by change detection
        assert_eq!(changed.iter(&world).collect::<Vec<_>>(), vec![b]);
    }
}
