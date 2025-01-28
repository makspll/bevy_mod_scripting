//! # Motivation
//!
//! Traits and structs needed to support the creation of bindings for scripting languages.
//! reflection gives us access to `dyn PartialReflect` objects via their type name,
//! Scripting languages only really support `Clone` objects so if we want to support references,
//! we need wrapper types which have owned and ref variants.

use super::{
    access_map::{AccessCount, AccessMap, ReflectAccessId},
    function::{
        namespace::Namespace,
        script_function::{AppScriptFunctionRegistry, DynamicScriptFunction, FunctionCallContext},
    },
    pretty_print::DisplayWithWorld,
    script_value::ScriptValue,
    AppReflectAllocator, ReflectBase, ReflectBaseType, ReflectReference,
    ScriptComponentRegistration, ScriptResourceRegistration, ScriptTypeRegistration,
};
use crate::{error::InteropError, with_access_read, with_access_write, with_global_access};
use bevy::{
    app::AppExit,
    ecs::{
        component::{Component, ComponentId},
        entity::Entity,
        reflect::{AppTypeRegistry, ReflectComponent, ReflectFromWorld, ReflectResource},
        system::{Commands, Resource},
        world::{unsafe_world_cell::UnsafeWorldCell, CommandQueue, Mut, World},
    },
    hierarchy::{BuildChildren, Children, DespawnRecursiveExt, Parent},
    reflect::{std_traits::ReflectDefault, ParsedPath, TypeRegistryArc},
};
use std::{
    any::TypeId,
    borrow::Cow,
    cell::{Cell, RefCell},
    fmt::Debug,
    rc::Rc,
    sync::Arc,
    time::Duration,
};

/// Prefer to directly using [`WorldAccessGuard`]. If the underlying type changes, this alias will be updated.
pub type WorldGuard<'w> = WorldAccessGuard<'w>;
/// Similar to [`WorldGuard`], but without the arc, use for when you don't need the outer Arc.
pub type WorldGuardRef<'w> = &'w WorldAccessGuard<'w>;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);
pub const DEFAULT_INTERVAL: Duration = Duration::from_millis(10);

/// Provides safe access to the world via [`WorldAccess`] permissions, which enforce aliasing rules at runtime in multi-thread environments
#[derive(Clone)]
pub struct WorldAccessGuard<'w>(pub(crate) Rc<WorldAccessGuardInner<'w>>);

/// Used to decrease the stack size of [`WorldAccessGuard`]
pub(crate) struct WorldAccessGuardInner<'w> {
    cell: Cell<Option<UnsafeWorldCell<'w>>>,
    // TODO: this is fairly hefty, explore sparse sets, bit fields etc
    pub(crate) accesses: AccessMap,
    /// Cached for convenience, since we need it for most operations, means we don't need to lock the type registry every time
    type_registry: TypeRegistryArc,
    allocator: AppReflectAllocator,
    function_registry: AppScriptFunctionRegistry,
}

