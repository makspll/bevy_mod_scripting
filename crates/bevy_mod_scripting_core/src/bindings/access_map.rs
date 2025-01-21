use std::thread::ThreadId;

use bevy::{
    ecs::{component::ComponentId, world::unsafe_world_cell::UnsafeWorldCell},
    prelude::Resource,
};
use dashmap::{DashMap, Entry};
use parking_lot::RwLock;
use smallvec::SmallVec;

use crate::error::InteropError;

use super::{ReflectAllocation2, ReflectBase};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimOwner {
    id: ThreadId,
    location: std::panic::Location<'static>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    ComponentOrResource,
    Allocation,
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
        // project two linear non-negative ranges to a single linear non-negative range, offset by 1 to avoid 0
        // y1 = 2x - 0 + 1
        // y2 = 2x - 1 + 1
        match self.kind {
            ReflectAccessKind::ComponentOrResource => (self.id * 2) + 1,
            ReflectAccessKind::Allocation => (self.id * 2) + 1,
            ReflectAccessKind::Global => 0,
        }
    }

    fn from_index(value: u64) -> Self {
        // retrieve the kind of range based on if the value is odd or even
        // y1 if even, y2 if odd
        // to retrieve value of x:
        // x1 = (y / 2) - 1
        // x2 = ((y - 1) / 2) - 1

        let (kind, id) = if value == 0 {
            (ReflectAccessKind::Global, 0)
        } else if value % 2 == 0 {
            (ReflectAccessKind::ComponentOrResource, (value / 2) - 1)
        } else {
            (ReflectAccessKind::Allocation, ((value - 1) / 2) - 1)
        };
        Self { kind, id }
    }
}

impl ReflectAccessId {
    pub fn for_global() -> Self {
        Self {
            kind: ReflectAccessKind::Global,
            id: 0,
        }
    }

    pub fn for_resource<R: Resource>(cell: &UnsafeWorldCell) -> Result<Self, InteropError> {
        let resource_id = cell.components().resource_id::<R>().ok_or_else(|| {
            InteropError::unregistered_component_or_resource_type(std::any::type_name::<R>())
        })?;

        Ok(Self {
            kind: ReflectAccessKind::ComponentOrResource,
            id: resource_id.index() as u64,
        })
    }

    pub fn for_component<C: bevy::ecs::component::Component>(
        cell: &UnsafeWorldCell,
    ) -> Result<Self, InteropError> {
        let component_id = cell.components().component_id::<C>().ok_or_else(|| {
            InteropError::unregistered_component_or_resource_type(std::any::type_name::<C>())
        })?;

        Ok(Self::for_component_id(component_id))
    }

    pub fn for_allocation(allocation: &ReflectAllocation2) -> Self {
        debug_assert!(
            usize::MAX as u128 <= u64::MAX as u128,
            "usize assumption broken"
        );
        Self {
            kind: ReflectAccessKind::Allocation,
            id: allocation.unique_id() as u64,
        }
    }

    pub fn for_component_id(id: ComponentId) -> Self {
        Self {
            kind: ReflectAccessKind::ComponentOrResource,
            id: id.index() as u64,
        }
    }

    pub fn for_reference(base: ReflectBase) -> Self {
        match base {
            ReflectBase::Resource(id) => Self::for_component_id(id),
            ReflectBase::Component(_, id) => Self::for_component_id(id),
            ReflectBase::Owned(id) => Self::for_allocation(&id),
        }
    }
}

impl From<ComponentId> for ReflectAccessId {
    fn from(value: ComponentId) -> Self {
        ReflectAccessId::for_component_id(value)
    }
}

impl From<&ReflectAllocation2> for ReflectAccessId {
    fn from(value: &ReflectAllocation2) -> Self {
        ReflectAccessId::for_allocation(value)
    }
}

impl From<ReflectAccessId> for ComponentId {
    fn from(val: ReflectAccessId) -> Self {
        ComponentId::new(val.id as usize)
    }
}

#[derive(Debug, Default)]
pub struct AccessMap {
    individual_accesses: DashMap<u64, AccessCount>,
    global_lock: RwLock<AccessCount>,
}

impl AccessMap {
    pub fn is_locked_exclusively(&self) -> bool {
        let global_lock = self.global_lock.read();
        !global_lock.can_write()
    }

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

    pub fn list_accesses<K: AccessMapKey>(&self) -> Vec<(K, AccessCount)> {
        self.individual_accesses
            .iter()
            .map(|e| (K::from_index(*e.key()), e.value().clone()))
            .collect()
    }

    pub fn count_accesses(&self) -> usize {
        self.individual_accesses.len()
    }

    pub fn release_all_accesses(&self) {
        self.individual_accesses.clear();
        self.release_global_access();
    }

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

    pub fn access_first_location(&self) -> Option<std::panic::Location<'static>> {
        self.individual_accesses
            .iter()
            .find_map(|e| e.value().as_location())
    }
}

pub trait DisplayCodeLocation {
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
}
