//! Matrices: [`Mat3`] and [`Mat4`].

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
use bevy_mod_scripting_bindings::{
    InteropError,
    function::from::{R, Union, V},
};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "mat3_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl Mat3 {
    /// The identity matrix, which leaves vectors unchanged.
    fn identity() -> V<Mat3> {
        V(Mat3::IDENTITY)
    }

    /// A matrix with the given columns.
    fn from_cols(x: R<Vec3>, y: R<Vec3>, z: R<Vec3>) -> V<Mat3> {
        V(Mat3::from_cols(*x, *y, *z))
    }

    /// A matrix with the given diagonal and zeros elsewhere.
    fn from_diagonal(diagonal: R<Vec3>) -> V<Mat3> {
        V(Mat3::from_diagonal(*diagonal))
    }

    /// The rotation matrix of a rotation.
    fn from_quat(q: R<Quat>) -> V<Mat3> {
        V(Mat3::from_quat(*q))
    }

    /// A rotation matrix of `angle` radians around the x axis.
    fn from_rotation_x(angle: f32) -> V<Mat3> {
        V(Mat3::from_rotation_x(angle))
    }

    /// A rotation matrix of `angle` radians around the y axis.
    fn from_rotation_y(angle: f32) -> V<Mat3> {
        V(Mat3::from_rotation_y(angle))
    }

    /// A rotation matrix of `angle` radians around the z axis.
    fn from_rotation_z(angle: f32) -> V<Mat3> {
        V(Mat3::from_rotation_z(angle))
    }

    /// The column at `index`, starting from `0`. Errors if `index` is out of range.
    fn col(_self: R<Mat3>, index: usize) -> Result<V<Vec3>, InteropError> {
        if index > 2 {
            return Err(InteropError::str(
                "column index out of range, expected 0 to 2",
            ));
        }
        Ok(V(_self.col(index)))
    }

    /// The row at `index`, starting from `0`. Errors if `index` is out of range.
    fn row(_self: R<Mat3>, index: usize) -> Result<V<Vec3>, InteropError> {
        if index > 2 {
            return Err(InteropError::str("row index out of range, expected 0 to 2"));
        }
        Ok(V(_self.row(index)))
    }

    /// The matrix with rows and columns swapped.
    fn transpose(_self: R<Mat3>) -> V<Mat3> {
        V(_self.transpose())
    }

    /// The matrix that undoes this one.
    fn inverse(_self: R<Mat3>) -> V<Mat3> {
        V(_self.inverse())
    }

    /// The determinant of the matrix.
    fn determinant(_self: R<Mat3>) -> f32 {
        _self.determinant()
    }

    /// Multiplies by a matrix, or transforms a vector, or scales every component by a number.
    fn mul(_self: R<Mat3>, rhs: Union<R<Mat3>, Union<R<Vec3>, f32>>) -> Union<V<Mat3>, V<Vec3>> {
        match rhs.into_left() {
            Ok(m) => Union::new_left(V(*_self * *m)),
            Err(rest) => match rest.into_left() {
                Ok(v) => Union::new_right(V(*_self * *v)),
                Err(s) => Union::new_left(V(*_self * s)),
            },
        }
    }

    /// Whether both matrices have exactly the same components.
    fn eq(_self: R<Mat3>, other: R<Mat3>) -> bool {
        *_self == *other
    }

    /// Adds two matrices component-wise.
    fn add(_self: R<Mat3>, other: R<Mat3>) -> V<Mat3> {
        V(*_self + *other)
    }

    /// Subtracts two matrices component-wise.
    fn sub(_self: R<Mat3>, other: R<Mat3>) -> V<Mat3> {
        V(*_self - *other)
    }

    /// Transforms a [`Vec3`](bevy_math::Vec3) by this matrix.
    fn mul_vec3(_self: R<Mat3>, v: R<Vec3>) -> V<Vec3> {
        V(_self.mul_vec3(*v))
    }

    /// Transforms a point in 2D, treating it as having a z of `1`.
    fn transform_point2(_self: R<Mat3>, point: R<Vec2>) -> V<Vec2> {
        V(_self.transform_point2(*point))
    }

    /// Transforms a direction in 2D, ignoring translation.
    fn transform_vector2(_self: R<Mat3>, vector: R<Vec2>) -> V<Vec2> {
        V(_self.transform_vector2(*vector))
    }
}

