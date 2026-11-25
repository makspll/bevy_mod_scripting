//! Matrices: [`Mat2`], [`Mat3`], [`Mat4`], [`DMat2`], [`DMat3`] and [`DMat4`], plus conversions for [`Mat3A`].

use std::{any::TypeId, mem::offset_of, ops::Mul};

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{
    DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3,
    Vec3A, Vec4,
};
use bevy_mod_scripting_bindings::{
    InteropError, assert_ffi_layout,
    conversions::from::{R, Union, V},
    function::script_function::AppScriptFunctionRegistry,
    native::{FfiArg, NativeType},
};
use bevy_mod_scripting_derive::script_bindings;

/// Functions shared by all matrices.
// rustfmt keeps indenting the attribute inside the macro further on every run
#[rustfmt::skip]
macro_rules! matrix_functions {
    ($mat:ty, $vec:ty, $scalar:ty, $name:literal) => {
        #[script_bindings(
            remote,
            name = $name,
            bms_bindings_path = "bevy_mod_scripting_bindings",
            namespace = "math",
            core
        )]
        impl $mat {
            /// The identity matrix, which leaves vectors unchanged.
            #[ffi]
            fn identity() -> V<$mat> {
                V(<$mat>::IDENTITY)
            }

            /// A matrix with the given diagonal and zeros elsewhere.
            #[ffi]
            fn from_diagonal(diagonal: R<$vec>) -> V<$mat> {
                V(<$mat>::from_diagonal(*diagonal))
            }

            /// The matrix with rows and columns swapped.
            #[ffi]
            fn transpose(_self: R<$mat>) -> V<$mat> {
                V(_self.transpose())
            }

            /// The matrix that undoes this one.
            #[ffi]
            fn inverse(_self: R<$mat>) -> V<$mat> {
                V(_self.inverse())
            }

            /// The determinant of the matrix.
            #[ffi]
            fn determinant(_self: R<$mat>) -> $scalar {
                _self.determinant()
            }

            /// Multiplies by a matrix, or transforms a vector, or scales every component by a number.
            fn mul(
                _self: R<$mat>,
                rhs: Union<R<$mat>, Union<R<$vec>, $scalar>>,
            ) -> Union<V<$mat>, V<$vec>> {
                match rhs.into_left() {
                    Ok(m) => Union::new_left(V(*_self * *m)),
                    Err(rest) => match rest.into_left() {
                        Ok(v) => Union::new_right(V(*_self * *v)),
                        Err(s) => Union::new_left(V(*_self * s)),
                    },
                }
            }

            /// Whether both matrices have exactly the same components.
            #[ffi]
            fn eq(_self: R<$mat>, other: R<$mat>) -> bool {
                *_self == *other
            }

            /// Adds two matrices component-wise.
            #[ffi]
            fn add(_self: R<$mat>, other: R<$mat>) -> V<$mat> {
                V(*_self + *other)
            }

            /// Subtracts two matrices component-wise.
            #[ffi]
            fn sub(_self: R<$mat>, other: R<$mat>) -> V<$mat> {
                V(*_self - *other)
            }

            /// Divides every component by a number.
            #[ffi]
            fn div(_self: R<$mat>, rhs: $scalar) -> V<$mat> {
                V(*_self / rhs)
            }
        }
    };
}

matrix_functions!(Mat2, Vec2, f32, "mat2_shared_functions");
matrix_functions!(Mat3, Vec3, f32, "mat3_shared_functions");
matrix_functions!(Mat4, Vec4, f32, "mat4_shared_functions");
matrix_functions!(DMat2, DVec2, f64, "dmat2_shared_functions");
matrix_functions!(DMat3, DVec3, f64, "dmat3_shared_functions");
matrix_functions!(DMat4, DVec4, f64, "dmat4_shared_functions");

