//! Values allocated for scripts, i.e. values returned by functions, owned by the references pointing to them.

use ::bevy_reflect::PartialReflect;
use bevy_mod_scripting_display::{
    DebugWithTypeInfo, DebugWithTypeInfoBuilder, DisplayWithTypeInfo,
};
use bevy_mod_scripting_world::{WorldAccessRange, WorldGuard};
use std::{
    any::Any,
    cell::UnsafeCell,
    cmp::Ordering,
    fmt::{Display, Formatter},
    hash::Hasher,
    sync::Arc,
};

/// A value allocated for scripts. Clones share the value, which is freed with the last of them.
///
/// Accesses to the value are claimed in the world's access map under [`WorldAccessRange::External`],
/// which holds the value alive until the claim is released.
#[derive(Clone)]
pub struct ReflectAllocationId {
    value: Arc<dyn AllocatedValue>,
}

/// An allocated value.
///
/// Both implementations hand out the pointer from an [`UnsafeCell`], which is the only way of mutating the value
/// through the shared `Arc`; references made from it must follow the claims in the access map.
trait AllocatedValue: Any + Send + Sync {
    fn get_ptr(&self) -> *mut dyn PartialReflect;
}

/// A value of a known type, in the same heap allocation as its reference counts.
struct Inline<T>(UnsafeCell<T>);

// Safety: the value is only accessed through claims in the access map, which never hand out aliasing mutable access.
unsafe impl<T: Send> Sync for Inline<T> {}

impl<T: PartialReflect> AllocatedValue for Inline<T> {
    fn get_ptr(&self) -> *mut dyn PartialReflect {
        self.0.get()
    }
}

/// A value which was already boxed.
struct Boxed(Box<UnsafeCell<dyn PartialReflect>>);

// Safety: as for [`Inline`]
unsafe impl Sync for Boxed {}

impl AllocatedValue for Boxed {
    fn get_ptr(&self) -> *mut dyn PartialReflect {
        // only a shared reference to the box and the cell, the pointer comes from the cell
        UnsafeCell::get(&*self.0)
    }
}

impl ReflectAllocationId {
    /// Allocates a value, in a single heap allocation together with its reference counts.
    pub fn new<T: PartialReflect>(value: T) -> Self {
        Self {
            value: Arc::new(Inline(UnsafeCell::new(value))),
        }
    }

    /// Allocates a value which is already boxed.
    pub fn new_boxed(value: Box<dyn PartialReflect>) -> Self {
        // Safety: `UnsafeCell<T>` has the same layout as `T`
        let value: Box<UnsafeCell<dyn PartialReflect>> = unsafe { std::mem::transmute(value) };
        Self {
            value: Arc::new(Boxed(value)),
        }
    }

    /// Returns the id of the allocation, unique while it's alive: its address, the same id as its access key
    pub fn id(&self) -> usize {
        Arc::as_ptr(&self.value) as *const () as usize
    }

    /// Returns a pointer to the allocated value.
    ///
    /// Creating references from it is only safe while holding the matching claim in the world's access map,
    /// which also keeps the value alive.
    pub fn get_ptr(&self) -> *mut dyn PartialReflect {
        self.value.get_ptr()
    }

    /// Returns the number of ids pointing to this allocation, including the ones held by claims
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.value)
    }
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
        // the same allocation, upcast
        WorldAccessRange::External(val.value.clone())
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

impl Display for ReflectAllocationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn access_keys_and_clones_share_the_allocation() {
        let id = ReflectAllocationId::new(1usize);
        let key = WorldAccessRange::from(&id);
        let clone = id.clone();
        assert_eq!(id.strong_count(), 3);
        // the key identifies the allocation by the same id
        let WorldAccessRange::External(owner) = &key else {
            panic!("allocations are external")
        };
        assert_eq!(bevy_mod_scripting_world::external_access_id(owner), id.id());
        assert_eq!(clone.id(), id.id());
        drop(key);
        drop(clone);
        assert_eq!(id.strong_count(), 1);
    }

    #[test]
    fn inline_and_boxed_values_are_readable() {
        let inline = ReflectAllocationId::new(3usize);
        let boxed = ReflectAllocationId::new_boxed(Box::new(String::from("string")));
        // Safety: no other references to the allocations exist
        unsafe {
            assert!((*inline.get_ptr()).reflect_partial_eq(&3usize).unwrap());
            assert!(
                (*boxed.get_ptr())
                    .reflect_partial_eq(&String::from("string"))
                    .unwrap()
            );
        }
        assert_ne!(inline.id(), boxed.id());
    }
}
