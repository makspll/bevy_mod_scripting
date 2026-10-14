//! Contains functions defined by the [`bevy_mod_scripting_core`] crate

use bevy_mod_scripting_asset::ScriptAsset;
use bevy_mod_scripting_script::{Recipients, ScriptAttachment};
use bevy_mod_scripting_world::ThreadWorldContainer;
use std::{collections::VecDeque, ops::Deref};

use bevy_app::App;
use bevy_asset::{AssetServer, Handle};
use bevy_ecs::{entity::Entity, prelude::AppTypeRegistry, schedule::Schedules, world::World};
use bevy_mod_scripting_bindings::{
    AnyTypeRegistrationV, DynamicScriptFunction, DynamicScriptFunctionMut, EnumVariantRef,
    FunctionInfo, GlobalNamespace, InteropError, PartialReflectExt, QueuedScriptCallback,
    ReflectReference, ScriptBundleRegistration, ScriptComponentRegistration,
    ScriptEventRegistration, ScriptQueryBuilder, ScriptQueryResult, ScriptResourceRegistration,
    ScriptTypeCatalogue, ScriptTypeRegistration, Union, VariadicTuple, WorldExtensions,
    construct::construct_reference,
    function::into::IntoScript,
    function::{
        bound::BoundFields,
        from::{FromScript, R, V},
        from_ref::FromScriptRef,
        into_ref::IntoScriptRef,
        script_function::{FunctionCallContext, ScriptFunctionMut},
    },
    script_path,
    script_value::ScriptValue,
};
use bevy_mod_scripting_core::{
    event::CallbackLabel,
    script_system::{ManageScriptSystems, ScriptSystemBuilder},
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_mod_scripting_display::{OrFakeId, WithTypeInfo};
use bevy_mod_scripting_world::WorldGuard;
use bevy_platform::collections::HashMap;
use bevy_reflect::PartialReflect;
use bevy_system_reflection::{ReflectSchedule, ReflectSystem};

#[allow(unused_variables, reason = "feature flags")]
pub fn register_bevy_bindings(app: &mut App) {
    #[cfg(feature = "bevy_a11y")]
    app.add_plugins(bevy_a11y_bms_bindings::BevyA11YScriptingPlugin);
    #[cfg(feature = "bevy_animation")]
    app.add_plugins(bevy_animation_bms_bindings::BevyAnimationScriptingPlugin);
    #[cfg(feature = "bevy_asset")]
    app.add_plugins(bevy_asset_bms_bindings::BevyAssetScriptingPlugin);
    #[cfg(feature = "bevy_camera")]
    app.add_plugins(bevy_camera_bms_bindings::BevyCameraScriptingPlugin);
    #[cfg(feature = "bevy_color")]
    app.add_plugins(bevy_color_bms_bindings::BevyColorScriptingPlugin);
    #[cfg(feature = "bevy_core_pipeline")]
    app.add_plugins(bevy_core_pipeline_bms_bindings::BevyCorePipelineScriptingPlugin);
    #[cfg(feature = "bevy_curve")]
    app.add_plugins(bevy_curve_bms_bindings::BevyCurveScriptingPlugin);
    #[cfg(feature = "bevy_ecs")]
    app.add_plugins(bevy_ecs_bms_bindings::BevyEcsScriptingPlugin);
    #[cfg(feature = "bevy_gizmos")]
    app.add_plugins(bevy_gizmos_bms_bindings::BevyGizmosScriptingPlugin);
    #[cfg(feature = "bevy_gltf")]
    app.add_plugins(bevy_gltf_bms_bindings::BevyGltfScriptingPlugin);
    #[cfg(feature = "bevy_image")]
    app.add_plugins(bevy_image_bms_bindings::BevyImageScriptingPlugin);
    #[cfg(feature = "bevy_input")]
    app.add_plugins(bevy_input_bms_bindings::BevyInputScriptingPlugin);
    #[cfg(feature = "bevy_input_focus")]
    app.add_plugins(bevy_input_focus_bms_bindings::BevyInputFocusScriptingPlugin);
    #[cfg(feature = "bevy_light")]
    app.add_plugins(bevy_light_bms_bindings::BevyLightScriptingPlugin);
    #[cfg(feature = "bevy_math")]
    app.add_plugins(bevy_math_bms_bindings::BevyMathScriptingPlugin);
    #[cfg(feature = "bevy_mesh")]
    app.add_plugins(bevy_mesh_bms_bindings::BevyMeshScriptingPlugin);
    #[cfg(feature = "bevy_pbr")]
    app.add_plugins(bevy_pbr_bms_bindings::BevyPbrScriptingPlugin);
    #[cfg(feature = "bevy_picking")]
    app.add_plugins(bevy_picking_bms_bindings::BevyPickingScriptingPlugin);
    #[cfg(feature = "bevy_post_process")]
    app.add_plugins(bevy_post_process_bms_bindings::BevyPostProcessScriptingPlugin);
    #[cfg(feature = "bevy_reflect")]
    app.add_plugins(bevy_reflect_bms_bindings::BevyReflectScriptingPlugin);
    #[cfg(feature = "bevy_render")]
    app.add_plugins(bevy_render_bms_bindings::BevyRenderScriptingPlugin);
    #[cfg(feature = "bevy_scene")]
    app.add_plugins(bevy_scene_bms_bindings::BevySceneScriptingPlugin);
    #[cfg(feature = "bevy_shape")]
    app.add_plugins(bevy_shape_bms_bindings::BevyShapeScriptingPlugin);
    #[cfg(feature = "bevy_sprite")]
    app.add_plugins(bevy_sprite_bms_bindings::BevySpriteScriptingPlugin);
    #[cfg(feature = "bevy_sprite_render")]
    app.add_plugins(bevy_sprite_renderer_bms_bindings::BevySpriteRendererScriptingPlugin);
    #[cfg(feature = "bevy_text")]
    app.add_plugins(bevy_text_bms_bindings::BevyTextScriptingPlugin);
    #[cfg(feature = "bevy_time")]
    app.add_plugins(bevy_time_bms_bindings::BevyTimeScriptingPlugin);
    #[cfg(feature = "bevy_transform")]
    app.add_plugins(bevy_transform_bms_bindings::BevyTransformScriptingPlugin);
    #[cfg(feature = "bevy_ui")]
    app.add_plugins(bevy_ui_bms_bindings::BevyUiScriptingPlugin);
    #[cfg(feature = "bevy_ui_render")]
    app.add_plugins(bevy_ui_renderer_bms_bindings::BevyUiRendererScriptingPlugin);
    #[cfg(feature = "bevy_ui_widgets")]
    app.add_plugins(bevy_ui_widgets_bms_bindings::BevyUiWidgetsScriptingPlugin);
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "world_functions",
    unregistered
)]
impl World {
    /// Returns either a `ScriptComponentRegistration`, `ScriptResourceRegistration` or `ScriptBundleRegistration` depending on the type of the type requested.
    /// If the type is none of these returns a `ScriptTypeRegistration`.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `type_name`: The name of the type to retrieve.
    /// Returns:
    /// * `type`: The registration of the type, if it exists.
    fn get_type_by_name(
        ctxt: FunctionCallContext,
        type_name: String,
    ) -> Result<Option<AnyTypeRegistrationV>, InteropError> {
        profiling::function_scope!("get_type_by_name");
        let world = ctxt.world()?;
        world
            .get_type_registration_by_name(type_name)
            .map(|v| v.map(|v| v.into_script()))
    }

