use std::sync::atomic::{AtomicBool, AtomicUsize};

use bevy::{
    ecs::{component::ComponentId, world::unsafe_world_cell::UnsafeWorldCell},
    prelude::Resource,
};
use dashmap::{try_result::TryResult, DashMap, Entry, Map};

use super::{ReflectAllocationId, ReflectBase};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessCount {
    count: usize,
    /// set if somebody is writing
    written_by: Option<std::panic::Location<'static>>,
}

impl Default for AccessCount {
    fn default() -> Self {
        Self::new()
    }
}

impl AccessCount {
    fn new() -> Self {
        Self {
            count: 0,
            written_by: None,
        }
    }

    fn can_read(&self) -> bool {
        self.written_by.is_none()
    }

    fn can_write(&self) -> bool {
        self.count == 0 && self.written_by.is_none()
    }

    fn as_location(&self) -> Option<std::panic::Location<'static>> {
        self.written_by
    }

    fn readers(&self) -> usize {
        self.count
    }
}

pub trait AccessMapKey {
    fn as_usize(&self) -> usize;
    fn from_usize(value: usize) -> Self;
}

impl AccessMapKey for usize {
    fn as_usize(&self) -> usize {
        *self
    }

    fn from_usize(value: usize) -> Self {
        value
    }
}

/// Describes kinds of base value we are accessing via reflection
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum ReflectAccessKind {
    ComponentOrResource,
    Allocation,
}

/// Describes the id pointing to the base value we are accessing via reflection, for components and resources this is the ComponentId
/// for script owned values this is an allocationId, this is used to ensure we have permission to access the value.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct ReflectAccessId {
    kind: ReflectAccessKind,
    id: usize,
}

impl AccessMapKey for ReflectAccessId {
    fn as_usize(&self) -> usize {
        // project two linear non-negative ranges to a single linear non-negative range
        // y1 = 2x - 0
        // y2 = 2x - 1
        match self.kind {
            ReflectAccessKind::ComponentOrResource => self.id * 2,
            ReflectAccessKind::Allocation => self.id * 2 + 1,
        }
    }

    fn from_usize(value: usize) -> Self {
        // retrieve the kind of range based on if the value is odd or even
        // y1 if even, y2 if odd
        // to retrieve value of x:
        // x1 = y / 2
        // x2 = (y - 1) / 2
        let (kind, id) = if value % 2 == 0 {
            (ReflectAccessKind::ComponentOrResource, value / 2)
        } else {
            (ReflectAccessKind::Allocation, (value - 1) / 2)
        };
        Self { kind, id }
    }
}

impl ReflectAccessId {
    pub fn for_resource<R: Resource>(cell: &UnsafeWorldCell) -> Option<Self> {
        Some(Self {
            kind: ReflectAccessKind::ComponentOrResource,
            id: cell.components().resource_id::<R>()?.index(),
        })
    }

    pub fn for_component<C: bevy::ecs::component::Component>(
        cell: &UnsafeWorldCell,
    ) -> Option<Self> {
        let component_id = cell.components().component_id::<C>()?;

        Some(Self::for_component_id(component_id))
    }

    pub fn for_allocation(id: ReflectAllocationId) -> Self {
        Self {
            kind: ReflectAccessKind::Allocation,
            id: id.id(),
        }
    }

    pub fn for_component_id(id: ComponentId) -> Self {
        Self {
            kind: ReflectAccessKind::ComponentOrResource,
            id: id.index(),
        }
    }

    pub fn for_reference(base: ReflectBase) -> Option<Self> {
        match base {
            ReflectBase::Resource(id) => Some(Self::for_component_id(id)),
            ReflectBase::Component(_, id) => Some(Self::for_component_id(id)),
            ReflectBase::Owned(id) => Some(Self::for_allocation(id)),
            ReflectBase::World => None,
        }
    }
}

impl From<ComponentId> for ReflectAccessId {
    fn from(value: ComponentId) -> Self {
        Self {
            kind: ReflectAccessKind::ComponentOrResource,
            id: value.index(),
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
        ComponentId::new(val.id)
    }
}

#[derive(Debug, Default)]
pub struct AccessMap {
    individual_accesses: DashMap<usize, AccessCount>,
    global_lock: AtomicBool,
}

impl AccessMap {
    /// Tries to claim read access, will return false if somebody else is writing to the same key, or holding a global lock
    pub fn claim_read_access<K: AccessMapKey>(&self, key: K) -> bool {
        if self.global_lock.load(std::sync::atomic::Ordering::Relaxed) {
            return false;
        }
        let key = key.as_usize();
        let access = self.individual_accesses.try_entry(key);
        match access.map(Entry::or_default) {
            Some(mut entry) if entry.can_read() => {
                entry.count += 1;
                true
            }
            _ => false,
        }
    }

