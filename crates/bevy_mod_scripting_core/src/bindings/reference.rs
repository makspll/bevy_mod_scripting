//! # Motivation
//!
//! Traits and structs needed to support the creation of bindings for scripting languages.
//! reflection gives us access to `dyn PartialReflect` objects via their type name,
//! Scripting languages only really support `Clone` objects so if we want to support references,
//! we need wrapper types which have owned and ref variants.
use itertools::Itertools;
use lockable::LockableHashMap;

pub use itertools::Either;

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
        Access, AccessError, DynamicEnum, DynamicTuple, ParsedPath, PartialReflect, Reflect, ReflectFromPtr, ReflectMut, ReflectPath, ReflectPathError, ReflectRef, TypeData, TypeInfo, TypeRegistry
    },
};
use smallvec::SmallVec;

use crate::{
    bindings::{ReflectAllocation, ReflectAllocationId},
    prelude::{ReflectAllocator, ScriptError, ScriptResult}, reflection_extensions::{PartialReflectExt, TypeIdExtensions},
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

/// Specifies where we should source the type id from when reflecting a ReflectReference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeIdSource {
    /// Use the type id the reference points to after walking the path
    Tail,
    /// Given the Tail referene is a container type, use the type id of the elements in the container
    Element,
    /// Givent the Tail reference is a container type, use the type id of the keys of the container
    Key,
}


impl ReflectReference {

    /// Creates a new infinite iterator. This iterator will keep returning the next element reference forever.
    pub fn into_iter_infinite(self) -> ReflectRefIter {
        ReflectRefIter::new_indexed(self)
    }

    /// If this is a reference to something with a length accessible via reflection, returns that length.
    pub fn len(&self, world: &WorldAccessGuard) -> ScriptResult<Option<usize>> {
        self
            .with_reflect(world, |r, _, _| {
                match r.reflect_ref() {
                    bevy::reflect::ReflectRef::Struct(s) => Some(s.field_len()),
                    bevy::reflect::ReflectRef::TupleStruct(ts) => Some(ts.field_len()),
                    bevy::reflect::ReflectRef::Tuple(t) => Some(t.field_len()),
                    bevy::reflect::ReflectRef::List(l) => Some(l.len()),
                    bevy::reflect::ReflectRef::Array(a) => Some(a.len()),
                    bevy::reflect::ReflectRef::Map(m) => Some(m.len( )),
                    bevy::reflect::ReflectRef::Set(s) => Some(s.len()),
                    bevy::reflect::ReflectRef::Enum(e) => Some(e.field_len()),
                    bevy::reflect::ReflectRef::Opaque(_) => None,
                }
            })
    }

    pub fn new_allocated<T: PartialReflect>(
        value: T,
        allocator: &mut ReflectAllocator,
    ) -> ReflectReference {
        let type_id = value.get_represented_type_info().map(|i| i.type_id()).unwrap_or_else(|| panic!("Type '{}' has no represented type information to allocate with.", std::any::type_name::<T>()));
        let (id, _) = allocator.allocate(value);
        ReflectReference {
            base: ReflectBaseType {
                type_id,
                base_id: ReflectBase::Owned(id),
            },
            reflect_path: Vec::default(),
        }
    }

