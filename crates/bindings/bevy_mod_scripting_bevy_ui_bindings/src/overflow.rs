//! [`Overflow`] settings: whether content outside a node is shown, clipped, hidden or scrolled.

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::V;
use bevy_mod_scripting_derive::script_bindings;
use bevy_ui::Overflow;

#[script_bindings(
    remote,
    name = "overflow_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "ui",
    core
)]
impl Overflow {
    /// Content that overflows the node is shown.
    fn visible() -> V<Overflow> {
        V(Overflow::visible())
    }

    /// Content that overflows the node is clipped on both axes.
    fn clip() -> V<Overflow> {
        V(Overflow::clip())
    }

    /// Content that overflows the node is clipped on the horizontal axis only.
    fn clip_x() -> V<Overflow> {
        V(Overflow::clip_x())
    }

    /// Content that overflows the node is clipped on the vertical axis only.
    fn clip_y() -> V<Overflow> {
        V(Overflow::clip_y())
    }

    /// Content that overflows the node is hidden on both axes, with the layout making room for it to be cut off.
    fn hidden() -> V<Overflow> {
        V(Overflow::hidden())
    }

    /// Content that overflows the node can be scrolled on both axes.
    fn scroll() -> V<Overflow> {
        V(Overflow::scroll())
    }
}

/// Registers the [`Overflow`] functions.
pub(crate) fn register(world: &mut World) {
    register_overflow_functions(world);
}
