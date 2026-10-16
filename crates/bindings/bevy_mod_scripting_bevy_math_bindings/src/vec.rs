//! Float vectors: [`Vec2`], [`Vec3`] and [`Vec4`], plus conversions for [`Vec3A`], [`DVec2`] and [`DVec3`].

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{DVec2, DVec3, IVec2, IVec3, UVec2, UVec3, Vec2, Vec3, Vec3A, Vec4};
use bevy_mod_scripting_bindings::{
    InteropError,
    function::from::{R, Union, V},
};
use bevy_mod_scripting_derive::script_bindings;

/// Functions shared by all float vectors.
// rustfmt keeps indenting the attribute inside the macro further on every run
#[rustfmt::skip]
macro_rules! float_vector_functions {
    ($ty:ty, $name:literal) => {
        #[script_bindings(
            remote,
            name = $name,
            bms_bindings_path = "bevy_mod_scripting_bindings",
            core
        )]
        impl $ty {
            /// A vector with all components set to `v`.
            fn splat(v: f32) -> V<$ty> {
                V(<$ty>::splat(v))
            }

            /// A vector with all components set to `0`.
            fn zero() -> V<$ty> {
                V(<$ty>::ZERO)
            }

            /// A vector with all components set to `1`.
            fn one() -> V<$ty> {
                V(<$ty>::ONE)
            }

            /// Adds a vector or a number to every component.
            fn add(_self: R<$ty>, rhs: Union<R<$ty>, f32>) -> V<$ty> {
                V(match rhs.into_left() {
                    Ok(v) => *_self + *v,
                    Err(s) => *_self + s,
                })
            }

            /// Subtracts a vector or a number from every component.
            fn sub(_self: R<$ty>, rhs: Union<R<$ty>, f32>) -> V<$ty> {
                V(match rhs.into_left() {
                    Ok(v) => *_self - *v,
                    Err(s) => *_self - s,
                })
            }

            /// Multiplies component-wise by a vector, or scales by a number.
            fn mul(_self: R<$ty>, rhs: Union<R<$ty>, f32>) -> V<$ty> {
                V(match rhs.into_left() {
                    Ok(v) => *_self * *v,
                    Err(s) => *_self * s,
                })
            }

            /// Divides component-wise by a vector, or by a number.
            fn div(_self: R<$ty>, rhs: Union<R<$ty>, f32>) -> V<$ty> {
                V(match rhs.into_left() {
                    Ok(v) => *_self / *v,
                    Err(s) => *_self / s,
                })
            }

            /// The component-wise remainder of dividing by a vector, or by a number.
            fn rem(_self: R<$ty>, rhs: Union<R<$ty>, f32>) -> V<$ty> {
                V(match rhs.into_left() {
                    Ok(v) => *_self % *v,
                    Err(s) => *_self % s,
                })
            }

            /// Subtracts every component from a number, for `n - v`.
            fn rsub(_self: R<$ty>, lhs: f32) -> V<$ty> {
                V(lhs - *_self)
            }

            /// Divides a number by every component, for `n / v`.
            fn rdiv(_self: R<$ty>, lhs: f32) -> V<$ty> {
                V(lhs / *_self)
            }

            /// Negates every component.
            fn neg(_self: R<$ty>) -> V<$ty> {
                V(-*_self)
            }

            /// Whether all components are equal.
            fn eq(_self: R<$ty>, other: R<$ty>) -> bool {
                *_self == *other
            }

            /// The length of the vector.
            fn length(_self: R<$ty>) -> f32 {
                _self.length()
            }

            /// The squared length of the vector, faster than [`length`](Self::length) when comparing lengths.
            fn length_squared(_self: R<$ty>) -> f32 {
                _self.length_squared()
            }

            /// The distance between two points.
            fn distance(_self: R<$ty>, other: R<$ty>) -> f32 {
                _self.distance(*other)
            }

            /// The squared distance between two points.
            fn distance_squared(_self: R<$ty>, other: R<$ty>) -> f32 {
                _self.distance_squared(*other)
            }

            /// The dot product of two vectors.
            fn dot(_self: R<$ty>, other: R<$ty>) -> f32 {
                _self.dot(*other)
            }

            /// The vector scaled to a length of `1`. Errors if the vector is zero, or not finite.
            fn normalize(_self: R<$ty>) -> Result<V<$ty>, InteropError> {
                _self.try_normalize().map(V).ok_or_else(|| {
                    InteropError::str("cannot normalize a vector of zero or non-finite length")
                })
            }

            /// The vector scaled to a length of `1`, or zero if that's not possible.
            fn normalize_or_zero(_self: R<$ty>) -> V<$ty> {
                V(_self.normalize_or_zero())
            }

            /// The vector scaled to a length of `1`, or nil if that's not possible.
            fn try_normalize(_self: R<$ty>) -> Option<V<$ty>> {
                _self.try_normalize().map(V)
            }

            /// Linearly interpolates towards `other`, `t = 0` gives this vector and `t = 1` gives `other`.
            fn lerp(_self: R<$ty>, other: R<$ty>, t: f32) -> V<$ty> {
                V(_self.lerp(*other, t))
            }

            /// Moves towards `target` by at most `max_distance`, without overshooting.
            fn move_towards(_self: R<$ty>, target: R<$ty>, max_distance: f32) -> V<$ty> {
                V(_self.move_towards(*target, max_distance))
            }

            /// The absolute value of every component.
            fn abs(_self: R<$ty>) -> V<$ty> {
                V(_self.abs())
            }

            /// Rounds every component down.
            fn floor(_self: R<$ty>) -> V<$ty> {
                V(_self.floor())
            }

            /// Rounds every component up.
            fn ceil(_self: R<$ty>) -> V<$ty> {
                V(_self.ceil())
            }

            /// Rounds every component to the nearest integer.
            fn round(_self: R<$ty>) -> V<$ty> {
                V(_self.round())
            }

            /// The fractional part of every component, i.e. `x - floor(x)`.
            fn fract(_self: R<$ty>) -> V<$ty> {
                V(_self.fract_gl())
            }

            /// The sign of every component: `1`, `-1`, or NaN.
            fn signum(_self: R<$ty>) -> V<$ty> {
                V(_self.signum())
            }

            /// The smaller of each pair of components.
            fn min(_self: R<$ty>, other: R<$ty>) -> V<$ty> {
                V(_self.min(*other))
            }

            /// The larger of each pair of components.
            fn max(_self: R<$ty>, other: R<$ty>) -> V<$ty> {
                V(_self.max(*other))
            }

            /// Clamps every component between the components of `min` and `max`.
            fn clamp(_self: R<$ty>, min: R<$ty>, max: R<$ty>) -> V<$ty> {
                V(_self.clamp(*min, *max))
            }

            /// Scales the vector so that its length is between `min` and `max`.
            fn clamp_length(_self: R<$ty>, min: f32, max: f32) -> V<$ty> {
                V(_self.clamp_length(min, max))
            }

            /// Scales the vector down so that its length is at most `max`.
            fn clamp_length_max(_self: R<$ty>, max: f32) -> V<$ty> {
                V(_self.clamp_length_max(max))
            }

            /// The projection of this vector onto `other`.
            fn project_onto(_self: R<$ty>, other: R<$ty>) -> V<$ty> {
                V(_self.project_onto(*other))
            }

            /// The part of this vector perpendicular to `other`.
            fn reject_from(_self: R<$ty>, other: R<$ty>) -> V<$ty> {
                V(_self.reject_from(*other))
            }

            /// Reflects this vector off a surface with the given `normal`, which must be of length `1`.
            fn reflect(_self: R<$ty>, normal: R<$ty>) -> V<$ty> {
                V(_self.reflect(*normal))
            }

            /// The smallest component.
            fn min_element(_self: R<$ty>) -> f32 {
                _self.min_element()
            }

            /// The largest component.
            fn max_element(_self: R<$ty>) -> f32 {
                _self.max_element()
            }

            /// The components as a list.
            fn to_array(_self: R<$ty>) -> Vec<f32> {
                _self.to_array().to_vec()
            }
        }
    };
}

