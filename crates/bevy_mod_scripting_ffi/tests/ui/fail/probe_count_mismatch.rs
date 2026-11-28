//! Expect: `b` has 2 probe values, but there are 3 probes
#[path = "../support/foreign.rs"]
mod foreign;
use bevy_mod_scripting_ffi::unsafe_c_overlay;

unsafe_c_overlay! {
    #[probe(foreign::Flags::new(true, false, false))]
    #[probe(foreign::Flags::new(false, true, false))]
    #[probe(foreign::Flags::new(false, false, true))]
    foreign::Flags as FlagsC {
        #[probe(true, false, false)] a: bool,
        #[probe(false, true)] b: bool,
        #[probe(false, false, true)] c: bool,
    }
}

fn main() {}
