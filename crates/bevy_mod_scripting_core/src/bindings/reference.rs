//! # Motivation
//!
//! Traits and structs needed to support the creation of bindings for scripting languages.
//! reflection gives us access to `dyn PartialReflect` objects via their type name,
//! Scripting languages only really support `Clone` objects so if we want to support references,
//! we need wrapper types which have owned and ref variants.
use lockable::LockableHashMap;

use std::{
    any::TypeId,
    cell::UnsafeCell,
    error::Error,
    fmt::Debug,
    marker::PhantomData,
    ops::Index,
    sync::{Arc, Weak},
    time::Duration,
};

use bevy::{
    ecs::{
        change_detection::MutUntyped,
        component::{Component, ComponentId},
        entity::Entity,
        reflect::AppTypeRegistry,
        system::Resource,
        world::{unsafe_world_cell::UnsafeWorldCell, Mut, World},
    },
    ptr::Ptr,
    reflect::{
        Access, ParsedPath, PartialReflect, Reflect, ReflectFromPtr, ReflectPath, ReflectPathError, TypeInfo, TypeRegistry
    },
};
use smallvec::SmallVec;

use crate::{
    bindings::{ReflectAllocation, ReflectAllocationId},
    prelude::{ReflectAllocator, ScriptError, ScriptResult},
};

use super::{
    proxy::{Proxy, Unproxy},
    ReflectAccessId, ReflectAccessKind, WorldAccessGuard, WorldAccessWrite, DEFAULT_INTERVAL,
    DEFAULT_TIMEOUT,
};

/// An accessor to a `dyn PartialReflect` struct, stores a base ID of the type and a reflection path
/// safe to build but to reflect on the value inside you need to ensure aliasing rules are upheld
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReflectReference {
    pub base: ReflectBaseType,
    // TODO: experiment with Fixed capacity vec, boxed array etc, compromise between heap allocation and runtime cost
    // needs benchmarks first though
    /// The path from the top level type to the actual value we want to access
    pub reflect_path: Vec<ReflectionPathElem>,
}

// just a dummy standin for unregistered types
struct UnregisteredType;

impl ReflectReference {

    pub fn new_allocated<T: PartialReflect>(
        value: T,
        allocator: &mut ReflectAllocator,
    ) -> ReflectReference {
        let (id, _) = allocator.allocate(value);
        ReflectReference {
            base: ReflectBaseType {
                type_id: TypeId::of::<T>(),
                base_id: ReflectBase::Owned(id),
            },
            reflect_path: Vec::default(),
        }
    }

    /// Indexes into the reflect path inside this reference.
    /// You can use [`Self::reflect`] and [`Self::reflect_mut`] to get the actual value.
    pub fn index_path<T: Into<ReflectionPathElem>>(&mut self, index: T) {
        self.reflect_path.push(index.into());
    }

    /// A form of [`Self::reflect`] which does the access checks for you.
    /// Panics if it waits for access too long to prevent deadlocks.
    pub fn with_reflect<O, F: FnOnce(&dyn PartialReflect) -> O>(
        &self,
        world: &WorldAccessGuard,
        type_registry: &TypeRegistry,
        allocator: Option<&ReflectAllocator>,
        f: F,
    ) -> O {
        let access = world
            .get_access_timeout(
                self.base.base_id.get_reflect_access_id(),
                DEFAULT_TIMEOUT,
                DEFAULT_INTERVAL,
            )
            .unwrap_or_else(|| panic!("Timeout when waiting for access for: `{:?}`", self));

        let reflect = self
            .reflect(
                world.as_unsafe_world_cell(),
                &access,
                type_registry,
                allocator,
            )
            .unwrap();
        let o = f(reflect);
        world.release_access(access);
        o
    }

    pub fn with_reflect_mut<O, F: FnOnce(&mut dyn PartialReflect) -> O>(
        &self,
        world: &WorldAccessGuard,
        type_registry: &TypeRegistry,
        allocator: Option<&ReflectAllocator>,
        f: F,
    ) -> O {
        let mut access = world
            .get_access_timeout(
                self.base.base_id.get_reflect_access_id(),
                DEFAULT_TIMEOUT,
                DEFAULT_INTERVAL,
            )
            .unwrap_or_else(|| panic!("Timeout when waiting for access for: `{:?}`", self));

        let reflect = self
            .reflect_mut(
                world.as_unsafe_world_cell(),
                &mut access,
                type_registry,
                allocator,
            )
            .unwrap();
        let o = f(reflect);
        world.release_access(access);
        o
    }

