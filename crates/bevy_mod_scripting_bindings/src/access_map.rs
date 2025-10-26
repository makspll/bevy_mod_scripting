//! A map of access claims used to safely and dynamically access the world.

use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_mod_scripting_display::{DisplayWithTypeInfo, GetTypeInfo, WithTypeInfo};
use bevy_platform::collections::{HashMap, HashSet};

use ::bevy_ecs::{component::ComponentId, world::unsafe_world_cell::UnsafeWorldCell};
use bevy_ecs::{component::Component, resource::Resource};
use bevy_log::error;
use parking_lot::Mutex;
use smallvec::SmallVec;
use std::hash::{BuildHasherDefault, Hasher};

use crate::error::InteropError;

use super::{ReflectAllocationId, ReflectBase};

#[derive(Debug, Clone, PartialEq, Eq)]
/// An owner of an access claim and the code location of the claim.
pub struct ClaimOwner {
    location: std::panic::Location<'static>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A count of the number of readers and writers of an access claim.
pub struct AccessCount {
    /// The number of readers including thread information
    read_by: SmallVec<[ClaimOwner; 1]>,
    /// If the current read is a write access, this will be set
    written: bool,
}

impl Default for AccessCount {
    fn default() -> Self {
        Self::new()
    }
}

#[profiling::all_functions]
impl AccessCount {
    fn new() -> Self {
        Self {
            read_by: Default::default(),
            written: false,
        }
    }

    fn can_read(&self) -> bool {
        !self.written
    }

    fn can_write(&self) -> bool {
        self.read_by.is_empty() && !self.written
    }

    fn as_location(&self) -> Option<std::panic::Location<'static>> {
        self.read_by.first().map(|o| o.location)
    }

    fn readers(&self) -> usize {
        self.read_by.len()
    }
}

/// For structs which can be mapped to a u64 index
pub trait AccessMapKey {
    /// Convert the key to an index
    ///
    /// The key 0 must not be be used as it's reserved for global access
    fn as_index(&self) -> u64;

    /// Convert an index back to the original struct
    fn from_index(value: u64) -> Self;
}

#[profiling::all_functions]
impl AccessMapKey for u64 {
    fn as_index(&self) -> u64 {
        *self
    }

    fn from_index(value: u64) -> Self {
        value
    }
}

/// Describes kinds of base value we are accessing via reflection
#[derive(PartialEq, Eq, Copy, Clone, Hash, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub enum ReflectAccessKind {
    /// Accessing a component or resource
    ComponentOrResource,
    /// Accessing an owned value
    Allocation,
    /// Accessing the world
    Global,
}

/// Describes the id pointing to the base value we are accessing via reflection, for components and resources this is the ComponentId
/// for script owned values this is an allocationId, this is used to ensure we have permission to access the value.
#[derive(PartialEq, Eq, Copy, Clone, Hash, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub struct ReflectAccessId {
    pub(crate) kind: ReflectAccessKind,
    pub(crate) id: u64,
}

impl DisplayWithTypeInfo for ReflectAccessId {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        match self.kind {
            ReflectAccessKind::ComponentOrResource => {
                write!(
                    f,
                    "Component or resource: {}",
                    WithTypeInfo::new_with_opt_info(
                        &ComponentId::new(self.id as usize),
                        type_info_provider
                    )
                )
            }
            ReflectAccessKind::Allocation => write!(
                f,
                "Allocation to: {}",
                WithTypeInfo::new_with_opt_info(
                    &ReflectAllocationId::new(self.id),
                    type_info_provider
                )
            ),
            ReflectAccessKind::Global => write!(f, "World(Global)"),
        }
    }
}

#[profiling::all_functions]
impl AccessMapKey for ReflectAccessId {
    fn as_index(&self) -> u64 {
        // project two linear non-negative ranges [0,inf] to a single linear non-negative range, offset by 1 to avoid 0
        // y1 = 2x - 0 + 2 = 2x + 2
        // y2 = 2x - 1 + 2 = 2x + 1
        match self.kind {
            ReflectAccessKind::ComponentOrResource => (self.id * 2) + 2,
            ReflectAccessKind::Allocation => (self.id * 2) + 1,
            ReflectAccessKind::Global => 0,
        }
    }

