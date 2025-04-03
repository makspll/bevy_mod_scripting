//! Contains functions defined by the [`bevy_mod_scripting_core`] crate

use std::{collections::HashMap, ops::Deref};

use bevy::prelude::*;
use bevy_mod_scripting_core::{
    bindings::{
        function::{
            from::Union, namespace::GlobalNamespace, script_function::DynamicScriptFunctionMut,
        },
        script_system::ScriptSystemBuilder,
    },
    docgen::info::FunctionInfo,
    *,
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_system_reflection::{ReflectSchedule, ReflectSystem};
use bindings::{
    function::{
        from::{Ref, Val},
        from_ref::FromScriptRef,
        into_ref::IntoScriptRef,
        script_function::{FunctionCallContext, ScriptFunctionMut},
    },
    pretty_print::DisplayWithWorld,
    script_value::ScriptValue,
    ReflectReference, ScriptComponentRegistration, ScriptQueryBuilder, ScriptQueryResult,
    ScriptResourceRegistration, ScriptTypeRegistration, ThreadWorldContainer, WorldContainer,
};
use error::InteropError;
use reflection_extensions::{PartialReflectExt, TypeIdExtensions};

#[allow(unused_variables, reason = "feature flags")]
pub fn register_bevy_bindings(app: &mut App) {
    #[cfg(feature = "bevy_core")]
    app.add_plugins(crate::bevy_bindings::bevy_core::BevyCoreScriptingPlugin);
    #[cfg(feature = "bevy_ecs")]
    app.add_plugins(crate::bevy_bindings::bevy_ecs::BevyEcsScriptingPlugin);
    #[cfg(feature = "bevy_hierarchy")]
    app.add_plugins(crate::bevy_bindings::bevy_hierarchy::BevyHierarchyScriptingPlugin);
    #[cfg(feature = "bevy_input")]
    app.add_plugins(crate::bevy_bindings::bevy_input::BevyInputScriptingPlugin);
    #[cfg(feature = "bevy_math")]
    app.add_plugins(crate::bevy_bindings::bevy_math::BevyMathScriptingPlugin);
    #[cfg(feature = "bevy_reflect")]
    app.add_plugins(crate::bevy_bindings::bevy_reflect::BevyReflectScriptingPlugin);
    #[cfg(feature = "bevy_time")]
    app.add_plugins(crate::bevy_bindings::bevy_time::BevyTimeScriptingPlugin);
    #[cfg(feature = "bevy_transform")]
    app.add_plugins(crate::bevy_bindings::bevy_transform::BevyTransformScriptingPlugin);
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "world_functions",
    unregistered
)]
impl World {
    /// Returns either a `ScriptComponentRegistration` or `ScriptResourceRegistration` depending on the type of the type requested.
    /// If the type is neither returns a `ScriptTypeRegistration`.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `type_name`: The name of the type to retrieve.
    /// Returns:
    /// * `type`: The registration of the type, if it exists.
    fn get_type_by_name(
        ctxt: FunctionCallContext,
        type_name: String,
    ) -> Result<
        Option<
            Union<
                Val<ScriptTypeRegistration>,
                Union<Val<ScriptComponentRegistration>, Val<ScriptResourceRegistration>>,
            >,
        >,
        InteropError,
    > {
        profiling::function_scope!("get_type_by_name");
        let world = ctxt.world()?;
        world
            .get_type_registration_by_name(type_name)
            .map(|v| v.map(|v| v.map_both(Val::from, |u| u.map_both(Val::from, Val::from))))
    }

    /// Retrieves the schedule with the given name, Also ensures the schedule is initialized before returning it.
    ///
    /// Schedules in bevy are "containers" for systems, each schedule runs separately and contains different systems.
    ///
    /// By default among others bevy contains the following schedules:
    /// - `Update`: Runs every frame.
    /// - `PostUpdate`: Runs after the `Update` schedule.
    /// - `FixedUpdate`: Runs at a fixed rate.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `name`: The name of the schedule to retrieve.
    /// Returns:
    /// * `schedule`: The schedule with the given name, if it exists
    fn get_schedule_by_name(
        ctxt: FunctionCallContext,
        name: String,
    ) -> Result<Option<Val<ReflectSchedule>>, InteropError> {
        profiling::function_scope!("get_schedule_by_name");
        let world = ctxt.world()?;
        let schedule = match world.get_schedule_by_name(name) {
            Some(schedule) => schedule,
            None => return Ok(None),
        };

        // do two things, check it actually exists
        world.scope_schedule(&schedule, |world, schedule| schedule.initialize(world))??;

        Ok(Some(schedule.into()))
    }