    #[track_caller]
    /// Tries to claim write access, will return false if somebody else is reading or writing to the same key, or holding a global lock
    pub fn claim_write_access<K: AccessMapKey>(&self, key: K) -> bool {
        if self.global_lock.load(std::sync::atomic::Ordering::Relaxed) {
            return false;
        }
        let key = key.as_usize();
        let access = self.individual_accesses.try_entry(key);
        match access.map(Entry::or_default) {
            Some(mut entry) if entry.can_write() => {
                entry.count += 1;
                entry.written_by = Some(*std::panic::Location::caller());
                true
            }
            _ => false,
        }
    }

    /// Tries to claim global access. This type of access prevents any other access from happening simulatenously
    /// Will return false if anybody else is currently accessing any part of the map
    pub fn claim_global_access(&self) -> bool {
        self.individual_accesses.len() == 0
            && self
                .global_lock
                .compare_exchange(
                    false,
                    true,
                    std::sync::atomic::Ordering::Relaxed,
                    std::sync::atomic::Ordering::Relaxed,
                )
                .is_ok()
    }

    pub fn release_access<K: AccessMapKey>(&self, key: K) {
        let key = key.as_usize();
        let access = self.individual_accesses.entry(key);
        match access {
            dashmap::mapref::entry::Entry::Occupied(mut entry) => {
                let entry_mut = entry.get_mut();
                if entry_mut.written_by.is_some() {
                    entry_mut.written_by = None;
                }
                entry_mut.count -= 1;
                if entry_mut.count == 0 {
                    entry.remove();
                }
            }
            dashmap::mapref::entry::Entry::Vacant(_) => {}
        }
    }

    /// Releases a global access
    pub fn release_global_access(&self) {
        self.global_lock
            .store(false, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn list_accesses<K: AccessMapKey>(&self) -> Vec<(K, AccessCount)> {
        self.individual_accesses
            .iter()
            .map(|e| (K::from_usize(*e.key()), e.value().clone()))
            .collect()
    }

    pub fn access_location<K: AccessMapKey>(
        &self,
        key: K,
    ) -> Option<std::panic::Location<'static>> {
        self.individual_accesses
            .try_get(&key.as_usize())
            .try_unwrap()
            .map(|access| access.as_location())
            .flatten()
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
            panic!(
                "{}. Aliasing access is held somewhere else: {}",
                $msg,
                $crate::bindings::access_map::DisplayCodeLocation::display_location(
                    $access_map.access_location($id)
                )
            );
        } else {
            let result = $body;
            $access_map.release_access($id);
            result
        }
    }};
}

#[macro_export]
macro_rules! with_access_write {
    ($access_map:expr, $id:expr, $msg:expr, $body:block) => {
        if !$access_map.claim_write_access($id) {
            panic!(
                "{}. Aliasing access is held somewhere else: {}",
                $msg,
                $crate::bindings::access_map::DisplayCodeLocation::display_location(
                    $access_map.access_location($id)
                )
            );
        } else {
            let result = $body;
            $access_map.release_access($id);
            result
        }
    };
}

#[macro_export]
macro_rules! with_global_access {
    ($access_map:expr, $msg:expr, $body:block) => {
        if !$access_map.claim_global_access() {
            panic!(
                "{}. Another access is held somewhere else preventing locking the world",
                $msg
            );
        } else {
            let result = (|| $body)();
            $access_map.release_global_access();
            result
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

        let accesses = access_map.list_accesses::<usize>();

        assert_eq!(accesses.len(), 2);
        let access_0 = accesses.iter().find(|(k, _)| *k == 0).unwrap();
        let access_1 = accesses.iter().find(|(k, _)| *k == 1).unwrap();

        assert_eq!(access_0.1.readers(), 1);
        assert_eq!(access_1.1.readers(), 1);

        assert_eq!(access_0.1.written_by, None);
        assert!(access_1.1.written_by.is_some());
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
}