#[script_bindings(
    remote,
    name = "mat2_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl Mat2 {
    /// A matrix with the given columns.
    #[ffi]
    fn from_cols(x: R<Vec2>, y: R<Vec2>) -> V<Mat2> {
        V(Mat2::from_cols(*x, *y))
    }

    /// A rotation matrix of `angle` radians.
    #[ffi]
    fn from_angle(angle: f32) -> V<Mat2> {
        V(Mat2::from_angle(angle))
    }

    /// The column at `index`, starting from `0`. Errors if `index` is out of range.
    fn col(_self: R<Mat2>, index: usize) -> Result<V<Vec2>, InteropError> {
        if index > 1 {
            return Err(InteropError::str(
                "column index out of range, expected 0 to 1",
            ));
        }
        Ok(V(_self.col(index)))
    }

    /// The row at `index`, starting from `0`. Errors if `index` is out of range.
    fn row(_self: R<Mat2>, index: usize) -> Result<V<Vec2>, InteropError> {
        if index > 1 {
            return Err(InteropError::str("row index out of range, expected 0 to 1"));
        }
        Ok(V(_self.row(index)))
    }

    /// Transforms a [`Vec2`](bevy_math::Vec2) by this matrix.
    #[ffi]
    fn mul_vec2(_self: R<Mat2>, v: R<Vec2>) -> V<Vec2> {
        V(_self.mul_vec2(*v))
    }

    /// Converts to a double precision matrix.
    #[ffi]
    fn as_dmat2(_self: R<Mat2>) -> V<DMat2> {
        V(_self.as_dmat2())
    }
}

#[script_bindings(
    remote,
    name = "mat3_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl Mat3 {
    /// A matrix with the given columns.
    #[ffi]
    fn from_cols(x: R<Vec3>, y: R<Vec3>, z: R<Vec3>) -> V<Mat3> {
        V(Mat3::from_cols(*x, *y, *z))
    }

    /// The rotation matrix of a rotation.
    #[ffi]
    fn from_quat(q: R<Quat>) -> V<Mat3> {
        V(Mat3::from_quat(*q))
    }

    /// A rotation matrix of `angle` radians around the x axis.
    #[ffi]
    fn from_rotation_x(angle: f32) -> V<Mat3> {
        V(Mat3::from_rotation_x(angle))
    }

    /// A rotation matrix of `angle` radians around the y axis.
    #[ffi]
    fn from_rotation_y(angle: f32) -> V<Mat3> {
        V(Mat3::from_rotation_y(angle))
    }

    /// A rotation matrix of `angle` radians around the z axis.
    #[ffi]
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

    /// Transforms a [`Vec3`](bevy_math::Vec3) by this matrix.
    #[ffi]
    fn mul_vec3(_self: R<Mat3>, v: R<Vec3>) -> V<Vec3> {
        V(_self.mul_vec3(*v))
    }

    /// Transforms a point in 2D, treating it as having a z of `1`.
    #[ffi]
    fn transform_point2(_self: R<Mat3>, point: R<Vec2>) -> V<Vec2> {
        V(_self.transform_point2(*point))
    }

    /// Transforms a direction in 2D, ignoring translation.
    #[ffi]
    fn transform_vector2(_self: R<Mat3>, vector: R<Vec2>) -> V<Vec2> {
        V(_self.transform_vector2(*vector))
    }

    /// Converts to a double precision matrix.
    #[ffi]
    fn as_dmat3(_self: R<Mat3>) -> V<DMat3> {
        V(_self.as_dmat3())
    }

    /// Converts to a [`Mat3A`](bevy_math::Mat3A).
    #[ffi]
    fn to_mat3a(_self: R<Mat3>) -> V<Mat3A> {
        V(Mat3A::from(*_self))
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
    /// A matrix with the given columns.
    #[ffi]
    fn from_cols(x: R<Vec4>, y: R<Vec4>, z: R<Vec4>, w: R<Vec4>) -> V<Mat4> {
        V(Mat4::from_cols(*x, *y, *z, *w))
    }

    /// The rotation matrix of a rotation.
    #[ffi]
    fn from_quat(q: R<Quat>) -> V<Mat4> {
        V(Mat4::from_quat(*q))
    }

    /// A matrix that moves points by `translation`.
    #[ffi]
    fn from_translation(translation: R<Vec3>) -> V<Mat4> {
        V(Mat4::from_translation(*translation))
    }

    /// A matrix that scales every axis by the matching component of `scale`.
    #[ffi]
    fn from_scale(scale: R<Vec3>) -> V<Mat4> {
        V(Mat4::from_scale(*scale))
    }

    /// A matrix that rotates by `rotation`, then moves by `translation`.
    #[ffi]
    fn from_rotation_translation(rotation: R<Quat>, translation: R<Vec3>) -> V<Mat4> {
        V(Mat4::from_rotation_translation(*rotation, *translation))
    }

    /// A matrix that scales by `scale`, then rotates by `rotation`, then moves by `translation`.
    #[ffi]
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

    /// Transforms a [`Vec4`](bevy_math::Vec4) by this matrix.
    #[ffi]
    fn mul_vec4(_self: R<Mat4>, v: R<Vec4>) -> V<Vec4> {
        V(_self.mul_vec4(*v))
    }

    /// Transforms a point in 3D, including translation and the projection divide.
    #[ffi]
    fn transform_point3(_self: R<Mat4>, point: R<Vec3>) -> V<Vec3> {
        V(_self.transform_point3(*point))
    }

    /// Transforms a direction in 3D, ignoring translation.
    #[ffi]
    fn transform_vector3(_self: R<Mat4>, vector: R<Vec3>) -> V<Vec3> {
        V(_self.transform_vector3(*vector))
    }

    /// Transforms a point in 3D by a projection matrix, including the perspective divide.
    #[ffi]
    fn project_point3(_self: R<Mat4>, point: R<Vec3>) -> V<Vec3> {
        V(_self.project_point3(*point))
    }

    /// Converts to a double precision matrix.
    #[ffi]
    fn as_dmat4(_self: R<Mat4>) -> V<DMat4> {
        V(_self.as_dmat4())
    }
}

