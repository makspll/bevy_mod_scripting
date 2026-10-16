//! Local transforms: [`Transform`](bevy_transform::components::Transform).

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{Dir3, Mat4, Quat, Vec3};
use bevy_mod_scripting_bindings::{
    InteropError,
    function::from::{M, R, Union, V},
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_transform::components::Transform;

/// Normalizes a direction, erroring if it is zero or not finite.
fn to_dir(v: Vec3) -> Result<Dir3, InteropError> {
    Dir3::new(v).map_err(|_| InteropError::str("direction must not be zero or non-finite"))
}

#[script_bindings(
    remote,
    name = "transform_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Transform {
    /// The transform with no translation, no rotation and a scale of `1`.
    fn identity() -> V<Transform> {
        V(Transform::IDENTITY)
    }

    /// A transform at the position `x`, `y`, `z`.
    ///
    /// ```multi-lang-example,lua
    /// local t = Transform.from_xyz(1, 2, 3)
    /// assert(t.translation.x == 1)
    /// ```
    ///
    /// ```multi-lang-example,rhai
    /// let t = Transform.from_xyz.call(1.0, 2.0, 3.0);
    /// assert(t.translation.x == 1.0);
    /// ```
    fn from_xyz(x: f32, y: f32, z: f32) -> V<Transform> {
        V(Transform::from_xyz(x, y, z))
    }

    /// A transform at the given position.
    fn from_translation(translation: R<Vec3>) -> V<Transform> {
        V(Transform::from_translation(*translation))
    }

    /// A transform with the given rotation.
    fn from_rotation(rotation: R<Quat>) -> V<Transform> {
        V(Transform::from_rotation(*rotation))
    }

    /// A transform with the given scale.
    fn from_scale(scale: R<Vec3>) -> V<Transform> {
        V(Transform::from_scale(*scale))
    }

    /// The transform described by a 4x4 matrix.
    fn from_matrix(matrix: R<Mat4>) -> V<Transform> {
        V(Transform::from_matrix(*matrix))
    }

    /// A copy of this transform with a new position.
    fn with_translation(_self: R<Transform>, translation: R<Vec3>) -> V<Transform> {
        V((*_self).with_translation(*translation))
    }

    /// A copy of this transform with a new rotation.
    fn with_rotation(_self: R<Transform>, rotation: R<Quat>) -> V<Transform> {
        V((*_self).with_rotation(*rotation))
    }

    /// A copy of this transform with a new scale.
    fn with_scale(_self: R<Transform>, scale: R<Vec3>) -> V<Transform> {
        V((*_self).with_scale(*scale))
    }

    /// A copy of this transform rotated to face `target`, with `up` as the up direction. Errors if `up` is zero.
    fn looking_at(
        _self: R<Transform>,
        target: R<Vec3>,
        up: R<Vec3>,
    ) -> Result<V<Transform>, InteropError> {
        let up = to_dir(*up)?;
        Ok(V((*_self).looking_at(*target, up)))
    }

    /// A copy of this transform rotated to face along `direction`, with `up` as the up direction. Errors if either vector is zero.
    fn looking_to(
        _self: R<Transform>,
        direction: R<Vec3>,
        up: R<Vec3>,
    ) -> Result<V<Transform>, InteropError> {
        let direction = to_dir(*direction)?;
        let up = to_dir(*up)?;
        Ok(V((*_self).looking_to(direction, up)))
    }

    /// Rotates this transform by `rotation`, in the parent's space.
    fn rotate(mut _self: M<Transform>, rotation: R<Quat>) {
        _self.rotate(*rotation);
    }

    /// Rotates this transform by `angle` radians around the x axis.
    fn rotate_x(mut _self: M<Transform>, angle: f32) {
        _self.rotate_x(angle);
    }

    /// Rotates this transform by `angle` radians around the y axis.
    fn rotate_y(mut _self: M<Transform>, angle: f32) {
        _self.rotate_y(angle);
    }

    /// Rotates this transform by `angle` radians around the z axis.
    fn rotate_z(mut _self: M<Transform>, angle: f32) {
        _self.rotate_z(angle);
    }

    /// Rotates this transform by `angle` radians around `axis`. Errors if the axis is zero.
    fn rotate_axis(mut _self: M<Transform>, axis: R<Vec3>, angle: f32) -> Result<(), InteropError> {
        let axis = to_dir(*axis)?;
        _self.rotate_axis(axis, angle);
        Ok(())
    }

    /// Rotates this transform by `rotation`, in its own local space.
    fn rotate_local(mut _self: M<Transform>, rotation: R<Quat>) {
        _self.rotate_local(*rotation);
    }

    /// Rotates this transform by `angle` radians around its local x axis.
    fn rotate_local_x(mut _self: M<Transform>, angle: f32) {
        _self.rotate_local_x(angle);
    }

    /// Rotates this transform by `angle` radians around its local y axis.
    fn rotate_local_y(mut _self: M<Transform>, angle: f32) {
        _self.rotate_local_y(angle);
    }

    /// Rotates this transform by `angle` radians around its local z axis.
    fn rotate_local_z(mut _self: M<Transform>, angle: f32) {
        _self.rotate_local_z(angle);
    }

    /// Rotates this transform by `rotation` around `point`, moving it along the circle.
    fn rotate_around(mut _self: M<Transform>, point: R<Vec3>, rotation: R<Quat>) {
        _self.rotate_around(*point, *rotation);
    }

    /// Moves this transform around `point` by `rotation`, without changing its rotation.
    fn translate_around(mut _self: M<Transform>, point: R<Vec3>, rotation: R<Quat>) {
        _self.translate_around(*point, *rotation);
    }

    /// Rotates this transform to face `target`, with `up` as the up direction. Errors if `up` is zero.
    fn look_at(mut _self: M<Transform>, target: R<Vec3>, up: R<Vec3>) -> Result<(), InteropError> {
        let up = to_dir(*up)?;
        _self.look_at(*target, up);
        Ok(())
    }

    /// Rotates this transform to face along `direction`, with `up` as the up direction. Errors if either vector is zero.
    fn look_to(
        mut _self: M<Transform>,
        direction: R<Vec3>,
        up: R<Vec3>,
    ) -> Result<(), InteropError> {
        let direction = to_dir(*direction)?;
        let up = to_dir(*up)?;
        _self.look_to(direction, up);
        Ok(())
    }

    /// The direction the transform faces, `-z` in local space.
    fn forward(_self: R<Transform>) -> V<Vec3> {
        V(_self.forward().as_vec3())
    }

    /// The direction opposite to [`forward`](Self::forward), `+z` in local space.
    fn back(_self: R<Transform>) -> V<Vec3> {
        V(_self.back().as_vec3())
    }

    /// The direction to the local left, `-x` in local space.
    fn left(_self: R<Transform>) -> V<Vec3> {
        V(_self.left().as_vec3())
    }

    /// The direction to the local right, `+x` in local space.
    fn right(_self: R<Transform>) -> V<Vec3> {
        V(_self.right().as_vec3())
    }

    /// The local up direction, `+y`.
    fn up(_self: R<Transform>) -> V<Vec3> {
        V(_self.up().as_vec3())
    }

    /// The local down direction, `-y`.
    fn down(_self: R<Transform>) -> V<Vec3> {
        V(_self.down().as_vec3())
    }

    /// The local x axis, as a direction.
    fn local_x(_self: R<Transform>) -> V<Vec3> {
        V(_self.local_x().as_vec3())
    }

    /// The local y axis, as a direction.
    fn local_y(_self: R<Transform>) -> V<Vec3> {
        V(_self.local_y().as_vec3())
    }

    /// The local z axis, as a direction.
    fn local_z(_self: R<Transform>) -> V<Vec3> {
        V(_self.local_z().as_vec3())
    }

    /// Applies the transform to a point: scales, rotates, then translates it.
    fn transform_point(_self: R<Transform>, point: R<Vec3>) -> V<Vec3> {
        V(_self.transform_point(*point))
    }

    /// Composes two transforms when given a transform, or transforms a point when given a [`Vec3`](bevy_math::Vec3).
    fn mul(_self: R<Transform>, rhs: Union<R<Transform>, R<Vec3>>) -> Union<V<Transform>, V<Vec3>> {
        match rhs.into_left() {
            Ok(t) => Union::new_left(V(*_self * *t)),
            Err(v) => Union::new_right(V(_self.transform_point(*v))),
        }
    }

    /// Composes this transform with `other`, applying `other` first.
    fn mul_transform(_self: R<Transform>, other: R<Transform>) -> V<Transform> {
        V(_self.mul_transform(*other))
    }

    /// The matrix equivalent to this transform.
    fn to_matrix(_self: R<Transform>) -> V<Mat4> {
        V(_self.to_matrix())
    }

    /// Whether every component is a finite number.
    fn is_finite(_self: R<Transform>) -> bool {
        _self.is_finite()
    }

    /// Whether both transforms have exactly the same components.
    fn eq(_self: R<Transform>, other: R<Transform>) -> bool {
        *_self == *other
    }
}

/// Registers the functions of [`Transform`].
pub(crate) fn register(world: &mut World) {
    register_transform_functions(world);
}
