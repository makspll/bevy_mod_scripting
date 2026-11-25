//! Integer vectors: [`IVec2`], [`IVec3`], [`IVec4`], [`UVec2`], [`UVec3`] and [`UVec4`].

use std::{
    any::TypeId,
    mem::offset_of,
    ops::{Add, Div, Mul, Rem, Sub},
};

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};
use bevy_mod_scripting_bindings::{
    InteropError, assert_ffi_layout,
    conversions::from::{R, Union, V},
    function::script_function::AppScriptFunctionRegistry,
    native::{FfiArg, NativeType},
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
            #[ffi]
            fn splat(v: $scalar) -> V<$ty> {
                V(<$ty>::splat(v))
            }

            /// A vector with all components set to `0`.
            #[ffi]
            fn zero() -> V<$ty> {
                V(<$ty>::ZERO)
            }

            /// A vector with all components set to `1`.
            #[ffi]
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
            #[ffi]
            fn eq(_self: R<$ty>, other: R<$ty>) -> bool {
                *_self == *other
            }

            /// The squared length of the vector.
            #[ffi]
            fn length_squared(_self: R<$ty>) -> $scalar {
                _self.length_squared()
            }

            /// The dot product of two vectors.
            #[ffi]
            fn dot(_self: R<$ty>, other: R<$ty>) -> $scalar {
                _self.dot(*other)
            }

            /// The smaller of each pair of components.
            #[ffi]
            fn min(_self: R<$ty>, other: R<$ty>) -> V<$ty> {
                V(_self.min(*other))
            }

            /// The larger of each pair of components.
            #[ffi]
            fn max(_self: R<$ty>, other: R<$ty>) -> V<$ty> {
                V(_self.max(*other))
            }

            /// Clamps every component between the components of `min` and `max`.
            #[ffi]
            fn clamp(_self: R<$ty>, min: R<$ty>, max: R<$ty>) -> V<$ty> {
                V(_self.clamp(*min, *max))
            }

            /// The smallest component.
            #[ffi]
            fn min_element(_self: R<$ty>) -> $scalar {
                _self.min_element()
            }

            /// The largest component.
            #[ffi]
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
int_vector_functions!(IVec4, i32, "ivec4_shared_functions");
int_vector_functions!(UVec2, u32, "uvec2_shared_functions");
int_vector_functions!(UVec3, u32, "uvec3_shared_functions");
int_vector_functions!(UVec4, u32, "uvec4_shared_functions");

#[script_bindings(
    remote,
    name = "ivec2_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl IVec2 {
    /// Creates a new vector.
    #[ffi]
    fn new(x: i32, y: i32) -> V<IVec2> {
        V(IVec2::new(x, y))
    }

    /// The unit vector along the x axis.
    #[ffi]
    fn unit_x() -> V<IVec2> {
        V(IVec2::X)
    }

    /// The unit vector along the y axis.
    #[ffi]
    fn unit_y() -> V<IVec2> {
        V(IVec2::Y)
    }

    /// Negates every component.
    #[ffi]
    fn neg(_self: R<IVec2>) -> V<IVec2> {
        V(-*_self)
    }

    /// The absolute value of every component.
    #[ffi]
    fn abs(_self: R<IVec2>) -> V<IVec2> {
        V(_self.abs())
    }

    /// A [`IVec3`](bevy_math::IVec3) with the given `z`.
    #[ffi]
    fn extend(_self: R<IVec2>, z: i32) -> V<IVec3> {
        V(_self.extend(z))
    }

    /// Converts to an unsigned vector.
    #[ffi]
    fn as_uvec2(_self: R<IVec2>) -> V<UVec2> {
        V(_self.as_uvec2())
    }

    /// Converts to a float vector.
    #[ffi]
    fn as_vec2(_self: R<IVec2>) -> V<Vec2> {
        V(_self.as_vec2())
    }

    /// Converts to a double precision float vector.
    #[ffi]
    fn as_dvec2(_self: R<IVec2>) -> V<DVec2> {
        V(_self.as_dvec2())
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
    #[ffi]
    fn new(x: i32, y: i32, z: i32) -> V<IVec3> {
        V(IVec3::new(x, y, z))
    }

    /// The unit vector along the x axis.
    #[ffi]
    fn unit_x() -> V<IVec3> {
        V(IVec3::X)
    }

    /// The unit vector along the y axis.
    #[ffi]
    fn unit_y() -> V<IVec3> {
        V(IVec3::Y)
    }

    /// The unit vector along the z axis.
    #[ffi]
    fn unit_z() -> V<IVec3> {
        V(IVec3::Z)
    }

    /// Negates every component.
    #[ffi]
    fn neg(_self: R<IVec3>) -> V<IVec3> {
        V(-*_self)
    }

    /// The absolute value of every component.
    #[ffi]
    fn abs(_self: R<IVec3>) -> V<IVec3> {
        V(_self.abs())
    }

    /// A [`IVec4`](bevy_math::IVec4) with the given `w`.
    #[ffi]
    fn extend(_self: R<IVec3>, w: i32) -> V<IVec4> {
        V(_self.extend(w))
    }

    /// A [`IVec2`](bevy_math::IVec2) of the `x` and `y` components.
    #[ffi]
    fn truncate(_self: R<IVec3>) -> V<IVec2> {
        V(_self.truncate())
    }

    /// Converts to an unsigned vector.
    #[ffi]
    fn as_uvec3(_self: R<IVec3>) -> V<UVec3> {
        V(_self.as_uvec3())
    }

    /// Converts to a float vector.
    #[ffi]
    fn as_vec3(_self: R<IVec3>) -> V<Vec3> {
        V(_self.as_vec3())
    }

    /// Converts to a double precision float vector.
    #[ffi]
    fn as_dvec3(_self: R<IVec3>) -> V<DVec3> {
        V(_self.as_dvec3())
    }
}

#[script_bindings(
    remote,
    name = "ivec4_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl IVec4 {
    /// Creates a new vector.
    #[ffi]
    fn new(x: i32, y: i32, z: i32, w: i32) -> V<IVec4> {
        V(IVec4::new(x, y, z, w))
    }

    /// The unit vector along the x axis.
    #[ffi]
    fn unit_x() -> V<IVec4> {
        V(IVec4::X)
    }

    /// The unit vector along the y axis.
    #[ffi]
    fn unit_y() -> V<IVec4> {
        V(IVec4::Y)
    }

    /// The unit vector along the z axis.
    #[ffi]
    fn unit_z() -> V<IVec4> {
        V(IVec4::Z)
    }

    /// The unit vector along the w axis.
    #[ffi]
    fn unit_w() -> V<IVec4> {
        V(IVec4::W)
    }

    /// Negates every component.
    #[ffi]
    fn neg(_self: R<IVec4>) -> V<IVec4> {
        V(-*_self)
    }

    /// The absolute value of every component.
    #[ffi]
    fn abs(_self: R<IVec4>) -> V<IVec4> {
        V(_self.abs())
    }

    /// A [`IVec3`](bevy_math::IVec3) of the `x`, `y` and `z` components.
    #[ffi]
    fn truncate(_self: R<IVec4>) -> V<IVec3> {
        V(_self.truncate())
    }

    /// Converts to an unsigned vector.
    #[ffi]
    fn as_uvec4(_self: R<IVec4>) -> V<UVec4> {
        V(_self.as_uvec4())
    }

    /// Converts to a float vector.
    #[ffi]
    fn as_vec4(_self: R<IVec4>) -> V<Vec4> {
        V(_self.as_vec4())
    }

    /// Converts to a double precision float vector.
    #[ffi]
    fn as_dvec4(_self: R<IVec4>) -> V<DVec4> {
        V(_self.as_dvec4())
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
    #[ffi]
    fn new(x: u32, y: u32) -> V<UVec2> {
        V(UVec2::new(x, y))
    }

    /// The unit vector along the x axis.
    #[ffi]
    fn unit_x() -> V<UVec2> {
        V(UVec2::X)
    }

    /// The unit vector along the y axis.
    #[ffi]
    fn unit_y() -> V<UVec2> {
        V(UVec2::Y)
    }

    /// A [`UVec3`](bevy_math::UVec3) with the given `z`.
    #[ffi]
    fn extend(_self: R<UVec2>, z: u32) -> V<UVec3> {
        V(_self.extend(z))
    }

    /// Converts to a signed vector.
    #[ffi]
    fn as_ivec2(_self: R<UVec2>) -> V<IVec2> {
        V(_self.as_ivec2())
    }

    /// Converts to a float vector.
    #[ffi]
    fn as_vec2(_self: R<UVec2>) -> V<Vec2> {
        V(_self.as_vec2())
    }

    /// Converts to a double precision float vector.
    #[ffi]
    fn as_dvec2(_self: R<UVec2>) -> V<DVec2> {
        V(_self.as_dvec2())
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
    #[ffi]
    fn new(x: u32, y: u32, z: u32) -> V<UVec3> {
        V(UVec3::new(x, y, z))
    }

    /// The unit vector along the x axis.
    #[ffi]
    fn unit_x() -> V<UVec3> {
        V(UVec3::X)
    }

    /// The unit vector along the y axis.
    #[ffi]
    fn unit_y() -> V<UVec3> {
        V(UVec3::Y)
    }

    /// The unit vector along the z axis.
    #[ffi]
    fn unit_z() -> V<UVec3> {
        V(UVec3::Z)
    }

    /// A [`UVec4`](bevy_math::UVec4) with the given `w`.
    #[ffi]
    fn extend(_self: R<UVec3>, w: u32) -> V<UVec4> {
        V(_self.extend(w))
    }

    /// A [`UVec2`](bevy_math::UVec2) of the `x` and `y` components.
    #[ffi]
    fn truncate(_self: R<UVec3>) -> V<UVec2> {
        V(_self.truncate())
    }

    /// Converts to a signed vector.
    #[ffi]
    fn as_ivec3(_self: R<UVec3>) -> V<IVec3> {
        V(_self.as_ivec3())
    }

    /// Converts to a float vector.
    #[ffi]
    fn as_vec3(_self: R<UVec3>) -> V<Vec3> {
        V(_self.as_vec3())
    }

    /// Converts to a double precision float vector.
    #[ffi]
    fn as_dvec3(_self: R<UVec3>) -> V<DVec3> {
        V(_self.as_dvec3())
    }
}

#[script_bindings(
    remote,
    name = "uvec4_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "math",
    core
)]
impl UVec4 {
    /// Creates a new vector.
    #[ffi]
    fn new(x: u32, y: u32, z: u32, w: u32) -> V<UVec4> {
        V(UVec4::new(x, y, z, w))
    }