float_vector_functions!(Vec2, "vec2_shared_functions");
float_vector_functions!(Vec3, "vec3_shared_functions");
float_vector_functions!(Vec4, "vec4_shared_functions");

#[script_bindings(
    remote,
    name = "vec2_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Vec2 {
    /// Creates a new vector.
    ///
    /// ```multi-lang-example,lua
    /// local v = Vec2.new(3, 4)
    /// assert(v:length() == 5)
    /// ```
    ///
    /// ```multi-lang-example,rhai
    /// let v = Vec2.new_.call(3.0, 4.0);
    /// assert(v.length.call() == 5.0);
    /// ```
    fn new(x: f32, y: f32) -> V<Vec2> {
        V(Vec2::new(x, y))
    }

    /// The unit vector along the x axis.
    fn unit_x() -> V<Vec2> {
        V(Vec2::X)
    }

    /// The unit vector along the y axis.
    fn unit_y() -> V<Vec2> {
        V(Vec2::Y)
    }

    /// The unit vector of the given angle in radians, measured counterclockwise from the x axis.
    fn from_angle(angle: f32) -> V<Vec2> {
        V(Vec2::from_angle(angle))
    }

    /// The angle of this vector in radians, measured counterclockwise from the x axis.
    fn to_angle(_self: R<Vec2>) -> f32 {
        _self.to_angle()
    }

    /// The signed angle in radians to rotate this vector onto `other`.
    fn angle_to(_self: R<Vec2>, other: R<Vec2>) -> f32 {
        _self.angle_to(*other)
    }

    /// The vector rotated by 90 degrees counterclockwise.
    fn perp(_self: R<Vec2>) -> V<Vec2> {
        V(_self.perp())
    }

    /// The 2D cross product, positive if `other` is counterclockwise from this vector.
    fn perp_dot(_self: R<Vec2>, other: R<Vec2>) -> f32 {
        _self.perp_dot(*other)
    }

    /// Rotates `other` by the angle of this vector, and scales it by its length.
    fn rotate(_self: R<Vec2>, other: R<Vec2>) -> V<Vec2> {
        V(_self.rotate(*other))
    }

    /// A [`Vec3`](bevy_math::Vec3) with the given `z`.
    fn extend(_self: R<Vec2>, z: f32) -> V<Vec3> {
        V(_self.extend(z))
    }

    /// Converts to an integer vector, rounding towards zero.
    fn as_ivec2(_self: R<Vec2>) -> V<IVec2> {
        V(_self.as_ivec2())
    }

    /// Converts to an unsigned integer vector, rounding towards zero, negative values become `0`.
    fn as_uvec2(_self: R<Vec2>) -> V<UVec2> {
        V(_self.as_uvec2())
    }

    /// Converts to a double precision vector.
    fn as_dvec2(_self: R<Vec2>) -> V<DVec2> {
        V(_self.as_dvec2())
    }
}

