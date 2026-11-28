//! Expect: the trait bound `String: FfiField` is not satisfied
#[path = "../support/foreign.rs"]
mod foreign;
use bevy_mod_scripting_ffi::unsafe_c_overlay;

unsafe_c_overlay! {
    #[probe(foreign::Named::new())]
    foreign::Named as NamedC {
        #[probe = _] name: String,
    }
}

fn main() {}
