//! Benchmarks driving the whole scripting plugin, with scripts from `assets/benchmarks/plugin`
use criterion::Criterion;
#[cfg(feature = "rune")]
use script_integration_test_harness::make_test_rune_plugin;
use script_integration_test_harness::{
    make_test_lua_plugin, run_plugin_callback_command_benchmark,
    run_plugin_callback_event_benchmark, run_plugin_script_load_benchmark,
    run_plugin_script_system_benchmark,
};

use crate::LUA;

pub fn script_load_benchmarks(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("loading");
    // lua
    let content = include_str!("../../assets/benchmarks/plugin/loading/empty.lua");
    run_plugin_script_load_benchmark(
        make_test_lua_plugin,
        &format!("empty {LUA}"),
        content,
        &mut group,
    );

    // rune
    #[cfg(feature = "rune")]
    {
        let content = include_str!("../../assets/benchmarks/plugin/loading/empty.rn");
        run_plugin_script_load_benchmark(make_test_rune_plugin, "empty Rune", content, &mut group);
    }
}

/// benchmarks measuring the overhead of dispatching callbacks to scripts, the callbacks themselves are empty
pub fn callback_benchmarks(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("callbacks");
    let lua = include_str!("../../assets/benchmarks/plugin/callbacks/empty_callback.lua");
    #[cfg(feature = "rune")]
    let rune = include_str!("../../assets/benchmarks/plugin/callbacks/empty_callback.rn");

    for script_count in [1, 100] {
        run_plugin_callback_event_benchmark(
            make_test_lua_plugin,
            &format!("event handler {script_count} scripts {LUA}"),
            lua,
            script_count,
            &mut group,
        );
        #[cfg(feature = "rune")]
        run_plugin_callback_event_benchmark(
            make_test_rune_plugin,
            &format!("event handler {script_count} scripts Rune"),
            rune,
            script_count,
            &mut group,
        );
    }

    run_plugin_callback_command_benchmark(
        make_test_lua_plugin,
        &format!("command {LUA}"),
        lua,
        &mut group,
    );
    #[cfg(feature = "rune")]
    run_plugin_callback_command_benchmark(make_test_rune_plugin, "command Rune", rune, &mut group);
}

/// benchmarks running a script system querying the components of many entities, measures building and passing the payload
pub fn script_system_benchmarks(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("script systems");
    let lua = include_str!("../../assets/benchmarks/plugin/script_systems/query_system.lua");
    let lua_iterate =
        include_str!("../../assets/benchmarks/plugin/script_systems/query_system_iterate.lua");
    for entity_count in [10, 100, 1000] {
        run_plugin_script_system_benchmark(
            make_test_lua_plugin,
            &format!("query {entity_count} entities {LUA}"),
            lua,
            entity_count,
            &mut group,
        );
        run_plugin_script_system_benchmark(
            make_test_lua_plugin,
            &format!("query and iterate {entity_count} entities {LUA}"),
            lua_iterate,
            entity_count,
            &mut group,
        );
    }
}
