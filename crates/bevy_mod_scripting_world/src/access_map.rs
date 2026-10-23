//! A map of access claims used to safely and dynamically access the world.

use std::{any::Any, sync::Arc};

use bevy_ecs::component::ComponentId;

use bevy_platform::collections::HashMap;
use parking_lot::Mutex;

use crate::bit_set::BitSet;

/// Describes access ranges in and outside a bevy world
#[derive(Clone)]
pub enum WorldAccessRange {
    /// An access into a resource or component in the world
    ComponentOrResource(ComponentId),
    /// An access to a value outside the world, i.e. a value allocated for scripts, identified by its owner.
    ///
    /// The access map holds the owner while the access is claimed, so the value outlives every claim to it.
    External(AccessOwner),
    /// A whole world read or write
    Global,
}

/// Keeps a value outside the world alive, see [`WorldAccessRange::External`].
pub type AccessOwner = Arc<dyn Any + Send + Sync>;

/// The id of a value outside the world: the address of its owner, unique while the owner is alive.
pub fn external_access_id(owner: &AccessOwner) -> usize {
    Arc::as_ptr(owner) as *const () as usize
}

impl WorldAccessRange {
    /// Returns true if this access conflicts with another access in 'space', two conflicting accesses may still be live, if they are read only for example
    pub fn overlaps_with_access_to(&self, other: &WorldAccessRange) -> bool {
        match (self, other) {
            (WorldAccessRange::Global, _) | (_, WorldAccessRange::Global) => true,
            _ => self == other,
        }
    }
}

impl PartialEq for WorldAccessRange {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ComponentOrResource(a), Self::ComponentOrResource(b)) => a == b,
            (Self::External(a), Self::External(b)) => {
                external_access_id(a) == external_access_id(b)
            }
            (Self::Global, Self::Global) => true,
            _ => false,
        }
    }
}

impl Eq for WorldAccessRange {}

impl std::hash::Hash for WorldAccessRange {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::ComponentOrResource(id) => id.hash(state),
            Self::External(owner) => external_access_id(owner).hash(state),
            Self::Global => {}
        }
    }
}

impl std::fmt::Debug for WorldAccessRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComponentOrResource(id) => {
                f.debug_tuple("ComponentOrResource").field(id).finish()
            }
            Self::External(owner) => f
                .debug_tuple("External")
                .field(&external_access_id(owner))
                .finish(),
            Self::Global => f.write_str("Global"),
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
        // another claim would wrap the count to 0, letting a write alias the existing reads
        let count_is_full = count == Self::MAX_COUNT;

        let allowed = !(conflicting_exclusive
            || aliasing_exclusive_access
            || access_is_forbidden
            || count_is_full);
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

        // an unmatched release must not wrap into a large count
        let new = count.saturating_sub(1);

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
    /// Accesses which can only be read, empty unless the set is a subset with read only accesses.
    /// Kept outside of the [`AccessBitField`]s so that only write claims pay for checking it.
    read_only: BitSet,
}

#[profiling::all_functions]
impl AccessByteSet {
    #[inline]
    /// Create a new empty access byte set
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            new_capacity_forbidden: false,
            read_only: BitSet::default(),
        }
    }

    /// Generates a new access byte set allowing only the given accesses
    pub fn from_allowed_list(iter: &[usize]) -> Self {
        Self::from_allowed_lists(iter, &[])
    }

    /// Generates a new access byte set allowing only the given accesses, the read only ones can't be written.
    ///
    /// The lists must be disjoint, callers split their accesses by whether they are written.
    /// Every other access is forbidden.
    pub fn from_allowed_lists(read_write: &[usize], read_only: &[usize]) -> Self {
        debug_assert!(
            read_only.iter().all(|i| !read_write.contains(i)),
            "read only and read write accesses overlap"
        );
        let max_index = read_write.iter().chain(read_only).max().unwrap_or(&0);
        let mut new = Self::new();
        new.new_capacity_forbidden = true;
        new.ensure_capacity(*max_index);
        for &i in read_write.iter().chain(read_only) {
            new.data[i] = new.data[i].with_forbidden(false);
        }
        for &i in read_only {
            new.read_only.insert(i);
        }

        new
    }

    #[inline]
    /// Returns true if the access can only be read
    pub fn is_read_only(&self, index: usize) -> bool {
        self.read_only.contains(index)
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
        self.data.iter_mut().for_each(|a| *a = a.clear_accesses());
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
    /// Any accesses claimed or released within the scope are rolled back once the closure returns or panics.
    /// Scopes can be nested, an inner scope rolls back only its own changes.
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
    /// The owner of a [`WorldAccessRange::External`] value is held until its last claim is released.
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

    /// Releases all active accesses, both individual and global.
    ///
    /// Inside a [`Self::with_scope`] only the accesses claimed in the innermost scope are released,
    /// the ones from before it are released by whoever claimed them.
    fn release_all_accesses(&self);
}

