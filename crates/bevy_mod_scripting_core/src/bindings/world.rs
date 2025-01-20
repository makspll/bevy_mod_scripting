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

// /// While [`WorldAccessGuard`] prevents aliasing at runtime and also makes sure world exists at least as long as the guard itself,
// /// borrows sadly do not persist the script-host boundary :(. That is to be expected, but instead we can make an abstraction which removes the lifetime parameter, making the outer type 'static,
// /// while making sure the lifetime is still satisfied!
// #[derive(Clone, Debug)]
// #[reflect(opaque)]
// pub struct WorldCallbackAccess(pub(crate) Weak<WorldAccessGuard<'static>>);

// impl WorldCallbackAccess {
//     /// Wraps a callback which requires access to the world in a 'static way via [`WorldCallbackAccess`].
//     pub fn with_callback_access<T>(
//         world: &mut World,
//         callback: impl FnOnce(&WorldCallbackAccess) -> T,
//     ) -> T {
//         // - the world cannot be dropped before the world drops since we have mutable reference to it in this entire function
//         // - nothing can alias inappropriately WorldAccessGuard since it's only instance is behind the raw Arc
//         let world_guard_arc = Arc::new(WorldAccessGuard::new(world));
//         let world_guard = unsafe { WorldCallbackAccess::new(Arc::downgrade(&world_guard_arc)) };
//         callback(&world_guard)
//     }

//     /// Creates a new [`WorldCallbackAccess`] with an erased lifetime.
//     ///
//     /// For safe alternative see [`Self::from_guard`]
//     ///
//     /// # Safety
//     /// - The caller must ensure the [`WorldAccessGuard`] will not outlive the 'w lifetime
//     /// - In practice this means that between the moment the original Arc is dropped, the lifetime 'w must be valid
//     /// - I.e. you *must* drop the original [`Arc<WorldAccessGuard>`] before the original 'w scope ends
//     pub unsafe fn new<'w>(world: Weak<WorldAccessGuard<'w>>) -> Self {
//         // Safety: the caller ensures `WorldAccessGuard` does not outlive the original lifetime 'w

//         let world = unsafe {
//             std::mem::transmute::<Weak<WorldAccessGuard<'w>>, Weak<WorldAccessGuard<'static>>>(
//                 world,
//             )
//         };

//         Self(world)
//     }

//     // pub fn from_guard(world: WorldGuard<'_>) -> Self {
//     //     // Safety: the caller ensures `WorldAccessGuard` does not outlive the original lifetime 'w
//     //     unsafe { Self::new(Arc::new(&world)) }
//     // }

//     /// Attempts to read the world access guard, if it still exists
//     pub fn try_read(&self) -> Result<WorldGuard<'static>, InteropError> {
//         self.0
//             .upgrade()
//             .map()
//             .ok_or_else(InteropError::stale_world_access)
//     }
// }

// /// common world methods, see:
// /// - [`crate::bindings::query`] for query related functionality
// impl WorldCallbackAccess {
//     pub fn spawn(&self) -> Result<Entity, InteropError> {
//         let world = self.try_read()?;
//         Ok(world.spawn())
//     }

//     // TODO: uses `String` for type_name to avoid lifetime issues with types proxying this via macros
//     pub fn get_type_by_name(
//         &self,
//         type_name: String,
//     ) -> Result<Option<ScriptTypeRegistration>, InteropError> {
//         let world = self.try_read()?;
//         Ok(world.get_type_by_name(type_name))
//     }

//     pub fn get_component_type(
//         &self,
//         registration: ScriptTypeRegistration,
//     ) -> Result<Result<ScriptComponentRegistration, ScriptTypeRegistration>, InteropError> {
//         let world = self.try_read()?;
//         Ok(world.get_component_type(registration))
//     }

//     pub fn get_resource_type(
//         &self,
//         registration: ScriptTypeRegistration,
//     ) -> Result<Result<ScriptResourceRegistration, ScriptTypeRegistration>, InteropError> {
//         let world = self.try_read()?;
//         Ok(world.get_resource_type(registration))
//     }

//     pub fn add_default_component(
//         &self,
//         entity: Entity,
//         registration: ScriptComponentRegistration,
//     ) -> Result<(), InteropError> {
//         let world = self.try_read()?;
//         world.add_default_component(entity, registration)
//     }

//     pub fn get_component(
//         &self,
//         entity: Entity,
//         component_id: ComponentId,
//     ) -> Result<Option<ReflectReference>, InteropError> {
//         let world = self.try_read()?;
//         world.get_component(entity, component_id)
//     }

//     pub fn has_component(
//         &self,
//         entity: Entity,
//         component_id: ComponentId,
//     ) -> Result<bool, InteropError> {
//         let world = self.try_read()?;
//         world.has_component(entity, component_id)
//     }

