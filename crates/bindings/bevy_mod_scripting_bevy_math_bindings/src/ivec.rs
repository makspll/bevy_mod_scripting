//! Integer vectors: [`IVec2`], [`IVec3`], [`UVec2`] and [`UVec3`].

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{IVec2, IVec3, UVec2, UVec3, Vec2, Vec3};
use bevy_mod_scripting_bindings::{
    InteropError,
    function::from::{R, Union, V},
};
use bevy_mod_scripting_derive::script_bindings;

/// Functions shared by all integer vectors.
// rustfmt keeps indenting the attribute inside the macro further on every run
#[rustfmt::skip]
macro_rules! int_vector_functions {
    ($ty:ty, $scalar:ty, $name:literal) => {
        #[script_bindings(
            remote,
            name = $name,
            bms_bindings_path = "bevy_mod_scripting_bindings",
            namespace = "math",
            core
        )]
        impl $ty {
            /// A vector with all components set to `v`.
            fn splat(v: $scalar) -> V<$ty> {
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
            fn add(_self: R<$ty>, rhs: Union<R<$ty>, $scalar>) -> V<$ty> {
                V(match rhs.into_left() {
                    Ok(v) => *_self + *v,
                    Err(s) => *_self + s,
                })
            }

            /// Subtracts a vector or a number from every component.
            fn sub(_self: R<$ty>, rhs: Union<R<$ty>, $scalar>) -> V<$ty> {
                V(match rhs.into_left() {
                    Ok(v) => *_self - *v,
                    Err(s) => *_self - s,
                })
            }

            /// Multiplies component-wise by a vector, or scales by a number.
            fn mul(_self: R<$ty>, rhs: Union<R<$ty>, $scalar>) -> V<$ty> {
                V(match rhs.into_left() {
                    Ok(v) => *_self * *v,
                    Err(s) => *_self * s,
                })
            }

            /// Divides component-wise by a vector, or by a number. Errors if any divisor component is `0`.
            fn div(_self: R<$ty>, rhs: Union<R<$ty>, $scalar>) -> Result<V<$ty>, InteropError> {
                match rhs.into_left() {
                    Ok(v) => {
                        if v.cmpeq(<$ty>::ZERO).any() {
                            return Err(InteropError::str("division by zero"));
                        }
                        Ok(V(*_self / *v))
                    }
                    Err(s) => {
                        if s == 0 {
                            return Err(InteropError::str("division by zero"));
                        }
                        Ok(V(*_self / s))
                    }
                }
            }

            /// The component-wise remainder of dividing by a vector, or by a number. Errors if any divisor component is `0`.
            fn rem(_self: R<$ty>, rhs: Union<R<$ty>, $scalar>) -> Result<V<$ty>, InteropError> {
                match rhs.into_left() {
                    Ok(v) => {
                        if v.cmpeq(<$ty>::ZERO).any() {
                            return Err(InteropError::str("division by zero"));
                        }
                        Ok(V(*_self % *v))
                    }
                    Err(s) => {
                        if s == 0 {
                            return Err(InteropError::str("division by zero"));
                        }
                        Ok(V(*_self % s))
                    }
                }
            }

            /// Whether all components are equal.
            fn eq(_self: R<$ty>, other: R<$ty>) -> bool {
                *_self == *other
            }

            /// The squared length of the vector.
            fn length_squared(_self: R<$ty>) -> $scalar {
                _self.length_squared()
            }

            /// The dot product of two vectors.
            fn dot(_self: R<$ty>, other: R<$ty>) -> $scalar {
                _self.dot(*other)
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

            /// The smallest component.
            fn min_element(_self: R<$ty>) -> $scalar {
                _self.min_element()
            }

            /// The largest component.
            fn max_element(_self: R<$ty>) -> $scalar {
                _self.max_element()
            }

            /// The components as a list.
            fn to_array(_self: R<$ty>) -> Vec<$scalar> {
                _self.to_array().to_vec()
            }
        }
    };
}

