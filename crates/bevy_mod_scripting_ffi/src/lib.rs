//! Compile-time verified `#[repr(C)]` overlays over foreign Rust types.
//!
//! An overlay is a C-compatible mirror of a type we don't control (e.g. `glam::Vec3`), which scripts can
//! read directly (e.g. through a LuaJIT `ffi.cdef`). Only types made of C scalars (and other overlays) are supported;
//! everything else goes through reflection.
//!
//! ```rust
//! # mod foreign {
//! #     #[repr(C)] #[derive(Clone, Copy)] pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }
//! #     impl Vec3 { pub const fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } } }
//! # }
//! use bevy_mod_scripting_ffi::{unsafe_c_overlay, FfiField};
//!
//! unsafe_c_overlay! {
//!     // SAFETY: Vec3 is three f32s at 0/4/8
//!     #[probe(foreign::Vec3::new(1.1, 2.2, 3.3))]
//!     foreign::Vec3 as Vec3C {
//!         #[probe = 1.1] x: f32,
//!         #[probe = 2.2] y: f32,
//!         #[probe = 3.3] z: f32,
//!     }
//! }
//!
//! // the overlay is a C struct with the same bytes as the real type
//! let c: <foreign::Vec3 as FfiField>::Overlay = unsafe { core::mem::transmute(foreign::Vec3::new(1.0, 2.0, 3.0)) };
//! assert_eq!((c.x, c.y, c.z), (1.0, 2.0, 3.0));
//! ```
//!
//! # Safety contract
//!
//! By invoking `unsafe_c_overlay!` you promise:
//! 1. every declared field lives inside the bytes of the type, at the offset implied by declaration order, with the declared type
//! 2. `#[pad]` bytes hold nothing a probed field depends on
//! 3. `#[probe = _]` fields are valid for any value of the type
//!
//! Everything else is checked at compile time:
//!
//! | Rule | Check |
//! |---|---|
//! | field types are C scalars or overlays | `F: FfiField` |
//! | fields aligned, no overlap, all bytes covered, align matches | const layout builder |
//! | fields have the declared type | `let _: &F = &value.field` (works through `Deref`), skipped for `#[opaque]` fields (private or unnamed) |
//! | overlay alignment equals the real type's | `align_of::<Overlay>() == align_of::<T>()`, so overlays made outside Rust (e.g. `ffi.new`) can be read as `T` |
//! | offsets and sizes match the real type | probe: transmute the probe value into the overlay, compare each field's bytes |
//! | probes are strong | every byte of every probe value is non-zero, values are pairwise distinct |
//! | every field is probed | `#[probe(..)]` on the type, `#[probe = ..]` (or `_`) on each non-pad field |
//!
//! Fields in the wrong order fail the probe:
//! ```compile_fail
//! # mod foreign {
//! #     #[repr(C)] #[derive(Clone, Copy)] pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }
//! #     impl Vec3 { pub const fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } } }
//! # }
//! # use bevy_mod_scripting_ffi::unsafe_c_overlay;
//! unsafe_c_overlay! {
//!     #[probe(foreign::Vec3::new(1.1, 2.2, 3.3))]
//!     foreign::Vec3 as Vec3C {
//!         #[probe = 1.1] x: f32,
//!         #[probe = 3.3] z: f32,
//!         #[probe = 2.2] y: f32,
//!     }
//! }
//! ```
//!
//! Undeclared padding fails the layout check:
//! ```compile_fail
//! # mod foreign {
//! #     #[repr(C)] #[derive(Clone, Copy)] pub struct Foo { pub a: u8, pub b: u32 }
//! #     impl Foo { pub const fn new(a: u8, b: u32) -> Self { Self { a, b } } }
//! # }
//! # use bevy_mod_scripting_ffi::unsafe_c_overlay;
//! unsafe_c_overlay! {
//!     #[probe(foreign::Foo::new(0x11, 0x2233_4455))]
//!     foreign::Foo as FooC {
//!         #[probe = 0x11] a: u8,
//!         #[probe = 0x2233_4455] b: u32,
//!     }
//! }
//! ```
//!
//! A too-narrow private field hidden by padding fails the probe, because probe values have no zero bytes:
//! ```compile_fail
//! # mod foreign {
//! #     #[repr(C)] #[derive(Clone, Copy)] pub struct Handle { id: u32, generation: u32 }
//! #     impl Handle { pub const fn new(id: u32, generation: u32) -> Self { Self { id, generation } } }
//! # }
//! # use bevy_mod_scripting_ffi::unsafe_c_overlay;
//! unsafe_c_overlay! {
//!     #[probe(foreign::Handle::new(0x0102_0304, 0x0506_0708))]
//!     foreign::Handle as HandleC {
//!         #[opaque] #[probe = 0x0102] id: u16,
//!         #[pad] _p: [u8; 2],
//!         #[opaque] #[probe = 0x0506_0708] generation: u32,
//!     }
//! }
//! ```
//!
//! Probe values with a zero byte are rejected (`1.0f32` is `0x3F800000`):
//! ```compile_fail
//! # mod foreign {
//! #     #[repr(C)] #[derive(Clone, Copy)] pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }
//! #     impl Vec3 { pub const fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } } }
//! # }
//! # use bevy_mod_scripting_ffi::unsafe_c_overlay;
//! unsafe_c_overlay! {
//!     #[probe(foreign::Vec3::new(1.0, 2.2, 3.3))]
//!     foreign::Vec3 as Vec3C {
//!         #[probe = 1.0] x: f32,
//!         #[probe = 2.2] y: f32,
//!         #[probe = 3.3] z: f32,
//!     }
//! }
//! ```
//!
//! Fields must be C scalars or overlays:
//! ```compile_fail
//! # mod foreign {
//! #     #[derive(Clone)] pub struct Named { pub name: String }
//! #     impl Named { pub const fn new() -> Self { Self { name: String::new() } } }
//! # }
//! # use bevy_mod_scripting_ffi::unsafe_c_overlay;
//! unsafe_c_overlay! {
//!     #[probe(foreign::Named::new())]
//!     foreign::Named as NamedC {
//!         #[probe = _] name: String,
//!     }
//! }
//! ```

