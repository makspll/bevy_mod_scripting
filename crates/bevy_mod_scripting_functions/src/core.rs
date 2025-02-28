//! Contains functions defined by the [`bevy_mod_scripting_core`] crate

use std::collections::HashMap;

use bevy::{prelude::*, reflect::ParsedPath};
use bevy_mod_scripting_core::{
    bindings::{
        function::{
            from::Union, namespace::GlobalNamespace, script_function::DynamicScriptFunctionMut,
        },
        schedule::{ReflectSchedule, ReflectSystem, ScriptSystemBuilder},
    },
    docgen::info::FunctionInfo,
    *,
};
use bevy_mod_scripting_derive::script_bindings;
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
    ) -> Result<Option<ReflectReference>, InteropError> {
        profiling::function_scope!("get_type_by_name");
        let world = ctxt.world()?;
        let val = world.get_type_by_name(type_name);

        Ok(match val {
            Some(registration) => {
                let allocator = world.allocator();

                let registration = match world.get_resource_type(registration)? {
                    Ok(res) => {
                        let mut allocator = allocator.write();
                        return Ok(Some(ReflectReference::new_allocated(res, &mut allocator)));
                    }
                    Err(registration) => registration,
                };

                let registration = match world.get_component_type(registration)? {
                    Ok(comp) => {
                        let mut allocator = allocator.write();
                        return Ok(Some(ReflectReference::new_allocated(comp, &mut allocator)));
                    }
                    Err(registration) => registration,
                };

                let mut allocator = allocator.write();
                Some(ReflectReference::new_allocated(
                    registration,
                    &mut allocator,
                ))
            }
            None => None,
        })
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
    fn variant_name(
        ctxt: FunctionCallContext,
        s: ReflectReference,
    ) -> Result<Option<String>, InteropError> {
        profiling::function_scope!("variant_name");
        let world = ctxt.world()?;
        s.variant_name(world)
    }

    fn display_ref(ctxt: FunctionCallContext, s: ReflectReference) -> Result<String, InteropError> {
        profiling::function_scope!("display_ref");
        let world = ctxt.world()?;
        Ok(s.display_with_world(world))
    }

    fn display_value(
        ctxt: FunctionCallContext,
        s: ReflectReference,
    ) -> Result<String, InteropError> {
        profiling::function_scope!("display_value");
        let world = ctxt.world()?;
        Ok(s.display_value_with_world(world))
    }

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

    fn clear(ctxt: FunctionCallContext, s: ReflectReference) -> Result<(), InteropError> {
        profiling::function_scope!("clear");
        let world = ctxt.world()?;
        s.with_reflect_mut(world, |s| s.try_clear())?
    }

    fn len(ctxt: FunctionCallContext, s: ReflectReference) -> Result<Option<usize>, InteropError> {
        profiling::function_scope!("len");
        let world = ctxt.world()?;
        s.len(world)
    }

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
        builder.after(system.into_inner());
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
        builder.before(system.into_inner());
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

    /// Creates a new script function builder
    /// Arguments:
    /// * `callback`: The functio name in the script, this system will call
    /// * `script_id`: The id of the script
    /// Returns:
    /// * `builder`: The new system builder
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
