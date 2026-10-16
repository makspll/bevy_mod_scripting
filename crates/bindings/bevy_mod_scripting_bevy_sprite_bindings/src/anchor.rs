//! [`Anchor`] positions, the point of a sprite that sits on its transform.

use bevy_math::Vec2;
use bevy_sprite::Anchor;
// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "anchor_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "sprite",
    core
)]
impl Anchor {
    /// The anchor in the middle of the sprite.
    fn center() -> V<Anchor> {
        V(Anchor::CENTER)
    }

    /// The anchor in the middle of the left edge.
    fn center_left() -> V<Anchor> {
        V(Anchor::CENTER_LEFT)
    }

    /// The anchor in the middle of the right edge.
    fn center_right() -> V<Anchor> {
        V(Anchor::CENTER_RIGHT)
    }

    /// The anchor in the bottom left corner.
    fn bottom_left() -> V<Anchor> {
        V(Anchor::BOTTOM_LEFT)
    }

    /// The anchor in the middle of the bottom edge.
    fn bottom_center() -> V<Anchor> {
        V(Anchor::BOTTOM_CENTER)
    }

    /// The anchor in the bottom right corner.
    fn bottom_right() -> V<Anchor> {
        V(Anchor::BOTTOM_RIGHT)
    }

    /// The anchor in the top left corner.
    fn top_left() -> V<Anchor> {
        V(Anchor::TOP_LEFT)
    }

    /// The anchor in the middle of the top edge.
    fn top_center() -> V<Anchor> {
        V(Anchor::TOP_CENTER)
    }

    /// The anchor in the top right corner.
    fn top_right() -> V<Anchor> {
        V(Anchor::TOP_RIGHT)
    }

    /// The offset of the anchor from the center, as a fraction of the sprite size.
    fn as_vec(_self: R<Anchor>) -> V<Vec2> {
        V(_self.as_vec())
    }
}

/// Registers the anchor functions.
pub(crate) fn register(world: &mut World) {
    register_anchor_functions(world);
}
