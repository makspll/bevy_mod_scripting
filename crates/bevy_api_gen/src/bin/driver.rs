use bevy_api_gen::*;

fn main() {
    env_logger::init();
    rustc_plugin::driver_main(BevyAnalyzer);
}
