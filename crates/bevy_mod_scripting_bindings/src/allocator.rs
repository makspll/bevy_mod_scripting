//! Values allocated for scripts, i.e. values returned by functions, owned by the references pointing to them.

use ::{
    bevy_app::{App, Plugin, PostUpdate},
    bevy_diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic},
    bevy_reflect::PartialReflect,
};
use bevy_mod_scripting_display::{
    DebugWithTypeInfo, DebugWithTypeInfoBuilder, DisplayWithTypeInfo,
};
use bevy_mod_scripting_world::{AccessOwner, WorldAccessRange, WorldGuard};
use std::{
    any::Any,
    cell::UnsafeCell,
    cmp::Ordering,
    fmt::{Display, Formatter},
    hash::Hasher,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering::Relaxed},
    },
};

/// The path used for the total number of allocations diagnostic
pub const ALLOCATOR_TOTAL_DIAG_PATH: DiagnosticPath =
    DiagnosticPath::const_new("scripting_allocator_total");

/// The path used for the number of allocations freed since the last measurement
pub const ALLOCATOR_TOTAL_COLLECTED_DIAG_PATH: DiagnosticPath =
    DiagnosticPath::const_new("scripting_allocator_total_collected");

/// The number of live allocations
static LIVE: AtomicUsize = AtomicUsize::new(0);
/// The number of allocations freed since the last [`measure_allocations`]
static FREED: AtomicUsize = AtomicUsize::new(0);

/// A value allocated for scripts and its unique id. Clones share the value, which is freed with the last of them.
///
/// Accesses to the value are claimed in the world's access map under [`WorldAccessRange::External`] with the id,
/// a claim made with [`Self::owner`] keeps the value alive until it's released, see [`crate::ReflectReference::reflect_unsafe`].
#[derive(Clone)]
pub struct ReflectAllocationId {
    /// Kept next to the pointer, so access keys don't need to read the allocation
    id: usize,
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

impl<T> Drop for Inline<T> {
    fn drop(&mut self) {
        freed();
    }
}

impl Drop for Boxed {
    fn drop(&mut self) {
        freed();
    }
}

/// Counts a freed allocation
fn freed() {
    LIVE.fetch_sub(1, Relaxed);
    FREED.fetch_add(1, Relaxed);
}

/// Hands out unique allocation ids
fn next_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    LIVE.fetch_add(1, Relaxed);
    COUNTER.fetch_add(1, Relaxed)
}

impl ReflectAllocationId {
    /// Allocates a value, in a single heap allocation together with its id.
    pub fn new<T: PartialReflect>(value: T) -> Self {
        Self {
            id: next_id(),
            value: Arc::new(Inline(UnsafeCell::new(value))),
        }
    }

    /// Allocates a value which is already boxed.
    pub fn new_boxed(value: Box<dyn PartialReflect>) -> Self {
        // Safety: `UnsafeCell<T>` has the same layout as `T`
        let value: Box<UnsafeCell<dyn PartialReflect>> = unsafe { std::mem::transmute(value) };
        Self {
            id: next_id(),
            value: Arc::new(Boxed(value)),
        }
    }

    /// Returns the id of the allocation
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns a pointer to the allocated value.
    ///
    /// Creating references from it is only safe while holding the matching claim in the world's access map,
    /// and while the allocation is alive (i.e. by keeping this id, or claiming with [`Self::owner`]).
    pub fn get_ptr(&self) -> *mut dyn PartialReflect {
        self.value.get_ptr()
    }

    /// Returns the number of ids pointing to this allocation, including owners held by claims
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.value)
    }

    /// Something keeping the allocation alive, for claims which must keep the value alive until they're released.
    pub fn owner(&self) -> AccessOwner {
        // the same allocation, upcast
        self.value.clone()
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

/// Measures the number of live allocations and the number freed since the last measurement
pub fn measure_allocations(mut diagnostics: Diagnostics) {
    diagnostics.add_measurement(&ALLOCATOR_TOTAL_DIAG_PATH, || LIVE.load(Relaxed) as f64);
    diagnostics.add_measurement(&ALLOCATOR_TOTAL_COLLECTED_DIAG_PATH, || {
        FREED.swap(0, Relaxed) as f64
    });
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
    fn owners_and_clones_share_the_allocation() {
        let id = ReflectAllocationId::new(1usize);
        let owner = id.owner();
        let clone = id.clone();
        assert_eq!(id.strong_count(), 3);
        drop(owner);
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
