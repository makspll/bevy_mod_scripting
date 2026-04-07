//! A map of access claims used to safely and dynamically access the world.

use bevy_ecs::component::ComponentId;

use bevy_platform::collections::HashMap;
use fixedbitset::FixedBitSet;
use parking_lot::Mutex;

/// Describes access ranges in and outside a bevy world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorldAccessRange {
    /// An access into a resource or component in the world
    ComponentOrResource(ComponentId),
    /// An access outside the world, for example into an external allocator
    External(usize),
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
        Self::ComponentOrResource(value)
    }
}

#[derive(Debug)]
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
    fn claim_read_access<K: Into<WorldAccessRange>>(&self, key: K) -> bool;

    /// Attempts to claim write access for the given key.
    ///
    /// Returns `true` if the write access is successfully claimed. Write access fails if any
    /// read or write access is active for the key or if a global lock is held.
    #[track_caller]
    fn claim_write_access<K: Into<WorldAccessRange>>(&self, key: K) -> bool;

    /// Releases an access claimed for the provided key.
    ///
    /// # Panics
    ///
    /// Panics if the access is released by a thread different from the one that claimed it.
    fn release_access<K: Into<WorldAccessRange>>(&self, key: K);

    /// Returns a list of active accesses.
    ///
    /// The list is provided as key and corresponding access count pairs.
    fn list_accesses(&self) -> Vec<(WorldAccessRange, bool)>;

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
    component_allowed_mask: FixedBitSet,
    component_reads: FixedBitSet,
    component_writes: FixedBitSet,
    world_read: bool,
    world_write: bool,
    external_accesses: HashMap<usize, bool>,
}

#[profiling::all_functions]
impl AccessMapInner {
    #[inline]
    fn claim_if_free(&mut self, key: WorldAccessRange, write: bool) -> bool {
        match key {
            // -- Normal mode
            WorldAccessRange::ComponentOrResource(component_id) => {
                let idx = component_id.index();

                if !self.component_allowed_mask.is_empty()
                    && !self.component_allowed_mask.contains(idx)
                {
                    return false;
                }

                // Normal mode
                if write {
                    return !self.component_reads.contains(idx)
                        && !self.world_read
                        && !self.world_write
                        && !self.component_writes.put(idx);
                } else {
                    let free = !self.world_write && !self.component_writes.contains(idx);
                    if free {
                        self.component_reads.set(idx, true);
                    }
                    return free;
                }
            }
            WorldAccessRange::External(idx) => self
                .external_accesses
                .get(&idx)
                .map(|conflicting_access_is_write| (!write && !conflicting_access_is_write))
                .unwrap_or(true),
            WorldAccessRange::Global => {
                let free = self.component_allowed_mask.is_empty()
                    && if write {
                        self.component_reads.is_clear()
                            && self.component_writes.is_clear()
                            && !self.world_write
                            && !self.world_read
                    } else {
                        self.component_writes.is_clear() && !self.world_write
                    };

                if free {
                    if write {
                        self.world_write = true;
                    } else {
                        self.world_read = true;
                    }
                }

                return free;
            }
        }
    }

