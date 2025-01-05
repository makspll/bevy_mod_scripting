use bevy::prelude::World;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use std::ops::Deref;
use std::sync::Arc;

/// Pointer to a bevy world, safely allows multiple access via RwLock
///
/// If the original [WorldPointerGuard] that created this pointer is dropped,
/// the `read` and `write` methods will panic, and the "try" variants will
/// return `None`.
#[derive(Debug, Clone)]
pub struct WorldPointer(Arc<RwLock<Option<*mut World>>>);

/// Guarded pointer to a bevy world, can be used to `clone` additional
/// [WorldPointer]s for safe access.
///
/// # Safety
/// The original `&mut World` used to create this guard _must not_ be used after
/// the guard is created in order for the cloned [WorldPointer]s to be safe.
///
/// On [Drop], it will "take back" access to the `&mut World`, preventing the
/// `WorldPointer`s from invoking UB.
#[derive(Debug)]
pub struct WorldPointerGuard(WorldPointer);

impl Deref for WorldPointerGuard {
    type Target = WorldPointer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl Send for WorldPointer {}
unsafe impl Sync for WorldPointer {}

impl WorldPointerGuard {
    /// Creates a new world pointer.
    /// # Safety
    /// The original `&mut World` must not be used while this guard is in scope.
    /// The [World] may only be accessed through this guard or one of its cloned
    /// [WorldPointer]s.
    #[allow(clippy::arc_with_non_send_sync)]
    pub unsafe fn new(world: &mut World) -> Self {
        WorldPointerGuard(WorldPointer(Arc::new(RwLock::new(Some(world)))))
    }
}

impl Drop for WorldPointerGuard {
    fn drop(&mut self) {
        // Being explicit about the types here to make sure we're getting things
        // correct.
        let world_ptr: &WorldPointer = &self.0;
        let _: Option<*mut World> = RwLock::write(&world_ptr.0).take();
    }
}

impl WorldPointer {
    /// Returns a read guard which can be used for immutable world access.
    ///
    /// Panics if the pointer is already locked or has gone out of scope.
    pub fn read(&self) -> MappedRwLockReadGuard<World> {
        self.try_read().expect("concurrent read/write world access")
    }

    /// Returns a write guard which can be used for mutable world access.
    ///
    /// Panics if the pointer is already locked or has gone out of scope.
    pub fn write(&self) -> MappedRwLockWriteGuard<World> {
        self.try_write()
            .expect("concurrent read/write world access")
    }

    /// Returns a read guard which can be used for immutable world access.
    ///
    /// Returns `None` if the pointer is already locked or has gone out of
    /// scope.
    pub fn try_read(&self) -> Option<MappedRwLockReadGuard<World>> {
        self.try_read_inner(false)
    }

    /// Returns a write guard which can be used for mutable world access.
    ///
    /// Returns `None` if the pointer is already locked or has gone out of
    /// scope.
    pub fn try_write(&self) -> Option<MappedRwLockWriteGuard<World>> {
        self.try_write_inner(false)
    }

    /// Returns a read guard which can be used for immutable world access.
    ///
    /// Panics if the pointer has gone out of scope. May block if another thread
    /// holds the lock.
    pub fn read_blocking(&self) -> MappedRwLockReadGuard<World> {
        self.try_read_blocking()
            .expect("the world pointer is out of scope")
    }

    /// Returns a write guard which can be used for mutable world access.
    ///
    /// Panics if the pointer has gone out of scope. May block if another thread
    /// holds the lock.
    pub fn write_blocking(&self) -> MappedRwLockWriteGuard<World> {
        self.try_write_blocking()
            .expect("the world pointer is out of scope")
    }

    /// Returns a read guard which can be used for immutable world access.
    ///
    /// Returns `None` if has gone out of scope. May block if another thread
    /// holds the lock.
    pub fn try_read_blocking(&self) -> Option<MappedRwLockReadGuard<World>> {
        self.try_read_inner(true)
    }

    /// Returns a write guard which can be used for mutable world access.
    ///
    /// Returns `None` if has gone out of scope. May block if another thread
    /// holds the lock.
    pub fn try_write_blocking(&self) -> Option<MappedRwLockWriteGuard<World>> {
        self.try_write_inner(true)
    }

    fn try_read_inner(&self, blocking: bool) -> Option<MappedRwLockReadGuard<World>> {
        let guard = if blocking {
            self.0.read()
        } else {
            self.0.try_read()?
        };
        // Check if the inner pointer is there so we can invert the `Option`.
        if guard.is_none() {
            return None;
        }

        Some(RwLockReadGuard::map(
            guard,
            |ptr: &Option<*mut World>| unsafe { &*ptr.unwrap() },
        ))
    }

    fn try_write_inner(&self, blocking: bool) -> Option<MappedRwLockWriteGuard<World>> {
        let guard = if blocking {
            self.0.write()
        } else {
            self.0.try_write()?
        };
        // Check if the inner pointer is there so we can invert the `Option`.
        if guard.is_none() {
            return None;
        }

        Some(RwLockWriteGuard::map(
            guard,
            |ptr: &mut Option<*mut World>| unsafe { &mut *ptr.unwrap() },
        ))
    }
}