    pub fn new_allocated_boxed(
        value: Box<dyn PartialReflect>,
        allocator: &mut ReflectAllocator,
    ) -> ReflectReference {
        let type_id = value.get_represented_type_info().map(|i| i.type_id()).unwrap_or_else(|| panic!("Type '{}' has no represented type information to allocate with.", std::any::type_name_of_val(value.as_ref())));
        let (id, _) = allocator.allocate_boxed(value);
        ReflectReference {
            base: ReflectBaseType {
                type_id,
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
    #[track_caller]
    pub fn with_reflect<O, F: FnOnce(&dyn PartialReflect, &TypeRegistry , &mut ReflectAllocator) -> O>(
        &self,
        world: &WorldAccessGuard,
        f: F,
    ) -> ScriptResult<O> {
        world.with_allocator_and_type_registry(|world, type_registry, mut allocator| {
            let type_registry = type_registry.write();
            self.with_reflect_only(world, &type_registry, &mut allocator, f)
        })
    }

    pub fn with_reflect_only<O, F: FnOnce(&dyn PartialReflect, &TypeRegistry , &mut ReflectAllocator) -> O>(
        &self,
        world: &WorldAccessGuard,
        type_registry: &TypeRegistry,
        allocator: &mut ReflectAllocator,
        f: F,
    ) -> ScriptResult<O> {
        let access = world
            .get_access_timeout(
                self.base.base_id.get_reflect_access_id(),
                DEFAULT_TIMEOUT,
                DEFAULT_INTERVAL,
            );

        let reflect = self
            .reflect(
                world.as_unsafe_world_cell(),
                &access,
                type_registry,
                Some(allocator),
            ).map(|r| f(r, type_registry, allocator));

        world.release_access(access);
        reflect
    }

    #[track_caller]
    pub fn with_reflect_mut<O, F: FnOnce(&mut dyn PartialReflect, &TypeRegistry, &mut ReflectAllocator) -> O>(
        &self,
        world: &WorldAccessGuard,
        f: F,
    ) -> ScriptResult<O> {
        world.with_allocator_and_type_registry(|_, type_registry, mut allocator| {
            let type_registry = type_registry.read();
            self.with_reflect_mut_only(world, &type_registry,  &mut allocator, f)
        })
    }

    pub fn with_reflect_mut_only<O, F: FnOnce(&mut dyn PartialReflect, &TypeRegistry, &mut ReflectAllocator) -> O>(
        &self,
        world: &WorldAccessGuard,
        type_registry: &TypeRegistry,
        allocator: &mut ReflectAllocator,
        f: F,
    ) -> ScriptResult<O> {
        let mut access = world
            .get_access_timeout(
                self.base.base_id.get_reflect_access_id(),
                DEFAULT_TIMEOUT,
                DEFAULT_INTERVAL,
            );

        let reflect = self
            .reflect_mut(
                world.as_unsafe_world_cell(),
                &mut access,
                type_registry,
                Some(allocator),
            ).map(|r| f(r, type_registry, allocator));
        
        world.release_access(access);
        reflect
    }


    fn tail_type_id(&self, world: &WorldAccessGuard) -> ScriptResult<Option<TypeId>> {
        self.with_reflect(world, |r, _, _| {
            r.get_represented_type_info().map(|t| t.type_id())
        })
    }

    fn element_type_id(&self, world: &WorldAccessGuard) -> ScriptResult<Option<TypeId>> {
        self.with_reflect(world, |r, _, _| {
            r.element_type_id()
        })
    }

    fn key_type_id(&self, world: &WorldAccessGuard) -> ScriptResult<Option<TypeId>> {
        self.with_reflect(world, |r, _, _| {
            r.key_type_id()
        })
    }

    pub fn type_id_of(&self, source: TypeIdSource, world: &WorldAccessGuard) -> ScriptResult<Option<TypeId>> {
        match source {
            TypeIdSource::Tail => self.tail_type_id(world),
            TypeIdSource::Element => self.element_type_id(world),
            TypeIdSource::Key => self.key_type_id(world),
        }
    }

    pub fn map_type_data<D,D2,O,F>(type_id: Option<TypeId>, world: &WorldAccessGuard, map: F) -> ScriptResult<O>
    where 
        F: FnOnce(Option<Either<D,D2>>) -> O,
        D: TypeData + Clone,
        D2: TypeData + Clone,
    {
        if let Some(type_id) = type_id {

            if let Some(type_data) = world.with_resource(|_, type_registry: Mut<AppTypeRegistry>| {
                let type_registry = type_registry.read();
                type_registry.get_type_data::<D>(type_id).cloned()
            }) {
                return Ok(map(Some(Either::Left(type_data))));
            } else if let Some(type_data) = world.with_resource(|_, type_registry: Mut<AppTypeRegistry>| {
                let type_registry = type_registry.read();
                type_registry.get_type_data::<D2>(type_id).cloned()
            }) {
                return Ok(map(Some(Either::Right(type_data))));
            }
        }

        Ok(map(None))
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
        allocator: Option<&ReflectAllocator>,
    ) -> ScriptResult<&'c dyn PartialReflect> {
        self.expect_read_access(access, type_registry, allocator, world)?;
        // Safety: since we have read access to the underlying componentId we can safely access the component
        // and we can return a reference tied to its lifetime, which will prevent invalid aliasing
        unsafe { self.reflect_unsafe(world, type_registry, allocator) }
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
        allocator: Option<&ReflectAllocator>,
    ) -> ScriptResult<&'c mut dyn PartialReflect> {
        self.expect_write_access(access, type_registry, allocator, world)?;
        // Safety: since we have write access to the underlying reflect access id we can safely access the component
        // and we can return a reference tied to its lifetime, which will prevent invalid aliasing
        unsafe { self.reflect_mut_unsafe(world, type_registry, allocator) }
    }

    /// Retrieves a reference to the underlying `dyn PartialReflect` type valid for the 'w lifetime of the world cell
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing mut references to the same value exist at all at the same time
    pub unsafe fn reflect_unsafe<'w>(
        &self,
        world: UnsafeWorldCell<'w>,
        type_registry: &TypeRegistry,
        allocator: Option<&ReflectAllocator>,
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
        allocator: Option<&ReflectAllocator>,
    ) -> ScriptResult<&'w mut dyn PartialReflect> {
        if let ReflectBase::Owned(id) = &self.base.base_id {
            let allocator =
                allocator.ok_or_else(|| ScriptError::new_reflection_error("Allocator missing"))?;

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

    fn walk_path_mut<'a>(
        &self,
        root: &'a mut dyn PartialReflect,
    ) -> ScriptResult<&'a mut dyn PartialReflect> {
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
        let type_name = Self::type_name(self.type_id, type_registry);

