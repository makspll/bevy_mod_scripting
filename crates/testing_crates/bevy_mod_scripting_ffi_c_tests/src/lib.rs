//! Round trips of `bevy_mod_scripting_ffi` overlays through real C code.
//!
//! `c/overlays.h` is generated from the overlays and committed (rerun the tests with `BLESS_MODE=1` to update it),
//! the build script compiles `c/round_trip.c` against it, so its `_Static_assert`s check the C layout equals the Rust
//! one. The tests then read and write Rust values from C. On Linux the C code runs with UBSan, and CI also runs these
//! tests under valgrind (`cargo xtask ffi`).

use bevy_mod_scripting_ffi::{CDecl, FfiField};

/// The overlays declared in `c/overlays.h`: every glam overlay, so the C compiler checks all of their layouts.
pub fn roots() -> Vec<CDecl> {
    macro_rules! decls {
        ($($ty:ident),* $(,)?) => { vec![$(<glam::$ty as FfiField>::C_DECL),*] };
    }
    decls![
        Vec2, Vec3, Vec3A, Vec4, Quat, Mat2, Mat3, Mat3A, Mat4, Affine2, Affine3A, //
        DVec2, DVec3, DVec4, DQuat, DMat2, DMat3, DMat4, DAffine2, DAffine3, //
        I8Vec2, I8Vec3, I8Vec4, U8Vec2, U8Vec3, U8Vec4, I16Vec2, I16Vec3, I16Vec4, U16Vec2,
        U16Vec3, U16Vec4, //
        IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, I64Vec2, I64Vec3, I64Vec4, U64Vec2, U64Vec3,
        U64Vec4, //
        USizeVec2, USizeVec3, USizeVec4, ISizeVec2, ISizeVec3, ISizeVec4, //
        BVec2, BVec3, BVec4,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/c/overlays.h");

    #[test]
    fn header_is_blessed() {
        let generated = bevy_mod_scripting_ffi::c_header(&roots()).unwrap();
        if std::env::var_os("BLESS_MODE").is_some() {
            std::fs::write(HEADER, &generated).unwrap();
            return;
        }
        let committed = std::fs::read_to_string(HEADER)
            .unwrap()
            .replace("\r\n", "\n");
        assert_eq!(
            committed, generated,
            "c/overlays.h is out of date, rerun the tests with BLESS_MODE=1"
        );
    }

    #[cfg(not(bms_ffi_bless))]
    #[allow(non_snake_case)]
    mod round_trip {
        use bevy_mod_scripting_ffi::FfiField;

        type Vec3C = <glam::Vec3 as bevy_mod_scripting_ffi::FfiField>::Overlay;
        type Vec3AC = <glam::Vec3A as bevy_mod_scripting_ffi::FfiField>::Overlay;
        type Mat3C = <glam::Mat3 as bevy_mod_scripting_ffi::FfiField>::Overlay;

        // c/round_trip.c. Overlays cross by pointer only: the lint looks into the zero-sized alignment field
        // (`[glam::Vec3A; 0]`, holding an `__m128`), which has no bytes and isn't part of the C declaration
        #[allow(improper_ctypes)]
        unsafe extern "C" {
            fn bms_ffi_vec3_get(v: *const Vec3C, i: i32) -> f32;
            fn bms_ffi_vec3_set(v: *mut Vec3C, i: i32, value: f32);
            fn bms_ffi_vec3a_get(v: *const Vec3AC, i: i32) -> f32;
            fn bms_ffi_vec3a_set(v: *mut Vec3AC, i: i32, value: f32);
            fn bms_ffi_mat3_get(m: *const Mat3C, col: i32, row: i32) -> f32;
            fn bms_ffi_mat3_set(m: *mut Mat3C, col: i32, row: i32, value: f32);
            fn bms_ffi_vec3a_new(x: f32, y: f32, z: f32) -> *mut Vec3AC;
            fn bms_ffi_vec3a_free(v: *mut Vec3AC);
        }

        type Vec4C = <glam::Vec4 as FfiField>::Overlay;
        type QuatC = <glam::Quat as FfiField>::Overlay;
        type Mat4C = <glam::Mat4 as FfiField>::Overlay;
        type Affine2C = <glam::Affine2 as FfiField>::Overlay;
        type DVec3C = <glam::DVec3 as FfiField>::Overlay;
        type IVec3C = <glam::IVec3 as FfiField>::Overlay;
        type U8Vec4C = <glam::U8Vec4 as FfiField>::Overlay;
        type USizeVec2C = <glam::USizeVec2 as FfiField>::Overlay;
        type BVec3C = <glam::BVec3 as FfiField>::Overlay;

        // `bms_ffi_<Overlay>_get/set` in c/round_trip.c, element `i` in `to_cols_array` order
        #[allow(improper_ctypes)]
        unsafe extern "C" {
            fn bms_ffi_Vec4C_get(v: *const Vec4C, i: i32) -> f32;
            fn bms_ffi_Vec4C_set(v: *mut Vec4C, i: i32, value: f32);
            fn bms_ffi_QuatC_get(v: *const QuatC, i: i32) -> f32;
            fn bms_ffi_QuatC_set(v: *mut QuatC, i: i32, value: f32);
            fn bms_ffi_Mat4C_get(v: *const Mat4C, i: i32) -> f32;
            fn bms_ffi_Mat4C_set(v: *mut Mat4C, i: i32, value: f32);
            fn bms_ffi_Affine2C_get(v: *const Affine2C, i: i32) -> f32;
            fn bms_ffi_Affine2C_set(v: *mut Affine2C, i: i32, value: f32);
            fn bms_ffi_DVec3C_get(v: *const DVec3C, i: i32) -> f64;
            fn bms_ffi_DVec3C_set(v: *mut DVec3C, i: i32, value: f64);
            fn bms_ffi_IVec3C_get(v: *const IVec3C, i: i32) -> i32;
            fn bms_ffi_IVec3C_set(v: *mut IVec3C, i: i32, value: i32);
            fn bms_ffi_U8Vec4C_get(v: *const U8Vec4C, i: i32) -> u8;
            fn bms_ffi_U8Vec4C_set(v: *mut U8Vec4C, i: i32, value: u8);
            fn bms_ffi_USizeVec2C_get(v: *const USizeVec2C, i: i32) -> usize;
            fn bms_ffi_USizeVec2C_set(v: *mut USizeVec2C, i: i32, value: usize);
            fn bms_ffi_BVec3C_get(v: *const BVec3C, i: i32) -> bool;
            fn bms_ffi_BVec3C_set(v: *mut BVec3C, i: i32, value: bool);
        }

        /// For each element `i`: with every element at `background`, C writes `mark` to `i` and Rust sees exactly
        /// that element change; then Rust writes `mark` to `i` and C reads it there and `background` elsewhere.
        #[allow(clippy::too_many_arguments)]
        fn round_trip_elements<R: FfiField, E: Copy + PartialEq + std::fmt::Debug>(
            real: &mut R,
            len: usize,
            (background, mark): (E, E),
            get: impl Fn(&R, usize) -> E,
            set: impl Fn(&mut R, usize, E),
            c_get: unsafe extern "C" fn(*const R::Overlay, i32) -> E,
            c_set: unsafe extern "C" fn(*mut R::Overlay, i32, E),
        ) {
            let fill = |real: &mut R| (0..len).for_each(|j| set(real, j, background));
            for i in 0..len {
                fill(real);
                // SAFETY: `real` is a live value of the overlay's real type, C only accesses its fields
                unsafe { c_set((real as *mut R).cast(), i as i32, mark) };
                for j in 0..len {
                    let expected = if j == i { mark } else { background };
                    assert_eq!(get(real, j), expected, "C wrote element {i}, Rust read {j}");
                }
                fill(real);
                set(real, i, mark);
                for j in 0..len {
                    let expected = if j == i { mark } else { background };
                    // SAFETY: as above
                    let read = unsafe { c_get((real as *const R).cast(), j as i32) };
                    assert_eq!(read, expected, "Rust wrote element {i}, C read {j}");
                }
            }
        }

        #[test]
        fn c_round_trips_every_family() {
            use glam::*;
            round_trip_elements(
                &mut { Vec4::ZERO },
                4,
                (0.0, 1.5),
                |v, i| v[i],
                |v, i, e| v[i] = e,
                bms_ffi_Vec4C_get,
                bms_ffi_Vec4C_set,
            );
            round_trip_elements(
                &mut { Quat::IDENTITY },
                4,
                (0.0, 1.5),
                |q, i| q.to_array()[i],
                |q, i, e| {
                    let mut a = q.to_array();
                    a[i] = e;
                    *q = Quat::from_array(a);
                },
                bms_ffi_QuatC_get,
                bms_ffi_QuatC_set,
            );
            round_trip_elements(
                &mut { Mat4::ZERO },
                16,
                (0.0, 1.5),
                |m, i| m.col(i / 4)[i % 4],
                |m, i, e| m.col_mut(i / 4)[i % 4] = e,
                bms_ffi_Mat4C_get,
                bms_ffi_Mat4C_set,
            );
            round_trip_elements(
                &mut { Affine2::ZERO },
                6,
                (0.0, 1.5),
                |a, i| a.to_cols_array()[i],
                |a, i, e| {
                    let mut c = a.to_cols_array();
                    c[i] = e;
                    *a = Affine2::from_cols_array(&c);
                },
                bms_ffi_Affine2C_get,
                bms_ffi_Affine2C_set,
            );
            round_trip_elements(
                &mut { DVec3::ZERO },
                3,
                (0.0, 1.5),
                |v, i| v[i],
                |v, i, e| v[i] = e,
                bms_ffi_DVec3C_get,
                bms_ffi_DVec3C_set,
            );
            round_trip_elements(
                &mut { IVec3::ZERO },
                3,
                (0, -7),
                |v, i| v[i],
                |v, i, e| v[i] = e,
                bms_ffi_IVec3C_get,
                bms_ffi_IVec3C_set,
            );
            round_trip_elements(
                &mut { U8Vec4::ZERO },
                4,
                (0, 0xAB),
                |v, i| v[i],
                |v, i, e| v[i] = e,
                bms_ffi_U8Vec4C_get,
                bms_ffi_U8Vec4C_set,
            );
            round_trip_elements(
                &mut { USizeVec2::ZERO },
                2,
                (0, usize::MAX),
                |v, i| v[i],
                |v, i, e| v[i] = e,
                bms_ffi_USizeVec2C_get,
                bms_ffi_USizeVec2C_set,
            );
            round_trip_elements(
                &mut { BVec3::FALSE },
                3,
                (false, true),
                |b, i| b.test(i),
                |b, i, e| b.set(i, e),
                bms_ffi_BVec3C_get,
                bms_ffi_BVec3C_set,
            );
        }

        /// Distinct value per element, so a field read from the wrong place shows up.
        fn value(i: usize) -> f32 {
            1.5 + i as f32
        }

        #[test]
        fn c_reads_and_writes_rust_values() {
            // SAFETY: each pointer comes from a live Rust value of the overlay's real type, the C functions only
            // access the fields of that value
            unsafe {
                let mut v = glam::Vec3::ZERO;
                let mut va = glam::Vec3A::ZERO;
                for i in 0..3 {
                    bms_ffi_vec3_set((&raw mut v).cast(), i as i32, value(i));
                    bms_ffi_vec3a_set((&raw mut va).cast(), i as i32, value(i));
                    assert_eq!((v[i], va[i]), (value(i), value(i)), "C wrote element {i}");
                    v[i] = -value(i);
                    va[i] = -value(i);
                    assert_eq!(
                        (
                            bms_ffi_vec3_get((&raw const v).cast(), i as i32),
                            bms_ffi_vec3a_get((&raw const va).cast(), i as i32)
                        ),
                        (-value(i), -value(i)),
                        "C read element {i}"
                    );
                }

                let mut m = glam::Mat3::ZERO;
                for col in 0..3 {
                    for row in 0..3 {
                        let i = col * 3 + row;
                        bms_ffi_mat3_set((&raw mut m).cast(), col as i32, row as i32, value(i));
                        assert_eq!(m.col(col)[row], value(i), "C wrote column {col} row {row}");
                        m.col_mut(col)[row] = -value(i);
                        assert_eq!(
                            bms_ffi_mat3_get((&raw const m).cast(), col as i32, row as i32),
                            -value(i),
                            "C read column {col} row {row}"
                        );
                    }
                }
            }
        }

        #[test]
        fn c_values_are_valid_rust_values() {
            // SAFETY: the C side allocates with the overlay's size and alignment, which equal the real type's
            unsafe {
                let ptr = bms_ffi_vec3a_new(value(0), value(1), value(2));
                assert!(!ptr.is_null());
                let real = ptr.cast::<glam::Vec3A>();
                assert!(real.is_aligned(), "C allocation is misaligned for Vec3A");
                assert_eq!(*real, glam::Vec3A::new(value(0), value(1), value(2)));
                // SIMD math on the C allocation
                assert_eq!((*real).dot(glam::Vec3A::X), value(0));
                bms_ffi_vec3a_free(ptr);
            }
        }
    }
}
