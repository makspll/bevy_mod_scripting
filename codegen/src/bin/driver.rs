#![feature(rustc_private)]
use bevy_mod_scripting_codegen::{driver::driver_main, *};

fn main() {
    // initially set it to high so no logs are missed, but later when we parse the args we will set it to the correct level
    unsafe { std::env::set_var("RUST_LOG", "trace") };
    env_logger::init();
    driver_main(BevyAnalyzer);
}