        let kind = match self.base_id {
            ReflectBase::Component(_, _) => "Component",
            ReflectBase::Resource(_) => "Resource",
            ReflectBase::Owned(_) => "Allocation",
        };

        format!("{}({})", kind, type_name)
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

fn map_key_to_concrete(key: &str, key_type_id: TypeId) -> Option<Box<dyn PartialReflect>> {
    let string_type_id = std::any::TypeId::of::<String>();
    let usize_type_id = std::any::TypeId::of::<usize>();
    let f32_type_id = std::any::TypeId::of::<f32>();
    let bool_type_id = std::any::TypeId::of::<bool>();


    match key_type_id {
        // string_type_id => Some(Box::new(key.to_owned())),
        usize_type_id => key.parse::<usize>().ok().map(|u| Box::new(u) as Box<dyn PartialReflect>),
        f32_type_id => key.parse::<f32>().ok().map(|f| Box::new(f) as Box<dyn PartialReflect>),
        bool_type_id => key.parse::<bool>().ok().map(|b| Box::new(b) as Box<dyn PartialReflect>),
        _ => return None,
    }
}



/// An element in the reflection path, the base reference included
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReflectionPathElem {
    /// A standard reflection path, i.e. `.field_name[vec_index]`, pre-parsed since we construct once potentially use many times
    Reflection(ParsedPath),
    /// a deferred reflection
    DeferredReflection(DeferredReflection),
    /// a map access, i.e. a reference to the key in a map
    MapAccess(String),
    /// a no-op reflection, i.e. a reference to the base object, useful identity to have
    Identity
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
    A: Fn(&dyn PartialReflect) -> Result<&dyn PartialReflect, ReflectPathError<'static>>
        + Send
        + Sync,
    B: Fn(&mut dyn PartialReflect) -> Result<&mut dyn PartialReflect, ReflectPathError<'static>>
        + Send
        + Sync,
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
            ReflectionPathElem::Identity => Ok(root),
            ReflectionPathElem::MapAccess(key) => {
                if let ReflectRef::Map(map) = root.reflect_ref() {
                    let key_type_id = map.key_type_id().expect("Expected map keys to represent a type to be able to assign values");
                    let key = map_key_to_concrete(key, key_type_id)
                        .ok_or_else(|| ReflectPathError::InvalidDowncast)?;
                    map.get(key.as_ref()).ok_or_else(|| ReflectPathError::InvalidDowncast)
                } else {
                    Err(ReflectPathError::InvalidDowncast)
                }
            },
            
        }
    }

    fn reflect_element_mut<'r>(
        self,
        root: &'r mut dyn PartialReflect,
    ) -> Result<&'r mut dyn PartialReflect, ReflectPathError<'a>> {
        match self {
            ReflectionPathElem::Reflection(path) => path.reflect_element_mut(root),
            ReflectionPathElem::DeferredReflection(defref) => (defref.get_mut)(root),
            ReflectionPathElem::Identity => Ok(root),
            ReflectionPathElem::MapAccess(key) => {
                if let ReflectMut::Map(map) = root.reflect_mut() {
                    let key_type_id = map.key_type_id().expect("Expected map keys to represent a type to be able to assign values");
                    let key = map_key_to_concrete(key, key_type_id)
                        .ok_or_else(|| ReflectPathError::InvalidDowncast)?;
                    map.get_mut(key.as_ref()).ok_or_else(|| ReflectPathError::InvalidDowncast)
                } else {
                    Err(ReflectPathError::InvalidDowncast)
                }
            },
            
        }
    }
}

