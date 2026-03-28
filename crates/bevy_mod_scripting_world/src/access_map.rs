//! A map of access claims used to safely and dynamically access the world.

use bevy_ecs::component::ComponentId;

use fixedbitset::FixedBitSet;
use parking_lot::Mutex;
use smallvec::SmallVec;

use std::num::NonZero;

#[derive(Debug, Clone, PartialEq, Eq)]
/// An owner of an access claim and the code location of the claim.
pub struct ClaimOwner {
    /// The code location of the claim
    pub location: std::panic::Location<'static>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A count of the number of readers and writers of an access claim.
pub struct AccessInstance {
    /// The number of readers including thread information
    pub owner: ClaimOwner,
    /// If the current read is a write access, this will be set
    written: bool,
}

#[profiling::all_functions]
impl AccessInstance {
    fn new(owner: ClaimOwner, write: bool) -> Self {
        Self {
            owner,
            written: write,
        }
    }
}

/// A wrapper for conversion between ComponentId's and nonzero indices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentRange(NonZero<u16>);

impl From<ComponentId> for ComponentRange {
    fn from(value: ComponentId) -> Self {
        // Safety: trivially holds that n + 1 cannot be zero
        Self(unsafe {
            NonZero::new_unchecked(
                (value.index() as u16)
                    .checked_add(1)
                    .unwrap_or_else(|| unreachable!("Too many components being used")),
            )
        })
    }
}

impl From<ComponentRange> for ComponentId {
    fn from(val: ComponentRange) -> Self {
        ComponentId::new((val.0.get() - 1) as usize)
    }
}
/// Describes access ranges in and outside a bevy world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorldAccessRange {
    /// An access into a resource or component in the world
    ComponentOrResource(ComponentRange),
    /// An access outside the world, for example into an external allocator
    External(NonZero<u64>),
    /// A whole world read or write
    Global,
}

impl WorldAccessRange {
    /// Returns true if this access conflicts with another access in 'space', two conflicting accesses may still be live, if they are read only for example
    pub fn overlaps_with_access_to(&self, other: WorldAccessRange) -> bool {
        match (self, other) {
            (
                WorldAccessRange::ComponentOrResource(a),
                WorldAccessRange::ComponentOrResource(b),
            ) => *a == b,
            (WorldAccessRange::External(a), WorldAccessRange::External(b)) => *a == b,
            (WorldAccessRange::Global, _) => true,
            (_, WorldAccessRange::Global) => true,
            _ => false,
        }
    }
}

impl From<ComponentId> for WorldAccessRange {
    fn from(value: ComponentId) -> Self {
        Self::ComponentOrResource(value.into())
    }
}

// impl<T: Resource> AccessMapKey for T {
//     /// Convert the key to an index
//     fn from_world(&self, world: &UnsafeWorldCell) -> WorldAccessRange {
//         world
//             .components()
//             .component_id::<T>()
//             .map(|c| {
//                 WorldAccessRange::Component(unsafe {
//                     NonZero::new_unchecked((c.index() as u16) + 1)
//                 })
//             })
//             .unwrap_or(WorldAccessRange::Unregistered)
//     }

//     /// Describes the type of access this key represents
//     fn describe(&self) -> String {
//         format!("Component: {}", std::any::type_name::<T>())
//     }
// }

#[derive(Debug, Default)]
/// A map of access claims
pub struct AccessMap(Mutex<AccessMapInner>);

/// A trait for controlling system world access at runtime.
///
/// This trait provides methods to claim and release read, write, and global access
/// to various parts of the world. Implementations of this trait manage internal state
/// to ensure safe and concurrent access to resources. Methods include scope-based locking,
/// as well as introspection of access state via code location information.
pub trait DynamicSystemMeta {
    /// Executes the provided closure within a temporary access scope.
    ///
    /// Any accesses claimed within the scope are rolled back once the closure returns.
    fn with_scope<O, F: FnOnce() -> O>(&self, f: F) -> O;

    /// Attempts to claim read access for the given key.
    ///
    /// Returns `true` if the read access is successfully claimed. The claim will fail if
    /// the key is currently locked for write or if a global lock is active.
    #[track_caller]
    fn claim_read_access<K: Into<WorldAccessRange>>(&self, key: K) -> Result<(), AccessInstance>;

