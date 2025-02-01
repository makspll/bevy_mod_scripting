//! A map of access claims used to safely and dynamically access the world.
use std::thread::ThreadId;

use bevy::{
    ecs::{component::ComponentId, world::unsafe_world_cell::UnsafeWorldCell},
    prelude::Resource,
};
use dashmap::{DashMap, Entry};
use parking_lot::RwLock;
use smallvec::SmallVec;

use crate::error::InteropError;

use super::{ReflectAllocationId, ReflectBase};

#[derive(Debug, Clone, PartialEq, Eq)]
/// An owner of an access claim and the code location of the claim.
pub struct ClaimOwner {
    id: ThreadId,
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

impl AccessMapKey for u64 {
    fn as_index(&self) -> u64 {
        *self
    }

    fn from_index(value: u64) -> Self {
        value
    }
}

/// Describes kinds of base value we are accessing via reflection
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
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
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct ReflectAccessId {
    pub(crate) kind: ReflectAccessKind,
    pub(crate) id: u64,
}

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
    pub fn for_component<C: bevy::ecs::component::Component>(
        cell: &UnsafeWorldCell,
    ) -> Result<Self, InteropError> {
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
        }
    }
}

impl From<ComponentId> for ReflectAccessId {
    fn from(value: ComponentId) -> Self {
        Self {
            kind: ReflectAccessKind::ComponentOrResource,
            id: value.index() as u64,
        }
    }
}

impl From<ReflectAllocationId> for ReflectAccessId {
    fn from(value: ReflectAllocationId) -> Self {
        Self {
            kind: ReflectAccessKind::Allocation,
            id: value.id(),
        }
    }
}

impl From<ReflectAccessId> for ComponentId {
    fn from(val: ReflectAccessId) -> Self {
        ComponentId::new(val.id as usize)
    }
}

impl From<ReflectAccessId> for ReflectAllocationId {
    fn from(val: ReflectAccessId) -> Self {
        ReflectAllocationId::new(val.id)
    }
}

#[derive(Debug, Default)]
/// A map of access claims
pub struct AccessMap {
    individual_accesses: DashMap<u64, AccessCount>,
    global_lock: RwLock<AccessCount>,
}

#[profiling::all_functions]
impl AccessMap {
    /// Checks if the map is locked exclusively
    pub fn is_locked_exclusively(&self) -> bool {
        let global_lock = self.global_lock.read();
        !global_lock.can_write()
    }

    /// retrieves the location of the global lock if any
    pub fn global_access_location(&self) -> Option<std::panic::Location<'static>> {
        let global_lock = self.global_lock.read();
        global_lock.as_location()
    }

    /// Tries to claim read access, will return false if somebody else is writing to the same key, or holding a global lock
    #[track_caller]
    pub fn claim_read_access<K: AccessMapKey>(&self, key: K) -> bool {
        if self.is_locked_exclusively() {
            return false;
        }

        let key = key.as_index();
        let access = self.individual_accesses.try_entry(key);
        match access.map(Entry::or_default) {
            Some(mut entry) if entry.can_read() => {
                entry.read_by.push(ClaimOwner {
                    id: std::thread::current().id(),
                    location: *std::panic::Location::caller(),
                });
                true
            }
            _ => false,
        }
    }

    #[track_caller]
    /// Tries to claim write access, will return false if somebody else is reading or writing to the same key, or holding a global lock
    pub fn claim_write_access<K: AccessMapKey>(&self, key: K) -> bool {
        if self.is_locked_exclusively() {
            return false;
        }
        let key = key.as_index();
        let access = self.individual_accesses.try_entry(key);
        match access.map(Entry::or_default) {
            Some(mut entry) if entry.can_write() => {
                entry.read_by.push(ClaimOwner {
                    id: std::thread::current().id(),
                    location: *std::panic::Location::caller(),
                });
                entry.written = true;
                true
            }
            _ => false,
        }
    }

    /// Tries to claim global access. This type of access prevents any other access from happening simulatenously
    /// Will return false if anybody else is currently accessing any part of the map
    #[track_caller]
    pub fn claim_global_access(&self) -> bool {
        let mut global_lock = self.global_lock.write();

        if !self.individual_accesses.is_empty() || !global_lock.can_write() {
            return false;
        }
        global_lock.read_by.push(ClaimOwner {
            id: std::thread::current().id(),
            location: *std::panic::Location::caller(),
        });
        global_lock.written = true;
        true
    }

    /// Releases an access
    ///
    /// # Panics
    /// if the access is released from a different thread than it was claimed from
    pub fn release_access<K: AccessMapKey>(&self, key: K) {
        let key = key.as_index();
        let access = self.individual_accesses.entry(key);
        match access {
            dashmap::mapref::entry::Entry::Occupied(mut entry) => {
                let entry_mut = entry.get_mut();
                entry_mut.written = false;
                if let Some(p) = entry_mut.read_by.pop() {
                    assert!(
                        p.id == std::thread::current().id(),
                        "Access released from wrong thread, claimed at {}",
                        p.location.display_location()
                    );
                }
                if entry_mut.readers() == 0 {
                    entry.remove();
                }
            }
            dashmap::mapref::entry::Entry::Vacant(_) => {}
        }
    }

    /// Releases a global access
    pub fn release_global_access(&self) {
        let mut global_lock = self.global_lock.write();
        global_lock.written = false;
        if let Some(p) = global_lock.read_by.pop() {
            assert!(
                p.id == std::thread::current().id(),
                "Access released from wrong thread, claimed at {}",
                p.location.display_location()
            );
        }
    }

    /// Lists all accesses
    pub fn list_accesses<K: AccessMapKey>(&self) -> Vec<(K, AccessCount)> {
        self.individual_accesses
            .iter()
            .map(|e| (K::from_index(*e.key()), e.value().clone()))
            .collect()
    }

    /// Counts the number of accesses
    pub fn count_accesses(&self) -> usize {
        self.individual_accesses.len()
    }

    /// Releases all accesses
    pub fn release_all_accesses(&self) {
        self.individual_accesses.clear();
        self.release_global_access();
    }

    /// Accesses the location of a key
    pub fn access_location<K: AccessMapKey>(
        &self,
        key: K,
    ) -> Option<std::panic::Location<'static>> {
        if key.as_index() == 0 {
            return self.global_access_location();
        }

        self.individual_accesses
            .try_get(&key.as_index())
            .try_unwrap()
            .and_then(|access| access.as_location())
    }

    /// Accesses the location of the first access
    pub fn access_first_location(&self) -> Option<std::panic::Location<'static>> {
        self.individual_accesses
            .iter()
            .find_map(|e| e.value().as_location())
    }
}