    fn from_index(value: u64) -> Self {
        // reverse the projection
        // x1 = (y1 - 2) / 2
        // x2 = (y2 - 1) / 2

        match value {
            0 => ReflectAccessId {
                kind: ReflectAccessKind::Global,
                id: 0,
            },
            v if v % 2 == 0 => ReflectAccessId {
                kind: ReflectAccessKind::ComponentOrResource,
                id: (v - 2) / 2,
            },
            v => ReflectAccessId {
                kind: ReflectAccessKind::Allocation,
                id: (v - 1) / 2,
            },
        }
    }
}

#[profiling::all_functions]
impl ReflectAccessId {
    /// Creates a new access id for the global world
    pub fn for_global() -> Self {
        Self {
            kind: ReflectAccessKind::Global,
            id: 0,
        }
    }

    /// Creates a new access id for a resource
    pub fn for_resource<R: Resource>(cell: &UnsafeWorldCell) -> Result<Self, InteropError> {
        let resource_id = cell.components().resource_id::<R>().ok_or_else(|| {
            InteropError::unregistered_component_or_resource_type(std::any::type_name::<R>())
        })?;

        Ok(Self {
            kind: ReflectAccessKind::ComponentOrResource,
            id: resource_id.index() as u64,
        })
    }

    /// Creates a new access id for a component
    pub fn for_component<C: Component>(cell: &UnsafeWorldCell) -> Result<Self, InteropError> {
        let component_id = cell.components().component_id::<C>().ok_or_else(|| {
            InteropError::unregistered_component_or_resource_type(std::any::type_name::<C>())
        })?;

        Ok(Self::for_component_id(component_id))
    }

    /// Creates a new access id for a component id
    pub fn for_allocation(id: ReflectAllocationId) -> Self {
        Self {
            kind: ReflectAccessKind::Allocation,
            id: id.id(),
        }
    }

    /// Creates a new access id for a component id
    pub fn for_component_id(id: ComponentId) -> Self {
        Self {
            kind: ReflectAccessKind::ComponentOrResource,
            id: id.index() as u64,
        }
    }

    /// Creates a new access id for a reference
    pub fn for_reference(base: ReflectBase) -> Self {
        match base {
            ReflectBase::Resource(id) => Self::for_component_id(id),
            ReflectBase::Component(_, id) => Self::for_component_id(id),
            ReflectBase::Owned(id) => Self::for_allocation(id),
            ReflectBase::Asset(_, assets_resource_id) => Self::for_component_id(assets_resource_id),
        }
    }
}

impl From<ComponentId> for ReflectAccessId {
    fn from(id: ComponentId) -> Self {
        ReflectAccessId::for_component_id(id)
    }
}

impl From<ReflectAllocationId> for ReflectAccessId {
    fn from(id: ReflectAllocationId) -> Self {
        ReflectAccessId::for_allocation(id)
    }
}

#[profiling::all_functions]
impl From<ReflectAccessId> for ComponentId {
    fn from(val: ReflectAccessId) -> Self {
        ComponentId::new(val.id as usize)
    }
}

#[profiling::all_functions]
impl From<ReflectAccessId> for ReflectAllocationId {
    fn from(val: ReflectAccessId) -> Self {
        ReflectAllocationId::new(val.id)
    }
}

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

    /// Returns `true` if the world is exclusively locked.
    ///
    /// When exclusively locked, no additional individual or global accesses may be claimed.
    fn is_locked_exclusively(&self) -> bool;

    /// Retrieves the code location where the global lock was claimed (if any).
    ///
    /// This is useful for debugging conflicts involving the global access lock.
    fn global_access_location(&self) -> Option<std::panic::Location<'static>>;

    /// Attempts to claim read access for the given key.
    ///
    /// Returns `true` if the read access is successfully claimed. The claim will fail if
    /// the key is currently locked for write or if a global lock is active.
    #[track_caller]
    fn claim_read_access<K: AccessMapKey>(&self, key: K) -> bool;

    /// Attempts to claim write access for the given key.
    ///
    /// Returns `true` if the write access is successfully claimed. Write access fails if any
    /// read or write access is active for the key or if a global lock is held.
    #[track_caller]
    fn claim_write_access<K: AccessMapKey>(&self, key: K) -> bool;

    /// Attempts to claim a global access lock.
    ///
    /// Returns `true` if the global access is successfully claimed. Global access precludes any
    /// individual accesses until it is released.
    #[track_caller]
    fn claim_global_access(&self) -> bool;

    /// Releases an access claimed for the provided key.
    ///
    /// # Panics
    ///
    /// Panics if the access is released by a thread different from the one that claimed it.
    fn release_access<K: AccessMapKey>(&self, key: K);

    /// Releases an active global access lock.
    ///
    /// # Panics
    ///
    /// Panics if the global access is released from a thread other than the one that claimed it.
    fn release_global_access(&self);

    /// Returns a list of active accesses.
    ///
    /// The list is provided as key and corresponding access count pairs.
    fn list_accesses<K: AccessMapKey>(&self) -> Vec<(K, AccessCount)>;

    /// Returns the number of active individual accesses.
    ///
    /// In the case of a global lock, this method considers that as a single active access.
    fn count_accesses(&self) -> usize;

    /// Releases all active accesses.
    ///
    /// Both individual and global accesses will be removed.
    fn release_all_accesses(&self);

    /// Returns the location where the specified key was first accessed.
    ///
    /// This is useful for debugging and tracing access failures.
    fn access_location<K: AccessMapKey>(&self, key: K) -> Option<std::panic::Location<'static>>;

    /// Returns the location of the first access among all keys.
    ///
    /// This can assist in identifying the origin of access conflicts.
    fn access_first_location(&self) -> Option<std::panic::Location<'static>>;
}

