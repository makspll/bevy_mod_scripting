//! Contains functions defined by the [`bevy_mod_scripting_core`] crate

use std::collections::HashMap;

use bevy::{prelude::*, reflect::ParsedPath};
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
    ReflectReference, ReflectionPathExt, ScriptComponentRegistration, ScriptQueryBuilder,
    ScriptQueryResult, ScriptResourceRegistration, ScriptTypeRegistration, ThreadWorldContainer,
    WorldContainer,
};
use error::InteropError;
use reflection_extensions::{PartialReflectExt, TypeIdExtensions};

pub fn register_bevy_bindings(app: &mut App) {
    #[cfg(feature = "bevy_bindings")]
    app.add_plugins(crate::bevy_bindings::LuaBevyScriptingPlugin);
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "world_functions",
    unregistered
)]
impl World {
    fn get_type_by_name(
        ctxt: FunctionCallContext,
        type_name: String,
    ) -> Result<Option<Union<Val<ScriptTypeRegistration>, Union<Val<ScriptComponentRegistration>, Val<ScriptResourceRegistration>>>>, InteropError> {
        profiling::function_scope!("get_type_by_name");
        let world = ctxt.world()?;
        world.get_type_registration_by_name(type_name).map(|v| v.map(|v| v.map_both(Val::from, |u| u.map_both(Val::from, Val::from))))
    }

    /// Retrieves the schedule with the given name, Also ensures the schedule is initialized before returning it.
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

    fn get_component(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
        registration: Val<ScriptComponentRegistration>,
    ) -> Result<Option<ReflectReference>, InteropError> {
        profiling::function_scope!("get_component");
        let world = ctxt.world()?;
        let val = world.get_component(*entity, registration.component_id())?;
        Ok(val)
    }

    fn has_component(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
        registration: Val<ScriptComponentRegistration>,
    ) -> Result<bool, InteropError> {
        profiling::function_scope!("has_component");
        let world = ctxt.world()?;
        world.has_component(*entity, registration.component_id())
    }

    fn remove_component(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
        registration: Val<ScriptComponentRegistration>,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("remove_component");
        let world = ctxt.world()?;
        world.remove_component(*entity, registration.clone())
    }

    fn get_resource(
        ctxt: FunctionCallContext,
        registration: Val<ScriptResourceRegistration>,
    ) -> Result<Option<ReflectReference>, InteropError> {
        profiling::function_scope!("get_resource");
        let world = ctxt.world()?;
        let val = world.get_resource(registration.resource_id())?;
        Ok(val)
    }

    fn has_resource(
        ctxt: FunctionCallContext,
        registration: Val<ScriptResourceRegistration>,
    ) -> Result<bool, InteropError> {
        profiling::function_scope!("has_resource");
        let world = ctxt.world()?;
        world.has_resource(registration.resource_id())
    }

    fn remove_resource(
        ctxt: FunctionCallContext,
        registration: Val<ScriptResourceRegistration>,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("remove_resource");
        let world = ctxt.world()?;
        world.remove_resource(registration.into_inner())
    }

    fn add_default_component(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
        registration: Val<ScriptComponentRegistration>,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("add_default_component");
        let world = ctxt.world()?;
        world.add_default_component(*entity, registration.clone())
    }

    fn spawn(ctxt: FunctionCallContext) -> Result<Val<Entity>, InteropError> {
        profiling::function_scope!("spawn");
        let world = ctxt.world()?;
        Ok(Val(world.spawn()?))
    }

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

    fn get_children(
        ctxt: FunctionCallContext,
        entity: Val<Entity>,
    ) -> Result<Vec<Val<Entity>>, InteropError> {
        profiling::function_scope!("get_children");
        let world = ctxt.world()?;
        let children = world.get_children(*entity)?;
        Ok(children.into_iter().map(Val).collect::<Vec<_>>())
    }

    fn get_parent(
        ctxt: FunctionCallContext,
        e: Val<Entity>,
    ) -> Result<Option<Val<Entity>>, InteropError> {
        profiling::function_scope!("get_parent");
        let world = ctxt.world()?;
        let parent = world.get_parent(*e)?;
        Ok(parent.map(Val))
    }

    fn despawn(ctxt: FunctionCallContext, e: Val<Entity>) -> Result<(), InteropError> {
        profiling::function_scope!("despawn");
        let world = ctxt.world()?;
        world.despawn(*e)
    }

    fn despawn_descendants(ctxt: FunctionCallContext, e: Val<Entity>) -> Result<(), InteropError> {
        profiling::function_scope!("despawn_descendants");
        let world = ctxt.world()?;
        world.despawn_descendants(*e)
    }

