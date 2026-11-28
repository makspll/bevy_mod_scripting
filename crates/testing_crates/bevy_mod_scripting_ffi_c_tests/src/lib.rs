//! Round trips of `bevy_mod_scripting_ffi` overlays through real C code.
//!
//! `c/overlays.h` is generated from the overlays and committed (rerun the tests with `BLESS_MODE=1` to update it),
//! the build script compiles `c/round_trip.c` against it, so its `_Static_assert`s check the C layout equals the Rust
//! one. The tests then read and write Rust values from C. On Linux the C code runs with UBSan, and CI also runs these
//! tests under valgrind (`cargo xtask valgrind`).

use bevy_mod_scripting_ffi::{CDecl, FfiField};

/// The overlays declared in `c/overlays.h`.
pub fn roots() -> [CDecl; 3] {
    [
        <glam::Vec3 as FfiField>::C_DECL,
        <glam::Vec3A as FfiField>::C_DECL,
        <glam::Mat3 as FfiField>::C_DECL,
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
    mod round_trip {
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
