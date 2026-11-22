extern crate bevy_mod_scripting;
extern crate script_integration_test_harness;
extern crate test_utils;
use std::{sync::LazyLock, time::Duration};

use bevy::log::{tracing, tracing_subscriber, tracing_subscriber::layer::SubscriberExt};
use criterion::{BenchmarkFilter, criterion_main};
use regex::Regex;

#[cfg(feature = "lua")]
mod conversions;
mod plugin;
mod scripts;

static ENABLE_PROFILING: LazyLock<bool> =
    LazyLock::new(|| std::env::var("ENABLE_PROFILING").is_ok());

/// The language suffix of Lua benchmark names, naming the dialect these benchmarks were built with
const LUA: &str = if cfg!(feature = "luajit") {
    "LuaJIT"
} else if cfg!(feature = "luajit52") {
    "LuaJIT52"
} else if cfg!(feature = "luau") {
    "Luau"
} else if cfg!(feature = "lua51") {
    "Lua51"
} else if cfg!(feature = "lua52") {
    "Lua52"
} else if cfg!(feature = "lua53") {
    "Lua53"
} else {
    "Lua54"
};

fn maybe_with_profiler(f: impl Fn(bool)) {
    if *ENABLE_PROFILING {
        println!(
            "profiling enabled, make sure to run tracy. If using it across windows/WSL you can use something like `tracy-capture.exe -o output.tracy -a localhost` on windows"
        );
        // set global tracing subscriber so bevy doesn't set it itself first
        let subscriber = tracing_subscriber::Registry::default();
        let tracy_layer = tracing_tracy::TracyLayer::default();

        let subscriber = subscriber.with(tracy_layer);

        tracing::subscriber::set_global_default(subscriber).unwrap();

        f(true);
    } else {
        f(false);
    }
}

pub fn benches() {
    maybe_with_profiler(|_profiler| {
        let mut criterion: criterion::Criterion<_> = (criterion::Criterion::default())
            .configure_from_args()
            .measurement_time(Duration::from_secs(10));
        let arguments = std::env::args()
            .skip(1) // the executable name
            .filter(|a| !a.starts_with("-"))
            .collect::<Vec<String>>();

        // take first argument as .*<val>.* regex for benchmarks
        // criterion will already have that as a filter, but we want to make sure we're on the same page
        let filter = if let Some(n) = arguments.first() {
            println!("using filter: '{n}'");
            let regex = Regex::new(n).unwrap();
            let filter = BenchmarkFilter::Regex(regex.clone());
            criterion = criterion.with_benchmark_filter(filter);
            Some(regex)
        } else {
            None
        };

        scripts::script_benchmarks(&mut criterion, filter);
        // named after the Lua dialect, so the Lua builds measure them
        #[cfg(feature = "lua")]
        conversions::conversion_benchmarks(&mut criterion);
        plugin::script_load_benchmarks(&mut criterion);
        plugin::callback_benchmarks(&mut criterion);
        #[cfg(feature = "lua")]
        plugin::script_system_benchmarks(&mut criterion);
    });
}
criterion_main!(benches);
