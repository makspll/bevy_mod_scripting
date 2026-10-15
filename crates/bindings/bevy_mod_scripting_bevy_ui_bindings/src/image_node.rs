//! [`ImageNode`] values: a UI node that draws an image.

// `script_bindings` expects these in scope
use bevy_asset::Handle;
use bevy_color::Color;
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_image::Image;
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;
use bevy_ui::widget::ImageNode;

#[script_bindings(
    remote,
    name = "image_node_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "ui",
    core
)]
impl ImageNode {
    /// A node that draws the given [`Image`](bevy_image::Image), untinted.
    fn new(image: R<Handle<Image>>) -> V<ImageNode> {
        V(ImageNode::new(image.clone()))
    }

    /// A copy of the node that tints its image with `color`.
    fn with_color(_self: R<ImageNode>, color: R<Color>) -> V<ImageNode> {
        V(_self.clone().with_color(*color))
    }
}

/// Registers the [`ImageNode`] functions.
pub(crate) fn register(world: &mut World) {
    register_image_node_functions(world);
}
