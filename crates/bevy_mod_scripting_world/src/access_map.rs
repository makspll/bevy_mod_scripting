//! A map of access claims used to safely and dynamically access the world.

use bevy_ecs::component::ComponentId;

use bevy_platform::collections::HashMap;
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

/// Describes the state of an access lock
#[derive(Copy, Clone, Debug, Default)]
pub struct AccessBitField(u8);

#[profiling::all_functions]
impl AccessBitField {
    const KIND_MASK: u8 = 0b0000_0001;
    const FORBIDDEN_MASK: u8 = 0b0000_0010;
    const COUNT_MASK: u8 = 0b1111_1100;
    const COUNT_SHIFT: u8 = 2;
    const MAX_COUNT: u8 = 0b0011_1111; // 63

    /// Claims the given access and returns an adjusted state
    pub fn claim_if_free(self, write: bool) -> (bool, Self) {
        let kind_requested = (write as u8) & 1;
        let kind = self.kind();
        let count = self.access_count();

        let one_is_exclusive = (kind_requested | kind) == 0b1;
        let conflicting_exclusive = one_is_exclusive && count != 0;
        let aliasing_exclusive_access = (kind_requested == 1) && (count != 0);
        let access_is_forbidden = self.is_forbidden();

        let allowed = !(conflicting_exclusive || aliasing_exclusive_access || access_is_forbidden);
        let allowed_u8 = allowed as u8;

        // Increment count only if allowed
        let new_count = count + (allowed_u8 & 1);

        // If count was 0 and we're allowed, adopt requested kind
        let is_first = (count == 0) as u8;
        let kind_mask = is_first & allowed_u8;

        // Select new kind branchlessly
        let new_kind_bits = (kind & !kind_mask) | (kind_requested & kind_mask);

        // Rebuild bitfield
        let forbidden_bits = self.0 & Self::FORBIDDEN_MASK;
        let new_count_bits = (new_count << Self::COUNT_SHIFT) & Self::COUNT_MASK;

        let new_bits = (new_kind_bits & Self::KIND_MASK) | forbidden_bits | new_count_bits;

        (allowed, Self(new_bits))
    }

    #[inline]
    /// Returns true if the access is a write
    pub fn is_write(self) -> bool {
        (self.0 & Self::KIND_MASK) != 0
    }

    #[inline]
    /// Returns true if the access is forbidden
    pub fn is_forbidden(self) -> bool {
        (self.0 & Self::FORBIDDEN_MASK) != 0
    }

    #[inline]
    /// Returns the amount of readers/writers
    pub fn access_count(self) -> u8 {
        (self.0 & Self::COUNT_MASK) >> Self::COUNT_SHIFT
    }

    #[inline]
    /// Returns the kind of access 1 if write and 0 otherwise
    pub fn kind(self) -> u8 {
        self.0 & Self::KIND_MASK
    }

    #[inline]
    /// Returns new state with adjusted kind
    pub fn with_kind(self, is_write: bool) -> Self {
        let bit = (is_write as u8) & 1;
        Self((self.0 & !Self::KIND_MASK) | bit)
    }

    #[inline]
    /// Returns a new state with adjusted forbidden flag
    pub fn with_forbidden(self, forbidden: bool) -> Self {
        let bit = (forbidden as u8) << 1;
        Self((self.0 & !Self::FORBIDDEN_MASK) | bit)
    }

    #[inline]
    /// Returns a new state with increased reader/writer count
    pub fn increment_count(self) -> Self {
        let count = self.access_count();
        debug_assert!(count < Self::MAX_COUNT, "count overflow");
        let new = count + 1;
        Self((self.0 & !Self::COUNT_MASK) | (new << Self::COUNT_SHIFT))
    }

    #[inline]
    /// Returns a new state with decreased reader/writer count
    pub fn decrement_count(self) -> Self {
        let count = self.access_count();
        debug_assert!(count > 0, "count underflow");

        let new = count - 1;

        // mask = 1 if new != 0, else 0
        let nonzero_mask = (new != 0) as u8;

        // Preserve kind only if new != 0, otherwise zero it
        let kind = self.kind() & nonzero_mask;

        let forbidden = self.0 & Self::FORBIDDEN_MASK;

        let new_bits =
            (kind & Self::KIND_MASK) | forbidden | ((new << Self::COUNT_SHIFT) & Self::COUNT_MASK);
        Self(new_bits)
    }