/// A ReflectPath which can perform arbitrary operations on the root object to produce a sub-reference
#[derive(Clone)]
pub struct DeferredReflection {
    pub get: Arc<
        dyn Fn(&dyn PartialReflect) -> Result<&dyn PartialReflect, ReflectPathError<'static>>
            + Send
            + Sync,
    >,
    pub get_mut: Arc<
        dyn Fn(
                &mut dyn PartialReflect,
            ) -> Result<&mut dyn PartialReflect, ReflectPathError<'static>>
            + Send
            + Sync,
    >,
}

/// Given a function, repeats it with a mutable reference for the get_mut deferred variant
#[macro_export]
macro_rules! new_deferred_reflection {
    (|$root:ident| {$($get:tt)*}) => {
        $crate::bindings::reference::DeferredReflection::from((
            |$root: &dyn PartialReflect| -> Result<&dyn PartialReflect, bevy::reflect::ReflectPathError<'static>> {
                $($get)*
            },
            |$root: &mut dyn PartialReflect| -> Result<&mut dyn PartialReflect, bevy::reflect::ReflectPathError<'static>> {
                $($get)*
            },
        ))
    };
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


/// A generic iterator over any reflected value.
/// Unlike a normal iterator, this one does not have a halting condition, it will keep returning elements forever.
/// The iterator does not try to access the value, it just works out the next index/key to access.
/// You will know you've reached the end when you get an error when trying to access the next element.
#[derive(Debug,Clone)]
pub struct ReflectRefIter {
    pub(crate) base: ReflectReference,
    // TODO: support maps etc
    pub(crate) index: IterationKey,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IterationKey {
    Index(usize),
}

impl ReflectRefIter {
    pub fn new_indexed(base: ReflectReference) -> Self {
        Self { base, index: IterationKey::Index(0) }
    }

    pub fn index(&self) -> IterationKey {
        self.index.clone()
    }

    /// Returns the next element in the iterator, it does not have a halting condition
    pub fn next_ref(&mut self) -> (ReflectReference, IterationKey) {
        let index = self.index();
        let next = match &mut self.index {
            IterationKey::Index(i) => {
                let mut next = self.base.clone();
                let parsed_path = ParsedPath::parse(&format!("[{}]", *i)).expect("invariant violated");
                next.index_path(ReflectionPathElem::Reflection(parsed_path));
                *i += 1;
                next
            }
        };
        (next, index)
    }
}

impl Iterator for ReflectRefIter {
    type Item = Result<ReflectReference, ScriptError>;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Result<_, _> = {
            match &mut self.index {
                IterationKey::Index(i) => {
                    let mut next = self.base.clone();
                    let parsed_path = ParsedPath::parse(&i.to_string()).unwrap();
                    next.index_path(ReflectionPathElem::Reflection(parsed_path));
                    *i += 1;
                    Ok(next)
                }
            }
        };

        Some(result)
    }
}

pub struct ReflectReferencePrinter{
    pub(crate) reference: ReflectReference,
}

#[derive(Clone, Copy, Debug)]
enum BracketType {
    Square,
    Curly,
    Round,
}

impl BracketType {

    fn open(self) -> char {
        match self {
            BracketType::Square => '[',
            BracketType::Curly => '{',
            BracketType::Round => '(',
        }
    }

    fn close(self) -> char {
        match self {
            BracketType::Square => ']',
            BracketType::Curly => '}',
            BracketType::Round => ')',
        }
    }

    fn surrounded<F: FnOnce(&mut String)>(self, output: &mut String, f: F) {
        output.push(self.open());
        f(output);
        output.push(self.close());
    }
}

macro_rules! downcast_case {
    ($id:ident, $out:ident, $t:path) => {
        {
            $out.push_str(stringify!($t));
            $out.push('(');
            if let Some($id) = $id.as_partial_reflect().try_downcast_ref::<$t>() {
                $out.push_str(&format!("{:?}", $id));
            } else {
                $out.push_str("<Could not downcast>");
            }
            $out.push(')');
        }
    }
}