// lets `unsafe_c_overlay!` output (which names `::bevy_mod_scripting_ffi`) work inside this crate
extern crate self as bevy_mod_scripting_ffi;

mod impls;

pub use bevy_mod_scripting_derive::unsafe_c_overlay;

/// Const helpers used by `unsafe_c_overlay!` output.
#[doc(hidden)]
pub mod __private {
    pub use core::mem::{ManuallyDrop, MaybeUninit, align_of, size_of};

    /// The bytes of `value`, possibly uninitialized.
    pub const fn bytes<T>(value: &T) -> &[MaybeUninit<u8>] {
        // SAFETY: any initialized `T` is readable as `size_of::<T>()` possibly-uninitialized bytes
        unsafe {
            core::slice::from_raw_parts(
                (value as *const T).cast::<MaybeUninit<u8>>(),
                size_of::<T>(),
            )
        }
    }

    /// `a == b` on the bytes where `mask` is true; other bytes are never read.
    pub const fn masked_eq(a: &[MaybeUninit<u8>], b: &[MaybeUninit<u8>], mask: &[bool]) -> bool {
        let mut i = 0;
        while i < mask.len() {
            // SAFETY: `mask` marks initialized bytes
            if mask[i] && unsafe { a[i].assume_init() != b[i].assume_init() } {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Every byte where `mask` is true is non-zero.
    pub const fn all_nonzero(a: &[MaybeUninit<u8>], mask: &[bool]) -> bool {
        let mut i = 0;
        while i < mask.len() {
            // SAFETY: `mask` marks initialized bytes
            if mask[i] && unsafe { a[i].assume_init() } == 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Two probe values are equal: same size, and equal on the bytes initialized in both.
    pub const fn same_value(
        a: &[MaybeUninit<u8>],
        a_mask: &[bool],
        b: &[MaybeUninit<u8>],
        b_mask: &[bool],
    ) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut i = 0;
        while i < a.len() {
            // SAFETY: both masks mark initialized bytes
            if a_mask[i] && b_mask[i] && unsafe { a[i].assume_init() != b[i].assume_init() } {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Copies a field's mask into `mask` at `at`, returns the offset after it.
    /// Bytes past the end are dropped, an overlay too big for its type is reported by the layout check.
    pub const fn mask_field(mask: &mut [bool], at: usize, field: &[bool]) -> usize {
        let mut i = 0;
        while i < field.len() && at + i < mask.len() {
            mask[at + i] = field[i];
            i += 1;
        }
        at + field.len()
    }
}

/// A type usable as an overlay field: a C scalar, or a foreign type with a generated overlay.
///
/// # Safety
/// `Overlay` is a `#[repr(C)]` mirror of `Self` with the same size and alignment, and `INIT_MASK` has one entry per byte,
/// `true` exactly where every value of `Self` has an initialized byte. Implemented here for scalars and by `unsafe_c_overlay!`.
pub unsafe trait FfiField {
    /// The `#[repr(C)]` mirror (`Self` for scalars).
    type Overlay: Copy;
    /// Per-byte initialized mask, `false` over padding.
    const INIT_MASK: &'static [bool];
}

macro_rules! impl_scalar_field {
    ($($ty:ty),*) => {$(
        // SAFETY: C scalars have no padding, every byte is initialized
        unsafe impl FfiField for $ty {
            type Overlay = $ty;
            const INIT_MASK: &'static [bool] = &[true; size_of::<$ty>()];
        }
    )*};
}

impl_scalar_field!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
