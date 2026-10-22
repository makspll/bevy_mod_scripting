//!
//! Implementations of [`WorldExtensions`] trait on the world guard.

use super::{
    AnyTypeRegistration, AppScriptCallbackRunners, AppScriptComponentRegistry, EnumVariantRef,
    ReflectBase, ReflectBaseType, ReflectReference, ScriptBundleRegistration,
    ScriptComponentRegistration, ScriptResourceRegistration, ScriptTypeRegistration, StaticTypeRef,
    Union,
    function::{
        namespace::FunctionScope,
        script_function::{AppScriptFunctionRegistry, DynamicScriptFunction, FunctionCallContext},
    },
    schedule::AppScheduleRegistry,
    script_value::ScriptValue,
    static_type::registered_type_info,
};
use crate::{
    AppScriptTypeCatalogue, DynamicComponent, DynamicComponentInfo, ScriptEvent,
    ScriptEventRegistration, ScriptQueryBuilder, ScriptQueryResult,
    error::InteropError,
    function::{from::V, from_ref::FromScriptErased, into::IntoScript},
    script_path, validate_script_namespace_and_name,
};
use ::{
    bevy_asset::{AssetServer, Handle, LoadState},
    bevy_ecs::{
        component::ComponentId,
        entity::Entity,
        reflect::{ReflectBundle, ReflectEvent, ReflectFromWorld, ReflectMessage, ReflectResource},
        world::World,
    },
    bevy_reflect::{PartialReflect, std_traits::ReflectDefault},
};
use bevy_app::AppExit;
use bevy_asset::AssetPath;
use bevy_ecs::{
    component::{Component, ComponentCloneBehavior, ComponentDescriptor, Mutable},
    hierarchy::{ChildOf, Children},
    observer::{Observer, On},
    resource::Resource,
    system::Commands,
    world::{CommandQueue, DeferredWorld, EntityRef, Mut},
};
use bevy_mod_scripting_asset::ScriptAsset;
use bevy_mod_scripting_script::{Recipients, ScriptAttachment};
use bevy_mod_scripting_world::{CachedRegistry, RegistryCache, WorldAccessGuard, WorldGuard};
use bevy_reflect::{GetTypeRegistration, Reflect, TypeInfo, TypeRegistration};
use bevy_system_reflection::ReflectSchedule;
use std::{
    alloc::Layout,
    any::TypeId,
    borrow::Cow,
    cell::{Ref, RefCell},
    collections::VecDeque,
    mem::needs_drop,
    rc::Rc,
    sync::Arc,
};

/// Functional extensions to the [`WorldGuard`]
pub trait WorldExtensions {
    /// Spawns a new empty entity and returns its ID.
    fn spawn(&self) -> Result<Entity, InteropError>;

    /// Despawns the given entity (also removes its Children component).
    fn despawn(&self, entity: Entity) -> Result<(), InteropError>;

    /// Despawns the entity and all of its descendants.
    fn despawn_recursive(&self, parent: Entity) -> Result<(), InteropError>;

    /// Despawns only the descendants of the given entity.
    fn despawn_descendants(&self, parent: Entity) -> Result<(), InteropError>;

    /// Checks whether the entity exists and is valid.
    fn is_valid_entity(&self, entity: Entity) -> Result<bool, InteropError>;

    /// Alias for `is_valid_entity`.
    fn has_entity(&self, entity: Entity) -> Result<bool, InteropError>;

    /// Runs a query and returns matching entities and component references.
    fn query(&self, query: ScriptQueryBuilder)
    -> Result<VecDeque<ScriptQueryResult>, InteropError>;