#[derive(Default)]
/// A hash function which doesn't do much. for maps which expect very small hashes.
/// Assumes only needs to hash u64 values, unsafe otherwise
struct SmallIdentityHash(u64);
impl Hasher for SmallIdentityHash {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        // concat all bytes via &&
        // this is a bit of a hack, but it works for our use case
        // and is faster than using a hash function
        #[allow(clippy::expect_used, reason = "cannot handle this panic otherwise")]
        let arr: &[u8; 8] = bytes.try_into().expect("this hasher only supports u64");
        // depending on endianess

        #[cfg(target_endian = "big")]
        let word = u64::from_be_bytes(*arr);
        #[cfg(target_endian = "little")]
        let word = u64::from_le_bytes(*arr);
        self.0 = word
    }
}

#[derive(Default, Debug, Clone)]
struct AccessMapInner {
    individual_accesses: HashMap<u64, AccessCount, BuildHasherDefault<SmallIdentityHash>>,
    global_lock: AccessCount,
}

#[profiling::all_functions]
impl AccessMapInner {
    #[inline]
    fn entry(&self, key: u64) -> Option<&AccessCount> {
        self.individual_accesses.get(&key)
    }

    #[inline]
    fn entry_mut(&mut self, key: u64) -> Option<&mut AccessCount> {
        self.individual_accesses.get_mut(&key)
    }

    #[inline]
    fn entry_or_default(&mut self, key: u64) -> &mut AccessCount {
        self.individual_accesses.entry(key).or_default()
    }

    #[inline]
    fn remove(&mut self, key: u64) {
        self.individual_accesses.remove(&key);
    }
}

const GLOBAL_KEY: u64 = 0;

#[profiling::all_functions]
impl DynamicSystemMeta for AccessMap {
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

    fn is_locked_exclusively(&self) -> bool {
        let inner = self.0.lock();
        // If global_lock cannot be written, then it is locked exclusively.
        !inner.global_lock.can_write()
    }

