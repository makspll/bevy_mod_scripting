//! Overlays over glam types.

use crate::unsafe_c_overlay;

unsafe_c_overlay! {
    // SAFETY: three f32s at 0/4/8
    #[probe(glam::Vec3::new(1.1, 2.2, 3.3))]
    glam::Vec3 as Vec3C {
        #[probe = 1.1] x: f32,
        #[probe = 2.2] y: f32,
        #[probe = 3.3] z: f32,
    }
}

unsafe_c_overlay! {
    // SAFETY: four f32 lanes on every glam backend; x/y/z are reached through Deref on SIMD backends,
    // the fourth lane is a private field (scalar) or an unnamed lane (SIMD), always an initialized f32
    #[probe(glam::Vec3A::new(1.1, 2.2, 3.3))]
    glam::Vec3A as Vec3AC {
        #[probe = 1.1] x: f32,
        #[probe = 2.2] y: f32,
        #[probe = 3.3] z: f32,
        #[opaque] #[probe = _] _w: f32,
    }
}

unsafe_c_overlay! {
    // SAFETY: three Vec3 columns
    #[probe(glam::Mat3::from_cols(
        glam::Vec3::new(1.1, 2.2, 3.3),
        glam::Vec3::new(4.4, 5.1, 6.6),
        glam::Vec3::new(7.7, 8.8, 9.9),
    ))]
    glam::Mat3 as Mat3C {
        #[probe = glam::Vec3::new(1.1, 2.2, 3.3)] x_axis: glam::Vec3,
        #[probe = glam::Vec3::new(4.4, 5.1, 6.6)] y_axis: glam::Vec3,
        #[probe = glam::Vec3::new(7.7, 8.8, 9.9)] z_axis: glam::Vec3,
    }
}

