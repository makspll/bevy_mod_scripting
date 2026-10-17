//! An allocator used to control the lifetime of allocations

use ::{
    bevy_app::{App, Plugin, PostUpdate},
    bevy_diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic},
    bevy_ecs::system::Res,
    bevy_reflect::PartialReflect,
};
use bevy_ecs::{resource::Resource, system::ResMut};
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_mod_scripting_display::{
    DebugWithTypeInfo, DebugWithTypeInfoBuilder, DisplayWithTypeInfo,
};
use bevy_mod_scripting_world::{WorldAccessRange, WorldGuard};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{
    cell::UnsafeCell,
    cmp::Ordering,
    fmt::{Display, Formatter},
    hash::Hasher,
    sync::{Arc, atomic::AtomicUsize},
};

/// The path used for the total number of allocations diagnostic
pub const ALLOCATOR_TOTAL_DIAG_PATH: DiagnosticPath =
    DiagnosticPath::const_new("scripting_allocator_total");

/// The path used for the total number of deallocated allocations diagnostic
pub const ALLOCATOR_TOTAL_COLLECTED_DIAG_PATH: DiagnosticPath =
    DiagnosticPath::const_new("scripting_allocator_total_collected");

/// Unique identifier for an allocation, which also holds the allocated value.
///
/// The [`ReflectAllocator`] keeps every allocation alive until a garbage collection pass finds no other ids pointing to it.
#[derive(Clone)]
pub struct ReflectAllocationId(pub(crate) Arc<AllocationEntry>);

/// The id and value of an allocation
pub(crate) struct AllocationEntry {
    id: usize,
    value: ReflectAllocation,
}

impl DebugWithTypeInfo for ReflectAllocationId {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&WorldGuard>,
    ) -> std::fmt::Result {
        f.debug_tuple_with_type_info("ReflectAllocationId", type_info_provider)
            .field(&self.id())
            .finish()
    }
}

impl From<&ReflectAllocationId> for WorldAccessRange {
    fn from(val: &ReflectAllocationId) -> Self {
        WorldAccessRange::External(val.id())
    }
}

impl DisplayWithTypeInfo for ReflectAllocationId {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _type_info_provider: Option<&WorldGuard>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

impl ReflectAllocationId {
    /// Returns the id of the allocation
    pub fn id(&self) -> usize {
        self.0.id
    }

    /// The allocated value
    pub fn allocation(&self) -> &ReflectAllocation {
        &self.0.value
    }

    /// Creates an id holding a value which is not kept alive by an allocator, i.e. for placeholders.
    pub(crate) fn unallocated<T: PartialReflect>(value: T) -> Self {
        Self(Arc::new(AllocationEntry {
            id: usize::MAX,
            value: ReflectAllocation::new(Box::new(value)),
        }))
    }

    /// Returns the number of strong references to this id
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.0)
    }
}

impl std::hash::Hash for ReflectAllocationId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq for ReflectAllocationId {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for ReflectAllocationId {}

impl PartialOrd for ReflectAllocationId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ReflectAllocationId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(&other.id())
    }
}

/// Pointer which owns the value it points to, and will deallocate it when dropped
#[derive(Debug)]
pub struct OwningPtr<T: ?Sized> {
    ptr: *mut T,
}

impl<T: ?Sized> OwningPtr<T> {
    /// Creates a new OwningPtr from a raw pointer
    /// # Safety
    /// The pointer must come from a Box::leak call, and no more than one OwningPtr can exist for a given pointer
    pub unsafe fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }
}

impl<T: ?Sized> Drop for OwningPtr<T> {
    fn drop(&mut self) {
        unsafe {
            // Safety: we own the pointer, only one OwningPtr can exist for a given pointer
            let _ = Box::from_raw(self.ptr);
        }
    }
}
// yikes, the indirection. I need this to store boxed values too though

/// A boxed [`PartialReflect`] value
pub struct ReflectAllocation(Box<UnsafeCell<dyn PartialReflect>>);

impl DebugWithTypeInfo for ReflectAllocation {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&WorldGuard>,
    ) -> std::fmt::Result {
        f.debug_tuple_with_type_info("ReflectAllocation", type_info_provider)
            .field(&((self.0.get() as *mut ()) as usize))
            .finish()
    }
}

// unsafe impl Send for ReflectAllocation {}
unsafe impl Sync for ReflectAllocation {}

impl ReflectAllocation {
    /// Returns a pointer to the [`PartialReflect`] value
    pub fn get_ptr(&self) -> *mut dyn PartialReflect {
        self.0.as_ref().get()
    }

    /// Creates a new [`ReflectAllocation`] from a boxed [`PartialReflect`] value
    pub fn new(value: Box<dyn PartialReflect>) -> Self {
        let value: Box<UnsafeCell<dyn PartialReflect>> = unsafe { std::mem::transmute(value) };
        Self(value)
    }

    /// Takes the value out of the allocation.
    ///
    /// # Safety
    /// - Must only be done if no other references to this allocation exist at the same time
    pub unsafe fn take(self) -> Box<dyn PartialReflect> {
        unsafe { std::mem::transmute(self.0) }
    }
}

impl Display for ReflectAllocationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

/// Wrapper around a [`ReflectAllocator`] which can be freely copied and shared between threads
#[derive(Resource, Clone, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub struct AppReflectAllocator {
    pub(crate) allocator: Arc<RwLock<ReflectAllocator>>,
}