    /// Tries to retrieve the given component type on an entity.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The entity to retrieve the component from.
    /// * `registration`: The component to retrieve.
    /// Returns:
    /// * `component`: The component on the entity, if it exists.
    fn get_component(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
        registration: Val<ScriptComponentRegistration>,
    ) -> Result<Option<ReflectReference>, InteropError> {
        profiling::function_scope!("get_component");
        let world = ctxt.world()?;
        let val = world.get_component(*entity, registration.into_inner())?;
        Ok(val)
    }

    /// Checks if the given entity has the given component.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The entity to check.
    /// * `registration`: The component to check for.
    /// Returns:
    /// * `has_component`: Whether the entity has the component.
    fn has_component(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
        registration: Val<ScriptComponentRegistration>,
    ) -> Result<bool, InteropError> {
        profiling::function_scope!("has_component");
        let world = ctxt.world()?;
        world.has_component(*entity, registration.component_id())
    }

    /// Removes the given component from the entity.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The entity to remove the component from.
    /// * `registration`: The component to remove.
    /// Returns:
    /// * `result`: Nothing if the component was removed successfully or didn't exist in the first place.
    fn remove_component(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
        registration: Val<ScriptComponentRegistration>,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("remove_component");
        let world = ctxt.world()?;
        world.remove_component(*entity, registration.clone())
    }

    /// Retrieves the resource with the given registration.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `registration`: The registration of the resource to retrieve.
    /// Returns:
    /// * `resource`: The resource, if it exists.
    fn get_resource(
        ctxt: FunctionCallContext,
        registration: Val<ScriptResourceRegistration>,
    ) -> Result<Option<ReflectReference>, InteropError> {
        profiling::function_scope!("get_resource");
        let world = ctxt.world()?;
        let val = world.get_resource(registration.resource_id())?;
        Ok(val)
    }

    /// Checks if the world has the given resource.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `registration`: The registration of the resource to check for.
    /// Returns:
    /// * `has_resource`: Whether the world has the resource.
    fn has_resource(
        ctxt: FunctionCallContext,
        registration: Val<ScriptResourceRegistration>,
    ) -> Result<bool, InteropError> {
        profiling::function_scope!("has_resource");
        let world = ctxt.world()?;
        world.has_resource(registration.resource_id())
    }

    /// Removes the given resource from the world.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `registration`: The resource to remove.
    /// Returns:
    /// * `result`: Nothing if the resource was removed successfully or didn't exist in the first place.
    fn remove_resource(
        ctxt: FunctionCallContext,
        registration: Val<ScriptResourceRegistration>,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("remove_resource");
        let world = ctxt.world()?;
        world.remove_resource(registration.into_inner())
    }

    /// Adds the given resource to the world.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `registration`: The resource to add.
    /// Returns:
    /// * `result`: Nothing if the resource was added successfully.
    fn add_default_component(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
        registration: Val<ScriptComponentRegistration>,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("add_default_component");
        let world = ctxt.world()?;
        world.add_default_component(*entity, registration.clone())
    }

    /// Spawns a new entity and returns it
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// Returns:
    /// * `entity`: The newly spawned entity
    fn spawn(ctxt: FunctionCallContext) -> Result<Val<Entity>, InteropError> {
        profiling::function_scope!("spawn");
        let world = ctxt.world()?;
        Ok(Val(world.spawn()?))
    }

    /// Inserts the given component value into the provided entity
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The entity to insert the component into.
    /// * `registration`: The component registration of the component to insert.
    /// * `value`: The value of the component to insert. Can be constructed using `construct`
    /// Returns:
    /// * `result`: Nothing if the component was inserted successfully.
    fn insert_component(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
        registration: Val<ScriptComponentRegistration>,
        value: ReflectReference,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("insert_component");
        let world = ctxt.world()?;
        world.insert_component(*entity, registration.into_inner(), value)
    }

    /// Inserts the given children entities into the provided parent entity.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The parent entity to receive children
    /// * `index`: The index to insert the children at
    /// * `children`: The children entities to insert
    /// Returns:
    /// * `result`: Nothing if the children were inserted successfully.
    fn insert_children(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
        index: usize,
        children: Vec<Val<Entity>>,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("insert_children");
        let world = ctxt.world()?;
        let index = if ctxt.convert_to_0_indexed() {
            index - 1
        } else {
            index
        };
        world.insert_children(
            *entity,
            index,
            &children.into_iter().map(|v| *v).collect::<Vec<_>>(),
        )
    }

    /// Pushes the given children entities into the provided parent entity.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The parent entity to receive children
    /// * `children`: The children entities to push
    /// Returns:
    /// * `result`: Nothing if the children were pushed successfully.
    fn push_children(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
        children: Vec<Val<Entity>>,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("push_children");
        let world = ctxt.world()?;
        world.push_children(
            *entity,
            &children.into_iter().map(|v| *v).collect::<Vec<_>>(),
        )
    }

