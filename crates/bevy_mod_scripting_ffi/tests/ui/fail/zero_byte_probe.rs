//! Expect: probe value for x has a zero byte (1.0f32 is 0x3F800000)
#[path = "../support/foreign.rs"]
mod foreign;
use bevy_mod_scripting_ffi::unsafe_c_overlay;

unsafe_c_overlay! {
    #[probe(foreign::Vec3::new(1.0, 2.2, 3.3))]
    foreign::Vec3 as Vec3C {
        #[probe = 1.0] x: f32,
        #[probe = 2.2] y: f32,
        #[probe = 3.3] z: f32,
    }
}

fn main() {}