//     pub fn remove_component(
//         &self,
//         entity: Entity,
//         registration: ScriptComponentRegistration,
//     ) -> Result<(), InteropError> {
//         let world = self.try_read()?;
//         world.remove_component(entity, registration)
//     }

//     pub fn get_resource(
//         &self,
//         resource_id: ComponentId,
//     ) -> Result<Option<ReflectReference>, InteropError> {
//         let world = self.try_read()?;
//         world.get_resource(resource_id)
//     }

//     pub fn remove_resource(
//         &self,
//         registration: ScriptResourceRegistration,
//     ) -> Result<(), InteropError> {
//         let world = self.try_read()?;
//         world.remove_resource(registration)
//     }

//     pub fn has_resource(&self, resource_id: ComponentId) -> Result<bool, InteropError> {
//         let world = self.try_read()?;
//         Ok(world.has_resource(resource_id)?)
//     }

//     pub fn has_entity(&self, entity: Entity) -> Result<bool, InteropError> {
//         let world = self.try_read()?;
//         Ok(world.has_entity(entity)?)
//     }

//     pub fn get_children(&self, entity: Entity) -> Result<Vec<Entity>, InteropError> {
//         let world = self.try_read()?;
//         world.get_children(entity)
//     }

//     pub fn get_parent(&self, entity: Entity) -> Result<Option<Entity>, InteropError> {
//         let world = self.try_read()?;
//         world.get_parent(entity)
//     }

//     pub fn push_children(&self, parent: Entity, children: &[Entity]) -> Result<(), InteropError> {
//         let world = self.try_read()?;
//         world.push_children(parent, children)
//     }

//     pub fn remove_children(&self, parent: Entity, children: &[Entity]) -> Result<(), InteropError> {
//         let world = self.try_read()?;
//         world.remove_children(parent, children)
//     }

//     pub fn insert_children(
//         &self,
//         parent: Entity,
//         index: usize,
//         children: &[Entity],
//     ) -> Result<(), InteropError> {
//         let world = self.try_read()?;
//         world.insert_children(parent, index, children)
//     }

//     pub fn despawn_recursive(&self, entity: Entity) -> Result<(), InteropError> {
//         let world = self.try_read()?;
//         world.despawn_recursive(entity)
//     }

//     pub fn despawn(&self, entity: Entity) -> Result<(), InteropError> {
//         let world = self.try_read()?;
//         world.despawn(entity)
//     }

//     pub fn despawn_descendants(&self, entity: Entity) -> Result<(), InteropError> {
//         let world = self.try_read()?;
//         world.despawn_descendants(entity)
//     }

//     pub fn exit(&self) -> Result<(), InteropError> {
//         let world = self.try_read()?;
//         world.exit();
//         Ok(())
//     }

// }

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
    ///
    /// # Panics
    /// - if the component does not exist
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
    ///
    /// # Panics
    /// - if the component does not exist
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
impl WorldAccessGuard<'_> {
    pub fn spawn(&self) -> Result<Entity, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        with_global_access!(self.0.accesses, "Could not spawn entity", {
            // Safety we have global access
            let entity = unsafe { cell.world_mut().spawn_empty() };
            Ok(entity.id())
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
        let cell = self.as_unsafe_world_cell()?;
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
            with_global_access!(self.0.accesses, "Could not add default component", {
                let world = unsafe { cell.world_mut() };
                from_world_td.from_world(world)
            })
        } else {
            return Err(InteropError::missing_type_data(
                registration.registration.type_id(),
                "ReflectDefault or ReflectFromWorld".to_owned(),
            ));
        };

        //  TODO: this shouldn't need entire world access it feels
        with_global_access!(self.0.accesses, "Could not add default component", {
            let type_registry = self.type_registry();
            let world = unsafe { cell.world_mut() };

            let mut entity = world
                .get_entity_mut(entity)
                .map_err(|_| InteropError::missing_entity(entity))?;
            {
                let registry = type_registry.read();
                component_data.insert(&mut entity, instance.as_partial_reflect(), &registry);
            }
            Ok(())
        })
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
        let cell = self.as_unsafe_world_cell()?;
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
        with_global_access!(self.0.accesses, "Could not remove component", {
            let world = unsafe { cell.world_mut() };
            let mut entity = world
                .get_entity_mut(entity)
                .map_err(|_| InteropError::missing_entity(entity))?;
            component_data.remove(&mut entity);
            Ok(())
        })
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
        let cell = self.as_unsafe_world_cell()?;
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
        with_global_access!(self.0.accesses, "Could not remove resource", {
            let world = unsafe { cell.world_mut() };
            component_data.remove(world);
            Ok(())
        })
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
        let cell = self.as_unsafe_world_cell()?;
        with_global_access!(self.0.accesses, "Could not push children", {
            let world = unsafe { cell.world_mut() };
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).add_children(children);
            queue.apply(world);
        });

        Ok(())
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
        let cell = self.as_unsafe_world_cell()?;

        with_global_access!(self.0.accesses, "Could not remove children", {
            let world = unsafe { cell.world_mut() };
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).remove_children(children);
            queue.apply(world);
        });

        Ok(())
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

        let cell = self.as_unsafe_world_cell()?;
        with_global_access!(self.0.accesses, "Could not insert children", {
            let world = unsafe { cell.world_mut() };
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).insert_children(index, children);
            queue.apply(world);
        });

        Ok(())
    }

    pub fn despawn_recursive(&self, parent: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }
        let cell = self.as_unsafe_world_cell()?;
        with_global_access!(self.0.accesses, "Could not despawn entity", {
            let world = unsafe { cell.world_mut() };
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).despawn_recursive();
            queue.apply(world);
        });

        Ok(())
    }

    pub fn despawn(&self, entity: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(entity)? {
            return Err(InteropError::missing_entity(entity));
        }

        let cell = self.as_unsafe_world_cell()?;
        with_global_access!(self.0.accesses, "Could not despawn entity", {
            let world = unsafe { cell.world_mut() };
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(entity).despawn();
            queue.apply(world);
        });

        Ok(())
    }

    pub fn despawn_descendants(&self, parent: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }

        let cell = self.as_unsafe_world_cell()?;

        with_global_access!(self.0.accesses, "Could not despawn descendants", {
            let world = unsafe { cell.world_mut() };
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).despawn_descendants();
            queue.apply(world);
        });

        Ok(())
    }

    /// Sends AppExit event to the world with success status
    pub fn exit(&self) -> Result<(), InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        with_global_access!(self.0.accesses, "Could not exit the app", {
            let world = unsafe { cell.world_mut() };
            world.send_event(AppExit::Success);
            Ok(())
        })
    }
}