impl ReflectReferencePrinter {
    const UNREGISTERED_TYPE: &'static str = "<Unregistered TypeId>";
    const UNKNOWN_FIELD: &'static str = "<Unknown Field>";

    pub fn new(reference: ReflectReference) -> Self {
        Self { reference }
    }
    
    /// Given a reflect reference, prints the type path of the reference resolving the type names with short names.
    /// I.e. `MyType(Component).field_name[0].field_name[1] -> FieldType::Name`
    pub fn pretty_print(&self, world: &WorldAccessGuard) -> String {
        let mut pretty_path = String::new();

        pretty_path.push_str("<Reference to ");

        let root_suffix = match self.reference.base.base_id {
            ReflectBase::Component(e, _) => format!("Component on entity {e}"),
            ReflectBase::Resource(_) => "Resource".to_owned(),
            ReflectBase::Owned(_) => "Allocation".to_owned(),
        };

        let tail_type_id = self.reference.tail_type_id(world).ok().flatten();

        world.with_resource(|_, type_registry : Mut<AppTypeRegistry>| {
            let type_registry = type_registry.read();
            let root_type_path = type_registry.get_type_info(self.reference.base.type_id).map(|t| t.type_path_table().short_path()).unwrap_or("<Unregistered TypeId>");
            pretty_path.push_str(&format!("{}({})", root_suffix, root_type_path));

            for elem in self.reference.reflect_path.iter() {
                match elem {
                    ReflectionPathElem::Reflection(path) => {
                        pretty_path.push_str(&path.to_string());
                    }
                    ReflectionPathElem::DeferredReflection(_) => {
                        pretty_path.push_str(".<Deferred>");
                    }
                    ReflectionPathElem::Identity => {
                        pretty_path.push_str("");
                    }
                    ReflectionPathElem::MapAccess(key) => {
                        pretty_path.push_str(&format!("[{}]", key));
                    },
                    
                }
            }

            if let Some(tail_type_id) = tail_type_id {
                let type_path = type_registry.get_type_info(tail_type_id).map(|t| t.type_path_table().short_path()).unwrap_or(Self::UNREGISTERED_TYPE);
                pretty_path.push_str(&format!(" -> {}", type_path));
            }
        });

        pretty_path.push('>');

        pretty_path
    }


    pub fn pretty_print_value_opaque(&self, v: &dyn PartialReflect, output: &mut String) {
        
        let type_id = v.get_represented_type_info().map(|t| t.type_id()).type_id_or_fake_id();
        output.push_str("Reflect(");
        match type_id {
            id if id == TypeId::of::<usize>() => downcast_case!(v, output, usize),
            id if id == TypeId::of::<isize>() => downcast_case!(v, output, isize),
            id if id == TypeId::of::<f32>() => downcast_case!(v, output, f32),
            id if id == TypeId::of::<f64>() => downcast_case!(v, output, f64),
            id if id == TypeId::of::<u128>() => downcast_case!(v, output, u128),
            id if id == TypeId::of::<u64>() => downcast_case!(v, output, u64),
            id if id == TypeId::of::<u32>() => downcast_case!(v, output, u32),
            id if id == TypeId::of::<u16>() => downcast_case!(v, output, u16),
            id if id == TypeId::of::<u8>() => downcast_case!(v, output, u8),
            id if id == TypeId::of::<i128>() => downcast_case!(v, output, i128),
            id if id == TypeId::of::<i64>() => downcast_case!(v, output, i64),
            id if id == TypeId::of::<i32>() => downcast_case!(v, output, i32),
            id if id == TypeId::of::<i16>() => downcast_case!(v, output, i16),
            id if id == TypeId::of::<i8>() => downcast_case!(v, output, i8),
            id if id == TypeId::of::<String>() => downcast_case!(v, output, String),
            id if id == TypeId::of::<bool>() => downcast_case!(v, output, bool),
            _ => {
                output.push_str(v.get_represented_type_info().map(|t| t.type_path()).unwrap_or(Self::UNREGISTERED_TYPE));
            }
        }
        output.push(')');

    }

    /// Prints the actual value of the reference. Tries to use best available method to print the value.
    pub fn pretty_print_value(&self, world: &WorldAccessGuard) -> String {
        let mut output = String::new();

        // instead of relying on type registrations, simply traverse the reflection tree and print sensible values
        self.reference.with_reflect(world, |r, _, _| {
            self.pretty_print_value_inner(r, &mut output);
        }).unwrap_or_else(|e| {
            output.push_str(&format!("<Error in printing: {}>", e));
        });

        output
    }