    /// Attempts to claim write access for the given key.
    ///
    /// Returns `true` if the write access is successfully claimed. Write access fails if any
    /// read or write access is active for the key or if a global lock is held.
    #[track_caller]
    fn claim_write_access<K: Into<WorldAccessRange>>(&self, key: K) -> Result<(), AccessInstance>;

    /// Releases an access claimed for the provided key.
    ///
    /// # Panics
    ///
    /// Panics if the access is released by a thread different from the one that claimed it.
    fn release_access<K: Into<WorldAccessRange>>(&self, key: K);

    /// Returns a list of active accesses.
    ///
    /// The list is provided as key and corresponding access count pairs.
    fn list_accesses(&self) -> Vec<(WorldAccessRange, AccessInstance)>;

    /// Returns the number of active individual accesses.
    ///
    /// In the case of a global lock, this method considers that as a single active access.
    fn count_accesses(&self) -> usize;

    /// Releases all active accesses.
    ///
    /// Both individual and global accesses will be removed.
    fn release_all_accesses(&self);
}

#[derive(Default, Debug, Clone)]
struct AccessMapInner {
    individual_accesses: SmallVec<[(WorldAccessRange, AccessInstance); 4]>,
}

#[profiling::all_functions]
impl AccessMapInner {
    #[inline]
    fn overlapping_access(
        &self,
        key: WorldAccessRange,
        instance: &AccessInstance,
    ) -> Option<&AccessInstance> {
        self.individual_accesses
            .iter()
            .find_map(|(entry_key, entry_instance)| {
                let overlaps = key.overlaps_with_access_to(*entry_key);
                let one_is_exclusive = instance.written || entry_instance.written;
                (overlaps && one_is_exclusive).then_some(entry_instance)
            })
    }

    #[inline]
    fn overlapping_access_mut(
        &mut self,
        key: WorldAccessRange,
        instance: &AccessInstance,
    ) -> Option<&mut AccessInstance> {
        self.individual_accesses
            .iter_mut()
            .find_map(|(entry_key, entry_instance)| {
                (key.overlaps_with_access_to(*entry_key)
                    && !(instance.written || entry_instance.written))
                    .then_some(entry_instance)
            })
    }

    #[inline]
    fn insert(&mut self, key: WorldAccessRange, count: AccessInstance) {
        self.individual_accesses.push((key, count));
    }

    #[inline]
    fn clear_access(&mut self, key: WorldAccessRange) {
        let idx = self
            .individual_accesses
            .iter()
            .position(|(entry_key, _)| *entry_key == key);
        if let Some(idx) = idx {
            self.individual_accesses.remove(idx);
        }
    }

    // #[inline]
    // fn entry(&self, key: WorldAccessRange) -> Option<&AccessCount> {
    //     self.individual_accesses
    //         .iter()
    //         .find_map(|(entry_key, access_count)| (key == *entry_key).then_some(access_count))
    // }

    // fn entry_index(&self, key: WorldAccessRange) -> Option<usize> {
    //     self.individual_accesses.iter().position(|(k, _)| *k == key)
    // }

    // #[inline]
    // fn entry_mut(&mut self, key: WorldAccessRange) -> Option<&mut AccessCount> {
    //     self.individual_accesses
    //         .iter_mut()
    //         .find_map(|(entry_key, access_count)| (key == *entry_key).then_some(access_count))
    // }

    // #[inline]
    // fn entry_or_insert_default(&mut self, key: WorldAccessRange) -> &mut AccessCount {
    //     if let Some(i) = self.entry_index(key) {
    //         return &mut self.individual_accesses[i].1;
    //     }

    //     self.individual_accesses.push((key, AccessCount::default()));
    //     // Safety: we just added one element, option is never None
    //     unsafe { &mut self.individual_accesses.last_mut().unwrap_unchecked().1 }
    // }

    // #[inline]
    // fn remove(&mut self, key: WorldAccessRange) {
    //     self.individual_accesses
    //         .retain(|(entry_key, _)| *entry_key != key);
    // }
}

#[profiling::all_functions]
impl DynamicSystemMeta for AccessMap {
    fn release_access<K: Into<WorldAccessRange>>(&self, key: K) {
        let mut inner = self.0.lock();
        let range: WorldAccessRange = key.into();
        inner.clear_access(range);
    }