    /// Retrieves the variant of an enum by name.
    ///
    /// This is the same as accessing `MyEnum.Variant`, but works for variants which share a name with a function.
    ///
    /// Arguments:
    /// * `registration`: The enum type.
    /// * `variant_name`: The name of the variant.
    /// Returns:
    /// * `variant`: The variant, if the type is an enum with such a variant.
    fn get_enum_variant(
        registration: AnyTypeRegistrationV,
        variant_name: String,
    ) -> Result<Option<V<EnumVariantRef>>, InteropError> {
        profiling::function_scope!("get_enum_variant");
        let type_info = registration
            .into_type_registration()
            .type_registration()
            .type_info();
        Ok(EnumVariantRef::from_name(type_info, &variant_name).map(V))
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
    ) -> Result<Option<V<ReflectSchedule>>, InteropError> {
        profiling::function_scope!("get_schedule_by_name");
        let world = ctxt.world()?;
        let schedule = match world.get_schedule_by_name(name) {
            Some(schedule) => schedule,
            None => return Ok(None),
        };

        // do two things, check it actually exists
        world
            .scope_schedule(&schedule, |world, schedule| schedule.initialize(world))?
            .map_err(InteropError::external)?;

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
        entity: V<Entity>,
        registration: V<ScriptComponentRegistration>,
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
        entity: V<Entity>,
        registration: V<ScriptComponentRegistration>,
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
        entity: V<Entity>,
        registration: V<ScriptComponentRegistration>,
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
        registration: V<ScriptResourceRegistration>,
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
        registration: V<ScriptResourceRegistration>,
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
        registration: V<ScriptResourceRegistration>,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("remove_resource");
        let world = ctxt.world()?;
        world.remove_resource(registration.into_inner())
    }

    /// Adds an observer which runs the callback `callback` of the calling script whenever an event of the given type is triggered.
    ///
    /// The callback is a normal script callback: either a function registered with `register_callback`, or a top level function with that name.
    /// It receives a copy of the event (or the payload, for script events), and is run by the same machinery as other callbacks:
    /// right after the operation which triggered the event, unless the script's context is busy running another callback
    /// (e.g. the one that triggered the event), in which case it runs as soon as that callback finishes.
    /// Observers never re-enter a running script. The order in which the observers of an event run is unspecified.
    /// Removed component data is no longer available inside the callback. Errors of the callback are reported like those of other callbacks.
    ///
    /// The event type must have `#[reflect(Event)]`. The lifecycle events (`AddEvent`, `InsertEvent`, `DiscardEvent`,
    /// `RemoveEvent`, `DespawnEvent`, imported with `import_namespace("bevy_ecs")`) are supported out of the box, and run for the given components.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `event`: The type of the event to observe, or an event registered with `register_new_event`.
    /// * `callback`: The label of the callback to run, receives the event.
    /// * `components`: The components the observer is interested in, for component lifecycle events. Ignored for script events.
    /// * `entities`: If given, the observer only runs for these entities. Ignored for script events.
    /// Returns:
    /// * `observer`: The observer entity, despawn it to remove the observer.
    fn add_observer(
        ctxt: FunctionCallContext,
        event: Union<V<ScriptTypeRegistration>, V<ScriptEventRegistration>>,
        callback: String,
        components: Option<Vec<V<ScriptComponentRegistration>>>,
        entities: Option<Vec<V<Entity>>>,
    ) -> Result<V<Entity>, InteropError> {
        profiling::function_scope!("add_observer");
        let world = ctxt.world()?;
        let attachment = world.current_attachment().0.ok_or_else(|| {
            InteropError::string(
                "Cannot add an observer outside of a script callback, as it needs a script to run its callback on".to_owned(),
            )
        })?;
        let event = match event.into_left() {
            Ok(event) => event,
            Err(script_event) => {
                return world
                    .add_script_event_observer(script_event.into_inner(), callback, attachment)
                    .map(V);
            }
        };
        world
            .add_script_observer(
                event.into_inner(),
                components
                    .unwrap_or_default()
                    .into_iter()
                    .map(V::into_inner)
                    .collect(),
                entities
                    .unwrap_or_default()
                    .into_iter()
                    .map(V::into_inner)
                    .collect(),
                callback,
                attachment,
            )
            .map(V)
    }

    /// Registers a new script event.
    ///
    /// Script events carry an arbitrary payload, and can be observed with `add_observer` and triggered with `trigger`.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `namespace`: The script namespace to register the event in, it must start with `script.`, i.e. `script.my_game`.
    /// * `name`: The name of the event, unique within the namespace.
    /// * `allow_existing`: If true, returns the already registered event with this name instead of failing. Defaults to false.
    /// Returns:
    /// * `registration`: The handle of the new event.
    fn register_new_event(
        ctxt: FunctionCallContext,
        namespace: String,
        name: String,
        allow_existing: Option<bool>,
    ) -> Result<V<ScriptEventRegistration>, InteropError> {
        profiling::function_scope!("register_new_event");
        let world = ctxt.world()?;
        world
            .register_script_event(&namespace, &name, allow_existing.unwrap_or(false))
            .map(V)
    }