    /// Retrieves the children of the given entity.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The entity to retrieve the children of.
    /// Returns:
    /// * `children`: The children of the entity.
    fn get_children(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
    ) -> Result<Vec<Val<Entity>>, InteropError> {
        profiling::function_scope!("get_children");
        let world = ctxt.world()?;
        let children = world.get_children(*entity)?;
        Ok(children.into_iter().map(Val).collect::<Vec<_>>())
    }

    /// Retrieves the parent of the given entity.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The entity to retrieve the parent of.
    /// Returns:
    /// * `parent`: The parent of the entity
    fn get_parent(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
    ) -> Result<Option<Val<Entity>>, InteropError> {
        profiling::function_scope!("get_parent");
        let world = ctxt.world()?;
        let parent = world.get_parent(*entity)?;
        Ok(parent.map(Val))
    }

    /// Despawns the given entity.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The entity to despawn.
    fn despawn(ctxt: FunctionCallContext, entity: Val<Entity>) -> Result<(), InteropError> {
        profiling::function_scope!("despawn");
        let world = ctxt.world()?;
        world.despawn(*entity)
    }

    /// Despawn the descendants of the given entity.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The entity to despawn the descendants of.
    /// Returns:
    /// * `result`: Nothing if the descendants were despawned successfully.
    fn despawn_descendants(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("despawn_descendants");
        let world = ctxt.world()?;
        world.despawn_descendants(*entity)
    }

    /// Despawns the entity and all its descendants.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The entity to despawn recursively.
    /// Returns:
    /// * `result`: Nothing if the entity and its descendants were despawned successfully.
    fn despawn_recursive(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("despawn_recursive");
        let world = ctxt.world()?;
        world.despawn_recursive(*entity)
    }

    /// Checks if the given entity exists.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The entity to check.
    /// Returns:
    /// * `has_entity`: Whether the entity exists.
    fn has_entity(ctxt: FunctionCallContext, e: Val<Entity>) -> Result<bool, InteropError> {
        profiling::function_scope!("has_entity");
        let world = ctxt.world()?;
        world.has_entity(*e)
    }

    /// Creates a new `ScriptQueryBuilder` which can be used to query the ECS.
    ///
    /// Returns:
    /// * `query`: The new query builder.
    fn query() -> Result<Val<ScriptQueryBuilder>, InteropError> {
        profiling::function_scope!("query");
        let query_builder = ScriptQueryBuilder::default();
        Ok(Val(query_builder))
    }

    /// Adds the given system to the world.
    ///
    /// Arguments:
    /// * `schedule`: The schedule to add the system to.
    /// * `builder`: The system builder specifying the system and its dependencies.
    /// Returns:
    /// * `system`: The system that was added.
    fn add_system(
        ctxt: FunctionCallContext,
        schedule: Val<ReflectSchedule>,
        builder: Val<ScriptSystemBuilder>,
    ) -> Result<Val<ReflectSystem>, InteropError> {
        profiling::function_scope!("add_system");
        let world = ctxt.world()?;
        let system = match ctxt.language() {
            #[cfg(feature = "lua_bindings")]
            asset::Language::Lua => world
                .add_system::<bevy_mod_scripting_lua::LuaScriptingPlugin>(
                    &schedule,
                    builder.into_inner(),
                )?,
            #[cfg(feature = "rhai_bindings")]
            asset::Language::Rhai => world
                .add_system::<bevy_mod_scripting_rhai::RhaiScriptingPlugin>(
                    &schedule,
                    builder.into_inner(),
                )?,
            _ => {
                return Err(InteropError::unsupported_operation(
                    None,
                    None,
                    format!(
                        "creating a system in {} scripting language",
                        ctxt.language()
                    ),
                ))
            }
        };
        Ok(Val(system))
    }

    /// Quits the program.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// Returns:
    /// * `result`: Nothing if the program was exited successfully.
    fn exit(ctxt: FunctionCallContext) -> Result<(), InteropError> {
        profiling::function_scope!("exit");
        let world = ctxt.world()?;
        world.exit()
    }

    /// Registers a new component type with the world.
    ///
    /// The component will behave like any other native component for all intents and purposes.
    /// The type that will be instantiated to back this component will be `DynamicComponent` which contains just one field:
    /// - `data`
    ///
    /// This field can be set to any value and modified freely.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `name`: The name of the component type
    /// Returns:
    /// * `registration`: The registration of the new component type if successful.
    fn register_new_component(
        ctxt: FunctionCallContext,
        name: String,
    ) -> Result<Val<ScriptComponentRegistration>, InteropError> {
        profiling::function_scope!("register_new_component");
        let world = ctxt.world()?;
        world.register_script_component(name).map(Val)
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "reflect_reference_functions",
    core
)]
impl ReflectReference {
    /// If this type is an enum, will return the name of the variant it represents on the type.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to get the variant name of.
    /// Returns:
    /// * `variant_name`: The name of the variant, if the reference is an enum.
    fn variant_name(
        ctxt: FunctionCallContext,
        reference: ReflectReference,
    ) -> Result<Option<String>, InteropError> {
        profiling::function_scope!("variant_name");
        let world = ctxt.world()?;
        reference.variant_name(world)
    }