    fn pretty_print_key_values<K: AsRef<str>,V: AsRef<str>,I: IntoIterator<Item=(Option<K>,V)>>(bracket: BracketType, i: I, output: &mut String) {
        bracket.surrounded(output, |output| {
            let mut iter = i.into_iter().peekable();
            while let Some((key, val)) = iter.next() {
                if let Some(key) = key {
                    output.push_str(key.as_ref());
                    output.push_str(": ");
                }
                output.push_str(val.as_ref());
                if iter.peek().is_some() {
                    output.push_str(", ");
                }
            }
        });
    }


    fn pretty_print_value_struct<'k,N: Iterator<Item=&'k str>, M: Iterator<Item=&'k dyn PartialReflect>>(&self, field_names: N, field_values: M, output: &mut String) {
        let field_names = field_names.into_iter();
        let fields = field_values.into_iter();
        let fields_iter = fields.zip_longest(field_names).map(|e| {
            let (val, name) = match e {
                itertools::EitherOrBoth::Both(val, name) => (val, name),
                itertools::EitherOrBoth::Left(val) => (val, Self::UNKNOWN_FIELD),
                itertools::EitherOrBoth::Right(name) => (().as_partial_reflect(), name),
            };
            let mut field_printed = String::new();
            self.pretty_print_value_inner(val, &mut field_printed);
            (Some(name), field_printed)
        });
        Self::pretty_print_key_values(BracketType::Curly, fields_iter, output);
    }

    fn pretty_print_value_inner(&self, v: &dyn PartialReflect, output: &mut String) {
        match v.reflect_ref() {
            bevy::reflect::ReflectRef::Struct(s) => {
                let field_names = s.get_represented_struct_info().map(|info| info.field_names()).unwrap_or_default().iter();
                let field_values = s.iter_fields();

                self.pretty_print_value_struct(field_names.copied(), field_values, output);

            },
            ReflectRef::TupleStruct(t) => {
                let fields_iter = t.iter_fields().enumerate().map(|(i, val)| {
                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(val, &mut field_printed);
                    (Some(i.to_string()), field_printed)
                });
                Self::pretty_print_key_values(BracketType::Round, fields_iter, output);
            },
            ReflectRef::Tuple(t) => {
                let fields_iter = t.iter_fields().map(|val| {
                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(val, &mut field_printed);
                    (None::<String>, field_printed)
                });
                Self::pretty_print_key_values(BracketType::Round, fields_iter, output);
            },
            ReflectRef::List(l) => {
                let fields_iter = l.iter().map(|val| {
                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(val, &mut field_printed);
                    (None::<String>, field_printed)
                });
                Self::pretty_print_key_values(BracketType::Square, fields_iter, output);
            },
            ReflectRef::Array(a) => {
                let fields_iter = a.iter().map(|val| {
                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(val, &mut field_printed);
                    (None::<String>, field_printed)
                });
                Self::pretty_print_key_values(BracketType::Square, fields_iter, output);
            },
            ReflectRef::Map(m) => {
                let fields_iter = m.iter().map(|(key, val)| {
                    let mut key_printed = String::new();
                    self.pretty_print_value_inner(key, &mut key_printed);

                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(val, &mut field_printed);
                    (Some(key_printed), field_printed)
                });
                Self::pretty_print_key_values(BracketType::Curly, fields_iter, output);
            },
            ReflectRef::Set(s) => {
                let fields_iter = s.iter().map(|val| {
                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(val, &mut field_printed);
                    (None::<String>, field_printed)
                });
                Self::pretty_print_key_values(BracketType::Curly, fields_iter, output);
            },
            ReflectRef::Enum(e) => {
                output.push_str(&e.variant_path());
                let bracket_type = match e.variant_type() {
                    bevy::reflect::VariantType::Tuple => BracketType::Round,
                    _ => BracketType::Curly,
                };
                let key_vals = e.iter_fields().map(|v| {
                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(v.value(), &mut field_printed);
                    (v.name(), field_printed)
                });
                Self::pretty_print_key_values(bracket_type, key_vals, output);
            },
            ReflectRef::Opaque(o) =>  {
                self.pretty_print_value_opaque(o, output);
            },
        }

    } 
}