#[script_bindings(
    remote,
    name = "mat3a_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl Mat3A {
    /// Converts to a [`Mat3`](bevy_math::Mat3).
    #[ffi]
    fn to_mat3(_self: R<Mat3A>) -> V<Mat3> {
        V(Mat3::from(*_self))
    }
}

#[script_bindings(
    remote,
    name = "dmat2_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl DMat2 {
    /// A matrix with the given columns.
    #[ffi]
    fn from_cols(x: R<DVec2>, y: R<DVec2>) -> V<DMat2> {
        V(DMat2::from_cols(*x, *y))
    }

    /// A rotation matrix of `angle` radians.
    #[ffi]
    fn from_angle(angle: f64) -> V<DMat2> {
        V(DMat2::from_angle(angle))
    }

    /// The column at `index`, starting from `0`. Errors if `index` is out of range.
    fn col(_self: R<DMat2>, index: usize) -> Result<V<DVec2>, InteropError> {
        if index > 1 {
            return Err(InteropError::str(
                "column index out of range, expected 0 to 1",
            ));
        }
        Ok(V(_self.col(index)))
    }

    /// The row at `index`, starting from `0`. Errors if `index` is out of range.
    fn row(_self: R<DMat2>, index: usize) -> Result<V<DVec2>, InteropError> {
        if index > 1 {
            return Err(InteropError::str("row index out of range, expected 0 to 1"));
        }
        Ok(V(_self.row(index)))
    }

    /// Transforms a [`DVec2`](bevy_math::DVec2) by this matrix.
    #[ffi]
    fn mul_vec2(_self: R<DMat2>, v: R<DVec2>) -> V<DVec2> {
        V(_self.mul_vec2(*v))
    }

    /// Converts to a single precision matrix.
    #[ffi]
    fn as_mat2(_self: R<DMat2>) -> V<Mat2> {
        V(_self.as_mat2())
    }
}