    /// Inserts a component value into an entity.
    fn insert_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
        value: ReflectReference,
    ) -> Result<(), InteropError>;

    /// Adds a default-constructed component to an entity.
    fn add_default_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
    ) -> Result<(), InteropError>;

    /// Removes a component from an entity.
    fn remove_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
    ) -> Result<(), InteropError>;

    /// Retrieves a component reference from an entity if present.
    fn get_component(
        &self,
        entity: Entity,
        component_registration: ScriptComponentRegistration,
    ) -> Result<Option<ReflectReference>, InteropError>;

    /// Checks if an entity contains a specific component.
    fn has_component(
        &self,
        entity: Entity,
        component_id: ComponentId,
    ) -> Result<bool, InteropError>;

    /// Inserts a bundle value into an entity.
    fn insert_bundle(
        &self,
        entity: Entity,
        registration: ScriptBundleRegistration,
        value: ReflectReference,
    ) -> Result<(), InteropError>;

    /// Spawns a new entity with the given bundle value and returns its ID.
    fn spawn_bundle(
        &self,
        registration: ScriptBundleRegistration,
        value: ReflectReference,
    ) -> Result<Entity, InteropError>;

    /// Executes a closure with access to a component (if present).
    fn with_component<C: Component, O, F: FnOnce(Option<&C>) -> O>(
        &self,
        entity: Entity,
        f: F,
    ) -> Result<O, InteropError>;

    /// Executes a closure with access to a component (if present).
    fn with_component_mut<C: Component<Mutability = Mutable>, O, F: FnOnce(Option<Mut<C>>) -> O>(
        &self,
        entity: Entity,
        f: F,
    ) -> Result<O, InteropError>;

    /// Executes a closure with access to a component after inserting its default value if it doesn't exist (if present).
    fn with_or_insert_component_mut<
        C: Component<Mutability = Mutable> + Default,
        O,
        F: FnOnce(&mut C) -> O,
    >(
        &self,
        entity: Entity,
        f: F,
    ) -> Result<O, InteropError>;

    /// Retrieves a resource reference if it exists.
    fn get_resource(
        &self,
        resource_id: ComponentId,
    ) -> Result<Option<ReflectReference>, InteropError>;

    /// Removes a resource from the world.
    fn remove_resource(&self, registration: ScriptResourceRegistration)
    -> Result<(), InteropError>;

    /// Checks if a resource exists.
    fn has_resource(&self, resource_id: ComponentId) -> Result<bool, InteropError>;

    /// Executes a closure with shared access to a resource.
    fn with_resource<R: Resource, O, F: FnOnce(&R) -> O>(&self, f: F) -> Result<O, InteropError>;

    /// Executes a closure with mutable access to a resource.
    fn with_resource_mut<R: Resource<Mutability = Mutable>, O, F: FnOnce(Mut<R>) -> O>(
        &self,
        f: F,
    ) -> Result<O, InteropError>;

    /// Triggers a reflected [`bevy_ecs::event::Event`] (observer style) on the world, using the type's [`ReflectEvent`] type data.
    ///
    /// The value is cloned out of the reference first, so no access to the reference is held while observers run.
    /// Observers run synchronously with exclusive world access, this fails if any other access is currently held.
    fn trigger_event(
        &self,
        registration: ScriptTypeRegistration,
        event: &ReflectReference,
    ) -> Result<(), InteropError>;

    /// Exports the callback `label` of the script `owner` as the function `name` of a script namespace (which must start with `script.`).
    ///
    /// The function can then be called by any script through `import_namespace(namespace).name`, in any language, see [`Self::call_script_export`].
    /// Exporting the same name again from the same script replaces it, from a different script it is an error.
    fn register_script_export(
        &self,
        namespace: &str,
        name: &str,
        label: String,
        owner: ScriptAttachment,
    ) -> Result<(), InteropError>;

    /// Calls the exported function with the given full path (i.e. `script.game.spawn_enemy`) in the script that exported it,
    /// by running its callback through the callback runners.
    ///
    /// Fails if the export does not exist (anymore), if the owning script is not loaded, or if its context is busy running another callback.
    fn call_script_export(
        &self,
        path: &str,
        args: Vec<ScriptValue>,
    ) -> Result<ScriptValue, InteropError>;

    /// Registers a script event, and returns a handle to it.
    ///
    /// Fails if an event with the same name is already registered, unless `allow_existing` is set,
    /// in which case the existing event is returned instead.
    fn register_script_event(
        &self,
        namespace: &str,
        name: &str,
        allow_existing: bool,
    ) -> Result<ScriptEventRegistration, InteropError>;

    /// Spawns an observer which queues the callback `label` on the script `attachment`, with the payload as argument, whenever the script event is triggered.
    ///
    /// See [`Self::add_script_observer`] for when callbacks run.
    fn add_script_event_observer(
        &self,
        event: ScriptEventRegistration,
        label: String,
        attachment: ScriptAttachment,
    ) -> Result<Entity, InteropError>;

    /// Triggers a script event with the given payload, observers of the event queue their callbacks.
    fn trigger_script_event(
        &self,
        event: ScriptEventRegistration,
        payload: ScriptValue,
    ) -> Result<(), InteropError>;

    /// Spawns an observer which queues the callback `label` on the script `attachment` whenever an event of the given type is triggered.
    ///
    /// The event type must have [`ReflectEvent`] type data. For events which are triggered for components
    /// (i.e. the lifecycle events `AddEvent`, `InsertEvent`, `DiscardEvent`, `RemoveEvent`, `DespawnEvent`), the observer runs for the given components.
    /// If `entities` is non-empty, it only runs for those entities.
    ///
    /// The callback receives a clone of the event as its only argument, and is run by the same machinery as every other script callback
    /// (see [`AppScriptCallbackRunners::run_or_queue`]): right after the operation which triggered the event, unless the script's context
    /// is busy running another callback (e.g. the one that triggered the event), in which case it is queued
    /// and runs as soon as that finishes. The order in which the observers of one event run is unspecified.
    /// The world is already past the change (i.e. removed component data is no longer readable).
    ///
    /// Returns the observer entity, despawn it to remove the observer.
    fn add_script_observer(
        &self,
        event: ScriptTypeRegistration,
        components: Vec<ScriptComponentRegistration>,
        entities: Vec<Entity>,
        label: String,
        attachment: ScriptAttachment,
    ) -> Result<Entity, InteropError>;

    /// Writes a reflected [`bevy_ecs::message::Message`] (buffered) to the world, using the type's [`ReflectMessage`] type data.
    fn write_message(
        &self,
        registration: ScriptTypeRegistration,
        message: &ReflectReference,
    ) -> Result<(), InteropError>;

    /// Executes a closure with immutable access to the world.
    fn with_world<O, F: FnOnce(&World) -> O>(&self, f: F) -> Result<O, InteropError>;

    /// Executes a closure with mutable access to the world.
    fn with_world_mut<O, F: FnOnce(&mut World) -> O>(&self, f: F) -> Result<O, InteropError>;

    /// Returns the parent of an entity if it has one.
    fn get_parent(&self, entity: Entity) -> Result<Option<Entity>, InteropError>;

    /// Returns all children of an entity.
    fn get_children(&self, entity: Entity) -> Result<Vec<Entity>, InteropError>;

    /// Appends children to the given parent entity.
    fn push_children(&self, parent: Entity, children: &[Entity]) -> Result<(), InteropError>;

    /// Inserts children at a specific index in the parent's child list.
    fn insert_children(
        &self,
        parent: Entity,
        index: usize,
        children: &[Entity],
    ) -> Result<(), InteropError>;

    /// Removes specific children from a parent entity.
    fn remove_children(&self, parent: Entity, children: &[Entity]) -> Result<(), InteropError>;

    /// Sends an AppExit::Success event to the world.
    fn exit(&self) -> Result<(), InteropError>;

    // /// Retrieves all systems in the given schedule.
    // fn systems(&self, schedule: &ReflectSchedule) -> Result<Vec<ReflectSystem>, InteropError>;

    // /// Temporarily removes a schedule, mutates it, then reinserts it.
    // fn scope_schedule<O, F>(&self, label: &ReflectSchedule, f: F) -> Result<O, InteropError>
    // where
    //     F: FnOnce(&mut World, &mut Schedule) -> O;

    /// Retrieves a schedule by its name.
    fn get_schedule_by_name(&self, schedule_name: String) -> Option<ReflectSchedule>;

    /// Loads a script asset from the given path.
    fn load_script_asset<'a>(
        &self,
        asset_path: impl Into<AssetPath<'a>>,
    ) -> Result<Handle<ScriptAsset>, InteropError>;

    /// Returns the load state of a script asset.
    fn get_script_asset_load_state(
        &self,
        script: Handle<ScriptAsset>,
    ) -> Result<LoadState, InteropError>;

    /// Calls the function with the given name registered on the given type, used to dispatch operators.
    fn call_operator(
        &self,
        type_id: TypeId,
        name: impl Into<Cow<'static, str>>,
        args: Vec<ScriptValue>,
        context: FunctionCallContext,
    ) -> Result<ScriptValue, InteropError>;

    /// Lists all functions available on a type (including reference methods).
    fn get_functions_on_type(
        &self,
        type_id: TypeId,
    ) -> Vec<(Cow<'static, str>, DynamicScriptFunction)>;

    /// Looks up a member accessed by name on a value, i.e. `value.name` in a script, before falling back to field access.
    ///
    /// Resolution order:
    /// 1. functions registered on the type itself, then those on [`ReflectReference`], which apply to all instances
    /// 2. for an enum variant: the functions of the enum, so `MyEnum.Variant:method()` works
    ///
    /// For a type registration, such as the entries of `import_namespace`, the static members of the registered type (see [`Self::lookup_static_member`])
    /// come right after the functions of the registration itself, before the functions shared by all references,
    /// so `math.Vec3.new(..)` and `my_namespace.MyEnum.Variant` work.
    ///
    /// `type_id` is the type of the value `reference` points to. Returns the name back if nothing was found.
    fn lookup_member<'n>(
        &self,
        reference: &ReflectReference,
        type_id: TypeId,
        name: impl Into<Cow<'n, str>>,
    ) -> Result<ScriptValue, Cow<'n, str>>;

    /// Looks up anything accessible as `Type.member` on the static reference to a type in a script.
    ///
    /// Resolution order:
    /// 1. functions registered on the type itself
    /// 2. functions shared by all types (registered on [`StaticTypeRef`]), with the type bound as their first argument
    /// 3. the variants of the type, if it is an enum
    ///
    /// Returns the name back if nothing was found.
    fn lookup_static_member<'n>(
        &self,
        type_id: TypeId,
        name: impl Into<Cow<'n, str>>,
    ) -> Result<ScriptValue, Cow<'n, str>>;

    /// Resolves a type registration by its full type path, i.e. `bevy_transform::components::transform::Transform`.
    fn get_type_by_name(&self, type_name: &str) -> Option<ScriptTypeRegistration>;

    /// Resolves a type registration and determines if it's a component or resource.
    fn get_type_registration(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<AnyTypeRegistration, InteropError>;

    /// Resolves a type registration by name with component/resource detection.
    fn get_type_registration_by_name(
        &self,
        type_name: String,
    ) -> Result<Option<AnyTypeRegistration>, InteropError>;

    /// Attempts to interpret a type as a resource type.
    fn get_resource_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptResourceRegistration, ScriptTypeRegistration>, InteropError>;

    /// Attempts to interpret a type as a component type.
    fn get_component_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptComponentRegistration, ScriptTypeRegistration>, InteropError>;

    /// Attempts to interpret a type as a bundle type, i.e. a type with `ReflectBundle` type data.
    fn get_bundle_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptBundleRegistration, ScriptTypeRegistration>, InteropError>;

    /// Returns the script function registry.
    fn script_function_registry(&'_ self) -> Ref<'_, AppScriptFunctionRegistry>;

    /// Returns the callback runners of all scripting plugins.
    fn script_callback_runners(&'_ self) -> Ref<'_, AppScriptCallbackRunners>;

    /// Returns the type catalogue.
    fn type_catalogue(&'_ self) -> Ref<'_, AppScriptTypeCatalogue>;

    /// Returns the component registry.
    fn component_registry(&'_ self) -> Ref<'_, AppScriptComponentRegistry>;

    /// Returns the schedule registry.
    fn schedule_registry(&'_ self) -> Ref<'_, AppScheduleRegistry>;

    /// Returns the current attachment if the guard is being used in the context of one.
    fn current_attachment(&self) -> CurrentScriptAttachment;

    /// Sets the current attachment for the world guard context.
    fn set_current_attachment(&self, attachment: ScriptAttachment);

    /// Registers a dynamic script component in a script namespace (which must start with `script.`), and returns a reference to its registration
    ///
    /// Fails if a component with the same namespace and name is already registered, unless `allow_existing` is set,
    /// in which case the existing component's registration is returned instead.
    fn register_script_component(
        &self,
        namespace: &str,
        component_name: &str,
        allow_existing: bool,
    ) -> Result<ScriptComponentRegistration, InteropError>;

    /// Initializes cached registries from the world.
    fn setup_cache(world: &World, attachment: CurrentScriptAttachment) -> RegistryCache;

    /// Initializes cached registries from the world, from raw components.
    fn setup_cache_raw(
        attachment: CurrentScriptAttachment,
        function_registry: AppScriptFunctionRegistry,
        schedule_registry: AppScheduleRegistry,
        component_registry: AppScriptComponentRegistry,
        callback_runners: AppScriptCallbackRunners,
        type_catalogue: AppScriptTypeCatalogue,
    ) -> RegistryCache;
}

impl<'w> WorldExtensions for WorldAccessGuard<'w> {
    fn spawn(&self) -> Result<Entity, InteropError> {
        self.with_world_mut(|world| {
            let mut command_queue = CommandQueue::default();
            let mut commands = Commands::new(&mut command_queue, world);
            let id = commands.spawn_empty().id();
            command_queue.apply(world);
            id
        })
    }

    fn despawn(&self, entity: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(entity)? {
            return Err(InteropError::missing_entity(entity));
        }

        self.with_world_mut(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(entity).remove::<Children>().despawn();
            queue.apply(world);
        })
    }

    fn despawn_recursive(&self, parent: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }
        self.with_world_mut(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).despawn();
            queue.apply(world);
        })
    }

    fn despawn_descendants(&self, parent: Entity) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }

        self.with_world_mut(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).despawn_related::<Children>();
            queue.apply(world);
        })
    }

    fn is_valid_entity(&self, entity: Entity) -> Result<bool, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        Ok(cell.get_entity(entity).is_ok() && entity.index().index() != 0)
    }

    fn has_entity(&self, entity: Entity) -> Result<bool, InteropError> {
        self.is_valid_entity(entity)
    }

    fn query(
        &self,
        query: crate::ScriptQueryBuilder,
    ) -> Result<VecDeque<ScriptQueryResult>, InteropError> {
        self.with_world_mut(|world| {
            let mut built_query = query.as_query_state::<EntityRef>(world, false);
            let query_result = built_query.iter(world);

            Ok(query_result
                .map(|r| {
                    let references: Vec<_> = query
                        .components
                        .iter()
                        .map(|c| ReflectReference {
                            base: super::ReflectBaseType {
                                type_id: c.type_registration().type_id(),
                                base_id: super::ReflectBase::Component(r.id(), c.component_id()),
                            },
                            reflect_path: Default::default(),
                        })
                        .collect();
                    ScriptQueryResult {
                        entity: r.id(),
                        components: references,
                    }
                })
                .collect())
        })?
    }

    /// insert the component into the entity
    fn insert_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
        value: ReflectReference,
    ) -> Result<(), InteropError> {
        let type_id = registration.type_registration().type_id();
        // allow enum variants, i.e. `Visibility.Hidden`, in place of values
        let value =
            crate::function::from::variant_as_value_of(&value, self, type_id).unwrap_or(value);
        let instance = <Box<dyn PartialReflect>>::from_script_erased(
            type_id,
            ScriptValue::Reference(value),
            self.clone(),
        )?;

        // inserting a value of another type panics inside bevy
        let instance_type_id = instance.get_represented_type_info().map(|i| i.type_id());
        if instance_type_id != Some(type_id) {
            return Err(InteropError::type_mismatch(type_id, instance_type_id));
        }

        let reflect = instance.try_into_reflect().map_err(|v| {
            InteropError::failed_from_reflect(
                Some(registration.type_registration().type_id()),
                format!("instance produced by conversion to target type when inserting component is not a full reflect type: {v:?}"),
            )
        })?;

        registration.insert_into_entity(self.clone(), entity, reflect)
    }

    /// add a default component to an entity
    fn add_default_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
    ) -> Result<(), InteropError> {
        let instance = default_instance(
            self.clone(),
            registration.type_registration().type_registration(),
        )?;

        registration.insert_into_entity(self.clone(), entity, instance)
    }

    /// remove the component from the entity
    fn remove_component(
        &self,
        entity: Entity,
        registration: ScriptComponentRegistration,
    ) -> Result<(), InteropError> {
        registration.remove_from_entity(self.clone(), entity)
    }

    /// get the component from the entity
    fn get_component(
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
                reflect_path: Default::default(),
            }))
        } else {
            Ok(None)
        }
    }

    /// check if the entity has the component
    fn has_component(
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

    fn insert_bundle(
        &self,
        entity: Entity,
        registration: ScriptBundleRegistration,
        value: ReflectReference,
    ) -> Result<(), InteropError> {
        let instance =
            registration.instance_from_script(self.clone(), ScriptValue::Reference(value))?;
        registration.insert_into_entity(self.clone(), entity, instance)
    }

    fn spawn_bundle(
        &self,
        registration: ScriptBundleRegistration,
        value: ReflectReference,
    ) -> Result<Entity, InteropError> {
        let instance =
            registration.instance_from_script(self.clone(), ScriptValue::Reference(value))?;
        let entity = self.spawn()?;
        if let Err(e) = registration.insert_into_entity(self.clone(), entity, instance) {
            self.despawn(entity)?;
            return Err(e);
        }
        Ok(entity)
    }

    fn with_component<C: Component, O, F: FnOnce(Option<&C>) -> O>(
        &self,
        entity: Entity,
        f: F,
    ) -> Result<O, InteropError> {
        let type_id = std::any::TypeId::of::<C>();
        let component_id = self.get_component_id(type_id)?.ok_or(
            InteropError::unregistered_component_or_resource_type(type_id),
        )?;
        let cell = self.as_unsafe_world_cell()?;
        // Safety: we claimed access to this component
        self.with_read_access(component_id, || {
            f(unsafe { cell.get_entity(entity).ok().and_then(|e| e.get::<C>()) })
        })
        .map_err(Into::into)
    }

    fn with_component_mut<C: Component<Mutability = Mutable>, O, F: FnOnce(Option<Mut<C>>) -> O>(
        &self,
        entity: Entity,
        f: F,
    ) -> Result<O, InteropError> {
        let type_id = std::any::TypeId::of::<C>();
        let component_id = self.get_component_id(type_id)?.ok_or(
            InteropError::unregistered_component_or_resource_type(type_id),
        )?;
        let cell = self.as_unsafe_world_cell()?;
        // Safety: we claimed access to this component
        self.with_write_access(component_id, || {
            f(unsafe { cell.get_entity(entity).ok().and_then(|e| e.get_mut::<C>()) })
        })
        .map_err(Into::into)
    }

    fn with_or_insert_component_mut<
        C: Component<Mutability = Mutable> + Default,
        O,
        F: FnOnce(&mut C) -> O,
    >(
        &self,
        entity: Entity,
        f: F,
    ) -> Result<O, InteropError> {
        self.with_world_mut(|world| match world.get_mut::<C>(entity) {
            Some(mut component) => f(&mut component),
            None => {
                let mut component = C::default();
                let mut commands = world.commands();
                let result = f(&mut component);
                commands.entity(entity).insert(component);
                world.flush();
                result
            }
        })
    }

    /// get the given resource
    fn get_resource(
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
                                component_info.name()
                            ),
                        )
                    })?,
                base_id: ReflectBase::Resource(resource_id),
            },
            reflect_path: Default::default(),
        }))
    }

    fn remove_resource(
        &self,
        registration: ScriptResourceRegistration,
    ) -> Result<(), InteropError> {
        // check is a resource
        registration
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
        self.with_world_mut(|world| {
            let _ = world.remove_resource_by_id(registration.resource_id);
        })
    }

    fn has_resource(&self, resource_id: ComponentId) -> Result<bool, InteropError> {
        let cell = self.as_unsafe_world_cell()?;
        // Safety: we are not reading the value at all
        let res_ptr = unsafe { cell.get_resource_by_id(resource_id) };
        Ok(res_ptr.is_some())
    }

    fn with_resource<R: Resource, O, F: FnOnce(&R) -> O>(&self, f: F) -> Result<O, InteropError> {
        let type_id = std::any::TypeId::of::<R>();
        let resource_id = self.get_resource_id(type_id)?.ok_or(
            InteropError::unregistered_component_or_resource_type(type_id),
        )?;
        let cell = self.as_unsafe_world_cell()?;
        // Safety: we claimed access to this resource
        self.with_read_access_and_then(resource_id, || {
            Ok(f(unsafe {
                cell.get_resource::<R>()
                    .ok_or_else(|| InteropError::missing_resource(type_id))?
            }))
        })
    }

    fn with_resource_mut<R: Resource<Mutability = Mutable>, O, F: FnOnce(Mut<R>) -> O>(
        &self,
        f: F,
    ) -> Result<O, InteropError> {
        let type_id = std::any::TypeId::of::<R>();
        let resource_id = self.get_resource_id(type_id)?.ok_or(
            InteropError::unregistered_component_or_resource_type(type_id),
        )?;
        let cell = self.as_unsafe_world_cell()?;
        // Safety: we claimed access to this resource
        self.with_write_access_and_then(resource_id, || {
            Ok(f(unsafe {
                cell.get_resource_mut::<R>()
                    .ok_or_else(|| InteropError::missing_resource(type_id))?
            }))
        })
    }

    fn trigger_event(
        &self,
        registration: ScriptTypeRegistration,
        event: &ReflectReference,
    ) -> Result<(), InteropError> {
        let reflect_event = registration
            .type_registration()
            .data::<ReflectEvent>()
            .ok_or_else(|| {
                InteropError::missing_type_data(registration.type_id(), "ReflectEvent".to_owned())
            })?
            .clone();
        let value = event.to_owned_value_of_type(self.clone(), registration.type_id())?;
        let registry = self.type_registry().clone();
        self.with_world_mut(|world| {
            let registry = registry.read();
            reflect_event.trigger(world, value.as_ref(), &registry);
            // apply the commands of observers, i.e. the callbacks of script observers
            world.flush();
        })
    }

    fn add_script_observer(
        &self,
        event: ScriptTypeRegistration,
        components: Vec<ScriptComponentRegistration>,
        entities: Vec<Entity>,
        label: String,
        attachment: ScriptAttachment,
    ) -> Result<Entity, InteropError> {
        let reflect_event = event
            .type_registration()
            .data::<ReflectEvent>()
            .ok_or_else(|| {
                InteropError::missing_type_data(event.type_id(), "ReflectEvent".to_owned())
            })?
            .clone();
        let runners = self.script_callback_runners().clone();
        let recipients = Recipients::from(attachment.clone());

        let mut observer = reflect_event.create_observer(Box::new(move |event, mut world| {
            let value = match event.reflect_clone() {
                Ok(value) => value,
                Err(e) => {
                    bevy_log::error!("script observer could not clone its event: {e}");
                    return;
                }
            };
            let reference = ReflectReference::new_allocated_boxed(value);
            run_observer_callback(
                &mut world,
                &runners,
                &recipients,
                &label,
                ScriptValue::Reference(reference),
            );
        }));
        for component in components {
            observer = observer.with_component(component.component_id);
        }
        if !entities.is_empty() {
            observer = observer.with_entities(entities);
        }
        // removed again when the script is unloaded
        self.with_world_mut(|world| {
            world
                .spawn((observer, crate::ScriptObserverOwner(attachment)))
                .id()
        })
    }

    fn register_script_export(
        &self,
        namespace: &str,
        name: &str,
        label: String,
        owner: ScriptAttachment,
    ) -> Result<(), InteropError> {
        validate_script_namespace_and_name(namespace, name)?;
        self.component_registry()
            .write()
            .register_export(crate::ScriptExport {
                name: name.to_owned(),
                namespace: namespace.to_owned(),
                owner,
                label,
            })
    }

    fn call_script_export(
        &self,
        path: &str,
        args: Vec<ScriptValue>,
    ) -> Result<ScriptValue, InteropError> {
        // looked up on every call, so replaced exports are picked up and removed ones fail
        let export = self
            .component_registry()
            .read()
            .get_export(path)
            .cloned()
            .ok_or_else(|| {
                InteropError::string(format!(
                    "the exported function `{path}` does not exist (anymore): the script that exported it may have been unloaded"
                ))
            })?;
        let runners = self.script_callback_runners().clone();
        let recipients = Recipients::from(export.owner.clone());
        let results = runners.run(self.clone(), &recipients, &export.label, args)?;
        match results.into_iter().next() {
            Some((_, Ok(value))) => Ok(value),
            Some((_, Err(e))) if e.is_context_busy() => Err(InteropError::string(format!(
                "cannot call the exported function `{path}` right now: the script {} that exports it is currently running. \
                Exported functions cannot be called by a script from inside a callback of the same script context (including scripts sharing it), \
                call the function directly instead, or use `queue_script_callback`.",
                export.owner
            ))),
            Some((_, Err(e))) => Err(e),
            None => Err(InteropError::string(format!(
                "cannot call the exported function `{path}`: the script {} that exports it is not loaded",
                export.owner
            ))),
        }
    }

    fn register_script_event(
        &self,
        namespace: &str,
        name: &str,
        allow_existing: bool,
    ) -> Result<ScriptEventRegistration, InteropError> {
        validate_script_namespace_and_name(namespace, name)?;
        let path = script_path(namespace, name);
        let component_registry = self.component_registry();
        let mut registry = component_registry.write();
        if let Some(existing) = registry.get_event(&path) {
            if allow_existing {
                return Ok(existing.clone());
            }
            return Err(InteropError::unsupported_operation(
                None,
                None,
                format!(
                    "script registered event '{path}' already exists, pass `allow_existing = true` to get the existing registration instead"
                ),
            ));
        }
        if registry.get(&path).is_some() {
            return Err(InteropError::unsupported_operation(
                None,
                None,
                format!("a script registered component named '{path}' already exists"),
            ));
        }
        if registry.get_export(&path).is_some() {
            return Err(InteropError::unsupported_operation(
                None,
                None,
                format!("a function named '{path}' is already exported"),
            ));
        }
        let registration = ScriptEventRegistration { name: path.into() };
        registry.register_event(registration.clone());
        Ok(registration)
    }

    fn add_script_event_observer(
        &self,
        event: ScriptEventRegistration,
        label: String,
        attachment: ScriptAttachment,
    ) -> Result<Entity, InteropError> {
        let name = event.name;
        let runners = self.script_callback_runners().clone();
        let recipients = Recipients::from(attachment.clone());
        let observer = Observer::new(move |event: On<ScriptEvent>, mut world: DeferredWorld| {
            if event.name != name {
                return;
            }
            run_observer_callback(
                &mut world,
                &runners,
                &recipients,
                &label,
                event.payload.clone(),
            );
        });
        // removed again when the script is unloaded
        self.with_world_mut(|world| {
            world
                .spawn((observer, crate::ScriptObserverOwner(attachment)))
                .id()
        })
    }

    fn trigger_script_event(
        &self,
        event: ScriptEventRegistration,
        payload: ScriptValue,
    ) -> Result<(), InteropError> {
        self.with_world_mut(|world| {
            world.trigger(ScriptEvent {
                name: event.name,
                payload,
            });
            // apply the commands of observers, i.e. the callbacks of script observers
            world.flush();
        })
    }

    fn write_message(
        &self,
        registration: ScriptTypeRegistration,
        message: &ReflectReference,
    ) -> Result<(), InteropError> {
        let reflect_message = registration
            .type_registration()
            .data::<ReflectMessage>()
            .ok_or_else(|| {
                InteropError::missing_type_data(registration.type_id(), "ReflectMessage".to_owned())
            })?
            .clone();
        let value = message.to_owned_value_of_type(self.clone(), registration.type_id())?;
        let registry = self.type_registry().clone();
        self.with_world_mut(|world| {
            let registry = registry.read();
            reflect_message.write_message(world, value.as_ref(), &registry);
        })
    }

    fn with_world<O, F: FnOnce(&World) -> O>(&self, f: F) -> Result<O, InteropError> {
        self.with_world_access(f).map_err(Into::into)
    }

    fn with_world_mut<O, F: FnOnce(&mut World) -> O>(&self, f: F) -> Result<O, InteropError> {
        self.with_world_mut_access(f).map_err(Into::into)
    }

    fn get_parent(&self, entity: Entity) -> Result<Option<Entity>, InteropError> {
        if !self.is_valid_entity(entity)? {
            return Err(InteropError::missing_entity(entity));
        }

        self.with_component(entity, |c: Option<&ChildOf>| c.map(|c| c.parent()))
    }

    fn get_children(&self, entity: Entity) -> Result<Vec<Entity>, InteropError> {
        if !self.is_valid_entity(entity)? {
            return Err(InteropError::missing_entity(entity));
        }

        self.with_component(entity, |c: Option<&Children>| {
            c.map(|c| c.to_vec()).unwrap_or_default()
        })
    }

    fn push_children(&self, parent: Entity, children: &[Entity]) -> Result<(), InteropError> {
        // verify entities exist
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }
        for c in children {
            if !self.is_valid_entity(*c)? {
                return Err(InteropError::missing_entity(*c));
            }
        }
        self.with_world_mut(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).add_children(children);
            queue.apply(world);
        })
    }

    fn insert_children(
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

        self.with_world_mut(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).insert_children(index, children);
            queue.apply(world);
        })
    }

    fn remove_children(&self, parent: Entity, children: &[Entity]) -> Result<(), InteropError> {
        if !self.is_valid_entity(parent)? {
            return Err(InteropError::missing_entity(parent));
        }

        for c in children {
            if !self.is_valid_entity(*c)? {
                return Err(InteropError::missing_entity(*c));
            }
        }
        self.with_world_mut(|world| {
            let mut queue = CommandQueue::default();
            let mut commands = Commands::new(&mut queue, world);
            commands.entity(parent).detach_children(children);
            queue.apply(world);
        })
    }

    fn exit(&self) -> Result<(), InteropError> {
        self.with_world_mut(|world| {
            world.write_message(AppExit::Success);
        })
    }

    fn get_schedule_by_name(&self, schedule_name: String) -> Option<ReflectSchedule> {
        let schedule_registry = self.schedule_registry();
        let schedule_registry = schedule_registry.read();

        schedule_registry
            .get_schedule_by_name(&schedule_name)
            .cloned()
    }

    fn load_script_asset<'a>(
        &self,
        asset_path: impl Into<AssetPath<'a>>,
    ) -> Result<Handle<ScriptAsset>, InteropError> {
        self.with_resource(|r: &AssetServer| r.load(asset_path))
    }

    fn get_script_asset_load_state(
        &self,
        script: Handle<ScriptAsset>,
    ) -> Result<LoadState, InteropError> {
        self.with_resource(|r: &AssetServer| r.load_state(script.id()))
    }

    fn call_operator(
        &self,
        type_id: TypeId,
        name: impl Into<Cow<'static, str>>,
        args: Vec<ScriptValue>,
        context: FunctionCallContext,
    ) -> Result<ScriptValue, InteropError> {
        let registry = self.script_function_registry();
        let registry = registry.read();

        // operators not defined on the type itself fall back to the generic ones on `ReflectReference`, i.e. `eq`
        let func = registry
            .get_function(type_id, name)
            .or_else(|name| registry.get_function(TypeId::of::<ReflectReference>(), name));
        match func {
            Ok(func) => func.call(args, context),
            Err(name) => Err(InteropError::missing_function(
                name.to_string(),
                FunctionScope::OnType(type_id),
                Some(context),
            )),
        }
    }

    fn get_functions_on_type(
        &self,
        type_id: TypeId,
    ) -> Vec<(Cow<'static, str>, DynamicScriptFunction)> {
        let registry = self.script_function_registry();
        let registry = registry.read();

        registry
            .iter_type(type_id)
            .chain(registry.iter_type(TypeId::of::<ReflectReference>()))
            .map(|(key, func)| (key.name.clone(), func.clone()))
            .collect()
    }

    fn lookup_member<'n>(
        &self,
        reference: &ReflectReference,
        type_id: TypeId,
        name: impl Into<Cow<'n, str>>,
    ) -> Result<ScriptValue, Cow<'n, str>> {
        if let Some(registered) = registered_type_id(self, reference, type_id) {
            // the static members of the type come before the functions shared by all references,
            // i.e. `PlaybackSettings.remove()` is not the generic `remove`
            let name = match self.lookup_function([type_id], name) {
                Ok(func) => return Ok(ScriptValue::Function(func)),
                Err(name) => name,
            };
            let name = match self.lookup_static_member(registered, name) {
                Ok(member) => return Ok(member),
                Err(name) => name,
            };
            return self
                .lookup_function([TypeId::of::<ReflectReference>()], name)
                .map(ScriptValue::Function);
        }
        let name = match self.lookup_instance_function(type_id, name) {
            Ok(func) => return Ok(ScriptValue::Function(func)),
            Err(name) => name,
        };
        if type_id == TypeId::of::<EnumVariantRef>() {
            return match reference.downcast::<EnumVariantRef>(self.clone()) {
                Ok(variant) => self
                    .lookup_function([variant.type_id], name)
                    .map(ScriptValue::Function),
                Err(_) => Err(name),
            };
        }
        Err(name)
    }

    fn lookup_static_member<'n>(
        &self,
        type_id: TypeId,
        name: impl Into<Cow<'n, str>>,
    ) -> Result<ScriptValue, Cow<'n, str>> {
        let name = match self.lookup_type_function(type_id, name) {
            Ok(func) => return Ok(ScriptValue::Function(func)),
            Err(name) => name,
        };
        let Ok(type_info) = registered_type_info(self, type_id) else {
            return Err(name);
        };
        match EnumVariantRef::from_name(type_info, &name) {
            Some(variant) => V(variant).into_script(self.clone()).map_err(|_| name),
            None => Err(name),
        }
    }

    fn get_type_by_name(&self, type_name: &str) -> Option<ScriptTypeRegistration> {
        let type_registry = self.type_registry();
        let type_registry = type_registry.read();
        type_registry
            .get_with_type_path(type_name)
            .map(|registration| ScriptTypeRegistration::new(Arc::new(registration.clone())))
    }

    fn get_type_registration(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<AnyTypeRegistration, InteropError> {
        let registration = match self.get_resource_type(registration)? {
            Ok(res) => {
                return Ok(Union::new_right(Union::new_right(Union::new_left(res))));
            }
            Err(registration) => registration,
        };

        let registration = match self.get_component_type(registration)? {
            Ok(comp) => {
                return Ok(Union::new_right(Union::new_left(comp)));
            }
            Err(registration) => registration,
        };

        let registration = match self.get_bundle_type(registration)? {
            Ok(bundle) => {
                return Ok(Union::new_right(Union::new_right(Union::new_right(bundle))));
            }
            Err(registration) => registration,
        };

        Ok(Union::new_left(registration))
    }

    fn get_type_registration_by_name(
        &self,
        type_name: String,
    ) -> Result<Option<AnyTypeRegistration>, InteropError> {
        let val = self.get_type_by_name(&type_name);
        Ok(match val {
            Some(registration) => Some(self.get_type_registration(registration)?),
            None => {
                // try the script registered components
                let components = self.component_registry();
                let components = components.read();
                // by full path (i.e. `script.game.Enemy`)
                components
                    .find(&type_name)
                    .map(|c| Union::new_right(Union::new_left(c.registration.clone())))
            }
        })
    }

    fn get_resource_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptResourceRegistration, ScriptTypeRegistration>, InteropError> {
        Ok(match self.get_resource_id(registration.type_id())? {
            Some(resource_id) => Ok(ScriptResourceRegistration::new(registration, resource_id)),
            None => Err(registration),
        })
    }

    fn get_component_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptComponentRegistration, ScriptTypeRegistration>, InteropError> {
        Ok(match self.get_component_id(registration.type_id())? {
            Some(comp_id) => Ok(ScriptComponentRegistration::new(registration, comp_id)),
            None => Err(registration),
        })
    }

    fn get_bundle_type(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<Result<ScriptBundleRegistration, ScriptTypeRegistration>, InteropError> {
        if registration
            .type_registration()
            .data::<ReflectBundle>()
            .is_none()
        {
            return Ok(Err(registration));
        }
        Ok(Ok(ScriptBundleRegistration::new(registration)))
    }

    fn script_function_registry(&'_ self) -> Ref<'_, AppScriptFunctionRegistry> {
        #[allow(
            clippy::unwrap_used,
            reason = "internal domain boundary, enforced at creation of the guard"
        )]
        self.get_cached_registry().unwrap()
    }

    fn script_callback_runners(&'_ self) -> Ref<'_, AppScriptCallbackRunners> {
        #[allow(
            clippy::unwrap_used,
            reason = "internal domain boundary, enforced at creation of the guard"
        )]
        self.get_cached_registry().unwrap()
    }

    fn type_catalogue(&'_ self) -> Ref<'_, AppScriptTypeCatalogue> {
        #[allow(
            clippy::unwrap_used,
            reason = "internal domain boundary, enforced at creation of the guard"
        )]
        self.get_cached_registry().unwrap()
    }

    fn component_registry(&'_ self) -> Ref<'_, AppScriptComponentRegistry> {
        #[allow(
            clippy::unwrap_used,
            reason = "internal domain boundary, enforced at creation of the guard"
        )]
        self.get_cached_registry().unwrap()
    }

    fn schedule_registry(&'_ self) -> Ref<'_, AppScheduleRegistry> {
        #[allow(
            clippy::unwrap_used,
            reason = "internal domain boundary, enforced at creation of the guard"
        )]
        self.get_cached_registry().unwrap()
    }

    fn current_attachment(&self) -> CurrentScriptAttachment {
        self.get_cached_registry::<CurrentScriptAttachment>()
            .map(|r| r.clone())
            .unwrap_or_else(CurrentScriptAttachment::empty)
    }

    fn set_current_attachment(&self, attachment: ScriptAttachment) {
        self.set_cached_registry::<CurrentScriptAttachment>(CurrentScriptAttachment(Some(
            attachment,
        )));
    }

    fn register_script_component(
        &self,
        namespace: &str,
        component_name: &str,
        allow_existing: bool,
    ) -> Result<ScriptComponentRegistration, InteropError> {
        validate_script_namespace_and_name(namespace, component_name)?;
        let path = script_path(namespace, component_name);
        let component_registry = self.component_registry();
        let component_registry_read = component_registry.read();
        if let Some(existing) = component_registry_read.get(&path) {
            if allow_existing {
                return Ok(existing.registration.clone());
            }
            return Err(InteropError::unsupported_operation(
                None,
                None,
                format!(
                    "script registered component '{path}' already exists, pass `allow_existing = true` to get the existing registration instead"
                ),
            ));
        }
        if component_registry_read.get_export(&path).is_some() {
            return Err(InteropError::unsupported_operation(
                None,
                None,
                format!("a function named '{path}' is already exported"),
            ));
        }
        if component_registry_read.get_event(&path).is_some() {
            return Err(InteropError::unsupported_operation(
                None,
                None,
                format!("a script registered event named '{path}' already exists"),
            ));
        }

        let component_id = self.with_world_mut_access(|w| {
            let descriptor = unsafe {
                // Safety: same safety guarantees as ComponentDescriptor::new
                // we know the type in advance
                // we only use this method to name the component
                ComponentDescriptor::new_with_layout(
                    path.clone(),
                    DynamicComponent::STORAGE_TYPE,
                    Layout::new::<DynamicComponent>(),
                    needs_drop::<DynamicComponent>().then_some(|x| x.drop_as::<DynamicComponent>()),
                    true,
                    false,
                    ComponentCloneBehavior::Default,
                    None,
                )
            };
            w.register_component_with_descriptor(descriptor)
        })?;
        drop(component_registry_read);
        let mut component_registry = component_registry.write();

        let mut registration = ScriptComponentRegistration::new(
            ScriptTypeRegistration::new(Arc::new(
                <DynamicComponent as GetTypeRegistration>::get_type_registration(),
            )),
            component_id,
        );
        registration.script_namespace = Some(namespace.to_owned());

        let component_info = DynamicComponentInfo {
            name: component_name.to_owned(),
            namespace: namespace.to_owned(),
            registration: registration.clone(),
        };

        component_registry.register(component_info);

        // TODO: we should probably retrieve this from the registry, but I don't see what people would want to register on this type
        // in addition to the existing registrations.
        Ok(registration)
    }

    fn setup_cache_raw(
        attachment: CurrentScriptAttachment,
        function_registry: AppScriptFunctionRegistry,
        schedule_registry: AppScheduleRegistry,
        component_registry: AppScriptComponentRegistry,
        callback_runners: AppScriptCallbackRunners,
        type_catalogue: AppScriptTypeCatalogue,
    ) -> RegistryCache {
        debug_assert_eq!(AppScriptFunctionRegistry::SLOT, 0);
        debug_assert_eq!(AppScheduleRegistry::SLOT, 1);
        debug_assert_eq!(AppScriptComponentRegistry::SLOT, 2);
        debug_assert_eq!(CurrentScriptAttachment::SLOT, 3);
        debug_assert_eq!(AppScriptCallbackRunners::SLOT, 4);
        debug_assert_eq!(AppScriptTypeCatalogue::SLOT, 5);

        [
            Rc::new(RefCell::new(function_registry)),
            Rc::new(RefCell::new(schedule_registry)),
            Rc::new(RefCell::new(component_registry)),
            Rc::new(RefCell::new(attachment)),
            Rc::new(RefCell::new(callback_runners)),
            Rc::new(RefCell::new(type_catalogue)),
        ]
    }

    fn setup_cache(world: &World, attachment: CurrentScriptAttachment) -> RegistryCache {
        debug_assert_eq!(AppScriptFunctionRegistry::SLOT, 0);
        debug_assert_eq!(AppScheduleRegistry::SLOT, 1);
        debug_assert_eq!(AppScriptComponentRegistry::SLOT, 2);
        debug_assert_eq!(CurrentScriptAttachment::SLOT, 3);
        debug_assert_eq!(AppScriptCallbackRunners::SLOT, 4);
        debug_assert_eq!(AppScriptTypeCatalogue::SLOT, 5);

        [
            Rc::new(RefCell::new(
                world
                    .get_resource::<AppScriptFunctionRegistry>()
                    .cloned()
                    .unwrap_or_default(),
            )),
            Rc::new(RefCell::new(
                world
                    .get_resource::<AppScheduleRegistry>()
                    .cloned()
                    .unwrap_or_default(),
            )),
            Rc::new(RefCell::new(
                world
                    .get_resource::<AppScriptComponentRegistry>()
                    .cloned()
                    .unwrap_or_default(),
            )),
            Rc::new(RefCell::new(attachment)),
            Rc::new(RefCell::new(
                world
                    .get_resource::<AppScriptCallbackRunners>()
                    .cloned()
                    .unwrap_or_default(),
            )),
            Rc::new(RefCell::new(
                world
                    .get_resource::<AppScriptTypeCatalogue>()
                    .cloned()
                    .unwrap_or_default(),
            )),
        ]
    }

    // /// Creates a system from a system builder and inserts it into the given schedule
    // pub fn add_system<P: IntoScriptPluginParams>(
    //     &self,
    //     schedule: &ReflectSchedule,
    //     builder: ScriptSystemBuilder,
    // ) -> Result<ReflectSystem, InteropError> {
    //     debug!(
    //         "Adding script system '{}' for script '{}' to schedule '{}'",
    //         builder.name,
    //         builder.attachment,
    //         schedule.identifier()
    //     );

    //     builder.build::<P>(self.clone(), schedule)
    // }
}

