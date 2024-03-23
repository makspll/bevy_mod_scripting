#![feature(rustc_private)]

use bevy_analyzer::*;
extern crate rustc_driver;
extern crate rustc_session;

fn main() {
    env_logger::init();
    rustc_plugin::driver_main(BevyAnalyzer);
}