/// Utility type for accessing the world in a callback
pub trait WorldContainer {
    type Error: Debug;
    /// Sets the world to the given value in the container
    fn set_world(&mut self, world: WorldGuard<'static>) -> Result<(), Self::Error>;

    /// Tries to get the world from the container
    fn try_get_world(&self) -> Result<WorldGuard<'static>, Self::Error>;
}

/// A world container that stores the world in a thread local
pub struct ThreadWorldContainer;

thread_local! {
    static WORLD_CALLBACK_ACCESS: RefCell<Option<WorldGuard<'static>>> = const { RefCell::new(None) };
}

impl WorldContainer for ThreadWorldContainer {
    type Error = InteropError;

    fn set_world(&mut self, world: WorldGuard<'static>) -> Result<(), Self::Error> {
        WORLD_CALLBACK_ACCESS.with(|w| {
            w.replace(Some(world));
        });
        Ok(())
    }

    fn try_get_world(&self) -> Result<WorldGuard<'static>, Self::Error> {
        WORLD_CALLBACK_ACCESS.with(|w| w.borrow().clone().ok_or_else(InteropError::missing_world))
    }
}

// #[cfg(test)]
// mod test {
//     use crate::bindings::ScriptTypeRegistration;
//     use crate::prelude::ScriptErrorKind;
//     use bevy::ecs::system::Commands;
//     use bevy::hierarchy::BuildChildren;
//     use bevy::reflect::{ParsedPath, Reflect};

//     use super::*;
//     use std::any::TypeId;

//     use crate::{
//         bindings::ReflectAllocator,
//         bindings::{
//             DeferredReflection, ReflectBase, ReflectBaseType, ReflectReference, ReflectionPathElem,
//         },
//     };

//     use bevy::ecs::world::World;
//     use test_utils::test_data::{
//         setup_world, CompWithFromWorld, GetTestComponentId, TestComponent, TestResource,
//     };

//     fn init_world() -> World {
//         setup_world(|w, _| {
//             w.init_resource::<AppReflectAllocator>();
//         })
//     }

//     /// Tests that the given ref_ can be accessed and the value is as expected and access is released correctly (not for allocated values)
//     fn assert_access_yields<
//         O: Reflect + PartialEq + Debug,
//         F: FnOnce(&mut World) -> ReflectReference,
//         G: FnOnce(&WorldAccessGuard),
//     >(
//         init: F,
//         post_check: G,
//         expected: O,
//     ) {
//         let mut world = init_world();
//         let ref_ = init(&mut world);

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let world = world.read().unwrap();

//             // test read
//             ref_.with_reflect(&world, |reflect| {
//                 let orig = reflect.try_downcast_ref::<O>();

//                 let orig = match orig {
//                     Some(v) => v,
//                     None => {
//                         panic!(
//                             "Could not downcast value {reflect:?} to {}",
//                             std::any::type_name::<O>()
//                         )
//                     }
//                 };