#[derive(Default, Debug, Clone)]
struct AccessMapInner {
    cant_access_global: bool,
    component_access: AccessByteSet,
    world_access: AccessBitField,
    external_accesses: HashMap<usize, ExternalAccess>,
    /// The length of [`Self::undo`] when each running [`DynamicSystemMeta::with_scope`] started, innermost last
    scope_marks: Vec<usize>,
    /// The changes made inside the running scopes, undone when they end
    undo: Vec<ScopeUndo>,
}

/// The claims on a value outside the world.
#[derive(Default, Debug, Clone)]
struct ExternalAccess {
    bits: AccessBitField,
    /// Keeps the value alive, set whenever `bits` counts a claim
    owner: Option<AccessOwner>,
}

/// A change to an [`AccessMapInner`], recorded so a scope can roll it back: the state of an access before it changed.
#[derive(Debug, Clone)]
enum ScopeUndo {
    Component(ComponentId, AccessBitField),
    /// `None` for an external access without an entry
    External(usize, Option<ExternalAccess>),
    Global(AccessBitField),
}

/// Owners which were released while the access map was locked,
/// dropped once it's unlocked since dropping one can free a value and run arbitrary code.
type Released = smallvec::SmallVec<[AccessOwner; 4]>;

#[profiling::all_functions]
impl AccessMapInner {
    /// Records the current state of an access before it changes, if a scope is running
    #[inline]
    fn record(&mut self, key: &WorldAccessRange) {
        if self.scope_marks.is_empty() {
            return;
        }
        let undo = match key {
            WorldAccessRange::ComponentOrResource(id) => {
                ScopeUndo::Component(*id, *self.component_access.entry_mut(id.index()))
            }
            WorldAccessRange::External(owner) => {
                let id = external_access_id(owner);
                ScopeUndo::External(id, self.external_accesses.get(&id).cloned())
            }
            WorldAccessRange::Global => ScopeUndo::Global(self.world_access),
        };
        self.undo.push(undo);
    }

    /// Undoes the changes recorded since the undo log had the given length
    fn rollback(&mut self, mark: usize, released: &mut Released) {
        while self.undo.len() > mark {
            let Some(undo) = self.undo.pop() else {
                break;
            };
            match undo {
                ScopeUndo::Component(id, previous) => {
                    *self.component_access.entry_mut(id.index()) = previous;
                }
                ScopeUndo::External(id, previous) => {
                    let replaced = match previous {
                        Some(previous) => self.external_accesses.insert(id, previous),
                        None => self.external_accesses.remove(&id),
                    };
                    released.extend(replaced.and_then(|access| access.owner));
                }
                ScopeUndo::Global(previous) => {
                    self.world_access = previous;
                }
            }
        }
    }

    pub fn claim_component_if_free(&mut self, id: ComponentId, write: bool) -> bool {
        let idx = id.index();
        if write && self.component_access.is_read_only(idx) {
            return false;
        }
        let elem = self.component_access.entry_mut(idx);
        let (free, new) = elem.claim_if_free(write);
        *elem = new;
        return free;
    }