    /// Returns `Ok(())` if the given access is sufficient to read the value or an appropriate error otherwise
    pub fn expect_read_access<'w>(
        &self,
        access: &WorldAccessWrite<'w>,
        type_registry: &TypeRegistry,
        allocator: Option<&ReflectAllocator>,
        world: UnsafeWorldCell<'w>,
    ) -> ScriptResult<()> {
        if !access.can_read(self.base.base_id.get_reflect_access_id()) {
            Err(ScriptError::new_reflection_error(format!(
                "Invalid access when trying to read: `{}`, instead got access to `{}`",
                self.base.display_with_type_name(type_registry),
                access.to_enriched_str(type_registry, allocator, world)
            )))
        } else {
            Ok(())
        }
    }

    /// Returns `Ok(())` if the given access is sufficient to write to the value or an appropriate error otherwise
    /// Note that this is not sufficient for write access, you also need to ensure the [`WorldAccessWrite`] won't be used to access the same value mutably elsewhere,
    /// if you have a `&mut WorldAccessWrite` you can guarantee this statically. This function just checks that the access itself is for the right base with write access
    pub fn expect_write_access<'w>(
        &self,
        access: &WorldAccessWrite<'w>,
        type_registry: &TypeRegistry,
        allocator: Option<&ReflectAllocator>,
        world: UnsafeWorldCell<'w>,
    ) -> ScriptResult<()> {
        if !access.can_read(self.base.base_id.get_reflect_access_id()) {
            Err(ScriptError::new_reflection_error(format!(
                "Invalid access when trying to write: `{}`, instead got access to `{}`",
                self.base.display_with_type_name(type_registry),
                access.to_enriched_str(type_registry, allocator, world)
            )))
        } else {
            Ok(())
        }
    }

    /// Retrieves a reference to the underlying `dyn PartialReflect` type valid for the 'w lifetime of the world cell.
    /// If the underlying componentId is not the same as the one we have access to, an error is returned.
    pub fn reflect<'w, 'c>(
        &self,
        world: UnsafeWorldCell<'w>,
        access: &'c WorldAccessWrite<'w>,
        type_registry: &TypeRegistry,
        allocator: Option<&'c ReflectAllocator>,
    ) -> ScriptResult<&'c dyn PartialReflect> {
        self.expect_read_access(access, type_registry, allocator, world)?;
        // Safety: since we have read access to the underlying componentId we can safely access the component
        // and we can return a reference tied to its lifetime, which will prevent invalid aliasing
        return unsafe { self.reflect_unsafe(world, type_registry, allocator) };
    }

    /// Retrieves a reference to the underlying `dyn PartialReflect` type valid for the 'w lifetime of the world cell.
    /// If the underlying componentId is not the same as the one we have access to, an error is returned.
    ///
    /// If we are accessing a component or resource, it's marked as changed
    pub fn reflect_mut<'w, 'c>(
        &self,
        world: UnsafeWorldCell<'w>,
        access: &'c mut WorldAccessWrite<'w>,
        type_registry: &TypeRegistry,
        allocator: Option<&'c ReflectAllocator>,
    ) -> ScriptResult<&'c mut dyn PartialReflect> {
        self.expect_write_access(access, type_registry, allocator, world)?;
        // Safety: since we have write access to the underlying reflect access id we can safely access the component
        // and we can return a reference tied to its lifetime, which will prevent invalid aliasing
        return unsafe { self.reflect_mut_unsafe(world, type_registry, allocator) };
    }

    /// Retrieves a reference to the underlying `dyn PartialReflect` type valid for the 'w lifetime of the world cell
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing mut references to the same value exist at all at the same time
    pub unsafe fn reflect_unsafe<'w>(
        &self,
        world: UnsafeWorldCell<'w>,
        type_registry: &TypeRegistry,
        allocator: Option<&'w ReflectAllocator>,
    ) -> ScriptResult<&'w dyn PartialReflect> {
        if let ReflectBase::Owned(id) = &self.base.base_id {
            let allocator =
                allocator.ok_or_else(|| ScriptError::new_reflection_error("Allocator missing"))?;
            let arc = allocator
                .get(*id)
                .ok_or_else(|| ScriptError::new_reflection_error("Missing allocation"))?;

            // safety: caller promises it's fine :)
            return self.walk_path(unsafe { &*arc.get_ptr() });
        };
        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = type_registry
            .get_type_data(self.base.type_id)
            .expect("FromPtr is not registered for this type, cannot retrieve reflect reference");

        let ptr = self
            .base
            .base_id
            .clone()
            .into_ptr(world)
            .ok_or_else(|| 
                ScriptError::new_reflection_error(
                    format!("Base reference is invalid, is the component/resource initialized? does the entity exist?. When accessing: `{}`", self.base.display_with_type_name(type_registry))))?;

        // (Ptr) Safety: we use the same type_id to both
        // 1) retrieve the ptr
        // 2) retrieve the ReflectFromPtr type data
        // (UnsafeWorldCell) Safety:
        // we already have access to &world so no &mut world exists
        debug_assert_eq!(
            from_ptr_data.type_id(),
            self.base.type_id,
            "Invariant violated"
        );
        let base = unsafe { from_ptr_data.as_reflect(ptr) };
        self.walk_path(base.as_partial_reflect())
    }

    /// Retrieves mutable reference to the underlying `dyn PartialReflect` type valid for the 'w lifetime of the world cell
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no other references to the same value exist at all at the same time (even if you have the correct access)
    pub unsafe fn reflect_mut_unsafe<'w>(
        &self,
        world: UnsafeWorldCell<'w>,
        type_registry: &TypeRegistry,
        allocator: Option<&'w ReflectAllocator>,
    ) -> ScriptResult<&'w mut dyn PartialReflect> {
        if let ReflectBase::Owned(id) = &self.base.base_id {
            let allocator = allocator.ok_or_else(|| ScriptError::new_reflection_error("Allocator missing"))?;

            let arc = allocator
                .get_mut(*id)
                .ok_or_else(|| ScriptError::new_reflection_error("Missing allocation"))?;

            // Safety: caller promises this is fine :)
            return self.walk_path_mut(unsafe { &mut *arc.get_ptr() });
        };

        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = type_registry
            .get_type_data(self.base.type_id)
            .expect("FromPtr is not registered for this type, cannot retrieve reflect reference");

        let ptr = self
         .base
         .base_id
         .clone()
         .into_ptr_mut(world)
         .ok_or_else(|| 
            ScriptError::new_reflection_error(
                format!("Base reference is invalid, is the component/resource initialized? does the entity exist?. When accessing: `{}`", self.base.display_with_type_name(type_registry))))?
         .into_inner();

        // (Ptr) Safety: we use the same type_id to both
        // 1) retrieve the ptr
        // 2) retrieve the ReflectFromPtr type data
        // (UnsafeWorldCell) Safety:
        // we already have access to &world so no &mut world exists
        debug_assert_eq!(
            from_ptr_data.type_id(),
            self.base.type_id,
            "Invariant violated"
        );
        let base = unsafe { from_ptr_data.as_reflect_mut(ptr) };
        self.walk_path_mut(base.as_partial_reflect_mut())
    }

    fn walk_path<'a>(&self, root: &'a dyn PartialReflect) -> ScriptResult<&'a dyn PartialReflect> {
        let mut current = root;
        for elem in self.reflect_path.iter() {
            current = elem
                .reflect_element(current)
                .map_err(|e| ScriptError::new_reflection_error(e.to_string()))?;
        }
        Ok(current)
    }

    fn walk_path_mut<'a>(&self, root: &'a mut dyn PartialReflect) -> ScriptResult<&'a mut dyn PartialReflect> {
        let mut current = root;
        for elem in self.reflect_path.iter() {
            current = elem
                .reflect_element_mut(current)
                .map_err(|e| ScriptError::new_reflection_error(e.to_string()))?;
        }
        Ok(current)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReflectBaseType {
    pub type_id: TypeId,
    pub base_id: ReflectBase,
}