//                 assert_eq!(orig, &expected);
//             });

//             assert!(
//                 world
//                     .get_component_access(TestComponent::test_component_id(), true)
//                     .is_some(),
//                 "access to component was not release correctly"
//             );

//             assert!(
//                 world
//                     .get_resource_access(TestResource::test_component_id())
//                     .is_some(),
//                 "access to component was not release correctly"
//             );

//             post_check(&world);
//         });
//     }

//     /// Tests that setting to the expected value works as well as follow up reads give the expected value
//     fn assert_set_then_get_yields<
//         O: Reflect + PartialEq + Debug + Clone,
//         F: FnOnce(&mut World) -> ReflectReference,
//         G: FnOnce(&WorldAccessGuard),
//     >(
//         init: F,
//         post_check: G,
//         expected: O,
//     ) {
//         let mut world = init_world();
//         let ref_ = init(&mut world);
//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let world = world.read().unwrap();
//             // test set
//             ref_.with_reflect_mut(&world, |reflect, _, _| {
//                 let orig = reflect.try_downcast_mut::<O>();

//                 let orig = match orig {
//                     Some(v) => v,
//                     None => {
//                         panic!(
//                             "Could not downcast value {reflect:?} to {}",
//                             std::any::type_name::<O>()
//                         )
//                     }
//                 };

//                 *orig = expected.clone();
//             });

//             // test read
//             ref_.with_reflect(&world, |reflect, _, _| {
//                 let orig = reflect.try_downcast_ref::<O>();

//                 let orig = match orig {
//                     Some(v) => v,
//                     None => {
//                         panic!(
//                             "Could not downcast value {reflect:?} to {}",
//                             std::any::type_name::<O>()
//                         )
//                     }
//                 };

//                 assert_eq!(orig, &expected);
//             });
//             post_check(&world);
//         });
//     }

//     #[test]
//     fn test_component_access() {
//         let init = |world: &mut World| {
//             let entity = world
//                 .spawn(TestComponent {
//                     strings: vec![String::from("initial")],
//                 })
//                 .id();

//             ReflectReference {
//                 base: ReflectBaseType {
//                     base_id: ReflectBase::Component(entity, TestComponent::test_component_id()),
//                     type_id: TypeId::of::<TestComponent>(),
//                 },
//                 reflect_path: vec![
//                     ReflectionPathElem::Reflection(ParsedPath::parse_static(".strings").unwrap()),
//                     ReflectionPathElem::DeferredReflection(DeferredReflection {
//                         get: Arc::new(|root| {
//                             let strings = root.try_downcast_ref::<Vec<String>>().unwrap();
//                             Ok(strings.first().unwrap())
//                         }),
//                         get_mut: Arc::new(|root| {
//                             let strings = root.try_downcast_mut::<Vec<String>>().unwrap();
//                             Ok(strings.first_mut().unwrap())
//                         }),
//                     }),
//                 ],
//             }
//         };

//         assert_access_yields(init, |_| {}, String::from("initial"));
//         assert_set_then_get_yields(init, |_| {}, String::from("set"));
//     }

//     #[test]
//     fn test_resource_access() {
//         let init = |world: &mut World| {
//             world.insert_resource(TestResource { bytes: vec![42u8] });

//             ReflectReference {
//                 base: ReflectBaseType {
//                     base_id: ReflectBase::Resource(TestResource::test_component_id()),
//                     type_id: TypeId::of::<TestResource>(),
//                 },
//                 reflect_path: vec![
//                     ReflectionPathElem::Reflection(ParsedPath::parse_static(".bytes").unwrap()),
//                     ReflectionPathElem::DeferredReflection(DeferredReflection {
//                         get: Arc::new(|root| {
//                             let strings = root.try_downcast_ref::<Vec<u8>>().unwrap();
//                             Ok(strings.first().unwrap())
//                         }),
//                         get_mut: Arc::new(|root| {
//                             let strings = root.try_downcast_mut::<Vec<u8>>().unwrap();
//                             Ok(strings.first_mut().unwrap())
//                         }),
//                     }),
//                 ],
//             }
//         };
//         assert_access_yields(init, |_| {}, 42u8);
//         assert_set_then_get_yields(init, |_| {}, 69u8);
//     }