    /// Displays this reference without printing the exact contents.
    ///
    /// This is useful for debugging and logging.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to display.
    /// Returns:
    /// * `display`: The display string.
    fn display_ref(
        ctxt: FunctionCallContext,
        reference: ReflectReference,
    ) -> Result<String, InteropError> {
        profiling::function_scope!("display_ref");
        let world = ctxt.world()?;
        Ok(reference.display_with_world(world))
    }

    /// Displays the "value" of this reference
    ///
    /// This is useful for debugging and logging.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to display.
    /// Returns:
    /// * `display`: The display string.
    fn display_value(
        ctxt: FunctionCallContext,
        reference: ReflectReference,
    ) -> Result<String, InteropError> {
        profiling::function_scope!("display_value");
        let world = ctxt.world()?;
        Ok(reference.display_value_with_world(world))
    }

    /// Gets and clones the value under the specified key if the underlying type is a map type.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to index into.
    /// * `key`: The key to index with.
    /// Returns:
    /// * `value`: The value at the key, if the reference is a map.
    fn map_get(
        ctxt: FunctionCallContext,
        reference: ReflectReference,
        key: ScriptValue,
    ) -> Result<Option<ScriptValue>, InteropError> {
        profiling::function_scope!("map_get");
        let world = ctxt.world()?;
        let key = <Box<dyn PartialReflect>>::from_script_ref(
            reference.key_type_id(world.clone())?.ok_or_else(|| {
                InteropError::unsupported_operation(
                    reference.tail_type_id(world.clone()).unwrap_or_default(),
                    Some(Box::new(key.clone())),
                    "Could not get key type id. Are you trying to index into a type that's not a map?".to_owned(),
                )
            })?,
            key,
            world.clone(),
        )?;
        reference.with_reflect_mut(world.clone(), |s| match s.try_map_get(key.as_ref())? {
            Some(value) => {
                let reference = {
                    let allocator = world.allocator();
                    let mut allocator = allocator.write();
                    let owned_value = <dyn PartialReflect>::from_reflect(value, world.clone())?;
                    ReflectReference::new_allocated_boxed(owned_value, &mut allocator)
                };
                Ok(Some(ReflectReference::into_script_ref(reference, world)?))
            }
            None => Ok(None),
        })?
    }

    /// Pushes the value into the reference, if the reference is an appropriate container type.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to push the value into.
    /// * `value`: The value to push.
    /// Returns:
    /// * `result`: Nothing if the value was pushed successfully.
    fn push(
        ctxt: FunctionCallContext,
        reference: ReflectReference,
        value: ScriptValue,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("push");
        let world = ctxt.world()?;
        let target_type_id = reference.element_type_id(world.clone())?.ok_or_else(|| {
            InteropError::unsupported_operation(
                reference.tail_type_id(world.clone()).unwrap_or_default(),
                Some(Box::new(value.clone())),
                "Could not get element type id. Are you trying to insert elements into a type that's not a list?".to_owned(),
            )
        })?;
        let other =
            <Box<dyn PartialReflect>>::from_script_ref(target_type_id, value, world.clone())?;
        reference.with_reflect_mut(world, |s| s.try_push_boxed(other))?
    }

    /// Pops the value from the reference, if the reference is an appropriate container type.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to pop the value from.
    /// Returns:
    /// * `value`: The value that was popped, if the reference supports popping.
    fn pop(
        ctxt: FunctionCallContext,
        reference: ReflectReference,
    ) -> Result<ScriptValue, InteropError> {
        profiling::function_scope!("pop");
        let world = ctxt.world()?;
        let o = reference.with_reflect_mut(world.clone(), |s| s.try_pop_boxed())??;
        let reference = {
            let allocator = world.allocator();
            let mut allocator = allocator.write();
            ReflectReference::new_allocated_boxed_parial_reflect(o, &mut allocator)?
        };

        ReflectReference::into_script_ref(reference, world)
    }

