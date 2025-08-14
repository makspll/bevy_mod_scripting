//! # Motivation
//!
//! Traits and structs needed to support the creation of bindings for scripting languages.
//! reflection gives us access to `dyn PartialReflect` objects via their type name,
//! Scripting languages only really support `Clone` objects so if we want to support references,
//! we need wrapper types which have owned and ref variants.

use super::{
    access_map::{
        AccessCount, AccessMapKey, AnyAccessMap, DynamicSystemMeta, ReflectAccessId,
        ReflectAccessKind, SubsetAccessMap,
    },
    function::{
        namespace::Namespace,
        script_function::{AppScriptFunctionRegistry, DynamicScriptFunction, FunctionCallContext},
    },
    pretty_print::DisplayWithWorld,
    schedule::AppScheduleRegistry,
    script_value::ScriptValue,
    with_global_access, AppReflectAllocator, AppScriptComponentRegistry, ReflectBase,
    ReflectBaseType, ReflectReference, ScriptComponentRegistration, ScriptResourceRegistration,
    ScriptTypeRegistration, Union,
};
use crate::{
    asset::ScriptAsset,
    bindings::{
        function::{from::FromScript, from_ref::FromScriptRef},
        with_access_read, with_access_write,
    },
    commands::AddStaticScript,
    error::InteropError,
    reflection_extensions::PartialReflectExt,
    script::{ScriptAttachment, ScriptComponent},
};
use bevy::ecs::{component::Mutable, system::Command};
use bevy::prelude::{ChildOf, Children};
use bevy::{
    app::AppExit,
    asset::{AssetServer, Handle, LoadState},
    ecs::{
        component::{Component, ComponentId},
        entity::Entity,
        prelude::Resource,
        reflect::{AppTypeRegistry, ReflectFromWorld, ReflectResource},
        system::Commands,
        world::{unsafe_world_cell::UnsafeWorldCell, CommandQueue, Mut, World},
    },
    reflect::{
        std_traits::ReflectDefault, DynamicEnum, DynamicStruct, DynamicTuple, DynamicTupleStruct,
        DynamicVariant, ParsedPath, PartialReflect, TypeRegistryArc,
    },
};
use bevy_system_reflection::ReflectSchedule;
use std::{
    any::TypeId,
    borrow::Cow,
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    rc::Rc,
    sync::{atomic::AtomicBool, Arc},
};

/// Prefer to directly using [`WorldAccessGuard`]. If the underlying type changes, this alias will be updated.
pub type WorldGuard<'w> = WorldAccessGuard<'w>;
/// Similar to [`WorldGuard`], but without the arc, use for when you don't need the outer Arc.
pub type WorldGuardRef<'w> = &'w WorldAccessGuard<'w>;

/// Provides safe access to the world via [`WorldAccess`] permissions, which enforce aliasing rules at runtime in multi-thread environments
#[derive(Clone, Debug)]
pub struct WorldAccessGuard<'w> {
    /// The guard this guard pointer represents
    pub(crate) inner: Rc<WorldAccessGuardInner<'w>>,
    /// if true the guard is invalid and cannot be used, stored as a second pointer so that this validity can be
    /// stored separate from the contents of the guard
    invalid: Rc<AtomicBool>,
}
/// Used to decrease the stack size of [`WorldAccessGuard`]
pub(crate) struct WorldAccessGuardInner<'w> {
    /// Safety: cannot be used unless the scope depth is less than the max valid scope
    cell: UnsafeWorldCell<'w>,
    // TODO: this is fairly hefty, explore sparse sets, bit fields etc
    pub(crate) accesses: AnyAccessMap,
    /// Cached for convenience, since we need it for most operations, means we don't need to lock the type registry every time
    type_registry: TypeRegistryArc,
    /// The script allocator for the world
    allocator: AppReflectAllocator,
    /// The function registry for the world
    function_registry: AppScriptFunctionRegistry,
    /// The schedule registry for the world
    schedule_registry: AppScheduleRegistry,
    /// The registry of script registered components
    script_component_registry: AppScriptComponentRegistry,
}

impl std::fmt::Debug for WorldAccessGuardInner<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorldAccessGuardInner").finish()
    }
}

#[profiling::all_functions]
impl WorldAccessGuard<'static> {
    /// Shortens the lifetime of the guard to the given lifetime.
    pub(crate) fn shorten_lifetime<'w>(self) -> WorldGuard<'w> {
        // Safety: todo
        unsafe { std::mem::transmute(self) }
    }
}
#[profiling::all_functions]
impl<'w> WorldAccessGuard<'w> {
    /// creates a new guard derived from this one, which if invalidated, will not invalidate the original
    fn scope(&self) -> Self {
        let mut new_guard = self.clone();
        new_guard.invalid = Rc::new(
            new_guard
                .invalid
                .load(std::sync::atomic::Ordering::Relaxed)
                .into(),
        );
        new_guard
    }

