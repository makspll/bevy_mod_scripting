use bevy::ecs::system::Resource;
use bevy::reflect::Reflect;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReflectAllocationId(pub(self) usize);
impl ReflectAllocationId {
    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct ReflectAllocation(pub(self) Arc<UnsafeCell<dyn Reflect>>);

unsafe impl Send for ReflectAllocation {}
unsafe impl Sync for ReflectAllocation {}

impl ReflectAllocation {
    pub fn get_ptr(&self) -> *mut dyn Reflect {
        self.0.get()
    }
    pub fn new(value: Arc<UnsafeCell<dyn Reflect>>) -> Self {
        Self(value)
    }
}

impl Display for ReflectAllocationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Allocator used to allocate and deallocate `dyn Reflect` values
/// Used to be able to ensure we have a "common root" for values allocated outside the world.
#[derive(Resource, Default)]
pub struct ReflectAllocator {
    // TODO: experiment with object pools, sparse set etc.
    allocations: HashMap<ReflectAllocationId, ReflectAllocation>,
}

impl ReflectAllocator {
    /// Allocates a new [`Reflect`] value and returns an [`AllocationId`] which can be used to access it later
    pub fn allocate(&mut self, value: ReflectAllocation) -> ReflectAllocationId {
        let id = ReflectAllocationId(self.allocations.len());
        self.allocations.insert(id, value);
        id
    }

    pub fn get(&self, id: ReflectAllocationId) -> Option<ReflectAllocation> {
        self.allocations.get(&id).cloned()
    }

    pub fn get_mut(&self, id: ReflectAllocationId) -> Option<ReflectAllocation> {
        self.allocations.get(&id).cloned()
    }

    /// Deallocates the [`Reflect`] value with the given [`AllocationId`]
    pub fn deallocate(&mut self, id: ReflectAllocationId) {
        self.allocations.remove(&id);
    }

    /// Runs a garbage collection pass on the allocations, removing any allocations which have no more strong references
    /// Needs to be run periodically to prevent memory leaks
    pub fn clean_garbage_allocations(&mut self) {
        self.allocations.retain(|_, v| Arc::strong_count(&v.0) > 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reflect_allocator() {
        let mut allocator = ReflectAllocator::default();
        let value = ReflectAllocation::new(Arc::new(UnsafeCell::new(0)));
        allocator.allocate(value.clone());
        assert_eq!(allocator.allocations.len(), 1);
        drop(value);
        allocator.clean_garbage_allocations();
        assert_eq!(allocator.allocations.len(), 0);
    }
}
