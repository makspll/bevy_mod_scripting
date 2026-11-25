//! Affine transforms: [`Affine2`], [`Affine3`], [`DAffine2`] and [`DAffine3`], plus conversions for [`Affine3A`].

use std::{any::TypeId, mem::offset_of};

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{
    Affine2, Affine3, Affine3A, DAffine2, DAffine3, DMat2, DMat3, DQuat, DVec2, DVec3, Mat2, Mat3,
    Mat3A, Quat, Vec2, Vec3, Vec3A,
};
use bevy_mod_scripting_bindings::{
    InteropError, assert_ffi_layout,
    conversions::from::{R, V},
    function::script_function::AppScriptFunctionRegistry,
    native::NativeType,
};
use bevy_mod_scripting_derive::script_bindings;

/// Functions shared by all affine transforms.
// rustfmt keeps indenting the attribute inside the macro further on every run
#[rustfmt::skip]
macro_rules! affine_functions {
    ($affine:ty, $vec:ty, $scalar:ty, $name:literal) => {
        #[script_bindings(
            remote,
            name = $name,
            bms_bindings_path = "bevy_mod_scripting_bindings",
            namespace = "math",
            core
        )]
        impl $affine {
            /// The identity transform, which leaves vectors unchanged.
            #[ffi]
            fn identity() -> V<$affine> {
                V(<$affine>::IDENTITY)
            }

            /// Creates an affine transform that changes scale.
            #[ffi]
            fn from_scale(scale: R<$vec>) -> V<$affine> {
                V(<$affine>::from_scale(*scale))
            }

            /// Creates an affine transform from the given translation.
            #[ffi]
            fn from_translation(translation: R<$vec>) -> V<$affine> {
                V(<$affine>::from_translation(*translation))
            }

            /// The affine transform that undoes this one.
            #[ffi]
            fn inverse(_self: R<$affine>) -> V<$affine> {
                V(_self.inverse())
            }

            /// Multiplies by a transform.
            #[ffi]
            fn mul(_self: R<$affine>, rhs: R<$affine>) -> V<$affine> {
                V(*_self * *rhs)
            }

            /// Whether both transforms have exactly the same components.
            #[ffi]
            fn eq(_self: R<$affine>, other: R<$affine>) -> bool {
                *_self == *other
            }
        }
    };
}

affine_functions!(Affine2, Vec2, f32, "affine2_shared_functions");
affine_functions!(Affine3, Vec3, f32, "affine3_shared_functions");
affine_functions!(DAffine2, DVec2, f64, "daffine2_shared_functions");
affine_functions!(DAffine3, DVec3, f64, "daffine3_shared_functions");

#[script_bindings(
    remote,
    name = "affine2_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl Affine2 {
    /// Creates an affine transform from the given rotation angle.
    #[ffi]
    fn from_angle(angle: f32) -> V<Affine2> {
        V(Affine2::from_angle(angle))
    }

    /// Creates an affine transform from a 2x2 matrix.
    #[ffi]
    fn from_mat2(matrix2: R<Mat2>) -> V<Affine2> {
        V(Affine2::from_mat2(*matrix2))
    }

    /// Transforms the given 2D point, applying shear, scale, rotation and translation.
    #[ffi]
    fn transform_point2(_self: R<Affine2>, rhs: R<Vec2>) -> V<Vec2> {
        V(_self.transform_point2(*rhs))
    }

    /// Transforms the given 2D vector, applying shear, scale and rotation (but NOT
    /// translation).
    ///
    /// To also apply translation, use [`Self::transform_point2()`] instead.
    #[ffi]
    fn transform_vector2(_self: R<Affine2>, rhs: R<Vec2>) -> V<Vec2> {
        V(_self.transform_vector2(*rhs))
    }

    /// Converts to a double precision transform.
    #[ffi]
    fn as_daffine2(_self: R<Affine2>) -> V<DAffine2> {
        V(_self.as_daffine2())
    }
}