impl Default for AppReflectAllocator {
    fn default() -> Self {
        Self {
            allocator: Arc::new(RwLock::new(ReflectAllocator::default())),
        }
    }
}

impl AppReflectAllocator {
    /// claim a read lock on the allocator
    pub fn read(&self) -> RwLockReadGuard<'_, ReflectAllocator> {
        self.allocator.read()
    }

    /// claim a write lock on the allocator
    pub fn write(&self) -> RwLockWriteGuard<'_, ReflectAllocator> {
        self.allocator.write()
    }
}

/// Allocator used to allocate and deallocate `dyn PartialReflect` values
/// Used to be able to ensure we have a "common root" for values allocated outside the world.
#[derive(Default, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub struct ReflectAllocator {
    /// Keeps the allocations alive until [`Self::clean_garbage_allocations`] finds them unused,
    /// the values are reached through their ids.
    allocations: Vec<ReflectAllocationId>,
}

#[profiling::all_functions]
impl ReflectAllocator {
    /// Allocates a new `Reflect` value and returns an [`ReflectAllocationId`] which can be used to access it later.
    /// Use [`Self::allocate_boxed`] if you already have an allocated boxed value.
    pub fn allocate<T: PartialReflect>(&mut self, value: T) -> ReflectAllocationId {
        self.allocate_boxed(Box::new(value))
    }

    /// Allocates a new boxed `PartialReflect` value and returns an [`ReflectAllocationId`] which can be used to access it later.
    pub fn allocate_boxed(&mut self, value: Box<dyn PartialReflect>) -> ReflectAllocationId {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        let id = ReflectAllocationId(Arc::new(AllocationEntry {
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            value: ReflectAllocation::new(value),
        }));
        self.allocations.push(id.clone());
        id
    }

    /// Get the reflect value with a given id
    pub fn get<'a>(&self, id: &'a ReflectAllocationId) -> Option<&'a ReflectAllocation> {
        Some(id.allocation())
    }

    /// Runs a garbage collection pass on the allocations, removing any allocations which have no more strong references
    /// Needs to be run periodically to prevent memory leaks
    pub fn clean_garbage_allocations(&mut self) {
        self.allocations.retain(|id| id.strong_count() > 1);
        // don't keep scanning the capacity of a past spike of allocations
        if self.allocations.len() * 4 < self.allocations.capacity() {
            self.allocations.shrink_to(self.allocations.len() * 2);
        }
    }

    /// Returns an iterator over all allocations
    pub fn iter_allocations(
        &self,
    ) -> impl Iterator<Item = (&ReflectAllocationId, &ReflectAllocation)> {
        self.allocations.iter().map(|id| (id, id.allocation()))
    }
}

/// Cleans up dangling script allocations
#[profiling::function]
pub fn garbage_collector(allocator: ResMut<AppReflectAllocator>, mut diagnostics: Diagnostics) {
    let mut allocator = allocator.write();
    let before = allocator.allocations.len();
    allocator.clean_garbage_allocations();
    let after = allocator.allocations.len();
    diagnostics.add_measurement(&ALLOCATOR_TOTAL_DIAG_PATH, || after as f64);
    diagnostics.add_measurement(&ALLOCATOR_TOTAL_COLLECTED_DIAG_PATH, || {
        (before - after) as f64
    });
}

/// Measures the number of allocations in the allocator and other diagnostics when enabled
pub fn measure_allocations(allocator: Res<AppReflectAllocator>, mut diagnostics: Diagnostics) {
    let allocator = allocator.read();
    let allocations_count = allocator.allocations.len();
    diagnostics.add_measurement(&ALLOCATOR_TOTAL_DIAG_PATH, || allocations_count as f64);
}

/// A plugin which registers various allocator diagnostics
pub struct AllocatorDiagnosticPlugin;
impl Plugin for AllocatorDiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app.register_diagnostic(Diagnostic::new(ALLOCATOR_TOTAL_DIAG_PATH).with_suffix(" allocs"))
            .register_diagnostic(
                Diagnostic::new(ALLOCATOR_TOTAL_COLLECTED_DIAG_PATH).with_suffix(" deallocs"),
            )
            .add_systems(PostUpdate, measure_allocations);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reflect_allocator_garbage_clean() {
        let mut allocator = ReflectAllocator::default();
        let id = allocator.allocate(0);
        assert_eq!(allocator.allocations.len(), 1);
        drop(id);
        allocator.clean_garbage_allocations();
        assert_eq!(allocator.allocations.len(), 0);
    }

    #[test]
    fn test_reflect_allocator_allocate_clean_and_access_does_not_overwrite_id() {
        let mut allocator = ReflectAllocator::default();
        let id = allocator.allocate(0);
        let id2 = allocator.allocate("string");
        assert_eq!(allocator.allocations.len(), 2);
        drop(id);
        allocator.clean_garbage_allocations();
        assert_eq!(allocator.allocations.len(), 1);
        allocator.allocate(3);
        assert_eq!(allocator.allocations.len(), 2);

        // Safety: only one reference to the allocation exists
        let ref_ = unsafe { &*allocator.get(&id2).unwrap().get_ptr() };
        assert!(ref_.reflect_partial_eq(&"string").unwrap());
    }

    #[test]
    fn test_reflect_allocator_garbage_clean_no_garbage() {
        let mut allocator = ReflectAllocator::default();
        let _id = allocator.allocate(0);
        assert_eq!(allocator.allocations.len(), 1);
        allocator.clean_garbage_allocations();
        assert_eq!(allocator.allocations.len(), 1);
    }
}