/// An overlay of a vector with one scalar per component, each probed with its own value.
macro_rules! vector {
    ($ty:ty as $name:ident: $scalar:ty { $($field:ident = $value:expr),+ }) => {
        unsafe_c_overlay! {
            // SAFETY: components of `$scalar` in declaration order, no padding
            #[probe(<$ty>::new($($value),+))]
            $ty as $name { $(#[probe = $value] $field: $scalar,)+ }
        }
    };
}

/// The 2, 3 and 4 component vectors of a scalar type, probed with `a`, `b`, `c`, `d`.
macro_rules! vectors {
    ($scalar:ty = [$a:expr, $b:expr, $c:expr, $d:expr]: $v2:ident as $c2:ident, $v3:ident as $c3:ident, $v4:ident as $c4:ident) => {
        vector!(glam::$v2 as $c2: $scalar { x = $a, y = $b });
        vector!(glam::$v3 as $c3: $scalar { x = $a, y = $b, z = $c });
        vector!(glam::$v4 as $c4: $scalar { x = $a, y = $b, z = $c, w = $d });
    };
}

// f32 (`Vec3`, `Vec3A` above). `Vec4` is a SIMD register on SIMD backends, its components are reached through
// `Deref`; its alignment (16, 4 with glam's `scalar-math`) comes from the type
vector!(glam::Vec2 as Vec2C: f32 { x = 1.1, y = 2.2 });
vector!(glam::Vec4 as Vec4C: f32 { x = 1.1, y = 2.2, z = 3.3, w = 4.4 });
vectors!(f64 = [1.1, 2.2, 3.3, 4.4]: DVec2 as DVec2C, DVec3 as DVec3C, DVec4 as DVec4C);
vectors!(i8 = [0x11, 0x22, 0x33, 0x44]: I8Vec2 as I8Vec2C, I8Vec3 as I8Vec3C, I8Vec4 as I8Vec4C);
vectors!(u8 = [0x11, 0x22, 0x33, 0x44]: U8Vec2 as U8Vec2C, U8Vec3 as U8Vec3C, U8Vec4 as U8Vec4C);
vectors!(i16 = [0x1122, 0x3344, 0x5566, 0x7711]: I16Vec2 as I16Vec2C, I16Vec3 as I16Vec3C, I16Vec4 as I16Vec4C);
vectors!(u16 = [0x1122, 0x3344, 0x5566, 0x7711]: U16Vec2 as U16Vec2C, U16Vec3 as U16Vec3C, U16Vec4 as U16Vec4C);
vectors!(i32 = [0x0102_0304, 0x0506_0708, 0x090A_0B0C, 0x0D0E_0F11]: IVec2 as IVec2C, IVec3 as IVec3C, IVec4 as IVec4C);
vectors!(u32 = [0x0102_0304, 0x0506_0708, 0x090A_0B0C, 0x0D0E_0F11]: UVec2 as UVec2C, UVec3 as UVec3C, UVec4 as UVec4C);
vectors!(i64 = [0x0102_0304_0506_0708, 0x1112_1314_1516_1718, 0x2122_2324_2526_2728, 0x3132_3334_3536_3738]:
    I64Vec2 as I64Vec2C, I64Vec3 as I64Vec3C, I64Vec4 as I64Vec4C);
vectors!(u64 = [0x0102_0304_0506_0708, 0x1112_1314_1516_1718, 0x2122_2324_2526_2728, 0x3132_3334_3536_3738]:
    U64Vec2 as U64Vec2C, U64Vec3 as U64Vec3C, U64Vec4 as U64Vec4C);
// pointer sized, so the probe values fill every byte on every target
vectors!(usize = [
        usize::from_ne_bytes([0x11; size_of::<usize>()]),
        usize::from_ne_bytes([0x22; size_of::<usize>()]),
        usize::from_ne_bytes([0x33; size_of::<usize>()]),
        usize::from_ne_bytes([0x44; size_of::<usize>()])
    ]: USizeVec2 as USizeVec2C, USizeVec3 as USizeVec3C, USizeVec4 as USizeVec4C);
vectors!(isize = [
        isize::from_ne_bytes([0x11; size_of::<isize>()]),
        isize::from_ne_bytes([0x22; size_of::<isize>()]),
        isize::from_ne_bytes([0x33; size_of::<isize>()]),
        isize::from_ne_bytes([0x44; size_of::<isize>()])
    ]: ISizeVec2 as ISizeVec2C, ISizeVec3 as ISizeVec3C, ISizeVec4 as ISizeVec4C);

// bool vectors: a `bool` has one non-zero value, so each component is told apart by its own probe
unsafe_c_overlay! {
    // SAFETY: bools in declaration order, no padding
    #[probe(glam::BVec2::new(true, false))]
    #[probe(glam::BVec2::new(false, true))]
    glam::BVec2 as BVec2C {
        #[probe(true, false)] x: bool,
        #[probe(false, true)] y: bool,
    }
}

unsafe_c_overlay! {
    // SAFETY: bools in declaration order, no padding
    #[probe(glam::BVec3::new(true, false, false))]
    #[probe(glam::BVec3::new(false, true, false))]
    #[probe(glam::BVec3::new(false, false, true))]
    glam::BVec3 as BVec3C {
        #[probe(true, false, false)] x: bool,
        #[probe(false, true, false)] y: bool,
        #[probe(false, false, true)] z: bool,
    }
}

unsafe_c_overlay! {
    // SAFETY: bools in declaration order, no padding
    #[probe(glam::BVec4::new(true, false, false, false))]
    #[probe(glam::BVec4::new(false, true, false, false))]
    #[probe(glam::BVec4::new(false, false, true, false))]
    #[probe(glam::BVec4::new(false, false, false, true))]
    glam::BVec4 as BVec4C {
        #[probe(true, false, false, false)] x: bool,
        #[probe(false, true, false, false)] y: bool,
        #[probe(false, false, true, false)] z: bool,
        #[probe(false, false, false, true)] w: bool,
    }
}

unsafe_c_overlay! {
    // SAFETY: x, y, z, w f32s, components reached through `Deref` on SIMD backends
    #[probe(glam::Quat::from_xyzw(1.1, 2.2, 3.3, 4.4))]
    glam::Quat as QuatC {
        #[probe = 1.1] x: f32,
        #[probe = 2.2] y: f32,
        #[probe = 3.3] z: f32,
        #[probe = 4.4] w: f32,
    }
}

unsafe_c_overlay! {
    // SAFETY: x, y, z, w f64s
    #[probe(glam::DQuat::from_xyzw(1.1, 2.2, 3.3, 4.4))]
    glam::DQuat as DQuatC {
        #[probe = 1.1] x: f64,
        #[probe = 2.2] y: f64,
        #[probe = 3.3] z: f64,
        #[probe = 4.4] w: f64,
    }
}

unsafe_c_overlay! {
    // SAFETY: two Vec2 columns, reached through `Deref` on SIMD backends (one register)
    #[probe(glam::Mat2::from_cols(glam::Vec2::new(1.1, 2.2), glam::Vec2::new(3.3, 4.4)))]
    glam::Mat2 as Mat2C {
        #[probe = glam::Vec2::new(1.1, 2.2)] x_axis: glam::Vec2,
        #[probe = glam::Vec2::new(3.3, 4.4)] y_axis: glam::Vec2,
    }
}

unsafe_c_overlay! {
    // SAFETY: three Vec3A columns
    #[probe(glam::Mat3A::from_cols(
        glam::Vec3A::new(1.1, 2.2, 3.3),
        glam::Vec3A::new(4.4, 5.1, 6.6),
        glam::Vec3A::new(7.7, 8.8, 9.9),
    ))]
    glam::Mat3A as Mat3AC {
        #[probe = glam::Vec3A::new(1.1, 2.2, 3.3)] x_axis: glam::Vec3A,
        #[probe = glam::Vec3A::new(4.4, 5.1, 6.6)] y_axis: glam::Vec3A,
        #[probe = glam::Vec3A::new(7.7, 8.8, 9.9)] z_axis: glam::Vec3A,
    }
}