#[script_bindings(
    remote,
    name = "affine3_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl Affine3 {
    /// Creates an affine transform from the given rotation quaternion.
    #[ffi]
    fn from_quat(rotation: R<Quat>) -> V<Affine3> {
        V(Affine3::from_quat(*rotation))
    }

    /// Creates an affine transform containing a 3D rotation around the x axis of
    /// `angle` (in radians).
    #[ffi]
    fn from_rotation_x(angle: f32) -> V<Affine3> {
        V(Affine3::from_rotation_x(angle))
    }

    /// Creates an affine transform containing a 3D rotation around the y axis of
    /// `angle` (in radians).
    #[ffi]
    fn from_rotation_y(angle: f32) -> V<Affine3> {
        V(Affine3::from_rotation_y(angle))
    }

    /// Creates an affine transform containing a 3D rotation around the z axis of
    /// `angle` (in radians).
    #[ffi]
    fn from_rotation_z(angle: f32) -> V<Affine3> {
        V(Affine3::from_rotation_z(angle))
    }

    /// Creates an affine transform from a 3x3 matrix.
    #[ffi]
    fn from_mat3(matrix3: R<Mat3>) -> V<Affine3> {
        V(Affine3::from_mat3(*matrix3))
    }

    /// Transforms the given 3D point, applying shear, scale, rotation and translation.
    #[ffi]
    fn transform_point3(_self: R<Affine3>, rhs: R<Vec3>) -> V<Vec3> {
        V(_self.transform_point3(*rhs))
    }

    /// Transforms the given 3D vector, applying shear, scale and rotation (but NOT
    /// translation).
    ///
    /// To also apply translation, use [`Self::transform_point3()`] instead.
    #[ffi]
    fn transform_vector3(_self: R<Affine3>, rhs: R<Vec3>) -> V<Vec3> {
        V(_self.transform_vector3(*rhs))
    }

    /// Converts to a double precision transform.
    #[ffi]
    fn as_daffine3(_self: R<Affine3>) -> V<DAffine3> {
        V(_self.as_daffine3())
    }

    /// Converts to an [`Affine3A`](bevy_math::Affine3A).
    #[ffi]
    fn to_affine3a(_self: R<Affine3>) -> V<Affine3A> {
        V(Affine3A::from(*_self))
    }
}

#[script_bindings(
    remote,
    name = "affine3a_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl Affine3A {
    /// Converts to an [`Affine3`](bevy_math::Affine3).
    #[ffi]
    fn to_affine3(_self: R<Affine3A>) -> V<Affine3> {
        V(Affine3::from(*_self))
    }
}

#[script_bindings(
    remote,
    name = "daffine2_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl DAffine2 {
    /// Creates an affine transform from the given rotation angle.
    #[ffi]
    fn from_angle(angle: f64) -> V<DAffine2> {
        V(DAffine2::from_angle(angle))
    }

    /// Creates an affine transform from a 2x2 matrix.
    #[ffi]
    fn from_mat2(matrix2: R<DMat2>) -> V<DAffine2> {
        V(DAffine2::from_mat2(*matrix2))
    }

    /// Transforms the given 2D point, applying shear, scale, rotation and translation.
    #[ffi]
    fn transform_point2(_self: R<DAffine2>, rhs: R<DVec2>) -> V<DVec2> {
        V(_self.transform_point2(*rhs))
    }

    /// Transforms the given 2D vector, applying shear, scale and rotation (but NOT
    /// translation).
    ///
    /// To also apply translation, use [`Self::transform_point2()`] instead.
    #[ffi]
    fn transform_vector2(_self: R<DAffine2>, rhs: R<DVec2>) -> V<DVec2> {
        V(_self.transform_vector2(*rhs))
    }

    /// Converts to a single precision transform.
    #[ffi]
    fn as_affine2(_self: R<DAffine2>) -> V<Affine2> {
        V(_self.as_affine2())
    }
}