    /// Returns true if the guard is valid, false if it is invalid
    fn is_valid(&self) -> bool {
        !self.invalid.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Invalidates the world access guard, making it and any guards derived from this one unusable.
    pub fn invalidate(&self) {
        self.invalid
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Safely allows access to the world for the duration of the closure via a static [`WorldAccessGuard`].
    ///
    /// The guard is invalidated at the end of the closure, meaning the world cannot be accessed at all after the closure ends.
    pub fn with_static_guard<O>(
        world: &'w mut World,
        f: impl FnOnce(WorldGuard<'static>) -> O,
    ) -> O {
        let guard = WorldAccessGuard::new_exclusive(world);
        // safety: we invalidate the guard after the closure is called, meaning the world cannot be accessed at all after the 'w lifetime ends
        let static_guard: WorldAccessGuard<'static> = unsafe { std::mem::transmute(guard) };
        let o = f(static_guard.clone());

        static_guard.invalidate();
        o
    }

    /// Safely allows access to the world for the duration of the closure via a static [`WorldAccessGuard`] using a previously lifetimed world guard.
    /// Will invalidate the static guard at the end but not the original.
    pub fn with_existing_static_guard<O>(
        guard: WorldAccessGuard<'w>,
        f: impl FnOnce(WorldGuard<'static>) -> O,
    ) -> O {
        // safety: we invalidate the guard after the closure is called, meaning the world cannot be accessed at all after the 'w lifetime ends, from the static guard
        // i.e. even if somebody squirells it away, it will be useless.
        let static_guard: WorldAccessGuard<'static> = unsafe { std::mem::transmute(guard.scope()) };
        let o = f(static_guard.clone());
        static_guard.invalidate();
        o
    }

    /// Creates a new [`WorldAccessGuard`] from a possibly non-exclusive access to the world.
    ///
    /// It requires specyfing the exact accesses that are allowed to be given out by the guard.
    /// Those accesses need to be safe to be given out to the script, as the guard will assume that it is safe to give them out in any way.
    ///
    /// # Safety
    /// - The caller must ensure that the accesses in subset are not aliased by any other access
    /// - If an access is allowed in this subset, but alised by someone else,
    /// either by being converted to mutable or non mutable reference, this guard will be unsafe.
    pub unsafe fn new_non_exclusive(
        world: UnsafeWorldCell<'w>,
        subset: impl IntoIterator<Item = ReflectAccessId>,
        type_registry: AppTypeRegistry,
        allocator: AppReflectAllocator,
        function_registry: AppScriptFunctionRegistry,
        schedule_registry: AppScheduleRegistry,
        script_component_registry: AppScriptComponentRegistry,
    ) -> Self {
        Self {
            inner: Rc::new(WorldAccessGuardInner {
                cell: world,
                accesses: AnyAccessMap::SubsetAccessMap(SubsetAccessMap::new(
                    subset,
                    // allocations live beyond the world, and can be safely accessed
                    |id| ReflectAccessId::from_index(id).kind == ReflectAccessKind::Allocation,
                )),
                type_registry: type_registry.0,
                allocator,
                function_registry,
                schedule_registry,
                script_component_registry,
            }),
            invalid: Rc::new(false.into()),
        }
    }

    /// Creates a new [`WorldAccessGuard`] for the given mutable borrow of the world.
    ///
    /// Creating a guard requires that some resources exist in the world, namely:
    /// - [`AppTypeRegistry`]
    /// - [`AppReflectAllocator`]
    /// - [`AppScriptFunctionRegistry`]
    ///
    /// If these resources do not exist, they will be initialized.
    pub fn new_exclusive(world: &'w mut World) -> Self {
        let type_registry = world.get_resource_or_init::<AppTypeRegistry>().0.clone();

        let allocator = world.get_resource_or_init::<AppReflectAllocator>().clone();

        let function_registry = world
            .get_resource_or_init::<AppScriptFunctionRegistry>()
            .clone();

        let script_component_registry = world
            .get_resource_or_init::<AppScriptComponentRegistry>()
            .clone();

        let schedule_registry = world.get_resource_or_init::<AppScheduleRegistry>().clone();
        Self {
            inner: Rc::new(WorldAccessGuardInner {
                cell: world.as_unsafe_world_cell(),
                accesses: AnyAccessMap::UnlimitedAccessMap(Default::default()),
                allocator,
                type_registry,
                function_registry,
                schedule_registry,
                script_component_registry,
            }),
            invalid: Rc::new(false.into()),
        }
    }

    /// Queues a command to the world, which will be executed later.
    pub(crate) fn queue(&self, command: impl Command) -> Result<(), InteropError> {
        self.with_global_access(|w| {
            w.commands().queue(command);
        })
    }

    /// Runs a closure within an isolated access scope, releasing leftover accesses, should only be used in a single-threaded context.
    ///
    /// Safety:
    /// - The caller must ensure it's safe to release any potentially locked accesses.
    pub(crate) unsafe fn with_access_scope<O, F: FnOnce() -> O>(
        &self,
        f: F,
    ) -> Result<O, InteropError> {
        Ok(self.inner.accesses.with_scope(f))
    }

    /// Purely debugging utility to list all accesses currently held.
    pub fn list_accesses(&self) -> Vec<(ReflectAccessId, AccessCount)> {
        self.inner.accesses.list_accesses()
    }

    /// Should only really be used for testing purposes
    pub unsafe fn release_all_accesses(&self) {
        self.inner.accesses.release_all_accesses();
    }

    /// Returns the number of accesses currently held.
    pub fn access_len(&self) -> usize {
        self.inner.accesses.count_accesses()
    }

    /// Retrieves the underlying unsafe world cell, with no additional guarantees of safety
    /// proceed with caution and only use this if you understand what you're doing
    pub fn as_unsafe_world_cell(&self) -> Result<UnsafeWorldCell<'w>, InteropError> {
        if !self.is_valid() {
            return Err(InteropError::missing_world());
        }

        Ok(self.inner.cell)
    }

    /// Retrieves the underlying read only unsafe world cell, with no additional guarantees of safety
    /// proceed with caution and only use this if you understand what you're doing
    pub fn as_unsafe_world_cell_readonly(&self) -> Result<UnsafeWorldCell<'w>, InteropError> {
        if !self.is_valid() {
            return Err(InteropError::missing_world());
        }

        Ok(self.inner.cell)
    }

    /// Gets the component id of the given component or resource
    pub fn get_component_id(&self, id: TypeId) -> Result<Option<ComponentId>, InteropError> {
        Ok(self
            .as_unsafe_world_cell_readonly()?
            .components()
            .get_id(id))
    }

    /// Gets the resource id of the given component or resource
    pub fn get_resource_id(&self, id: TypeId) -> Result<Option<ComponentId>, InteropError> {
        Ok(self
            .as_unsafe_world_cell_readonly()?
            .components()
            .get_resource_id(id))
    }

    /// Get the location of the given access
    pub fn get_access_location(
        &self,
        raid: ReflectAccessId,
    ) -> Option<std::panic::Location<'static>> {
        self.inner.accesses.access_location(raid)
    }

    #[track_caller]
    /// Claims read access to the given type.
    pub fn claim_read_access(&self, raid: ReflectAccessId) -> bool {
        self.inner.accesses.claim_read_access(raid)
    }

    #[track_caller]
    /// Claims write access to the given type.
    pub fn claim_write_access(&self, raid: ReflectAccessId) -> bool {
        self.inner.accesses.claim_write_access(raid)
    }

    /// Releases read or write access to the given type.
    ///
    /// # Safety
    /// - This can only be called safely after all references to the type created using the access have been dropped
    /// - You can only call this if you previously called one of: [`WorldAccessGuard::claim_read_access`] or [`WorldAccessGuard::claim_write_access`]
    /// - The number of claim and release calls for the same id must always match
    pub unsafe fn release_access(&self, raid: ReflectAccessId) {
        self.inner.accesses.release_access(raid)
    }

    /// Claims global access to the world
    pub fn claim_global_access(&self) -> bool {
        self.inner.accesses.claim_global_access()
    }

    /// Releases global access to the world
    ///
    /// # Safety
    /// - This can only be called safely after all references created using the access have been dropped
    pub unsafe fn release_global_access(&self) {
        self.inner.accesses.release_global_access()
    }

    /// Returns the type registry for the world
    pub fn type_registry(&self) -> TypeRegistryArc {
        self.inner.type_registry.clone()
    }

    /// Returns the schedule registry for the world
    pub fn schedule_registry(&self) -> AppScheduleRegistry {
        self.inner.schedule_registry.clone()
    }

    /// Returns the component registry for the world
    pub fn component_registry(&self) -> AppScriptComponentRegistry {
        self.inner.script_component_registry.clone()
    }

    /// Returns the script allocator for the world
    pub fn allocator(&self) -> AppReflectAllocator {
        self.inner.allocator.clone()
    }

    /// Returns the function registry for the world
    pub fn script_function_registry(&self) -> AppScriptFunctionRegistry {
        self.inner.function_registry.clone()
    }

    /// Claims access to the world for the duration of the closure, allowing for global access to the world.
    #[track_caller]
    pub fn with_global_access<F: FnOnce(&mut World) -> O, O>(
        &self,
        f: F,
    ) -> Result<O, InteropError> {
        with_global_access!(
            &self.inner.accesses,
            "Could not claim exclusive world access",
            {
                // safety: we have global access for the duration of the closure
                let world = unsafe { self.as_unsafe_world_cell()?.world_mut() };
                Ok(f(world))
            }
        )?
    }

    /// Safely accesses the resource by claiming and releasing access to it.
    ///
    /// # Panics
    /// - if the resource does not exist
    pub fn with_resource<F, R, O>(&self, f: F) -> Result<O, InteropError>
    where
        R: Resource,
        F: FnOnce(&R) -> O,
    {
        let cell = self.as_unsafe_world_cell()?;
        let access_id = ReflectAccessId::for_resource::<R>(&cell)?;

        with_access_read!(
            &self.inner.accesses,
            access_id,
            format!("Could not access resource: {}", std::any::type_name::<R>()),
            {
                // Safety: we have acquired access for the duration of the closure
                f(unsafe {
                    cell.get_resource::<R>().ok_or_else(|| {
                        InteropError::unregistered_component_or_resource_type(
                            std::any::type_name::<R>(),
                        )
                    })?
                })
            }
        )
    }

    /// Safely accesses the resource by claiming and releasing access to it.
    ///
    /// # Panics
    /// - if the resource does not exist
    pub fn with_resource_mut<F, R, O>(&self, f: F) -> Result<O, InteropError>
    where
        R: Resource,
        F: FnOnce(Mut<R>) -> O,
    {
        let cell = self.as_unsafe_world_cell()?;
        let access_id = ReflectAccessId::for_resource::<R>(&cell)?;
        with_access_write!(
            &self.inner.accesses,
            access_id,
            format!("Could not access resource: {}", std::any::type_name::<R>()),
            {
                // Safety: we have acquired access for the duration of the closure
                f(unsafe {
                    cell.get_resource_mut::<R>().ok_or_else(|| {
                        InteropError::unregistered_component_or_resource_type(
                            std::any::type_name::<R>(),
                        )
                    })?
                })
            }
        )
    }

    /// Safely accesses the component by claiming and releasing access to it.
    pub fn with_component<F, T, O>(&self, entity: Entity, f: F) -> Result<O, InteropError>
    where
        T: Component,
        F: FnOnce(Option<&T>) -> O,
    {
        let cell = self.as_unsafe_world_cell()?;
        let access_id = ReflectAccessId::for_component::<T>(&cell)?;
        with_access_read!(
            &self.inner.accesses,
            access_id,
            format!("Could not access component: {}", std::any::type_name::<T>()),
            {
                // Safety: we have acquired access for the duration of the closure
                f(unsafe { cell.get_entity(entity).map(|e| e.get::<T>()) }
                    .ok()
                    .unwrap_or(None))
            }
        )
    }

    /// Safely accesses the component by claiming and releasing access to it.
    pub fn with_component_mut<F, T, O>(&self, entity: Entity, f: F) -> Result<O, InteropError>
    where
        T: Component<Mutability = Mutable>,
        F: FnOnce(Option<Mut<T>>) -> O,
    {
        let cell = self.as_unsafe_world_cell()?;
        let access_id = ReflectAccessId::for_component::<T>(&cell)?;

        with_access_write!(
            &self.inner.accesses,
            access_id,
            format!("Could not access component: {}", std::any::type_name::<T>()),
            {
                // Safety: we have acquired access for the duration of the closure
                f(unsafe { cell.get_entity(entity).map(|e| e.get_mut::<T>()) }
                    .ok()
                    .unwrap_or(None))
            }
        )
    }

    /// Safey modify or insert a component by claiming and releasing global access.
    pub fn with_or_insert_component_mut<F, T, O>(
        &self,
        entity: Entity,
        f: F,
    ) -> Result<O, InteropError>
    where
        T: Component<Mutability = Mutable> + Default,
        F: FnOnce(&mut T) -> O,
    {
        self.with_global_access(|world| match world.get_mut::<T>(entity) {
            Some(mut component) => f(&mut component),
            None => {
                let mut component = T::default();
                let mut commands = world.commands();
                let result = f(&mut component);
                commands.entity(entity).insert(component);
                result
            }
        })
    }

    /// Try to lookup a function with the given name on the given type id's namespaces.
    ///
    /// Returns the function if found, otherwise returns the name of the function that was not found.
    pub fn lookup_function(
        &self,
        type_ids: impl IntoIterator<Item = TypeId>,
        name: impl Into<Cow<'static, str>>,
    ) -> Result<DynamicScriptFunction, Cow<'static, str>> {
        let registry = self.script_function_registry();
        let registry = registry.read();

        let mut name = name.into();
        for type_id in type_ids {
            name = match registry.get_function(Namespace::OnType(type_id), name) {
                Ok(func) => return Ok(func.clone()),
                Err(name) => name,
            };
        }

        Err(name)
    }