unsafe_c_overlay! {
    // SAFETY: four Vec4 columns
    #[probe(glam::Mat4::from_cols(
        glam::Vec4::new(1.1, 2.2, 3.3, 4.4),
        glam::Vec4::new(6.6, 7.7, 8.8, 9.9),
        glam::Vec4::new(10.1, 11.1, 12.1, 13.1),
        glam::Vec4::new(14.1, 15.1, 16.1, 17.1),
    ))]
    glam::Mat4 as Mat4C {
        #[probe = glam::Vec4::new(1.1, 2.2, 3.3, 4.4)] x_axis: glam::Vec4,
        #[probe = glam::Vec4::new(6.6, 7.7, 8.8, 9.9)] y_axis: glam::Vec4,
        #[probe = glam::Vec4::new(10.1, 11.1, 12.1, 13.1)] z_axis: glam::Vec4,
        #[probe = glam::Vec4::new(14.1, 15.1, 16.1, 17.1)] w_axis: glam::Vec4,
    }
}

unsafe_c_overlay! {
    // SAFETY: two DVec2 columns
    #[probe(glam::DMat2::from_cols(glam::DVec2::new(1.1, 2.2), glam::DVec2::new(3.3, 4.4)))]
    glam::DMat2 as DMat2C {
        #[probe = glam::DVec2::new(1.1, 2.2)] x_axis: glam::DVec2,
        #[probe = glam::DVec2::new(3.3, 4.4)] y_axis: glam::DVec2,
    }
}

