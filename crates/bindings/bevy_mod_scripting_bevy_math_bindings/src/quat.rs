//! Rotations as quaternions: [`Quat`] and [`DQuat`].

use std::{any::TypeId, mem::offset_of};

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{DMat3, DQuat, DVec3, EulerRot, Mat3, Quat, Vec3};
use bevy_mod_scripting_bindings::{
    InteropError,
    conversions::from::{R, Union, V},
    function::script_function::AppScriptFunctionRegistry,
    native::NativeType,
};
use bevy_mod_scripting_derive::script_bindings;

/// Normalizes a direction, erroring if it is zero or not finite.
fn direction_f(v: Vec3) -> Result<Vec3, InteropError> {
    v.try_normalize()
        .ok_or_else(|| InteropError::str("cannot use a zero or non-finite vector as a direction"))
}

/// Normalizes a direction, erroring if it is zero or not finite.
fn direction_d(v: DVec3) -> Result<DVec3, InteropError> {
    v.try_normalize()
        .ok_or_else(|| InteropError::str("cannot use a zero or non-finite vector as a direction"))
}

/// Functions shared by all quaternions.
// rustfmt keeps indenting the attribute inside the macro further on every run
#[rustfmt::skip]
macro_rules! quaternion_functions {
    ($quat:ty, $vec:ty, $scalar:ty, $name:literal) => {
        #[script_bindings(
            remote,
            name = $name,
            bms_bindings_path = "bevy_mod_scripting_bindings",
            namespace = "math",
            core
        )]
        impl $quat {
            /// The identity rotation, which leaves everything unchanged.
            #[ffi]
            fn identity() -> V<$quat> {
                V(<$quat>::IDENTITY)
            }

            /// A rotation of `angle` radians around the x axis.
            #[ffi]
            fn from_rotation_x(angle: $scalar) -> V<$quat> {
                V(<$quat>::from_rotation_x(angle))
            }

            /// A rotation of `angle` radians around the y axis.
            #[ffi]
            fn from_rotation_y(angle: $scalar) -> V<$quat> {
                V(<$quat>::from_rotation_y(angle))
            }

            /// A rotation of `angle` radians around the z axis.
            #[ffi]
            fn from_rotation_z(angle: $scalar) -> V<$quat> {
                V(<$quat>::from_rotation_z(angle))
            }

            /// A rotation from three angles in radians, applied in the given [`EulerRot`](bevy_math::EulerRot) order, i.e. `EulerRot.YXZ`.
            fn from_euler(order: R<EulerRot>, a: $scalar, b: $scalar, c: $scalar) -> V<$quat> {
                V(<$quat>::from_euler(*order, a, b, c))
            }

            /// The three angles in radians of this rotation, in the given [`EulerRot`](bevy_math::EulerRot) order.
            fn to_euler(_self: R<$quat>, order: R<EulerRot>) -> Vec<$scalar> {
                let (a, b, c) = _self.to_euler(*order);
                vec![a, b, c]
            }

            /// Composes two rotations when given a rotation, or rotates a vector when given a [`$vec`](bevy_math::$vec).
            fn mul(_self: R<$quat>, rhs: Union<R<$quat>, R<$vec>>) -> Union<V<$quat>, V<$vec>> {
                match rhs.into_left() {
                    Ok(q) => Union::new_left(V(*_self * *q)),
                    Err(v) => Union::new_right(V(*_self * *v)),
                }
            }

            /// Rotates a [`$vec`](bevy_math::$vec).
            #[ffi]
            fn mul_vec3(_self: R<$quat>, v: R<$vec>) -> V<$vec> {
                V(_self.mul_vec3(*v))
            }

            /// Whether both rotations have exactly the same components.
            #[ffi]
            fn eq(_self: R<$quat>, other: R<$quat>) -> bool {
                *_self == *other
            }

            /// The opposite rotation, with every component negated.
            #[ffi]
            fn neg(_self: R<$quat>) -> V<$quat> {
                V(-*_self)
            }

            /// The rotation scaled to a length of `1`.
            #[ffi]
            fn normalize(_self: R<$quat>) -> V<$quat> {
                V(_self.normalize())
            }

            /// The rotation that undoes this one.
            #[ffi]
            fn inverse(_self: R<$quat>) -> V<$quat> {
                V(_self.inverse())
            }

            /// The rotation with the vector part negated, which is the inverse for unit quaternions.
            #[ffi]
            fn conjugate(_self: R<$quat>) -> V<$quat> {
                V(_self.conjugate())
            }

            /// The length of the quaternion, which is `1` for a rotation.
            #[ffi]
            fn length(_self: R<$quat>) -> $scalar {
                _self.length()
            }

            /// The dot product of two rotations.
            #[ffi]
            fn dot(_self: R<$quat>, other: R<$quat>) -> $scalar {
                _self.dot(*other)
            }

            /// Spherically interpolates towards `other`, `t = 0` gives this rotation and `t = 1` gives `other`.
            #[ffi]
            fn slerp(_self: R<$quat>, other: R<$quat>, t: $scalar) -> V<$quat> {
                V(_self.slerp(*other, t))
            }

            /// Linearly interpolates towards `other`, `t = 0` gives this rotation and `t = 1` gives `other`.
            #[ffi]
            fn lerp(_self: R<$quat>, other: R<$quat>, t: $scalar) -> V<$quat> {
                V(_self.lerp(*other, t))
            }

            /// The angle in radians between two rotations.
            #[ffi]
            fn angle_between(_self: R<$quat>, other: R<$quat>) -> $scalar {
                _self.angle_between(*other)
            }

            /// The axis this rotation turns around.
            #[ffi]
            fn axis(_self: R<$quat>) -> V<$vec> {
                V(_self.to_axis_angle().0)
            }

            /// The angle in radians this rotation turns by around its [`axis`](Self::axis).
            #[ffi]
            fn angle(_self: R<$quat>) -> $scalar {
                _self.to_axis_angle().1
            }

            /// The vector part of the quaternion, i.e. the x, y and z components.
            #[ffi]
            fn xyz(_self: R<$quat>) -> V<$vec> {
                V(_self.xyz())
            }

            /// Whether the quaternion has a length of `1`.
            #[ffi]
            fn is_normalized(_self: R<$quat>) -> bool {
                _self.is_normalized()
            }
        }
    };
}

