//! # Motivation
//!
//! Traits and structs needed to support the creation of bindings for scripting languages.
//! reflection gives us access to `dyn PartialReflect` objects via their type name,
//! Scripting languages only really support `Clone` objects so if we want to support references,
//! we need wrapper types which have owned and ref variants.
use super::{WorldGuard, access_map::ReflectAccessId};
use crate::{
    ReferencePart, ReferencePath, ReflectAllocationId, ReflectAllocator, ThreadWorldContainer,
    error::InteropError, reflection_extensions::PartialReflectExt, with_access_read,
    with_access_write,
};
use bevy_asset::{ReflectAsset, UntypedHandle};
use bevy_ecs::{component::Component, ptr::Ptr, resource::Resource};
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_mod_scripting_display::{
    DebugWithTypeInfo, DisplayWithTypeInfo, OrFakeId, PrintReflectAsDebug, WithTypeInfo,
};
use bevy_reflect::{Access, OffsetAccess, ReflectRef, TypeRegistry};
use core::alloc;
use std::{
    any::{Any, TypeId},
    fmt::Debug,
};
use {
    bevy_ecs::{
        change_detection::MutUntyped, component::ComponentId, entity::Entity,
        world::unsafe_world_cell::UnsafeWorldCell,
    },
    bevy_reflect::{ParsedPath, PartialReflect, Reflect, ReflectFromPtr, prelude::ReflectDefault},
};

/// A reference to an arbitrary reflected instance.
///
/// The reference can point to either the ECS, or to the allocator.
///
/// References are composed of two parts:
/// - The base kind, which specifies where the reference points to
/// - The path, which specifies how to access the value from the base.
///
/// Bindings defined on this type, apply to ALL references.
#[derive(Clone, PartialEq, Reflect, DebugWithTypeInfo)]
#[reflect(Default, opaque)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
#[non_exhaustive]
pub struct ReflectReference {
    /// The base type and id of the value we want to access
    pub base: ReflectBaseType,
    // TODO: experiment with Fixed capacity vec, boxed array etc, compromise between heap allocation and runtime cost
    // needs benchmarks first though
    /// The path from the top level type to the actual value we want to access
    pub reflect_path: ReferencePath,
}

impl DisplayWithTypeInfo for ReflectReference {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn bevy_mod_scripting_display::GetTypeInfo>,
    ) -> std::fmt::Result {
        // try to display the most information we can, the type info provider happens to be the world guard, we can
        // actually display the reference
        if let Some(type_info_provider) = type_info_provider {
            // Safety: should be safe as the guard is invalidated when world is released per iteration
            let any: &dyn Any = unsafe { type_info_provider.as_any_static() };

            let guard = any.downcast_ref::<WorldGuard>().cloned().or_else(|| {
                any.downcast_ref::<ThreadWorldContainer>()
                    .and_then(|t| t.try_get_context().ok().map(|c| c.world))
            });

            if let Some(guard) = guard {
                if let Ok(r) = self.with_reflect(guard.clone(), |s| {
                    PrintReflectAsDebug::new_with_opt_info(s, Some(type_info_provider))
                        .to_string_with_type_info(f, Some(type_info_provider))
                }) {
                    return r;
                }
            }
        }

        f.write_str("(cannot access value, showing reference) ")?;
        self.base.display_with_type_info(f, type_info_provider)?;
        if !self.reflect_path.is_empty() {
            f.write_str(" at path ")?;
            self.reflect_path
                .display_with_type_info(f, type_info_provider)?;
        }

        Ok(())
    }
}

impl Default for ReflectReference {
    fn default() -> Self {
        Self {
            base: ReflectBaseType {
                type_id: None::<TypeId>.or_fake_id(),
                base_id: ReflectBase::Owned(ReflectAllocationId::new(0)),
            },
            reflect_path: Default::default(),
        }
    }
}

/// Specifies where we should source the type id from when reflecting a ReflectReference
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TypeIdSource {
    /// Use the type id the reference points to after walking the path
    Tail,
    /// Given the Tail referene is a container type, use the type id of the elements in the container
    Element,
    /// Givent the Tail reference is a container type, use the type id of the keys of the container
    Key,
}

#[profiling::all_functions]

impl ReflectReference {
    /// If this points to a variant of an enum, returns the name of the variant.
    pub fn variant_name(&self, world: WorldGuard) -> Result<Option<String>, InteropError> {
        self.with_reflect(world, |s| {
            s.reflect_ref()
                .as_enum()
                .ok()
                .map(|enum_ref| enum_ref.variant_name().to_owned())
        })
    }

    /// Creates a new infinite iterator. This iterator will keep returning the next element reference forever.
    pub fn into_iter_infinite(self) -> ReflectRefIter {
        ReflectRefIter::new_indexed(self)
    }

