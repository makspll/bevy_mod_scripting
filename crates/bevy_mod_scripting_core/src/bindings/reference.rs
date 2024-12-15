//! # Motivation
//!
//! Traits and structs needed to support the creation of bindings for scripting languages.
//! reflection gives us access to `dyn PartialReflect` objects via their type name,
//! Scripting languages only really support `Clone` objects so if we want to support references,
//! we need wrapper types which have owned and ref variants.
use std::{
    any::TypeId,
    fmt::Debug,
    sync::Arc,
};
use bevy::{
    ecs::{
        change_detection::MutUntyped,
        component::ComponentId,
        entity::Entity,
        world::unsafe_world_cell::UnsafeWorldCell,
    }, prelude::{Component, Resource}, ptr::Ptr, reflect::{
        func::{args::ArgInfo, ArgValue},
        ParsedPath,
        PartialReflect,
        ReflectFromPtr,
        ReflectMut,
        ReflectPath,
        ReflectPathError,
        ReflectRef,
        TypeData,
    }
};
use itertools::Either;
use crate::{
    bindings::{pretty_print::DisplayWithWorld, ReflectAllocationId}, error::{ReflectReferenceError, ValueConversionError}, prelude::{ReflectAllocator, ScriptError, ScriptResult}, reflection_extensions::PartialReflectExt, with_access_read, with_access_write
};
use super::{
    access_map::{AccessMapKey, ReflectAccessId},
    WorldAccessGuard, WorldCallbackAccess, WorldGuard,
};

