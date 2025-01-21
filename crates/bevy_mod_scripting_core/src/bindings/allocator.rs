use allocator_api2::alloc::{Allocator, Global};
use bevy::reflect::{PartialReflect, Reflect};
use blink_alloc::SyncBlinkAlloc;
use std::alloc::Layout;
use std::mem::ManuallyDrop;
use std::{cell::UnsafeCell, cmp::Ordering, sync::Arc};

// TODO: we should build a custom allocator which uses information about our domain
// and then avoid having to check for the addresses in the deallocation queue when allocating

/// A special allocator that defers deallocations until a specific point in time.
pub struct DeferredAllocator {
    arena: SyncBlinkAlloc,
}

impl DeferredAllocator {
    pub(crate) const fn new() -> Self {
        Self {
            arena: SyncBlinkAlloc::new(),
        }
    }
}

static DEFERRED_ALLOCATOR: DeferredAllocator = DeferredAllocator {
    arena: SyncBlinkAlloc::new(),
};

unsafe impl Allocator for DeferredAllocator {
    fn allocate(
        &self,
        layout: Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, allocator_api2::alloc::AllocError> {
        self.arena.allocate(layout)
    }

    unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: Layout) {
        // don't need to do anything we're that cool 😎
    }
}

/// A type alias for a boxed value that uses the [`DeferredAllocator`] for memory management.
pub type DeferredBox<T> = allocator_api2::boxed::Box<T, DeferredAllocator>;

pub(crate) fn cleanup_deferred_allocations() {
    // safety: we are the only ones allowed to call this function
    // we call this at very specific points in time to avoid any aliasing issues
    unsafe { DEFERRED_ALLOCATOR.arena.reset() };
}

pub trait CoerceDeferredBox {
    fn coerce_deferred_box(self: Box<Self>) -> DeferredBox<dyn PartialReflect>;
}

impl<T: PartialReflect + Sized> CoerceDeferredBox for DeferredBox<T> {
    fn coerce_deferred_box(self: Box<Self>) -> DeferredBox<dyn PartialReflect> {
        // copies box bitwise into the arena
        let layout = Layout::for_value(&self);

        // one exception to the rule, running out of memory is a valid reason to panic
        #[allow(clippy::expect_used)]
        let memory = DEFERRED_ALLOCATOR
            .arena
            .allocate(layout)
            .expect("Failed to allocate memory");

        let old_ptr = Box::into_raw(self);

        let new_memory_ptr = memory.as_ptr().cast::<DeferredBox<T as dyn PartialReflect>>();
        unsafe {
            std::ptr::copy_nonoverlapping(old_ptr, new_memory_ptr, layout.size());
        }
        // free the old memory
        unsafe { Global.deallocate(std::ptr::NonNull::new_unchecked(old_ptr.cast()), layout) };

        // safety: we have copied the value into the arena
        unsafe { DeferredBox::from_raw_in(new_memory_ptr, DEFERRED_ALLOCATOR) }
    }
}

#[derive(Clone, Debug)]
pub struct ReflectAllocation2(Arc<DeferredBox<UnsafeCell<dyn PartialReflect>>>);

impl ReflectAllocation2 {
    pub fn unique_id(&self) -> usize {
        self.0.as_ref().get() as *const () as usize
    }

    /// Gets a reference to the value
    ///
    /// # Safety
    /// The caller must ensure that no mutable references to the value exist at the same time as this reference is active
    /// The caller must also ensure that deferred alocations are not deallocated while this reference is active
    pub unsafe fn get<'l>(&self) -> &'l dyn PartialReflect {
        &*self.0.as_ref().get()
    }

    /// Gets a mutable reference to the value
    ///
    /// # Safety
    /// The caller must ensure that no other references to the value exist at the same time as this reference is active
    /// The caller must also ensure that deferred alocations are not deallocated while this reference is active
    pub unsafe fn get_mut<'l>(&self) -> &'l mut dyn PartialReflect {
        &mut *self.0.as_ref().get()
    }

    /// Creates a new [`ReflectAllocation2`] from a boxed value
    pub fn new_boxed(value: DeferredBox<dyn PartialReflect>) -> Self {
        let transmuted: DeferredBox<UnsafeCell<dyn PartialReflect>> =
            unsafe { std::mem::transmute(value) };

        // I believe this is a false positive, as we do impl Send and Sync at top level
        // i.e. we do need the Arc, I just didn't want to bother with a sync wrapper on the unsafe cell
        #[allow(clippy::arc_with_non_send_sync)]
        Self(Arc::new(transmuted))
    }

    /// Creates a new [`ReflectAllocation2`] from a concrete value
    pub fn new<T: PartialReflect + Sized + 'static>(value: T) -> Self {
        Self::new_boxed(DeferredBox::new_in(value, DEFERRED_ALLOCATOR))
    }

    /// Takes the value out of the allocation if it's the only reference to it.
    /// Otherwise, returns the allocation back.
    pub fn take(self) -> Result<DeferredBox<dyn PartialReflect>, Self> {
        match Arc::try_unwrap(self.0) {
            Ok(inner) => {
                let transmuted: DeferredBox<dyn PartialReflect> =
                    unsafe { std::mem::transmute(inner) };
                Ok(transmuted)
            }
            Err(arc) => Err(Self(arc)),
        }
    }

    /// Takes the value out of the allocation if it's the only reference to it and the underlying value is [`Reflect`].
    ///
    /// # Safety
    /// The caller must ensure no references to the value exist at the same time as this function is called.
    pub unsafe fn take_if_reflect(self) -> Result<Box<dyn Reflect>, Self> {
        match self.take() {
            Ok(partial) => partial.try_into_reflect().map_err(Self::new_boxed),
            Err(e) => Err(e),
        }
    }

    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.0)
    }
}

impl PartialEq for ReflectAllocation2 {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(self.0.get(), other.0.get())
    }
}

impl Eq for ReflectAllocation2 {}

impl PartialOrd for ReflectAllocation2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ReflectAllocation2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.get().cast::<()>().cmp(&other.0.get().cast::<()>())
    }
}

unsafe impl Send for ReflectAllocation2 {}
unsafe impl Sync for ReflectAllocation2 {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_allocator() {
        let allocator = DeferredAllocator::new();
        let layout = Layout::from_size_align(1, 1).unwrap();
        let ptr = allocator.allocate(layout).unwrap();
        let address = ptr.as_ptr() as *mut () as usize;
        unsafe { allocator.deallocate(ptr.cast(), layout) };

        cleanup_deferred_allocations();

        let ptr = allocator.allocate(layout).unwrap();
        let new_address = ptr.as_ptr() as *mut () as usize;
        assert_eq!(address, new_address);
    }

    #[test]
    #[cfg(miri)]
    fn miri_test_allocator() {
        // we test that we don't get any errors when running miri after deallocating Arcs if we cleanup
        // allocations after references are gone

        let allocation = DeferredBox::new_in(UnsafeCell::new(123usize).into(), DeferredAllocator);

        let ptr = allocation.get();

        // we deref but drop the box as well
        let _value = unsafe { *ptr };
        unsafe { drop(allocation) };

        // do something with the value
        assert_eq!(unsafe { *ptr }, 123);

        // cleanup allocations
        cleanup_deferred_allocations();

        // no errors should be thrown
    }
}
