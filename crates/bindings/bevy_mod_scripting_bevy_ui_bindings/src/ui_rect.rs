//! [`UiRect`] values: a length for each of the left, right, top and bottom sides of a node.

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;
use bevy_ui::{UiRect, Val};

#[script_bindings(
    remote,
    name = "ui_rect_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl UiRect {
    /// A rectangle with the same length on all four sides.
    fn all(value: R<Val>) -> V<UiRect> {
        V(UiRect::all(*value))
    }

    /// A rectangle with lengths in logical pixels for its left, right, top and bottom sides.
    fn px(left: f32, right: f32, top: f32, bottom: f32) -> V<UiRect> {
        V(UiRect::px(left, right, top, bottom))
    }

    /// A rectangle with percentages of the parent for its left, right, top and bottom sides.
    fn percent(left: f32, right: f32, top: f32, bottom: f32) -> V<UiRect> {
        V(UiRect::percent(left, right, top, bottom))
    }

    /// A rectangle with one length for the left and right sides and another for the top and bottom.
    fn axes(horizontal: R<Val>, vertical: R<Val>) -> V<UiRect> {
        V(UiRect::axes(*horizontal, *vertical))
    }

    /// A rectangle with the length on its left and right sides only.
    fn horizontal(value: R<Val>) -> V<UiRect> {
        V(UiRect::horizontal(*value))
    }

    /// A rectangle with the length on its top and bottom sides only.
    fn vertical(value: R<Val>) -> V<UiRect> {
        V(UiRect::vertical(*value))
    }

    /// A rectangle with a length for each of its left, right, top and bottom sides.
    fn new(left: R<Val>, right: R<Val>, top: R<Val>, bottom: R<Val>) -> V<UiRect> {
        V(UiRect::new(*left, *right, *top, *bottom))
    }

    /// A rectangle with no space on any side.
    fn zero() -> V<UiRect> {
        V(UiRect::ZERO)
    }
}

/// Registers the [`UiRect`] functions.
pub(crate) fn register(world: &mut World) {
    register_ui_rect_functions(world);
}