    #[inline]
    /// Returns a new state with everything but the forbidden mask cleared
    pub fn clear_accesses(self) -> Self {
        Self(self.0 & Self::FORBIDDEN_MASK)
    }
}

#[derive(Clone, Debug, Default)]
/// A collection of AccessBitFields indexed by id of access.
/// Must not be used with potentially large ID's as it will allocate large amounts of memory.
pub struct AccessByteSet {
    new_capacity_forbidden: bool,
    data: Vec<AccessBitField>,
}

#[profiling::all_functions]
impl AccessByteSet {
    #[inline]
    /// Create a new empty access byte set
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            new_capacity_forbidden: false,
        }
    }

    /// Generates a new access byte set with the given forbidden list
    pub fn from_allowed_list(iter: &[usize]) -> Self {
        let max_index = iter.iter().max().unwrap_or(&0);
        let mut new = Self::new();
        new.new_capacity_forbidden = true;
        new.ensure_capacity(*max_index);
        for i in iter {
            new.data[*i] = new.data[*i].with_forbidden(false)
        }

        new
    }

    fn ensure_capacity(&mut self, index: usize) {
        if index + 1 > self.data.len() {
            self.data.resize_with(index + 1, || {
                AccessBitField(
                    AccessBitField::FORBIDDEN_MASK
                        & ((self.new_capacity_forbidden as u8 & 0b1) << 1),
                )
            });
        }
    }

    /// Return a mutable reference to the given access index, initializing it if it doesn't exist
    pub fn entry_mut(&mut self, index: usize) -> &mut AccessBitField {
        self.ensure_capacity(index);

        &mut self.data[index]
    }

    /// Iterate over all accesses with non-zero reader/writer counts
    pub fn iter_accessed(&self) -> impl Iterator<Item = (usize, AccessBitField)> {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, a)| a.access_count() > 0)
            .map(|(idx, a)| (idx, *a))
    }

    /// Clear accesses leaving the current capacity intact
    pub fn clear(&mut self) {
        self.data.iter_mut().for_each(|a| _ = a.clear_accesses());
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
    cant_access_global: bool,
    component_access: AccessByteSet,
    world_access: AccessBitField,
    external_accesses: HashMap<usize, AccessBitField>,
}

#[profiling::all_functions]
impl AccessMapInner {
    pub fn claim_component_if_free(&mut self, id: ComponentId, write: bool) -> bool {
        let idx = id.index();
        let elem = self.component_access.entry_mut(idx);
        let (free, new) = elem.claim_if_free(write);
        *elem = new;
        return free;
    }