    fn with_scope<O, F: FnOnce() -> O>(&self, f: F) -> O {
        // Snapshot the current inner state.
        let backup = {
            let inner = self.0.lock();
            inner.clone()
        };

        let result = f();

        // Roll back the inner state.
        {
            let mut inner = self.0.lock();
            *inner = backup;
        }

        result
    }

    #[track_caller]
    fn claim_read_access<K: Into<WorldAccessRange>>(&self, key: K) -> Result<(), AccessInstance> {
        let mut inner = self.0.lock();

        let key = key.into();

        let instance = AccessInstance {
            owner: ClaimOwner {
                location: *std::panic::Location::caller(),
            },
            written: false,
        };

        if let Some(access) = inner.overlapping_access(key, &instance) {
            Err(access.clone())
        } else {
            inner.insert(key, instance);
            Ok(())
        }
    }

    #[track_caller]
    fn claim_write_access<K: Into<WorldAccessRange>>(&self, key: K) -> Result<(), AccessInstance> {
        let mut inner = self.0.lock();

        let key = key.into();

        let instance = AccessInstance {
            owner: ClaimOwner {
                location: *std::panic::Location::caller(),
            },
            written: true,
        };

        if let Some(access) = inner.overlapping_access(key, &instance) {
            Err(access.clone())
        } else {
            inner.insert(key, instance);
            Ok(())
        }
    }

    fn list_accesses(&self) -> Vec<(WorldAccessRange, AccessInstance)> {
        let inner = self.0.lock();
        inner
            .individual_accesses
            .iter()
            .map(|(key, a)| (*key, a.clone()))
            .collect()
    }

    fn count_accesses(&self) -> usize {
        let inner = self.0.lock();
        inner.individual_accesses.len()
    }

    fn release_all_accesses(&self) {
        let mut inner = self.0.lock();
        inner.individual_accesses.clear();
    }
}

/// An inverse of [`AccessMap`], It limits the resource/component accesses allowed to be claimed to those in a pre-specified subset.
pub struct SubsetAccessMap {
    inner: AccessMap,
    component_subset: FixedBitSet,
}

#[profiling::all_functions]
impl SubsetAccessMap {
    /// Creates a new subset access map with the provided subset of ID's as well as a exception function.
    pub fn new(subset: impl IntoIterator<Item = impl Into<WorldAccessRange>>) -> Self {
        let components = subset.into_iter().filter_map(|a| match a.into() {
            WorldAccessRange::ComponentOrResource(range) => Some(range.0.get() as usize),
            _ => None,
        });
        Self {
            inner: Default::default(),
            component_subset: FixedBitSet::from_iter(components),
        }
    }

    fn allowed_access(&self, range: WorldAccessRange) -> bool {
        match range {
            WorldAccessRange::ComponentOrResource(s) => {
                self.component_subset.contains(s.0.get() as usize)
            }
            WorldAccessRange::External(_) => true,
            WorldAccessRange::Global => false,
        }
    }
}

#[profiling::all_functions]
impl DynamicSystemMeta for SubsetAccessMap {
    fn with_scope<O, F: FnOnce() -> O>(&self, f: F) -> O {
        self.inner.with_scope(f)
    }

    fn release_access<K: Into<WorldAccessRange>>(&self, key: K) {
        let key = key.into();
        if !self.allowed_access(key) {
            return;
        }
        self.inner.release_access(key);
    }

    #[track_caller]
    fn claim_read_access<K: Into<WorldAccessRange>>(&self, key: K) -> Result<(), AccessInstance> {
        let key = key.into();
        if !self.allowed_access(key) {
            return Err(AccessInstance {
                owner: ClaimOwner {
                    location: *std::panic::Location::caller(),
                },
                written: true,
            });
        }
        self.inner.claim_read_access(key)
    }

    #[track_caller]
    fn claim_write_access<K: Into<WorldAccessRange>>(&self, key: K) -> Result<(), AccessInstance> {
        let key = key.into();
        if !self.allowed_access(key) {
            return Err(AccessInstance {
                owner: ClaimOwner {
                    location: *std::panic::Location::caller(),
                },
                written: true,
            });
        }
        self.inner.claim_write_access(key)
    }

    fn list_accesses(&self) -> Vec<(WorldAccessRange, AccessInstance)> {
        self.inner.list_accesses()
    }

    fn count_accesses(&self) -> usize {
        self.inner.count_accesses()
    }

