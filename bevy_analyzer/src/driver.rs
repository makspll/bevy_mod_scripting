#![feature(rustc_private)]
use std::{env, io, str::FromStr};

use bevy_analyzer::*;
use env_logger::Env;
use log::LevelFilter;
use rustc_session::{config::ErrorOutputType, EarlyDiagCtxt};
extern crate rustc_driver;
extern crate rustc_session;

fn main() {
    env_logger::init();
    rustc_plugin::driver_main(BevyAnalyzer);
}
