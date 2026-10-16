//! Axis-aligned rectangles: [`Rect`](bevy_math::Rect).

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{Rect, Vec2};
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "rect_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl Rect {
    /// A rectangle from its minimum corner `(x0, y0)` and maximum corner `(x1, y1)`.
    fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> V<Rect> {
        V(Rect::new(x0, y0, x1, y1))
    }

    /// A rectangle with the given center and full size.
    fn from_center_size(origin: R<Vec2>, size: R<Vec2>) -> V<Rect> {
        V(Rect::from_center_size(*origin, *size))
    }

    /// A rectangle with the given center and half of its full size.
    fn from_center_half_size(origin: R<Vec2>, half_size: R<Vec2>) -> V<Rect> {
        V(Rect::from_center_half_size(*origin, *half_size))
    }

    /// A rectangle spanning the two given corners, in either order.
    fn from_corners(p0: R<Vec2>, p1: R<Vec2>) -> V<Rect> {
        V(Rect::from_corners(*p0, *p1))
    }

    /// The width of the rectangle.
    fn width(_self: R<Rect>) -> f32 {
        _self.width()
    }

    /// The height of the rectangle.
    fn height(_self: R<Rect>) -> f32 {
        _self.height()
    }

    /// The full size of the rectangle as a [`Vec2`](bevy_math::Vec2).
    fn size(_self: R<Rect>) -> V<Vec2> {
        V(_self.size())
    }

    /// Half of the full size of the rectangle as a [`Vec2`](bevy_math::Vec2).
    fn half_size(_self: R<Rect>) -> V<Vec2> {
        V(_self.half_size())
    }

    /// The center of the rectangle as a [`Vec2`](bevy_math::Vec2).
    fn center(_self: R<Rect>) -> V<Vec2> {
        V(_self.center())
    }

    /// Whether the point lies inside the rectangle, including its edges.
    fn contains(_self: R<Rect>, point: R<Vec2>) -> bool {
        _self.contains(*point)
    }

    /// The smallest rectangle that contains both this one and `other`.
    fn union(_self: R<Rect>, other: R<Rect>) -> V<Rect> {
        V(_self.union(*other))
    }

    /// The overlap of this rectangle and `other`, which is empty if they do not overlap.
    fn intersect(_self: R<Rect>, other: R<Rect>) -> V<Rect> {
        V(_self.intersect(*other))
    }

    /// This rectangle grown by `expansion` on every side, or shrunk if it is negative.
    fn inflate(_self: R<Rect>, expansion: f32) -> V<Rect> {
        V(_self.inflate(expansion))
    }

    /// Whether the rectangle has no area.
    fn is_empty(_self: R<Rect>) -> bool {
        _self.is_empty()
    }

    /// Whether both rectangles have exactly the same corners.
    fn eq(_self: R<Rect>, other: R<Rect>) -> bool {
        *_self == *other
    }
}

/// Registers the functions of rectangles.
pub(crate) fn register(world: &mut World) {
    register_rect_functions(world);
}