    #[inline]
    fn clear_access(&mut self, key: WorldAccessRange) {
        // cannot claim read if write is active, so either read or write is set
        // we can clear both
        match key {
            WorldAccessRange::ComponentOrResource(component_id) => {
                self.component_reads.remove(component_id.index());
                self.component_writes.remove(component_id.index());
            }
            WorldAccessRange::External(idx) => _ = self.external_accesses.remove(&idx),
            WorldAccessRange::Global => {
                self.world_write = false;
                self.world_read = false;
            }
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

impl AccessMap {
    /// Creates a new access map which will only allow access up to the given component ID
    pub fn new(max_component_id: ComponentId) -> Self {
        let max_id = max_component_id.index();
        Self(Mutex::new(AccessMapInner {
            component_allowed_mask: FixedBitSet::with_capacity(0),
            component_reads: FixedBitSet::with_capacity(max_id + 1),
            component_writes: FixedBitSet::with_capacity(max_id + 1),
            ..Default::default()
        }))
    }

    /// Creates a new access map which will only allow access to the given set of components
    pub fn new_subset(allowed: FixedBitSet) -> Self {
        Self(Mutex::new(AccessMapInner {
            component_reads: FixedBitSet::with_capacity(allowed.maximum().unwrap_or(0) + 1),
            component_writes: FixedBitSet::with_capacity(allowed.maximum().unwrap_or(0) + 1),
            component_allowed_mask: allowed,
            ..Default::default()
        }))
    }
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
    fn claim_read_access<K: Into<WorldAccessRange>>(&self, key: K) -> bool {
        let mut inner = self.0.lock();
        let key = key.into();
        inner.claim_if_free(key, false)
    }

    #[track_caller]
    fn claim_write_access<K: Into<WorldAccessRange>>(&self, key: K) -> bool {
        let mut inner = self.0.lock();
        let key = key.into();
        inner.claim_if_free(key, true)
    }

    fn list_accesses(&self) -> Vec<(WorldAccessRange, bool)> {
        let inner = self.0.lock();
        let reads = inner.component_reads.ones().map(|idx| {
            (
                WorldAccessRange::ComponentOrResource(ComponentId::new(idx)),
                false,
            )
        });
        let writes = inner.component_writes.ones().map(|idx| {
            (
                WorldAccessRange::ComponentOrResource(ComponentId::new(idx)),
                true,
            )
        });
        let external = inner
            .external_accesses
            .iter()
            .map(|(k, v)| (WorldAccessRange::External(*k), *v));
        let world_reads = if inner.world_read {
            vec![(WorldAccessRange::Global, false)].into_iter()
        } else {
            vec![].into_iter()
        };

        let world_writes = if inner.world_write {
            vec![(WorldAccessRange::Global, true)].into_iter()
        } else {
            vec![].into_iter()
        };

        reads
            .chain(writes)
            .chain(external)
            .chain(world_reads)
            .chain(world_writes)
            .collect()
    }

    fn count_accesses(&self) -> usize {
        self.list_accesses().len()
    }

    fn release_all_accesses(&self) {
        let mut inner = self.0.lock();
        inner.component_reads.clear();
        inner.component_writes.clear();
        inner.external_accesses.clear();
        inner.world_read = false;
        inner.world_write = false;
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
            WorldAccessRange::ComponentOrResource(ComponentId::new(val.0))
        }
    }

    #[test]
    fn access_map_list_accesses() {
        let access_map = AccessMap::new(ComponentId::new(10));

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
        assert!(!access_0.1);
        assert!(access_1.1);
    }

    #[test]
    fn access_map_read_access_blocks_write() {
        let access_map = AccessMap::new(ComponentId::new(10));

        assert!(access_map.claim_read_access(TestAccess(1)));
        assert!(!access_map.claim_write_access(TestAccess(1)));
        access_map.release_access(TestAccess(1));
        assert!(access_map.claim_write_access(TestAccess(1)));
    }

    #[test]
    fn subset_access_map_read_access_blocks_write() {
        let subset_access_map = AccessMap::new_subset([1].into_iter().collect());

        assert!(subset_access_map.claim_read_access(TestAccess(1)));
        assert!(!subset_access_map.claim_write_access(TestAccess(1)));
        subset_access_map.release_access(TestAccess(1));
        assert!(subset_access_map.claim_write_access(TestAccess(1)));
    }

    #[test]
    fn access_map_write_access_blocks_read() {
        let access_map = AccessMap::new(ComponentId::new(10));

        assert!(access_map.claim_write_access(TestAccess(1)));
        assert!(!access_map.claim_read_access(TestAccess(1)));
        access_map.release_access(TestAccess(1));
        assert!(access_map.claim_read_access(TestAccess(1)));
    }

    #[test]
    fn subset_access_map_write_access_blocks_read() {
        let subset_access_map = AccessMap::new_subset([1].into_iter().collect());

        assert!(subset_access_map.claim_write_access(TestAccess(1)));
        assert!(!subset_access_map.claim_read_access(TestAccess(1)));
        subset_access_map.release_access(TestAccess(1));
        assert!(subset_access_map.claim_read_access(TestAccess(1)));
    }

    #[test]
    fn access_map_read_global_access_blocks_all_writes() {
        let access_map = AccessMap::new(ComponentId::new(10));

        assert!(access_map.claim_read_access(WorldAccessRange::Global));
        assert!(!access_map.claim_write_access(TestAccess(1)));
        assert!(access_map.claim_read_access(TestAccess(1)));
        access_map.release_access(WorldAccessRange::Global);
        access_map.release_access(TestAccess(1));

        // can re-claim after releasing global
        assert!(access_map.claim_write_access(TestAccess(1)));
        access_map.release_access(TestAccess(1));
        assert!(access_map.claim_read_access(TestAccess(1)));
    }

    #[test]
    fn access_map_write_global_access_blocks_all_access() {
        let access_map = AccessMap::new(ComponentId::new(10));

        assert!(access_map.claim_write_access(WorldAccessRange::Global));
        assert!(!access_map.claim_write_access(TestAccess(1)));
        assert!(!access_map.claim_read_access(TestAccess(1)));
        access_map.release_access(WorldAccessRange::Global);

        // can re-claim after releasing global
        assert!(access_map.claim_write_access(TestAccess(1)));
        access_map.release_access(TestAccess(1));
        assert!(access_map.claim_read_access(TestAccess(1)));
    }

    #[test]
    fn subset_access_map_cannot_read_global_access() {
        let subset_access_map = AccessMap::new_subset([1, 2].into_iter().collect());

        assert!(!subset_access_map.claim_read_access(WorldAccessRange::Global));
    }

    #[test]
    fn access_map_any_access_blocks_write_global() {
        let access_map = AccessMap::new(ComponentId::new(10));

        assert!(access_map.claim_read_access(TestAccess(1)));
        assert!(!access_map.claim_write_access(WorldAccessRange::Global));
        access_map.release_access(TestAccess(1));

        assert!(access_map.claim_write_access(TestAccess(1)));
        assert!(!access_map.claim_write_access(WorldAccessRange::Global));
    }

    #[test]
    fn access_map_with_scope_unrolls_individual_accesses() {
        let access_map = AccessMap::new(ComponentId::new(10));
        // Claim a read access outside the scope
        assert!(access_map.claim_read_access(TestAccess(3)));

        // Inside with_scope, claim additional accesses
        access_map.with_scope(|| {
            assert!(access_map.claim_read_access(TestAccess(1)));
            assert!(access_map.claim_write_access(TestAccess(2)));
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
        assert!(!count);
    }

    #[test]
    fn subset_map_with_scope_unrolls_individual_accesses() {
        let subset_access_map = AccessMap::new_subset([1, 2, 3].into_iter().collect());

        // Claim a read access outside the scope
        assert!(subset_access_map.claim_read_access(TestAccess(3)));

        // Inside with_scope, claim additional accesses
        subset_access_map.with_scope(|| {
            assert!(subset_access_map.claim_read_access(TestAccess(1)));
            assert!(subset_access_map.claim_write_access(TestAccess(2)));
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
        assert!(!count);
    }

    #[test]
    fn access_map_with_scope_unrolls_global_accesses() {
        let access_map = AccessMap::new(ComponentId::new(10));

        access_map.with_scope(|| {
            assert!(access_map.claim_write_access(WorldAccessRange::Global));
            // At this point, global_access is claimed.
            assert!(!access_map.claim_read_access(TestAccess(1)));
        });

        let accesses = access_map.list_accesses();
        assert_eq!(accesses.len(), 0);
    }

    #[test]
    fn access_map_count_accesses_counts_globals() {
        let access_map = AccessMap::new(ComponentId::new(10));

        // Initially, no accesses are active.
        assert_eq!(access_map.count_accesses(), 0);

        // Claim global access. When global access is active,
        // count_accesses should return 1.
        assert!(access_map.claim_write_access(WorldAccessRange::Global));
        assert_eq!(access_map.count_accesses(), 1);
        access_map.release_access(WorldAccessRange::Global);

        // Now claim individual accesses.
        assert!(access_map.claim_read_access(TestAccess(1)));
        assert!(access_map.claim_write_access(TestAccess(2)));
        // Since two separate keys were claimed, count_accesses should return 2.
        assert_eq!(access_map.count_accesses(), 2);

        // Cleanup individual accesses.
        access_map.release_access(TestAccess(1));
        access_map.release_access(TestAccess(2));
    }

    #[test]
    fn subset_map_prevents_access_to_out_of_subset_access() {
        let subset_access_map = AccessMap::new_subset([1].into_iter().collect());

        assert!(!subset_access_map.claim_read_access(TestAccess(2)));
        assert!(!subset_access_map.claim_write_access(TestAccess(2)));
        assert!(!subset_access_map.claim_read_access(WorldAccessRange::Global));
    }

    #[test]
    fn subset_map_retains_subset_in_scope() {
        let subset_access_map = AccessMap::new_subset([1].into_iter().collect());

        subset_access_map.with_scope(|| {
            assert!(subset_access_map.claim_read_access(TestAccess(1)));
            assert!(!subset_access_map.claim_read_access(TestAccess(2)));
            assert!(!subset_access_map.claim_write_access(TestAccess(2)));
        });

        assert!(subset_access_map.claim_read_access(TestAccess(1)));
        assert!(!subset_access_map.claim_read_access(TestAccess(2)));
        assert!(!subset_access_map.claim_write_access(TestAccess(2)));
    }
}