#[script_bindings(
    remote,
    name = "vec3_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Vec3 {
    /// Creates a new vector.
    ///
    /// ```multi-lang-example,lua
    /// local v = Vec3.new(1, 2, 3)
    /// assert(v + v == Vec3.new(2, 4, 6))
    /// ```
    ///
    /// ```multi-lang-example,rhai
    /// let v = Vec3.new_.call(1.0, 2.0, 3.0);
    /// assert(v + v == Vec3.new_.call(2.0, 4.0, 6.0));
    /// ```
    fn new(x: f32, y: f32, z: f32) -> V<Vec3> {
        V(Vec3::new(x, y, z))
    }

    /// The unit vector along the x axis.
    fn unit_x() -> V<Vec3> {
        V(Vec3::X)
    }

    /// The unit vector along the y axis, which is up in bevy.
    fn unit_y() -> V<Vec3> {
        V(Vec3::Y)
    }

    /// The unit vector along the z axis.
    fn unit_z() -> V<Vec3> {
        V(Vec3::Z)
    }

    /// The cross product, perpendicular to both vectors.
    fn cross(_self: R<Vec3>, other: R<Vec3>) -> V<Vec3> {
        V(_self.cross(*other))
    }

    /// The angle in radians between two vectors, between `0` and `pi`.
    fn angle_between(_self: R<Vec3>, other: R<Vec3>) -> f32 {
        _self.angle_between(*other)
    }

    /// Some vector perpendicular to this one.
    fn any_orthogonal_vector(_self: R<Vec3>) -> V<Vec3> {
        V(_self.any_orthogonal_vector())
    }

    /// A [`Vec4`](bevy_math::Vec4) with the given `w`.
    fn extend(_self: R<Vec3>, w: f32) -> V<Vec4> {
        V(_self.extend(w))
    }

    /// A [`Vec2`](bevy_math::Vec2) of the `x` and `y` components.
    fn truncate(_self: R<Vec3>) -> V<Vec2> {
        V(_self.truncate())
    }

    /// Converts to an integer vector, rounding towards zero.
    fn as_ivec3(_self: R<Vec3>) -> V<IVec3> {
        V(_self.as_ivec3())
    }

    /// Converts to an unsigned integer vector, rounding towards zero, negative values become `0`.
    fn as_uvec3(_self: R<Vec3>) -> V<UVec3> {
        V(_self.as_uvec3())
    }

    /// Converts to a double precision vector.
    fn as_dvec3(_self: R<Vec3>) -> V<DVec3> {
        V(_self.as_dvec3())
    }

    /// Converts to a [`Vec3A`](bevy_math::Vec3A), used by some bevy types such as bounding boxes.
    fn to_vec3a(_self: R<Vec3>) -> V<Vec3A> {
        V(Vec3A::from(*_self))
    }
}

