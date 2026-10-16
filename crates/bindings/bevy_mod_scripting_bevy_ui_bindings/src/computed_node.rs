//! [`ComputedNode`] values: the size of a node as calculated by the last layout.

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::Vec2;
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;
use bevy_ui::ComputedNode;

#[script_bindings(
    remote,
    name = "computed_node_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl ComputedNode {
    /// The width and height of the node in physical pixels, as calculated by the last layout.
    fn size(_self: R<ComputedNode>) -> V<Vec2> {
        V(_self.size())
    }

    /// The width and height of the node's content in physical pixels, as calculated by the last layout.
    fn content_size(_self: R<ComputedNode>) -> V<Vec2> {
        V(_self.content_size())
    }

    /// Whether the node has no width or no height, so that nothing is drawn for it.
    fn is_empty(_self: R<ComputedNode>) -> bool {
        _self.is_empty()
    }

    /// The factor that turns physical pixels into logical pixels for this node.
    fn inverse_scale_factor(_self: R<ComputedNode>) -> f32 {
        _self.inverse_scale_factor()
    }
}

/// Registers the [`ComputedNode`] functions.
pub(crate) fn register(world: &mut World) {
    register_computed_node_functions(world);
}
