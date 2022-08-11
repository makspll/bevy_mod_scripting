use std::sync::Arc;

use bevy::prelude::{World};
use parking_lot::{RwLock, MappedRwLockReadGuard, RwLockReadGuard, MappedRwLockWriteGuard, RwLockWriteGuard};


/// Pointer to a bevy world, safely allows multiple access via RwLock
/// # Safety
/// This pointer does not prevent dangling pointers, i.e. you must ensure the world is not dropped while any world pointers still exist,
/// the world must also not change, from the moment a world pointer is created it must always point to the same world.
#[derive(Debug,Clone)]
pub struct WorldPointer(Arc<RwLock<*mut World>>);


unsafe impl Send for WorldPointer {}
unsafe impl Sync for WorldPointer {}

impl WorldPointer {
    /// Creates a new world pointer.
    /// # Safety
    /// satisfies world constancy, since it's impossible to change the underlying pointer
    /// However you must ensure that the world does not go out of scope while this pointer is live    
    pub unsafe fn new(world: &mut World) -> Self{
        WorldPointer(Arc::new(RwLock::new(world)))
    }

    /// Returns a read guard which can be used for immutable world access.
    pub fn read(&self) -> MappedRwLockReadGuard<World> {
        RwLockReadGuard::map(self.0.try_read().expect(""), |ptr : &*mut World| {
            unsafe{&**ptr}
        })
    }

    /// Returns a write guard which can be used for mutable world access.
    pub fn write(&self) -> MappedRwLockWriteGuard<World> {
        RwLockWriteGuard::map(self.0.try_write().expect(""), |ptr : &mut *mut World| {
            unsafe{&mut **ptr}
        })
    }
}