#[script_bindings(
    remote,
    name = "vec4_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Vec4 {
    /// Creates a new vector.
    fn new(x: f32, y: f32, z: f32, w: f32) -> V<Vec4> {
        V(Vec4::new(x, y, z, w))
    }

    /// A [`Vec3`](bevy_math::Vec3) of the `x`, `y` and `z` components.
    fn truncate(_self: R<Vec4>) -> V<Vec3> {
        V(_self.truncate())
    }
}

#[script_bindings(
    remote,
    name = "vec3a_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Vec3A {
    /// Creates a new vector.
    fn new(x: f32, y: f32, z: f32) -> V<Vec3A> {
        V(Vec3A::new(x, y, z))
    }

    /// Converts to a [`Vec3`](bevy_math::Vec3).
    fn to_vec3(_self: R<Vec3A>) -> V<Vec3> {
        V(Vec3::from(*_self))
    }
}

#[script_bindings(
    remote,
    name = "dvec2_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl DVec2 {
    /// Creates a new double precision vector.
    fn new(x: f64, y: f64) -> V<DVec2> {
        V(DVec2::new(x, y))
    }

    /// Converts to a single precision [`Vec2`](bevy_math::Vec2).
    fn as_vec2(_self: R<DVec2>) -> V<Vec2> {
        V(_self.as_vec2())
    }
}

#[script_bindings(
    remote,
    name = "dvec3_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl DVec3 {
    /// Creates a new double precision vector.
    fn new(x: f64, y: f64, z: f64) -> V<DVec3> {
        V(DVec3::new(x, y, z))
    }

    /// Converts to a single precision [`Vec3`](bevy_math::Vec3).
    fn as_vec3(_self: R<DVec3>) -> V<Vec3> {
        V(_self.as_vec3())
    }
}

/// Registers the functions of all float vectors.
pub(crate) fn register(world: &mut World) {
    register_vec2_shared_functions(world);
    register_vec3_shared_functions(world);
    register_vec4_shared_functions(world);
    register_vec2_functions(world);
    register_vec3_functions(world);
    register_vec4_functions(world);
    register_vec3a_functions(world);
    register_dvec2_functions(world);
    register_dvec3_functions(world);
}