    /// Inserts the value into the reference at the specified index, if the reference is an appropriate container type.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to insert the value into.
    /// * `key`: The index to insert the value at.
    /// * `value`: The value to insert.
    /// Returns:
    /// * `result`: Nothing if the value was inserted successfully.
    fn insert(
        ctxt: FunctionCallContext,
        reference: ReflectReference,
        key: ScriptValue,
        value: ScriptValue,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("insert");
        let world = ctxt.world()?;
        let key_type_id = reference.key_type_id(world.clone())?.ok_or_else(|| {
            InteropError::unsupported_operation(
                reference.tail_type_id(world.clone()).unwrap_or_default(),
                Some(Box::new(key.clone())),
                "Could not get key type id. Are you trying to insert elements into a type that's not a map?".to_owned(),
            )
        })?;

        let mut key = <Box<dyn PartialReflect>>::from_script_ref(key_type_id, key, world.clone())?;

        if ctxt.convert_to_0_indexed() {
            key.convert_to_0_indexed_key();
        }

        let value_type_id = reference.element_type_id(world.clone())?.ok_or_else(|| {
            InteropError::unsupported_operation(
                reference.tail_type_id(world.clone()).unwrap_or_default(),
                Some(Box::new(value.clone())),
                "Could not get element type id. Are you trying to insert elements into a type that's not a map?".to_owned(),
            )
        })?;

        let value =
            <Box<dyn PartialReflect>>::from_script_ref(value_type_id, value, world.clone())?;

        reference.with_reflect_mut(world, |s| s.try_insert_boxed(key, value))?
    }

    /// Clears the container, if the reference is an appropriate container type.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to clear.
    /// Returns:
    /// * `result`: Nothing if the reference was cleared
    fn clear(ctxt: FunctionCallContext, reference: ReflectReference) -> Result<(), InteropError> {
        profiling::function_scope!("clear");
        let world = ctxt.world()?;
        reference.with_reflect_mut(world, |s| s.try_clear())?
    }

    /// Retrieves the length of the reference, if the reference is an appropriate container type.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to get the length of.
    /// Returns:
    /// * `len`: The length of the reference, if the reference is a container.
    fn len(
        ctxt: FunctionCallContext,
        reference: ReflectReference,
    ) -> Result<Option<usize>, InteropError> {
        profiling::function_scope!("len");
        let world = ctxt.world()?;
        reference.len(world)
    }

    /// Removes the value at the specified key from the reference, if the reference is an appropriate container type.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to remove the value from.
    /// * `key`: The key to remove the value at.
    /// Returns:
    /// * `result`: The removed value if any
    fn remove(
        ctxt: FunctionCallContext,
        reference: ReflectReference,
        key: ScriptValue,
    ) -> Result<ScriptValue, InteropError> {
        profiling::function_scope!("remove");
        let world = ctxt.world()?;
        let key_type_id = reference.key_type_id(world.clone())?.ok_or_else(|| {
            InteropError::unsupported_operation(
                reference.tail_type_id(world.clone()).unwrap_or_default(),
                Some(Box::new(key.clone())),
                "Could not get key type id. Are you trying to remove elements from a type that's not a map?".to_owned(),
            )
        })?;

        let mut key = <Box<dyn PartialReflect>>::from_script_ref(key_type_id, key, world.clone())?;

        if ctxt.convert_to_0_indexed() {
            key.convert_to_0_indexed_key();
        }

        let removed = reference.with_reflect_mut(world.clone(), |s| s.try_remove_boxed(key))??;
        match removed {
            Some(removed) => {
                let reference = {
                    let allocator = world.allocator();
                    let mut allocator = allocator.write();
                    ReflectReference::new_allocated_boxed_parial_reflect(removed, &mut allocator)?
                };
                ReflectReference::into_script_ref(reference, world)
            }
            None => Ok(ScriptValue::Unit),
        }
    }

    /// Iterates over the reference, if the reference is an appropriate container type.
    ///
    /// Returns an "next" iterator function.
    ///
    /// The iterator function should be called until it returns `nil` to signal the end of the iteration.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to iterate over.
    /// Returns:
    /// * `iter`: The iterator function.
    fn iter(
        ctxt: FunctionCallContext,
        reference: ReflectReference,
    ) -> Result<DynamicScriptFunctionMut, InteropError> {
        profiling::function_scope!("iter");
        let world = ctxt.world()?;
        let mut len = reference.len(world.clone())?.unwrap_or_default();
        let mut infinite_iter = reference.into_iter_infinite();
        let iter_function = move || {
            // world is not thread safe, we can't capture it in the closure
            // or it will also be non-thread safe
            let world = ThreadWorldContainer.try_get_world()?;
            if len == 0 {
                return Ok(ScriptValue::Unit);
            }

            let (next_ref, _) = infinite_iter.next_ref();

            let converted = ReflectReference::into_script_ref(next_ref, world);
            // println!("idx: {idx:?}, converted: {converted:?}");
            len -= 1;
            // we stop once the reflection path is invalid
            converted
        };

        Ok(iter_function.into_dynamic_script_function_mut())
    }