    /// If this is a reference to something with a length accessible via reflection, returns that length.
    pub fn len(&self, world: WorldGuard) -> Result<Option<usize>, InteropError> {
        self.with_reflect(world, |r| match r.reflect_ref() {
            ReflectRef::Struct(s) => Some(s.field_len()),
            ReflectRef::TupleStruct(ts) => Some(ts.field_len()),
            ReflectRef::Tuple(t) => Some(t.field_len()),
            ReflectRef::List(l) => Some(l.len()),
            ReflectRef::Array(a) => Some(a.len()),
            ReflectRef::Map(m) => Some(m.len()),
            ReflectRef::Set(s) => Some(s.len()),
            ReflectRef::Enum(e) => Some(e.field_len()),
            _ => None,
        })
    }

    /// Create a new reference to a value by allocating it.
    ///
    /// You can retrieve the allocator from the world using [`WorldGuard::allocator`].
    /// Make sure to drop the allocator write guard before doing anything with the reference to prevent deadlocks.
    ///
    pub fn new_allocated<T: Reflect>(
        value: T,
        allocator: &mut ReflectAllocator,
    ) -> ReflectReference {
        let type_id = std::any::TypeId::of::<T>();
        let id = allocator.allocate(value);
        ReflectReference {
            base: ReflectBaseType {
                type_id,
                base_id: ReflectBase::Owned(id),
            },
            reflect_path: Default::default(),
        }
    }

    /// Creates a raw reference to an existing allocation without checking the type ID.
    /// If the type id does not match you will get runtime errors
    pub fn new_allocated_raw(type_id: TypeId, id: ReflectAllocationId) -> ReflectReference {
        ReflectReference {
            base: ReflectBaseType {
                type_id,
                base_id: ReflectBase::Owned(id),
            },
            reflect_path: Default::default(),
        }
    }

    /// Create a new reference to a value by allocating it.
    ///
    /// Prefer using [`Self::new_allocated`] if you have a value that implements [`Reflect`].
    /// Will fail if the value does not have represented type info with a specific type id.
    pub fn new_allocated_boxed_parial_reflect(
        value: Box<dyn PartialReflect>,
        allocator: &mut ReflectAllocator,
    ) -> Result<ReflectReference, InteropError> {
        Ok(ReflectReference {
            base: ReflectBaseType::new_allocated_base_partial(value, allocator)?,
            reflect_path: Default::default(),
        })
    }

    /// Create a new reference to a value by allocating it.
    pub fn new_allocated_boxed(
        value: Box<dyn Reflect>,
        allocator: &mut ReflectAllocator,
    ) -> ReflectReference {
        ReflectReference {
            base: ReflectBaseType::new_allocated_base(value, allocator),
            reflect_path: Default::default(),
        }
    }

    /// Create a new reference to resource
    pub fn new_resource_ref<T: Resource>(world: WorldGuard) -> Result<Self, InteropError> {
        Ok(Self {
            base: ReflectBaseType::new_resource_base::<T>(world)?,
            reflect_path: Default::default(),
        })
    }

    /// Create a new reference to component
    pub fn new_component_ref<T: Component>(
        entity: Entity,
        world: WorldGuard,
    ) -> Result<Self, InteropError> {
        Ok(Self {
            base: ReflectBaseType::new_component_base::<T>(entity, world)?,
            reflect_path: Default::default(),
        })
    }

    /// Create a new reference to component by id.
    /// If the type id is incorrect, you will get runtime errors when trying to access the value.
    pub fn new_component_ref_by_id(
        entity: Entity,
        component_id: ComponentId,
        type_id: TypeId,
    ) -> Self {
        Self {
            base: ReflectBaseType {
                type_id,
                base_id: ReflectBase::Component(entity, component_id),
            },
            reflect_path: Default::default(),
        }
    }

    /// Create a new reference to resource by id.
    /// If the type id is incorrect, you will get runtime errors when trying to access the value.
    pub fn new_resource_ref_by_id(component_id: ComponentId, type_id: TypeId) -> Self {
        Self {
            base: ReflectBaseType {
                type_id,
                base_id: ReflectBase::Resource(component_id),
            },
            reflect_path: Default::default(),
        }
    }

    /// Create a new reference to an asset by untyped handle.
    /// If the type id is incorrect, you will get runtime errors when trying to access the value.
    pub fn new_asset_ref(
        handle: UntypedHandle,
        asset_type_id: TypeId,
        world: WorldGuard,
    ) -> Result<Self, InteropError> {
        Ok(Self {
            base: ReflectBaseType::new_asset_base(handle, asset_type_id, world)?,
            reflect_path: Default::default(),
        })
    }

    /// Tries get an untyped asset handle from this reference.
    pub fn try_untyped_asset_handle(
        &self,
        world: WorldGuard,
    ) -> Result<UntypedHandle, InteropError> {
        let handle_type_id = self.tail_type_id(world.clone())?.ok_or_else(|| {
            InteropError::invariant("Cannot determine handle type ID from reflection")
                .with_context("Asset handle reflection failed - handle may be invalid or corrupted")
        })?;

        let type_registry = world.type_registry();
        let type_registry = type_registry.read();
        let reflect_handle = type_registry
            .get_type_data::<bevy_asset::ReflectHandle>(handle_type_id)
            .ok_or_else(|| {
                InteropError::missing_type_data(
                    handle_type_id,
                    stringify!(ReflectHandle).into(),
                )
                .with_context("Handle type is not registered for asset operations - ensure that you registered it with bevy::App::register_asset_reflect::<T>()")
            })?;

        let untyped_handle = self.with_reflect(world.clone(), |reflect| {
            let reflect_any = reflect.try_as_reflect().ok_or_else(|| {
                InteropError::unsupported_operation(
                    Some(handle_type_id),
                    None,
                    "Asset handle must implement Reflect trait for asset operations",
                )
            })?;

            reflect_handle
                .downcast_handle_untyped(reflect_any.as_any())
                .ok_or_else(|| {
                    InteropError::could_not_downcast(self.clone(), handle_type_id).with_context(
                        "UntypedHandle downcast failed - handle may be of wrong type or corrupted",
                    )
                })
        })??;
        Ok(untyped_handle)
    }

