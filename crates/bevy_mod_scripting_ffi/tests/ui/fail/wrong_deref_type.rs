//! Expect: mismatched types (field type check works through Deref: `y` is f32, declared u32)
#[path = "../support/foreign.rs"]
mod foreign;
use bevy_mod_scripting_ffi::unsafe_c_overlay;

unsafe_c_overlay! {
    #[probe(foreign::Vec3A::new(1.1, 2.2, 3.3))]
    foreign::Vec3A as Vec3AC {
        #[probe = 1.1] x: f32,
        #[probe = 0x0102_0304] y: u32,
        #[probe = 3.3] z: f32,
        #[opaque] #[probe = _] _w: f32,
    }
}

fn main() {}