    fn release_all_accesses(&self) {
        self.inner.release_all_accesses();
    }
}

/// A polymorphic enum for access map types.
///
/// Equivalent to `dyn DynamicSystemMeta` for most purposes
pub enum AnyAccessMap {
    /// A map which allows any and all accesses to be claimed
    UnlimitedAccessMap(AccessMap),
    /// A map which only allows accesses to keys in a pre-specified subset
    SubsetAccessMap(SubsetAccessMap),
}

#[profiling::all_functions]
impl DynamicSystemMeta for AnyAccessMap {
    fn with_scope<O, F: FnOnce() -> O>(&self, f: F) -> O {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.with_scope(f),
            AnyAccessMap::SubsetAccessMap(map) => map.with_scope(f),
        }
    }

    fn release_access<K: Into<WorldAccessRange>>(&self, key: K) {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.release_access(key),
            AnyAccessMap::SubsetAccessMap(map) => map.release_access(key),
        }
    }

    #[track_caller]
    fn claim_read_access<K: Into<WorldAccessRange>>(&self, key: K) -> Result<(), AccessInstance> {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.claim_read_access(key),
            AnyAccessMap::SubsetAccessMap(map) => map.claim_read_access(key),
        }
    }

    #[track_caller]
    fn claim_write_access<K: Into<WorldAccessRange>>(&self, key: K) -> Result<(), AccessInstance> {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.claim_write_access(key),
            AnyAccessMap::SubsetAccessMap(map) => map.claim_write_access(key),
        }
    }

    fn list_accesses(&self) -> Vec<(WorldAccessRange, AccessInstance)> {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.list_accesses(),
            AnyAccessMap::SubsetAccessMap(map) => map.list_accesses(),
        }
    }

    fn count_accesses(&self) -> usize {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.count_accesses(),
            AnyAccessMap::SubsetAccessMap(map) => map.count_accesses(),
        }
    }

    fn release_all_accesses(&self) {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.release_all_accesses(),
            AnyAccessMap::SubsetAccessMap(map) => map.release_all_accesses(),
        }
    }
}

/// A trait for displaying a code location nicely
pub trait DisplayCodeLocation {
    /// Displays the location
    fn display_location(self) -> String;
}

#[profiling::all_functions]
impl DisplayCodeLocation for std::panic::Location<'_> {
    fn display_location(self) -> String {
        format!("\"{}:{}\"", self.file(), self.line())
    }
}

