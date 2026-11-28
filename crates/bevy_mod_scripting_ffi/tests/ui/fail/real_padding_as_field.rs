//! Expect: using uninitialized data (byte 1 of Foo is real padding, probing it reads uninit memory)
#[path = "../support/foreign.rs"]
mod foreign;
use bevy_mod_scripting_ffi::unsafe_c_overlay;

unsafe_c_overlay! {
    #[probe(foreign::Foo::new(0x11, 0x2233_4455))]
    foreign::Foo as FooC {
        #[probe = 0x11] a: u8,
        #[probe = 0x22] p: u8,
        #[pad] _p0: [u8; 2],
        #[probe = 0x2233_4455] b: u32,
    }
}

fn main() {}
