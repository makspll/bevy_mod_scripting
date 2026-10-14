//! Rotations as quaternions: [`Quat`].

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{EulerRot, Mat3, Quat, Vec3};
use bevy_mod_scripting_bindings::{
    InteropError,
    function::from::{R, Union, V},
};
use bevy_mod_scripting_derive::script_bindings;

/// Normalizes a direction, erroring if it is zero or not finite.
fn direction(v: Vec3) -> Result<Vec3, InteropError> {
    v.try_normalize()
        .ok_or_else(|| InteropError::str("cannot use a zero or non-finite vector as a direction"))
}

#[script_bindings(
    remote,
    name = "quat_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Quat {
    /// The identity rotation, which leaves everything unchanged.
    fn identity() -> V<Quat> {
        V(Quat::IDENTITY)
    }

    /// A rotation of `angle` radians around the x axis.
    fn from_rotation_x(angle: f32) -> V<Quat> {
        V(Quat::from_rotation_x(angle))
    }

    /// A rotation of `angle` radians around the y axis.
    fn from_rotation_y(angle: f32) -> V<Quat> {
        V(Quat::from_rotation_y(angle))
    }

    /// A rotation of `angle` radians around the z axis.
    fn from_rotation_z(angle: f32) -> V<Quat> {
        V(Quat::from_rotation_z(angle))
    }

    /// A rotation of `angle` radians around `axis`. Errors if the axis is zero or not finite.
    fn from_axis_angle(axis: R<Vec3>, angle: f32) -> Result<V<Quat>, InteropError> {
        let axis = direction(*axis)?;
        Ok(V(Quat::from_axis_angle(axis, angle)))
    }

    /// The rotation that turns the direction `from` onto the direction `to`. Errors if either vector is zero or not finite.
    fn from_rotation_arc(from: R<Vec3>, to: R<Vec3>) -> Result<V<Quat>, InteropError> {
        let from = direction(*from)?;
        let to = direction(*to)?;
        Ok(V(Quat::from_rotation_arc(from, to)))
    }

    /// A rotation from three angles in radians, applied in the given [`EulerRot`](bevy_math::EulerRot) order, i.e. `EulerRot.YXZ`.
    fn from_euler(order: R<EulerRot>, a: f32, b: f32, c: f32) -> V<Quat> {
        V(Quat::from_euler(*order, a, b, c))
    }

    /// The three angles in radians of this rotation, in the given [`EulerRot`](bevy_math::EulerRot) order.
    fn to_euler(_self: R<Quat>, order: R<EulerRot>) -> Vec<f32> {
        let (a, b, c) = _self.to_euler(*order);
        vec![a, b, c]
    }

    /// The rotation described by a 3x3 rotation matrix.
    fn from_mat3(m: R<Mat3>) -> V<Quat> {
        V(Quat::from_mat3(&m))
    }

    /// Composes two rotations when given a rotation, or rotates a vector when given a [`Vec3`](bevy_math::Vec3).
    fn mul(_self: R<Quat>, rhs: Union<R<Quat>, R<Vec3>>) -> Union<V<Quat>, V<Vec3>> {
        match rhs.into_left() {
            Ok(q) => Union::new_left(V(*_self * *q)),
            Err(v) => Union::new_right(V(*_self * *v)),
        }
    }

    /// Rotates a [`Vec3`](bevy_math::Vec3).
    fn mul_vec3(_self: R<Quat>, v: R<Vec3>) -> V<Vec3> {
        V(_self.mul_vec3(*v))
    }

    /// Whether both rotations have exactly the same components.
    fn eq(_self: R<Quat>, other: R<Quat>) -> bool {
        *_self == *other
    }

    /// The opposite rotation, with every component negated.
    fn neg(_self: R<Quat>) -> V<Quat> {
        V(-*_self)
    }

    /// The rotation scaled to a length of `1`.
    fn normalize(_self: R<Quat>) -> V<Quat> {
        V(_self.normalize())
    }

    /// The rotation that undoes this one.
    fn inverse(_self: R<Quat>) -> V<Quat> {
        V(_self.inverse())
    }

    /// The rotation with the vector part negated, which is the inverse for unit quaternions.
    fn conjugate(_self: R<Quat>) -> V<Quat> {
        V(_self.conjugate())
    }

    /// The length of the quaternion, which is `1` for a rotation.
    fn length(_self: R<Quat>) -> f32 {
        _self.length()
    }

    /// The dot product of two rotations.
    fn dot(_self: R<Quat>, other: R<Quat>) -> f32 {
        _self.dot(*other)
    }

    /// Spherically interpolates towards `other`, `t = 0` gives this rotation and `t = 1` gives `other`.
    fn slerp(_self: R<Quat>, other: R<Quat>, t: f32) -> V<Quat> {
        V(_self.slerp(*other, t))
    }

    /// Linearly interpolates towards `other`, `t = 0` gives this rotation and `t = 1` gives `other`.
    fn lerp(_self: R<Quat>, other: R<Quat>, t: f32) -> V<Quat> {
        V(_self.lerp(*other, t))
    }

    /// The angle in radians between two rotations.
    fn angle_between(_self: R<Quat>, other: R<Quat>) -> f32 {
        _self.angle_between(*other)
    }

    /// The axis this rotation turns around.
    fn axis(_self: R<Quat>) -> V<Vec3> {
        V(_self.to_axis_angle().0)
    }

    /// The angle in radians this rotation turns by around its [`axis`](Self::axis).
    fn angle(_self: R<Quat>) -> f32 {
        _self.to_axis_angle().1
    }

    /// The vector part of the quaternion, i.e. the x, y and z components.
    fn xyz(_self: R<Quat>) -> V<Vec3> {
        V(_self.xyz())
    }

    /// Whether the quaternion has a length of `1`.
    fn is_normalized(_self: R<Quat>) -> bool {
        _self.is_normalized()
    }
}

/// Registers the functions of quaternions.
pub(crate) fn register(world: &mut World) {
    register_quat_functions(world);
}