    /// Triggers a script event, observers added with `add_observer` run right after.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `event`: The event registered with `register_new_event`.
    /// * `payload`: The data to pass to the observers.
    /// Returns:
    /// * `result`: Nothing if the event was triggered successfully.
    fn trigger(
        ctxt: FunctionCallContext,
        event: V<ScriptEventRegistration>,
        payload: Option<ScriptValue>,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("trigger");
        let world = ctxt.world()?;
        world.trigger_script_event(event.into_inner(), payload.unwrap_or(ScriptValue::Unit))
    }

    /// Triggers an observer-style event. The event type must be registered and have `#[reflect(Event)]`.
    ///
    /// Observers run immediately, before this function returns.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `registration`: The type of the event.
    /// * `event`: The event value, e.g. built with `construct`.
    /// Returns:
    /// * `result`: Nothing if the event was triggered successfully.
    fn trigger_event(
        ctxt: FunctionCallContext,
        registration: V<ScriptTypeRegistration>,
        event: ReflectReference,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("trigger_event");
        let world = ctxt.world()?;
        world.trigger_event(registration.into_inner(), &event)
    }

    /// Writes a buffered message. The message type must be registered and have `#[reflect(Message)]`.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `registration`: The type of the message.
    /// * `message`: The message value, e.g. built with `construct`.
    /// Returns:
    /// * `result`: Nothing if the message was written successfully.
    fn write_message(
        ctxt: FunctionCallContext,
        registration: V<ScriptTypeRegistration>,
        message: ReflectReference,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("write_message");
        let world = ctxt.world()?;
        world.write_message(registration.into_inner(), &message)
    }