#[script_bindings(
    remote,
    name = "mat4_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl Mat4 {
    /// The identity matrix, which leaves vectors unchanged.
    fn identity() -> V<Mat4> {
        V(Mat4::IDENTITY)
    }

    /// A matrix with the given columns.
    fn from_cols(x: R<Vec4>, y: R<Vec4>, z: R<Vec4>, w: R<Vec4>) -> V<Mat4> {
        V(Mat4::from_cols(*x, *y, *z, *w))
    }

    /// The rotation matrix of a rotation.
    fn from_quat(q: R<Quat>) -> V<Mat4> {
        V(Mat4::from_quat(*q))
    }

    /// A matrix that moves points by `translation`.
    fn from_translation(translation: R<Vec3>) -> V<Mat4> {
        V(Mat4::from_translation(*translation))
    }

    /// A matrix that scales every axis by the matching component of `scale`.
    fn from_scale(scale: R<Vec3>) -> V<Mat4> {
        V(Mat4::from_scale(*scale))
    }

    /// A matrix that rotates by `rotation`, then moves by `translation`.
    fn from_rotation_translation(rotation: R<Quat>, translation: R<Vec3>) -> V<Mat4> {
        V(Mat4::from_rotation_translation(*rotation, *translation))
    }

    /// A matrix that scales by `scale`, then rotates by `rotation`, then moves by `translation`.
    fn from_scale_rotation_translation(
        scale: R<Vec3>,
        rotation: R<Quat>,
        translation: R<Vec3>,
    ) -> V<Mat4> {
        V(Mat4::from_scale_rotation_translation(
            *scale,
            *rotation,
            *translation,
        ))
    }

    /// The column at `index`, starting from `0`. Errors if `index` is out of range.
    fn col(_self: R<Mat4>, index: usize) -> Result<V<Vec4>, InteropError> {
        if index > 3 {
            return Err(InteropError::str(
                "column index out of range, expected 0 to 3",
            ));
        }
        Ok(V(_self.col(index)))
    }

    /// The row at `index`, starting from `0`. Errors if `index` is out of range.
    fn row(_self: R<Mat4>, index: usize) -> Result<V<Vec4>, InteropError> {
        if index > 3 {
            return Err(InteropError::str("row index out of range, expected 0 to 3"));
        }
        Ok(V(_self.row(index)))
    }

    /// The matrix with rows and columns swapped.
    fn transpose(_self: R<Mat4>) -> V<Mat4> {
        V(_self.transpose())
    }

    /// The matrix that undoes this one.
    fn inverse(_self: R<Mat4>) -> V<Mat4> {
        V(_self.inverse())
    }

    /// The determinant of the matrix.
    fn determinant(_self: R<Mat4>) -> f32 {
        _self.determinant()
    }

    /// Multiplies by a matrix, or transforms a vector, or scales every component by a number.
    fn mul(_self: R<Mat4>, rhs: Union<R<Mat4>, Union<R<Vec4>, f32>>) -> Union<V<Mat4>, V<Vec4>> {
        match rhs.into_left() {
            Ok(m) => Union::new_left(V(*_self * *m)),
            Err(rest) => match rest.into_left() {
                Ok(v) => Union::new_right(V(*_self * *v)),
                Err(s) => Union::new_left(V(*_self * s)),
            },
        }
    }

    /// Whether both matrices have exactly the same components.
    fn eq(_self: R<Mat4>, other: R<Mat4>) -> bool {
        *_self == *other
    }

    /// Adds two matrices component-wise.
    fn add(_self: R<Mat4>, other: R<Mat4>) -> V<Mat4> {
        V(*_self + *other)
    }

    /// Subtracts two matrices component-wise.
    fn sub(_self: R<Mat4>, other: R<Mat4>) -> V<Mat4> {
        V(*_self - *other)
    }

    /// Transforms a point in 3D, including translation and the projection divide.
    fn transform_point3(_self: R<Mat4>, point: R<Vec3>) -> V<Vec3> {
        V(_self.transform_point3(*point))
    }

    /// Transforms a direction in 3D, ignoring translation.
    fn transform_vector3(_self: R<Mat4>, vector: R<Vec3>) -> V<Vec3> {
        V(_self.transform_vector3(*vector))
    }

    /// Transforms a point in 3D by a projection matrix, including the perspective divide.
    fn project_point3(_self: R<Mat4>, point: R<Vec3>) -> V<Vec3> {
        V(_self.project_point3(*point))
    }
}

/// Registers the functions of matrices.
pub(crate) fn register(world: &mut World) {
    register_mat3_functions(world);
    register_mat4_functions(world);
}