    /// Get asset from world and return a mutable reference to it
    unsafe fn load_asset_mut<'w>(
        &self,
        handle: &UntypedHandle,
        world: WorldGuard<'w>,
    ) -> Result<&'w mut dyn Reflect, InteropError> {
        let type_registry = world.type_registry();
        let type_registry = type_registry.read();

        let reflect_asset: &ReflectAsset = type_registry
            .get_type_data(self.base.type_id)
            .ok_or_else(|| InteropError::unregistered_base(self.base.clone()))?;

        let world_cell = world.as_unsafe_world_cell()?;
        // Safety: The caller guarantees exclusive access to the asset through the WorldGuard,
        // and we've validated that the type_id matches the ReflectAsset type data.
        // The UnsafeWorldCell is valid for the lifetime 'w of the WorldGuard.
        let asset = unsafe { reflect_asset.get_unchecked_mut(world_cell, handle.clone()) }
            .ok_or_else(|| {
                InteropError::unsupported_operation(
                    Some(self.base.type_id),
                    None,
                    "Asset not loaded or handle is invalid",
                )
            })?;

        Ok(asset)
    }

    /// Indexes into the reflect path inside this reference.
    /// You can use [`Self::reflect`] and [`Self::reflect_mut`] to get the actual value.
    pub fn extend_path(&mut self, index: impl Iterator<Item = ReferencePart>) {
        self.reflect_path.extend(index);
    }

    /// Indexes into the reflect path inside this reference.
    /// You can use [`Self::reflect`] and [`Self::reflect_mut`] to get the actual value.
    pub fn push_path(&mut self, index: ReferencePart) {
        self.reflect_path.push(index);
    }

    /// Tries to downcast to the specified type and cloning the value if successful.
    pub fn downcast<O: Clone + PartialReflect>(
        &self,
        world: WorldGuard,
    ) -> Result<O, InteropError> {
        self.with_reflect(world, |r| {
            r.try_downcast_ref::<O>()
                .cloned()
                .ok_or_else(|| InteropError::could_not_downcast(self.clone(), TypeId::of::<O>()))
        })?
    }

    /// Attempts to create a `Box<dyn PartialReflect>` from the reference. This is possible using a few strategies:
    /// - If the reference is to a world, a [`crate::world::WorldCallbackAccess`] is created and boxed
    /// - If the reference is to an allocation with no reflection path and references to it, the value is taken as is.
    /// - If the reference has a [`bevy_reflect::ReflectFromReflect`] type data associated with it, the value is cloned using that impl.
    /// - If all above fails, [`bevy_reflect::PartialReflect::clone_value`] is used to clone the value.
    ///
    pub fn to_owned_value(
        &self,
        world: WorldGuard,
    ) -> Result<Box<dyn PartialReflect>, InteropError> {
        if let ReflectBase::Owned(id) = &self.base.base_id
            && self.reflect_path.is_empty()
            && id.strong_count() == 0
        {
            let allocator = world.allocator();
            let mut allocator = allocator.write();
            let arc = allocator
                .remove(id)
                .ok_or_else(|| InteropError::garbage_collected_allocation(self.clone()))?;

            let access_id = ReflectAccessId::for_allocation(id.clone());
            if world.claim_write_access(access_id) {
                // Safety: we claim write access, nobody else is accessing this
                if unsafe { &*arc.get_ptr() }.try_as_reflect().is_some() {
                    // Safety: the only accesses exist in this function
                    unsafe { world.release_access(access_id) };
                    return Ok(unsafe { arc.take() });
                } else {
                    unsafe { world.release_access(access_id) };
                }
            }
            allocator.insert(id.clone(), arc);
        }

        self.with_reflect(world.clone(), |r| {
            <dyn PartialReflect>::from_reflect_or_clone(r, world.clone())
        })?
    }

    /// The way to access the value of the reference, that is the pointed-to value.
    /// This method is safe to use as it ensures no-one else has aliasing access to the value at the same time.
    #[track_caller]
    pub fn with_reflect<O, F: FnOnce(&dyn PartialReflect) -> O>(
        &self,
        world: WorldGuard,
        f: F,
    ) -> Result<O, InteropError> {
        let access_id = ReflectAccessId::for_reference(self.base.base_id.clone());
        with_access_read!(
            &world.inner.accesses,
            access_id,
            "could not access reflect reference",
            {
                f(
                    unsafe { self.reflect_unsafe(world.clone()) }?.ok_or_else(|| {
                        InteropError::reflection_path_error(
                            "Reference was out of bounds or value is missing".into(),
                            Some(self.clone()),
                        )
                    })?,
                )
            }
        )
    }

    /// The way to access the value of the reference, that is the pointed-to value.
    /// This method is safe to use as it ensures no-one else has aliasing access to the value at the same time.
    #[track_caller]
    pub fn with_reflect_mut<O, F: FnOnce(&mut dyn PartialReflect) -> O>(
        &self,
        world: WorldGuard,
        f: F,
    ) -> Result<O, InteropError> {
        let access_id = ReflectAccessId::for_reference(self.base.base_id.clone());
        with_access_write!(
            &world.inner.accesses,
            access_id,
            "Could not access reflect reference mutably",
            {
                f(
                    unsafe { self.reflect_mut_unsafe(world.clone()) }?.ok_or_else(|| {
                        InteropError::reflection_path_error(
                            "Reference was out of bounds or value is missing".into(),
                            Some(self.clone()),
                        )
                    })?,
                )
            }
        )
    }

    /// Retrieves the type id of the value the reference points to.
    pub fn tail_type_id(&self, world: WorldGuard) -> Result<Option<TypeId>, InteropError> {
        if self.reflect_path.is_empty() {
            return Ok(Some(self.base.type_id));
        }
        self.with_reflect(world, |r| {
            r.get_represented_type_info().map(|t| t.type_id())
        })
    }

    /// Retrieves the type id of the elements in the value the reference points to.
    pub fn element_type_id(&self, world: WorldGuard) -> Result<Option<TypeId>, InteropError> {
        self.with_reflect(world, |r| r.element_type_id())
    }

    /// Retrieves the type id of the keys in the value the reference points to.
    pub fn key_type_id(&self, world: WorldGuard) -> Result<Option<TypeId>, InteropError> {
        self.with_reflect(world, |r| r.key_type_id())
    }

    /// Retrieves the type id of the value the reference points to based on the given source.
    pub fn type_id_of(
        &self,
        source: TypeIdSource,
        world: WorldGuard,
    ) -> Result<Option<TypeId>, InteropError> {
        match source {
            TypeIdSource::Tail => self.tail_type_id(world),
            TypeIdSource::Element => self.element_type_id(world),
            TypeIdSource::Key => self.key_type_id(world),
        }
    }

    /// Equivalent to [`Self::reflect_unsafe`] but expecting a non-None value from the reference, i.e. will return an Err if the reference is to a missing key in a dictionary.
    /// # Safety
    /// - The bounds of [`Self::reflect_unsafe`] must be upheld
    pub unsafe fn reflect_unsafe_non_empty<'w>(
        &self,
        world: WorldGuard<'w>,
    ) -> Result<&'w dyn PartialReflect, InteropError> {
        let val = unsafe { self.reflect_unsafe(world) }?;
        val.ok_or_else(|| {
            InteropError::reflection_path_error(
                "Reference out of bounds or value missing".into(),
                Some(self.clone()),
            )
        })
    }

    /// Retrieves a reference to the underlying `dyn PartialReflect` type valid for the 'w lifetime of the world cell
    /// # Safety
    ///
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no aliasing references to the same value exist at all at the same time
    ///
    /// To do this safely you need to use [`crate::world::WorldAccessGuard::claim_read_access`] or [`crate::world::WorldAccessGuard::claim_global_access`] to ensure nobody else is currently accessing the value.
    pub unsafe fn reflect_unsafe<'w>(
        &self,
        world: WorldGuard<'w>,
    ) -> Result<Option<&'w dyn PartialReflect>, InteropError> {
        if let ReflectBase::Owned(id) = &self.base.base_id {
            let allocator = world.allocator();
            let allocator = allocator.read();

            let arc = allocator
                .get(id)
                .ok_or_else(|| InteropError::garbage_collected_allocation(self.clone()))?;

            // safety: caller promises it's fine :)
            let type_registry = world.type_registry();
            let type_registry = type_registry.read();

            return self.walk_path(unsafe { &*arc.get_ptr() }, &type_registry);
        }

        if let ReflectBase::Asset(handle, _) = &self.base.base_id {
            let asset = unsafe { self.load_asset_mut(handle, world.clone())? };
            let type_registry = world.type_registry();
            let type_registry = type_registry.read();
            return self.walk_path(asset.as_partial_reflect(), &type_registry);
        }

        let type_registry = world.type_registry();
        let type_registry = type_registry.read();

        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = type_registry
            .get_type_data(self.base.type_id)
            .ok_or_else(|| InteropError::unregistered_base(self.base.clone()))?;

        let ptr = unsafe {
            self.base
                .base_id
                .clone()
                .into_ptr(world.as_unsafe_world_cell()?)
        }
        .ok_or_else(|| InteropError::unregistered_base(self.base.clone()))?;

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
        self.walk_path(base.as_partial_reflect(), &type_registry)
    }

    /// Equivalent to [`Self::reflect_mut_unsafe`] but expecting a non-None value from the reference, i.e. will return an Err if the reference is to a missing key in a dictionary.
    /// # Safety
    /// - The bounds of [`Self::reflect_mut_unsafe`] must be upheld
    pub unsafe fn reflect_mut_unsafe_non_empty<'w>(
        &self,
        world: WorldGuard<'w>,
    ) -> Result<&'w mut dyn PartialReflect, InteropError> {
        let val = unsafe { self.reflect_mut_unsafe(world) }?;
        val.ok_or_else(|| {
            InteropError::reflection_path_error(
                "Reference out of bounds or value missing".into(),
                Some(self.clone()),
            )
        })
    }

    /// Retrieves mutable reference to the underlying `dyn PartialReflect` type valid for the 'w lifetime of the world cell
    /// # Safety
    /// - The caller must ensure the cell has permission to access the underlying value
    /// - The caller must ensure no other references to the same value exist at all at the same time (even if you have the correct access)
    ///
    /// To do this safely you need to use [`crate::world::WorldAccessGuard::claim_write_access`] or [`crate::world::WorldAccessGuard::claim_global_access`] to ensure nobody else is currently accessing the value.
    pub unsafe fn reflect_mut_unsafe<'w>(
        &self,
        world: WorldGuard<'w>,
    ) -> Result<Option<&'w mut dyn PartialReflect>, InteropError> {
        if let ReflectBase::Owned(id) = &self.base.base_id {
            let allocator = world.allocator();
            let allocator = allocator.read();
            let arc = allocator
                .get(id)
                .ok_or_else(|| InteropError::garbage_collected_allocation(self.clone()))?;

            // Safety: caller promises this is fine :)
            let type_registry = world.type_registry();
            let type_registry = type_registry.read();
            return self.walk_path_mut(unsafe { &mut *arc.get_ptr() }, &type_registry);
        };

        if let ReflectBase::Asset(handle, _) = &self.base.base_id {
            let asset = unsafe { self.load_asset_mut(handle, world.clone())? };
            let type_registry = world.type_registry();
            let type_registry = type_registry.read();
            return self.walk_path_mut(asset.as_partial_reflect_mut(), &type_registry);
        };

        let type_registry = world.type_registry();
        let type_registry = type_registry.read();

        // all Reflect types should have this derived
        let from_ptr_data: &ReflectFromPtr = type_registry
            .get_type_data(self.base.type_id)
            .ok_or_else(|| InteropError::unregistered_base(self.base.clone()))?;

        let ptr = unsafe {
            self.base
                .base_id
                .clone()
                .into_ptr_mut(world.as_unsafe_world_cell()?)
        }
        .ok_or_else(|| InteropError::unregistered_base(self.base.clone()))?;

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
        self.walk_path_mut(base.as_partial_reflect_mut(), &type_registry)
    }

    fn walk_path<'a>(
        &self,
        root: &'a dyn PartialReflect,
        type_registry: &TypeRegistry,
    ) -> Result<Option<&'a dyn PartialReflect>, InteropError> {
        self.reflect_path
            .reflect_element(root, type_registry)
            .map_err(|e| InteropError::reflection_path_error(e.to_string(), Some(self.clone())))
    }

    fn walk_path_mut<'a>(
        &self,
        root: &'a mut dyn PartialReflect,
        type_registry: &TypeRegistry,
    ) -> Result<Option<&'a mut dyn PartialReflect>, InteropError> {
        self.reflect_path
            .reflect_element_mut(root, type_registry)
            .map_err(|e| InteropError::reflection_path_error(e.to_string(), Some(self.clone())))
    }
}

