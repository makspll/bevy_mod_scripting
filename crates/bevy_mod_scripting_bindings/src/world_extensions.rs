//!
//! Implementations of [`WorldExtensions`] trait on the world guard.

use super::{
    AppReflectAllocator, AppScriptComponentRegistry, ReflectBase, ReflectBaseType,
    ReflectReference, ScriptComponentRegistration, ScriptResourceRegistration,
    ScriptTypeRegistration, Union,
    function::{
        namespace::Namespace,
        script_function::{AppScriptFunctionRegistry, DynamicScriptFunction, FunctionCallContext},
    },
    schedule::AppScheduleRegistry,
    script_value::ScriptValue,
};
use crate::{
    DynamicComponent, DynamicComponentInfo, ScriptQueryBuilder, ScriptQueryResult,
    error::InteropError,
    function::{from::FromScript, from_ref::FromScriptRef},
    reflection_extensions::PartialReflectExt,
};
use ::{
    bevy_asset::{AssetServer, Handle, LoadState},
    bevy_ecs::{
        component::ComponentId,
        entity::Entity,
        reflect::{ReflectFromWorld, ReflectResource},
        world::World,
    },
    bevy_reflect::{
        DynamicEnum, DynamicStruct, DynamicTuple, DynamicTupleStruct, DynamicVariant,
        PartialReflect, std_traits::ReflectDefault,
    },
};
use bevy_app::AppExit;
use bevy_asset::AssetPath;
use bevy_ecs::{
    component::{Component, ComponentCloneBehavior, ComponentDescriptor},
    hierarchy::{ChildOf, Children},
    resource::Resource,
    system::Commands,
    world::{CommandQueue, EntityRef, Mut},
};
use bevy_mod_scripting_asset::ScriptAsset;
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_mod_scripting_world::{CachedRegistry, WorldAccessGuard, WorldGuard};
use bevy_platform::collections::HashMap;
use bevy_reflect::{GetTypeRegistration, TypeInfo, VariantInfo};
use bevy_system_reflection::{ReflectSchedule};
use std::{
    alloc::Layout,
    any::{Any, TypeId},
    borrow::Cow,
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

    /// Executes a closure with access to a component (if present).
    fn with_component<C: Component, O, F: FnOnce(Option<&C>) -> O>(
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
    fn with_resource_mut<R: Resource, O, F: FnOnce(Mut<R>) -> O>(
        &self,
        f: F,
    ) -> Result<O, InteropError>;

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

    /// Constructs a reflected value from a type and field payload.
    fn construct(
        &self,
        type_: ScriptTypeRegistration,
        payload: HashMap<String, ScriptValue>,
        one_indexed: bool,
    ) -> Result<Box<dyn PartialReflect>, InteropError>;

    /// Attempts to call a function overload matching the provided arguments.
    fn try_call_overloads(
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

    /// Looks up a function by name across multiple type namespaces.
    fn lookup_function(
        &self,
        type_ids: impl IntoIterator<Item = TypeId>,
        name: impl Into<Cow<'static, str>>,
    ) -> Result<DynamicScriptFunction, Cow<'static, str>>;

    /// Resolves a type registration by name.
    fn get_type_by_name(&self, type_name: &str) -> Option<ScriptTypeRegistration>;

    /// Resolves a type registration and determines if it's a component or resource.
    fn get_type_registration(
        &self,
        registration: ScriptTypeRegistration,
    ) -> Result<
        Union<
            ScriptTypeRegistration,
            Union<ScriptComponentRegistration, ScriptResourceRegistration>,
        >,
        InteropError,
    >;

    /// Resolves a type registration by name with component/resource detection.
    fn get_type_registration_by_name(
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
    >;

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

    /// Returns the script function registry.
    fn script_function_registry(&self) -> &AppScriptFunctionRegistry;

    /// Returns the allocator used for reflection.
    fn allocator(&self) -> &AppReflectAllocator;

    /// Returns the component registry.
    fn component_registry(&self) -> &AppScriptComponentRegistry;

    /// Returns the schedule registry.
    fn schedule_registry(&self) -> &AppScheduleRegistry;

    /// Returns the current attachment if the guard is being used in the context of one.
    fn current_attachment(&self) -> CurrentScriptAttachment;

    /// Registers a dynamic script component, and returns a reference to its registration
    fn register_script_component(
        &self,
        component_name: String,
    ) -> Result<ScriptComponentRegistration, InteropError>;

    /// Initializes cached registries from the world.
    fn setup_cache(world: &World) -> [Rc<dyn Any>; 5];

    /// Initializes cached registries from the world, from raw components.
    fn setup_cache_raw(
        attachment: CurrentScriptAttachment,
        allocator: AppReflectAllocator,
        function_registry: AppScriptFunctionRegistry,
        schedule_registry: AppScheduleRegistry,
        component_registry: AppScriptComponentRegistry,
    ) -> [Rc<dyn Any>; 5];
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
            let mut built_query = query.as_query_state::<EntityRef>(world);
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

    /// add a default component to an entity
    fn add_default_component(
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
            self.with_world_mut(|world| from_world_td.from_world(world))?
        } else {
            return Err(InteropError::missing_type_data(
                registration.registration.type_id(),
                "ReflectDefault or ReflectFromWorld".to_owned(),
            ));
        };

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
        self.with_world_mut(|world| component_data.remove(world))
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

    fn with_resource_mut<R: Resource, O, F: FnOnce(Mut<R>) -> O>(
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

    // fn systems(&self, schedule: &ReflectSchedule) -> Result<Vec<ReflectSystem>, InteropError> {
    //     self.with_resource(|schedules: &Schedules| {
    //         let schedule = schedules
    //             .get(*schedule.label())
    //             .ok_or_else(|| InteropError::missing_schedule(schedule.identifier()))?;

    //         let systems = schedule.systems().map_err(|_| {
    //             InteropError::string(format!(
    //                 "failed to get systems from schedule '{:?}', schedule is not initialized.",
    //                 schedule.label()
    //             ))
    //         })?;

    //         Ok(systems
    //             .map(|(node_id, system)| ReflectSystem::from_system(system.as_ref(), node_id))
    //             .collect())
    //     })?
    // }

    // fn scope_schedule<O, F: FnOnce(&mut World, &mut Schedule) -> O>(
    //     &self,
    //     label: &ReflectSchedule,
    //     f: F,
    // ) -> Result<O, InteropError> {
    //     self.with_world_mut(|world| {
    //         let mut schedules = world.get_resource_mut::<Schedules>().ok_or_else(|| {
    //             InteropError::unsupported_operation(
    //                 None,
    //                 None,
    //                 "accessing schedules in a world with no schedules",
    //             )
    //         })?;

    //         let mut removed_schedule = schedules
    //             .remove(*label.label())
    //             .ok_or_else(|| InteropError::missing_schedule(label.identifier()))?;

    //         let result = f(world, &mut removed_schedule);

    //         let mut schedules = world.get_resource_mut::<Schedules>().ok_or_else(|| {
    //             InteropError::unsupported_operation(
    //                 None,
    //                 None,
    //                 "removing `Schedules` resource within a schedule scope",
    //             )
    //         })?;

    //         assert!(
    //             removed_schedule.label() == *label.label(),
    //             "removed schedule label doesn't match the original"
    //         );
    //         schedules.insert(removed_schedule);

    //         Ok(result)
    //     })?
    // }

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

    fn construct(
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
            TypeInfo::Struct(struct_info) => {
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
                let mut dynamic = construct_dynamic_struct(self, &mut payload, fields_iter)?;
                dynamic.set_represented_type(Some(type_info));
                Box::new(dynamic)
            }
            TypeInfo::TupleStruct(tuple_struct_info) => {
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
                    construct_dynamic_tuple_struct(self, &mut payload, fields_iter, one_indexed)?;
                dynamic.set_represented_type(Some(type_info));
                Box::new(dynamic)
            }
            TypeInfo::Tuple(tuple_info) => {
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
                    construct_dynamic_tuple(self, &mut payload, fields_iter, one_indexed)?;
                dynamic.set_represented_type(Some(type_info));
                Box::new(dynamic)
            }
            TypeInfo::Enum(enum_info) => {
                // extract variant from "variant"
                let variant = payload.remove("variant").ok_or_else(|| {
                    InteropError::function_interop_error(
                        "construct",
                        Namespace::OnType(TypeId::of::<World>()),
                        InteropError::str("missing 'variant' field in enum constructor payload"),
                        None,
                    )
                })?;

                let variant_name = String::from_script(variant, self.clone())?;

                let variant = enum_info.variant(&variant_name).ok_or_else(|| {
                    InteropError::function_interop_error(
                        "construct",
                        Namespace::OnType(TypeId::of::<World>()),
                        InteropError::string(format!(
                            "invalid variant name '{}' for enum '{}'",
                            variant_name,
                            enum_info.type_path()
                        )),
                        None,
                    )
                })?;

                let variant = match variant {
                    VariantInfo::Struct(struct_variant_info) => {
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

                        let dynamic = construct_dynamic_struct(self, &mut payload, fields_iter)?;
                        DynamicVariant::Struct(dynamic)
                    }
                    VariantInfo::Tuple(tuple_variant_info) => {
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
                            construct_dynamic_tuple(self, &mut payload, fields_iter, one_indexed)?;
                        DynamicVariant::Tuple(dynamic)
                    }
                    VariantInfo::Unit(_) => DynamicVariant::Unit,
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
                ));
            }
        };

        // try to construct type from reflect
        // TODO: it would be nice to have a <dyn PartialReflect>::from_reflect_with_fallback equivalent, that does exactly that
        // only using this as it's already there and convenient, the clone variant hitting will be confusing to end users
        <dyn PartialReflect>::from_reflect_or_clone(dynamic.as_ref(), self.clone())
    }

    fn try_call_overloads(
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
            Err(name) => {
                return Err(InteropError::missing_function(
                    name.to_string(),
                    Namespace::OnType(type_id),
                    Some(context.clone()),
                ));
            }
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

    fn get_functions_on_type(
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

    fn lookup_function(
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

    fn get_type_by_name(&self, type_name: &str) -> Option<ScriptTypeRegistration> {
        let type_registry = self.type_registry();
        let type_registry = type_registry.read();
        type_registry
            .get_with_short_type_path(type_name)
            .or_else(|| type_registry.get_with_type_path(type_name))
            .map(|registration| ScriptTypeRegistration::new(Arc::new(registration.clone())))
    }

    fn get_type_registration(
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

    fn get_type_registration_by_name(
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

    fn script_function_registry(&self) -> &AppScriptFunctionRegistry {
        #[allow(
            clippy::unwrap_used,
            reason = "internal domain boundary, enforced at creation of the guard"
        )]
        self.get_cached_registry().unwrap()
    }

    fn allocator(&self) -> &AppReflectAllocator {
        #[allow(
            clippy::unwrap_used,
            reason = "internal domain boundary, enforced at creation of the guard"
        )]
        self.get_cached_registry().unwrap()
    }

    fn component_registry(&self) -> &AppScriptComponentRegistry {
        #[allow(
            clippy::unwrap_used,
            reason = "internal domain boundary, enforced at creation of the guard"
        )]
        self.get_cached_registry().unwrap()
    }

    fn schedule_registry(&self) -> &AppScheduleRegistry {
        #[allow(
            clippy::unwrap_used,
            reason = "internal domain boundary, enforced at creation of the guard"
        )]
        self.get_cached_registry().unwrap()
    }

    fn current_attachment(&self) -> CurrentScriptAttachment {
        self.get_cached_registry()
            .cloned()
            .unwrap_or(CurrentScriptAttachment(None))
    }

    fn register_script_component(
        &self,
        component_name: String,
    ) -> Result<ScriptComponentRegistration, InteropError> {
        let component_registry = self.component_registry();
        let component_registry_read = component_registry.read();
        if component_registry_read.get(&component_name).is_some() {
            return Err(InteropError::unsupported_operation(
                None,
                None,
                "script registered component already exists",
            ));
        }

        let component_id = self.with_world_mut_access(|w| {
            let descriptor = unsafe {
                // Safety: same safety guarantees as ComponentDescriptor::new
                // we know the type in advance
                // we only use this method to name the component
                ComponentDescriptor::new_with_layout(
                    component_name.clone(),
                    DynamicComponent::STORAGE_TYPE,
                    Layout::new::<DynamicComponent>(),
                    needs_drop::<DynamicComponent>().then_some(|x| x.drop_as::<DynamicComponent>()),
                    true,
                    ComponentCloneBehavior::Default,
                    None,
                )
            };
            w.register_component_with_descriptor(descriptor)
        })?;
        drop(component_registry_read);
        let mut component_registry = component_registry.write();

        let registration = ScriptComponentRegistration::new(
            ScriptTypeRegistration::new(Arc::new(
                <DynamicComponent as GetTypeRegistration>::get_type_registration(),
            )),
            component_id,
        );

        let component_info = DynamicComponentInfo {
            name: component_name.clone(),
            registration: registration.clone(),
        };

        component_registry.register(component_info);

        // TODO: we should probably retrieve this from the registry, but I don't see what people would want to register on this type
        // in addition to the existing registrations.
        Ok(registration)
    }

    fn setup_cache_raw(
        attachment: CurrentScriptAttachment,
        allocator: AppReflectAllocator,
        function_registry: AppScriptFunctionRegistry,
        schedule_registry: AppScheduleRegistry,
        component_registry: AppScriptComponentRegistry,
    ) -> [Rc<dyn Any>; 5] {
        debug_assert_eq!(AppReflectAllocator::SLOT, 0);
        debug_assert_eq!(AppScriptFunctionRegistry::SLOT, 1);
        debug_assert_eq!(AppScheduleRegistry::SLOT, 2);
        debug_assert_eq!(AppScriptComponentRegistry::SLOT, 3);
        debug_assert_eq!(CurrentScriptAttachment::SLOT, 4);

        [
            Rc::new(allocator),
            Rc::new(function_registry),
            Rc::new(schedule_registry),
            Rc::new(component_registry),
            Rc::new(attachment),
        ]
    }

    fn setup_cache(world: &World) -> [Rc<dyn Any>; 5] {
        debug_assert_eq!(AppReflectAllocator::SLOT, 0);
        debug_assert_eq!(AppScriptFunctionRegistry::SLOT, 1);
        debug_assert_eq!(AppScheduleRegistry::SLOT, 2);
        debug_assert_eq!(AppScriptComponentRegistry::SLOT, 3);
        debug_assert_eq!(CurrentScriptAttachment::SLOT, 4);

        [
            Rc::new(
                world
                    .get_resource::<AppReflectAllocator>()
                    .cloned()
                    .unwrap_or_default(),
            ),
            Rc::new(
                world
                    .get_resource::<AppScriptFunctionRegistry>()
                    .cloned()
                    .unwrap_or_default(),
            ),
            Rc::new(
                world
                    .get_resource::<AppScheduleRegistry>()
                    .cloned()
                    .unwrap_or_default(),
            ),
            Rc::new(
                world
                    .get_resource::<AppScriptComponentRegistry>()
                    .cloned()
                    .unwrap_or_default(),
            ),
            Rc::new(CurrentScriptAttachment(None)),
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

fn construct_from_script_value(
    guard: &WorldGuard,
    descriptor: impl Into<Cow<'static, str>>,
    type_id: TypeId,
    value: Option<ScriptValue>,
) -> Result<Box<dyn PartialReflect>, InteropError> {
    // if the value is missing, try to construct a default and return it
    let value = match value {
        Some(value) => value,
        None => {
            let type_registry = guard.type_registry();
            let type_registry = type_registry.read();
            let default_data = type_registry
                .get_type_data::<ReflectDefault>(type_id)
                .ok_or_else(|| {
                    InteropError::function_interop_error(
                        "construct",
                        Namespace::OnType(TypeId::of::<World>()),
                        InteropError::string(format!(
                            "field missing and no default provided: '{}'",
                            descriptor.into()
                        )),
                        None,
                    )
                })?;
            return Ok(default_data.default().into_partial_reflect());
        }
    };

    // otherwise we need to use from_script_ref
    <Box<dyn PartialReflect>>::from_script_ref(type_id, value, guard.clone())
}

fn construct_dynamic_struct(
    guard: &WorldGuard,
    payload: &mut HashMap<String, ScriptValue>,
    fields: Vec<(&'static str, TypeId)>,
) -> Result<DynamicStruct, InteropError> {
    let mut dynamic = DynamicStruct::default();
    for (field_name, field_type_id) in fields {
        let constructed = construct_from_script_value(
            guard,
            field_name,
            field_type_id,
            payload.remove(field_name),
        )?;

        dynamic.insert_boxed(field_name, constructed);
    }
    Ok(dynamic)
}

fn construct_dynamic_tuple_struct(
    guard: &WorldGuard,
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
        let field_string = script_idx.to_string();
        dynamic.insert_boxed(construct_from_script_value(
            guard,
            field_string.clone(),
            field_type_id,
            payload.remove(&field_string),
        )?);
    }
    Ok(dynamic)
}

fn construct_dynamic_tuple(
    guard: &WorldGuard,
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

        let field_string = script_idx.to_string();

        dynamic.insert_boxed(construct_from_script_value(
            guard,
            field_string.clone(),
            field_type_id,
            payload.remove(&field_string),
        )?);
    }
    Ok(dynamic)
}

impl CachedRegistry for AppReflectAllocator {
    const SLOT: usize = 0;
}
impl CachedRegistry for AppScriptFunctionRegistry {
    const SLOT: usize = 1;
}
impl CachedRegistry for AppScheduleRegistry {
    const SLOT: usize = 2;
}
impl CachedRegistry for AppScriptComponentRegistry {
    const SLOT: usize = 3;
}

/// A wrapper around [`ScriptAttachment`] implementing [`CachedRegistry`]
#[derive(Clone, Default)]
pub struct CurrentScriptAttachment(pub Option<ScriptAttachment>);
impl CachedRegistry for CurrentScriptAttachment {
    const SLOT: usize = 4;
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy_reflect::{GetTypeRegistration, ReflectFromReflect};
    use test_utils::test_data::{SimpleEnum, SimpleStruct, SimpleTupleStruct, setup_world};

    #[test]
    fn test_construct_struct() {
        let mut world = setup_world(|_, _| {});
        let cache = WorldAccessGuard::setup_cache(&world);
        let world = WorldAccessGuard::new_exclusive(&mut world, cache);

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
        let cache = WorldAccessGuard::setup_cache(&world);
        let world = WorldAccessGuard::new_exclusive(&mut world, cache);

        let registry = world.type_registry();
        let registry = registry.read();

        let registration = registry
            .get(TypeId::of::<SimpleTupleStruct>())
            .unwrap()
            .clone();
        let type_registration = ScriptTypeRegistration::new(Arc::new(registration));

        // zero indexed
        let payload = HashMap::from_iter(vec![("0".to_owned(), ScriptValue::Integer(1))]);

        let result = world.construct(type_registration.clone(), payload, false);
        let expected =
            Ok::<_, InteropError>(Box::new(SimpleTupleStruct(1)) as Box<dyn PartialReflect>);
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));

        // one indexed
        let payload = HashMap::from_iter(vec![("1".to_owned(), ScriptValue::Integer(1))]);

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
        let cache = WorldAccessGuard::setup_cache(&world);

        <usize as GetTypeRegistration>::get_type_registration();
        let world = WorldAccessGuard::new_exclusive(&mut world, cache);

        let registry = world.type_registry();
        let registry = registry.read();

        let registration = registry
            .get(TypeId::of::<(usize, usize)>())
            .unwrap()
            .clone();
        let type_registration = ScriptTypeRegistration::new(Arc::new(registration));

        // zero indexed
        let payload = HashMap::from_iter(vec![
            ("0".to_owned(), ScriptValue::Integer(1)),
            ("1".to_owned(), ScriptValue::Integer(2)),
        ]);

        let result = world.construct(type_registration.clone(), payload, false);
        let expected = Ok::<_, InteropError>(Box::new((1, 2)) as Box<dyn PartialReflect>);
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));

        // one indexed
        let payload = HashMap::from_iter(vec![
            ("1".to_owned(), ScriptValue::Integer(1)),
            ("2".to_owned(), ScriptValue::Integer(2)),
        ]);

        let result = world.construct(type_registration.clone(), payload, true);
        let expected = Ok::<_, InteropError>(Box::new((1, 2)) as Box<dyn PartialReflect>);
        pretty_assertions::assert_str_eq!(format!("{result:#?}"), format!("{expected:#?}"));
    }

    #[test]
    fn test_construct_enum() {
        let mut world = setup_world(|_, _| {});
        let cache = WorldAccessGuard::setup_cache(&world);
        let world = WorldAccessGuard::new_exclusive(&mut world, cache);

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
            ("0".to_owned(), ScriptValue::Integer(1)),
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
}
