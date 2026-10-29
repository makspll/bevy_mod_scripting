//! Script bindings for bevy app states.
//!
//! States are read and set through the `State<T>` and `NextState<T>` resources of a state type `T`.

use bevy_app::{App, Plugin};
// `script_bindings` expects these in scope
use bevy_ecs::world::World;
use bevy_mod_scripting_bindings::{
    InteropError, ReferencePart, ReflectReference, ScriptTypeRegistration, WorldExtensions,
    function::{
        from::V,
        generics::{A, TypeOf, ValueOf},
        script_function::FunctionCallContext,
    },
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_reflect::{
    enums::{DynamicEnum, DynamicVariant},
    tuple::DynamicTuple,
};

/// Registers the state script bindings.
#[derive(Default)]
pub struct BevyStateBindingsPlugin;

impl Plugin for BevyStateBindingsPlugin {
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_state"],
            "state",
        );
    }

    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        register_world_state_functions(world);
    }
}

#[script_bindings(
    remote,
    name = "world_state_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    unregistered
)]
impl World {
    /// The current value of the state of the given type, or nil if the app has no such state.
    ///
    /// The state type is passed as its registration, see `world.get_type_by_name`.
    fn get_state(
        ctxt: FunctionCallContext,
        state_type: TypeOf<V<ScriptTypeRegistration>, A>,
    ) -> Result<Option<ValueOf<A>>, InteropError> {
        let world = ctxt.world()?;
        let state_type = state_type.into_inner().into_inner();
        let state_path = format!(
            "bevy_state::state::resources::State<{}>",
            state_type.type_name()
        );
        let Some(state) = world.get_type_by_name(&state_path) else {
            return Ok(None);
        };
        let Ok(resource) = world.get_resource_type(state)? else {
            return Ok(None);
        };
        if !world.has_resource(resource.resource_id())? {
            return Ok(None);
        }
        // the state value is the only field of the `State<T>` resource
        let mut reference = ReflectReference::new_resource_ref_by_id(
            resource.resource_id(),
            resource.type_registration().type_id(),
        );
        reference.push_path(ReferencePart::IntegerAccess(0, false));
        Ok(Some(ValueOf::new(reference)))
    }

    /// Sets the state to change to in the next state transition, for the type of the given state value.
    ///
    /// Errors if the value is not of a state type that the app has.
    fn set_next_state(
        ctxt: FunctionCallContext,
        value: ReflectReference,
    ) -> Result<(), InteropError> {
        let world = ctxt.world()?;
        let type_id = value
            .tail_type_id(world.clone())?
            .ok_or_else(|| InteropError::str("cannot determine the type of the state value"))?;
        let type_name = {
            let registry = world.type_registry();
            let registry = registry.read();
            registry
                .get(type_id)
                .map(|registration| registration.type_info().type_path_table().path())
                .ok_or_else(|| InteropError::str("the type of the state value is not registered"))?
        };
        let next_state = world
            .get_type_by_name(&format!(
                "bevy_state::state::resources::NextState<{type_name}>"
            ))
            .ok_or_else(|| {
                InteropError::string(format!("`{type_name}` is not a state type of the app"))
            })?;
        let resource = world
            .get_resource_type(next_state)?
            .map_err(|_| InteropError::string(format!("`{type_name}` is not a state type")))?;
        if !world.has_resource(resource.resource_id())? {
            return Err(InteropError::string(format!(
                "the state `{type_name}` is not inserted in the app"
            )));
        }

        let reference = ReflectReference::new_resource_ref_by_id(
            resource.resource_id(),
            resource.type_registration().type_id(),
        );
        let state_value = value.to_owned_value(world.clone())?;
        let mut pending_fields = DynamicTuple::default();
        pending_fields.insert_boxed(state_value);
        let pending = DynamicEnum::new("Pending", DynamicVariant::Tuple(pending_fields));
        reference
            .with_reflect_mut(world, |next_state| next_state.try_apply(&pending))?
            .map_err(|e| InteropError::string(format!("could not set the next state: {e}")))
    }
}