unsafe_c_overlay! {
    // SAFETY: three DVec3 columns
    #[probe(glam::DMat3::from_cols(
        glam::DVec3::new(1.1, 2.2, 3.3),
        glam::DVec3::new(4.4, 5.1, 6.6),
        glam::DVec3::new(7.7, 8.8, 9.9),
    ))]
    glam::DMat3 as DMat3C {
        #[probe = glam::DVec3::new(1.1, 2.2, 3.3)] x_axis: glam::DVec3,
        #[probe = glam::DVec3::new(4.4, 5.1, 6.6)] y_axis: glam::DVec3,
        #[probe = glam::DVec3::new(7.7, 8.8, 9.9)] z_axis: glam::DVec3,
    }
}

unsafe_c_overlay! {
    // SAFETY: four DVec4 columns
    #[probe(glam::DMat4::from_cols(
        glam::DVec4::new(1.1, 2.2, 3.3, 4.4),
        glam::DVec4::new(6.6, 7.7, 8.8, 9.9),
        glam::DVec4::new(10.1, 11.1, 12.1, 13.1),
        glam::DVec4::new(14.1, 15.1, 16.1, 17.1),
    ))]
    glam::DMat4 as DMat4C {
        #[probe = glam::DVec4::new(1.1, 2.2, 3.3, 4.4)] x_axis: glam::DVec4,
        #[probe = glam::DVec4::new(6.6, 7.7, 8.8, 9.9)] y_axis: glam::DVec4,
        #[probe = glam::DVec4::new(10.1, 11.1, 12.1, 13.1)] z_axis: glam::DVec4,
        #[probe = glam::DVec4::new(14.1, 15.1, 16.1, 17.1)] w_axis: glam::DVec4,
    }
}

unsafe_c_overlay! {
    // SAFETY: Mat2 then Vec2, then tail padding up to Mat2's alignment (8 bytes on SIMD backends, none with glam's
    // `scalar-math`, where Mat2 is 4 aligned)
    #[probe(glam::Affine2 {
        matrix2: glam::Mat2::from_cols(glam::Vec2::new(1.1, 2.2), glam::Vec2::new(3.3, 4.4)),
        translation: glam::Vec2::new(6.6, 7.7),
    })]
    glam::Affine2 as Affine2C {
        #[probe = glam::Mat2::from_cols(glam::Vec2::new(1.1, 2.2), glam::Vec2::new(3.3, 4.4))] matrix2: glam::Mat2,
        #[probe = glam::Vec2::new(6.6, 7.7)] translation: glam::Vec2,
        #[pad] _p0: [u8; size_of::<glam::Affine2>() - size_of::<glam::Mat2>() - size_of::<glam::Vec2>()],
    }
}

unsafe_c_overlay! {
    // SAFETY: Mat3A then Vec3A
    #[probe(glam::Affine3A {
        matrix3: glam::Mat3A::from_cols(
            glam::Vec3A::new(1.1, 2.2, 3.3),
            glam::Vec3A::new(4.4, 5.1, 6.6),
            glam::Vec3A::new(7.7, 8.8, 9.9),
        ),
        translation: glam::Vec3A::new(10.1, 11.1, 12.1),
    })]
    glam::Affine3A as Affine3AC {
        #[probe = glam::Mat3A::from_cols(
            glam::Vec3A::new(1.1, 2.2, 3.3),
            glam::Vec3A::new(4.4, 5.1, 6.6),
            glam::Vec3A::new(7.7, 8.8, 9.9),
        )] matrix3: glam::Mat3A,
        #[probe = glam::Vec3A::new(10.1, 11.1, 12.1)] translation: glam::Vec3A,
    }
}

unsafe_c_overlay! {
    // SAFETY: DMat2 then DVec2
    #[probe(glam::DAffine2 {
        matrix2: glam::DMat2::from_cols(glam::DVec2::new(1.1, 2.2), glam::DVec2::new(3.3, 4.4)),
        translation: glam::DVec2::new(6.6, 7.7),
    })]
    glam::DAffine2 as DAffine2C {
        #[probe = glam::DMat2::from_cols(glam::DVec2::new(1.1, 2.2), glam::DVec2::new(3.3, 4.4))] matrix2: glam::DMat2,
        #[probe = glam::DVec2::new(6.6, 7.7)] translation: glam::DVec2,
    }
}

