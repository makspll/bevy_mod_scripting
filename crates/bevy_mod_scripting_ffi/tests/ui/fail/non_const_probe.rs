//! Expect: cannot call non-const function in constants
#[path = "../support/foreign.rs"]
mod foreign;
use bevy_mod_scripting_ffi::unsafe_c_overlay;

fn make() -> foreign::Vec3 {
    foreign::Vec3::new(1.1, 2.2, 3.3)
}

unsafe_c_overlay! {
    #[probe(make())]
    foreign::Vec3 as Vec3C {
        #[probe = 1.1] x: f32,
        #[probe = 2.2] y: f32,
        #[probe = 3.3] z: f32,
    }
}

fn main() {}