    fn despawn_recursive(ctxt: FunctionCallContext, e: Val<Entity>) -> Result<(), InteropError> {
        profiling::function_scope!("despawn_recursive");
        let world = ctxt.world()?;
        world.despawn_recursive(*e)
    }

    fn has_entity(ctxt: FunctionCallContext, e: Val<Entity>) -> Result<bool, InteropError> {
        profiling::function_scope!("has_entity");
        let world = ctxt.world()?;
        world.has_entity(*e)
    }

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

    fn exit(ctxt: FunctionCallContext) -> Result<(), InteropError> {
        profiling::function_scope!("exit");
        let world = ctxt.world()?;
        world.exit()
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "reflect_reference_functions"
)]
impl ReflectReference {
    /// If this type is an enum, will return the name of the variant it represents on the type.
    fn variant_name(
        ctxt: FunctionCallContext,
        s: ReflectReference,
    ) -> Result<Option<String>, InteropError> {
        profiling::function_scope!("variant_name");
        let world = ctxt.world()?;
        s.variant_name(world)
    }

    /// Displays this reference without printing the exact contents.
    fn display_ref(ctxt: FunctionCallContext, s: ReflectReference) -> Result<String, InteropError> {
        profiling::function_scope!("display_ref");
        let world = ctxt.world()?;
        Ok(s.display_with_world(world))
    }

    /// Displays the "value" of this reference
    fn display_value(
        ctxt: FunctionCallContext,
        s: ReflectReference,
    ) -> Result<String, InteropError> {
        profiling::function_scope!("display_value");
        let world = ctxt.world()?;
        Ok(s.display_value_with_world(world))
    }