    /// Adds the given resource to the world.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `registration`: The resource to add.
    /// Returns:
    /// * `result`: Nothing if the resource was added successfully.
    fn add_default_component(
        ctxt: FunctionCallContext,
        entity: V<Entity>,
        registration: V<ScriptComponentRegistration>,
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
    fn spawn(ctxt: FunctionCallContext) -> Result<V<Entity>, InteropError> {
        profiling::function_scope!("spawn");
        let world = ctxt.world()?;
        Ok(V(world.spawn()?))
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
        entity: V<Entity>,
        registration: V<ScriptComponentRegistration>,
        value: ReflectReference,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("insert_component");
        let world = ctxt.world()?;
        world.insert_component(*entity, registration.into_inner(), value)
    }

    /// Inserts the given bundle value into the provided entity, all of its components at once.
    ///
    /// The bundle type must be registered in rust with `#[reflect(Bundle)]`.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The entity to insert the bundle into.
    /// * `registration`: The bundle registration of the bundle to insert.
    /// * `value`: The value of the bundle to insert. Can be constructed like any other value, i.e. using `construct` or the type's `new` function.
    /// Returns:
    /// * `result`: Nothing if the bundle was inserted successfully.
    fn insert_bundle(
        ctxt: FunctionCallContext,
        entity: V<Entity>,
        registration: V<ScriptBundleRegistration>,
        value: ReflectReference,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("insert_bundle");
        let world = ctxt.world()?;
        world.insert_bundle(*entity, registration.into_inner(), value)
    }

    /// Spawns a new entity with the given bundle value and returns it.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `registration`: The bundle registration of the bundle to insert.
    /// * `value`: The value of the bundle to insert.
    /// Returns:
    /// * `entity`: The newly spawned entity
    fn spawn_bundle(
        ctxt: FunctionCallContext,
        registration: V<ScriptBundleRegistration>,
        value: ReflectReference,
    ) -> Result<V<Entity>, InteropError> {
        profiling::function_scope!("spawn_bundle");
        let world = ctxt.world()?;
        Ok(V(world.spawn_bundle(registration.into_inner(), value)?))
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
        entity: V<Entity>,
        index: usize,
        children: Vec<V<Entity>>,
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
        entity: V<Entity>,
        children: Vec<V<Entity>>,
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
        entity: V<Entity>,
    ) -> Result<Vec<V<Entity>>, InteropError> {
        profiling::function_scope!("get_children");
        let world = ctxt.world()?;
        let children = world.get_children(*entity)?;
        Ok(children.into_iter().map(V).collect::<Vec<_>>())
    }

    /// Retrieves the parent of the given entity.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The entity to retrieve the parent of.
    /// Returns:
    /// * `parent`: The parent of the entity
    fn get_parent(
        ctxt: FunctionCallContext,
        entity: V<Entity>,
    ) -> Result<Option<V<Entity>>, InteropError> {
        profiling::function_scope!("get_parent");
        let world = ctxt.world()?;
        let parent = world.get_parent(*entity)?;
        Ok(parent.map(V))
    }

    /// Despawns the given entity.
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `entity`: The entity to despawn.
    fn despawn(ctxt: FunctionCallContext, entity: V<Entity>) -> Result<(), InteropError> {
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
        entity: V<Entity>,
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
    fn despawn_recursive(ctxt: FunctionCallContext, entity: V<Entity>) -> Result<(), InteropError> {
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
    fn has_entity(ctxt: FunctionCallContext, e: V<Entity>) -> Result<bool, InteropError> {
        profiling::function_scope!("has_entity");
        let world = ctxt.world()?;
        world.has_entity(*e)
    }

    /// Creates a new `ScriptQueryBuilder` which can be used to query the ECS.
    ///
    /// Returns:
    /// * `query`: The new query builder.
    fn query() -> Result<V<ScriptQueryBuilder>, InteropError> {
        profiling::function_scope!("query");
        let query_builder = ScriptQueryBuilder::default();
        Ok(V(query_builder))
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
        #[allow(unused_variables)] schedule: V<ReflectSchedule>,
        #[allow(unused_variables)] builder: V<ScriptSystemBuilder>,
    ) -> Result<V<ReflectSystem>, InteropError> {
        profiling::function_scope!("add_system");
        let _world = ctxt.world()?;
        let _system = match ctxt.language() {
            #[cfg(feature = "lua_bindings")]
            bevy_mod_scripting_asset::Language::Lua => _world
                .add_system::<bevy_mod_scripting_lua::LuaScriptingPlugin>(
                &schedule,
                builder.into_inner(),
            )?,
            #[cfg(feature = "rhai_bindings")]
            bevy_mod_scripting_asset::Language::Rhai => {
                _world.add_system::<bevy_mod_scripting_rhai::RhaiScriptingPlugin>(
                    &schedule,
                    builder.into_inner(),
                )?
            }
            _ => {
                return Err(InteropError::unsupported_operation(
                    None,
                    None,
                    format!(
                        "creating a system in {} scripting language",
                        ctxt.language()
                    ),
                ));
            }
        };
        #[allow(unreachable_code)]
        Ok(V(_system))
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
    /// * `namespace`: The script namespace to register the component in, it must start with `script.`, i.e. `script.my_game`. Scripts cannot add to other namespaces.
    /// * `name`: The name of the component type, unique within the namespace. The component can later be found as `import_namespace(namespace).name`.
    /// * `allow_existing`: If true, returns the registration of the already registered component with this name instead of failing. Defaults to false.
    /// Returns:
    /// * `registration`: The registration of the new component type if successful.
    fn register_new_component(
        ctxt: FunctionCallContext,
        namespace: String,
        name: String,
        allow_existing: Option<bool>,
    ) -> Result<V<ScriptComponentRegistration>, InteropError> {
        profiling::function_scope!("register_new_component");
        let world = ctxt.world()?;
        world
            .register_script_component(&namespace, &name, allow_existing.unwrap_or(false))
            .map(V)
    }

    /// Retrieves an asset by its handle and asset type registration.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `handle_reference`: The handle to the asset (as a reflect reference).
    /// * `registration`: The type registration of the asset type.
    /// Returns:
    /// * `asset`: The asset reference, if the asset is loaded.
    fn get_asset(
        ctxt: FunctionCallContext,
        handle_reference: ReflectReference,
        registration: V<ScriptTypeRegistration>,
    ) -> Result<Option<ReflectReference>, InteropError> {
        profiling::function_scope!("get_asset");
        let untyped_handle = handle_reference.try_untyped_asset_handle(ctxt.world()?)?;
        Ok(Some(ReflectReference::new_asset_ref(
            untyped_handle,
            registration.type_id(),
            ctxt.world()?,
        )?))
    }

    /// Checks if can get asset handle
    fn has_asset(
        ctxt: FunctionCallContext,
        handle_reference: ReflectReference,
    ) -> Result<bool, InteropError> {
        profiling::function_scope!("has_asset");
        Ok(handle_reference
            .try_untyped_asset_handle(ctxt.world()?)
            .is_ok())
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "reflect_reference_functions",
    core,
    namespace = "bms"
)]
impl ReflectReference {
    /// If this is an enum, returns a reference to the variant it is currently in.
    ///
    /// This can be compared to `MyEnum.Variant` to check which variant a value is in.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to get the variant of.
    /// Returns:
    /// * `variant`: The variant, if the reference is an enum.
    fn variant(
        ctxt: FunctionCallContext,
        reference: ReflectReference,
    ) -> Result<Option<V<EnumVariantRef>>, InteropError> {
        profiling::function_scope!("variant");
        let world = ctxt.world()?;
        let Some(type_id) = reference.tail_type_id(world.clone())? else {
            return Ok(None);
        };
        reference
            .with_reflect(world, |value| EnumVariantRef::from_value(type_id, value))
            .map(|variant| variant.map(V))
    }

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

    /// Displays this reference and its contents if possible.
    ///
    /// This is useful for debugging and logging.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to display.
    /// Returns:
    /// * `display`: The display string.
    fn display(
        _ctxt: FunctionCallContext,
        reference: ReflectReference,
    ) -> Result<String, InteropError> {
        profiling::function_scope!("display");
        Ok(format!("{}", WithTypeInfo::new(&reference)))
    }

    /// Displays a debug representation of this reference.
    ///
    /// This is useful for debugging and logging.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `reference`: The reference to display.
    /// Returns:
    /// * `display`: The display string.
    fn debug(
        _ctxt: FunctionCallContext,
        reference: ReflectReference,
    ) -> Result<String, InteropError> {
        profiling::function_scope!("debug");
        Ok(format!("{reference:#?}"))
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
            let world = ThreadWorldContainer.try_get_context()?.world;
            if len == 0 {
                return Ok(ScriptValue::Unit);
            }

            let (next_ref, _) = infinite_iter.next_ref();

            let converted = ReflectReference::into_script_ref(next_ref, world);
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
    ) -> Result<Vec<V<FunctionInfo>>, InteropError> {
        profiling::function_scope!("functions");
        let world = ctxt.world()?;
        let type_id = reference.tail_type_id(world.clone())?.or_fake_id();
        let functions = world
            .get_functions_on_type(type_id)
            .into_iter()
            .map(|(_, v)| V::new(v.info.deref().clone()))
            .collect::<Vec<_>>();
        // convert to info
        Ok(functions)
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "enum_variant_ref_functions",
    core,
    namespace = "bms"
)]
impl EnumVariantRef {
    /// Retrieves the name of the variant.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `variant`: The variant.
    /// Returns:
    /// * `variant_name`: The name of the variant.
    fn variant_name(
        ctxt: FunctionCallContext,
        variant: V<EnumVariantRef>,
    ) -> Result<String, InteropError> {
        profiling::function_scope!("variant_name");
        variant.variant_name(&ctxt.world()?).map(str::to_owned)
    }

    /// Checks whether two variants are the same variant of the same enum.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `variant`: The variant.
    /// * `other`: The value to compare with.
    /// Returns:
    /// * `equal`: Whether the other value is the same variant.
    fn eq(
        ctxt: FunctionCallContext,
        variant: V<EnumVariantRef>,
        other: ScriptValue,
    ) -> Result<bool, InteropError> {
        profiling::function_scope!("eq");
        let world = ctxt.world()?;
        Ok(V::<EnumVariantRef>::from_script(other, world)
            .is_ok_and(|other| other.into_inner() == variant.into_inner()))
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "script_type_registration_functions",
    core,
    namespace = "bms"
)]
impl ScriptTypeRegistration {
    /// Retrieves the name of the type.
    ///
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `type_name`: The name of the type.
    fn type_name(registration: R<ScriptTypeRegistration>) -> String {
        profiling::function_scope!("type_name");
        registration.type_name().to_string()
    }

    /// Retrieves the short name of the type.
    /// The short name is a more human-readable version of the type name.
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `short_name`: The short name of the
    fn short_name(registration: R<ScriptTypeRegistration>) -> String {
        profiling::function_scope!("short_name");
        registration.short_name().to_string()
    }

    /// Retrieves the script namespace of the type, by default the name of the crate defining it.
    /// Types in the root namespace have the empty string as namespace.
    ///
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `namespace`: The namespace of the type.
    fn namespace(registration: R<ScriptTypeRegistration>) -> String {
        profiling::function_scope!("namespace");
        registration.namespace().to_owned()
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "script_component_registration_functions",
    core,
    namespace = "bms"
)]
impl ScriptComponentRegistration {
    /// Retrieves the name of the type.
    ///
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `type_name`: The name of the type.
    fn type_name(registration: R<ScriptComponentRegistration>) -> &'static str {
        profiling::function_scope!("type_name");
        registration.type_registration().type_name()
    }

    /// Retrieves the short name of the type.
    /// The short name is a more human-readable version of the type name.
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `short_name`: The short name of the
    fn short_name(registration: R<ScriptComponentRegistration>) -> &'static str {
        profiling::function_scope!("short_name");
        registration.type_registration().short_name()
    }

    /// Retrieves the script namespace of the type, by default the name of the crate defining it.
    /// Types in the root namespace have the empty string as namespace.
    ///
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `namespace`: The namespace of the type.
    fn namespace(registration: R<ScriptComponentRegistration>) -> String {
        profiling::function_scope!("namespace");
        registration.namespace().to_owned()
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "script_resource_registration_functions",
    core,
    namespace = "bms"
)]
impl ScriptResourceRegistration {
    /// Retrieves the name of the type.
    ///
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `type_name`: The name of the type.
    fn type_name(registration: R<ScriptResourceRegistration>) -> &'static str {
        profiling::function_scope!("type_name");
        registration.type_registration().type_name()
    }

    /// Retrieves the short name of the type.
    /// The short name is a more human-readable version of the type name.
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `short_name`: The short name of the
    fn short_name(registration: R<ScriptResourceRegistration>) -> &'static str {
        profiling::function_scope!("short_name");
        registration.type_registration().short_name()
    }