    fn global_access_location(&self) -> Option<std::panic::Location<'static>> {
        let inner = self.0.lock();
        inner.global_lock.as_location()
    }

    #[track_caller]
    fn claim_read_access<K: AccessMapKey>(&self, key: K) -> bool {
        let mut inner = self.0.lock();

        if !inner.global_lock.can_write() {
            return false;
        }

        let key = key.as_index();
        if key == GLOBAL_KEY {
            error!("Trying to claim read access to global key, this is not allowed");
            return false;
        }

        let entry = inner.entry_or_default(key);

        if entry.can_read() {
            entry.read_by.push(ClaimOwner {
                location: *std::panic::Location::caller(),
            });
            true
        } else {
            false
        }
    }

    #[track_caller]
    fn claim_write_access<K: AccessMapKey>(&self, key: K) -> bool {
        let mut inner = self.0.lock();

        if !inner.global_lock.can_write() {
            return false;
        }

        let key = key.as_index();
        if key == GLOBAL_KEY {
            error!("Trying to claim write access to global key, this is not allowed");
            return false;
        }

        let entry = inner.entry_or_default(key);

        if entry.can_write() {
            entry.read_by.push(ClaimOwner {
                location: *std::panic::Location::caller(),
            });
            entry.written = true;
            true
        } else {
            false
        }
    }

    #[track_caller]
    fn claim_global_access(&self) -> bool {
        let mut inner = self.0.lock();

        if !inner.individual_accesses.is_empty() || !inner.global_lock.can_write() {
            return false;
        }
        inner.global_lock.read_by.push(ClaimOwner {
            location: *std::panic::Location::caller(),
        });
        inner.global_lock.written = true;
        true
    }

    fn release_access<K: AccessMapKey>(&self, key: K) {
        let mut inner = self.0.lock();
        let key = key.as_index();

        if let Some(entry) = inner.entry_mut(key) {
            entry.written = false;
            entry.read_by.pop();
            if entry.readers() == 0 {
                inner.remove(key);
            }
        }
    }

    fn release_global_access(&self) {
        let mut inner = self.0.lock();
        inner.global_lock.read_by.pop();
        inner.global_lock.written = false;
    }

    fn list_accesses<K: AccessMapKey>(&self) -> Vec<(K, AccessCount)> {
        let inner = self.0.lock();
        inner
            .individual_accesses
            .iter()
            .map(|(key, a)| (K::from_index(*key), a.clone()))
            .collect()
    }

    fn count_accesses(&self) -> usize {
        if self.is_locked_exclusively() {
            1
        } else {
            let inner = self.0.lock();
            inner.individual_accesses.len()
        }
    }

    fn release_all_accesses(&self) {
        let mut inner = self.0.lock();
        inner.individual_accesses.clear();
        // Release global access if held.
        inner.global_lock.written = false;
        inner.global_lock.read_by.clear();
    }

    fn access_location<K: AccessMapKey>(&self, key: K) -> Option<std::panic::Location<'static>> {
        let inner = self.0.lock();
        if key.as_index() == 0 {
            // it blocked by individual access
            inner.global_lock.as_location().or_else(|| {
                inner
                    .individual_accesses
                    .iter()
                    .next()
                    .and_then(|(_, access_count)| access_count.as_location())
            })
        } else {
            inner
                .entry(key.as_index())
                .and_then(|access| access.as_location())
        }
    }

    fn access_first_location(&self) -> Option<std::panic::Location<'static>> {
        let inner = self.0.lock();
        inner
            .individual_accesses
            .iter()
            .next()
            .and_then(|(_, access)| access.as_location())
    }
}

/// An inverse of [`AccessMap`], It limits the accesses allowed to be claimed to those in a pre-specified subset.
pub struct SubsetAccessMap {
    inner: AccessMap,
    subset: Box<dyn Fn(u64) -> bool + Send + Sync + 'static>,
}

#[profiling::all_functions]
impl SubsetAccessMap {
    /// Creates a new subset access map with the provided subset of ID's as well as a exception function.
    pub fn new(
        subset: impl IntoIterator<Item = impl AccessMapKey>,
        exception: impl Fn(u64) -> bool + Send + Sync + 'static,
    ) -> Self {
        let set = subset
            .into_iter()
            .map(|k| k.as_index())
            .collect::<HashSet<_>>();
        Self {
            inner: Default::default(),
            subset: Box::new(move |id| set.contains(&id) || exception(id)),
        }
    }

    fn in_subset(&self, key: u64) -> bool {
        (self.subset)(key)
    }
}

#[profiling::all_functions]
impl DynamicSystemMeta for SubsetAccessMap {
    fn with_scope<O, F: FnOnce() -> O>(&self, f: F) -> O {
        self.inner.with_scope(f)
    }

    fn is_locked_exclusively(&self) -> bool {
        self.inner.is_locked_exclusively()
    }