unsafe_c_overlay! {
    // SAFETY: DMat3 then DVec3
    #[probe(glam::DAffine3 {
        matrix3: glam::DMat3::from_cols(
            glam::DVec3::new(1.1, 2.2, 3.3),
            glam::DVec3::new(4.4, 5.1, 6.6),
            glam::DVec3::new(7.7, 8.8, 9.9),
        ),
        translation: glam::DVec3::new(10.1, 11.1, 12.1),
    })]
    glam::DAffine3 as DAffine3C {
        #[probe = glam::DMat3::from_cols(
            glam::DVec3::new(1.1, 2.2, 3.3),
            glam::DVec3::new(4.4, 5.1, 6.6),
            glam::DVec3::new(7.7, 8.8, 9.9),
        )] matrix3: glam::DMat3,
        #[probe = glam::DVec3::new(10.1, 11.1, 12.1)] translation: glam::DVec3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FfiField;

    fn view<T: FfiField>(value: &T) -> &T::Overlay {
        // SAFETY: FfiField guarantees Overlay mirrors T
        unsafe { &*(value as *const T).cast::<T::Overlay>() }
    }

    #[test]
    fn glam_overlays_mirror_real_values() {
        let v = glam::Vec3::new(1.0, 2.0, 3.0);
        let c: &Vec3C = view(&v);
        assert_eq!((c.x, c.y, c.z), (1.0, 2.0, 3.0));

        let v = glam::Vec3A::new(1.0, 2.0, 3.0);
        let c: &Vec3AC = view(&v);
        assert_eq!((c.x, c.y, c.z), (1.0, 2.0, 3.0));
        assert_eq!(align_of::<Vec3AC>(), align_of::<glam::Vec3A>());

        let m = glam::Mat3::from_cols(
            glam::Vec3::new(1.0, 2.0, 3.0),
            glam::Vec3::new(4.0, 5.0, 6.0),
            glam::Vec3::new(7.0, 8.0, 9.0),
        );
        let c: &Mat3C = view(&m);
        assert_eq!((c.x_axis.x, c.y_axis.y, c.z_axis.z), (1.0, 5.0, 9.0));

        assert_eq!(<glam::Vec3A as FfiField>::INIT_MASK, &[true; 16]);
        assert_eq!(<glam::Mat3 as FfiField>::INIT_MASK, &[true; 36]);

        // Affine2's tail padding exists only on SIMD backends (Mat2 is 16 aligned there), and is left out of the cdef
        // when empty
        let tail = size_of::<glam::Affine2>() - 24;
        let mask = <glam::Affine2 as FfiField>::INIT_MASK;
        assert!(
            mask[..24].iter().all(|b| *b)
                && mask[24..].iter().all(|b| !b)
                && mask.len() == 24 + tail
        );
        assert_eq!(
            <glam::Affine2 as FfiField>::C_DECL.cdef.contains("_p0"),
            tail > 0
        );
        let a = glam::Affine2::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let c: &Affine2C = view(&a);
        assert_eq!((c.matrix2.y_axis.x, c.translation.y), (3.0, 6.0));

        let q = glam::Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
        let c: &QuatC = view(&q);
        assert_eq!((c.x, c.w), (1.0, 4.0));

        let b = glam::BVec3::new(false, true, false);
        let c: &BVec3C = view(&b);
        assert_eq!((c.x, c.y, c.z), (false, true, false));

        let u = glam::USizeVec2::new(usize::MAX, 7);
        let c: &USizeVec2C = view(&u);
        assert_eq!((c.x, c.y), (usize::MAX, 7));
        assert_eq!(
            <glam::USizeVec2 as FfiField>::C_DECL
                .cdef
                .matches("uintptr_t")
                .count(),
            2
        );
    }
}
