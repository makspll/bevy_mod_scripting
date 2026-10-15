//! 2D rotations: [`Rot2`].

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{Rot2, Vec2};
use bevy_mod_scripting_bindings::function::from::{R, Union, V};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "rot2_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl Rot2 {
    /// A counterclockwise rotation of `radians` radians.
    fn radians(radians: f32) -> V<Rot2> {
        V(Rot2::radians(radians))
    }

    /// A counterclockwise rotation of `degrees` degrees.
    fn degrees(degrees: f32) -> V<Rot2> {
        V(Rot2::degrees(degrees))
    }

    /// A counterclockwise rotation of `fraction` of a full turn, so `1.0` is a full turn.
    fn turn_fraction(fraction: f32) -> V<Rot2> {
        V(Rot2::turn_fraction(fraction))
    }

    /// The rotation that leaves everything unchanged.
    fn identity() -> V<Rot2> {
        V(Rot2::IDENTITY)
    }

    /// The angle of this rotation in radians.
    fn as_radians(_self: R<Rot2>) -> f32 {
        _self.as_radians()
    }

    /// The angle of this rotation in degrees.
    fn as_degrees(_self: R<Rot2>) -> f32 {
        _self.as_degrees()
    }

    /// The rotation that undoes this one.
    fn inverse(_self: R<Rot2>) -> V<Rot2> {
        V(_self.inverse())
    }

    /// Spherically interpolates between this rotation and `other` by `t`, from 0 to 1.
    fn slerp(_self: R<Rot2>, other: R<Rot2>, t: f32) -> V<Rot2> {
        V(_self.slerp(*other, t))
    }

    /// Linearly interpolates between this rotation and `other` by `t`, from 0 to 1, then normalizes.
    fn nlerp(_self: R<Rot2>, other: R<Rot2>, t: f32) -> V<Rot2> {
        V(_self.nlerp(*other, t))
    }

    /// Composes two rotations when given a rotation, or rotates a [`Vec2`](bevy_math::Vec2) when given a vector.
    fn mul(_self: R<Rot2>, rhs: Union<R<Rot2>, R<Vec2>>) -> Union<V<Rot2>, V<Vec2>> {
        match rhs.into_left() {
            Ok(r) => Union::new_left(V(*_self * *r)),
            Err(v) => Union::new_right(V(*_self * *v)),
        }
    }

    /// Whether both rotations have exactly the same components.
    fn eq(_self: R<Rot2>, other: R<Rot2>) -> bool {
        *_self == *other
    }
}

/// Registers the functions of 2D rotations.
pub(crate) fn register(world: &mut World) {
    register_rot2_functions(world);
}