    /// Iterates over all available functions on the type id's namespace + those available on any reference if any exist.
    pub fn get_functions_on_type(
        &self,
        type_id: TypeId,
    ) -> Vec<(Cow<'static, str>, DynamicScriptFunction)> {
        let registry = self.script_function_registry();
        let registry = registry.read();

        registry
            .iter_namespace(Namespace::OnType(type_id))
            .chain(
                registry
                    .iter_namespace(Namespace::OnType(std::any::TypeId::of::<ReflectReference>())),
            )
            .map(|(key, func)| (key.name.clone(), func.clone()))
            .collect()
    }

    /// checks if a given entity exists and is valid
    pub fn is_valid_entity(&self, entity: Entity) -> Result<bool, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        Ok(cell.get_entity(entity).is_ok() && entity.index() != 0)
    }

    /// Tries to call a fitting overload of the function with the given name and in the type id's namespace based on the arguments provided.
    /// Currently does this by repeatedly trying each overload until one succeeds or all fail.
    pub fn try_call_overloads(
        &self,
        type_id: TypeId,
        name: impl Into<Cow<'static, str>>,
        args: Vec<ScriptValue>,
        context: FunctionCallContext,
    ) -> Result<ScriptValue, InteropError> {
        let registry = self.script_function_registry();
        let registry = registry.read();

        let name = name.into();
        let overload_iter = match registry.iter_overloads(Namespace::OnType(type_id), name) {
            Ok(iter) => iter,
            Err(name) => return Err(InteropError::missing_function(type_id, name.to_string())),
        };

        let mut last_error = None;
        for overload in overload_iter {
            match overload.call(args.clone(), context.clone()) {
                Ok(out) => return Ok(out),
                Err(e) => last_error = Some(e),
            }
        }

        Err(last_error.ok_or_else(|| InteropError::invariant("invariant, iterator should always return at least one item, and if the call fails it should return an error"))?)
    }
}

