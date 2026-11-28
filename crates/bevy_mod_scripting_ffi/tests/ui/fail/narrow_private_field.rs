//! Expect: probe failed: HandleC::id expected 0x0102
//! (`id` is a private u32; the u16 reads its low half 0x0304, the high half hides under `_p`)
#[path = "../support/foreign.rs"]
mod foreign;
use bevy_mod_scripting_ffi::unsafe_c_overlay;

unsafe_c_overlay! {
    #[probe(foreign::Handle::new(0x0102_0304, 0x0506_0708))]
    foreign::Handle as HandleC {
        #[opaque] #[probe = 0x0102] id: u16,
        #[pad] _p: [u8; 2],
        #[opaque] #[probe = 0x0506_0708] generation: u32,
    }
}

fn main() {}
