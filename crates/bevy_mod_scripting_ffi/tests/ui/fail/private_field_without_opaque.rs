//! Expect: field `id` of struct `Handle` is private (unreachable fields need `#[opaque]`)
#[path = "../support/foreign.rs"]
mod foreign;
use bevy_mod_scripting_ffi::unsafe_c_overlay;

unsafe_c_overlay! {
    #[probe(foreign::Handle::new(0x0102_0304, 0x0506_0708))]
    foreign::Handle as HandleC {
        #[probe = 0x0102_0304] id: u32,
        #[opaque] #[probe = 0x0506_0708] generation: u32,
    }
}

fn main() {}