/// Function lookups used by [`WorldExtensions`] and [`crate::dispatch`], not exposed to language implementations.
pub(crate) trait FunctionLookups {
    /// Looks up a function by name across multiple type namespaces.
    fn lookup_function<'n>(
        &self,
        type_ids: impl IntoIterator<Item = TypeId>,
        name: impl Into<Cow<'n, str>>,
    ) -> Result<DynamicScriptFunction, Cow<'n, str>>;

    /// Looks up a function callable on an instance of the given type, i.e. `value:name(..)` in a script.
    ///
    /// Resolution order:
    /// 1. functions registered on the type itself
    /// 2. functions registered on [`ReflectReference`], which apply to all instances
    ///
    /// Returns the name back if nothing was found.
    fn lookup_instance_function<'n>(
        &self,
        type_id: TypeId,
        name: impl Into<Cow<'n, str>>,
    ) -> Result<DynamicScriptFunction, Cow<'n, str>>;

    /// Looks up a reserved function (see the "Reserved Functions" section of the book) on the `ReflectReference` namespace,
    /// ignoring functions registered on the types themselves.
    ///
    /// Used for functions languages rely on and which types cannot override, such as `display` and `iter`.
    fn lookup_reserved_function(
        &self,
        name: impl Into<Cow<'static, str>>,
        context: FunctionCallContext,
    ) -> Result<DynamicScriptFunction, InteropError>;

    /// Looks up a function callable on the static reference to a type, i.e. `Type.name(..)` in a script.
    ///
    /// Resolution order:
    /// 1. functions registered on the type itself
    /// 2. type level functions registered on [`StaticTypeRef`], with the type bound as their first argument
    ///
    /// Returns the name back if nothing was found.
    fn lookup_type_function<'n>(
        &self,
        type_id: TypeId,
        name: impl Into<Cow<'n, str>>,
    ) -> Result<DynamicScriptFunction, Cow<'n, str>>;
}

