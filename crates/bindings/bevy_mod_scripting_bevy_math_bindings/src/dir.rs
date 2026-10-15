//! Unit directions: [`Dir2`] and [`Dir3`].

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{Dir2, Dir3, Vec2, Vec3};
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "dir2_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl Dir2 {
    /// A unit direction pointing the same way as the vector, or nil if the vector is zero or not finite.
    fn new(v: R<Vec2>) -> Option<V<Dir2>> {
        Dir2::new(*v).ok().map(V)
    }

    /// The direction along the positive x axis.
    fn x() -> V<Dir2> {
        V(Dir2::X)
    }

    /// The direction along the positive y axis.
    fn y() -> V<Dir2> {
        V(Dir2::Y)
    }

    /// The direction along the negative x axis.
    fn neg_x() -> V<Dir2> {
        V(Dir2::NEG_X)
    }

    /// The direction along the negative y axis.
    fn neg_y() -> V<Dir2> {
        V(Dir2::NEG_Y)
    }

    /// This direction as a unit [`Vec2`](bevy_math::Vec2).
    fn as_vec2(_self: R<Dir2>) -> V<Vec2> {
        V(Vec2::from(*_self))
    }

    /// Spherically interpolates between this direction and `other` by `t`, from 0 to 1.
    fn slerp(_self: R<Dir2>, other: R<Dir2>, t: f32) -> V<Dir2> {
        V(_self.slerp(*other, t))
    }

    /// This direction scaled by `length`, as a [`Vec2`](bevy_math::Vec2).
    fn mul(_self: R<Dir2>, length: f32) -> V<Vec2> {
        V(*_self * length)
    }

    /// The opposite direction.
    fn neg(_self: R<Dir2>) -> V<Dir2> {
        V(-*_self)
    }

    /// Whether both directions have exactly the same components.
    fn eq(_self: R<Dir2>, other: R<Dir2>) -> bool {
        *_self == *other
    }
}

#[script_bindings(
    remote,
    name = "dir3_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl Dir3 {
    /// A unit direction pointing the same way as the vector, or nil if the vector is zero or not finite.
    fn new(v: R<Vec3>) -> Option<V<Dir3>> {
        Dir3::new(*v).ok().map(V)
    }

    /// The direction along the positive x axis.
    fn x() -> V<Dir3> {
        V(Dir3::X)
    }

    /// The direction along the positive y axis.
    fn y() -> V<Dir3> {
        V(Dir3::Y)
    }

    /// The direction along the positive z axis.
    fn z() -> V<Dir3> {
        V(Dir3::Z)
    }

    /// The direction along the negative x axis.
    fn neg_x() -> V<Dir3> {
        V(Dir3::NEG_X)
    }

    /// The direction along the negative y axis.
    fn neg_y() -> V<Dir3> {
        V(Dir3::NEG_Y)
    }

    /// The direction along the negative z axis.
    fn neg_z() -> V<Dir3> {
        V(Dir3::NEG_Z)
    }

    /// This direction as a unit [`Vec3`](bevy_math::Vec3).
    fn as_vec3(_self: R<Dir3>) -> V<Vec3> {
        V(Vec3::from(*_self))
    }

    /// Spherically interpolates between this direction and `other` by `t`, from 0 to 1.
    fn slerp(_self: R<Dir3>, other: R<Dir3>, t: f32) -> V<Dir3> {
        V(_self.slerp(*other, t))
    }

    /// This direction scaled by `length`, as a [`Vec3`](bevy_math::Vec3).
    fn mul(_self: R<Dir3>, length: f32) -> V<Vec3> {
        V(*_self * length)
    }

    /// The opposite direction.
    fn neg(_self: R<Dir3>) -> V<Dir3> {
        V(-*_self)
    }

    /// Whether both directions have exactly the same components.
    fn eq(_self: R<Dir3>, other: R<Dir3>) -> bool {
        *_self == *other
    }
}

/// Registers the functions of directions.
pub(crate) fn register(world: &mut World) {
    register_dir2_functions(world);
    register_dir3_functions(world);
}