    fn global_access_location(&self) -> Option<std::panic::Location<'static>> {
        self.inner.global_access_location()
    }

    fn claim_read_access<K: AccessMapKey>(&self, key: K) -> bool {
        if !self.in_subset(key.as_index()) {
            return false;
        }
        self.inner.claim_read_access(key)
    }

    fn claim_write_access<K: AccessMapKey>(&self, key: K) -> bool {
        if !self.in_subset(key.as_index()) {
            return false;
        }
        self.inner.claim_write_access(key)
    }

    fn claim_global_access(&self) -> bool {
        if !self.in_subset(0) {
            return false;
        }
        self.inner.claim_global_access()
    }

    fn release_access<K: AccessMapKey>(&self, key: K) {
        self.inner.release_access(key);
    }

    fn release_global_access(&self) {
        self.inner.release_global_access();
    }

    fn list_accesses<K: AccessMapKey>(&self) -> Vec<(K, AccessCount)> {
        self.inner.list_accesses()
    }

    fn count_accesses(&self) -> usize {
        self.inner.count_accesses()
    }

    fn release_all_accesses(&self) {
        self.inner.release_all_accesses();
    }

    fn access_location<K: AccessMapKey>(&self, key: K) -> Option<std::panic::Location<'static>> {
        self.inner.access_location(key)
    }

    fn access_first_location(&self) -> Option<std::panic::Location<'static>> {
        self.inner.access_first_location()
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

    fn is_locked_exclusively(&self) -> bool {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.is_locked_exclusively(),
            AnyAccessMap::SubsetAccessMap(map) => map.is_locked_exclusively(),
        }
    }

    fn global_access_location(&self) -> Option<std::panic::Location<'static>> {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.global_access_location(),
            AnyAccessMap::SubsetAccessMap(map) => map.global_access_location(),
        }
    }

    fn claim_read_access<K: AccessMapKey>(&self, key: K) -> bool {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.claim_read_access(key),
            AnyAccessMap::SubsetAccessMap(map) => map.claim_read_access(key),
        }
    }

    fn claim_write_access<K: AccessMapKey>(&self, key: K) -> bool {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.claim_write_access(key),
            AnyAccessMap::SubsetAccessMap(map) => map.claim_write_access(key),
        }
    }

    fn claim_global_access(&self) -> bool {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.claim_global_access(),
            AnyAccessMap::SubsetAccessMap(map) => map.claim_global_access(),
        }
    }

    fn release_access<K: AccessMapKey>(&self, key: K) {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.release_access(key),
            AnyAccessMap::SubsetAccessMap(map) => map.release_access(key),
        }
    }

    fn release_global_access(&self) {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.release_global_access(),
            AnyAccessMap::SubsetAccessMap(map) => map.release_global_access(),
        }
    }

    fn list_accesses<K: AccessMapKey>(&self) -> Vec<(K, AccessCount)> {
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

    fn access_location<K: AccessMapKey>(&self, key: K) -> Option<std::panic::Location<'static>> {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.access_location(key),
            AnyAccessMap::SubsetAccessMap(map) => map.access_location(key),
        }
    }

    fn access_first_location(&self) -> Option<std::panic::Location<'static>> {
        match self {
            AnyAccessMap::UnlimitedAccessMap(map) => map.access_first_location(),
            AnyAccessMap::SubsetAccessMap(map) => map.access_first_location(),
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

/// A macro for claiming access to a value for reading
macro_rules! with_access_read {
    ($access_map:expr, $id:expr, $msg:expr, $body:block) => {{
        if !$crate::access_map::DynamicSystemMeta::claim_read_access($access_map, $id) {
            Err($crate::error::InteropError::cannot_claim_access(
                $id,
                $crate::access_map::DynamicSystemMeta::access_location($access_map, $id),
                $msg,
            ))
        } else {
            let result = $body;
            $crate::access_map::DynamicSystemMeta::release_access($access_map, $id);
            Ok(result)
        }
    }};
}

pub(crate) use with_access_read;
/// A macro for claiming access to a value for writing
macro_rules! with_access_write {
    ($access_map:expr, $id:expr, $msg:expr, $body:block) => {
        if !$crate::access_map::DynamicSystemMeta::claim_write_access($access_map, $id) {
            Err($crate::error::InteropError::cannot_claim_access(
                $id,
                $crate::access_map::DynamicSystemMeta::access_location($access_map, $id),
                $msg,
            ))
        } else {
            let result = $body;
            $crate::access_map::DynamicSystemMeta::release_access($access_map, $id);
            Ok(result)
        }
    };
}
pub(crate) use with_access_write;

/// A macro for claiming global access
macro_rules! with_global_access {
    ($access_map:expr, $msg:expr, $body:block) => {
        if !$crate::access_map::DynamicSystemMeta::claim_global_access($access_map) {
            Err($crate::error::InteropError::cannot_claim_access(
                $crate::access_map::ReflectAccessId::for_global(),
                $crate::access_map::DynamicSystemMeta::access_location(
                    $access_map,
                    $crate::access_map::ReflectAccessId::for_global(),
                ),
                $msg,
            ))
        } else {
            #[allow(clippy::redundant_closure_call)]
            let result = (|| $body)();
            $crate::access_map::DynamicSystemMeta::release_global_access($access_map);
            Ok(result)
        }
    };
}

pub(crate) use with_global_access;

#[cfg(test)]
mod test {
    use std::hash::Hash;

    use super::*;

    #[test]
    fn access_map_list_accesses() {
        let access_map = AccessMap::default();

        access_map.claim_read_access(1);
        access_map.claim_write_access(2);

        let accesses = access_map.list_accesses::<u64>();

        assert_eq!(accesses.len(), 2);
        let access_0 = accesses.iter().find(|(k, _)| *k == 1).unwrap();
        let access_1 = accesses.iter().find(|(k, _)| *k == 2).unwrap();

        assert_eq!(access_0.1.readers(), 1);
        assert_eq!(access_1.1.readers(), 1);

        assert!(!access_0.1.written);
        assert!(access_1.1.written);
    }

    #[test]
    fn subset_access_map_list_accesses() {
        let access_map = AccessMap::default();
        let subset_access_map = SubsetAccessMap {
            inner: access_map,
            subset: Box::new(|id| id == 1 || id == 2),
        };

        subset_access_map.claim_read_access(1);
        subset_access_map.claim_write_access(2);

        let accesses = subset_access_map.list_accesses::<u64>();

        assert_eq!(accesses.len(), 2);
        let access_0 = accesses.iter().find(|(k, _)| *k == 1).unwrap();
        let access_1 = accesses.iter().find(|(k, _)| *k == 2).unwrap();

        assert_eq!(access_0.1.readers(), 1);
        assert_eq!(access_1.1.readers(), 1);

        assert!(!access_0.1.written);
        assert!(access_1.1.written);
    }

    #[test]
    fn access_map_read_access_blocks_write() {
        let access_map = AccessMap::default();

        assert!(access_map.claim_read_access(1));
        assert!(!access_map.claim_write_access(1));
        access_map.release_access(1);
        assert!(access_map.claim_write_access(1));
    }

    #[test]
    fn subset_access_map_read_access_blocks_write() {
        let access_map = AccessMap::default();
        let subset_access_map = SubsetAccessMap {
            inner: access_map,
            subset: Box::new(|id| id == 1),
        };

        assert!(subset_access_map.claim_read_access(1));
        assert!(!subset_access_map.claim_write_access(1));
        subset_access_map.release_access(1);
        assert!(subset_access_map.claim_write_access(1));
    }

    #[test]
    fn access_map_write_access_blocks_read() {
        let access_map = AccessMap::default();

        assert!(access_map.claim_write_access(1));
        assert!(!access_map.claim_read_access(1));
        access_map.release_access(1);
        assert!(access_map.claim_read_access(1));
    }

    #[test]
    fn subset_access_map_write_access_blocks_read() {
        let access_map = AccessMap::default();
        let subset_access_map = SubsetAccessMap {
            inner: access_map,
            subset: Box::new(|id| id == 1),
        };

        assert!(subset_access_map.claim_write_access(1));
        assert!(!subset_access_map.claim_read_access(1));
        subset_access_map.release_access(1);
        assert!(subset_access_map.claim_read_access(1));
    }

    #[test]
    fn access_map_global_access_blocks_all() {
        let access_map = AccessMap::default();

        assert!(access_map.claim_global_access());
        assert!(!access_map.claim_read_access(1));
        assert!(!access_map.claim_write_access(1));
        access_map.release_global_access();
        assert!(access_map.claim_write_access(1));
        access_map.release_access(1);
        assert!(access_map.claim_read_access(1));
    }

    #[test]
    fn subset_access_map_global_access_blocks_all() {
        let access_map = AccessMap::default();
        let subset_access_map = SubsetAccessMap {
            inner: access_map,
            subset: Box::new(|id| id == 1 || id == 0),
        };

        assert!(subset_access_map.claim_global_access());
        assert!(!subset_access_map.claim_read_access(1));
        assert!(!subset_access_map.claim_write_access(1));
        subset_access_map.release_global_access();
        assert!(subset_access_map.claim_write_access(1));
        subset_access_map.release_access(1);
        assert!(subset_access_map.claim_read_access(1));
    }

    #[test]
    fn access_map_any_access_blocks_global() {
        let access_map = AccessMap::default();

        assert!(access_map.claim_read_access(1));
        assert!(!access_map.claim_global_access());
        access_map.release_access(1);

        assert!(access_map.claim_write_access(1));
        assert!(!access_map.claim_global_access());
    }

    #[test]
    fn subset_map_any_access_blocks_global() {
        let access_map = AccessMap::default();
        let subset_access_map = SubsetAccessMap {
            inner: access_map,
            subset: Box::new(|id| id == 0 || id == 1),
        };

        assert!(subset_access_map.claim_read_access(1));
        assert!(!subset_access_map.claim_global_access());
        subset_access_map.release_access(1);

        assert!(subset_access_map.claim_write_access(1));
        assert!(!subset_access_map.claim_global_access());
    }

    #[test]
    fn as_and_from_index_for_access_id_non_overlapping() {
        let global = ReflectAccessId::for_global();

        let first_component = ReflectAccessId {
            kind: ReflectAccessKind::ComponentOrResource,
            id: 0,
        };

        let first_allocation = ReflectAccessId {
            kind: ReflectAccessKind::Allocation,
            id: 0,
        };

        let second_component = ReflectAccessId {
            kind: ReflectAccessKind::ComponentOrResource,
            id: 1,
        };

        let second_allocation = ReflectAccessId {
            kind: ReflectAccessKind::Allocation,
            id: 1,
        };

        assert_eq!(global.as_index(), 0);
        assert_eq!(first_allocation.as_index(), 1);
        assert_eq!(first_component.as_index(), 2);
        assert_eq!(second_allocation.as_index(), 3);
        assert_eq!(second_component.as_index(), 4);

        assert_eq!(ReflectAccessId::from_index(0), global);
        assert_eq!(ReflectAccessId::from_index(1), first_allocation);
        assert_eq!(ReflectAccessId::from_index(2), first_component);
        assert_eq!(ReflectAccessId::from_index(3), second_allocation);
        assert_eq!(ReflectAccessId::from_index(4), second_component);
    }

    #[test]
    fn access_map_with_scope_unrolls_individual_accesses() {
        let access_map = AccessMap::default();
        // Claim a read access outside the scope
        assert!(access_map.claim_read_access(3));

        // Inside with_scope, claim additional accesses
        access_map.with_scope(|| {
            assert!(access_map.claim_read_access(1));
            assert!(access_map.claim_write_access(2));
            // At this point, individual_accesses contains keys 0, 1 and 2.
            let accesses = access_map.list_accesses::<u64>();
            assert_eq!(accesses.len(), 3);
        });

        // After with_scope returns, accesses claimed inside (keys 1 and 2) are unrolled.
        let accesses = access_map.list_accesses::<u64>();
        // Only the access claimed outside (key 3) remains.
        assert_eq!(accesses.len(), 1);
        let (k, count) = &accesses[0];
        assert_eq!(*k, 3);
        // The outside access remains valid.
        assert!(count.readers() > 0);
    }

    #[test]
    fn subset_map_with_scope_unrolls_individual_accesses() {
        let access_map = AccessMap::default();
        let subset_access_map = SubsetAccessMap {
            inner: access_map,
            subset: Box::new(|id| id == 1 || id == 2 || id == 3),
        };

        // Claim a read access outside the scope
        assert!(subset_access_map.claim_read_access(3));

        // Inside with_scope, claim additional accesses
        subset_access_map.with_scope(|| {
            assert!(subset_access_map.claim_read_access(1));
            assert!(subset_access_map.claim_write_access(2));
            // At this point, individual_accesses contains keys 0, 1 and 2.
            let accesses = subset_access_map.list_accesses::<u64>();
            assert_eq!(accesses.len(), 3);
        });

        // After with_scope returns, accesses claimed inside (keys 1 and 2) are unrolled.
        let accesses = subset_access_map.list_accesses::<u64>();
        // Only the access claimed outside (key 3) remains.
        assert_eq!(accesses.len(), 1);
        let (k, count) = &accesses[0];
        assert_eq!(*k, 3);
        // The outside access remains valid.
        assert!(count.readers() > 0);
    }

    #[test]
    fn access_map_with_scope_unrolls_global_accesses() {
        let access_map = AccessMap::default();

        access_map.with_scope(|| {
            assert!(access_map.claim_global_access());
            // At this point, global_access is claimed.
            assert!(!access_map.claim_read_access(1));
        });

        let accesses = access_map.list_accesses::<u64>();
        assert_eq!(accesses.len(), 0);
    }

    #[test]
    fn subset_map_with_scope_unrolls_global_accesses() {
        let access_map = AccessMap::default();
        let subset_access_map = SubsetAccessMap {
            inner: access_map,
            subset: Box::new(|id| id == 0 || id == 1),
        };

        subset_access_map.with_scope(|| {
            assert!(subset_access_map.claim_global_access());
            // At this point, global_access is claimed.
            assert!(!subset_access_map.claim_read_access(1));
        });

        let accesses = subset_access_map.list_accesses::<u64>();
        assert_eq!(accesses.len(), 0);
    }

    #[test]
    fn access_map_count_accesses_counts_globals() {
        let access_map = AccessMap::default();

        // Initially, no accesses are active.
        assert_eq!(access_map.count_accesses(), 0);

        // Claim global access. When global access is active,
        // count_accesses should return 1.
        assert!(access_map.claim_global_access());
        assert_eq!(access_map.count_accesses(), 1);
        access_map.release_global_access();

        // Now claim individual accesses.
        assert!(access_map.claim_read_access(1));
        assert!(access_map.claim_write_access(2));
        // Since two separate keys were claimed, count_accesses should return 2.
        assert_eq!(access_map.count_accesses(), 2);

        // Cleanup individual accesses.
        access_map.release_access(1);
        access_map.release_access(2);
    }

    #[test]
    fn subset_map_count_accesses_counts_globals() {
        let access_map = AccessMap::default();
        let subset_access_map = SubsetAccessMap {
            inner: access_map,
            subset: Box::new(|id| id == 0 || id == 1 || id == 2),
        };

        // Initially, no accesses are active.
        assert_eq!(subset_access_map.count_accesses(), 0);

        // Claim global access. When global access is active,
        // count_accesses should return 1.
        assert!(subset_access_map.claim_global_access());
        assert_eq!(subset_access_map.count_accesses(), 1);
        subset_access_map.release_global_access();

        // Now claim individual accesses.
        assert!(subset_access_map.claim_read_access(1));
        assert!(subset_access_map.claim_write_access(2));
        // Since two separate keys were claimed, count_accesses should return 2.
        assert_eq!(subset_access_map.count_accesses(), 2);

        // Cleanup individual accesses.
        subset_access_map.release_access(1);
        subset_access_map.release_access(2);
    }

    #[test]
    fn access_map_location_is_tracked_for_all_types_of_accesses() {
        let access_map = AccessMap::default();

        assert!(access_map.claim_global_access());
        assert!(
            access_map
                .access_location(ReflectAccessId::for_global())
                .is_some()
        );
        access_map.release_global_access();

        // Claim a read access
        assert!(access_map.claim_read_access(1));
        assert!(access_map.access_location(1).is_some());
        access_map.release_access(1);

        // Claim a write access
        assert!(access_map.claim_write_access(2));
        assert!(access_map.access_location(2).is_some());
        access_map.release_access(2);
    }

    #[test]
    fn subset_map_location_is_tracked_for_all_types_of_accesses() {
        let access_map = AccessMap::default();
        let subset_access_map = SubsetAccessMap {
            inner: access_map,
            subset: Box::new(|id| id == 0 || id == 1 || id == 2),
        };

        assert!(subset_access_map.claim_global_access());
        assert!(
            subset_access_map
                .access_location(ReflectAccessId::for_global())
                .is_some()
        );
        subset_access_map.release_global_access();

        // Claim a read access
        assert!(subset_access_map.claim_read_access(1));
        assert!(subset_access_map.access_location(1).is_some());
        subset_access_map.release_access(1);

        // Claim a write access
        assert!(subset_access_map.claim_write_access(2));
        assert!(subset_access_map.access_location(2).is_some());
        subset_access_map.release_access(2);
    }

    #[test]
    fn subset_map_prevents_access_to_out_of_subset_access() {
        let access_map = AccessMap::default();
        let subset_access_map = SubsetAccessMap {
            inner: access_map,
            subset: Box::new(|id| id == 1),
        };

        assert!(!subset_access_map.claim_read_access(2));
        assert!(!subset_access_map.claim_write_access(2));
        assert!(!subset_access_map.claim_global_access());
    }

    #[test]
    fn subset_map_retains_subset_in_scope() {
        let access_map = AccessMap::default();
        let subset_access_map = SubsetAccessMap {
            inner: access_map,
            subset: Box::new(|id| id == 1),
        };

        subset_access_map.with_scope(|| {
            assert!(subset_access_map.claim_read_access(1));
            assert!(!subset_access_map.claim_read_access(2));
            assert!(!subset_access_map.claim_write_access(2));
        });

        assert!(subset_access_map.claim_read_access(1));
        assert!(!subset_access_map.claim_read_access(2));
        assert!(!subset_access_map.claim_write_access(2));
    }

    #[test]
    fn test_hasher_on_u64() {
        let mut hasher = SmallIdentityHash::default();
        let value = 42u64;
        value.hash(&mut hasher);
        assert_eq!(hasher.finish(), 42);
    }
}