impl WorldAccessGuard<'static> {
    /// Shortens the lifetime of the guard to the given lifetime.
    pub(crate) fn shorten_lifetime<'w>(self) -> WorldGuard<'w> {
        // Safety: todo
        unsafe { std::mem::transmute(self) }
    }
}
#[profiling::all_functions]
impl<'w> WorldAccessGuard<'w> {
    /// Safely allows access to the world for the duration of the closure via a static [`WorldAccessGuard`].
    ///
    /// The guard is invalidated at the end of the closure, meaning the world cannot be accessed at all after the closure ends.
    pub fn with_static_guard<O>(
        world: &'w mut World,
        f: impl FnOnce(WorldGuard<'static>) -> O,
    ) -> O {
        let guard = WorldAccessGuard::new(world);
        // safety: we invalidate the guard after the closure is called, meaning the world cannot be accessed at all after the 'w lifetime ends
        let static_guard: WorldAccessGuard<'static> = unsafe { std::mem::transmute(guard) };
        let o = f(static_guard.clone());

        static_guard.invalidate();
        o
    }

    /// Creates a new [`WorldAccessGuard`] for the given mutable borrow of the world.
    ///
    /// Creating a guard requires that some resources exist in the world, namely:
    /// - [`AppTypeRegistry`]
    /// - [`AppReflectAllocator`]
    /// - [`AppScriptFunctionRegistry`]
    ///
    /// If these resources do not exist, they will be initialized.
    pub fn new(world: &'w mut World) -> Self {
        let type_registry = world.get_resource_or_init::<AppTypeRegistry>().0.clone();

        let allocator = world.get_resource_or_init::<AppReflectAllocator>().clone();

        let function_registry = world
            .get_resource_or_init::<AppScriptFunctionRegistry>()
            .clone();
        let cell = Cell::new(Some(world.as_unsafe_world_cell()));
        Self(Rc::new(WorldAccessGuardInner {
            cell,
            accesses: Default::default(),
            allocator,
            type_registry,
            function_registry,
        }))
    }

    /// Invalidates the world access guard, making it unusable.
    pub fn invalidate(&self) {
        self.0.cell.replace(None);
    }

    /// Begins a new access scope. Currently this simply throws an erorr if there are any accesses held. Should only be used in a single-threaded context
    pub(crate) fn begin_access_scope(&self) -> Result<(), InteropError> {
        if self.0.accesses.count_accesses() != 0 {
            return Err(InteropError::invalid_access_count(self.0.accesses.count_accesses(), 0, "When beginning access scope, presumably for a function call, some accesses are still held".to_owned()));
        }

        Ok(())
    }

    /// Ends the access scope, releasing all accesses. Should only be used in a single-threaded context
    pub(crate) unsafe fn end_access_scope(&self) -> Result<(), InteropError> {
        self.0.accesses.release_all_accesses();

        Ok(())
    }

    /// Purely debugging utility to list all accesses currently held.
    pub fn list_accesses(&self) -> Vec<(ReflectAccessId, AccessCount)> {
        self.0.accesses.list_accesses()
    }

    /// Retrieves the underlying unsafe world cell, with no additional guarantees of safety
    /// proceed with caution and only use this if you understand what you're doing
    pub fn as_unsafe_world_cell(&self) -> Result<UnsafeWorldCell<'w>, InteropError> {
        self.0.cell.get().ok_or_else(InteropError::missing_world)
    }

