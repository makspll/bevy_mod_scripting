//! Expect: probe values for y and z are equal
#[path = "../support/foreign.rs"]
mod foreign;
use bevy_mod_scripting_ffi::unsafe_c_overlay;

unsafe_c_overlay! {
    #[probe(foreign::Vec3::new(1.1, 2.2, 2.2))]
    foreign::Vec3 as Vec3C {
        #[probe = 1.1] x: f32,
        #[probe = 2.2] y: f32,
        #[probe = 2.2] z: f32,
    }
}

fn main() {}