/// The type id and base id of the value we want to access
#[derive(Clone, PartialEq, Eq, PartialOrd, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub struct ReflectBaseType {
    /// The type id of the value we want to access
    /// This MUST always be inline with the type id we are pointing to
    pub(crate) type_id: TypeId,
    /// The base kind of the value we want to access
    pub base_id: ReflectBase,
}

impl DisplayWithTypeInfo for ReflectBaseType {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn bevy_mod_scripting_display::GetTypeInfo>,
    ) -> std::fmt::Result {
        f.write_str("base type: ")?;
        WithTypeInfo::new_with_opt_info(&self.type_id, type_info_provider)
            .display_with_type_info(f, type_info_provider)?;
        f.write_str(", of kind: ")?;
        self.base_id.display_with_type_info(f, type_info_provider)?;
        Ok(())
    }
}

impl ReflectBaseType {
    #[inline]
    /// Returns the type id of the value pointed to by the base
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// Create a new reflection base pointing to a component on the given entity
    pub fn new_component_base<T: Component>(
        entity: Entity,
        world: WorldGuard,
    ) -> Result<Self, InteropError> {
        let reflect_id = ReflectAccessId::for_component::<T>(&world.as_unsafe_world_cell()?)?;
        Ok(Self {
            type_id: TypeId::of::<T>(),
            base_id: ReflectBase::Component(entity, reflect_id.into()),
        })
    }