/// Impl block for higher level world methods
#[profiling::all_functions]
impl WorldAccessGuard<'_> {
    fn construct_from_script_value(
        &self,
        descriptor: impl Into<Cow<'static, str>>,
        type_id: TypeId,
        value: Option<ScriptValue>,
    ) -> Result<Box<dyn PartialReflect>, InteropError> {
        // if the value is missing, try to construct a default and return it
        let value = match value {
            Some(value) => value,
            None => {
                let type_registry = self.type_registry();
                let type_registry = type_registry.read();
                let default_data = type_registry
                    .get_type_data::<ReflectDefault>(type_id)
                    .ok_or_else(|| {
                        InteropError::missing_data_in_constructor(type_id, descriptor)
                    })?;
                return Ok(default_data.default().into_partial_reflect());
            }
        };

        // otherwise we need to use from_script_ref
        <Box<dyn PartialReflect>>::from_script_ref(type_id, value, self.clone())
    }

    fn construct_dynamic_struct(
        &self,
        payload: &mut HashMap<String, ScriptValue>,
        fields: Vec<(&'static str, TypeId)>,
    ) -> Result<DynamicStruct, InteropError> {
        let mut dynamic = DynamicStruct::default();
        for (field_name, field_type_id) in fields {
            let constructed = self.construct_from_script_value(
                field_name,
                field_type_id,
                payload.remove(field_name),
            )?;

            dynamic.insert_boxed(field_name, constructed);
        }
        Ok(dynamic)
    }

    fn construct_dynamic_tuple_struct(
        &self,
        payload: &mut HashMap<String, ScriptValue>,
        fields: Vec<TypeId>,
        one_indexed: bool,
    ) -> Result<DynamicTupleStruct, InteropError> {
        let mut dynamic = DynamicTupleStruct::default();
        for (field_idx, field_type_id) in fields.into_iter().enumerate() {
            // correct for indexing
            let script_idx = if one_indexed {
                field_idx + 1
            } else {
                field_idx
            };
            let field_string = format!("_{script_idx}");
            dynamic.insert_boxed(self.construct_from_script_value(
                field_string.clone(),
                field_type_id,
                payload.remove(&field_string),
            )?);
        }
        Ok(dynamic)
    }

    fn construct_dynamic_tuple(
        &self,
        payload: &mut HashMap<String, ScriptValue>,
        fields: Vec<TypeId>,
        one_indexed: bool,
    ) -> Result<DynamicTuple, InteropError> {
        let mut dynamic = DynamicTuple::default();
        for (field_idx, field_type_id) in fields.into_iter().enumerate() {
            // correct for indexing
            let script_idx = if one_indexed {
                field_idx + 1
            } else {
                field_idx
            };

            let field_string = format!("_{script_idx}");

            dynamic.insert_boxed(self.construct_from_script_value(
                field_string.clone(),
                field_type_id,
                payload.remove(&field_string),
            )?);
        }
        Ok(dynamic)
    }

    /// An arbitrary type constructor utility.
    ///
    /// Allows the construction of arbitrary types (within limits dictated by the API) from the script directly
    pub fn construct(
        &self,
        type_: ScriptTypeRegistration,
        mut payload: HashMap<String, ScriptValue>,
        one_indexed: bool,
    ) -> Result<Box<dyn PartialReflect>, InteropError> {
        // figure out the kind of type we're building
        let type_info = type_.registration.type_info();
        // we just need to a) extract fields, if enum we need a "variant" field specifying the variant
        // then build the corresponding dynamic structure, whatever it may be

        let dynamic: Box<dyn PartialReflect> = match type_info {
            bevy::reflect::TypeInfo::Struct(struct_info) => {
                let fields_iter = struct_info
                    .field_names()
                    .iter()
                    .map(|f| {
                        Ok((
                            *f,
                            struct_info
                                .field(f)
                                .ok_or_else(|| {
                                    InteropError::invariant(
                                        "field in field_names should have reflection information",
                                    )
                                })?
                                .type_id(),
                        ))
                    })
                    .collect::<Result<Vec<_>, InteropError>>()?;
                let mut dynamic = self.construct_dynamic_struct(&mut payload, fields_iter)?;
                dynamic.set_represented_type(Some(type_info));
                Box::new(dynamic)
            }
            bevy::reflect::TypeInfo::TupleStruct(tuple_struct_info) => {
                let fields_iter = (0..tuple_struct_info.field_len())
                    .map(|f| {
                        Ok(tuple_struct_info
                            .field_at(f)
                            .ok_or_else(|| {
                                InteropError::invariant(
                                    "field in field_names should have reflection information",
                                )
                            })?
                            .type_id())
                    })
                    .collect::<Result<Vec<_>, InteropError>>()?;

                let mut dynamic =
                    self.construct_dynamic_tuple_struct(&mut payload, fields_iter, one_indexed)?;
                dynamic.set_represented_type(Some(type_info));
                Box::new(dynamic)
            }
            bevy::reflect::TypeInfo::Tuple(tuple_info) => {
                let fields_iter = (0..tuple_info.field_len())
                    .map(|f| {
                        Ok(tuple_info
                            .field_at(f)
                            .ok_or_else(|| {
                                InteropError::invariant(
                                    "field in field_names should have reflection information",
                                )
                            })?
                            .type_id())
                    })
                    .collect::<Result<Vec<_>, InteropError>>()?;

                let mut dynamic =
                    self.construct_dynamic_tuple(&mut payload, fields_iter, one_indexed)?;
                dynamic.set_represented_type(Some(type_info));
                Box::new(dynamic)
            }
            bevy::reflect::TypeInfo::Enum(enum_info) => {
                // extract variant from "variant"
                let variant = payload.remove("variant").ok_or_else(|| {
                    InteropError::missing_data_in_constructor(
                        enum_info.type_id(),
                        "\"variant\" field for enum",
                    )
                })?;

                let variant_name = String::from_script(variant, self.clone())?;

                let variant = enum_info.variant(&variant_name).ok_or_else(|| {
                    InteropError::invalid_enum_variant(enum_info.type_id(), variant_name.clone())
                })?;

                let variant = match variant {
                    bevy::reflect::VariantInfo::Struct(struct_variant_info) => {
                        // same as above struct variant
                        let fields_iter = struct_variant_info
                            .field_names()
                            .iter()
                            .map(|f| {
                                Ok((
                                    *f,
                                    struct_variant_info
                                        .field(f)
                                        .ok_or_else(|| {
                                            InteropError::invariant(
                                                "field in field_names should have reflection information",
                                            )
                                        })?
                                        .type_id(),
                                ))
                            })
                            .collect::<Result<Vec<_>, InteropError>>()?;

                        let dynamic = self.construct_dynamic_struct(&mut payload, fields_iter)?;
                        DynamicVariant::Struct(dynamic)
                    }
                    bevy::reflect::VariantInfo::Tuple(tuple_variant_info) => {
                        // same as tuple variant
                        let fields_iter = (0..tuple_variant_info.field_len())
                            .map(|f| {
                                Ok(tuple_variant_info
                                    .field_at(f)
                                    .ok_or_else(|| {
                                        InteropError::invariant(
                                            "field in field_names should have reflection information",
                                        )
                                    })?
                                    .type_id())
                            })
                            .collect::<Result<Vec<_>, InteropError>>()?;

                        let dynamic =
                            self.construct_dynamic_tuple(&mut payload, fields_iter, one_indexed)?;
                        DynamicVariant::Tuple(dynamic)
                    }
                    bevy::reflect::VariantInfo::Unit(_) => DynamicVariant::Unit,
                };
                let mut dynamic = DynamicEnum::new(variant_name, variant);
                dynamic.set_represented_type(Some(type_info));
                Box::new(dynamic)
            }
            _ => {
                return Err(InteropError::unsupported_operation(
                    Some(type_info.type_id()),
                    Some(Box::new(payload)),
                    "Type constructor not supported",
                ))
            }
        };

        // try to construct type from reflect
        // TODO: it would be nice to have a <dyn PartialReflect>::from_reflect_with_fallback equivalent, that does exactly that
        // only using this as it's already there and convenient, the clone variant hitting will be confusing to end users
        <dyn PartialReflect>::from_reflect_or_clone(dynamic.as_ref(), self.clone())
    }

    /// Loads a script from the given asset path with default settings.
    pub fn load_script_asset(&self, asset_path: &str) -> Result<Handle<ScriptAsset>, InteropError> {
        self.with_resource(|r: &AssetServer| r.load(asset_path))
    }

    /// Checks the load state of a script asset.
    pub fn get_script_asset_load_state(
        &self,
        script: Handle<ScriptAsset>,
    ) -> Result<LoadState, InteropError> {
        self.with_resource(|r: &AssetServer| r.load_state(script.id()))
    }

    /// Attaches a script
    pub fn attach_script(&self, attachment: ScriptAttachment) -> Result<(), InteropError> {
        match attachment {
            ScriptAttachment::EntityScript(entity, handle) => {
                // find existing script components on the entity
                self.with_or_insert_component_mut(entity, |c: &mut ScriptComponent| {
                    c.0.push(handle.clone())
                })?;
            }
            ScriptAttachment::StaticScript(handle) => {
                self.queue(AddStaticScript::new(handle))?;
            }
        };

        Ok(())
    }

    /// Spawns a new entity in the world
    pub fn spawn(&self) -> Result<Entity, InteropError> {
        self.with_global_access(|world| {
            let mut command_queue = CommandQueue::default();
            let mut commands = Commands::new(&mut command_queue, world);
            let id = commands.spawn_empty().id();
            command_queue.apply(world);
            id
        })
    }

    /// get a type registration for the type, without checking if it's a component or resource
    pub fn get_type_by_name(&self, type_name: &str) -> Option<ScriptTypeRegistration> {
        let type_registry = self.type_registry();
        let type_registry = type_registry.read();
        type_registry
            .get_with_short_type_path(type_name)
            .or_else(|| type_registry.get_with_type_path(type_name))
            .map(|registration| ScriptTypeRegistration::new(Arc::new(registration.clone())))
    }

    /// get a type erased type registration for the type including information about whether it's a component or resource
    pub(crate) fn get_type_registration(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<
        Union<
            ScriptTypeRegistration,
            Union<ScriptComponentRegistration, ScriptResourceRegistration>,
        >,
        InteropError,
    > {
        let registration = match self.get_resource_type(registration)? {
            Ok(res) => {
                return Ok(Union::new_right(Union::new_right(res)));
            }
            Err(registration) => registration,
        };

        let registration = match self.get_component_type(registration)? {
            Ok(comp) => {
                return Ok(Union::new_right(Union::new_left(comp)));
            }
            Err(registration) => registration,
        };

        Ok(Union::new_left(registration))
    }

    /// Similar to [`Self::get_type_by_name`] but returns a type erased [`ScriptTypeRegistration`], [`ScriptComponentRegistration`] or [`ScriptResourceRegistration`]
    /// depending on the underlying type and state of the world.
    pub fn get_type_registration_by_name(
        &self,
        type_name: String,
    ) -> Result<
        Option<
            Union<
                ScriptTypeRegistration,
                Union<ScriptComponentRegistration, ScriptResourceRegistration>,
            >,
        >,
        InteropError,
    > {
        let val = self.get_type_by_name(&type_name);
        Ok(match val {
            Some(registration) => Some(self.get_type_registration(registration)?),
            None => {
                // try the component registry
                let components = self.component_registry();
                let components = components.read();
                components
                    .get(&type_name)
                    .map(|c| Union::new_right(Union::new_left(c.registration.clone())))
            }
        })
    }

    /// get a schedule by name
    pub fn get_schedule_by_name(&self, schedule_name: String) -> Option<ReflectSchedule> {
        let schedule_registry = self.schedule_registry();
        let schedule_registry = schedule_registry.read();

        schedule_registry
            .get_schedule_by_name(&schedule_name)
            .cloned()
    }

    /// get a component type registration for the type
    pub fn get_component_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptComponentRegistration, ScriptTypeRegistration>, InteropError> {
        Ok(match self.get_component_id(registration.type_id())? {
            Some(comp_id) => Ok(ScriptComponentRegistration::new(registration, comp_id)),
            None => Err(registration),
        })
    }

    /// get a resource type registration for the type
    pub fn get_resource_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptResourceRegistration, ScriptTypeRegistration>, InteropError> {
        Ok(match self.get_resource_id(registration.type_id())? {
            Some(resource_id) => Ok(ScriptResourceRegistration::new(registration, resource_id)),
            None => Err(registration),
        })
    }

    /// add a default component to an entity
    pub fn add_default_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
    ) -> Result<(), InteropError> {
        // we look for ReflectDefault or ReflectFromWorld data then a ReflectComponent data
        let instance = if let Some(default_td) = registration
            .type_registration()
            .type_registration()
            .data::<ReflectDefault>()
        {
            default_td.default()
        } else if let Some(from_world_td) = registration
            .type_registration()
            .type_registration()
            .data::<ReflectFromWorld>()
        {
            self.with_global_access(|world| from_world_td.from_world(world))?
        } else {
            return Err(InteropError::missing_type_data(
                registration.registration.type_id(),
                "ReflectDefault or ReflectFromWorld".to_owned(),
            ));
        };

        registration.insert_into_entity(self.clone(), entity, instance)
    }

    /// insert the component into the entity
    pub fn insert_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
        value: ReflectReference,
    ) -> Result<(), InteropError> {
        let instance = <Box<dyn PartialReflect>>::from_script_ref(
            registration.type_registration().type_id(),
            ScriptValue::Reference(value),
            self.clone(),
        )?;

        let reflect = instance.try_into_reflect().map_err(|v| {
            InteropError::failed_from_reflect(
                Some(registration.type_registration().type_id()),
                format!("instance produced by conversion to target type when inserting component is not a full reflect type: {v:?}"),
            )
        })?;

        registration.insert_into_entity(self.clone(), entity, reflect)
    }

    /// get the component from the entity
    pub fn get_component(
        &self,
        entity: Entity,
        component_registration: ScriptComponentRegistration,
    ) -> Result<Option<ReflectReference>, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        let entity = cell
            .get_entity(entity)
            .map_err(|_| InteropError::missing_entity(entity))?;

        if entity.contains_id(component_registration.component_id) {
            Ok(Some(ReflectReference {
                base: ReflectBaseType {
                    type_id: component_registration.type_registration().type_id(),
                    base_id: ReflectBase::Component(
                        entity.id(),
                        component_registration.component_id,
                    ),
                },
                reflect_path: ParsedPath(vec![]),
            }))
        } else {
            Ok(None)
        }
    }

    /// check if the entity has the component
    pub fn has_component(
        &self,
        entity: Entity,
        component_id: ComponentId,
    ) -> Result<bool, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        let entity = cell
            .get_entity(entity)
            .map_err(|_| InteropError::missing_entity(entity))?;

        Ok(entity.contains_id(component_id))
    }

    /// remove the component from the entity
    pub fn remove_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
    ) -> Result<(), InteropError> {
        registration.remove_from_entity(self.clone(), entity)
    }

    /// get the given resource
    pub fn get_resource(
        &self,
        resource_id: ComponentId,
    ) -> Result<Option<ReflectReference>, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        let component_info = match cell.components().get_info(resource_id) {
            Some(info) => info,
            None => return Ok(None),
        };

        Ok(Some(ReflectReference {
            base: ReflectBaseType {
                type_id: component_info
                    .type_id()
                    .ok_or_else(|| {
                        InteropError::unsupported_operation(
                            None,
                            None,
                            format!(
                                "Resource {} does not have a type id. Such resources are not supported by BMS.",
                                resource_id.display_without_world()
                            ),
                        )
                    })?,
                base_id: ReflectBase::Resource(resource_id),
            },
            reflect_path: ParsedPath(vec![]),
        }))
    }

    /// remove the given resource
    pub fn remove_resource(
        &self,
        registration: ScriptResourceRegistration,
    ) -> Result<(), InteropError> {
        let component_data = registration
            .type_registration()
            .type_registration()
            .data::<ReflectResource>()
            .ok_or_else(|| {
                InteropError::missing_type_data(
                    registration.registration.type_id(),
                    "ReflectResource".to_owned(),
                )
            })?;

        //  TODO: this shouldn't need entire world access it feels
        self.with_global_access(|world| component_data.remove(world))
    }

    /// check if the entity has the resource
    pub fn has_resource(&self, resource_id: ComponentId) -> Result<bool, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        // Safety: we are not reading the value at all
        let res_ptr = unsafe { cell.get_resource_by_id(resource_id) };
        Ok(res_ptr.is_some())
    }

    /// check the given entity exists
    pub fn has_entity(&self, entity: Entity) -> Result<bool, InteropError> {
        self.is_valid_entity(entity)
    }

    /// get the children of the given entity
    pub fn get_children(&self, entity: Entity) -> Result<Vec<Entity>, InteropError> {
        if !self.is_valid_entity(entity)? {
            return Err(InteropError::missing_entity(entity));
        }

        self.with_component(entity, |c: Option<&Children>| {
            c.map(|c| c.to_vec()).unwrap_or_default()
        })
    }

    /// get the parent of the given entity
    pub fn get_parent(&self, entity: Entity) -> Result<Option<Entity>, InteropError> {
        if !self.is_valid_entity(entity)? {
            return Err(InteropError::missing_entity(entity));
        }

        self.with_component(entity, |c: Option<&ChildOf>| c.map(|c| c.parent()))
    }

    /// insert children into the given entity
    pub fn push_children(&self, parent: Entity, children: &[Entity]) -> Result<(), InteropError> {
        // verify entities exist
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }
        for c in children {
            if !self.is_valid_entity(*c)? {
                return Err(InteropError::missing_entity(*c));
            }
        }
        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).add_children(children);
            queue.apply(world);
        })
    }

    /// remove children from the given entity
    pub fn remove_children(&self, parent: Entity, children: &[Entity]) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }

        for c in children {
            if !self.is_valid_entity(*c)? {
                return Err(InteropError::missing_entity(*c));
            }
        }
        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).remove_children(children);
            queue.apply(world);
        })
    }

    /// insert children into the given entity at the given index
    pub fn insert_children(
        &self,
        parent: Entity,
        index: usize,
        children: &[Entity],
    ) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }

        for c in children {
            if !self.is_valid_entity(*c)? {
                return Err(InteropError::missing_entity(*c));
            }
        }

        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).insert_children(index, children);
            queue.apply(world);
        })
    }

    /// despawn this and all children of the given entity recursively
    pub fn despawn_recursive(&self, parent: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }
        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).despawn();
            queue.apply(world);
        })
    }

    /// despawn the given entity
    pub fn despawn(&self, entity: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(entity)? {
            return Err(InteropError::missing_entity(entity));
        }

        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(entity).remove::<Children>().despawn();
            queue.apply(world);
        })
    }

    /// despawn all children of the given entity recursively
    pub fn despawn_descendants(&self, parent: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }

        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).despawn_related::<Children>();
            queue.apply(world);
        })
    }

    /// Sends AppExit event to the world with success status
    pub fn exit(&self) -> Result<(), InteropError> {
        self.with_global_access(|world| {
            world.send_event(AppExit::Success);
        })
    }
}

