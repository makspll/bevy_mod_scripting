//! Expect: `#[probe(<const constructor>)]` is required
#[path = "../support/foreign.rs"]
mod foreign;
use bevy_mod_scripting_ffi::unsafe_c_overlay;

unsafe_c_overlay! {
    foreign::Vec3 as Vec3C {
        #[probe = 1.1] x: f32,
        #[probe = 2.2] y: f32,
        #[probe = 3.3] z: f32,
    }
}

fn main() {}