//     #[test]
//     fn test_script_alloc_access() {
//         let init = |world: &mut World| {
//             let mut script_allocator = ReflectAllocator::default();
//             let mut ref_ = ReflectReference::new_allocated(
//                 TestComponent {
//                     strings: vec![String::from("initial")],
//                 },
//                 &mut script_allocator,
//             );
//             ref_.index_path(ParsedPath::parse_static(".strings").unwrap());
//             ref_.index_path(DeferredReflection {
//                 get: Arc::new(|root| {
//                     let strings = root.try_downcast_ref::<Vec<String>>().unwrap();
//                     Ok(strings.first().unwrap())
//                 }),
//                 get_mut: Arc::new(|root| {
//                     let strings = root.try_downcast_mut::<Vec<String>>().unwrap();
//                     Ok(strings.first_mut().unwrap())
//                 }),
//             });
//             world.insert_resource(script_allocator);
//             ref_
//         };
//         let post_check = |world: &WorldAccessGuard| {
//             assert!(
//                 world
//                     .get_allocation_access(ReflectAllocationId(0))
//                     .is_some(),
//                 "allocation access was not released correctly"
//             );
//         };
//         assert_access_yields(init, post_check, String::from("initial"));
//         assert_set_then_get_yields(init, post_check, String::from("set"));
//     }

//     #[test]
//     #[allow(clippy::drop_non_drop)]
//     fn test_invalid_runtime_access() {
//         let mut world = World::new();
//         let world = WorldAccessGuard::new(&mut world);
//         let access = world.get_component_access(ComponentId::new(0));
//         assert!(
//             world.get_component_access(ComponentId::new(0)).is_none(),
//             "access was allowed to alias"
//         );
//         drop(access);
//     }

//     fn get_reg(world: &WorldCallbackAccess, name: &str) -> ScriptTypeRegistration {
//         world
//             .get_type_by_name(name.to_owned())
//             .expect("Type not found")
//     }

//     fn test_comp_reg(world: &WorldCallbackAccess) -> ScriptTypeRegistration {
//         world
//             .get_type_by_name("TestComponent".to_owned())
//             .expect("Component not found")
//     }

//     fn test_resource_reg(world: &WorldCallbackAccess) -> ScriptTypeRegistration {
//         world
//             .get_type_by_name("TestResource".to_owned())
//             .expect("Resource not found")
//     }

//     #[test]
//     fn test_get_type_by_name() {
//         let mut world = init_world();
//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let comp_reg = world.get_type_by_name("TestComponent".to_owned()).unwrap();
//             let resource_reg = world.get_type_by_name("TestResource".to_owned()).unwrap();

//             assert_eq!(
//                 comp_reg.registration.type_info().type_id(),
//                 std::any::TypeId::of::<TestComponent>()
//             );
//             assert_eq!(
//                 resource_reg.registration.type_info().type_id(),
//                 std::any::TypeId::of::<TestResource>()
//             );
//         });
//     }

//     #[test]
//     fn test_get_type_by_name_invalid() {
//         let mut world = init_world();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let comp_reg = world.get_type_by_name("x".to_owned());
//             let resource_reg = world.get_type_by_name("z".to_owned());

//             assert!(comp_reg.is_none());
//             assert!(resource_reg.is_none());
//         });
//     }

//     #[test]
//     fn test_add_default_component_from_world() {
//         let mut world = init_world();

//         let entity = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let comp_reg = get_reg(world, "CompWithFromWorld");
//             world.add_default_component(entity, comp_reg).unwrap()
//         });

//         assert_eq!(
//             world.get_entity(entity).unwrap().get::<CompWithFromWorld>(),
//             Some(&CompWithFromWorld(String::from("FromWorld")))
//         );
//     }

//     #[test]
//     fn test_add_default_component_default() {
//         let mut world = init_world();

//         let entity = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let comp_reg = get_reg(world, "CompWithFromWorld");
//             world.add_default_component(entity, comp_reg).unwrap()
//         });

//         assert_eq!(
//             world.get_entity(entity).unwrap().get::<CompWithFromWorld>(),
//             Some(&CompWithFromWorld(String::from("Default")))
//         );
//     }

//     #[test]
//     fn test_add_default_component_missing_from_world_and_default() {
//         let mut world = init_world();

//         let entity = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let comp_reg = get_reg(world, "CompWithFromWorld");
//             match world.add_default_component(entity, comp_reg.clone()) {
//                 Ok(_) => {
//                     panic!("Expected error")
//                 }
//                 Err(ScriptError(inner)) => {
//                     assert_eq!(inner.kind, ScriptErrorKind::RuntimeError);
//                     assert_eq!(inner.reason.to_string(), format!("Cannot add default component since type: `{}`, Does not have ReflectDefault or ReflectFromWorld data registered.", comp_reg.registration.type_info().type_path()));
//                 }
//             }
//         });
//     }

//     #[test]
//     fn test_add_default_component_missing_component_data() {
//         let mut world = init_world();

