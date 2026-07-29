#![feature(rustc_private)]
use std::process::ExitCode;

use bevy_mod_scripting_codegen::{*};
use bevy_mod_scripting_rustc_driver::driver_main;
// use rustc_log::LoggerConfig;

extern crate rustc_log;

fn main() -> ExitCode {
    pretty_env_logger::formatted_builder()
        .parse_write_style("always")
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "warn".into()))
        .init();

    driver_main(BevyAnalyzer)
}