    /// The unit vector along the x axis.
    #[ffi]
    fn unit_x() -> V<UVec4> {
        V(UVec4::X)
    }

    /// The unit vector along the y axis.
    #[ffi]
    fn unit_y() -> V<UVec4> {
        V(UVec4::Y)
    }

    /// The unit vector along the z axis.
    #[ffi]
    fn unit_z() -> V<UVec4> {
        V(UVec4::Z)
    }

    /// The unit vector along the w axis.
    #[ffi]
    fn unit_w() -> V<UVec4> {
        V(UVec4::W)
    }

    /// A [`UVec3`](bevy_math::UVec3) of the `x`, `y` and `z` components.
    #[ffi]
    fn truncate(_self: R<UVec4>) -> V<UVec3> {
        V(_self.truncate())
    }

    /// Converts to a signed vector.
    #[ffi]
    fn as_ivec4(_self: R<UVec4>) -> V<IVec4> {
        V(_self.as_ivec4())
    }

    /// Converts to a float vector.
    #[ffi]
    fn as_vec4(_self: R<UVec4>) -> V<Vec4> {
        V(_self.as_vec4())
    }

    /// Converts to a double precision float vector.
    #[ffi]
    fn as_dvec4(_self: R<UVec4>) -> V<DVec4> {
        V(_self.as_dvec4())
    }
}