//         let entity = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let comp_reg = get_reg(world, "CompWithFromWorld");
//             match world.add_default_component(entity, comp_reg.clone()) {
//                 Ok(_) => {
//                     panic!("Expected error")
//                 }
//                 Err(ScriptError(inner)) => {
//                     assert_eq!(inner.kind, ScriptErrorKind::RuntimeError);
//                     assert_eq!(inner.reason.to_string(), format!("Cannot add default component since type: `{}`, Does not have ReflectComponent data registered.", comp_reg.registration.type_info().type_path()));
//                 }
//             }
//         });
//     }

//     #[test]
//     fn test_get_component_existing() {
//         let mut world = init_world();

//         let entity = world.spawn(TestComponent { strings: vec![] }).id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let comp_ref = world
//                 .get_component(entity, TestComponent::test_component_id())
//                 .unwrap()
//                 .unwrap();
//             assert_eq!(
//                 comp_ref,
//                 ReflectReference {
//                     base: ReflectBaseType {
//                         type_id: std::any::TypeId::of::<TestComponent>(),
//                         base_id: ReflectBase::Component(entity, TestComponent::test_component_id()),
//                     },
//                     reflect_path: Default::default(),
//                 }
//             );
//         });
//     }

//     #[test]
//     fn test_get_component_missing() {
//         let mut world = init_world();

//         let entity = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let comp_ref = world
//                 .get_component(entity, TestComponent::test_component_id())
//                 .unwrap();
//             assert_eq!(comp_ref, None);
//         });
//     }

//     #[test]
//     fn test_get_component_missing_entity() {
//         let mut world = init_world();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let comp_ref =
//                 world.get_component(Entity::from_raw(0), TestComponent::test_component_id());
//             match comp_ref {
//                 Ok(_) => {
//                     panic!("Expected error")
//                 }
//                 Err(e) => {
//                     assert_eq!(e.kind, ScriptErrorKind::RuntimeError);
//                     assert_eq!(e.reason.to_string(), "Entity does not exist: 0v1");
//                 }
//             }
//         });
//     }

//     #[test]
//     fn test_get_component_unregistered_component() {
//         let mut world = init_world();

//         let entity = world.spawn_empty().id();
//         let fake_id = ComponentId::new(999);

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let comp_ref = world.get_component(entity, fake_id);
//             match comp_ref {
//                 Ok(_) => {
//                     panic!("Expected error")
//                 }
//                 Err(e) => {
//                     assert_eq!(e.kind, ScriptErrorKind::RuntimeError);
//                     assert_eq!(
//                         e.reason.to_string(),
//                         format!("Component does not exist: {fake_id:?}"),
//                     );
//                 }
//             }
//         });
//     }

//     #[test]
//     fn test_remove_component() {
//         let mut world = init_world();

//         let entity = world
//             .spawn(TestComponent {
//                 strings: vec![String::from("strings")],
//             })
//             .id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world
//                 .remove_component(entity, test_comp_reg(world))
//                 .unwrap();
//         });

//         assert_eq!(
//             world.get_entity(entity).unwrap().get::<TestComponent>(),
//             None
//         );
//     }

//     #[test]
//     fn test_remove_component_empty_idempotent() {
//         let mut world = init_world();

//         let entity = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world
//                 .remove_component(entity, test_comp_reg(world))
//                 .unwrap();
//         });

//         assert_eq!(
//             world.get_entity(entity).unwrap().get::<TestComponent>(),
//             None
//         );
//     }

//     #[test]
//     fn test_remove_component_missing_comp_registration() {
//         let mut world = init_world();

//         let entity = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let result = world.remove_component(entity, test_resource_reg(world));
//             match result {
//                 Ok(_) => {
//                     panic!("Expected error")
//                 }
//                 Err(e) => {
//                     assert_eq!(e.kind, ScriptErrorKind::RuntimeError);
//                     assert_eq!(
//                         e.reason.to_string(),
//                         format!("Cannot remove component since type: `{}`, Does not have ReflectComponent data registered.", test_resource_reg(world).registration.type_info().type_path())
//                     );
//                 }
//             }
//         });

//         assert_eq!(
//             world.get_entity(entity).unwrap().get::<TestComponent>(),
//             None
//         );
//     }

//     #[test]
//     fn test_remove_component_missing_entity() {
//         let mut world = init_world();

//         let fake_entity = Entity::from_raw(0);

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let result = world.remove_component(fake_entity, test_comp_reg(world));
//             match result {
//                 Ok(_) => {
//                     panic!("Expected error")
//                 }
//                 Err(e) => {
//                     assert_eq!(e.kind, ScriptErrorKind::RuntimeError);
//                     assert_eq!(e.reason.to_string(), "Entity does not exist: 0v1");
//                 }
//             }
//         });
//     }

//     #[test]
//     fn test_get_resource_existing() {
//         let mut world = init_world();