/// Utility type for accessing the world in a callback
pub trait WorldContainer {
    /// The error type for the container
    type Error: Debug;
    /// Sets the world to the given value in the container
    fn set_world(&mut self, world: WorldGuard<'static>) -> Result<(), Self::Error>;

    /// Tries to get the world from the container
    fn try_get_world<'l>(&self) -> Result<WorldGuard<'l>, Self::Error>;
}

/// A world container that stores the world in a thread local
pub struct ThreadWorldContainer;

thread_local! {
    static WORLD_CALLBACK_ACCESS: RefCell<Option<WorldGuard<'static>>> = const { RefCell::new(None) };
}
#[profiling::all_functions]
impl WorldContainer for ThreadWorldContainer {
    type Error = InteropError;

    fn set_world(&mut self, world: WorldGuard<'static>) -> Result<(), Self::Error> {
        WORLD_CALLBACK_ACCESS.with(|w| {
            w.replace(Some(world));
        });
        Ok(())
    }

    fn try_get_world<'l>(&self) -> Result<WorldGuard<'l>, Self::Error> {
        WORLD_CALLBACK_ACCESS
            .with(|w| w.borrow().clone().ok_or_else(InteropError::missing_world))
            .map(|v| v.shorten_lifetime())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy::reflect::{GetTypeRegistration, ReflectFromReflect};
    use test_utils::test_data::{setup_world, SimpleEnum, SimpleStruct, SimpleTupleStruct};

    #[test]
    fn test_construct_struct() {
        let mut world = setup_world(|_, _| {});
        let world = WorldAccessGuard::new_exclusive(&mut world);

        let registry = world.type_registry();
        let registry = registry.read();

        let registration = registry.get(TypeId::of::<SimpleStruct>()).unwrap().clone();
        let type_registration = ScriptTypeRegistration::new(Arc::new(registration));

        let payload = HashMap::from_iter(vec![("foo".to_owned(), ScriptValue::Integer(1))]);

        let result = world.construct(type_registration, payload, false);
        let expected =
            Ok::<_, InteropError>(Box::new(SimpleStruct { foo: 1 }) as Box<dyn PartialReflect>);
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));
    }

