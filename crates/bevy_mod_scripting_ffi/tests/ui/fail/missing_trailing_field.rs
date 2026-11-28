//! Expect: fields + pad != size_of::<T>() (12 of 16 bytes declared)
#[path = "../support/foreign.rs"]
mod foreign;
use bevy_mod_scripting_ffi::unsafe_c_overlay;

unsafe_c_overlay! {
    #[probe(foreign::Vec3A::new(1.1, 2.2, 3.3))]
    foreign::Vec3A as Vec3AC {
        #[probe = 1.1] x: f32,
        #[probe = 2.2] y: f32,
        #[probe = 3.3] z: f32,
    }
}

fn main() {}
