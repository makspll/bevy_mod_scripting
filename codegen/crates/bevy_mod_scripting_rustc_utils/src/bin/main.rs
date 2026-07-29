#![feature(rustc_private)]
use std::{
    env,
};

use bevy_mod_scripting_rustc_driver::cli_main;
use bevy_mod_scripting_rustc_utils::plugin::RustcUtilsPlugin;
use log::info;

fn main() {

    if env::var("RUST_LOG").is_err() {
        unsafe { env::set_var("RUST_LOG", "info") };
    }
    pretty_env_logger::init();

    info!("Using RUST_LOG: {:?}", env::var("RUST_LOG"));

    let metadata = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .other_options(["--all-features".to_string(), "--offline".to_string()])
        .exec()
        .unwrap();
    cli_main(
        RustcUtilsPlugin {},
        [].into(),
        &metadata,
    );
}