#[profiling::all_functions]
impl DisplayCodeLocation for Option<std::panic::Location<'_>> {
    fn display_location(self) -> String {
        self.map(|l| l.display_location())
            .unwrap_or_else(|| "\"unknown location\"".to_owned())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    struct TestAccess(pub usize);
    impl From<TestAccess> for WorldAccessRange {
        fn from(val: TestAccess) -> Self {
            WorldAccessRange::ComponentOrResource(ComponentRange(unsafe {
                NonZero::new_unchecked((val.0 + 1) as u16)
            }))
        }
    }

    #[test]
    fn access_map_list_accesses() {
        let access_map = AccessMap::default();

        let _ = access_map.claim_read_access(TestAccess(1));
        let _ = access_map.claim_write_access(TestAccess(2));

        let accesses = access_map.list_accesses();

        assert_eq!(accesses.len(), 2);
        let access_0 = accesses
            .iter()
            .find(|(k, _)| *k == TestAccess(1).into())
            .unwrap();
        let access_1 = accesses
            .iter()
            .find(|(k, _)| *k == TestAccess(2).into())
            .unwrap();

        assert!(!access_0.1.written);
        assert!(access_1.1.written);
    }

    #[test]
    fn subset_access_map_list_accesses() {
        let subset_access_map = SubsetAccessMap::new([TestAccess(1), TestAccess(2)]);

        assert!(subset_access_map.claim_read_access(TestAccess(1)).is_ok());
        assert!(subset_access_map.claim_write_access(TestAccess(2)).is_ok());

        let accesses = subset_access_map.list_accesses();

        assert_eq!(accesses.len(), 2);
        let access_0 = accesses
            .iter()
            .find(|(k, _)| *k == TestAccess(1).into())
            .unwrap();
        let access_1 = accesses
            .iter()
            .find(|(k, _)| *k == TestAccess(2).into())
            .unwrap();

        assert!(!access_0.1.written);
        assert!(access_1.1.written);
    }

    #[test]
    fn access_map_read_access_blocks_write() {
        let access_map = AccessMap::default();

        assert!(access_map.claim_read_access(TestAccess(1)).is_ok());
        assert!(access_map.claim_write_access(TestAccess(1)).is_err());
        access_map.release_access(TestAccess(1));
        assert!(access_map.claim_write_access(TestAccess(1)).is_ok());
    }

    #[test]
    fn subset_access_map_read_access_blocks_write() {
        let subset_access_map = SubsetAccessMap::new([TestAccess(1)]);

        assert!(subset_access_map.claim_read_access(TestAccess(1)).is_ok());
        assert!(subset_access_map.claim_write_access(TestAccess(1)).is_err());
        subset_access_map.release_access(TestAccess(1));
        assert!(subset_access_map.claim_write_access(TestAccess(1)).is_ok());
    }

    #[test]
    fn access_map_write_access_blocks_read() {
        let access_map = AccessMap::default();

        assert!(access_map.claim_write_access(TestAccess(1)).is_ok());
        assert!(access_map.claim_read_access(TestAccess(1)).is_err());
        access_map.release_access(TestAccess(1));
        assert!(access_map.claim_read_access(TestAccess(1)).is_ok());
    }

    #[test]
    fn subset_access_map_write_access_blocks_read() {
        let subset_access_map = SubsetAccessMap::new([TestAccess(1)]);

        assert!(subset_access_map.claim_write_access(TestAccess(1)).is_ok());
        assert!(subset_access_map.claim_read_access(TestAccess(1)).is_err());
        subset_access_map.release_access(TestAccess(1));
        assert!(subset_access_map.claim_read_access(TestAccess(1)).is_ok());
    }

    #[test]
    fn access_map_read_global_access_blocks_all_writes() {
        let access_map = AccessMap::default();

        assert!(
            access_map
                .claim_read_access(WorldAccessRange::Global)
                .is_ok()
        );
        assert!(access_map.claim_write_access(TestAccess(1)).is_err());
        assert!(access_map.claim_read_access(TestAccess(1)).is_ok());
        access_map.release_access(WorldAccessRange::Global);
        access_map.release_access(TestAccess(1));

        // can re-claim after releasing global
        assert!(access_map.claim_write_access(TestAccess(1)).is_ok());
        access_map.release_access(TestAccess(1));
        assert!(access_map.claim_read_access(TestAccess(1)).is_ok());
    }

    #[test]
    fn access_map_write_global_access_blocks_all_access() {
        let access_map = AccessMap::default();

        assert!(
            access_map
                .claim_write_access(WorldAccessRange::Global)
                .is_ok()
        );
        assert!(access_map.claim_write_access(TestAccess(1)).is_err());
        assert!(access_map.claim_read_access(TestAccess(1)).is_err());
        access_map.release_access(WorldAccessRange::Global);

        // can re-claim after releasing global
        assert!(access_map.claim_write_access(TestAccess(1)).is_ok());
        access_map.release_access(TestAccess(1));
        assert!(access_map.claim_read_access(TestAccess(1)).is_ok());
    }

    #[test]
    fn subset_access_map_cannot_read_global_access() {
        let subset_access_map = SubsetAccessMap::new([TestAccess(1), TestAccess(2)]);

        assert!(
            subset_access_map
                .claim_read_access(WorldAccessRange::Global)
                .is_err()
        );
    }

    #[test]
    fn access_map_any_access_blocks_write_global() {
        let access_map = AccessMap::default();

        assert!(access_map.claim_read_access(TestAccess(1)).is_ok());
        assert!(
            access_map
                .claim_write_access(WorldAccessRange::Global)
                .is_err()
        );
        access_map.release_access(TestAccess(1));

        assert!(access_map.claim_write_access(TestAccess(1)).is_ok());
        assert!(
            access_map
                .claim_write_access(WorldAccessRange::Global)
                .is_err()
        );
    }

    #[test]
    fn access_map_with_scope_unrolls_individual_accesses() {
        let access_map = AccessMap::default();
        // Claim a read access outside the scope
        assert!(access_map.claim_read_access(TestAccess(3)).is_ok());

        // Inside with_scope, claim additional accesses
        access_map.with_scope(|| {
            assert!(access_map.claim_read_access(TestAccess(1)).is_ok());
            assert!(access_map.claim_write_access(TestAccess(2)).is_ok());
            // At this point, individual_accesses contains keys 0, 1 and 2.
            let accesses = access_map.list_accesses();
            assert_eq!(accesses.len(), 3);
        });

        // After with_scope returns, accesses claimed inside (keys 1 and 2) are unrolled.
        let accesses = access_map.list_accesses();
        // Only the access claimed outside (key 3) remains.
        assert_eq!(accesses.len(), 1);
        let (k, count) = &accesses[0];
        assert_eq!(*k, TestAccess(3).into());
        assert!(!count.written);
    }

    #[test]
    fn subset_map_with_scope_unrolls_individual_accesses() {
        let subset_access_map = SubsetAccessMap::new([TestAccess(1), TestAccess(2), TestAccess(3)]);

        // Claim a read access outside the scope
        assert!(subset_access_map.claim_read_access(TestAccess(3)).is_ok());

        // Inside with_scope, claim additional accesses
        subset_access_map.with_scope(|| {
            assert!(subset_access_map.claim_read_access(TestAccess(1)).is_ok());
            assert!(subset_access_map.claim_write_access(TestAccess(2)).is_ok());
            // At this point, individual_accesses contains keys 0, 1 and 2.
            let accesses = subset_access_map.list_accesses();
            assert_eq!(accesses.len(), 3);
        });

        // After with_scope returns, accesses claimed inside (keys 1 and 2) are unrolled.
        let accesses = subset_access_map.list_accesses();
        // Only the access claimed outside (key 3) remains.
        assert_eq!(accesses.len(), 1);
        let (k, count) = &accesses[0];
        assert_eq!(*k, TestAccess(3).into());
        assert!(!count.written);
    }

    #[test]
    fn access_map_with_scope_unrolls_global_accesses() {
        let access_map = AccessMap::default();

        access_map.with_scope(|| {
            assert!(
                access_map
                    .claim_write_access(WorldAccessRange::Global)
                    .is_ok()
            );
            // At this point, global_access is claimed.
            assert!(access_map.claim_read_access(TestAccess(1)).is_err());
        });

        let accesses = access_map.list_accesses();
        assert_eq!(accesses.len(), 0);
    }

    #[test]
    fn access_map_count_accesses_counts_globals() {
        let access_map = AccessMap::default();

        // Initially, no accesses are active.
        assert_eq!(access_map.count_accesses(), 0);

        // Claim global access. When global access is active,
        // count_accesses should return 1.
        assert!(
            access_map
                .claim_write_access(WorldAccessRange::Global)
                .is_ok()
        );
        assert_eq!(access_map.count_accesses(), 1);
        access_map.release_access(WorldAccessRange::Global);

        // Now claim individual accesses.
        assert!(access_map.claim_read_access(TestAccess(1)).is_ok());
        assert!(access_map.claim_write_access(TestAccess(2)).is_ok());
        // Since two separate keys were claimed, count_accesses should return 2.
        assert_eq!(access_map.count_accesses(), 2);

        // Cleanup individual accesses.
        access_map.release_access(TestAccess(1));
        access_map.release_access(TestAccess(2));
    }

    #[test]
    fn subset_map_prevents_access_to_out_of_subset_access() {
        let subset_access_map = SubsetAccessMap::new([TestAccess(1)]);

        assert!(subset_access_map.claim_read_access(TestAccess(2)).is_err());
        assert!(subset_access_map.claim_write_access(TestAccess(2)).is_err());
        assert!(
            subset_access_map
                .claim_read_access(WorldAccessRange::Global)
                .is_err()
        );
    }

    #[test]
    fn subset_map_retains_subset_in_scope() {
        let subset_access_map = SubsetAccessMap::new([TestAccess(1)]);

        subset_access_map.with_scope(|| {
            assert!(subset_access_map.claim_read_access(TestAccess(1)).is_ok());
            assert!(subset_access_map.claim_read_access(TestAccess(2)).is_err());
            assert!(subset_access_map.claim_write_access(TestAccess(2)).is_err());
        });

        assert!(subset_access_map.claim_read_access(TestAccess(1)).is_ok());
        assert!(subset_access_map.claim_read_access(TestAccess(2)).is_err());
        assert!(subset_access_map.claim_write_access(TestAccess(2)).is_err());
    }
}