#[script_bindings(
    remote,
    name = "dmat3_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl DMat3 {
    /// A matrix with the given columns.
    #[ffi]
    fn from_cols(x: R<DVec3>, y: R<DVec3>, z: R<DVec3>) -> V<DMat3> {
        V(DMat3::from_cols(*x, *y, *z))
    }

    /// The rotation matrix of a rotation.
    #[ffi]
    fn from_quat(q: R<DQuat>) -> V<DMat3> {
        V(DMat3::from_quat(*q))
    }

    /// A rotation matrix of `angle` radians around the x axis.
    #[ffi]
    fn from_rotation_x(angle: f64) -> V<DMat3> {
        V(DMat3::from_rotation_x(angle))
    }

    /// A rotation matrix of `angle` radians around the y axis.
    #[ffi]
    fn from_rotation_y(angle: f64) -> V<DMat3> {
        V(DMat3::from_rotation_y(angle))
    }

    /// A rotation matrix of `angle` radians around the z axis.
    #[ffi]
    fn from_rotation_z(angle: f64) -> V<DMat3> {
        V(DMat3::from_rotation_z(angle))
    }

    /// The column at `index`, starting from `0`. Errors if `index` is out of range.
    fn col(_self: R<DMat3>, index: usize) -> Result<V<DVec3>, InteropError> {
        if index > 2 {
            return Err(InteropError::str(
                "column index out of range, expected 0 to 2",
            ));
        }
        Ok(V(_self.col(index)))
    }

    /// The row at `index`, starting from `0`. Errors if `index` is out of range.
    fn row(_self: R<DMat3>, index: usize) -> Result<V<DVec3>, InteropError> {
        if index > 2 {
            return Err(InteropError::str("row index out of range, expected 0 to 2"));
        }
        Ok(V(_self.row(index)))
    }

    /// Transforms a [`DVec3`](bevy_math::DVec3) by this matrix.
    #[ffi]
    fn mul_vec3(_self: R<DMat3>, v: R<DVec3>) -> V<DVec3> {
        V(_self.mul_vec3(*v))
    }

    /// Transforms a point in 2D, treating it as having a z of `1`.
    #[ffi]
    fn transform_point2(_self: R<DMat3>, point: R<DVec2>) -> V<DVec2> {
        V(_self.transform_point2(*point))
    }

    /// Transforms a direction in 2D, ignoring translation.
    #[ffi]
    fn transform_vector2(_self: R<DMat3>, vector: R<DVec2>) -> V<DVec2> {
        V(_self.transform_vector2(*vector))
    }

    /// Converts to a single precision matrix.
    #[ffi]
    fn as_mat3(_self: R<DMat3>) -> V<Mat3> {
        V(_self.as_mat3())
    }
}