/// A trait for displaying a code location nicely
pub trait DisplayCodeLocation {
    /// Displays the location
    fn display_location(self) -> String;
}

impl DisplayCodeLocation for std::panic::Location<'_> {
    fn display_location(self) -> String {
        format!("\"{}:{}\"", self.file(), self.line())
    }
}

impl DisplayCodeLocation for Option<std::panic::Location<'_>> {
    fn display_location(self) -> String {
        self.map(|l| l.display_location())
            .unwrap_or_else(|| "\"unknown location\"".to_owned())
    }
}

#[macro_export]
/// A macro for claiming access to a value for reading
macro_rules! with_access_read {
    ($access_map:expr, $id:expr, $msg:expr, $body:block) => {{
        if !$access_map.claim_read_access($id) {
            Err($crate::error::InteropError::cannot_claim_access(
                $id,
                $access_map.access_location($id),
                $msg,
            ))
        } else {
            let result = $body;
            $access_map.release_access($id);
            Ok(result)
        }
    }};
}

#[macro_export]
/// A macro for claiming access to a value for writing
macro_rules! with_access_write {
    ($access_map:expr, $id:expr, $msg:expr, $body:block) => {
        if !$access_map.claim_write_access($id) {
            Err($crate::error::InteropError::cannot_claim_access(
                $id,
                $access_map.access_location($id),
                $msg,
            ))
        } else {
            let result = $body;
            $access_map.release_access($id);
            Ok(result)
        }
    };
}

#[macro_export]
/// A macro for claiming global access
macro_rules! with_global_access {
    ($access_map:expr, $msg:expr, $body:block) => {
        if !$access_map.claim_global_access() {
            Err($crate::error::InteropError::cannot_claim_access(
                $crate::bindings::access_map::ReflectAccessId::for_global(),
                $access_map
                    .access_location($crate::bindings::access_map::ReflectAccessId::for_global()),
                $msg,
            ))
        } else {
            #[allow(clippy::redundant_closure_call)]
            let result = (|| $body)();
            $access_map.release_global_access();
            Ok(result)
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_list_accesses() {
        let access_map = AccessMap::default();

        access_map.claim_read_access(0);
        access_map.claim_write_access(1);

        let accesses = access_map.list_accesses::<u64>();

        assert_eq!(accesses.len(), 2);
        let access_0 = accesses.iter().find(|(k, _)| *k == 0).unwrap();
        let access_1 = accesses.iter().find(|(k, _)| *k == 1).unwrap();

        assert_eq!(access_0.1.readers(), 1);
        assert_eq!(access_1.1.readers(), 1);

        assert!(!access_0.1.written);
        assert!(access_1.1.written);
    }

    #[test]
    fn test_read_access_blocks_write() {
        let access_map = AccessMap::default();

        assert!(access_map.claim_read_access(0));
        assert!(!access_map.claim_write_access(0));
        access_map.release_access(0);
        assert!(access_map.claim_write_access(0));
    }

    #[test]
    fn test_write_access_blocks_read() {
        let access_map = AccessMap::default();

        assert!(access_map.claim_write_access(0));
        assert!(!access_map.claim_read_access(0));
        access_map.release_access(0);
        assert!(access_map.claim_read_access(0));
    }

    #[test]
    fn test_global_access_blocks_all() {
        let access_map = AccessMap::default();

        assert!(access_map.claim_global_access());
        assert!(!access_map.claim_read_access(0));
        assert!(!access_map.claim_write_access(0));
        access_map.release_global_access();
        assert!(access_map.claim_write_access(0));
        access_map.release_access(0);
        assert!(access_map.claim_read_access(0));
    }

    #[test]
    fn any_access_blocks_global() {
        let access_map = AccessMap::default();

        assert!(access_map.claim_read_access(0));
        assert!(!access_map.claim_global_access());
        access_map.release_access(0);

        assert!(access_map.claim_write_access(0));
        assert!(!access_map.claim_global_access());
    }

    #[test]
    #[should_panic]
    fn releasing_read_access_from_wrong_thread_panics() {
        let access_map = AccessMap::default();

        access_map.claim_read_access(0);
        std::thread::spawn(move || {
            access_map.release_access(0);
        })
        .join()
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn releasing_write_access_from_wrong_thread_panics() {
        let access_map = AccessMap::default();

        access_map.claim_write_access(0);
        std::thread::spawn(move || {
            access_map.release_access(0);
        })
        .join()
        .unwrap();
    }

    #[test]
    fn test_as_and_from_index_for_access_id_non_overlapping() {
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
}