    /// Retrieves the script namespace of the type, by default the name of the crate defining it.
    /// Types in the root namespace have the empty string as namespace.
    ///
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `namespace`: The namespace of the type.
    fn namespace(registration: R<ScriptResourceRegistration>) -> String {
        profiling::function_scope!("namespace");
        registration.type_registration().namespace().to_owned()
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "script_bundle_registration_functions",
    core,
    namespace = "bms"
)]
impl ScriptBundleRegistration {
    /// Retrieves the name of the type.
    ///
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `type_name`: The name of the type.
    fn type_name(registration: R<ScriptBundleRegistration>) -> &'static str {
        profiling::function_scope!("type_name");
        registration.type_registration().type_name()
    }

    /// Retrieves the short name of the type.
    /// The short name is a more human-readable version of the type name.
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `short_name`: The short name of the
    fn short_name(registration: R<ScriptBundleRegistration>) -> &'static str {
        profiling::function_scope!("short_name");
        registration.type_registration().short_name()
    }

    /// Retrieves the script namespace of the type, by default the name of the crate defining it.
    /// Types in the root namespace have the empty string as namespace.
    ///
    /// Arguments:
    /// * `registration`: The type registration.
    /// Returns:
    /// * `namespace`: The namespace of the type.
    fn namespace(registration: R<ScriptBundleRegistration>) -> String {
        profiling::function_scope!("namespace");
        registration.namespace().to_owned()
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "script_query_builder_functions",
    core,
    namespace = "bms"
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
        query: V<ScriptQueryBuilder>,
        components: V<ScriptComponentRegistration>,
    ) -> V<ScriptQueryBuilder> {
        profiling::function_scope!("component");
        let mut builder = query.into_inner();
        builder.component(components.into_inner());
        V(builder)
    }

    /// Adds a component to filter the query by. This component will NOT be retrieved.
    ///
    /// Arguments:
    /// * `query`: The query to add the component to
    /// * `component`: The component to filter by
    /// Returns:
    /// * `query`: The query with the component added
    fn with(
        query: V<ScriptQueryBuilder>,
        with: V<ScriptComponentRegistration>,
    ) -> V<ScriptQueryBuilder> {
        profiling::function_scope!("with");
        let mut builder = query.into_inner();
        builder.with_component(with.into_inner());
        V(builder)
    }

    /// Adds a component to filter the query by. This component will NOT be retrieved.
    ///
    /// Arguments:
    /// * `query`: The query to add the component to
    /// * `component`: The component to filter by
    /// Returns:
    /// * `query`: The query with the component added
    fn without(
        query: V<ScriptQueryBuilder>,
        without: V<ScriptComponentRegistration>,
    ) -> V<ScriptQueryBuilder> {
        profiling::function_scope!("without");
        let mut builder = query.into_inner();
        builder.without_component(without.into_inner());
        V(builder)
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
        query: V<ScriptQueryBuilder>,
    ) -> Result<Vec<V<ScriptQueryResult>>, InteropError> {
        profiling::function_scope!("build");
        let world = ctxt.world()?;
        let builder = query.into_inner();
        let result = world.query(builder)?;
        let result = result.into_iter().map(V).collect::<Vec<_>>();
        Ok(result)
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "script_query_result_functions",
    core,
    namespace = "bms"
)]
impl ScriptQueryResult {
    /// Retrieves the entity from the query result.
    ///
    /// Arguments:
    /// * `query`: The query result to retrieve the entity from.
    /// Returns:
    /// * `entity`: The entity from the query result.
    fn entity(query: R<ScriptQueryResult>) -> V<Entity> {
        profiling::function_scope!("entity");
        V::new(query.entity)
    }

    /// Retrieves the components from the query result.
    ///
    /// These are ordered by the order they were added to the query.
    ///
    /// Arguments:
    /// * `query`: The query result to retrieve the components from.
    /// Returns:
    /// * `components`: The components from the query result.
    fn components(query: R<ScriptQueryResult>) -> Vec<ReflectReference> {
        profiling::function_scope!("components");
        query.components.to_vec()
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
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
        schedule: R<ReflectSchedule>,
    ) -> Result<Vec<V<ReflectSystem>>, InteropError> {
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
        schedule: R<ReflectSchedule>,
        name: String,
    ) -> Result<Option<V<ReflectSystem>>, InteropError> {
        profiling::function_scope!("system_by_name");
        let world = ctxt.world()?;
        let system = world.systems(&schedule)?;
        Ok(system.into_iter().find_map(|s| {
            (s.identifier() == name || s.path() == name || s.path().contains(&name))
                .then_some(s.into())
        }))
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
        schedule: R<ReflectSchedule>,
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
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "reflect_system_functions",
    core
)]
impl ReflectSystem {
    /// Retrieves the identifier of the system
    /// Arguments:
    /// * `system`: The system to retrieve the identifier from.
    /// Returns:
    /// * `identifier`: The identifier of the system, e.g. `my_system`
    fn identifier(system: R<ReflectSystem>) -> String {
        profiling::function_scope!("identifier");
        system.identifier().to_string()
    }