#[script_bindings(
    remote,
    name = "dmat4_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl DMat4 {
    /// A matrix with the given columns.
    #[ffi]
    fn from_cols(x: R<DVec4>, y: R<DVec4>, z: R<DVec4>, w: R<DVec4>) -> V<DMat4> {
        V(DMat4::from_cols(*x, *y, *z, *w))
    }

    /// The rotation matrix of a rotation.
    #[ffi]
    fn from_quat(q: R<DQuat>) -> V<DMat4> {
        V(DMat4::from_quat(*q))
    }

    /// A matrix that moves points by `translation`.
    #[ffi]
    fn from_translation(translation: R<DVec3>) -> V<DMat4> {
        V(DMat4::from_translation(*translation))
    }

    /// A matrix that scales every axis by the matching component of `scale`.
    #[ffi]
    fn from_scale(scale: R<DVec3>) -> V<DMat4> {
        V(DMat4::from_scale(*scale))
    }

    /// A matrix that rotates by `rotation`, then moves by `translation`.
    #[ffi]
    fn from_rotation_translation(rotation: R<DQuat>, translation: R<DVec3>) -> V<DMat4> {
        V(DMat4::from_rotation_translation(*rotation, *translation))
    }

    /// A matrix that scales by `scale`, then rotates by `rotation`, then moves by `translation`.
    #[ffi]
    fn from_scale_rotation_translation(
        scale: R<DVec3>,
        rotation: R<DQuat>,
        translation: R<DVec3>,
    ) -> V<DMat4> {
        V(DMat4::from_scale_rotation_translation(
            *scale,
            *rotation,
            *translation,
        ))
    }

    /// The column at `index`, starting from `0`. Errors if `index` is out of range.
    fn col(_self: R<DMat4>, index: usize) -> Result<V<DVec4>, InteropError> {
        if index > 3 {
            return Err(InteropError::str(
                "column index out of range, expected 0 to 3",
            ));
        }
        Ok(V(_self.col(index)))
    }

    /// The row at `index`, starting from `0`. Errors if `index` is out of range.
    fn row(_self: R<DMat4>, index: usize) -> Result<V<DVec4>, InteropError> {
        if index > 3 {
            return Err(InteropError::str("row index out of range, expected 0 to 3"));
        }
        Ok(V(_self.row(index)))
    }

    /// Transforms a [`DVec4`](bevy_math::DVec4) by this matrix.
    #[ffi]
    fn mul_vec4(_self: R<DMat4>, v: R<DVec4>) -> V<DVec4> {
        V(_self.mul_vec4(*v))
    }

    /// Transforms a point in 3D, including translation and the projection divide.
    #[ffi]
    fn transform_point3(_self: R<DMat4>, point: R<DVec3>) -> V<DVec3> {
        V(_self.transform_point3(*point))
    }

    /// Transforms a direction in 3D, ignoring translation.
    #[ffi]
    fn transform_vector3(_self: R<DMat4>, vector: R<DVec3>) -> V<DVec3> {
        V(_self.transform_vector3(*vector))
    }

    /// Transforms a point in 3D by a projection matrix, including the perspective divide.
    #[ffi]
    fn project_point3(_self: R<DMat4>, point: R<DVec3>) -> V<DVec3> {
        V(_self.project_point3(*point))
    }

    /// Converts to a single precision matrix.
    #[ffi]
    fn as_mat4(_self: R<DMat4>) -> V<Mat4> {
        V(_self.as_mat4())
    }
}

/// Registers the FFI forms of the operators of the matrix `T`: one for each way their `Union` parameter is passed,
/// a matrix, a vector or a number.
fn register_operator_ffi<T, V, S>(world: &mut World)
where
    T: Copy + 'static + Mul<Output = T> + Mul<V, Output = V> + Mul<S, Output = T>,
    V: Copy + 'static,
    S: FfiArg,
{
    let mut registry = world
        .get_resource_or_init::<AppScriptFunctionRegistry>()
        .0
        .clone();
    let mut registry = registry.write();
    let ty = TypeId::of::<T>();
    registry.register_ffi(ty, "mul", |a: R<T>, b: R<T>| V(*a * *b));
    registry.register_ffi(ty, "mul", |a: R<T>, b: R<V>| V(*a * *b));
    registry.register_ffi(ty, "mul", |a: R<T>, b: S| V(*a * b));
}