    /// Gets and clones the value under the specified key if the underlying type is a map type.
    fn map_get(
        ctxt: FunctionCallContext,
        self_: ReflectReference,
        key: ScriptValue,
    ) -> Result<Option<ScriptValue>, InteropError> {
        profiling::function_scope!("map_get");
        let world = ctxt.world()?;
        let key = <Box<dyn PartialReflect>>::from_script_ref(
            self_.key_type_id(world.clone())?.ok_or_else(|| {
                InteropError::unsupported_operation(
                    self_.tail_type_id(world.clone()).unwrap_or_default(),
                    Some(Box::new(key.clone())),
                    "Could not get key type id. Are you trying to index into a type that's not a map?".to_owned(),
                )
            })?,
            key,
            world.clone(),
        )?;
        self_.with_reflect_mut(world.clone(), |s| match s.try_map_get(key.as_ref())? {
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

    /// Indexes into the given reference and if the nested type is a reference type, returns a deeper reference, otherwise
    /// returns the concrete value.
    ///
    /// Does not support map types at the moment, for maps see `map_get`
    fn get(
        ctxt: FunctionCallContext,
        mut self_: ReflectReference,
        key: ScriptValue,
    ) -> Result<ScriptValue, InteropError> {
        profiling::function_scope!("get");
        let mut path: ParsedPath = key.try_into()?;
        if ctxt.convert_to_0_indexed() {
            path.convert_to_0_indexed();
        }
        self_.index_path(path);
        let world = ctxt.world()?;
        ReflectReference::into_script_ref(self_, world)
    }

    /// Sets the value under the specified path on the underlying value.
    fn set(
        ctxt: FunctionCallContext,
        self_: ScriptValue,
        key: ScriptValue,
        value: ScriptValue,
    ) -> Result<ScriptValue, InteropError> {
        profiling::function_scope!("set");
        if let ScriptValue::Reference(mut self_) = self_ {
            let world = ctxt.world()?;
            let mut path: ParsedPath = key.try_into()?;
            if ctxt.convert_to_0_indexed() {
                path.convert_to_0_indexed();
            }
            self_.index_path(path);
            let r: ScriptValue = self_
                .with_reflect_mut(world.clone(), |r| {
                    let target_type_id = r
                        .get_represented_type_info()
                        .map(|i| i.type_id())
                        .or_fake_id();
                    let other = <Box<dyn PartialReflect>>::from_script_ref(
                        target_type_id,
                        value,
                        world.clone(),
                    )?;
                    r.try_apply(other.as_partial_reflect())
                        .map_err(|e| InteropError::external_error(Box::new(e)))?;
                    Ok::<_, InteropError>(())
                })
                .into();
            return Ok(r);
        }
        Ok(ScriptValue::Unit)
    }

    /// Pushes the value into the reference, if the reference is an appropriate container type.
    fn push(
        ctxt: FunctionCallContext,
        s: ReflectReference,
        v: ScriptValue,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("push");
        let world = ctxt.world()?;
        let target_type_id = s.element_type_id(world.clone())?.ok_or_else(|| {
            InteropError::unsupported_operation(
                s.tail_type_id(world.clone()).unwrap_or_default(),
                Some(Box::new(v.clone())),
                "Could not get element type id. Are you trying to insert elements into a type that's not a list?".to_owned(),
            )
        })?;
        let other = <Box<dyn PartialReflect>>::from_script_ref(target_type_id, v, world.clone())?;
        s.with_reflect_mut(world, |s| s.try_push_boxed(other))?
    }

    /// Pops the value from the reference, if the reference is an appropriate container type.
    fn pop(ctxt: FunctionCallContext, s: ReflectReference) -> Result<ScriptValue, InteropError> {
        profiling::function_scope!("pop");
        let world = ctxt.world()?;
        let o = s.with_reflect_mut(world.clone(), |s| s.try_pop_boxed())??;
        let reference = {
            let allocator = world.allocator();
            let mut allocator = allocator.write();
            ReflectReference::new_allocated_boxed_parial_reflect(o, &mut allocator)?
        };

        ReflectReference::into_script_ref(reference, world)
    }

    /// Inserts the value into the reference at the specified index, if the reference is an appropriate container type.
    fn insert(
        ctxt: FunctionCallContext,
        s: ReflectReference,
        k: ScriptValue,
        v: ScriptValue,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("insert");
        let world = ctxt.world()?;
        let key_type_id = s.key_type_id(world.clone())?.ok_or_else(|| {
            InteropError::unsupported_operation(
                s.tail_type_id(world.clone()).unwrap_or_default(),
                Some(Box::new(k.clone())),
                "Could not get key type id. Are you trying to insert elements into a type that's not a map?".to_owned(),
            )
        })?;

        let mut key = <Box<dyn PartialReflect>>::from_script_ref(key_type_id, k, world.clone())?;

        if ctxt.convert_to_0_indexed() {
            key.convert_to_0_indexed_key();
        }

        let value_type_id = s.element_type_id(world.clone())?.ok_or_else(|| {
            InteropError::unsupported_operation(
                s.tail_type_id(world.clone()).unwrap_or_default(),
                Some(Box::new(v.clone())),
                "Could not get element type id. Are you trying to insert elements into a type that's not a map?".to_owned(),
            )
        })?;

        let value = <Box<dyn PartialReflect>>::from_script_ref(value_type_id, v, world.clone())?;

        s.with_reflect_mut(world, |s| s.try_insert_boxed(key, value))?
    }

    /// Clears the container, if the reference is an appropriate container type.
    fn clear(ctxt: FunctionCallContext, s: ReflectReference) -> Result<(), InteropError> {
        profiling::function_scope!("clear");
        let world = ctxt.world()?;
        s.with_reflect_mut(world, |s| s.try_clear())?
    }

    /// Retrieves the length of the reference, if the reference is an appropriate container type.
    fn len(ctxt: FunctionCallContext, s: ReflectReference) -> Result<Option<usize>, InteropError> {
        profiling::function_scope!("len");
        let world = ctxt.world()?;
        s.len(world)
    }

    /// Removes the value at the specified key from the reference, if the reference is an appropriate container type.
    fn remove(
        ctxt: FunctionCallContext,
        s: ReflectReference,
        k: ScriptValue,
    ) -> Result<ScriptValue, InteropError> {
        profiling::function_scope!("remove");
        let world = ctxt.world()?;
        let key_type_id = s.key_type_id(world.clone())?.ok_or_else(|| {
            InteropError::unsupported_operation(
                s.tail_type_id(world.clone()).unwrap_or_default(),
                Some(Box::new(k.clone())),
                "Could not get key type id. Are you trying to remove elements from a type that's not a map?".to_owned(),
            )
        })?;

        let mut key = <Box<dyn PartialReflect>>::from_script_ref(key_type_id, k, world.clone())?;

        if ctxt.convert_to_0_indexed() {
            key.convert_to_0_indexed_key();
        }

        let removed = s.with_reflect_mut(world.clone(), |s| s.try_remove_boxed(key))??;
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
    fn iter(
        ctxt: FunctionCallContext,
        s: ReflectReference,
    ) -> Result<DynamicScriptFunctionMut, InteropError> {
        profiling::function_scope!("iter");
        let world = ctxt.world()?;
        let mut len = s.len(world.clone())?.unwrap_or_default();
        let mut infinite_iter = s.into_iter_infinite();
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
    fn functions(
        ctxt: FunctionCallContext,
        s: ReflectReference,
    ) -> Result<Vec<Val<FunctionInfo>>, InteropError> {
        profiling::function_scope!("functions");
        let world = ctxt.world()?;
        let type_id = s.tail_type_id(world.clone())?.or_fake_id();
        let functions = world
            .get_functions_on_type(type_id)
            .into_iter()
            .map(|(_, v)| Val::new(v.info))
            .collect::<Vec<_>>();
        // convert to info
        Ok(functions)
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "script_type_registration_functions"
)]
impl ScriptTypeRegistration {
    fn type_name(s: Ref<ScriptTypeRegistration>) -> String {
        profiling::function_scope!("type_name");
        s.type_name().to_string()
    }

    fn short_name(s: Ref<ScriptTypeRegistration>) -> String {
        profiling::function_scope!("short_name");
        s.short_name().to_string()
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "script_component_registration_functions"
)]
impl ScriptComponentRegistration {
    fn type_name(s: Ref<ScriptComponentRegistration>) -> &'static str {
        profiling::function_scope!("type_name");
        s.type_registration().type_name()
    }

    fn short_name(s: Ref<ScriptComponentRegistration>) -> &'static str {
        profiling::function_scope!("short_name");
        s.type_registration().short_name()
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "script_resource_registration_functions"
)]
impl ScriptResourceRegistration {
    fn type_name(s: Ref<ScriptResourceRegistration>) -> &'static str {
        profiling::function_scope!("type_name");
        s.type_registration().type_name()
    }

    fn short_name(s: Ref<ScriptResourceRegistration>) -> &'static str {
        profiling::function_scope!("short_name");
        s.type_registration().short_name()
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "script_query_builder_functions"
)]
impl ScriptQueryBuilder {
    fn component(
        s: Val<ScriptQueryBuilder>,
        components: Val<ScriptComponentRegistration>,
    ) -> Val<ScriptQueryBuilder> {
        profiling::function_scope!("component");
        let mut builder = s.into_inner();
        builder.component(components.into_inner());
        Val(builder)
    }

