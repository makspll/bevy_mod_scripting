use bevy::ecs::system::Resource;
use bevy::reflect::Reflect;

use std::sync::Arc;

use std::collections::HashMap;

pub type AllocationId = usize;

/// Allocator used to allocate and deallocate `dyn Reflect` values
/// Used to be able to ensure we have a "common root" for values allocated outside the world.
#[derive(Resource, Default)]
pub struct ReflectAllocator {
    // TODO: experiment with object pools, sparse set etc.
    pub allocations: HashMap<AllocationId, Arc<dyn Reflect>>,
}

impl ReflectAllocator {
    /// Allocates a new [`Reflect`] value and returns an [`AllocationId`] which can be used to access it later
    pub fn allocate(&mut self, value: Arc<dyn Reflect>) -> AllocationId {
        let id = self.allocations.len();
        self.allocations.insert(id, value);
        id
    }

    /// Deallocates the [`Reflect`] value with the given [`AllocationId`]
    pub fn deallocate(&mut self, id: AllocationId) {
        self.allocations.remove(&id);
    }

    /// Runs a garbage collection pass on the allocations, removing any allocations which have no more strong references
    /// Needs to be run periodically to prevent memory leaks
    pub fn clean_garbage_allocations(&mut self) {
        self.allocations.retain(|_, v| Arc::strong_count(v) > 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reflect_allocator() {
        let mut allocator = ReflectAllocator::default();
        let value = Arc::new(0);
        allocator.allocate(value.clone());
        assert_eq!(allocator.allocations.len(), 1);
        drop(value);
        allocator.clean_garbage_allocations();
        assert_eq!(allocator.allocations.len(), 0);
    }
}