    /// Lists the functions available on the reference.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to list the functions of.
    /// Returns:
    /// * `functions`: The functions available on the reference.
    fn functions(
        ctxt: FunctionCallContext,
        reference: ReflectReference,
    ) -> Result<Vec<Val<FunctionInfo>>, InteropError> {
        profiling::function_scope!("functions");
        let world = ctxt.world()?;
        let type_id = reference.tail_type_id(world.clone())?.or_fake_id();
        let functions = world
            .get_functions_on_type(type_id)
            .into_iter()
            .map(|(_, v)| Val::new(v.info.deref().clone()))
            .collect::<Vec<_>>();
        // convert to info
        Ok(functions)
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "script_type_registration_functions",
    core
)]
impl ScriptTypeRegistration {
    /// Retrieves the name of the type.
    ///
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `type_name`: The name of the type.
    fn type_name(registration: Ref<ScriptTypeRegistration>) -> String {
        profiling::function_scope!("type_name");
        registration.type_name().to_string()
    }

    /// Retrieves the short name of the type.
    /// The short name is a more human-readable version of the type name.
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `short_name`: The short name of the
    fn short_name(registration: Ref<ScriptTypeRegistration>) -> String {
        profiling::function_scope!("short_name");
        registration.short_name().to_string()
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "script_component_registration_functions",
    core
)]
impl ScriptComponentRegistration {
    /// Retrieves the name of the type.
    ///
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `type_name`: The name of the type.
    fn type_name(registration: Ref<ScriptComponentRegistration>) -> &'static str {
        profiling::function_scope!("type_name");
        registration.type_registration().type_name()
    }

    /// Retrieves the short name of the type.
    /// The short name is a more human-readable version of the type name.
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `short_name`: The short name of the
    fn short_name(registration: Ref<ScriptComponentRegistration>) -> &'static str {
        profiling::function_scope!("short_name");
        registration.type_registration().short_name()
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "script_resource_registration_functions",
    core
)]
impl ScriptResourceRegistration {
    /// Retrieves the name of the type.
    ///
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `type_name`: The name of the type.
    fn type_name(registration: Ref<ScriptResourceRegistration>) -> &'static str {
        profiling::function_scope!("type_name");
        registration.type_registration().type_name()
    }

    /// Retrieves the short name of the type.
    /// The short name is a more human-readable version of the type name.
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `short_name`: The short name of the
    fn short_name(registration: Ref<ScriptResourceRegistration>) -> &'static str {
        profiling::function_scope!("short_name");
        registration.type_registration().short_name()
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "script_query_builder_functions",
    core
)]
impl ScriptQueryBuilder {
    /// Adds a component to be retrieved by the query
    ///
    /// Arguments:
    /// * `query`: The query to add the component to
    /// * `component`: The component to add
    /// Returns:
    /// * `query`: The query with the component added
    fn component(
        query: Val<ScriptQueryBuilder>,
        components: Val<ScriptComponentRegistration>,
    ) -> Val<ScriptQueryBuilder> {
        profiling::function_scope!("component");
        let mut builder = query.into_inner();
        builder.component(components.into_inner());
        Val(builder)
    }

    /// Adds a component to filter the query by. This component will NOT be retrieved.
    ///
    /// Arguments:
    /// * `query`: The query to add the component to
    /// * `component`: The component to filter by
    /// Returns:
    /// * `query`: The query with the component added
    fn with(
        query: Val<ScriptQueryBuilder>,
        with: Val<ScriptComponentRegistration>,
    ) -> Val<ScriptQueryBuilder> {
        profiling::function_scope!("with");
        let mut builder = query.into_inner();
        builder.with_component(with.into_inner());
        Val(builder)
    }

    /// Adds a component to filter the query by. This component will NOT be retrieved.
    ///
    /// Arguments:
    /// * `query`: The query to add the component to
    /// * `component`: The component to filter by
    /// Returns:
    /// * `query`: The query with the component added
    fn without(
        query: Val<ScriptQueryBuilder>,
        without: Val<ScriptComponentRegistration>,
    ) -> Val<ScriptQueryBuilder> {
        profiling::function_scope!("without");
        let mut builder = query.into_inner();
        builder.without_component(without.into_inner());
        Val(builder)
    }

    /// Builds the query and retrieves the entities and component references.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `query`: The query to build.
    /// Returns:
    /// * `result`: The entities and component references that match the query.
    fn build(
        ctxt: FunctionCallContext,
        query: Val<ScriptQueryBuilder>,
    ) -> Result<Vec<Val<ScriptQueryResult>>, InteropError> {
        profiling::function_scope!("build");
        let world = ctxt.world()?;
        let builder = query.into_inner();
        let result = world.query(builder)?;
        let result = result.into_iter().map(Val).collect::<Vec<_>>();
        Ok(result)
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "script_query_result_functions",
    core
)]
impl ScriptQueryResult {
    /// Retrieves the entity from the query result.
    ///
    /// Arguments:
    /// * `query`: The query result to retrieve the entity from.
    /// Returns:
    /// * `entity`: The entity from the query result.
    fn entity(query: Ref<ScriptQueryResult>) -> Val<Entity> {
        profiling::function_scope!("entity");
        Val::new(query.entity)
    }

