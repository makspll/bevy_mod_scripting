use bevy_analyzer::*;
use log::debug;

fn main() {
    env_logger::init();
    debug!("cli entrypoint");
    rustc_plugin::cli_main(BevyAnalyzer);
}