    fn claim_if_free(&mut self, key: WorldAccessRange, write: bool) -> bool {
        match key {
            WorldAccessRange::ComponentOrResource(component_id) => {
                if self.world_access.is_write() || self.world_access.access_count() != 0 && write {
                    return false;
                }
                self.claim_component_if_free(component_id, write)
            }

            WorldAccessRange::External(idx) => {
                let elem = self.external_accesses.entry(idx).or_default();
                let (free, new) = elem.claim_if_free(write);
                *elem = new;
                return free;
            }

            WorldAccessRange::Global => {
                let elem = &mut self.world_access;
                if self.cant_access_global
                    || self
                        .component_access
                        .iter_accessed()
                        .any(|(_, a)| a.is_write() || (!a.is_write() && write))
                {
                    return false;
                }
                let (free, new) = elem.claim_if_free(write);
                *elem = new;
                return free;
            }
        }
    }
    #[inline]
    fn clear_access(&mut self, key: WorldAccessRange) {
        match key {
            WorldAccessRange::ComponentOrResource(component_id) => {
                let entry = self.component_access.entry_mut(component_id.index());
                *entry = entry.decrement_count();
            }

            WorldAccessRange::External(idx) => {
                if let Some(field) = self.external_accesses.get_mut(&idx) {
                    *field = field.decrement_count()
                }
            }

            WorldAccessRange::Global => {
                self.world_access = self.world_access.decrement_count();
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

impl Default for AccessMap {
    fn default() -> Self {
        Self::new()
    }
}

impl AccessMap {
    /// Creates a new access map which will only allow access up to the given component ID
    pub fn new() -> Self {
        Self(Mutex::new(AccessMapInner {
            ..Default::default()
        }))
    }

    /// Creates a new access map which will only allow access to the given set of components
    pub fn new_subset(filter: AccessByteSet) -> Self {
        Self(Mutex::new(AccessMapInner {
            component_access: filter,
            cant_access_global: true,
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
        let comps = inner.component_access.iter_accessed().flat_map(|(idx, a)| {
            (0..a.access_count()).map(move |_| {
                (
                    WorldAccessRange::ComponentOrResource(ComponentId::new(idx)),
                    a.is_write(),
                )
            })
        });
        let external = inner.external_accesses.iter().flat_map(|(k, v)| {
            (0..v.access_count()).map(|_| (WorldAccessRange::External(*k), v.is_write()))
        });

        let mut accesses = comps.chain(external).collect::<Vec<_>>();

        accesses.extend(
            (0..inner.world_access.access_count())
                .map(|_| (WorldAccessRange::Global, inner.world_access.is_write())),
        );

        accesses
    }

    fn count_accesses(&self) -> usize {
        self.list_accesses().len()
    }

    fn release_all_accesses(&self) {
        let mut inner = self.0.lock();
        inner.component_access.clear();
        inner.external_accesses.clear();
        inner.world_access = Default::default();
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
        let access_map = AccessMap::new();

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
        let access_map = AccessMap::new();

        assert!(access_map.claim_read_access(TestAccess(1)));
        assert!(!access_map.claim_write_access(TestAccess(1)));
        access_map.release_access(TestAccess(1));
        assert!(access_map.claim_write_access(TestAccess(1)));
    }

    #[test]
    fn subset_access_map_read_access_blocks_write() {
        let subset_access_map = AccessMap::new_subset(AccessByteSet::from_allowed_list(&[1]));

        assert!(subset_access_map.claim_read_access(TestAccess(1)));
        assert!(!subset_access_map.claim_write_access(TestAccess(1)));
        subset_access_map.release_access(TestAccess(1));
        assert!(subset_access_map.claim_write_access(TestAccess(1)));
    }

    #[test]
    fn access_map_write_access_blocks_read() {
        let access_map = AccessMap::new();

        assert!(access_map.claim_write_access(TestAccess(1)));
        assert!(!access_map.claim_read_access(TestAccess(1)));
        access_map.release_access(TestAccess(1));
        assert!(access_map.claim_read_access(TestAccess(1)));
    }

    #[test]
    fn subset_access_map_write_access_blocks_read() {
        let subset_access_map = AccessMap::new_subset(AccessByteSet::from_allowed_list(&[1]));

        assert!(subset_access_map.claim_write_access(TestAccess(1)));
        assert!(!subset_access_map.claim_read_access(TestAccess(1)));
        subset_access_map.release_access(TestAccess(1));
        assert!(subset_access_map.claim_read_access(TestAccess(1)));
    }

    #[test]
    fn access_map_read_global_access_blocks_all_writes() {
        let access_map = AccessMap::new();

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
        let access_map = AccessMap::new();

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
        let subset_access_map = AccessMap::new_subset(AccessByteSet::from_allowed_list(&[1, 2]));

        assert!(!subset_access_map.claim_read_access(WorldAccessRange::Global));
    }

    #[test]
    fn access_map_any_access_blocks_write_global() {
        let access_map = AccessMap::new();

        assert!(access_map.claim_read_access(TestAccess(1)));
        assert!(!access_map.claim_write_access(WorldAccessRange::Global));
        access_map.release_access(TestAccess(1));

        assert!(access_map.claim_write_access(TestAccess(1)));
        assert!(!access_map.claim_write_access(WorldAccessRange::Global));
    }

    #[test]
    fn access_map_with_scope_unrolls_individual_accesses() {
        let access_map = AccessMap::new();
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
        let subset_access_map = AccessMap::new_subset(AccessByteSet::from_allowed_list(&[1, 2, 3]));

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
        let access_map = AccessMap::new();

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
        let access_map = AccessMap::new();

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
        let subset_access_map = AccessMap::new_subset(AccessByteSet::from_allowed_list(&[1]));

        assert!(!subset_access_map.claim_read_access(TestAccess(2)));
        assert!(!subset_access_map.claim_write_access(TestAccess(2)));
        assert!(!subset_access_map.claim_read_access(WorldAccessRange::Global));
    }

    #[test]
    fn subset_map_retains_subset_in_scope() {
        let subset_access_map = AccessMap::new_subset(AccessByteSet::from_allowed_list(&[1]));

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
