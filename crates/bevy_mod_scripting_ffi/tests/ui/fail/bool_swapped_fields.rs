//! Expect: probe failed: `FlagsC::b` expected `false` in probe 0
//! (`a` and `b` swapped: one probe can't tell bools apart, one probe per field can)
#[path = "../support/foreign.rs"]
mod foreign;
use bevy_mod_scripting_ffi::unsafe_c_overlay;

unsafe_c_overlay! {
    #[probe(foreign::Flags::new(true, false, false))]
    #[probe(foreign::Flags::new(false, true, false))]
    #[probe(foreign::Flags::new(false, false, true))]
    foreign::Flags as FlagsC {
        #[probe(false, true, false)] b: bool,
        #[probe(true, false, false)] a: bool,
        #[probe(false, false, true)] c: bool,
    }
}

fn main() {}