    #[test]
    fn test_construct_tuple_struct() {
        let mut world = setup_world(|_, _| {});
        let world = WorldAccessGuard::new_exclusive(&mut world);

        let registry = world.type_registry();
        let registry = registry.read();

        let registration = registry
            .get(TypeId::of::<SimpleTupleStruct>())
            .unwrap()
            .clone();
        let type_registration = ScriptTypeRegistration::new(Arc::new(registration));

        // zero indexed
        let payload = HashMap::from_iter(vec![("_0".to_owned(), ScriptValue::Integer(1))]);

        let result = world.construct(type_registration.clone(), payload, false);
        let expected =
            Ok::<_, InteropError>(Box::new(SimpleTupleStruct(1)) as Box<dyn PartialReflect>);
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));

        // one indexed
        let payload = HashMap::from_iter(vec![("_1".to_owned(), ScriptValue::Integer(1))]);

        let result = world.construct(type_registration, payload, true);
        let expected =
            Ok::<_, InteropError>(Box::new(SimpleTupleStruct(1)) as Box<dyn PartialReflect>);

        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));
    }

    #[test]
    fn test_construct_tuple() {
        let mut world = setup_world(|_, registry| {
            registry.register::<(usize, usize)>();
            // TODO: does this ever get registered on normal types? I don't think so: https://github.com/bevyengine/bevy/issues/17981
            registry.register_type_data::<(usize, usize), ReflectFromReflect>();
        });

        <usize as GetTypeRegistration>::get_type_registration();
        let world = WorldAccessGuard::new_exclusive(&mut world);

        let registry = world.type_registry();
        let registry = registry.read();

        let registration = registry
            .get(TypeId::of::<(usize, usize)>())
            .unwrap()
            .clone();
        let type_registration = ScriptTypeRegistration::new(Arc::new(registration));

        // zero indexed
        let payload = HashMap::from_iter(vec![
            ("_0".to_owned(), ScriptValue::Integer(1)),
            ("_1".to_owned(), ScriptValue::Integer(2)),
        ]);

        let result = world.construct(type_registration.clone(), payload, false);
        let expected = Ok::<_, InteropError>(Box::new((1, 2)) as Box<dyn PartialReflect>);
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));

        // one indexed
        let payload = HashMap::from_iter(vec![
            ("_1".to_owned(), ScriptValue::Integer(1)),
            ("_2".to_owned(), ScriptValue::Integer(2)),
        ]);

        let result = world.construct(type_registration.clone(), payload, true);
        let expected = Ok::<_, InteropError>(Box::new((1, 2)) as Box<dyn PartialReflect>);
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));
    }

    #[test]
    fn test_construct_enum() {
        let mut world = setup_world(|_, _| {});
        let world = WorldAccessGuard::new_exclusive(&mut world);

        let registry = world.type_registry();
        let registry = registry.read();

        let registration = registry.get(TypeId::of::<SimpleEnum>()).unwrap().clone();
        let type_registration = ScriptTypeRegistration::new(Arc::new(registration));

        // struct version
        let payload = HashMap::from_iter(vec![
            ("foo".to_owned(), ScriptValue::Integer(1)),
            ("variant".to_owned(), ScriptValue::String("Struct".into())),
        ]);

        let result = world.construct(type_registration.clone(), payload, false);
        let expected = Ok::<_, InteropError>(
            Box::new(SimpleEnum::Struct { foo: 1 }) as Box<dyn PartialReflect>
        );
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));

        // tuple struct version
        let payload = HashMap::from_iter(vec![
            ("_0".to_owned(), ScriptValue::Integer(1)),
            (
                "variant".to_owned(),
                ScriptValue::String("TupleStruct".into()),
            ),
        ]);

        let result = world.construct(type_registration.clone(), payload, false);
        let expected =
            Ok::<_, InteropError>(Box::new(SimpleEnum::TupleStruct(1)) as Box<dyn PartialReflect>);

        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));

        // unit version
        let payload = HashMap::from_iter(vec![(
            "variant".to_owned(),
            ScriptValue::String("Unit".into()),
        )]);

        let result = world.construct(type_registration, payload, false);
        let expected = Ok::<_, InteropError>(Box::new(SimpleEnum::Unit) as Box<dyn PartialReflect>);
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));
    }

    #[test]
    fn test_scoped_handle_invalidate_doesnt_invalidate_parent() {
        let mut world = setup_world(|_, _| {});
        let world = WorldAccessGuard::new_exclusive(&mut world);
        let scoped_world = world.scope();

        // can use scoped & normal worlds
        scoped_world.spawn().unwrap();
        world.spawn().unwrap();
        pretty_assertions::assert_eq!(scoped_world.is_valid(), true);
        pretty_assertions::assert_eq!(world.is_valid(), true);

        scoped_world.invalidate();

        // can only use normal world
        pretty_assertions::assert_eq!(scoped_world.is_valid(), false);
        pretty_assertions::assert_eq!(world.is_valid(), true);
        world.spawn().unwrap();
    }

    #[test]
    fn with_existing_static_guard_does_not_invalidate_original() {
        let mut world = setup_world(|_, _| {});
        let world = WorldAccessGuard::new_exclusive(&mut world);

        let mut sneaky_clone = None;
        WorldAccessGuard::with_existing_static_guard(world.clone(), |g| {
            pretty_assertions::assert_eq!(g.is_valid(), true);
            sneaky_clone = Some(g.clone());
        });
        pretty_assertions::assert_eq!(world.is_valid(), true, "original world was invalidated");
        pretty_assertions::assert_eq!(
            sneaky_clone.map(|c| c.is_valid()),
            Some(false),
            "scoped world was not invalidated"
        );
    }

    #[test]
    fn test_with_access_scope_success() {
        let mut world = setup_world(|_, _| {});
        let guard = WorldAccessGuard::new_exclusive(&mut world);

        // within the access scope, no extra accesses are claimed
        let result = unsafe { guard.with_access_scope(|| 100) };
        assert_eq!(result.unwrap(), 100);
    }
}