//         world.insert_resource(TestResource { bytes: vec![1] });

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let comp_ref = world
//                 .get_resource(TestResource::test_component_id())
//                 .unwrap()
//                 .unwrap();
//             assert_eq!(
//                 comp_ref,
//                 ReflectReference {
//                     base: ReflectBaseType {
//                         type_id: std::any::TypeId::of::<TestResource>(),
//                         base_id: ReflectBase::Resource(TestResource::test_component_id()),
//                     },
//                     reflect_path: Default::default(),
//                 }
//             );
//         });
//     }

//     #[test]
//     fn test_get_resource_non_existing() {
//         let mut world = init_world();

//         let fake_comp = ComponentId::new(999);
//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let comp_ref = world.get_resource(fake_comp);
//             match comp_ref {
//                 Ok(None) => {}
//                 e => {
//                     panic!("Expected ok with none, got: {:?}", e);
//                 }
//             }
//         });
//     }

//     #[test]
//     fn test_remove_resource() {
//         let mut world = init_world();

//         world.insert_resource(TestResource { bytes: vec![1] });

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world.remove_resource(test_resource_reg(world)).unwrap();
//         });

//         assert_eq!(world.get_resource::<TestResource>(), None);
//     }

//     #[test]
//     fn test_remove_resource_missing_idempotent() {
//         let mut world = init_world();

//         world.remove_resource::<TestResource>();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world.remove_resource(test_resource_reg(world)).unwrap();
//         });

//         assert_eq!(world.get_resource::<TestResource>(), None);
//     }

//     #[test]
//     fn test_remove_resource_missing_resource_registration() {
//         let mut world = init_world();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             match world.remove_resource(test_comp_reg(world)) {
//                 Ok(_) => panic!("Expected error"),
//                 Err(e) => {
//                     assert_eq!(e.kind, ScriptErrorKind::RuntimeError);
//                     assert_eq!(e.reason.to_string(), format!("Cannot remove resource since type: `{}`, Does not have ReflectResource data registered.", test_comp_reg(world).registration.type_info().type_path()));
//                 }
//             }
//         });
//     }

//     #[test]
//     fn test_has_resource_existing() {
//         let mut world = init_world();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             assert!(world.has_resource(TestResource::test_component_id()));
//         });
//     }

//     #[test]
//     fn test_has_resource_missing() {
//         let mut world = init_world();

//         world.remove_resource::<TestResource>();
//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             assert!(world.has_resource(TestResource::test_component_id()));
//         });
//     }

//     #[test]
//     fn test_get_children_1_child() {
//         let mut world = init_world();

//         let parent = world.spawn_empty().id();
//         let child = world.spawn_empty().id();
//         let mut cmnds = CommandQueue::default();
//         let mut cmnd = Commands::new(&mut cmnds, &world);
//         cmnd.entity(parent).add_children(&[child]);
//         cmnds.apply(&mut world);

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let children = world.get_children(parent).unwrap();
//             assert_eq!(children.len(), 1);
//             assert_eq!(children[0], child);
//         });
//     }

//     #[test]
//     #[should_panic(
//         expected = "Component not registered: `bevy_hierarchy::components::children::Children`"
//     )]
//     fn test_get_children_children_component_unregistered() {
//         let mut world = init_world();

//         let parent = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world.get_children(parent).unwrap();
//         });
//     }

//     #[test]
//     fn test_get_children_no_children() {
//         let mut world = init_world();

//         world.register_component::<Children>();
//         let parent = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let children = world.get_children(parent).unwrap();
//             assert_eq!(children.len(), 0);
//         });
//     }

//     #[test]
//     fn test_get_parent_1_parent() {
//         let mut world = init_world();

//         let parent = world.spawn_empty().id();
//         let child = world.spawn_empty().id();
//         let mut cmnds = CommandQueue::default();
//         let mut cmnd = Commands::new(&mut cmnds, &world);
//         cmnd.entity(parent).add_children(&[child]);
//         cmnds.apply(&mut world);

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let found_parent = world.get_parent(child).unwrap();
//             assert_eq!(found_parent, Some(parent));
//         });
//     }

//     #[test]
//     fn test_get_parent_no_parent() {
//         let mut world = init_world();

//         world.register_component::<Parent>();

//         let child = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             let found_parent = world.get_parent(child).unwrap();
//             assert_eq!(found_parent, None);
//         });
//     }

//     #[test]
//     #[should_panic(
//         expected = "Component not registered: `bevy_hierarchy::components::parent::Parent`"
//     )]
//     fn test_get_parent_parent_component_unregistered() {
//         let mut world = init_world();

//         let child = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world.get_parent(child).unwrap();
//         });
//     }

//     #[test]
//     fn test_push_children_empty_entity() {
//         let mut world = init_world();

//         let parent = world.spawn_empty().id();
//         let child = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world.push_children(parent, &[child]).unwrap();
//         });