    /// Create a new reflection base pointing to a resource
    pub fn new_resource_base<T: Resource>(world: WorldGuard) -> Result<Self, InteropError> {
        let reflect_id = ReflectAccessId::for_resource::<T>(&world.as_unsafe_world_cell()?)?;
        Ok(Self {
            type_id: TypeId::of::<T>(),
            base_id: ReflectBase::Resource(reflect_id.into()),
        })
    }

    /// Create a new reflection base pointing to a value which will be allocated in the allocator
    pub fn new_allocated_base(value: Box<dyn Reflect>, allocator: &mut ReflectAllocator) -> Self {
        let type_id = (*value).type_id();
        let id = allocator.allocate_boxed(value.into_partial_reflect());
        Self {
            type_id,
            base_id: ReflectBase::Owned(id),
        }
    }

    /// Create a new reflection base pointing to a value which will be allocated in the allocator
    pub fn new_allocated_base_partial(
        value: Box<dyn PartialReflect>,
        allocator: &mut ReflectAllocator,
    ) -> Result<Self, InteropError> {
        match value.get_represented_type_info() {
            Some(i) => {
                let id = allocator.allocate_boxed(value);
                Ok(Self {
                    type_id: i.type_id(),
                    base_id: ReflectBase::Owned(id),
                })
            }
            None => Err(InteropError::unsupported_operation(
                None,
                Some(value),
                "Tried to create a reference base to a partial reflect value with no represented type info",
            )),
        }
    }