/// Registers the FFI forms of the operators of the vector `T`: one for each way their `Union` parameter is passed,
/// a vector or a number.
fn register_operator_ffi<T, S>(world: &mut World)
where
    T: Copy
        + 'static
        + Add<Output = T>
        + Add<S, Output = T>
        + Sub<Output = T>
        + Sub<S, Output = T>
        + Mul<Output = T>
        + Mul<S, Output = T>
        + Div<Output = T>
        + Div<S, Output = T>
        + Rem<Output = T>
        + Rem<S, Output = T>,
    S: FfiArg,
{
    let mut registry = world
        .get_resource_or_init::<AppScriptFunctionRegistry>()
        .0
        .clone();
    let mut registry = registry.write();
    let ty = TypeId::of::<T>();
    registry.register_ffi(ty, "add", |a: R<T>, b: R<T>| V(*a + *b));
    registry.register_ffi(ty, "add", |a: R<T>, b: S| V(*a + b));
    registry.register_ffi(ty, "sub", |a: R<T>, b: R<T>| V(*a - *b));
    registry.register_ffi(ty, "sub", |a: R<T>, b: S| V(*a - b));
    registry.register_ffi(ty, "mul", |a: R<T>, b: R<T>| V(*a * *b));
    registry.register_ffi(ty, "mul", |a: R<T>, b: S| V(*a * b));
    registry.register_ffi(ty, "div", |a: R<T>, b: R<T>| V(*a / *b));
    registry.register_ffi(ty, "div", |a: R<T>, b: S| V(*a / b));
    registry.register_ffi(ty, "rem", |a: R<T>, b: R<T>| V(*a % *b));
    registry.register_ffi(ty, "rem", |a: R<T>, b: S| V(*a % b));
}