/// Registers the functions of matrices.
pub(crate) fn register(world: &mut World) {
    register_mat2_shared_functions(world);
    register_mat3_shared_functions(world);
    register_mat4_shared_functions(world);
    register_dmat2_shared_functions(world);
    register_dmat3_shared_functions(world);
    register_dmat4_shared_functions(world);
    register_mat2_functions(world);
    register_mat3_functions(world);
    register_mat4_functions(world);
    register_mat3a_functions(world);
    register_dmat2_functions(world);
    register_dmat3_functions(world);
    register_dmat4_functions(world);
    register_operator_ffi::<Mat2, Vec2, f32>(world);
    register_operator_ffi::<Mat3, Vec3, f32>(world);
    register_operator_ffi::<Mat4, Vec4, f32>(world);
    register_operator_ffi::<DMat2, DVec2, f64>(world);
    register_operator_ffi::<DMat3, DVec3, f64>(world);
    register_operator_ffi::<DMat4, DVec4, f64>(world);

    // stored by the languages as values (LuaJIT: cdata): the columns are read as read-only copies and written whole
    assert_ffi_layout!(Mat2 {
        x_axis: Vec2,
        y_axis: Vec2
    });
    assert_ffi_layout!(Mat3 {
        x_axis: Vec3,
        y_axis: Vec3,
        z_axis: Vec3
    });
    assert_ffi_layout!(Mat4 {
        x_axis: Vec4,
        y_axis: Vec4,
        z_axis: Vec4,
        w_axis: Vec4
    });
    assert_ffi_layout!(Mat3A {
        x_axis: Vec3A,
        y_axis: Vec3A,
        z_axis: Vec3A
    });
    assert_ffi_layout!(DMat2 {
        x_axis: DVec2,
        y_axis: DVec2
    });
    assert_ffi_layout!(DMat3 {
        x_axis: DVec3,
        y_axis: DVec3,
        z_axis: DVec3
    });
    assert_ffi_layout!(DMat4 {
        x_axis: DVec4,
        y_axis: DVec4,
        z_axis: DVec4,
        w_axis: DVec4
    });
    // Safety: the columns are at their offsets, matrices have no invalid values
    unsafe {
        NativeType::of::<Mat3>()
            .value_field("x_axis", offset_of!(Mat3, x_axis), |m| &m.x_axis)
            .value_field("y_axis", offset_of!(Mat3, y_axis), |m| &m.y_axis)
            .value_field("z_axis", offset_of!(Mat3, z_axis), |m| &m.z_axis)
            .ffi()
            .register(world);
        NativeType::of::<Mat4>()
            .value_field("x_axis", offset_of!(Mat4, x_axis), |m| &m.x_axis)
            .value_field("y_axis", offset_of!(Mat4, y_axis), |m| &m.y_axis)
            .value_field("z_axis", offset_of!(Mat4, z_axis), |m| &m.z_axis)
            .value_field("w_axis", offset_of!(Mat4, w_axis), |m| &m.w_axis)
            .ffi()
            .register(world);
        NativeType::of::<DMat2>()
            .value_field("x_axis", offset_of!(DMat2, x_axis), |m| &m.x_axis)
            .value_field("y_axis", offset_of!(DMat2, y_axis), |m| &m.y_axis)
            .ffi()
            .register(world);
        NativeType::of::<DMat3>()
            .value_field("x_axis", offset_of!(DMat3, x_axis), |m| &m.x_axis)
            .value_field("y_axis", offset_of!(DMat3, y_axis), |m| &m.y_axis)
            .value_field("z_axis", offset_of!(DMat3, z_axis), |m| &m.z_axis)
            .ffi()
            .register(world);
        NativeType::of::<DMat4>()
            .value_field("x_axis", offset_of!(DMat4, x_axis), |m| &m.x_axis)
            .value_field("y_axis", offset_of!(DMat4, y_axis), |m| &m.y_axis)
            .value_field("z_axis", offset_of!(DMat4, z_axis), |m| &m.z_axis)
            .value_field("w_axis", offset_of!(DMat4, w_axis), |m| &m.w_axis)
            .ffi()
            .register(world);
    }
    // SIMD vectors: fields through `Deref`
    // Safety: glam's `Deref` casts the matrix itself to a `repr(C)` struct of its components, at the same offset in every
    // value, and matrices have no invalid values
    unsafe {
        NativeType::of::<Mat2>()
            .deref_value_field("x_axis", |m| &m.x_axis)
            .deref_value_field("y_axis", |m| &m.y_axis)
            .ffi()
            .register(world);
        NativeType::of::<Mat3A>()
            .deref_value_field("x_axis", |m| &m.x_axis)
            .deref_value_field("y_axis", |m| &m.y_axis)
            .deref_value_field("z_axis", |m| &m.z_axis)
            .ffi()
            .register(world);
    }
}