impl FunctionLookups for WorldAccessGuard<'_> {
    fn lookup_function<'n>(
        &self,
        type_ids: impl IntoIterator<Item = TypeId>,
        name: impl Into<Cow<'n, str>>,
    ) -> Result<DynamicScriptFunction, Cow<'n, str>> {
        let registry = self.script_function_registry();
        let registry = registry.read();

        let mut name = name.into();
        for type_id in type_ids {
            name = match registry.get_function(type_id, name) {
                Ok(func) => return Ok(func.clone()),
                Err(name) => name,
            };
        }

        Err(name)
    }

    fn lookup_instance_function<'n>(
        &self,
        type_id: TypeId,
        name: impl Into<Cow<'n, str>>,
    ) -> Result<DynamicScriptFunction, Cow<'n, str>> {
        self.lookup_function([type_id, TypeId::of::<ReflectReference>()], name)
    }

    fn lookup_reserved_function(
        &self,
        name: impl Into<Cow<'static, str>>,
        context: FunctionCallContext,
    ) -> Result<DynamicScriptFunction, InteropError> {
        self.lookup_function([TypeId::of::<ReflectReference>()], name)
            .map_err(|name| {
                InteropError::missing_function(name, FunctionScope::all_references(), Some(context))
            })
    }

    fn lookup_type_function<'n>(
        &self,
        type_id: TypeId,
        name: impl Into<Cow<'n, str>>,
    ) -> Result<DynamicScriptFunction, Cow<'n, str>> {
        let name = match self.lookup_function([type_id], name) {
            Ok(func) => return Ok(func),
            Err(name) => name,
        };
        match registered_type_info(self, type_id) {
            Ok(type_info) => lookup_bound_type_function(self, type_info, name),
            Err(_) => Err(name),
        }
    }
}