#[script_bindings(
    remote,
    name = "daffine3_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl DAffine3 {
    /// Creates an affine transform from the given rotation quaternion.
    #[ffi]
    fn from_quat(rotation: R<DQuat>) -> V<DAffine3> {
        V(DAffine3::from_quat(*rotation))
    }

    /// Creates an affine transform containing a 3D rotation around the x axis of
    /// `angle` (in radians).
    #[ffi]
    fn from_rotation_x(angle: f64) -> V<DAffine3> {
        V(DAffine3::from_rotation_x(angle))
    }

    /// Creates an affine transform containing a 3D rotation around the y axis of
    /// `angle` (in radians).
    #[ffi]
    fn from_rotation_y(angle: f64) -> V<DAffine3> {
        V(DAffine3::from_rotation_y(angle))
    }

    /// Creates an affine transform containing a 3D rotation around the z axis of
    /// `angle` (in radians).
    #[ffi]
    fn from_rotation_z(angle: f64) -> V<DAffine3> {
        V(DAffine3::from_rotation_z(angle))
    }

    /// Creates an affine transform from a 3x3 matrix.
    #[ffi]
    fn from_mat3(matrix3: R<DMat3>) -> V<DAffine3> {
        V(DAffine3::from_mat3(*matrix3))
    }

    /// Transforms the given 3D point, applying shear, scale, rotation and translation.
    #[ffi]
    fn transform_point3(_self: R<DAffine3>, rhs: R<DVec3>) -> V<DVec3> {
        V(_self.transform_point3(*rhs))
    }

    /// Transforms the given 3D vector, applying shear, scale and rotation (but NOT
    /// translation).
    ///
    /// To also apply translation, use [`Self::transform_point3()`] instead.
    #[ffi]
    fn transform_vector3(_self: R<DAffine3>, rhs: R<DVec3>) -> V<DVec3> {
        V(_self.transform_vector3(*rhs))
    }

    /// Converts to a single precision transform.
    #[ffi]
    fn as_affine3(_self: R<DAffine3>) -> V<Affine3> {
        V(_self.as_affine3())
    }
}

/// Registers the functions of affine transforms.
pub(crate) fn register(world: &mut World) {
    register_affine2_shared_functions(world);
    register_affine3_shared_functions(world);
    register_daffine2_shared_functions(world);
    register_daffine3_shared_functions(world);
    register_affine2_functions(world);
    register_affine3_functions(world);
    register_affine3a_functions(world);
    register_daffine2_functions(world);
    register_daffine3_functions(world);

    assert_ffi_layout!(Affine2 {
        matrix2: Mat2,
        translation: Vec2
    });
    assert_ffi_layout!(Affine3 {
        matrix3: Mat3,
        translation: Vec3
    });
    assert_ffi_layout!(Affine3A {
        matrix3: Mat3A,
        translation: Vec3A
    });
    assert_ffi_layout!(DAffine2 {
        matrix2: DMat2,
        translation: DVec2
    });
    assert_ffi_layout!(DAffine3 {
        matrix3: DMat3,
        translation: DVec3
    });
    // Safety: transforms have no invalid values
    unsafe {
        NativeType::of::<Affine2>()
            .value_field("matrix2", offset_of!(Affine2, matrix2), |m| &m.matrix2)
            .value_field("translation", offset_of!(Affine2, translation), |m| {
                &m.translation
            })
            .ffi()
            .register(world);
        NativeType::of::<Affine3>()
            .value_field("matrix3", offset_of!(Affine3, matrix3), |m| &m.matrix3)
            .value_field("translation", offset_of!(Affine3, translation), |m| {
                &m.translation
            })
            .ffi()
            .register(world);
        NativeType::of::<Affine3A>()
            .value_field("matrix3", offset_of!(Affine3A, matrix3), |m| &m.matrix3)
            .value_field("translation", offset_of!(Affine3A, translation), |m| {
                &m.translation
            })
            .ffi()
            .register(world);
        NativeType::of::<DAffine2>()
            .value_field("matrix2", offset_of!(DAffine2, matrix2), |m| &m.matrix2)
            .value_field("translation", offset_of!(DAffine2, translation), |m| {
                &m.translation
            })
            .ffi()
            .register(world);
        NativeType::of::<DAffine3>()
            .value_field("matrix3", offset_of!(DAffine3, matrix3), |m| &m.matrix3)
            .value_field("translation", offset_of!(DAffine3, translation), |m| {
                &m.translation
            })
            .ffi()
            .register(world);
    }
}