    /// Claims the access if it's free, returns whether it was claimed,
    /// and the key's owner if the map didn't keep it, to be dropped once the map is unlocked.
    fn claim_if_free(&mut self, key: WorldAccessRange, write: bool) -> (bool, Option<AccessOwner>) {
        self.record(&key);
        match key {
            WorldAccessRange::ComponentOrResource(component_id) => {
                if self.world_access.is_write() || self.world_access.access_count() != 0 && write {
                    return (false, None);
                }
                (self.claim_component_if_free(component_id, write), None)
            }

            WorldAccessRange::External(owner) => {
                let access = self
                    .external_accesses
                    .entry(external_access_id(&owner))
                    .or_default();
                let (free, new) = access.bits.claim_if_free(write);
                access.bits = new;
                if free && access.owner.is_none() {
                    // held until the last claim is released
                    access.owner = Some(owner);
                    return (true, None);
                }
                (free, Some(owner))
            }

            WorldAccessRange::Global => {
                let elem = &mut self.world_access;
                if self.cant_access_global
                    || self
                        .component_access
                        .iter_accessed()
                        .any(|(_, a)| a.is_write() || write)
                {
                    return (false, None);
                }
                let (free, new) = elem.claim_if_free(write);
                *elem = new;
                (free, None)
            }
        }
    }

    /// Releases a claim, returns the owner of an external value once its last claim is released
    #[inline]
    fn clear_access(&mut self, key: &WorldAccessRange) -> Option<AccessOwner> {
        self.record(key);
        match key {
            WorldAccessRange::ComponentOrResource(component_id) => {
                let entry = self.component_access.entry_mut(component_id.index());
                *entry = entry.decrement_count();
            }

            WorldAccessRange::External(owner) => {
                if let Some(access) = self.external_accesses.get_mut(&external_access_id(owner)) {
                    access.bits = access.bits.decrement_count();
                    if access.bits.access_count() == 0 {
                        return access.owner.take();
                    }
                }
            }

            WorldAccessRange::Global => {
                self.world_access = self.world_access.decrement_count();
            }
        }
        None
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

    /// Whether this map covers the whole world, i.e. it was not created as a subset with [`Self::new_subset`].
    ///
    /// Subsets are used where other systems can run in parallel, such as non-exclusive script systems.
    pub fn is_exclusive(&self) -> bool {
        !self.0.lock().cant_access_global
    }

    fn claim(&self, key: WorldAccessRange, write: bool) -> bool {
        let (claimed, unused_owner) = self.0.lock().claim_if_free(key, write);
        // dropped once unlocked
        drop(unused_owner);
        claimed
    }
}

/// Ends a [`DynamicSystemMeta::with_scope`] when dropped, so the scope is rolled back even if its closure panics.
///
/// Scopes nest and end innermost first (also when a panic unwinds through them),
/// so the scope ending is always the last one in [`AccessMapInner::scope_marks`],
/// and it rolls back only the changes recorded after its mark.
struct ScopeEnd<'a>(&'a Mutex<AccessMapInner>);

impl Drop for ScopeEnd<'_> {
    fn drop(&mut self) {
        let mut released = Released::new();
        {
            let mut inner = self.0.lock();
            if let Some(mark) = inner.scope_marks.pop() {
                inner.rollback(mark, &mut released);
            }
        }
        drop(released);
    }
}

#[profiling::all_functions]
impl DynamicSystemMeta for AccessMap {
    fn release_access<K: Into<WorldAccessRange>>(&self, key: K) {
        let key = key.into();
        let released = self.0.lock().clear_access(&key);
        // dropped once unlocked, after the key, which may have been the last other owner
        drop(key);
        drop(released);
    }

    fn with_scope<O, F: FnOnce() -> O>(&self, f: F) -> O {
        // record the changes made inside the scope instead of copying the whole map
        {
            let mut inner = self.0.lock();
            let mark = inner.undo.len();
            inner.scope_marks.push(mark);
        }
        let _end = ScopeEnd(&self.0);
        f()
    }