/// Delivers an event to the callback of a script observer through the callback runners.
///
/// Runs once the commands of the triggering operation are applied, see [`AppScriptCallbackRunners::run_or_queue`].
/// Errors are reported through [`AppScriptCallbackRunners::report_error`].
fn run_observer_callback(
    world: &mut DeferredWorld,
    runners: &AppScriptCallbackRunners,
    recipients: &Recipients,
    label: &str,
    arg: ScriptValue,
) {
    let runners = runners.clone();
    let recipients = recipients.clone();
    let label = label.to_owned();
    world.commands().queue(move |world: &mut World| {
        let cache = WorldAccessGuard::setup_cache(world, CurrentScriptAttachment::empty());
        WorldGuard::with_static_guard(world, cache, |guard| {
            let context = || format!("in observer callback: {label}");
            match runners.run_or_queue(guard.clone(), &recipients, &label, vec![arg]) {
                Ok(results) => {
                    for (attachment, result) in results {
                        if let Err(e) = result {
                            runners.report_error(guard.clone(), Some(&attachment), context(), e);
                        }
                    }
                }
                Err(e) => runners.report_error(guard, None, context(), e),
            }
        })
    });
}

/// The type a type registration points to, if `type_id` (the type of the value behind `reference`) is one of the registration types.
fn registered_type_id(
    guard: &WorldAccessGuard,
    reference: &ReflectReference,
    type_id: TypeId,
) -> Option<TypeId> {
    fn registered<T: Reflect>(
        guard: &WorldAccessGuard,
        reference: &ReflectReference,
        registered: fn(&T) -> TypeId,
    ) -> Option<TypeId> {
        reference
            .with_reflect(guard.clone(), |r| r.try_downcast_ref::<T>().map(registered))
            .ok()
            .flatten()
    }
    if type_id == TypeId::of::<ScriptTypeRegistration>() {
        registered(guard, reference, ScriptTypeRegistration::type_id)
    } else if type_id == TypeId::of::<ScriptComponentRegistration>() {
        registered(guard, reference, |r: &ScriptComponentRegistration| {
            r.type_registration().type_id()
        })
    } else if type_id == TypeId::of::<ScriptResourceRegistration>() {
        registered(guard, reference, |r: &ScriptResourceRegistration| {
            r.type_registration().type_id()
        })
    } else if type_id == TypeId::of::<ScriptBundleRegistration>() {
        registered(guard, reference, |r: &ScriptBundleRegistration| {
            r.type_registration().type_id()
        })
    } else {
        None
    }
}

/// A type level function registered on [`StaticTypeRef`], bound to the given type.
fn lookup_bound_type_function<'n>(
    guard: &WorldAccessGuard,
    type_info: &'static TypeInfo,
    name: Cow<'n, str>,
) -> Result<DynamicScriptFunction, Cow<'n, str>> {
    // the type level functions of `StaticTypeRef` are already addressed at that type directly
    if type_info.type_id() == TypeId::of::<StaticTypeRef>() {
        return Err(name);
    }
    let registry = guard.script_function_registry();
    let registry = registry.read();
    registry.get_bound_to_type(type_info, name)
}

impl CachedRegistry for AppScriptFunctionRegistry {
    const SLOT: usize = 0;
}
impl CachedRegistry for AppScheduleRegistry {
    const SLOT: usize = 1;
}
impl CachedRegistry for AppScriptComponentRegistry {
    const SLOT: usize = 2;
}

/// The script whose callback is currently running, cached in each world guard.
///
/// [`None`] when no script is running, see [`Self::empty`].
#[derive(Clone, Default)]
pub struct CurrentScriptAttachment(pub Option<ScriptAttachment>);

impl CurrentScriptAttachment {
    /// No script is running.
    ///
    /// Use this when creating a guard which is not tied to one script, e.g. in a system running callbacks on many scripts.
    /// Each callback marks its own script as current for the duration of the call (see `ScriptingHandler::handle` in core).
    pub fn empty() -> Self {
        Self(None)
    }
}
impl CachedRegistry for CurrentScriptAttachment {
    const SLOT: usize = 3;
}
impl CachedRegistry for AppScriptCallbackRunners {
    const SLOT: usize = 4;
}
impl CachedRegistry for AppScriptTypeCatalogue {
    const SLOT: usize = 5;
}