    /// Retrieves the underlying read only unsafe world cell, with no additional guarantees of safety
    /// proceed with caution and only use this if you understand what you're doing
    pub fn as_unsafe_world_cell_readonly(&self) -> Result<UnsafeWorldCell<'w>, InteropError> {
        self.0.cell.get().ok_or_else(InteropError::missing_world)
    }

    /// Gets the component id of the given component or resource
    pub fn get_component_id(&self, id: TypeId) -> Result<Option<ComponentId>, InteropError> {
        Ok(self
            .as_unsafe_world_cell_readonly()?
            .components()
            .get_id(id))
    }

    pub fn get_resource_id(&self, id: TypeId) -> Result<Option<ComponentId>, InteropError> {
        Ok(self
            .as_unsafe_world_cell_readonly()?
            .components()
            .get_resource_id(id))
    }

    pub fn get_access_location(
        &self,
        raid: ReflectAccessId,
    ) -> Option<std::panic::Location<'static>> {
        self.0.accesses.access_location(raid)
    }

    #[track_caller]
    pub fn claim_read_access(&self, raid: ReflectAccessId) -> bool {
        self.0.accesses.claim_read_access(raid)
    }

    #[track_caller]
    pub fn claim_write_access(&self, raid: ReflectAccessId) -> bool {
        self.0.accesses.claim_write_access(raid)
    }

    /// Releases read or write access to the given type.
    ///
    /// # Safety
    /// - This can only be called safely after all references to the type created using the access have been dropped
    /// - You can only call this if you previously called one of: [`WorldAccessGuard::claim_read_access`] or [`WorldAccessGuard::claim_write_access`]
    /// - The number of claim and release calls for the same id must always match
    pub unsafe fn release_access(&self, raid: ReflectAccessId) {
        self.0.accesses.release_access(raid)
    }

    pub fn claim_global_access(&self) -> bool {
        self.0.accesses.claim_global_access()
    }

    /// Releases global access to the world
    ///
    /// # Safety
    /// - This can only be called safely after all references created using the access have been dropped
    pub unsafe fn release_global_access(&self) {
        self.0.accesses.release_global_access()
    }

    /// Returns the type registry for the world
    pub fn type_registry(&self) -> TypeRegistryArc {
        self.0.type_registry.clone()
    }

    /// Returns the script allocator for the world
    pub fn allocator(&self) -> AppReflectAllocator {
        self.0.allocator.clone()
    }

    /// Returns the function registry for the world
    pub fn script_function_registry(&self) -> AppScriptFunctionRegistry {
        self.0.function_registry.clone()
    }

    /// Claims access to the world for the duration of the closure, allowing for global access to the world.
    #[track_caller]
    pub fn with_global_access<F: FnOnce(&mut World) -> O, O>(
        &self,
        f: F,
    ) -> Result<O, InteropError> {
        with_global_access!(self.0.accesses, "Could not claim exclusive world access", {
            // safety: we have global access for the duration of the closure
            let world = unsafe { self.as_unsafe_world_cell()?.world_mut() };
            Ok(f(world))
        })?
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
            self.0.accesses,
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
            self.0.accesses,
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
            self.0.accesses,
            access_id,
            format!("Could not access component: {}", std::any::type_name::<T>()),
            {
                // Safety: we have acquired access for the duration of the closure
                f(unsafe { cell.get_entity(entity).and_then(|e| e.get::<T>()) })
            }
        )
    }

    /// Safely accesses the component by claiming and releasing access to it.
    pub fn with_component_mut<F, T, O>(&self, entity: Entity, f: F) -> Result<O, InteropError>
    where
        T: Component,
        F: FnOnce(Option<Mut<T>>) -> O,
    {
        let cell = self.as_unsafe_world_cell()?;
        let access_id = ReflectAccessId::for_component::<T>(&cell)?;

        with_access_write!(
            self.0.accesses,
            access_id,
            format!("Could not access component: {}", std::any::type_name::<T>()),
            {
                // Safety: we have acquired access for the duration of the closure
                f(unsafe { cell.get_entity(entity).and_then(|e| e.get_mut::<T>()) })
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
        T: Component + Default,
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
        Ok(cell.get_entity(entity).is_some())
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
            match overload.call(args.clone(), context) {
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
    pub fn spawn(&self) -> Result<Entity, InteropError> {
        self.with_global_access(|world| {
            let entity = world.spawn_empty();
            entity.id()
        })
    }

    pub fn get_type_by_name(&self, type_name: String) -> Option<ScriptTypeRegistration> {
        let type_registry = self.type_registry();
        let type_registry = type_registry.read();
        type_registry
            .get_with_short_type_path(&type_name)
            .or_else(|| type_registry.get_with_type_path(&type_name))
            .map(|registration| ScriptTypeRegistration::new(Arc::new(registration.clone())))
    }

    pub fn get_component_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptComponentRegistration, ScriptTypeRegistration>, InteropError> {
        Ok(match self.get_component_id(registration.type_id())? {
            Some(comp_id) => Ok(ScriptComponentRegistration::new(registration, comp_id)),
            None => Err(registration),
        })
    }

    pub fn get_resource_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptResourceRegistration, ScriptTypeRegistration>, InteropError> {
        Ok(match self.get_resource_id(registration.type_id())? {
            Some(resource_id) => Ok(ScriptResourceRegistration::new(registration, resource_id)),
            None => Err(registration),
        })
    }

    pub fn add_default_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
    ) -> Result<(), InteropError> {
        // let cell = self.as_unsafe_world_cell()?;
        let component_data = registration
            .type_registration()
            .type_registration()
            .data::<ReflectComponent>()
            .ok_or_else(|| {
                InteropError::missing_type_data(
                    registration.registration.type_id(),
                    "ReflectComponent".to_owned(),
                )
            })?;

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

        //  TODO: this shouldn't need entire world access it feels
        self.with_global_access(|world| {
            let type_registry = self.type_registry();

            let mut entity = world
                .get_entity_mut(entity)
                .map_err(|_| InteropError::missing_entity(entity))?;
            {
                let registry = type_registry.read();
                component_data.insert(&mut entity, instance.as_partial_reflect(), &registry);
            }
            Ok(())
        })?
    }

    pub fn insert_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
        value: ReflectReference,
    ) -> Result<(), InteropError> {
        let component_data = registration
            .type_registration()
            .type_registration()
            .data::<ReflectComponent>()
            .ok_or_else(|| {
                InteropError::missing_type_data(
                    registration.registration.type_id(),
                    "ReflectComponent".to_owned(),
                )
            })?;

        with_global_access!(self.0.accesses, "Could not insert element", {
            let cell = self.as_unsafe_world_cell()?;
            let type_registry = self.type_registry();
            let type_registry = type_registry.read();
            let world_mut = unsafe { cell.world_mut() };
            let mut entity = world_mut
                .get_entity_mut(entity)
                .map_err(|_| InteropError::missing_entity(entity))?;

            let ref_ = unsafe { value.reflect_unsafe(self.clone())? };
            component_data.apply_or_insert(&mut entity, ref_, &type_registry);

            Ok(())
        })?
    }

    pub fn get_component(
        &self,
        entity: Entity,
        component_id: ComponentId,
    ) -> Result<Option<ReflectReference>, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        let entity = cell
            .get_entity(entity)
            .ok_or_else(|| InteropError::missing_entity(entity))?;

        let component_info = cell
            .components()
            .get_info(component_id)
            .ok_or_else(|| InteropError::invalid_component(component_id))?;

        if entity.contains_id(component_id) {
            Ok(Some(ReflectReference {
                base: ReflectBaseType {
                    type_id: component_info.type_id().ok_or_else(|| {
                        InteropError::unsupported_operation(
                            None,
                            None,
                            format!(
                                "Component {} does not have a type id. Such components are not supported by BMS.",
                                component_id.display_without_world()
                            ),
                        )
                    })?,
                    base_id: ReflectBase::Component(entity.id(), component_id),
                },
                reflect_path: ParsedPath(vec![]),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn has_component(
        &self,
        entity: Entity,
        component_id: ComponentId,
    ) -> Result<bool, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        let entity = cell
            .get_entity(entity)
            .ok_or_else(|| InteropError::missing_entity(entity))?;

        Ok(entity.contains_id(component_id))
    }

    pub fn remove_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
    ) -> Result<(), InteropError> {
        let component_data = registration
            .type_registration()
            .type_registration()
            .data::<ReflectComponent>()
            .ok_or_else(|| {
                InteropError::missing_type_data(
                    registration.registration.type_id(),
                    "ReflectComponent".to_owned(),
                )
            })?;

        //  TODO: this shouldn't need entire world access it feels
        self.with_global_access(|world| {
            let mut entity = world
                .get_entity_mut(entity)
                .map_err(|_| InteropError::missing_entity(entity))?;
            component_data.remove(&mut entity);
            Ok(())
        })?
    }

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

    pub fn has_resource(&self, resource_id: ComponentId) -> Result<bool, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        // Safety: we are not reading the value at all
        let res_ptr = unsafe { cell.get_resource_by_id(resource_id) };
        Ok(res_ptr.is_some())
    }

    pub fn has_entity(&self, entity: Entity) -> Result<bool, InteropError> {
        self.is_valid_entity(entity)
    }

    pub fn get_children(&self, entity: Entity) -> Result<Vec<Entity>, InteropError> {
        if !self.is_valid_entity(entity)? {
            return Err(InteropError::missing_entity(entity));
        }

        self.with_component(entity, |c: Option<&Children>| {
            c.map(|c| c.to_vec()).unwrap_or_default()
        })
    }

    pub fn get_parent(&self, entity: Entity) -> Result<Option<Entity>, InteropError> {
        if !self.is_valid_entity(entity)? {
            return Err(InteropError::missing_entity(entity));
        }

        self.with_component(entity, |c: Option<&Parent>| c.map(|c| c.get()))
    }

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

    pub fn despawn_recursive(&self, parent: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }
        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).despawn_recursive();
            queue.apply(world);
        })
    }

    pub fn despawn(&self, entity: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(entity)? {
            return Err(InteropError::missing_entity(entity));
        }

        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(entity).despawn();
            queue.apply(world);
        })
    }

    pub fn despawn_descendants(&self, parent: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }

        self.with_global_access(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).despawn_descendants();
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