    /// Create a new reflection base pointing to an asset with untyped handle
    pub fn new_asset_base(
        handle: UntypedHandle,
        asset_type_id: TypeId,
        world: WorldGuard,
    ) -> Result<Self, InteropError> {
        // We need to get the Assets<T> resource ComponentId by type registry lookup
        let type_registry = world.type_registry();
        let type_registry = type_registry.read();

        // Get the ReflectAsset data to find the Assets<T> resource type ID
        let reflect_asset: &ReflectAsset =
            type_registry.get_type_data(asset_type_id).ok_or_else(|| {
                InteropError::unsupported_operation(
                    Some(asset_type_id),
                    None,
                    "Asset type is not registered with ReflectAsset type data",
                )
            })?;

        let assets_resource_type_id = reflect_asset.assets_resource_type_id();

        // Convert the TypeId to ComponentId via unsafe world cell
        let world_cell = world.as_unsafe_world_cell()?;
        let components = world_cell.components();
        let assets_resource_id = components
            .get_resource_id(assets_resource_type_id)
            .ok_or_else(|| {
                InteropError::unsupported_operation(
                    Some(assets_resource_type_id),
                    None,
                    "Assets<T> resource is not registered in the world",
                )
            })?;

        Ok(Self {
            type_id: asset_type_id,
            base_id: ReflectBase::Asset(handle, assets_resource_id),
        })
    }
}

/// The Id of the kind of reflection base being pointed to
#[derive(Clone, PartialEq, Eq, PartialOrd, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub enum ReflectBase {
    /// A component of an entity
    Component(Entity, ComponentId),
    /// A resource
    Resource(ComponentId),
    /// An allocation
    Owned(ReflectAllocationId),
    /// An asset accessed by handle
    Asset(UntypedHandle, ComponentId),
}

impl DisplayWithTypeInfo for ReflectBase {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn bevy_mod_scripting_display::GetTypeInfo>,
    ) -> std::fmt::Result {
        match self {
            ReflectBase::Component(entity, component_id) => {
                f.write_str("component: ")?;
                WithTypeInfo::new_with_opt_info(component_id, type_info_provider)
                    .display_with_type_info(f, type_info_provider)?;
                f.write_str(", on entity: ")?;
                entity.fmt(f)
            }
            ReflectBase::Resource(component_id) => {
                f.write_str("resource: ")?;
                WithTypeInfo::new_with_opt_info(component_id, type_info_provider)
                    .display_with_type_info(f, type_info_provider)
            }
            ReflectBase::Owned(id) => {
                if let Some(type_info_provider) = type_info_provider {
                    // Safety: should generally be safe, as the world guard is invalidated once the world is out of scope for the iteration
                    let any: &dyn Any = unsafe { type_info_provider.as_any_static() };

                    let guard = any.downcast_ref::<WorldGuard>().cloned().or_else(|| {
                        any.downcast_ref::<ThreadWorldContainer>()
                            .and_then(|t| t.try_get_context().ok().map(|c| c.world))
                    });

                    if let Some(guard) = guard {
                        let allocator = guard.allocator();
                        let allocator = allocator.read();
                        if let Some(allocation) = allocator.get(id) {
                            let ptr = allocation.get_ptr();
                            if let Ok(v) = guard.with_read_access(id.clone(), |_| {
                                // Safety:: have access to this id
                                PrintReflectAsDebug::new_with_opt_info(
                                    unsafe { &*ptr },
                                    Some(type_info_provider),
                                )
                                .to_string_with_type_info(f, Some(type_info_provider))
                            }) {
                                return v;
                            }
                        }
                    }
                }

                f.write_str("allocated value with id: ")?;
                WithTypeInfo::new_with_opt_info(id, type_info_provider)
                    .display_with_type_info(f, type_info_provider)
            }
            ReflectBase::Asset(handle, assets_resource_id) => {
                f.write_str("asset with handle: ")?;
                write!(f, "{handle:?}")?;
                f.write_str(", in Assets resource: ")?;
                WithTypeInfo::new_with_opt_info(assets_resource_id, type_info_provider)
                    .display_with_type_info(f, type_info_provider)
            }
        }
    }
}