    /// Retrieves the components from the query result.
    ///
    /// These are ordered by the order they were added to the query.
    ///
    /// Arguments:
    /// * `query`: The query result to retrieve the components from.
    /// Returns:
    /// * `components`: The components from the query result.
    fn components(query: Ref<ScriptQueryResult>) -> Vec<ReflectReference> {
        profiling::function_scope!("components");
        query.components.to_vec()
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "reflect_schedule_functions",
    core
)]
impl ReflectSchedule {
    /// Retrieves all the systems in the schedule.
    ///
    /// Arguments:
    /// * `schedule`: The schedule to retrieve the systems from.
    /// Returns:
    /// * `systems`: The systems in the schedule.
    fn systems(
        ctxt: FunctionCallContext,
        schedule: Ref<ReflectSchedule>,
    ) -> Result<Vec<Val<ReflectSystem>>, InteropError> {
        profiling::function_scope!("systems");
        let world = ctxt.world()?;
        let systems = world.systems(&schedule);
        Ok(systems?.into_iter().map(Into::into).collect())
    }

    /// Retrieves the system with the given name in the schedule
    ///
    /// Arguments:
    /// * `schedule`: The schedule to retrieve the system from.
    /// * `name`: The identifier or full path of the system to retrieve.
    /// Returns:
    /// * `system`: The system with the given name, if it exists.
    fn get_system_by_name(
        ctxt: FunctionCallContext,
        schedule: Ref<ReflectSchedule>,
        name: String,
    ) -> Result<Option<Val<ReflectSystem>>, InteropError> {
        profiling::function_scope!("system_by_name");
        let world = ctxt.world()?;
        let system = world.systems(&schedule)?;
        Ok(system
            .into_iter()
            .find_map(|s| (s.identifier() == name || s.path() == name).then_some(s.into())))
    }

    /// Renders the schedule as a dot graph string.
    ///
    /// Useful for debugging scheduling.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context
    /// * `schedule`: The schedule to render.
    /// Returns:
    /// * `dot`: The dot graph string.
    fn render_dot(
        ctxt: FunctionCallContext,
        schedule: Ref<ReflectSchedule>,
    ) -> Result<String, InteropError> {
        profiling::function_scope!("render_dot");
        let world = ctxt.world()?;
        world.with_resource(|schedules: &Schedules| {
            let schedule = schedules
                .get(*schedule.label())
                .ok_or_else(|| InteropError::missing_schedule(schedule.identifier()))?;
            let mut graph = bevy_system_reflection::schedule_to_reflect_graph(schedule);
            graph.absorb_type_system_sets();
            graph.sort();
            let graph = bevy_system_reflection::reflect_graph_to_dot(graph);
            Ok(graph)
        })?
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "reflect_system_functions",
    core
)]
impl ReflectSystem {
    /// Retrieves the identifier of the system
    /// Arguments:
    /// * `system`: The system to retrieve the identifier from.
    /// Returns:
    /// * `identifier`: The identifier of the system, e.g. `my_system`
    fn identifier(system: Ref<ReflectSystem>) -> String {
        profiling::function_scope!("identifier");
        system.identifier().to_string()
    }

    /// Retrieves the full path of the system
    /// Arguments:
    /// * `system`: The system to retrieve the path from.
    /// Returns:
    /// * `path`: The full path of the system, e.g. `my_crate::systems::my_system<T>`
    fn path(system: Ref<ReflectSystem>) -> String {
        profiling::function_scope!("path");
        system.path().to_string()
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "script_system_builder_functions",
    core
)]
impl ScriptSystemBuilder {
    /// Adds a query to the system builder.
    ///
    /// Arguments:
    /// * `builder`: The system builder to add the query to.
    /// * `query`: The query to add.
    /// Returns:
    /// * `builder`: The system builder with the query added.
    fn query(
        builder: Val<ScriptSystemBuilder>,
        query: Val<ScriptQueryBuilder>,
    ) -> Result<Val<ScriptSystemBuilder>, InteropError> {
        profiling::function_scope!("query");
        let mut builder = builder.into_inner();
        builder.query(query.into_inner());
        Ok(builder.into())
    }

    /// Requests the system have access to the given resource. The resource will be added to the
    /// list of arguments of the callback in the order they're provided.
    /// Arguments:
    /// * `builder`: The system builder to add the resource to.
    /// * `resource`: The resource to add.
    /// Returns:
    /// * `builder`: The system builder with the resource added.
    fn resource(
        builder: Val<ScriptSystemBuilder>,
        resource: Val<ScriptResourceRegistration>,
    ) -> Val<ScriptSystemBuilder> {
        profiling::function_scope!("resource");
        let mut builder = builder.into_inner();
        builder.resource(resource.into_inner());
        builder.into()
    }