/// Default constructs an instance of the given type, using its `ReflectDefault` or `ReflectFromWorld` type data.
pub(crate) fn default_instance(
    world: WorldGuard,
    registration: &TypeRegistration,
) -> Result<Box<dyn Reflect>, InteropError> {
    if let Some(default_td) = registration.data::<ReflectDefault>() {
        Ok(default_td.default())
    } else if let Some(from_world_td) = registration.data::<ReflectFromWorld>() {
        world.with_world_mut(|world| from_world_td.from_world(world))
    } else {
        Err(InteropError::missing_type_data(
            registration.type_id(),
            "ReflectDefault or ReflectFromWorld".to_owned(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::function::from::R;
    use bevy_reflect::Reflect;
    use std::sync::Arc;
    use test_utils::test_data::{
        CompWithDefaultAndComponentData, GetTestEntityId, SimpleEnum, SimpleStruct,
        SimpleTupleStruct, TestBundle, TestNestedBundle, TestResource, UnitStruct, setup_world,
    };

    #[test]
    fn lookup_type_function_prefers_own_functions_and_binds_shared_ones() {
        use crate::function::namespace::NamespaceBuilder;

        let mut world = setup_world(|_, _| {});
        NamespaceBuilder::<StaticTypeRef>::new_unregistered(&mut world)
            .register("describe", |_: StaticTypeRef| 1usize);
        NamespaceBuilder::<SimpleStruct>::new_unregistered(&mut world)
            .register("describe", || 2usize);

        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        let world = WorldAccessGuard::new_exclusive(&mut world, cache);

        // shared functions have the type bound
        let shared = world
            .lookup_type_function(TypeId::of::<SimpleTupleStruct>(), "describe")
            .unwrap();
        assert!(shared.info.arg_info.is_empty());
        assert_eq!(
            shared.info.namespace,
            FunctionScope::OnType(TypeId::of::<SimpleTupleStruct>())
        );

        // functions on the type itself take priority
        let own = world
            .lookup_type_function(TypeId::of::<SimpleStruct>(), "describe")
            .unwrap();
        assert_eq!(
            own.info.namespace,
            FunctionScope::OnType(TypeId::of::<SimpleStruct>())
        );

        assert!(
            world
                .lookup_type_function(TypeId::of::<SimpleStruct>(), "missing")
                .is_err()
        );
    }

    #[test]
    fn lookup_member_reaches_static_members_through_registrations_and_enum_functions_through_variants()
     {
        use crate::function::namespace::NamespaceBuilder;

        let mut world = setup_world(|_, _| {});
        NamespaceBuilder::<SimpleStruct>::new_unregistered(&mut world)
            .register("describe", || 2usize);
        NamespaceBuilder::<SimpleEnum>::new_unregistered(&mut world)
            .register("describe", |_: R<SimpleEnum>| 3usize);

        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        let world = WorldAccessGuard::new_exclusive(&mut world, cache);
        let registration_of = |type_id: TypeId| {
            let registration = world.type_registry().read().get(type_id).unwrap().clone();
            let registration = ScriptTypeRegistration::new(Arc::new(registration));
            ReflectReference::new_allocated(registration)
        };

        // a registration, i.e. an entry of `import_namespace`, gives access to the static functions of its type
        let registration = registration_of(TypeId::of::<SimpleStruct>());
        let registration_type = TypeId::of::<ScriptTypeRegistration>();
        let Ok(ScriptValue::Function(function)) =
            world.lookup_member(&registration, registration_type, "describe")
        else {
            panic!("expected the static function of the registered type")
        };
        assert_eq!(
            function.info.namespace,
            FunctionScope::OnType(TypeId::of::<SimpleStruct>())
        );
        // the functions of the registration itself still come first
        assert!(
            world
                .lookup_member(&registration, registration_type, "missing")
                .is_err()
        );

        // and to its enum variants
        let enum_registration = registration_of(TypeId::of::<SimpleEnum>());
        let Ok(ScriptValue::Reference(variant)) =
            world.lookup_member(&enum_registration, registration_type, "Unit")
        else {
            panic!("expected the variant of the registered enum")
        };

        // a variant gives access to the functions of its enum
        let Ok(ScriptValue::Function(function)) =
            world.lookup_member(&variant, TypeId::of::<EnumVariantRef>(), "describe")
        else {
            panic!("expected the function of the enum")
        };
        assert_eq!(
            function.info.namespace,
            FunctionScope::OnType(TypeId::of::<SimpleEnum>())
        );
    }

    #[test]
    fn lookup_static_member_finds_variants_after_functions() {
        use crate::function::namespace::NamespaceBuilder;

        let mut world = setup_world(|_, _| {});
        NamespaceBuilder::<SimpleEnum>::new_unregistered(&mut world).register("Struct", || 1usize);

        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        let world = WorldAccessGuard::new_exclusive(&mut world, cache);
        let enum_id = TypeId::of::<SimpleEnum>();

        // a variant is a reference to a variant ref value
        assert!(matches!(
            world.lookup_static_member(enum_id, "Unit"),
            Ok(ScriptValue::Reference(_))
        ));

        // functions take priority
        assert!(matches!(
            world.lookup_static_member(enum_id, "Struct"),
            Ok(ScriptValue::Function(_))
        ));

        assert!(world.lookup_static_member(enum_id, "Missing").is_err());
        // not an enum
        assert!(
            world
                .lookup_static_member(TypeId::of::<SimpleStruct>(), "Unit")
                .is_err()
        );
    }
    fn make_guard(world: &mut World) -> WorldAccessGuard<'_> {
        let cache = WorldAccessGuard::setup_cache(world, CurrentScriptAttachment::empty());
        WorldAccessGuard::new_exclusive(world, cache)
    }

    fn comp_reg<C: Component + Reflect>(
        guard: &WorldAccessGuard<'_>,
    ) -> ScriptComponentRegistration {
        guard
            .get_component_type(guard.get_type_by_name(std::any::type_name::<C>()).unwrap())
            .unwrap()
            .unwrap()
    }

    fn res_reg<R: Resource + Reflect>(guard: &WorldAccessGuard<'_>) -> ScriptResourceRegistration {
        guard
            .get_resource_type(guard.get_type_by_name(std::any::type_name::<R>()).unwrap())
            .unwrap()
            .unwrap()
    }

    // ── spawn / is_valid_entity / has_entity ──────────────────────────────────

    #[test]
    fn spawn_produces_valid_entity() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let e = guard.spawn().unwrap();
        assert!(guard.is_valid_entity(e).unwrap());
        assert_eq!(
            guard.is_valid_entity(e).unwrap(),
            guard.has_entity(e).unwrap()
        );
    }

    // ── despawn ───────────────────────────────────────────────────────────────

    #[test]
    fn despawn_invalidates_entity() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let e = guard.spawn().unwrap();
        guard.despawn(e).unwrap();
        assert!(!guard.is_valid_entity(e).unwrap());
    }

    #[test]
    fn despawn_missing_entity_errors() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let e = guard.spawn().unwrap();
        guard.despawn(e).unwrap();
        assert!(guard.despawn(e).is_err());
    }

    // ── despawn_recursive / despawn_descendants ───────────────────────────────

    #[test]
    fn despawn_recursive_removes_parent_and_child() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let parent = guard.spawn().unwrap();
        let child = guard.spawn().unwrap();
        guard.push_children(parent, &[child]).unwrap();
        guard.despawn_recursive(parent).unwrap();
        assert!(!guard.is_valid_entity(parent).unwrap());
        assert!(!guard.is_valid_entity(child).unwrap());
    }

    #[test]
    fn despawn_descendants_keeps_parent_removes_child() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let parent = guard.spawn().unwrap();
        let child = guard.spawn().unwrap();
        guard.push_children(parent, &[child]).unwrap();
        guard.despawn_descendants(parent).unwrap();
        assert!(guard.is_valid_entity(parent).unwrap());
        assert!(!guard.is_valid_entity(child).unwrap());
    }

    // ── component CRUD ────────────────────────────────────────────────────────

    #[test]
    fn add_default_has_get_remove_component_roundtrip() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let e = guard.spawn().unwrap();
        let reg = comp_reg::<CompWithDefaultAndComponentData>(&guard);

        assert!(!guard.has_component(e, reg.component_id).unwrap());
        assert!(guard.get_component(e, reg.clone()).unwrap().is_none());

        guard.add_default_component(e, reg.clone()).unwrap();

        assert!(guard.has_component(e, reg.component_id).unwrap());
        assert!(guard.get_component(e, reg.clone()).unwrap().is_some());

        guard.remove_component(e, reg.clone()).unwrap();

        assert!(!guard.has_component(e, reg.component_id).unwrap());
    }

    // ── with_component access-map: all four conflict cases ────────────────────

    #[test]
    fn with_component_read_read_succeeds() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let e = guard.spawn().unwrap();
        guard
            .add_default_component(e, comp_reg::<CompWithDefaultAndComponentData>(&guard))
            .unwrap();

        // two shared borrows of the same component must not conflict
        let result = guard.with_component(e, |_: Option<&CompWithDefaultAndComponentData>| {
            guard.with_component(e, |_: Option<&CompWithDefaultAndComponentData>| ())
        });
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn with_component_write_read_conflicts() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let e = guard.spawn().unwrap();
        guard
            .add_default_component(e, comp_reg::<CompWithDefaultAndComponentData>(&guard))
            .unwrap();

        let result =
            guard.with_component_mut(e, |_: Option<Mut<CompWithDefaultAndComponentData>>| {
                guard.with_component(e, |_: Option<&CompWithDefaultAndComponentData>| ())
            });
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn with_component_read_write_conflicts() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let e = guard.spawn().unwrap();
        guard
            .add_default_component(e, comp_reg::<CompWithDefaultAndComponentData>(&guard))
            .unwrap();

        let result = guard.with_component(e, |_: Option<&CompWithDefaultAndComponentData>| {
            guard.with_component_mut(e, |_: Option<Mut<CompWithDefaultAndComponentData>>| ())
        });
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn with_component_write_write_conflicts() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let e = guard.spawn().unwrap();
        guard
            .add_default_component(e, comp_reg::<CompWithDefaultAndComponentData>(&guard))
            .unwrap();

        let result =
            guard.with_component_mut(e, |_: Option<Mut<CompWithDefaultAndComponentData>>| {
                guard.with_component_mut(e, |_: Option<Mut<CompWithDefaultAndComponentData>>| ())
            });
        assert!(result.unwrap().is_err());
    }

    // ── with_or_insert_component_mut ─────────────────────────────────────────

    #[test]
    fn with_or_insert_inserts_when_absent_and_mutates_when_present() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let e = guard.spawn().unwrap();

        // absent: inserts default then applies mutation
        guard
            .with_or_insert_component_mut(e, |c: &mut UnitStruct| {
                let _ = c;
            })
            .unwrap();
        assert!(
            guard
                .has_component(e, comp_reg::<UnitStruct>(&guard).component_id)
                .unwrap()
        );

        // present: mutates existing value
        guard
            .with_or_insert_component_mut(e, |c: &mut UnitStruct| {
                let _ = c;
            })
            .unwrap();
    }

    // ── resource operations ───────────────────────────────────────────────────

    #[test]
    fn has_resource_and_get_resource_and_remove_resource() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let reg = res_reg::<TestResource>(&guard);

        assert!(guard.has_resource(reg.resource_id).unwrap());
        assert!(guard.get_resource(reg.resource_id).unwrap().is_some());

        guard.remove_resource(reg.clone()).unwrap();

        assert!(!guard.has_resource(reg.resource_id).unwrap());
        // this might be unexpected but the resource component ID persists for some reason
        // I guess it gets re-used if the resource is re-inserted, but
        // the reference will fail
        let ref_ = guard.get_resource(reg.resource_id).unwrap().unwrap();
        assert!(ref_.downcast::<TestResource>(guard).is_err());
    }

    // ── with_resource access-map: all four conflict cases ────────────────────

    #[test]
    fn with_resource_read_read_succeeds() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);

        let result =
            guard.with_resource(|_: &TestResource| guard.with_resource(|_: &TestResource| ()));
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn with_resource_write_read_conflicts() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);

        let result = guard
            .with_resource_mut(|_: Mut<TestResource>| guard.with_resource(|_: &TestResource| ()));
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn with_resource_read_write_conflicts() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);

        let result = guard
            .with_resource(|_: &TestResource| guard.with_resource_mut(|_: Mut<TestResource>| ()));
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn with_resource_write_write_conflicts() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);

        let result = guard.with_resource_mut(|_: Mut<TestResource>| {
            guard.with_resource_mut(|_: Mut<TestResource>| ())
        });
        assert!(result.unwrap().is_err());
    }

    // ── hierarchy ─────────────────────────────────────────────────────────────

    #[test]
    fn push_get_remove_children_and_get_parent() {
        let mut world = setup_world(|_, _| {});
        world.register_component::<ChildOf>();
        world.register_component::<Children>();
        let guard = make_guard(&mut world);
        let parent = guard.spawn().unwrap();
        let c1 = guard.spawn().unwrap();
        let c2 = guard.spawn().unwrap();

        assert_eq!(guard.get_parent(c1).unwrap(), None);

        guard.push_children(parent, &[c1, c2]).unwrap();

        assert_eq!(guard.get_children(parent).unwrap().len(), 2);
        assert_eq!(guard.get_parent(c1).unwrap(), Some(parent));

        guard.remove_children(parent, &[c1]).unwrap();
        assert_eq!(guard.get_children(parent).unwrap(), vec![c2]);
    }

    #[test]
    fn insert_children_at_index() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let parent = guard.spawn().unwrap();
        let c1 = guard.spawn().unwrap();
        let c2 = guard.spawn().unwrap();
        let c3 = guard.spawn().unwrap();
        guard.push_children(parent, &[c1, c3]).unwrap();
        guard.insert_children(parent, 1, &[c2]).unwrap();
        assert_eq!(guard.get_children(parent).unwrap()[1], c2);
    }

    #[test]
    fn hierarchy_ops_error_on_missing_entity() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let ghost = guard.spawn().unwrap();
        guard.despawn(ghost).unwrap();

        assert!(guard.get_parent(ghost).is_err());
        assert!(guard.get_children(ghost).is_err());
        assert!(guard.push_children(ghost, &[]).is_err());
        assert!(guard.insert_children(ghost, 0, &[]).is_err());
        assert!(guard.remove_children(ghost, &[]).is_err());
        assert!(guard.despawn_recursive(ghost).is_err());
        assert!(guard.despawn_descendants(ghost).is_err());
    }

    // ── type registry helpers ─────────────────────────────────────────────────

    #[test]
    fn get_type_by_name_known_and_unknown() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        assert!(
            guard
                .get_type_by_name("test_utils::test_data::SimpleStruct")
                .is_some()
        );
        assert!(guard.get_type_by_name("NoSuchType_XYZ").is_none());
    }

    #[test]
    fn get_type_registration_classifies_correctly() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);

        let plain = guard
            .get_type_registration(guard.get_type_by_name("usize").unwrap())
            .unwrap();
        assert!(plain.is_left());

        // component → Right(Left(_))
        let comp = guard
            .get_type_registration(
                guard
                    .get_type_by_name("test_utils::test_data::CompWithDefaultAndComponentData")
                    .unwrap(),
            )
            .unwrap();
        assert!(comp.into_right().unwrap().is_left());

        // resource → Right(Right(_))
        let res = guard
            .get_type_registration(
                guard
                    .get_type_by_name("test_utils::test_data::TestResource")
                    .unwrap(),
            )
            .unwrap();
        assert!(res.into_right().unwrap().is_right());
    }

    #[test]
    fn get_type_registration_by_name_unknown_returns_none() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        assert!(
            guard
                .get_type_registration_by_name("NoSuchType_XYZ".to_owned())
                .unwrap()
                .is_none()
        );
    }

    // ── dynamic component registration ────────────────────────────────────────

    #[test]
    fn register_script_component_and_duplicate_errors() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        guard
            .register_script_component("script.test", "MyDynComp", false)
            .unwrap();
        assert!(
            guard
                .register_script_component("script.test", "MyDynComp", false)
                .is_err()
        );
        assert!(
            guard
                .component_registry()
                .read()
                .get("script.test.MyDynComp")
                .is_some()
        );
    }

    #[test]
    fn type_namespaces_default_to_the_crate_and_can_be_overridden() {
        use crate::{AppScriptTypeCatalogue, ScriptNamespace};

        #[derive(Reflect)]
        struct DefaultNs;
        #[derive(Reflect)]
        struct OverriddenNs;

        let mut world = setup_world(|_, registry| {
            registry.register::<DefaultNs>();
            registry.register::<OverriddenNs>();
            registry.register_type_data_with::<OverriddenNs, ScriptNamespace, _>("custom");
        });
        let guard = make_guard(&mut world);
        let catalogue = AppScriptTypeCatalogue::default()
            .get(guard.clone())
            .unwrap();

        // defined in this crate's test module
        let default = catalogue.get_by_short_name("DefaultNs").unwrap();
        assert_eq!(&*default.namespace, "bevy_mod_scripting_bindings");
        let overridden = catalogue.get_by_short_name("OverriddenNs").unwrap();
        assert_eq!(&*overridden.namespace, "custom");

        assert!(catalogue.namespaces().any(|n| n == "custom"));
        let in_custom: Vec<_> = catalogue.iter_namespace("custom").collect();
        assert_eq!(in_custom.len(), 1);
        assert_eq!(&*in_custom[0].short_name, "OverriddenNs");
        assert_eq!(catalogue.iter_namespace("does_not_exist").count(), 0);
    }

    #[test]
    fn namespace_views_and_suggestions() {
        use crate::{AppScriptTypeCatalogue, ScriptNamespace};

        #[derive(Reflect)]
        struct InTransform;
        #[derive(Reflect)]
        struct InEcs;

        let mut world = setup_world(|_, registry| {
            registry.register::<InTransform>();
            registry.register::<InEcs>();
            registry.register_type_data_with::<InTransform, ScriptNamespace, _>("bevy_transform");
            registry.register_type_data_with::<InEcs, ScriptNamespace, _>("bevy_ecs");
        });
        let guard = make_guard(&mut world);
        let catalogue = AppScriptTypeCatalogue::default().get(guard).unwrap();

        let view = catalogue.namespace_view("bevy_transform").unwrap();
        assert!(view.contains_key("InTransform"));
        assert!(!view.contains_key("InEcs"));
        assert!(catalogue.namespace_view("nope").is_none());

        assert_eq!(
            catalogue.similar_namespaces("bevy_transfrom"),
            vec!["bevy_transform"]
        );
        // substrings count as similar
        assert_eq!(
            catalogue.similar_namespaces("transform"),
            vec!["bevy_transform"]
        );
        assert!(catalogue.similar_namespaces("zzzzzzzzzzzz").is_empty());
    }

    #[test]
    fn first_script_namespace_wins() {
        use crate::{ScriptNamespace, set_script_namespace_if_unset};

        #[derive(Reflect)]
        struct Contested;

        let mut world = setup_world(|_, registry| {
            registry.register::<Contested>();
            set_script_namespace_if_unset::<Contested>(registry, "first");
            set_script_namespace_if_unset::<Contested>(registry, "second");
            // the same namespace again is fine
            set_script_namespace_if_unset::<Contested>(registry, "first");
        });
        let guard = make_guard(&mut world);
        let registry = guard.type_registry();
        let registry = registry.read();
        let namespace = registry
            .get(TypeId::of::<Contested>())
            .and_then(|r| r.data::<ScriptNamespace>())
            .unwrap();
        assert_eq!(namespace.0, "first");
    }

    #[test]
    fn type_catalogue_is_shared_and_refreshes_on_new_types() {
        use crate::AppScriptTypeCatalogue;

        #[derive(Reflect)]
        struct LateType;

        let mut world = setup_world(|_, _| {});
        let registry = world
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        let catalogue = AppScriptTypeCatalogue::default();
        let guard = make_guard(&mut world);

        let first = catalogue.get(guard.clone()).unwrap();
        let again = catalogue.get(guard.clone()).unwrap();
        assert!(
            Arc::ptr_eq(&first, &again),
            "unchanged world must reuse the catalogue"
        );
        assert!(first.get_by_short_name("LateType").is_none());

        registry.write().register::<LateType>();
        let refreshed = catalogue.get(guard).unwrap();
        assert!(!Arc::ptr_eq(&first, &refreshed));
        assert!(refreshed.get_by_short_name("LateType").is_some());
    }

    #[test]
    fn script_registrations_enter_catalogue_and_bump_generation_once() {
        use crate::AppScriptTypeCatalogue;

        let mut world = setup_world(|_, _| {});
        let catalogue = AppScriptTypeCatalogue::default();
        let guard = make_guard(&mut world);
        let generation = || guard.component_registry().read().generation();

        let start = generation();
        guard
            .register_script_component("script.test", "CatalogueComp", false)
            .unwrap();
        assert_eq!(generation(), start + 1);

        // returning an existing registration must not look like a change
        guard
            .register_script_component("script.test", "CatalogueComp", true)
            .unwrap();
        assert_eq!(generation(), start + 1);

        let built = catalogue.get(guard.clone()).unwrap();
        let comp = built.get_by_short_name("CatalogueComp").unwrap();
        assert_eq!(comp.namespace.as_ref(), "script.test");
    }

    #[test]
    fn script_event_observers_run_right_away_or_queue_when_context_is_busy() {
        use bevy_mod_scripting_asset::Language;

        let mut world = setup_world(|_, _| {});
        let runners = AppScriptCallbackRunners::default();
        let ran = Arc::new(parking_lot::Mutex::new(Vec::<String>::new()));
        let r = ran.clone();
        let attachment = ScriptAttachment::StaticScript(Handle::default());
        let a = attachment.clone();
        runners.register(
            Language::Lua,
            Arc::new(move |_, _, label, _| {
                r.lock().push(label.to_owned());
                // pretend the context is busy for one of the callbacks
                Ok(vec![(
                    a.clone(),
                    if label == "busy_cb" {
                        Err(InteropError::context_busy("busy".to_owned()))
                    } else {
                        Ok(ScriptValue::Unit)
                    },
                )])
            }),
        );
        world.insert_resource(runners.clone());

        let guard = make_guard(&mut world);
        let event = guard
            .register_script_event("script.test", "E", false)
            .unwrap();
        for label in ["free_cb", "busy_cb"] {
            guard
                .add_script_event_observer(event.clone(), label.into(), attachment.clone())
                .unwrap();
        }
        guard
            .trigger_script_event(event, ScriptValue::Unit)
            .unwrap();

        // both were attempted right away (in unspecified order), only the busy one was queued
        let mut attempted = ran.lock().clone();
        attempted.sort();
        assert_eq!(attempted, vec!["busy_cb", "free_cb"]);
        assert_eq!(runners.pop_queued().unwrap().label, "busy_cb");
        assert!(!runners.has_queued());
    }

    #[test]
    fn script_registrations_need_valid_script_namespaces() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);

        for bad in [
            "game",
            "bevy_transform",
            "script",
            "script.",
            "script..a",
            "script.a b",
            "",
        ] {
            for result in [
                guard.register_script_component(bad, "C", false).map(|_| ()),
                guard.register_script_event(bad, "E", false).map(|_| ()),
            ] {
                let err = result.unwrap_err().to_string();
                assert!(err.contains("script"), "{bad}: {err}");
            }
        }
        let err = guard
            .register_script_component("game", "C", false)
            .unwrap_err()
            .to_string();
        assert!(err.contains("must start with `script.`"), "{err}");

        // names cannot be empty or contain dots
        assert!(
            guard
                .register_script_component("script.a", "", false)
                .is_err()
        );
        assert!(
            guard
                .register_script_component("script.a", "x.y", false)
                .is_err()
        );
        assert!(
            guard
                .register_script_component("script.a.b_c", "X", false)
                .is_ok()
        );
    }

    #[test]
    fn script_registrations_are_unique_by_full_path() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);

        let a = guard
            .register_script_component("script.one", "Same", false)
            .unwrap();
        let b = guard
            .register_script_component("script.two", "Same", false)
            .unwrap();
        assert_ne!(
            a.component_id, b.component_id,
            "different namespaces, different components"
        );
        assert!(
            guard
                .register_script_component("script.one", "Same", false)
                .is_err()
        );

        // lookups work by full path only
        let by_path = guard
            .get_type_registration_by_name("script.two.Same".to_owned())
            .unwrap();
        assert!(by_path.is_some());
        let by_name = guard
            .get_type_registration_by_name("Same".to_owned())
            .unwrap();
        assert!(by_name.is_none());
        let registry = guard.component_registry();
        let registry = registry.read();
        assert!(registry.find("Same").is_none());
        assert_eq!(
            registry.find("script.two.Same").unwrap().namespace,
            "script.two"
        );
    }

    #[test]
    fn script_events_share_paths_with_components_and_exports() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let owner = ScriptAttachment::StaticScript(Handle::default());

        guard
            .register_script_component("script.game", "Enemy", false)
            .unwrap();
        guard
            .register_script_event("script.game", "Hit", false)
            .unwrap();
        guard
            .register_script_export("script.game", "spawn", "spawn_impl".into(), owner.clone())
            .unwrap();

        // every kind is rejected on the paths of the other kinds
        assert!(
            guard
                .register_script_event("script.game", "Enemy", false)
                .is_err()
        );
        assert!(
            guard
                .register_script_event("script.game", "spawn", false)
                .is_err()
        );
        assert!(
            guard
                .register_script_component("script.game", "Hit", false)
                .is_err()
        );
        assert!(
            guard
                .register_script_export("script.game", "Hit", "hit_impl".into(), owner)
                .is_err()
        );

        let registry = guard.component_registry();
        let registry = registry.read();
        let events = registry.iter_events_in("script.game").collect::<Vec<_>>();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0, "Hit");
        assert!(registry.iter_events_in("script").next().is_none());
    }

    #[test]
    fn register_script_event_duplicates_need_allow_existing() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let first = guard
            .register_script_event("script.test", "Ev", false)
            .unwrap();
        let err = guard
            .register_script_event("script.test", "Ev", false)
            .unwrap_err();
        assert!(err.to_string().contains("allow_existing"), "{err}");
        let again = guard
            .register_script_event("script.test", "Ev", true)
            .unwrap();
        assert_eq!(first.name, again.name);
        // events do not take part in the type catalogue generation
        assert!(guard.trigger_script_event(first, ScriptValue::Unit).is_ok());
    }

    #[test]
    fn script_exports_are_unique_per_path_and_owned_by_one_script() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let (a, b) = (guard.spawn().unwrap(), guard.spawn().unwrap());
        let owner_a = ScriptAttachment::EntityScript(a, Handle::default());
        let owner_b = ScriptAttachment::EntityScript(b, Handle::default());

        guard
            .register_script_export("script.game", "spawn", "spawn_impl".into(), owner_a.clone())
            .unwrap();
        // the same script exporting again replaces the export
        guard
            .register_script_export("script.game", "spawn", "other_impl".into(), owner_a)
            .unwrap();
        assert_eq!(
            guard
                .component_registry()
                .read()
                .get_export("script.game.spawn")
                .unwrap()
                .label,
            "other_impl"
        );

        // another script can't take it
        let err = guard
            .register_script_export("script.game", "spawn", "x".into(), owner_b.clone())
            .unwrap_err()
            .to_string();
        assert!(err.contains("already exported by script"), "{err}");

        // namespaces are validated like for components
        assert!(
            guard
                .register_script_export("game", "spawn", "x".into(), owner_b.clone())
                .is_err()
        );

        // exports and components cannot share a path, in both directions
        assert!(
            guard
                .register_script_component("script.game", "spawn", false)
                .is_err()
        );
        guard
            .register_script_component("script.game", "Enemy", false)
            .unwrap();
        assert!(
            guard
                .register_script_export("script.game", "Enemy", "x".into(), owner_b)
                .is_err()
        );

        // calling something which is not exported fails clearly
        let err = guard
            .call_script_export("script.game.nope", vec![])
            .unwrap_err()
            .to_string();
        assert!(err.contains("does not exist"), "{err}");
    }

    #[test]
    fn unloading_a_script_removes_its_exports_and_observers_only() {
        let mut world = setup_world(|_, _| {});
        world.insert_resource(AppScriptComponentRegistry::default());
        let (a, b, observers) = {
            let guard = make_guard(&mut world);
            let (a, b) = (guard.spawn().unwrap(), guard.spawn().unwrap());
            let owner_a = ScriptAttachment::EntityScript(a, Handle::default());
            let owner_b = ScriptAttachment::EntityScript(b, Handle::default());

            guard
                .register_script_export("script.game", "one", "f".into(), owner_a.clone())
                .unwrap();
            guard
                .register_script_export("script.game", "two", "f".into(), owner_b.clone())
                .unwrap();
            let event = guard
                .register_script_event("script.game", "Ev", false)
                .unwrap();
            let observers = [
                guard
                    .add_script_event_observer(event.clone(), "f".into(), owner_a)
                    .unwrap(),
                guard
                    .add_script_event_observer(event, "f".into(), owner_b)
                    .unwrap(),
            ];
            (a, b, observers)
        };

        crate::remove_script_registrations(
            &mut world,
            &ScriptAttachment::EntityScript(a, Handle::default()),
        );

        let registry = world.resource::<AppScriptComponentRegistry>().read();
        assert!(registry.get_export("script.game.one").is_none());
        assert!(registry.get_export("script.game.two").is_some());
        drop(registry);
        assert!(
            world.get_entity(observers[0]).is_err(),
            "the observer of the unloaded script is removed"
        );
        assert!(
            world.get_entity(observers[1]).is_ok(),
            "other scripts' observers stay"
        );
        let _ = b;
    }

    #[test]
    fn register_script_component_allow_existing_returns_existing() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let first = guard
            .register_script_component("script.test", "MyDynComp", false)
            .unwrap();
        let err = guard
            .register_script_component("script.test", "MyDynComp", false)
            .unwrap_err();
        assert!(err.to_string().contains("allow_existing"), "{err}");

        let second = guard
            .register_script_component("script.test", "MyDynComp", true)
            .unwrap();
        assert_eq!(first.component_id, second.component_id);

        // a fresh name is still registered normally with the flag set
        let other = guard
            .register_script_component("script.test", "OtherDynComp", true)
            .unwrap();
        assert_ne!(first.component_id, other.component_id);
    }

    #[test]
    fn script_component_insert_moves_value_into_entity() {
        use bevy_reflect::structs::Struct;

        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let registration = guard
            .register_script_component("script.test", "MyDynComp", false)
            .unwrap();
        let entity = guard.spawn().unwrap();

        let mut component = DynamicComponent::default();
        *component
            .field_mut("data")
            .and_then(|f| f.try_downcast_mut::<ScriptValue>())
            .unwrap() = ScriptValue::String("hello".into());
        registration
            .insert_into_entity(guard.clone(), entity, Box::new(component))
            .unwrap();

        let data = guard
            .with_world_mut(|world| {
                let ptr = world.get_by_id(entity, registration.component_id).unwrap();
                // Safety: script components are always backed by `DynamicComponent`
                let component = unsafe { ptr.deref::<DynamicComponent>() };
                component
                    .field("data")
                    .and_then(|f| f.try_downcast_ref::<ScriptValue>())
                    .cloned()
            })
            .unwrap();
        assert!(matches!(data, Some(ScriptValue::String(s)) if s == "hello"));

        guard.despawn(entity).unwrap();
    }

    // ── bundles ───────────────────────────────────────────────────────────────

    fn native_bundle_reg(guard: &WorldAccessGuard<'_>) -> ScriptBundleRegistration {
        guard
            .get_bundle_type(
                guard
                    .get_type_by_name("test_utils::test_data::TestBundle")
                    .unwrap(),
            )
            .unwrap()
            .unwrap()
    }

    fn allocate<T: Reflect>(_guard: &WorldAccessGuard<'_>, value: T) -> ReflectReference {
        ReflectReference::new_allocated(value)
    }

    #[test]
    fn native_bundle_is_classified_as_bundle() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let registration = guard
            .get_type_registration(
                guard
                    .get_type_by_name("test_utils::test_data::TestBundle")
                    .unwrap(),
            )
            .unwrap();
        assert!(
            registration
                .into_right()
                .unwrap()
                .into_right()
                .unwrap()
                .is_right()
        );
    }

    #[test]
    fn native_bundle_insert_and_spawn() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let registration = native_bundle_reg(&guard);
        let simple = comp_reg::<SimpleStruct>(&guard).component_id;

        let entity = guard.spawn().unwrap();
        let value = allocate(
            &guard,
            TestBundle {
                comp: CompWithDefaultAndComponentData("bundled".to_owned()),
                nested: TestNestedBundle::default(),
            },
        );
        guard
            .insert_bundle(entity, registration.clone(), value)
            .unwrap();
        let comp = guard
            .with_component::<CompWithDefaultAndComponentData, _, _>(entity, |c| {
                c.unwrap().0.clone()
            })
            .unwrap();
        assert_eq!(comp, "bundled");
        // nested bundles are inserted too
        assert!(guard.has_component(entity, simple).unwrap());

        let value = allocate(&guard, TestBundle::default());
        let spawned = guard.spawn_bundle(registration, value).unwrap();
        assert!(guard.has_component(spawned, simple).unwrap());
    }

    #[test]
    fn native_bundle_insert_rejects_other_types() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let registration = native_bundle_reg(&guard);
        let entity = guard.spawn().unwrap();
        let value = allocate(&guard, SimpleStruct::default());
        assert!(guard.insert_bundle(entity, registration, value).is_err());
    }

    // ── query ─────────────────────────────────────────────────────────────────

    #[test]
    fn query_returns_only_matching_entities() {
        let mut world = setup_world(|_, _| {});
        let guard = make_guard(&mut world);
        let _without = guard.spawn().unwrap();
        let reg = comp_reg::<CompWithDefaultAndComponentData>(&guard);
        let mut builder = ScriptQueryBuilder::new();
        builder.with_components(vec![reg]);

        let results = guard.query(builder).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].entity,
            CompWithDefaultAndComponentData::test_entity_id()
        );
    }

    // ── setup_cache_raw ───────────────────────────────────────────────────────

    #[test]
    fn setup_cache_raw_produces_functional_guard() {
        let mut world = setup_world(|_, _| {});
        let cache = WorldAccessGuard::setup_cache_raw(
            CurrentScriptAttachment::empty(),
            world
                .get_resource::<AppScriptFunctionRegistry>()
                .cloned()
                .unwrap_or_default(),
            world
                .get_resource::<AppScheduleRegistry>()
                .cloned()
                .unwrap_or_default(),
            world
                .get_resource::<AppScriptComponentRegistry>()
                .cloned()
                .unwrap_or_default(),
            AppScriptCallbackRunners::default(),
            AppScriptTypeCatalogue::default(),
        );
        let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
        assert!(guard.spawn().is_ok());
    }

    /// Script observers deliver their callbacks through queued commands, so every operation
    /// which can trigger observers has to flush before it returns.
    mod observer_flushing {
        use super::*;
        use bevy_ecs::{
            component::Component,
            event::Event,
            lifecycle::{AddEvent, DespawnEvent, InsertEvent, RemoveEvent},
            reflect::{ReflectComponent, ReflectEvent},
        };
        use bevy_mod_scripting_asset::Language;
        use bevy_reflect::prelude::ReflectDefault;

        #[derive(Component, Reflect, Default)]
        #[reflect(Component, Default)]
        struct ObsComp;

        #[derive(Event, Reflect, Clone)]
        #[reflect(Event)]
        struct ObsEvent;

        struct Fixture {
            world: World,
            counts: Arc<parking_lot::Mutex<std::collections::HashMap<String, usize>>>,
            runners: AppScriptCallbackRunners,
        }

        fn fixture() -> Fixture {
            let mut world = setup_world(|world, registry| {
                world.register_component::<ObsComp>();
                registry.register::<ObsComp>();
                registry.register::<ObsEvent>();
                registry.register::<AddEvent>();
                registry.register_type_data::<AddEvent, ReflectEvent>();
                registry.register::<InsertEvent>();
                registry.register_type_data::<InsertEvent, ReflectEvent>();
                registry.register::<RemoveEvent>();
                registry.register_type_data::<RemoveEvent, ReflectEvent>();
                registry.register::<DespawnEvent>();
                registry.register_type_data::<DespawnEvent, ReflectEvent>();
            });
            let runners = AppScriptCallbackRunners::default();
            let counts = Arc::new(parking_lot::Mutex::new(std::collections::HashMap::<
                String,
                usize,
            >::new()));
            let (c, attachment) = (
                counts.clone(),
                ScriptAttachment::StaticScript(Handle::default()),
            );
            runners.register(
                Language::Lua,
                Arc::new(move |_, _, label, _| {
                    *c.lock().entry(label.to_owned()).or_insert(0) += 1;
                    Ok(vec![(attachment.clone(), Ok(ScriptValue::Unit))])
                }),
            );
            world.insert_resource(runners.clone());
            Fixture {
                world,
                counts,
                runners,
            }
        }

        impl Fixture {
            fn count(&self, label: &str) -> usize {
                self.counts.lock().get(label).copied().unwrap_or(0)
            }
        }

        fn observe<T: 'static>(
            guard: &WorldGuard,
            label: &str,
            comp: &ScriptComponentRegistration,
        ) {
            let registry = guard.type_registry();
            let registration = ScriptTypeRegistration::new(Arc::new(
                registry.read().get(TypeId::of::<T>()).unwrap().clone(),
            ));
            guard
                .add_script_observer(
                    registration,
                    vec![comp.clone()],
                    vec![],
                    label.to_owned(),
                    ScriptAttachment::StaticScript(Handle::default()),
                )
                .unwrap();
        }

        #[test]
        fn operations_flush_script_observer_callbacks() {
            let mut f = fixture();
            let counts = f.counts.clone();
            let count = move |l: &str| counts.lock().get(l).copied().unwrap_or(0);
            let guard = make_guard(&mut f.world);
            let comp = comp_reg::<ObsComp>(&guard);
            observe::<AddEvent>(&guard, "add", &comp);
            observe::<InsertEvent>(&guard, "insert", &comp);
            observe::<RemoveEvent>(&guard, "remove", &comp);
            observe::<DespawnEvent>(&guard, "despawn", &comp);

            let entity = guard.spawn().unwrap();
            guard.add_default_component(entity, comp.clone()).unwrap();
            assert_eq!(count("add"), 1, "add_default_component");
            assert_eq!(count("insert"), 1, "add_default_component insert");

            guard.remove_component(entity, comp.clone()).unwrap();
            assert_eq!(count("remove"), 1, "remove_component");

            guard.add_default_component(entity, comp.clone()).unwrap();
            guard.despawn(entity).unwrap();
            assert_eq!(count("despawn"), 1, "despawn");

            let parent = guard.spawn().unwrap();
            let child = guard.spawn().unwrap();
            guard.add_default_component(child, comp.clone()).unwrap();
            guard.insert_children(parent, 0, &[child]).unwrap();
            guard.despawn_recursive(parent).unwrap();
            assert_eq!(count("despawn"), 2, "despawn_recursive");
            assert!(!f.runners.has_queued());
        }

        #[test]
        fn trigger_event_flushes_script_observer_callbacks() {
            let mut f = fixture();
            let guard = make_guard(&mut f.world);
            let registry = guard.type_registry();
            let registration = ScriptTypeRegistration::new(Arc::new(
                registry
                    .read()
                    .get(TypeId::of::<ObsEvent>())
                    .unwrap()
                    .clone(),
            ));
            guard
                .add_script_observer(
                    registration.clone(),
                    vec![],
                    vec![],
                    "event".to_owned(),
                    ScriptAttachment::StaticScript(Handle::default()),
                )
                .unwrap();
            let reference = ReflectReference::new_allocated(ObsEvent);
            guard.trigger_event(registration, &reference).unwrap();
            drop(guard);
            assert_eq!(f.count("event"), 1, "trigger_event");
        }
    }

    mod reflected_dispatch {
        use super::*;
        use bevy_ecs::{
            event::Event,
            message::{Message, MessageCursor, Messages},
            observer::On,
            reflect::{ReflectEvent, ReflectMessage},
            system::ResMut,
        };
        use bevy_mod_scripting_world::WorldGuard;

        #[derive(Event, Reflect, Clone)]
        #[reflect(Event)]
        struct TestEvent {
            value: usize,
        }

        #[derive(Message, Reflect, Clone)]
        #[reflect(Message)]
        struct TestMessage {
            value: usize,
        }

        /// A reflected type with neither `ReflectEvent` nor `ReflectMessage` data
        #[derive(Reflect)]
        struct Plain {
            value: usize,
        }

        #[derive(bevy_ecs::resource::Resource, Default)]
        struct Seen(Vec<usize>);

        fn setup() -> World {
            let mut world = setup_world(|_, registry| {
                registry.register::<TestEvent>();
                registry.register::<TestMessage>();
                registry.register::<Plain>();
            });
            world.init_resource::<Seen>();
            world.init_resource::<Messages<TestMessage>>();
            world.add_observer(|e: On<TestEvent>, mut seen: ResMut<Seen>| {
                seen.0.push(e.value);
            });
            world
        }

        fn registration<T: 'static>(guard: &WorldGuard) -> ScriptTypeRegistration {
            let registry = guard.type_registry();
            let registry = registry.read();
            ScriptTypeRegistration::new(Arc::new(registry.get(TypeId::of::<T>()).unwrap().clone()))
        }

        fn allocate<T: Reflect>(_guard: &WorldGuard, value: T) -> ReflectReference {
            ReflectReference::new_allocated(value)
        }

        #[test]
        fn trigger_event_runs_observers_before_returning() {
            let mut world = setup();
            let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
            {
                let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
                let reference = allocate(&guard, TestEvent { value: 7 });
                guard
                    .trigger_event(registration::<TestEvent>(&guard), &reference)
                    .unwrap();
                // the script's value is untouched, it can be triggered again
                guard
                    .trigger_event(registration::<TestEvent>(&guard), &reference)
                    .unwrap();
            }
            assert_eq!(world.resource::<Seen>().0, vec![7, 7]);
        }

        #[test]
        fn write_message_is_readable_by_message_readers() {
            let mut world = setup();
            let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
            {
                let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
                let reference = allocate(&guard, TestMessage { value: 3 });
                guard
                    .write_message(registration::<TestMessage>(&guard), &reference)
                    .unwrap();
            }
            let messages = world.resource::<Messages<TestMessage>>();
            let mut cursor = MessageCursor::<TestMessage>::default();
            let read: Vec<_> = cursor.read(messages).map(|m| m.value).collect();
            assert_eq!(read, vec![3]);
        }

        #[test]
        fn missing_type_data_is_an_error() {
            let mut world = setup();
            let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
            let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
            let reference = allocate(&guard, Plain { value: 1 });
            let registration = registration::<Plain>(&guard);

            assert!(matches!(
                guard.trigger_event(registration.clone(), &reference),
                Err(InteropError::MissingTypeData { .. })
            ));
            assert!(matches!(
                guard.write_message(registration, &reference),
                Err(InteropError::MissingTypeData { .. })
            ));
        }

        #[test]
        fn value_of_wrong_type_is_rejected() {
            let mut world = setup();
            let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
            let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
            let reference = allocate(&guard, TestMessage { value: 1 });

            assert!(matches!(
                guard.trigger_event(registration::<TestEvent>(&guard), &reference),
                Err(InteropError::TypeMismatch { .. })
            ));
        }

        #[test]
        fn cannot_trigger_while_other_access_is_held() {
            let mut world = setup();
            let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
            let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
            let reference = allocate(&guard, TestEvent { value: 1 });
            let registration = registration::<TestEvent>(&guard);

            let nested = guard
                .with_read_access(bevy_mod_scripting_world::WorldAccessRange::Global, || {
                    guard.trigger_event(registration.clone(), &reference)
                });
            assert!(matches!(nested, Ok(Err(_))));
            // the failed attempt must not leak any claim
            assert!(guard.trigger_event(registration, &reference).is_ok());
        }

        #[test]
        fn commands_queued_by_observers_are_applied_before_trigger_event_returns() {
            let mut world = setup();
            world.add_observer(
                |e: On<TestEvent>, mut commands: bevy_ecs::system::Commands| {
                    let value = e.value;
                    commands.queue(move |world: &mut World| {
                        world.resource_mut::<Seen>().0.push(value + 100);
                    });
                },
            );
            // like the structural operations (which flush as bevy does for them), triggering flushes,
            // so that script observers (which deliver their callbacks via commands) run before it returns
            {
                let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
                let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
                let reference = allocate(&guard, TestEvent { value: 1 });
                guard
                    .trigger_event(registration::<TestEvent>(&guard), &reference)
                    .unwrap();
            }
            assert_eq!(world.resource::<Seen>().0, vec![1, 101]);
        }

        #[test]
        fn custom_type_data_is_exposed() {
            // sanity check the bevy side of the contract these bindings rely on
            let world = setup();
            let registry = world
                .resource::<bevy_ecs::reflect::AppTypeRegistry>()
                .read();
            let reg = registry.get(TypeId::of::<TestEvent>()).unwrap();
            assert!(reg.data::<ReflectEvent>().is_some());
            assert!(reg.data::<ReflectMessage>().is_none());
        }
    }
}
