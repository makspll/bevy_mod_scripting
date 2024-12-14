use bevy::ecs::system::Resource;
use bevy::reflect::{PartialReflect, Reflect};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::any::{Any, TypeId};
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct ReflectAllocationId(pub(crate) Arc<usize>);
impl ReflectAllocationId {
    pub fn id(&self) -> usize {
        *self.0
    }

    /// Creates a new [`ReflectAllocationId`] from its id
    pub(crate) fn new(id: usize) -> Self {
        Self(Arc::new(id))
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
#[derive(Clone, Debug)]
pub enum ReflectAllocation {
    Double(Arc<OwningPtr<dyn PartialReflect>>),
    Single(Arc<UnsafeCell<dyn PartialReflect>>),
}

unsafe impl Send for ReflectAllocation {}
unsafe impl Sync for ReflectAllocation {}

impl ReflectAllocation {
    pub fn get_ptr(&self) -> *mut dyn PartialReflect {
        match self {
            ReflectAllocation::Double(v) => v.ptr,
            ReflectAllocation::Single(v) => v.get(),
        }
    }
    pub fn new(value: Arc<UnsafeCell<dyn PartialReflect>>) -> Self {
        Self::Single(value)
    }

    pub fn new_boxed(value: Box<dyn PartialReflect>) -> Self {
        let ptr = Box::leak::<'static>(value);
        Self::Double(Arc::new(unsafe { OwningPtr::new(ptr) }))
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
    types: HashMap<usize, TypeId>,
}

impl ReflectAllocator {
    /// Allocates a new [`Reflect`] value and returns an [`AllocationId`] which can be used to access it later.
    /// Use [`Self::allocate_boxed`] if you already have an allocated boxed value.
    pub fn allocate<T: PartialReflect>(
        &mut self,
        value: T,
    ) -> (ReflectAllocationId, ReflectAllocation) {
        let type_id = value.get_represented_type_info().map(|i| i.type_id());

        let id = ReflectAllocationId::new(self.allocations.len());
        let index = id.id();
        let value = ReflectAllocation::new(Arc::new(UnsafeCell::new(value)));
        self.allocations.insert(id.clone(), value.clone());
        if let Some(type_id) = type_id {
            self.types.insert(index, type_id);
        }
        (id, value)
    }

    pub fn allocate_boxed(
        &mut self,
        value: Box<dyn PartialReflect>,
    ) -> (ReflectAllocationId, ReflectAllocation) {
        let type_id = value.get_represented_type_info().map(|i| i.type_id());

        let id = ReflectAllocationId::new(self.allocations.len());
        let index = id.id();
        let value = ReflectAllocation::new_boxed(value);
        self.allocations.insert(id.clone(), value.clone());
        if let Some(type_id) = type_id {
            self.types.insert(index, type_id);
        }
        (id, value)
    }

    // /// Moves the given boxed [`PartialReflect`] value into the allocator, returning an [`AllocationId`] which can be used to access it later
    // pub fn allocate_boxed(
    //     &mut self,
    //     existing: Box<dyn PartialReflect>,
    // ) -> (ReflectAllocationId, ReflectAllocation) {
    //     let type_id = existing.get_represented_type_info().map(|i| i.type_id());
    //     let id = ReflectAllocationId(self.allocations.len());

    //     let raw_ptr = Box::into_raw(existing);
    //     // Safety:
    //     // - we are the only ones to have access to this value since we have the Box
    //     // - UnsafeCell is repr(transparent), meaning we can safely transmute between it and the trait object
    //     // TODO: I don't think we can use this, because from_raw has a pre-condition that requires the pointer to have been an arc before
    //     let arc: Arc<UnsafeCell<dyn PartialReflect>> =
    //         unsafe { Arc::from_raw(raw_ptr as *const _) };
    //     let allocation = ReflectAllocation::new(arc);
    //     self.allocations.insert(id, allocation.clone());
    //     if let Some(type_id) = type_id {
    //         self.types.insert(id, type_id);
    //     }
    //     (id, allocation)
    // }

    pub fn get(&self, id: &ReflectAllocationId) -> Option<ReflectAllocation> {
        self.allocations.get(id).cloned()
    }

    pub fn get_type_id(&self, id: &ReflectAllocationId) -> Option<TypeId> {
        self.types.get(&id.id()).cloned()
    }

    pub fn get_mut(&self, id: &ReflectAllocationId) -> Option<ReflectAllocation> {
        self.allocations.get(id).cloned()
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reflect_allocator_garbage_clean() {
        let mut allocator = ReflectAllocator::default();
        let (id, _) = allocator.allocate(0);
        assert_eq!(allocator.allocations.len(), 1);
        drop(id);
        allocator.clean_garbage_allocations();
        assert_eq!(allocator.allocations.len(), 0);
    }

    #[test]
    fn test_reflect_allocator_garbage_clean_no_garbage() {
        let mut allocator = ReflectAllocator::default();
        let (_id, _) = allocator.allocate(0);
        assert_eq!(allocator.allocations.len(), 1);
        allocator.clean_garbage_allocations();
        assert_eq!(allocator.allocations.len(), 1);
    }
}