int_vector_functions!(IVec2, i32, "ivec2_shared_functions");
int_vector_functions!(IVec3, i32, "ivec3_shared_functions");
int_vector_functions!(UVec2, u32, "uvec2_shared_functions");
int_vector_functions!(UVec3, u32, "uvec3_shared_functions");

#[script_bindings(
    remote,
    name = "ivec2_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl IVec2 {
    /// Creates a new vector.
    fn new(x: i32, y: i32) -> V<IVec2> {
        V(IVec2::new(x, y))
    }

    /// The unit vector along the x axis.
    fn unit_x() -> V<IVec2> {
        V(IVec2::X)
    }

    /// The unit vector along the y axis.
    fn unit_y() -> V<IVec2> {
        V(IVec2::Y)
    }

    /// Negates every component.
    fn neg(_self: R<IVec2>) -> V<IVec2> {
        V(-*_self)
    }

    /// The absolute value of every component.
    fn abs(_self: R<IVec2>) -> V<IVec2> {
        V(_self.abs())
    }

    /// Converts to a float vector.
    fn as_vec2(_self: R<IVec2>) -> V<Vec2> {
        V(_self.as_vec2())
    }
}

#[script_bindings(
    remote,
    name = "ivec3_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl IVec3 {
    /// Creates a new vector.
    fn new(x: i32, y: i32, z: i32) -> V<IVec3> {
        V(IVec3::new(x, y, z))
    }

    /// The unit vector along the x axis.
    fn unit_x() -> V<IVec3> {
        V(IVec3::X)
    }

    /// The unit vector along the y axis.
    fn unit_y() -> V<IVec3> {
        V(IVec3::Y)
    }

    /// The unit vector along the z axis.
    fn unit_z() -> V<IVec3> {
        V(IVec3::Z)
    }

    /// Negates every component.
    fn neg(_self: R<IVec3>) -> V<IVec3> {
        V(-*_self)
    }

    /// The absolute value of every component.
    fn abs(_self: R<IVec3>) -> V<IVec3> {
        V(_self.abs())
    }

    /// Converts to a float vector.
    fn as_vec3(_self: R<IVec3>) -> V<Vec3> {
        V(_self.as_vec3())
    }
}

#[script_bindings(
    remote,
    name = "uvec2_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl UVec2 {
    /// Creates a new vector.
    fn new(x: u32, y: u32) -> V<UVec2> {
        V(UVec2::new(x, y))
    }

    /// The unit vector along the x axis.
    fn unit_x() -> V<UVec2> {
        V(UVec2::X)
    }

    /// The unit vector along the y axis.
    fn unit_y() -> V<UVec2> {
        V(UVec2::Y)
    }

    /// Converts to a float vector.
    fn as_vec2(_self: R<UVec2>) -> V<Vec2> {
        V(_self.as_vec2())
    }
}

#[script_bindings(
    remote,
    name = "uvec3_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl UVec3 {
    /// Creates a new vector.
    fn new(x: u32, y: u32, z: u32) -> V<UVec3> {
        V(UVec3::new(x, y, z))
    }

    /// The unit vector along the x axis.
    fn unit_x() -> V<UVec3> {
        V(UVec3::X)
    }

    /// The unit vector along the y axis.
    fn unit_y() -> V<UVec3> {
        V(UVec3::Y)
    }

    /// The unit vector along the z axis.
    fn unit_z() -> V<UVec3> {
        V(UVec3::Z)
    }

    /// Converts to a float vector.
    fn as_vec3(_self: R<UVec3>) -> V<Vec3> {
        V(_self.as_vec3())
    }
}

/// Registers the functions of all integer vectors.
pub(crate) fn register(world: &mut World) {
    register_ivec2_shared_functions(world);
    register_ivec3_shared_functions(world);
    register_uvec2_shared_functions(world);
    register_uvec3_shared_functions(world);
    register_ivec2_functions(world);
    register_ivec3_functions(world);
    register_uvec2_functions(world);
    register_uvec3_functions(world);
}