/// Registers the functions of all integer vectors, which are stored natively by the languages.
pub(crate) fn register(world: &mut World) {
    register_ivec2_shared_functions(world);
    register_ivec3_shared_functions(world);
    register_ivec4_shared_functions(world);
    register_uvec2_shared_functions(world);
    register_uvec3_shared_functions(world);
    register_uvec4_shared_functions(world);
    register_ivec2_functions(world);
    register_ivec3_functions(world);
    register_ivec4_functions(world);
    register_uvec2_functions(world);
    register_uvec3_functions(world);
    register_uvec4_functions(world);
    register_operator_ffi::<IVec2, i32>(world);
    register_operator_ffi::<IVec3, i32>(world);
    register_operator_ffi::<IVec4, i32>(world);
    register_operator_ffi::<UVec2, u32>(world);
    register_operator_ffi::<UVec3, u32>(world);
    register_operator_ffi::<UVec4, u32>(world);

    assert_ffi_layout!(IVec2 { x: i32, y: i32 });
    assert_ffi_layout!(IVec3 {
        x: i32,
        y: i32,
        z: i32
    });
    assert_ffi_layout!(IVec4 {
        x: i32,
        y: i32,
        z: i32,
        w: i32
    });
    assert_ffi_layout!(UVec2 { x: u32, y: u32 });
    assert_ffi_layout!(UVec3 {
        x: u32,
        y: u32,
        z: u32
    });
    assert_ffi_layout!(UVec4 {
        x: u32,
        y: u32,
        z: u32,
        w: u32
    });
    // Safety: the fields are at their offsets, vectors have no invalid values
    unsafe {
        NativeType::of::<IVec2>()
            .field("x", offset_of!(IVec2, x), |v| &v.x)
            .field("y", offset_of!(IVec2, y), |v| &v.y)
            .ffi()
            .register(world);
        NativeType::of::<IVec3>()
            .field("x", offset_of!(IVec3, x), |v| &v.x)
            .field("y", offset_of!(IVec3, y), |v| &v.y)
            .field("z", offset_of!(IVec3, z), |v| &v.z)
            .ffi()
            .register(world);
        NativeType::of::<IVec4>()
            .field("x", offset_of!(IVec4, x), |v| &v.x)
            .field("y", offset_of!(IVec4, y), |v| &v.y)
            .field("z", offset_of!(IVec4, z), |v| &v.z)
            .field("w", offset_of!(IVec4, w), |v| &v.w)
            .ffi()
            .register(world);
        NativeType::of::<UVec2>()
            .field("x", offset_of!(UVec2, x), |v| &v.x)
            .field("y", offset_of!(UVec2, y), |v| &v.y)
            .ffi()
            .register(world);
        NativeType::of::<UVec3>()
            .field("x", offset_of!(UVec3, x), |v| &v.x)
            .field("y", offset_of!(UVec3, y), |v| &v.y)
            .field("z", offset_of!(UVec3, z), |v| &v.z)
            .ffi()
            .register(world);
        NativeType::of::<UVec4>()
            .field("x", offset_of!(UVec4, x), |v| &v.x)
            .field("y", offset_of!(UVec4, y), |v| &v.y)
            .field("z", offset_of!(UVec4, z), |v| &v.z)
            .field("w", offset_of!(UVec4, w), |v| &v.w)
            .ffi()
            .register(world);
    }
}
