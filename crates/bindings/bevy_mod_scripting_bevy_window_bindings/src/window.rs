//! Windows: the [`Window`] component, and the `Window` functions which find the primary window.

// `script_bindings` expects these in scope
use bevy_ecs::{entity::Entity, reflect::AppTypeRegistry, world::World};
use bevy_math::Vec2;
use bevy_mod_scripting_bindings::{
    InteropError, ReflectReference, ScriptQueryBuilder, WorldExtensions, WorldGuard,
    function::{
        from::{M, R, V},
        script_function::FunctionCallContext,
    },
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_window::{PrimaryWindow, Window};

use crate::component_registration;

/// The entity of the primary window, or nil if there is none.
fn primary_window_entity(world: &WorldGuard) -> Result<Option<Entity>, InteropError> {
    let Some(primary) = component_registration::<PrimaryWindow>(world) else {
        return Ok(None);
    };
    let mut query = ScriptQueryBuilder::new();
    query.with_component(primary);
    if let Some(result) = world.query(query)?.into_iter().next() {
        return Ok(Some(result.entity));
    }
    Ok(None)
}

#[script_bindings(
    remote,
    name = "window_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Window {
    /// The width of the window in logical pixels.
    fn width(_self: R<Window>) -> f32 {
        _self.width()
    }

    /// The height of the window in logical pixels.
    fn height(_self: R<Window>) -> f32 {
        _self.height()
    }

    /// The size of the window in logical pixels.
    fn size(_self: R<Window>) -> V<Vec2> {
        V(_self.size())
    }

    /// The width of the window in physical pixels.
    fn physical_width(_self: R<Window>) -> u32 {
        _self.physical_width()
    }

    /// The height of the window in physical pixels.
    fn physical_height(_self: R<Window>) -> u32 {
        _self.physical_height()
    }

    /// The ratio of physical to logical pixels, for example `2` on a high density screen.
    fn scale_factor(_self: R<Window>) -> f32 {
        _self.scale_factor()
    }

    /// The position of the cursor in logical pixels from the top left, or nil if the cursor is outside the window.
    fn cursor_position(_self: R<Window>) -> Option<V<Vec2>> {
        _self.cursor_position().map(V)
    }

    /// Moves the cursor to the position in logical pixels, or hides it outside the window when given nil.
    fn set_cursor_position(mut _self: M<Window>, position: Option<R<Vec2>>) {
        _self.set_cursor_position(position.map(|position| *position));
    }

    /// The primary window, or nil if there is none.
    fn primary(ctxt: FunctionCallContext) -> Result<Option<ReflectReference>, InteropError> {
        let world = ctxt.world()?;
        match primary_window_entity(&world)? {
            Some(entity) => Ok(Some(ReflectReference::new_component_ref::<Window>(
                entity,
                world.clone(),
            )?)),
            None => Ok(None),
        }
    }

    /// The entity of the primary window, or nil if there is none.
    fn primary_entity(ctxt: FunctionCallContext) -> Result<Option<V<Entity>>, InteropError> {
        Ok(primary_window_entity(&ctxt.world()?)?.map(V))
    }
}

/// Registers the functions of windows.
pub(crate) fn register(world: &mut World) {
    register_window_functions(world);
}