    fn with(
        s: Val<ScriptQueryBuilder>,
        with: Val<ScriptComponentRegistration>,
    ) -> Val<ScriptQueryBuilder> {
        profiling::function_scope!("with");
        let mut builder = s.into_inner();
        builder.with_component(with.into_inner());
        Val(builder)
    }

    fn without(
        s: Val<ScriptQueryBuilder>,
        without: Val<ScriptComponentRegistration>,
    ) -> Val<ScriptQueryBuilder> {
        profiling::function_scope!("without");
        let mut builder = s.into_inner();
        builder.without_component(without.into_inner());
        Val(builder)
    }

    fn build(
        ctxt: FunctionCallContext,
        s: Val<ScriptQueryBuilder>,
    ) -> Result<Vec<Val<ScriptQueryResult>>, InteropError> {
        profiling::function_scope!("build");
        let world = ctxt.world()?;
        let builder = s.into_inner();
        let result = world.query(builder)?;
        let result = result.into_iter().map(Val).collect::<Vec<_>>();
        Ok(result)
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "script_query_result_functions"
)]
impl ScriptQueryResult {
    fn entity(s: Ref<ScriptQueryResult>) -> Val<Entity> {
        profiling::function_scope!("entity");
        Val::new(s.entity)
    }

    fn components(s: Ref<ScriptQueryResult>) -> Vec<ReflectReference> {
        profiling::function_scope!("components");
        s.components.to_vec()
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "reflect_schedule_functions"
)]
impl ReflectSchedule {
    /// Retrieves all the systems in the schedule.
    ///
    /// Arguments:
    /// * `self_`: The schedule to retrieve the systems from.
    /// Returns:
    /// * `systems`: The systems in the schedule.
    fn systems(
        ctxt: FunctionCallContext,
        self_: Ref<ReflectSchedule>,
    ) -> Result<Vec<Val<ReflectSystem>>, InteropError> {
        profiling::function_scope!("systems");
        let world = ctxt.world()?;
        let systems = world.systems(&self_);
        Ok(systems?.into_iter().map(Into::into).collect())
    }