quaternion_functions!(Quat, Vec3, f32, "quat_shared_functions");
quaternion_functions!(DQuat, DVec3, f64, "dquat_shared_functions");

#[script_bindings(
    remote,
    name = "quat_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl Quat {
    /// A rotation of `angle` radians around `axis`. Errors if the axis is zero or not finite.
    #[ffi]
    fn from_axis_angle(axis: R<Vec3>, angle: f32) -> Result<V<Quat>, InteropError> {
        let axis = direction_f(*axis)?;
        Ok(V(Quat::from_axis_angle(axis, angle)))
    }

    /// The rotation that turns the direction `from` onto the direction `to`. Errors if either vector is zero or not finite.
    #[ffi]
    fn from_rotation_arc(from: R<Vec3>, to: R<Vec3>) -> Result<V<Quat>, InteropError> {
        let from = direction_f(*from)?;
        let to = direction_f(*to)?;
        Ok(V(Quat::from_rotation_arc(from, to)))
    }

    /// The rotation described by a 3x3 rotation matrix.
    #[ffi]
    fn from_mat3(m: R<Mat3>) -> V<Quat> {
        V(Quat::from_mat3(&m))
    }

    /// Converts to a double precision quaternion.
    #[ffi]
    fn as_dquat(_self: R<Quat>) -> V<DQuat> {
        V(_self.as_dquat())
    }
}

#[script_bindings(
    remote,
    name = "dquat_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl DQuat {
    /// A rotation of `angle` radians around `axis`. Errors if the axis is zero or not finite.
    #[ffi]
    fn from_axis_angle(axis: R<DVec3>, angle: f64) -> Result<V<DQuat>, InteropError> {
        let axis = direction_d(*axis)?;
        Ok(V(DQuat::from_axis_angle(axis, angle)))
    }

    /// The rotation that turns the direction `from` onto the direction `to`. Errors if either vector is zero or not finite.
    #[ffi]
    fn from_rotation_arc(from: R<DVec3>, to: R<DVec3>) -> Result<V<DQuat>, InteropError> {
        let from = direction_d(*from)?;
        let to = direction_d(*to)?;
        Ok(V(DQuat::from_rotation_arc(from, to)))
    }

    /// The rotation described by a 3x3 rotation matrix.
    #[ffi]
    fn from_mat3(m: R<DMat3>) -> V<DQuat> {
        V(DQuat::from_mat3(&m))
    }

    /// Converts to a single precision quaternion.
    #[ffi]
    fn as_quat(_self: R<DQuat>) -> V<Quat> {
        V(_self.as_quat())
    }
}

/// Registers the functions of quaternions, which are stored natively by the languages.
pub(crate) fn register(world: &mut World) {
    register_quat_shared_functions(world);
    register_dquat_shared_functions(world);
    register_quat_functions(world);
    register_dquat_functions(world);
    // `mul` takes a `Union`: an FFI form for each alternative
    {
        let mut registry = world
            .get_resource_or_init::<AppScriptFunctionRegistry>()
            .0
            .clone();
        let mut registry = registry.write();
        let quat = TypeId::of::<Quat>();
        registry.register_ffi(quat, "mul", |a: R<Quat>, b: R<Quat>| V(*a * *b));
        registry.register_ffi(quat, "mul", |a: R<Quat>, b: R<Vec3>| V(*a * *b));
        let dquat = TypeId::of::<Quat>();
        registry.register_ffi(dquat, "mul", |a: R<DQuat>, b: R<DQuat>| V(*a * *b));
        registry.register_ffi(dquat, "mul", |a: R<DQuat>, b: R<DVec3>| V(*a * *b));
    }
    bevy_mod_scripting_bindings::assert_ffi_layout!(Quat {
        x: f32,
        y: f32,
        z: f32,
        w: f32
    });
    bevy_mod_scripting_bindings::assert_ffi_layout!(DQuat {
        x: f64,
        y: f64,
        z: f64,
        w: f64
    });
    // Safety: the fields are at their offsets, any bits are a quaternion: functions needing a normalised one check it
    unsafe {
        NativeType::of::<DQuat>()
            .field("x", offset_of!(DQuat, x), |v| &v.x)
            .field("y", offset_of!(DQuat, y), |v| &v.y)
            .field("z", offset_of!(DQuat, z), |v| &v.z)
            .field("w", offset_of!(DQuat, w), |v| &v.w)
            .ffi()
            .register(world);
    }
    // a SIMD type: fields through `Deref`
    // Safety: glam's `Deref` casts the quaternion itself to a `repr(C)` struct of its components, at the same offset in
    // every value. Any bits are a quaternion: functions needing a normalised one check it.
    unsafe {
        NativeType::of::<Quat>()
            .deref_field("x", |q| &q.x)
            .deref_field("y", |q| &q.y)
            .deref_field("z", |q| &q.z)
            .deref_field("w", |q| &q.w)
            .ffi()
            .register(world);
    }
}