//         let children = world.get::<Children>(parent).unwrap();
//         assert_eq!(children.len(), 1);
//         assert_eq!(children[0], child);
//     }

//     #[test]
//     fn test_push_children_entity_with_1_child() {
//         let mut world = init_world();

//         let parent = world.spawn_empty().id();
//         let child = world.spawn_empty().id();
//         let mut cmnds = CommandQueue::default();
//         let mut cmnd = Commands::new(&mut cmnds, &world);
//         cmnd.entity(parent).add_children(&[child]);
//         cmnds.apply(&mut world);

//         let child_2 = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world.push_children(parent, &[child_2]).unwrap();
//         });

//         let children = world.get::<Children>(parent).unwrap();
//         assert_eq!(children.len(), 2);
//         assert_eq!(children[0], child);
//         assert_eq!(children[1], child_2);
//     }

//     #[test]
//     fn test_remove_children_entity_with_1_child() {
//         let mut world = init_world();

//         let parent = world.spawn_empty().id();
//         let child = world.spawn_empty().id();
//         let mut cmnds = CommandQueue::default();
//         let mut cmnd = Commands::new(&mut cmnds, &world);
//         cmnd.entity(parent).add_children(&[child]);
//         cmnds.apply(&mut world);

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world.remove_children(parent, &[child]).unwrap();
//         });

//         let children = world.get::<Children>(parent);
//         assert!(children.is_none());
//     }

//     #[test]
//     fn test_remove_children_remove_half_children() {
//         let mut world = init_world();

//         let parent = world.spawn_empty().id();
//         let child = world.spawn_empty().id();
//         let child_2 = world.spawn_empty().id();
//         let mut cmnds = CommandQueue::default();
//         let mut cmnd = Commands::new(&mut cmnds, &world);
//         cmnd.entity(parent).add_children(&[child, child_2]);
//         cmnds.apply(&mut world);

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world.remove_children(parent, &[child]).unwrap();
//         });

//         let children = world.get::<Children>(parent).unwrap();
//         assert_eq!(children.len(), 1);
//         assert_eq!(children[0], child_2);
//     }

//     #[test]
//     fn test_insert_children_empty() {
//         let mut world = init_world();

//         let parent = world.spawn_empty().id();
//         let child = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world.insert_children(parent, 0, &[child]).unwrap();
//         });

//         let children = world.get::<Children>(parent).unwrap();
//         assert_eq!(children.len(), 1);
//         assert_eq!(children[0], child);
//     }

//     #[test]
//     fn test_insert_children_middle() {
//         let mut world = init_world();

//         let parent = world.spawn_empty().id();
//         let child = world.spawn_empty().id();
//         let child_2 = world.spawn_empty().id();
//         let mut cmnds = CommandQueue::default();
//         let mut cmnd = Commands::new(&mut cmnds, &world);
//         cmnd.entity(parent).add_children(&[child, child_2]);
//         cmnds.apply(&mut world);

//         let child_to_insert = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world
//                 .insert_children(parent, 1, &[child_to_insert])
//                 .unwrap();
//         });

//         let children = world.get::<Children>(parent).unwrap();
//         assert_eq!(children.len(), 3);
//         assert_eq!(children[0], child);
//         assert_eq!(children[1], child_to_insert);
//         assert_eq!(children[2], child_2);
//     }

//     #[test]
//     fn test_despawn_entity() {
//         let mut world = init_world();

//         let entity = world.spawn_empty().id();

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world.despawn(entity).unwrap();
//         });

//         assert!(world.get_entity(entity).is_err());
//     }

//     #[test]
//     fn test_despawn_recursive() {
//         let mut world = init_world();

//         let parent = world.spawn_empty().id();
//         let child = world.spawn_empty().id();
//         let mut cmnds = CommandQueue::default();
//         let mut cmnd = Commands::new(&mut cmnds, &world);
//         cmnd.entity(parent).add_children(&[child]);
//         cmnds.apply(&mut world);

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world.despawn_recursive(parent).unwrap();
//         });

//         assert!(world.get_entity(parent).is_err());
//         assert!(world.get_entity(child).is_err());
//     }

//     #[test]
//     fn test_despawn_descendants() {
//         let mut world = init_world();

//         let parent = world.spawn_empty().id();
//         let child = world.spawn_empty().id();
//         let mut cmnds = CommandQueue::default();
//         let mut cmnd = Commands::new(&mut cmnds, &world);
//         cmnd.entity(parent).add_children(&[child]);
//         cmnds.apply(&mut world);

//         WorldCallbackAccess::with_callback_access(&mut world, |world| {
//             world.despawn_descendants(parent).unwrap();
//         });

//         assert!(world.get_entity(parent).is_ok());
//         assert!(world.get_entity(child).is_err());
//     }
// }
