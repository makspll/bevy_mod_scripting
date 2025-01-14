use bevy::{ecs::system::Resource, prelude::ResMut, reflect::PartialReflect};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{
    any::TypeId,
    cell::UnsafeCell,
    cmp::Ordering,
    collections::HashMap,
    fmt::{Display, Formatter},
    hash::Hasher,
    sync::{atomic::AtomicU64, Arc},
};

#[derive(Clone, Debug)]
pub struct ReflectAllocationId(pub(crate) Arc<u64>);
impl ReflectAllocationId {
    pub fn id(&self) -> u64 {
        *self.0
    }

    /// Creates a new [`ReflectAllocationId`] from its id
    pub(crate) fn new(id: u64) -> Self {
        Self(Arc::new(id))
    }

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
        Some(self.id().cmp(&other.id()))
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
#[derive(Debug)]
pub struct ReflectAllocation(Box<UnsafeCell<dyn PartialReflect>>);

// unsafe impl Send for ReflectAllocation {}
unsafe impl Sync for ReflectAllocation {}

impl ReflectAllocation {
    pub fn get_ptr(&self) -> *mut dyn PartialReflect {
        self.0.as_ref().get()
    }

    pub fn new(value: Box<dyn PartialReflect>) -> Self {
        let value: Box<UnsafeCell<dyn PartialReflect>> = unsafe { std::mem::transmute(value) };
        Self(value)
    }

    /// Takes the value out of the allocation.
    ///
    /// # Safety
    /// - Must only be done if no other references to this allocation exist at the same time
    pub unsafe fn take(self) -> Box<dyn PartialReflect> {
        std::mem::transmute(self.0)
    }
}

impl Display for ReflectAllocationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Wrapper around a [`ReflectAllocator`] which can be freely copied and shared between threads
#[derive(Debug, Resource, Clone)]
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
    pub fn read(&self) -> RwLockReadGuard<ReflectAllocator> {
        self.allocator.read()
    }

    pub fn write(&self) -> RwLockWriteGuard<ReflectAllocator> {
        self.allocator.write()
    }
}

/// Allocator used to allocate and deallocate `dyn PartialReflect` values
/// Used to be able to ensure we have a "common root" for values allocated outside the world.
#[derive(Default, Debug)]
pub struct ReflectAllocator {
    // TODO: experiment with object pools, sparse set etc.
    allocations: HashMap<ReflectAllocationId, ReflectAllocation>,
    types: HashMap<u64, TypeId>,
}

impl ReflectAllocator {
    /// Allocates a new [`Reflect`] value and returns an [`AllocationId`] which can be used to access it later.
    /// Use [`Self::allocate_boxed`] if you already have an allocated boxed value.
    pub fn allocate<T: PartialReflect>(&mut self, value: T) -> ReflectAllocationId {
        self.allocate_boxed(Box::new(value))
    }

    pub fn allocate_boxed(&mut self, value: Box<dyn PartialReflect>) -> ReflectAllocationId {
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let type_id = value.get_represented_type_info().map(|i| i.type_id());
        let id =
            ReflectAllocationId::new(COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
        let index = id.id();
        let value = ReflectAllocation::new(value);
        self.allocations.insert(id.clone(), value);
        if let Some(type_id) = type_id {
            self.types.insert(index, type_id);
        }
        id
    }
    pub fn insert(
        &mut self,
        id: ReflectAllocationId,
        value: ReflectAllocation,
    ) -> Option<ReflectAllocation> {
        self.allocations.insert(id, value)
    }

    pub fn remove(&mut self, id: &ReflectAllocationId) -> Option<ReflectAllocation> {
        self.allocations.remove(id)
    }

    pub fn get_type_id(&self, id: &ReflectAllocationId) -> Option<TypeId> {
        self.types.get(&id.id()).cloned()
    }

    pub fn get_mut(&mut self, id: &ReflectAllocationId) -> Option<&mut ReflectAllocation> {
        self.allocations.get_mut(id)
    }

    pub fn get(&self, id: &ReflectAllocationId) -> Option<&ReflectAllocation> {
        self.allocations.get(id)
    }

    /// Deallocates the [`PartialReflect`] value with the given [`AllocationId`]
    pub fn deallocate(&mut self, id: &ReflectAllocationId) {
        self.allocations.remove(id);
    }

    /// Runs a garbage collection pass on the allocations, removing any allocations which have no more strong references
    /// Needs to be run periodically to prevent memory leaks
    pub fn clean_garbage_allocations(&mut self) {
        self.allocations.retain(|k, _| Arc::strong_count(&k.0) > 1);
    }

    pub fn iter_allocations(
        &self,
    ) -> impl Iterator<Item = (&ReflectAllocationId, &ReflectAllocation)> {
        self.allocations.iter()
    }
}

/// Cleans up dangling script allocations
pub fn garbage_collector(allocator: ResMut<AppReflectAllocator>) {
    let mut allocator = allocator.write();
    allocator.clean_garbage_allocations()
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