#[profiling::all_functions]
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
                unsafe { world.get_entity(entity).ok()?.get_by_id(component_id) }
            }
            ReflectBase::Resource(component_id) => {
                // Safety: the caller ensures invariants hold
                unsafe { world.get_resource_by_id(component_id) }
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
                unsafe { world.get_entity(entity).ok()?.get_mut_by_id(component_id) }.ok()
            }
            ReflectBase::Resource(component_id) => {
                // Safety: the caller ensures invariants hold
                unsafe { world.get_resource_mut_by_id(component_id) }
            }
            _ => None,
        }
    }
}

/// An iterator over a reflect reference that will keep returning the next element forever.
pub trait ReflectionPathExt {
    /// Assumes the accesses are 1 indexed and converts them to 0 indexed
    fn convert_to_0_indexed(&mut self);
    /// Returns true if the path is empty
    fn is_empty(&self) -> bool;
    /// Returns an iterator over the accesses
    fn iter(&self) -> impl Iterator<Item = &OffsetAccess>;
}
#[profiling::all_functions]
impl ReflectionPathExt for ParsedPath {
    /// Assumes the accesses are 1 indexed and converts them to 0 indexed
    fn convert_to_0_indexed(&mut self) {
        self.0.iter_mut().for_each(|a| match a.access {
            Access::FieldIndex(ref mut i) => *i -= 1,
            Access::TupleIndex(ref mut i) => *i -= 1,
            Access::ListIndex(ref mut i) => *i -= 1,
            _ => {}
        });
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = &OffsetAccess> {
        self.0.iter()
    }
}

/// A generic iterator over any reflected value.
/// Unlike a normal iterator, this one does not have a halting condition, it will keep returning elements forever.
/// The iterator does not try to access the value, it just works out the next index/key to access.
/// You will know you've reached the end when you get an error when trying to access the next element.
#[derive(Clone, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub struct ReflectRefIter {
    pub(crate) base: ReflectReference,
    // TODO: support maps etc
    pub(crate) index: IterationKey,
}

#[derive(Clone, PartialEq, Eq, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
/// The key of the current iteration
pub enum IterationKey {
    /// The current index
    Index(usize),
}

#[profiling::all_functions]
impl ReflectRefIter {
    /// Creates a new iterator that will keep returning the next element forever.
    pub fn new_indexed(base: ReflectReference) -> Self {
        Self {
            base,
            index: IterationKey::Index(0),
        }
    }

    /// Returns the current index of the iterator
    pub fn index(&self) -> IterationKey {
        self.index.clone()
    }

    /// Returns the next element in the iterator, it does not have a halting condition
    pub fn next_ref(&mut self) -> (ReflectReference, IterationKey) {
        let index = self.index();
        let next = match &mut self.index {
            IterationKey::Index(i) => {
                let mut next = self.base.clone();
                let parsed_path = ReferencePart::IntegerAccess(*i as i64, false);
                next.push_path(parsed_path);
                *i += 1;
                next
            }
        };
        (next, index)
    }
}

#[profiling::all_functions]
impl Iterator for ReflectRefIter {
    type Item = Result<ReflectReference, InteropError>;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Result<_, _> = {
            match &mut self.index {
                IterationKey::Index(i) => {
                    let mut next = self.base.clone();
                    let parsed_path = ReferencePart::IntegerAccess(*i as i64, false);
                    next.push_path(parsed_path);
                    *i += 1;
                    Ok(next)
                }
            }
        };

        Some(result)
    }
}

#[cfg(test)]
mod test {
    use bevy_ecs::{
        component::Component, reflect::AppTypeRegistry, resource::Resource, world::World,
    };

    use crate::{AppReflectAllocator, function::script_function::AppScriptFunctionRegistry};

    use super::*;

    #[derive(Reflect, Component, Debug, Clone, PartialEq)]
    struct TestComponent(Vec<String>);

    #[derive(Reflect, Resource, Debug, Clone, PartialEq)]
    struct TestResource(Vec<String>);

    fn setup_world() -> World {
        let mut world = World::default();

        let type_registry = AppTypeRegistry::default();
        {
            let mut guard_type_registry = type_registry.write();
            guard_type_registry.register::<TestComponent>();
            guard_type_registry.register::<TestResource>();
        }

        world.insert_resource(type_registry);

        let allocator = AppReflectAllocator::default();
        world.insert_resource(allocator);

        let script_function_registry = AppScriptFunctionRegistry::default();
        world.insert_resource(script_function_registry);

        world
    }

