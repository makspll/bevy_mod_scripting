//! Expect: probe value for y also occurs elsewhere (in z), and for a (inside c, which hides the wrong order)
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

unsafe_c_overlay! {
    #[probe(foreign::Mixed::new(0x11, 0x22, u16::from_ne_bytes([0x11, 0x22])))]
    foreign::Mixed as MixedC {
        #[probe = 0x11] a: u8,
        #[probe = 0x22] b: u8,
        #[probe = u16::from_ne_bytes([0x11, 0x22])] c: u16,
    }
}

fn main() {}