/// An accessor to a `dyn PartialReflect` struct, stores a base ID of the type and a reflection path
/// safe to build but to reflect on the value inside you need to ensure aliasing rules are upheld
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReflectReference {
    pub base: ReflectBaseType,
    // TODO: experiment with Fixed capacity vec, boxed array etc, compromise between heap allocation and runtime cost
    // needs benchmarks first though
    /// The path from the top level type to the actual value we want to access
    pub reflect_path: ParsedPath,
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
    pub fn len(&self, world: WorldGuard) -> ScriptResult<Option<usize>> {
        self
            .with_reflect(world, |r| {
                match r.reflect_ref() {
                    bevy::reflect::ReflectRef::Struct(s) => Some(s.field_len()),
                    bevy::reflect::ReflectRef::TupleStruct(ts) => Some(ts.field_len()),
                    bevy::reflect::ReflectRef::Tuple(t) => Some(t.field_len()),
                    bevy::reflect::ReflectRef::List(l) => Some(l.len()),
                    bevy::reflect::ReflectRef::Array(a) => Some(a.len()),
                    bevy::reflect::ReflectRef::Map(m) => Some(m.len( )),
                    bevy::reflect::ReflectRef::Set(s) => Some(s.len()),
                    bevy::reflect::ReflectRef::Enum(e) => Some(e.field_len()),
                    _ => None,
                }
            })
    }

    pub fn new_world() -> Self {
        Self {
            base: ReflectBaseType {
                type_id: TypeId::of::<WorldCallbackAccess>(),
                base_id: ReflectBase::World,
            },
            reflect_path: ParsedPath(Vec::default()),
        }
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
            reflect_path: ParsedPath(Vec::default()),
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
            reflect_path: ParsedPath(Vec::default()),
        }
    }

    pub fn new_resource_ref<T: Resource>(world: WorldGuard) -> Option<Self> {
        let reflect_id = ReflectAccessId::for_resource::<T>(&world.as_unsafe_world_cell())?;
        Some(Self {
            base: ReflectBaseType {
                type_id: TypeId::of::<T>(),
                base_id: ReflectBase::Resource(reflect_id.into()),
            },
            reflect_path: ParsedPath(Vec::default()),
        })
    }

    pub fn new_component_ref<T: Component>(
        entity: Entity,
        world: WorldGuard,
    ) -> Option<Self> {
        let reflect_id = ReflectAccessId::for_component::<T>(&world.as_unsafe_world_cell())?;
        Some(Self {
            base: ReflectBaseType {
                type_id: TypeId::of::<T>(),
                base_id: ReflectBase::Component(entity, reflect_id.into()),
            },
            reflect_path: ParsedPath(Vec::default()),
        })
    }

    /// Indexes into the reflect path inside this reference.
    /// You can use [`Self::reflect`] and [`Self::reflect_mut`] to get the actual value.
    pub fn index_path<T: Into<ParsedPath>>(&mut self, index: T) {
        debug_assert!(!matches!(self.base.base_id, ReflectBase::World), "Trying to index into a world reference. This will always fail");
        self.reflect_path.0.extend(index.into().0);
    }


    /// Tries to downcast to the specified type and cloning the value if successful.
    pub fn downcast<O: Clone + PartialReflect>(&self, world: WorldGuard) -> ScriptResult<O> {
        self.with_reflect(world, |r| {
            r.try_downcast_ref::<O>().cloned()
        }).transpose().ok_or_else(|| ScriptError::new_reflection_error(ValueConversionError::TypeMismatch {
            expected_type: std::any::type_name::<O>().into(),
            actual_type: None,
        }))?
    }

    /// Reflects into the value of the reference, cloning it using [`PartialReflect::clone_value`] or if it's a world reference, cloning the world access.
    pub fn clone_value(&self, world: WorldGuard) -> ScriptResult<Box<dyn PartialReflect>> {
        if matches!(self.base.base_id, ReflectBase::World) {
            return Ok(Box::new(WorldCallbackAccess::from_guard(world)));
        }
        self.with_reflect(world, |r| {
            r.clone_value()
        })
    }

    /// The way to access the value of the reference, that is the pointed-to value.
    /// This method is safe to use as it ensures no-one else has aliasing access to the value at the same time.
    /// 
    /// # Panics
    /// - if the value is aliased and the access is not allowed
    pub fn with_reflect<O, F: FnOnce(&dyn PartialReflect) -> O>(
        &self,
        world: WorldGuard,
        f: F,
    ) -> ScriptResult<O> {
        debug_assert!(!matches!(self.base.base_id, ReflectBase::World), "Trying to access a world reference directly. This will always fail");

        let access_id = ReflectAccessId::for_reference(self.base.base_id.clone()).ok_or_else(|| ReflectReferenceError::InvalidBaseReference { reason: format!("For Reference: {}. Component or allocation id is invalid.", self.display_with_world(world.clone())).into() })?;
        with_access_read!(world.0.accesses, access_id ,"could not access reflect reference",{
            unsafe { self.reflect_unsafe(world.clone()) }
            .map(f)
        })
    }


    /// The way to access the value of the reference, that is the pointed-to value.
    /// This method is safe to use as it ensures no-one else has aliasing access to the value at the same time.
    /// 
    /// # Panics
    /// - if the value is aliased and the access is not allowed
    pub fn with_reflect_mut<O, F: FnOnce(&mut dyn PartialReflect) -> O>(
        &self,
        world: WorldGuard,
        f: F,
    ) -> ScriptResult<O> {
        debug_assert!(!matches!(self.base.base_id, ReflectBase::World), "Trying to access a world reference directly. This will always fail");

        let access_id = ReflectAccessId::for_reference(self.base.base_id.clone()).ok_or_else(|| ReflectReferenceError::InvalidBaseReference { reason: format!("For Reference: {}. Component or allocation id is invalid.", self.display_with_world(world.clone())).into() })?;
        with_access_write!(world.0.accesses, access_id, "Could not access reflect reference mutably", {
            unsafe { self.reflect_mut_unsafe(world.clone()) }
            .map(f)
        })
    }


    pub fn tail_type_id(&self, world: WorldGuard) -> ScriptResult<Option<TypeId>> {
        if self.reflect_path.is_empty() {
            return Ok(Some(self.base.type_id));
        }
        self.with_reflect(world, |r| {
            r.get_represented_type_info().map(|t| t.type_id())
        })
    }

    pub fn element_type_id(&self, world: WorldGuard) -> ScriptResult<Option<TypeId>> {
        self.with_reflect(world, |r| {
            r.element_type_id()
        })
    }

    pub fn key_type_id(&self, world: WorldGuard) -> ScriptResult<Option<TypeId>> {
        self.with_reflect(world, |r| {
            r.key_type_id()
        })
    }

    pub fn type_id_of(&self, source: TypeIdSource, world: WorldGuard) -> ScriptResult<Option<TypeId>> {
        match source {
            TypeIdSource::Tail => self.tail_type_id(world),
            TypeIdSource::Element => self.element_type_id(world),
            TypeIdSource::Key => self.key_type_id(world),
        }
    }

    pub fn map_type_data<D,D2,O,F>(type_id: Option<TypeId>, world: WorldGuard, map: F) -> ScriptResult<O>
    where 
        F: FnOnce(Option<Either<D,D2>>) -> O,
        D: TypeData + Clone,
        D2: TypeData + Clone,
    {
        if let Some(type_id) = type_id {
            let type_registry = world.type_registry();
            let type_registry = type_registry.read();
            if let Some(type_data) = type_registry.get_type_data::<D>(type_id).cloned() {
                drop(type_registry);
                return Ok(map(Some(Either::Left(type_data))));
            } else if let Some(type_data) = type_registry.get_type_data::<D2>(type_id).cloned() {
                drop(type_registry);
                return Ok(map(Some(Either::Right(type_data))));
            }
        }

        Ok(map(None))
    }

    /// Converts the reference into an argument that can be consumed by a dynamic function.
    /// 
    /// In the case that the underlying value is dynamic, this will convert to an [`ArgValue::Owned`] Which notably will not be directly usable by the function.
    /// To use the output you must make sure your calling mechanism is lenient enough to accept Owned variants in place of refs/muts.
    /// 
    /// Or alternatively convert any non concrete types to concrete types using `from_reflect` before calling this function.
    /// 
    /// # Safety
    /// - The caller must ensure this reference has permission to access the underlying value
    pub unsafe fn into_arg_value<'w>(self, world: WorldGuard<'w>, arg_info: &ArgInfo) -> ScriptResult<ArgValue<'w>> {
        if ReflectBase::World == self.base.base_id {
            // Safety: we already have an Arc<WorldAccessGuard<'w>> so creating a new one from the existing one is safe
            // as the caller of this function will make sure the Arc is dropped after the lifetime 'w is done.
            let new_guard = WorldCallbackAccess::from_guard(world.clone());
            new_guard.read().unwrap();
            return Ok(ArgValue::Owned(Box::new(WorldCallbackAccess::from_guard(world))));
        }

        match arg_info.ownership() {
            bevy::reflect::func::args::Ownership::Ref => {
                
                let mut ref_ = unsafe {self.reflect_unsafe(world.clone())?};

                if ref_.is_dynamic() {
                    let allocator = world.allocator();
                    let mut allocator = allocator.write();
                    
                    let boxed = <dyn PartialReflect as PartialReflectExt>::from_reflect(ref_, world)?.into_partial_reflect();
                    let (_, allocation) = allocator.allocate_boxed(boxed); 
                    // Safety:
                    // we are the only ones with this id, nobody else can touch this.
                    // we only need to make sure this doesn't get dropped before we're done with it
                    // the only time that will happen is when ReflectAllocator::clean_garbage_allocations is called
                    // this will not happen while script event handlers are running
                    ref_ = unsafe { &*allocation.get_ptr() };
                }
                Ok(ArgValue::Ref(ref_))
            },
            bevy::reflect::func::args::Ownership::Mut => {
                
                let mut mut_ = unsafe {self.reflect_mut_unsafe(world.clone())?};

                if mut_.is_dynamic() {
                    let allocator = world.allocator();
                    let mut allocator = allocator.write();
                    
                    let boxed = <dyn PartialReflect as PartialReflectExt>::from_reflect(mut_, world)?.into_partial_reflect();
                    let (_, allocation) = allocator.allocate_boxed(boxed); 
                    // Safety:
                    // same as the ref branch
                    mut_ = unsafe { &mut *allocation.get_ptr() };
                }

                Ok(ArgValue::Mut(mut_))
                
            }
            _ => {
                let ref_ = unsafe {self.reflect_unsafe(world.clone())?};
                Ok(ArgValue::Owned(<dyn PartialReflect as PartialReflectExt>::from_reflect(ref_, world)?.into_partial_reflect()))
            }
        }
        

    }


    /// Retrieves a reference to the underlying `dyn PartialReflect` type valid for the 'w lifetime of the world cell
    /// # Safety
    /// 
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing references to the same value exist at all at the same time
    /// 
    /// To do this safely you need to use [`WorldAccessGuard::claim_read_access`] or [`WorldAccessGuard::claim_global_access`] to ensure nobody else is currently accessing the value.
    pub unsafe fn reflect_unsafe<'w>(
        &self,
        world: WorldGuard<'w>
    ) -> ScriptResult<&'w dyn PartialReflect> {
        
        if let ReflectBase::Owned(id) = &self.base.base_id {
            
            let allocator = world.allocator();
            let allocator = allocator.read();
            
            let arc = allocator
                .get(id)
                .ok_or_else(|| ScriptError::new_reflection_error("Missing allocation"))?;

            // safety: caller promises it's fine :)
            return self.walk_path(unsafe { &*arc.get_ptr() });
        }

        
        let type_registry = world.type_registry();
        let type_registry = type_registry.read();
        

        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = 
            type_registry
                .get_type_data(self.base.type_id)
                .ok_or_else(|| ScriptError::new_reflection_error(
                    format!("FromPtr is not registered for {}", self.display_with_world(world.clone())))
                )?;

        let ptr = self
            .base
            .base_id
            .clone()
            .into_ptr(world.as_unsafe_world_cell())
            .ok_or_else(|| 
                ScriptError::new_reflection_error(
                    format!("Base reference is invalid, is the component/resource initialized? does the entity exist?. When accessing: `{}`", self.base.display_with_world(world))))?;
        

        // (Ptr) Safety: we use the same type_id to both
        // 1) retrieve the ptr
        // 2) retrieve the ReflectFromPtr type data
        // (UnsafeWorldCell) Safety:
        // we already have access to &world so no &mut world exists
        debug_assert_eq!(
            from_ptr_data.type_id(),
            self.base.type_id,
            "Safety invariant violated"
        );

        let base = unsafe { from_ptr_data.as_reflect(ptr) };
        

        drop(type_registry);

        self.walk_path(base.as_partial_reflect())
    }

    /// Retrieves mutable reference to the underlying `dyn PartialReflect` type valid for the 'w lifetime of the world cell
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no other references to the same value exist at all at the same time (even if you have the correct access)
    /// 
    /// To do this safely you need to use [`WorldAccessGuard::claim_write_access`] or [`WorldAccessGuard::claim_global_access`] to ensure nobody else is currently accessing the value.
    pub unsafe fn reflect_mut_unsafe<'w>(
        &self,
        world: WorldGuard<'w>
    ) -> ScriptResult<&'w mut dyn PartialReflect> {
        if let ReflectBase::Owned(id) = &self.base.base_id {
            let allocator = world.allocator();
            let allocator = allocator.read();
            let arc = allocator
                .get_mut(id)
                .ok_or_else(|| ScriptError::new_reflection_error("Missing allocation"))?;

            // Safety: caller promises this is fine :)
            return self.walk_path_mut(unsafe { &mut *arc.get_ptr() });
        };


        let type_registry = world.type_registry();
        let type_registry = type_registry.read();

        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = type_registry
            .get_type_data(self.base.type_id)
            .ok_or_else(|| 
                ScriptError::new_reflection_error(
                    format!("Base reference is invalid, is the component/resource initialized? When accessing: `{}`", self.base.display_with_world(world.clone()))))?;

        let ptr = self
         .base
         .base_id
         .clone()
         .into_ptr_mut(world.as_unsafe_world_cell())
         .ok_or_else(|| 
            ScriptError::new_reflection_error(
                format!("Base reference is invalid, is the component/resource initialized? does the entity exist?. When accessing: `{}`", self.base.display_with_world(world))))?;

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
        let base = unsafe { from_ptr_data.as_reflect_mut(ptr.into_inner()) };
        drop(type_registry);
        self.walk_path_mut(base.as_partial_reflect_mut())
    }

    fn walk_path<'a>(&self, root: &'a dyn PartialReflect) -> ScriptResult<&'a dyn PartialReflect> {
        self.reflect_path.reflect_element(root).map_err(|e| ScriptError::new_reflection_error(e.to_string()))
    }

    fn walk_path_mut<'a>(
        &self,
        root: &'a mut dyn PartialReflect,
    ) -> ScriptResult<&'a mut dyn PartialReflect> {
        self.reflect_path.reflect_element_mut(root).map_err(|e| ScriptError::new_reflection_error(e.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct ReflectBaseType {
    pub type_id: TypeId,
    pub base_id: ReflectBase,
}

/// The Id of the kind of reflection base being pointed to
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum ReflectBase {
    Component(Entity, ComponentId),
    Resource(ComponentId),
    Owned(ReflectAllocationId),
    World
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
}

fn map_key_to_concrete(key: &str, key_type_id: TypeId) -> Option<Box<dyn PartialReflect>> {


    match key_type_id {
        _ if key_type_id == std::any::TypeId::of::<String>() => Some(Box::new(key.to_owned())),
        _ if key_type_id == std::any::TypeId::of::<usize>()  => key.parse::<usize>().ok().map(|u| Box::new(u) as Box<dyn PartialReflect>),
        _ if key_type_id == std::any::TypeId::of::<f32>() => key.parse::<f32>().ok().map(|f| Box::new(f) as Box<dyn PartialReflect>),
        _ if key_type_id == std::any::TypeId::of::<bool>() => key.parse::<bool>().ok().map(|b| Box::new(b) as Box<dyn PartialReflect>),
        _ => None,
    }
}


pub trait ReflectionPathExt {
    fn convert_to_0_indexed(&mut self);

    fn is_empty(&self) -> bool;

    fn iter(&self) -> impl Iterator<Item = &bevy::reflect::OffsetAccess>;
}

impl ReflectionPathExt for ParsedPath {

    /// Assumes the accesses are 1 indexed and converts them to 0 indexed
    fn convert_to_0_indexed(&mut self){
        self.0.iter_mut().for_each(|a| match a.access {
            bevy::reflect::Access::FieldIndex(ref mut i) => *i -= 1,
            bevy::reflect::Access::TupleIndex(ref mut i) => *i -= 1,
            bevy::reflect::Access::ListIndex(ref mut i) => *i -= 1,
            _ => {}
        });
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = &bevy::reflect::OffsetAccess> {
        self.0.iter()
    }
    
}


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
                next.index_path(parsed_path);
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
                    next.index_path(parsed_path);
                    *i += 1;
                    Ok(next)
                }
            }
        };

        Some(result)
    }
}