    /// Specifies the system is to run exclusively, meaning it can access anything, but will not run in parallel with other systems.
    /// Arguments:
    /// * `builder`: The system builder to make exclusive.
    /// Returns:
    /// * `builder`: The system builder that is now exclusive.
    fn exclusive(builder: Val<ScriptSystemBuilder>) -> Val<ScriptSystemBuilder> {
        profiling::function_scope!("exclusive");
        let mut builder = builder.into_inner();
        builder.exclusive(true);
        builder.into()
    }

    /// Specifies the system is to run *after* the given system
    ///
    /// Note: this is an experimental feature, and the ordering might not work correctly for script initialized systems
    ///
    /// Arguments:
    /// * `builder`: The system builder to add the dependency to.
    /// * `system`: The system to run after.
    /// Returns:
    /// * `builder`: The system builder with the dependency added.
    fn after(
        builder: Val<ScriptSystemBuilder>,
        system: Val<ReflectSystem>,
    ) -> Val<ScriptSystemBuilder> {
        profiling::function_scope!("after");
        let mut builder = builder.into_inner();
        builder.after_system(system.into_inner());
        Val(builder)
    }

    /// Specifies the system is to run *before* the given system.
    ///
    /// Note: this is an experimental feature, and the ordering might not work correctly for script initialized systems
    ///
    /// Arguments:
    /// * `builder`: The system builder to add the dependency to.
    /// * `system`: The system to run before.
    /// Returns:
    /// * `builder`: The system builder with the dependency added.
    fn before(
        builder: Val<ScriptSystemBuilder>,
        system: Val<ReflectSystem>,
    ) -> Val<ScriptSystemBuilder> {
        profiling::function_scope!("before");
        let mut builder = builder.into_inner();
        builder.before_system(system.into_inner());
        Val(builder)
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "global_namespace_functions",
    unregistered
)]
impl GlobalNamespace {
    /// Attempts to construct the given type, given an arbitrary map of values.
    ///
    /// Arguments:
    /// * `registration`: The type to construct.
    /// * `payload`: The values to use to construct the type.
    /// Returns:
    /// * `reference`: The constructed type.
    fn construct(
        ctxt: FunctionCallContext,
        registration: Union<
            Val<ScriptTypeRegistration>,
            Union<Val<ScriptComponentRegistration>, Val<ScriptResourceRegistration>>,
        >,
        payload: HashMap<String, ScriptValue>,
    ) -> Result<ReflectReference, InteropError> {
        let registration = match registration.into_left() {
            Ok(l) => l.into_inner(),
            Err(r) => match r.into_left() {
                Ok(l) => (l.into_inner()).into_type_registration(),
                Err(r) => (r.into_inner()).into_type_registration(),
            },
        };

        let world = ctxt.world()?;
        let one_indexed = ctxt.convert_to_0_indexed();

        let val = world.construct(registration.clone(), payload, one_indexed)?;
        let allocator = world.allocator();
        let mut allocator = allocator.write();
        let reflect_val = val.try_into_reflect().map_err(|_| {
            InteropError::failed_from_reflect(
                Some(registration.type_id()),
                "Could not construct the type",
            )
        })?;

        Ok(ReflectReference::new_allocated_boxed(
            reflect_val,
            &mut allocator,
        ))
    }

    /// Creates a new script system builder, which can be used to add new systems to the world.
    ///
    /// Arguments:
    /// * `callback`: The function name in the script this system should call when run.
    /// * `script_id`: The id of the script this system will execute when run.
    /// Returns:
    /// * `builder`: The system builder
    fn system_builder(
        callback: String,
        script_id: String,
    ) -> Result<Val<ScriptSystemBuilder>, InteropError> {
        Ok(ScriptSystemBuilder::new(callback.into(), script_id.into()).into())
    }
}

pub fn register_core_functions(app: &mut App) {
    let world = app.world_mut();
    // we don't exclude from compilation here,
    // since these are much smaller and still useful if not included initially
    // perhaps people might want to include some but not all of these

    #[cfg(feature = "core_functions")]
    {
        register_world_functions(world);

        register_reflect_reference_functions(world);

        register_script_type_registration_functions(world);
        register_script_component_registration_functions(world);
        register_script_resource_registration_functions(world);

        register_script_query_builder_functions(world);
        register_script_query_result_functions(world);

        register_reflect_schedule_functions(world);
        register_reflect_system_functions(world);
        register_script_system_builder_functions(world);

        register_global_namespace_functions(world);
    }
}
