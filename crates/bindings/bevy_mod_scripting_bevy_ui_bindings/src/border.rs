//! [`BorderRadius`] and [`BorderColor`]: the rounding of each corner and the color of each side of a node's border.

// `script_bindings` expects these in scope
use bevy_color::Color;
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;
use bevy_ui::{BorderColor, BorderRadius, Val};

#[script_bindings(
    remote,
    name = "border_radius_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "ui",
    core
)]
impl BorderRadius {
    /// Every corner rounded by the same length.
    fn all(radius: R<Val>) -> V<BorderRadius> {
        V(BorderRadius::all(*radius))
    }

    /// Each corner rounded by a length in logical pixels, in the order top left, top right, bottom right, bottom left.
    fn px(top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) -> V<BorderRadius> {
        V(BorderRadius::px(
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        ))
    }

    /// Each corner rounded by a percentage, in the order top left, top right, bottom right, bottom left.
    fn percent(
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32,
    ) -> V<BorderRadius> {
        V(BorderRadius::percent(
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        ))
    }

    /// The most rounding possible, which makes the node a capsule, or a circle if it is square.
    fn max() -> V<BorderRadius> {
        V(BorderRadius::MAX)
    }

    /// No rounding, so every corner is a right angle.
    fn zero() -> V<BorderRadius> {
        V(BorderRadius::ZERO)
    }
}

#[script_bindings(
    remote,
    name = "border_color_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "ui",
    core
)]
impl BorderColor {
    /// A border with the same color on all four sides.
    fn all(color: R<Color>) -> V<BorderColor> {
        V(BorderColor::all(*color))
    }
}

/// Registers the [`BorderRadius`] and [`BorderColor`] functions.
pub(crate) fn register(world: &mut World) {
    register_border_radius_functions(world);
    register_border_color_functions(world);
}