    #[test]
    fn test_component_ref() {
        let mut world = setup_world();

        let entity = world
            .spawn(TestComponent(vec!["hello".to_owned(), "world".to_owned()]))
            .id();

        let world_guard = WorldGuard::new_exclusive(&mut world);

        let mut component_ref =
            ReflectReference::new_component_ref::<TestComponent>(entity, world_guard.clone())
                .expect("could not create component reference");

        // index into component
        assert_eq!(
            component_ref
                .tail_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<TestComponent>()
        );

        component_ref
            .with_reflect(world_guard.clone(), |s| {
                let s = s.try_downcast_ref::<TestComponent>().unwrap();
                assert_eq!(
                    s,
                    &TestComponent(vec!["hello".to_owned(), "world".to_owned()])
                );
            })
            .unwrap();

        // index into vec field
        component_ref.push_path(ReferencePart::IntegerAccess(0, true));
        assert_eq!(
            component_ref
                .tail_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<Vec<String>>()
        );

        assert_eq!(
            component_ref
                .element_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<String>()
        );

        assert_eq!(
            component_ref
                .key_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<usize>()
        );

        component_ref
            .with_reflect(world_guard.clone(), |s| {
                let s = s.try_downcast_ref::<Vec<String>>().unwrap();
                assert_eq!(s, &vec!["hello".to_owned(), "world".to_owned()]);
            })
            .unwrap();

        // index into vec
        component_ref.push_path(ReferencePart::IntegerAccess(0, true));

        component_ref
            .with_reflect(world_guard.clone(), |s| {
                let s = s.try_downcast_ref::<String>().unwrap();
                assert_eq!(s, "hello");
            })
            .unwrap();

        assert_eq!(
            component_ref
                .tail_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<String>()
        );
    }

    #[test]
    fn test_resource_ref() {
        let mut world = setup_world();

        world.insert_resource(TestResource(vec!["hello".to_owned(), "world".to_owned()]));

        let world_guard = WorldGuard::new_exclusive(&mut world);

        let mut resource_ref =
            ReflectReference::new_resource_ref::<TestResource>(world_guard.clone())
                .expect("could not create resource reference");

        // index into resource
        assert_eq!(
            resource_ref
                .tail_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<TestResource>()
        );

        resource_ref
            .with_reflect(world_guard.clone(), |s| {
                let s = s.try_downcast_ref::<TestResource>().unwrap();
                assert_eq!(
                    s,
                    &TestResource(vec!["hello".to_owned(), "world".to_owned()])
                );
            })
            .unwrap();

        // index into vec field
        resource_ref.push_path(ReferencePart::IntegerAccess(0, true));

        assert_eq!(
            resource_ref
                .tail_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<Vec<String>>()
        );

        assert_eq!(
            resource_ref
                .element_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<String>()
        );

        assert_eq!(
            resource_ref
                .key_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<usize>()
        );

        resource_ref
            .with_reflect(world_guard.clone(), |s| {
                let s = s.try_downcast_ref::<Vec<String>>().unwrap();
                assert_eq!(s, &vec!["hello".to_owned(), "world".to_owned()]);
            })
            .unwrap();

        // index into vec
        resource_ref.push_path(ReferencePart::IntegerAccess(0, true));

        resource_ref
            .with_reflect(world_guard.clone(), |s| {
                let s = s.try_downcast_ref::<String>().unwrap();
                assert_eq!(s, "hello");
            })
            .unwrap();

        assert_eq!(
            resource_ref
                .tail_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<String>()
        );
    }

    #[test]
    fn test_allocation_ref() {
        let mut world = setup_world();

        let value: TestComponent = TestComponent(vec!["hello".to_owned(), "world".to_owned()]);

        let world_guard = WorldGuard::new_exclusive(&mut world);
        let allocator = world_guard.allocator();
        let mut allocator_write = allocator.write();
        let mut allocation_ref = ReflectReference::new_allocated(value, &mut allocator_write);
        drop(allocator_write);

        // index into component
        assert_eq!(
            allocation_ref
                .tail_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<TestComponent>()
        );

        allocation_ref
            .with_reflect(world_guard.clone(), |s| {
                let s = s.try_downcast_ref::<TestComponent>().unwrap();
                assert_eq!(
                    s,
                    &TestComponent(vec!["hello".to_owned(), "world".to_owned()])
                );
            })
            .unwrap();

        // index into vec field
        allocation_ref.push_path(ReferencePart::IntegerAccess(0, true));
        assert_eq!(
            allocation_ref
                .tail_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<Vec<String>>()
        );

        assert_eq!(
            allocation_ref
                .element_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<String>()
        );

        assert_eq!(
            allocation_ref
                .key_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<usize>()
        );

        allocation_ref
            .with_reflect(world_guard.clone(), |s| {
                let s = s.try_downcast_ref::<Vec<String>>().unwrap();
                assert_eq!(s, &vec!["hello".to_owned(), "world".to_owned()]);
            })
            .unwrap();

        // index into vec
        allocation_ref.push_path(ReferencePart::IntegerAccess(0, true));

        allocation_ref
            .with_reflect(world_guard.clone(), |s| {
                let s = s.try_downcast_ref::<String>().unwrap();
                assert_eq!(s, "hello");
            })
            .unwrap();

        assert_eq!(
            allocation_ref
                .tail_type_id(world_guard.clone())
                .unwrap()
                .unwrap(),
            TypeId::of::<String>()
        );
    }
}