    /// Retrieves the system with the given name in the schedule
    ///
    /// Arguments:
    /// * `self_`: The schedule to retrieve the system from.
    /// * `name`: The identifier or full path of the system to retrieve.
    /// Returns:
    /// * `system`: The system with the given name, if it exists.
    fn get_system_by_name(
        ctxt: FunctionCallContext,
        self_: Ref<ReflectSchedule>,
        name: String,
    ) -> Result<Option<Val<ReflectSystem>>, InteropError> {
        profiling::function_scope!("system_by_name");
        let world = ctxt.world()?;
        let system = world.systems(&self_)?;
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
    /// * `self_`: The schedule to render.
    /// Returns:
    /// * `dot`: The dot graph string.
    fn render_dot(
        ctxt: FunctionCallContext,
        self_: Ref<ReflectSchedule>,
    ) -> Result<String, InteropError> {
        profiling::function_scope!("render_dot");
        let world = ctxt.world()?;
        world.with_resource(|schedules: &Schedules| {
            let schedule = schedules
                .get(*self_.label())
                .ok_or_else(|| InteropError::missing_schedule(self_.identifier()))?;
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
    name = "reflect_system_functions"
)]
impl ReflectSystem {
    /// Retrieves the identifier of the system
    /// Arguments:
    /// * `self_`: The system to retrieve the identifier from.
    /// Returns:
    /// * `identifier`: The identifier of the system, e.g. `my_system`
    fn identifier(self_: Ref<ReflectSystem>) -> String {
        profiling::function_scope!("identifier");
        self_.identifier().to_string()
    }

    /// Retrieves the full path of the system
    /// Arguments:
    /// * `self_`: The system to retrieve the path from.
    /// Returns:
    /// * `path`: The full path of the system, e.g. `my_crate::systems::my_system<T>`
    fn path(self_: Ref<ReflectSystem>) -> String {
        profiling::function_scope!("path");
        self_.path().to_string()
    }
}

#[script_bindings(
    remote,
    bms_core_path = "bevy_mod_scripting_core",
    name = "script_system_builder_functions"
)]
impl ScriptSystemBuilder {
    fn query(
        self_: Val<ScriptSystemBuilder>,
        query: Val<ScriptQueryBuilder>,
    ) -> Result<Val<ScriptSystemBuilder>, InteropError> {
        profiling::function_scope!("query");
        let mut builder = self_.into_inner();
        builder.query(query.into_inner());
        Ok(builder.into())
    }

    /// Requests the system have access to the given resource. The resource will be added to the
    /// list of arguments of the callback in the order they're provided.
    /// Arguments:
    /// * `self_`: The system builder to add the resource to.
    /// * `resource`: The resource to add.
    /// Returns:
    /// * `builder`: The system builder with the resource added.
    fn resource(
        self_: Val<ScriptSystemBuilder>,
        resource: Val<ScriptResourceRegistration>,
    ) -> Val<ScriptSystemBuilder> {
        profiling::function_scope!("resource");
        let mut builder = self_.into_inner();
        builder.resource(resource.into_inner());
        builder.into()
    }

    /// Specifies the system is to run exclusively, meaning it can access anything, but will not run in parallel with other systems.
    /// Arguments:
    /// * `self_`: The system builder to make exclusive.
    /// Returns:
    /// * `builder`: The system builder that is now exclusive.
    fn exclusive(self_: Val<ScriptSystemBuilder>) -> Val<ScriptSystemBuilder> {
        profiling::function_scope!("exclusive");
        let mut builder = self_.into_inner();
        builder.exclusive(true);
        builder.into()
    }

    /// Specifies the system is to run *after* the given system
    ///
    /// Note: this is an experimental feature, and the ordering might not work correctly for script initialized systems
    ///
    /// Arguments:
    /// * `self_`: The system builder to add the dependency to.
    /// * `system`: The system to run after.
    /// Returns:
    /// * `builder`: The system builder with the dependency added.
    fn after(
        self_: Val<ScriptSystemBuilder>,
        system: Val<ReflectSystem>,
    ) -> Val<ScriptSystemBuilder> {
        profiling::function_scope!("after");
        let mut builder = self_.into_inner();
        builder.after_system(system.into_inner());
        Val(builder)
    }

    /// Specifies the system is to run *before* the given system.
    ///
    /// Note: this is an experimental feature, and the ordering might not work correctly for script initialized systems
    ///
    /// Arguments:
    /// * `self_`: The system builder to add the dependency to.
    /// * `system`: The system to run before.
    /// Returns:
    /// * `builder`: The system builder with the dependency added.
    fn before(
        self_: Val<ScriptSystemBuilder>,
        system: Val<ReflectSystem>,
    ) -> Val<ScriptSystemBuilder> {
        profiling::function_scope!("before");
        let mut builder = self_.into_inner();
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
                "Could not construct the type".into(),
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