    /// Retrieves the full path of the system
    /// Arguments:
    /// * `system`: The system to retrieve the path from.
    /// Returns:
    /// * `path`: The full path of the system, e.g. `my_crate::systems::my_system<T>`
    fn path(system: R<ReflectSystem>) -> String {
        profiling::function_scope!("path");
        system.path().to_string()
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "script_system_builder_functions",
    core,
    namespace = "bms"
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
        builder: V<ScriptSystemBuilder>,
        query: V<ScriptQueryBuilder>,
    ) -> Result<V<ScriptSystemBuilder>, InteropError> {
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
        builder: V<ScriptSystemBuilder>,
        resource: V<ScriptResourceRegistration>,
    ) -> V<ScriptSystemBuilder> {
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
    fn exclusive(builder: V<ScriptSystemBuilder>) -> V<ScriptSystemBuilder> {
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
    fn after(builder: V<ScriptSystemBuilder>, system: V<ReflectSystem>) -> V<ScriptSystemBuilder> {
        profiling::function_scope!("after");
        let mut builder = builder.into_inner();
        builder.after_system(system.into_inner());
        V(builder)
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
    fn before(builder: V<ScriptSystemBuilder>, system: V<ReflectSystem>) -> V<ScriptSystemBuilder> {
        profiling::function_scope!("before");
        let mut builder = builder.into_inner();
        builder.before_system(system.into_inner());
        V(builder)
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "script_attachment_functions",
    core,
    namespace = "bms"
)]
impl ScriptAttachment {
    /// Creates a new script attachment descriptor from a script asset.
    ///  
    /// Arguments:
    /// * `script`: The script asset to create the attachment from.
    /// Returns:    
    /// * `attachment`: The new script attachment.
    pub fn new_static_script(
        script: V<Handle<ScriptAsset>>,
    ) -> Result<V<ScriptAttachment>, InteropError> {
        profiling::function_scope!("new_static_script");
        Ok(V(ScriptAttachment::StaticScript(script.into_inner())))
    }

    /// Creates a new script attachment descriptor for an entity attached script.
    ///
    /// Arguments:
    /// * `entity`: The entity to attach the script to.
    /// * `script`: The script asset to attach to the entity.
    /// Returns:
    /// * `attachment`: The new script attachment for the entity.
    pub fn new_entity_script(
        entity: V<Entity>,
        script: V<Handle<ScriptAsset>>,
    ) -> Result<V<ScriptAttachment>, InteropError> {
        profiling::function_scope!("new_entity_script");
        Ok(V(ScriptAttachment::EntityScript(
            *entity,
            script.into_inner(),
        )))
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "recipients_functions",
    core,
    namespace = "bms"
)]
impl Recipients {
    /// Targets every loaded script, once per script even if scripts share a context.
    ///
    /// Returns:
    /// * `recipients`: The recipients.
    fn all_scripts() -> V<Recipients> {
        V(Recipients::AllScripts)
    }

    /// Targets every loaded context once, i.e. scripts sharing a context only run the callback once.
    ///
    /// Returns:
    /// * `recipients`: The recipients.
    fn all_contexts() -> V<Recipients> {
        V(Recipients::AllContexts)
    }

    /// Targets a single static script.
    ///
    /// Arguments:
    /// * `script`: The script asset.
    /// Returns:
    /// * `recipients`: The recipients.
    fn static_script(script: V<Handle<ScriptAsset>>) -> V<Recipients> {
        V(Recipients::StaticScript(script.into_inner()))
    }

    /// Targets a single script attached to an entity.
    ///
    /// Arguments:
    /// * `entity`: The entity the script is attached to.
    /// * `script`: The script asset.
    /// Returns:
    /// * `recipients`: The recipients.
    fn entity_script(entity: V<Entity>, script: V<Handle<ScriptAsset>>) -> V<Recipients> {
        V(Recipients::ScriptEntity(script.into_inner(), *entity))
    }
}

#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "script_handle_functions",
    core
)]
impl Handle<ScriptAsset> {
    /// Retrieves the path of the script asset if present.
    /// Assets can be unloaded, and as such if the given handle is no longer active, this will return `None`.
    ///
    /// Arguments:
    /// * `handle`: The handle to the script asset.
    /// Returns:
    /// * `path`: The asset path of the script asset.
    fn asset_path(ctxt: FunctionCallContext, handle: R<Handle<ScriptAsset>>) -> Option<String> {
        profiling::function_scope!("path");
        handle.path().map(|p| p.to_string()).or_else(|| {
            ctxt.world().ok().and_then(|w| {
                w.with_resource(|asset_server: &AssetServer| {
                    asset_server.get_path(&*handle).map(|p| p.to_string())
                })
                .ok()
                .flatten()
            })
        })
    }
}

/// globals which are being registered at lower level within each language plugin.
#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "global_namespace_dummy_functions",
    unregistered,
    use_dummy_registry
)]
impl GlobalNamespace {
    /// Registers a "frozen" callback handler,
    ///
    /// For example, this code:
    ///
    /// ```lua
    /// register_callback("on_script_unloaded", my_unload_handler)
    ///
    /// function my_unload_handler()
    ///     print("handling unload!")
    /// end
    /// ```
    ///
    /// would call the `my_unload_handler` function, whenever the `on_script_unloaded` callback is triggered, which is when your script is about to be unloaded.
    ///
    /// Registered callbacks take precedence over free-standing function callbacks, i.e. the below top level function:
    /// ```lua
    /// function on_script_unloaded()
    ///     print("freestanding unload handler!")
    /// end
    /// ```
    ///
    /// would be a valid handler, but if a registered callback existed, it would be called instead.
    ///
    /// Arguments:
    /// * `callback`: the callback label to register this function against.
    /// * `function`: the callback function which will be stored as a handler for this callback label.
    fn register_callback(callback: String, function: DynamicScriptFunction) {
        // to avoid clippy unused errors.
        println!("dummy called!: {callback:?}, {function:?}");
    }
}