impl ReflectBaseType {
    pub fn type_name(type_id: TypeId, type_registry: &TypeRegistry) -> &'static str {
        type_registry
            .get_type_info(type_id)
            .map(TypeInfo::type_path)
            .unwrap_or("<Unregistered TypeId>")
    }

    pub fn display_with_type_name(&self, type_registry: &TypeRegistry) -> String {
        format!(
            "ReflectBase({}, {:?})",
            Self::type_name(self.type_id, type_registry),
            self.base_id
        )
    }
}

/// The Id of the kind of reflection base being pointed to
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReflectBase {
    Component(Entity, ComponentId),
    Resource(ComponentId),
    Owned(ReflectAllocationId),
}

impl ReflectBase {
    /// Retrieves the pointer to the underlying `dyn PartialReflect` object valid for the 'w lifteime of the world cell
    ///
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing mutable references to the same value exist at the same time
    pub unsafe fn into_ptr(self, world: UnsafeWorldCell<'_>) -> Option<Ptr<'_>> {
        match self {
            ReflectBase::Component(entity, component_id) => {
                // Safety: the caller ensures invariants hold
                world.get_entity(entity)?.get_by_id(component_id)
            }
            ReflectBase::Resource(component_id) => {
                // Safety: the caller ensures invariants hold
                world.get_resource_by_id(component_id)
            }
            _ => None,
        }
    }

    /// Retrieves the pointer to the underlying `dyn PartialReflect` object valid for the 'w lifteime of the world cell
    ///
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing references to the same value exist at all at the same time
    pub unsafe fn into_ptr_mut(self, world: UnsafeWorldCell<'_>) -> Option<MutUntyped<'_>> {
        match self {
            ReflectBase::Component(entity, component_id) => {
                // Safety: the caller ensures invariants hold
                world.get_entity(entity)?.get_mut_by_id(component_id)
            }
            ReflectBase::Resource(component_id) => {
                // Safety: the caller ensures invariants hold
                world.get_resource_mut_by_id(component_id)
            }
            _ => None,
        }
    }

    pub fn get_reflect_access_id(&self) -> ReflectAccessId {
        match self {
            ReflectBase::Component(_, cid) | ReflectBase::Resource(cid) => (*cid).into(),
            ReflectBase::Owned(id) => (*id).into(),
        }
    }
}

/// An element in the reflection path, the base reference included
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ReflectionPathElem {
    /// A standard reflection path, i.e. `.field_name[vec_index]`, pre-parsed since we construct once potentially use many times
    Reflection(ParsedPath),
    /// a deferred reflection
    DeferredReflection(DeferredReflection),
}

impl ReflectionPathElem {
    pub fn new_reflection<I: Into<ParsedPath>>(path: I) -> Self {
        Self::Reflection(path.into())
    }

    pub fn new_deferred<I: Into<DeferredReflection>>(defref: I) -> Self {
        Self::DeferredReflection(defref.into())
    }
}

impl<A: 'static, B: 'static> From<(A, B)> for DeferredReflection
where
    A: Fn(&dyn PartialReflect) -> Result<&dyn PartialReflect, ReflectPathError<'static>> + Send + Sync,
    B: Fn(&mut dyn PartialReflect) -> Result<&mut dyn PartialReflect, ReflectPathError<'static>> + Send + Sync,
{
    fn from((get, get_mut): (A, B)) -> Self {
        Self {
            get: Arc::new(get),
            get_mut: Arc::new(get_mut),
        }
    }
}

impl<T: Into<DeferredReflection>> From<T> for ReflectionPathElem {
    fn from(value: T) -> Self {
        Self::DeferredReflection(value.into())
    }
}

impl From<ParsedPath> for ReflectionPathElem {
    fn from(value: ParsedPath) -> Self {
        Self::Reflection(value)
    }
}

impl<'a> ReflectPath<'a> for &'a ReflectionPathElem {
    fn reflect_element<'r>(
        self,
        root: &'r dyn PartialReflect,
    ) -> Result<&'r dyn PartialReflect, ReflectPathError<'a>> {
        match self {
            ReflectionPathElem::Reflection(path) => path.reflect_element(root),
            ReflectionPathElem::DeferredReflection(f) => (f.get)(root),
        }
    }

    fn reflect_element_mut<'r>(
        self,
        root: &'r mut dyn PartialReflect,
    ) -> Result<&'r mut dyn PartialReflect, ReflectPathError<'a>> {
        match self {
            ReflectionPathElem::Reflection(path) => path.reflect_element_mut(root),
            ReflectionPathElem::DeferredReflection(defref) => (defref.get_mut)(root),
        }
    }
}

/// A ReflectPath which can perform arbitrary operations on the root object to produce a sub-reference
#[derive(Clone)]
pub struct DeferredReflection {
    pub get:
        Arc<dyn Fn(&dyn PartialReflect) -> Result<&dyn PartialReflect, ReflectPathError<'static>> + Send + Sync>,
    pub get_mut: Arc<
        dyn Fn(&mut dyn PartialReflect) -> Result<&mut dyn PartialReflect, ReflectPathError<'static>>
            + Send
            + Sync,
    >,
}

impl Debug for DeferredReflection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DeferredReflection")
    }
}

impl PartialEq for DeferredReflection {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.get, &other.get) && Arc::ptr_eq(&self.get_mut, &other.get_mut)
    }
}

impl Eq for DeferredReflection {}