    #[track_caller]
    fn claim_read_access<K: Into<WorldAccessRange>>(&self, key: K) -> bool {
        self.claim(key.into(), false)
    }

    #[track_caller]
    fn claim_write_access<K: Into<WorldAccessRange>>(&self, key: K) -> bool {
        self.claim(key.into(), true)
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
        let external = inner.external_accesses.values().flat_map(|v| {
            v.owner.iter().flat_map(move |owner| {
                (0..v.bits.access_count())
                    .map(|_| (WorldAccessRange::External(owner.clone()), v.bits.is_write()))
            })
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
        let mut released = Released::new();
        {
            let mut inner = self.0.lock();
            match inner.scope_marks.last().copied() {
                // the accesses from before the scope belong to callers, which release them themselves
                Some(mark) => inner.rollback(mark, &mut released),
                None => {
                    inner.component_access.clear();
                    released.extend(inner.external_accesses.drain().filter_map(|(_, a)| a.owner));
                    inner.world_access = Default::default();
                }
            }
        }
        drop(released);
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
    fn access_map_multiple_reads_block_write() {
        let access_map = AccessMap::new();
        assert!(access_map.claim_read_access(TestAccess(1)));
        assert!(!access_map.claim_write_access(TestAccess(1)));
        assert!(access_map.claim_read_access(TestAccess(1)));
        assert!(!access_map.claim_write_access(TestAccess(1)));
        access_map.release_access(TestAccess(1));
        assert!(!access_map.claim_write_access(TestAccess(1)));
        access_map.release_access(TestAccess(1));
        assert!(access_map.claim_write_access(TestAccess(1)));
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
    fn only_full_access_maps_are_exclusive() {
        assert!(AccessMap::new().is_exclusive());
        assert!(!AccessMap::new_subset(AccessByteSet::from_allowed_list(&[1])).is_exclusive());
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
    fn access_map_nested_scopes_unroll_only_their_own_accesses() {
        let access_map = AccessMap::new();
        access_map.with_scope(|| {
            assert!(access_map.claim_read_access(TestAccess(1)));
            access_map.with_scope(|| {
                assert!(access_map.claim_write_access(TestAccess(2)));
                // releasing an outer access inside the inner scope only lasts until it ends
                access_map.release_access(TestAccess(1));
                assert_eq!(access_map.list_accesses().len(), 1);
            });
            // the inner write is gone, the outer read is back
            assert_eq!(
                access_map.list_accesses(),
                vec![(WorldAccessRange::from(TestAccess(1)), false)]
            );
            assert!(access_map.claim_write_access(TestAccess(2)));
        });
        assert!(access_map.list_accesses().is_empty());
    }

    #[test]
    fn access_map_scope_unrolls_when_its_closure_panics() {
        let access_map = AccessMap::new();
        access_map.with_scope(|| {
            assert!(access_map.claim_read_access(TestAccess(1)));
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                access_map.with_scope(|| {
                    assert!(access_map.claim_write_access(TestAccess(2)));
                    panic!("binding panicked");
                })
            }));
            assert!(result.is_err());
            // the panicking scope ended, the outer one is intact
            assert_eq!(
                access_map.list_accesses(),
                vec![(WorldAccessRange::from(TestAccess(1)), false)]
            );
        });
        assert!(access_map.list_accesses().is_empty());
        // no scope is left open, so nothing is recorded anymore
        assert!(access_map.claim_write_access(TestAccess(3)));
        assert!(access_map.0.lock().undo.is_empty());
    }

    #[test]
    fn access_map_release_all_in_scope_releases_only_the_scope_accesses() {
        let access_map = AccessMap::new();
        assert!(access_map.claim_read_access(TestAccess(1)));
        access_map.with_scope(|| {
            assert!(access_map.claim_read_access(TestAccess(2)));
            access_map.with_scope(|| {
                assert!(access_map.claim_write_access(TestAccess(3)));
                access_map.release_all_accesses();
                // only the innermost scope's write is gone
                let mut accesses = access_map.list_accesses();
                accesses.sort_by_key(|(k, _)| format!("{k:?}"));
                assert_eq!(
                    accesses,
                    vec![
                        (WorldAccessRange::from(TestAccess(1)), false),
                        (WorldAccessRange::from(TestAccess(2)), false)
                    ]
                );
                // and the scope can keep claiming
                assert!(access_map.claim_write_access(TestAccess(3)));
            });
            assert_eq!(access_map.list_accesses().len(), 2);
        });
        assert_eq!(
            access_map.list_accesses(),
            vec![(WorldAccessRange::from(TestAccess(1)), false)]
        );
        // outside a scope everything is released
        access_map.release_all_accesses();
        assert!(access_map.list_accesses().is_empty());
    }

    #[test]
    fn access_map_owners_live_until_their_access_is_released() {
        let access_map = AccessMap::new();
        let owner: AccessOwner = Arc::new(());
        let count = || Arc::strong_count(&owner);
        let key = || WorldAccessRange::External(owner.clone());

        assert!(access_map.claim_read_access(key()));
        // a second claim keeps the first owner, its own is dropped
        assert!(access_map.claim_read_access(key()));
        assert_eq!(count(), 2);
        // a failed claim doesn't keep its owner either
        assert!(!access_map.claim_write_access(key()));
        assert_eq!(count(), 2);
        access_map.release_access(key());
        assert_eq!(count(), 2);
        access_map.release_access(key());
        assert_eq!(count(), 1);
    }

    #[test]
    fn access_map_scopes_drop_and_restore_owners() {
        let access_map = AccessMap::new();
        let owner: AccessOwner = Arc::new(());
        let count = || Arc::strong_count(&owner);
        let key = || WorldAccessRange::External(owner.clone());

        // claimed in a scope: dropped when it ends
        access_map.with_scope(|| {
            assert!(access_map.claim_write_access(key()));
            assert_eq!(count(), 2);
        });
        assert_eq!(count(), 1);

        // claimed before a scope and released in it: back when it ends
        assert!(access_map.claim_write_access(key()));
        access_map.with_scope(|| {
            access_map.release_access(key());
            assert!(access_map.list_accesses().is_empty());
        });
        assert_eq!(access_map.list_accesses(), vec![(key(), true)]);
        assert_eq!(count(), 2);

        // released outside any scope: dropped
        access_map.release_all_accesses();
        assert_eq!(count(), 1);
    }

    #[test]
    fn access_map_external_ids_are_owner_addresses() {
        let a: AccessOwner = Arc::new(1u8);
        let b: AccessOwner = Arc::new(1u8);
        assert_eq!(
            WorldAccessRange::External(a.clone()),
            WorldAccessRange::External(a.clone())
        );
        assert_ne!(
            WorldAccessRange::External(a.clone()),
            WorldAccessRange::External(b.clone())
        );
        let access_map = AccessMap::new();
        assert!(access_map.claim_write_access(WorldAccessRange::External(a)));
        assert!(access_map.claim_write_access(WorldAccessRange::External(b)));
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
    fn subset_map_read_only_accesses_cannot_be_written() {
        let subset_access_map =
            AccessMap::new_subset(AccessByteSet::from_allowed_lists(&[1], &[2]));

        assert!(!subset_access_map.claim_write_access(TestAccess(2)));
        assert!(subset_access_map.claim_read_access(TestAccess(2)));
        assert!(subset_access_map.claim_read_access(TestAccess(2)));
        assert!(subset_access_map.claim_write_access(TestAccess(1)));

        // the permission survives releasing all accesses
        subset_access_map.release_all_accesses();
        assert!(!subset_access_map.claim_write_access(TestAccess(2)));
    }

    #[test]
    fn access_counts_do_not_overflow() {
        let access_map = AccessMap::default();
        let mut claimed = 0;
        while access_map.claim_read_access(TestAccess(0)) {
            claimed += 1;
            assert!(claimed < 1000, "claims never fail");
        }
        assert_eq!(claimed, 63);
        // all reads are still counted, so writes stay blocked
        assert!(!access_map.claim_write_access(TestAccess(0)));
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