/// Globals registered by us
#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "global_namespace_functions",
    unregistered
)]
impl GlobalNamespace {
    /// Imports a namespace, returning a table of the types and exported functions in it by name.
    ///
    /// Native namespaces are the names of the crates defining the types (i.e. `bevy_transform`), or `bms` for BMS' own types.
    /// The empty string is the root namespace, which holds types without a crate such as primitives.
    /// Namespaces defined by scripts start with `script.`, and contain the components, bundles and functions
    /// scripts registered or exported into them.
    ///
    /// ```lua
    /// local bevy_transform = import_namespace("bevy_transform")
    /// local transform = bevy_transform.Transform
    ///
    /// local game = import_namespace("script.game")
    /// game.spawn_enemy(5)
    /// ```
    ///
    /// Fails with a list of similar namespaces if the namespace does not exist.
    ///
    /// Arguments:
    /// * `path`: the name of the namespace to import.
    /// Returns:
    /// * `namespace`: a table of the types and functions in the namespace keyed by name.
    fn import_namespace(
        ctxt: FunctionCallContext,
        path: String,
    ) -> Result<ScriptValue, InteropError> {
        profiling::function_scope!("import_namespace");
        let world = ctxt.world()?;
        let catalogue = world.type_catalogue().clone().get(world.clone())?;
        let types = catalogue.namespace_view(&path);
        let exports = world
            .component_registry()
            .read()
            .iter_exports_in(&path)
            .map(|e| e.name.clone())
            .collect::<Vec<_>>();
        if types.is_none() && exports.is_empty() {
            return Err(namespace_not_found(&catalogue, &world, &path));
        }

        let mut table = HashMap::default();
        for (name, registration) in types.into_iter().flatten() {
            table.insert(name, registration.into_script(world.clone())?);
        }
        for name in exports {
            let function = exported_function(script_path(&path, &name));
            table.insert(name, ScriptValue::Function(function));
        }
        Ok(ScriptValue::Map(table))
    }

    /// Exports a callback of the calling script as a function in a script namespace, so that any script (in any language) can call it.
    ///
    /// The callback is a normal script callback: either a function registered with `register_callback`, or a top level function with that name.
    /// Calls run in the exporting script, through the same machinery as other callbacks, and return what the callback returns.
    /// Only functions can be exported, share plain data by returning it from a function.
    ///
    /// ```lua
    /// register_callback("spawn_enemy_impl", function(health)
    ///     return world.spawn()
    /// end)
    /// export_namespace("script.game", "spawn_enemy", "spawn_enemy_impl")
    ///
    /// -- in another script, once the first one has run
    /// local entity = import_namespace("script.game").spawn_enemy(5)
    /// ```
    ///
    /// Exporting the same name again from the same script replaces the export, as happens when a script runs again.
    /// A script cannot call its own exports (or those of scripts sharing its context) from inside its own callbacks, as the context is busy,
    /// that fails with an error.
    ///
    /// Arguments:
    /// * `namespace`: the script namespace to export into, it must start with `script.`.
    /// * `name`: the name of the function within the namespace.
    /// * `callback`: the label of the callback of this script to export.
    /// Returns:
    /// * `result`: Nothing if the function was exported.
    fn export_namespace(
        ctxt: FunctionCallContext,
        namespace: String,
        name: String,
        callback: String,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("export_namespace");
        let world = ctxt.world()?;
        let owner = world.current_attachment().0.ok_or_else(|| {
            InteropError::string(
                "Cannot export a function outside of a script callback, as it needs a script to run it in".to_owned(),
            )
        })?;
        world.register_script_export(&namespace, &name, callback, owner)
    }

    /// Attempts to construct the given type, given an arbitrary map of values.
    ///
    /// Unlike `Type.new`, this works for every type, including ones with a `new` function of their own.
    /// Missing fields are filled in from the type's default, then from the defaults of the fields' types, as for `Type.new`.
    ///
    /// Arguments:
    /// * `registration`: The type to construct.
    /// * `payload`: The values to use to construct the type, keyed by field name or index, or a list of the fields in order.
    /// Returns:
    /// * `reference`: The constructed type.
    fn construct(
        ctxt: FunctionCallContext,
        registration: AnyTypeRegistrationV,
        payload: BoundFields,
    ) -> Result<ReflectReference, InteropError> {
        let registration = registration.into_type_registration();
        construct_reference(
            &ctxt.world()?,
            registration.type_registration().type_info(),
            None,
            payload,
            ctxt.convert_to_0_indexed(),
        )
    }

    /// Creates a new script system builder, which can be used to add new systems to the world.
    ///
    /// Arguments:
    /// * `callback`: The function name in the script this system should call when run.
    /// * `attachment`: The script attachment to use for the system. This is the attachment that will be used for the system's callback.
    /// Returns:
    /// * `builder`: The system builder
    fn system_builder(
        callback: String,
        attachment: V<ScriptAttachment>,
    ) -> Result<V<ScriptSystemBuilder>, InteropError> {
        Ok(ScriptSystemBuilder::new(callback.into(), attachment.into_inner()).into())
    }

    /// Runs a callback on other scripts immediately, and returns what it returned.
    ///
    /// For example `run_script_callback("take_damage", ScriptAttachment.new_entity_script(enemy, script), 10)`
    /// calls `take_damage(10)` in that script before this function returns.
    ///
    /// The target is either a single script attachment, returning that callback's value,
    /// or [`Recipients`], returning a list with the value of each script which ran the callback.
    /// Errors of the callbacks are raised in the calling script, for recipients only after all of them ran.
    ///
    /// Callbacks cannot run immediately in a script whose context is busy, i.e. the calling script itself,
    /// scripts sharing its context (depending on the context policy), or scripts further up the current chain of callbacks.
    /// Use `queue_script_callback` for those. It is also unavailable in script systems running in parallel with other systems.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `label`: The name of the callback to run.
    /// * `target`: The script attachment or recipients to run the callback on.
    /// * `args`: The arguments to pass to the callback.
    /// Returns:
    /// * `result`: The value returned by the callback, or a list of the values returned by each recipient.
    fn run_script_callback(
        ctxt: FunctionCallContext,
        label: String,
        target: Union<V<ScriptAttachment>, V<Recipients>>,
        args: VariadicTuple,
    ) -> Result<ScriptValue, InteropError> {
        profiling::function_scope!("run_script_callback");
        let world = ctxt.world()?;
        if !world.is_exclusive() {
            return Err(InteropError::string(format!(
                "Cannot run callback `{label}` immediately from a script system running in parallel with other systems, \
                as another system might be running the target script at the same time. \
                Use `queue_script_callback` instead, or make the system exclusive."
            )));
        }
        let runners = world.script_callback_runners().clone();
        let args = Vec::from(args.0);

        match target.into_left() {
            Ok(attachment) => {
                let attachment = attachment.into_inner();
                let recipients = Recipients::from(attachment.clone());
                runners
                    .run(world, &recipients, &label, args)?
                    .into_iter()
                    .next()
                    .map(|(_, result)| result)
                    .unwrap_or_else(|| {
                        Err(InteropError::string(format!(
                            "Cannot run callback `{label}` on script {attachment}: no such script is loaded"
                        )))
                    })
            }
            Err(recipients) => {
                let results = runners.run(world, &recipients.into_inner(), &label, args)?;
                let mut values = VecDeque::with_capacity(results.len());
                let mut first_error = None;
                for (_, result) in results {
                    match result {
                        Ok(value) => values.push_back(value),
                        Err(err) => {
                            first_error.get_or_insert(err);
                        }
                    }
                }
                match first_error {
                    Some(err) => Err(err),
                    None => Ok(ScriptValue::List(values)),
                }
            }
        }
    }

    /// Queues a callback to run on other scripts as soon as the current callback finishes, in the same frame.
    ///
    /// Unlike `run_script_callback`, this works for any script, including the calling script itself and scripts sharing its context.
    /// Queued callbacks run in the order they were queued, and their errors are reported as script errors instead of being raised here.
    ///
    /// Arguments:
    /// * `ctxt`: The function call context.
    /// * `label`: The name of the callback to run.
    /// * `target`: The script attachment or recipients to run the callback on.
    /// * `args`: The arguments to pass to the callback.
    /// Returns:
    /// * `result`: Nothing if the callback was queued.
    fn queue_script_callback(
        ctxt: FunctionCallContext,
        label: String,
        target: Union<V<ScriptAttachment>, V<Recipients>>,
        args: VariadicTuple,
    ) -> Result<(), InteropError> {
        profiling::function_scope!("queue_script_callback");
        if CallbackLabel::new(&label).is_none() {
            return Err(InteropError::string(format!(
                "`{label}` is not a valid callback name"
            )));
        }
        let recipients = match target.into_left() {
            Ok(attachment) => Recipients::from(attachment.into_inner()),
            Err(recipients) => recipients.into_inner(),
        };
        ctxt.world()?
            .script_callback_runners()
            .queue(QueuedScriptCallback {
                recipients,
                label,
                args: Vec::from(args.0),
            });
        Ok(())
    }

    /// Unpacks, a list of values, into many separate values.
    ///
    /// Arguments:
    /// * `values`: The list of values to uncurry
    /// Returns:
    /// * `tuple`: The tuple of all values provided
    fn unpack_args(values: VecDeque<ScriptValue>) -> VariadicTuple {
        VariadicTuple(values)
    }

    /// Packs, a variable amount of values, into a single list.
    ///
    /// Arguments:
    /// * `tuple`: The tuple of values to curry
    /// Returns:
    /// * `values`: The list of values destructured from the tuple
    fn pack_args(tuple: VariadicTuple) -> VecDeque<ScriptValue> {
        tuple.0
    }

    /// Logs a `Information` level message to the console
    /// Arguments:
    /// * `message`: the message to log
    fn log_info(message: String) {
        bevy_log::info!(message)
    }

    /// Logs a `Warning` level message to the console
    /// * `message`: the message to log
    fn log_warn(message: String) {
        bevy_log::warn!(message)
    }

    /// Logs a `Error` level message to the console
    /// * `message`: the message to log
    fn log_error(message: String) {
        bevy_log::error!(message)
    }

    /// Logs a `Debug` level message to the console
    /// * `message`: the message to log
    fn log_debug(message: String) {
        bevy_log::debug!(message)
    }

    /// Logs a `Trace` level message to the console
    /// * `message`: the message to log
    fn log_trace(message: String) {
        bevy_log::trace!(message)
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
        register_enum_variant_ref_functions(world);
        register_script_component_registration_functions(world);
        register_script_resource_registration_functions(world);
        register_script_bundle_registration_functions(world);

        register_script_query_builder_functions(world);
        register_script_query_result_functions(world);

        register_reflect_schedule_functions(world);
        register_reflect_system_functions(world);
        register_script_system_builder_functions(world);

        register_script_attachment_functions(world);
        register_recipients_functions(world);

        register_script_handle_functions(world);

        register_global_namespace_functions(world);
        register_global_namespace_dummy_functions(world);
    }
}

/// Builds the error for importing a namespace which does not exist.
fn namespace_not_found(
    catalogue: &ScriptTypeCatalogue,
    world: &WorldGuard,
    path: &str,
) -> InteropError {
    let mut message = format!("namespace \"{path}\" does not exist.");
    if path.starts_with("script.") {
        message.push_str(
            " Script namespaces only exist once a loaded script has registered or exported something into them, \
            make sure that script has loaded and run before importing it.",
        );
    } else {
        message.push_str(
            " Native namespaces are the names of the crates defining the types (i.e. `bevy_transform`), \
            and only contain types registered in the type registry: check that the crate's bindings plugin is added and the type is registered.",
        );
    }
    let export_namespaces = world
        .component_registry()
        .read()
        .export_namespaces()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let similar = catalogue.similar_namespaces_with(path, &export_namespaces);
    if !similar.is_empty() {
        message.push_str(&format!(" Did you mean: {}?", similar.join(", ")));
    }
    InteropError::string(message)
}

/// A function calling the function with the given full path exported by a script, in the script that exported it.
///
/// The export is looked up on every call, so it picks up replacements, and fails clearly if the export is gone.
fn exported_function(path: String) -> DynamicScriptFunction {
    (move |ctxt: FunctionCallContext, args: VecDeque<ScriptValue>| -> ScriptValue {
        match ctxt
            .world()
            .and_then(|world| world.call_script_export(&path, Vec::from(args)))
        {
            Ok(value) => value,
            Err(error) => ScriptValue::Error(error),
        }
    })
    .into()
}
