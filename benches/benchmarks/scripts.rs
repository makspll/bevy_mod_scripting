//! Benchmarks defined by scripts under `assets/benchmarks`, each script's `bench` function is the measured routine
use std::path::{Path, PathBuf};

use bevy::log::{Level, tracing::span};
use bevy_platform::collections::HashMap;
use criterion::{BenchmarkGroup, Criterion, measurement::Measurement};
use regex::Regex;
#[cfg(feature = "lua")]
use script_integration_test_harness::run_lua_benchmark;
#[cfg(feature = "rune")]
use script_integration_test_harness::run_rune_benchmark;
use test_utils::{Test, TestKind, discover_all_tests};

use crate::LUA;

/// Scripts driven by the benchmarks in [`crate::plugin`], which name them in Rust
const PLUGIN_SCRIPTS: &str = "benchmarks/plugin";

pub trait BenchmarkExecutor {
    fn benchmark_group(&self) -> String;
    fn benchmark_name(&self) -> String;
    fn execute<M: Measurement>(&self, criterion: &mut BenchmarkGroup<M>);
}

/// Reads the `group/name` a benchmark script declares on its first line, i.e. `-- #bench interop/field get` in Lua or
/// `// #bench interop/field get` in Rune. The language is appended to the name by [`BenchmarkExecutor::benchmark_name`].
fn bench_header(script_asset_path: &Path) -> (String, String) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(script_asset_path);
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("could not read {}: {e}", path.display()));
    content
        .lines()
        .next()
        .and_then(|l| {
            l.trim_start_matches(['-', '/'])
                .trim()
                .strip_prefix("#bench ")
        })
        .and_then(|header| header.split_once('/'))
        .map(|(group, name)| (group.trim().to_owned(), name.trim().to_owned()))
        .unwrap_or_else(|| {
            panic!(
                "{} must start with a `#bench group/name` header",
                path.display()
            )
        })
}

impl BenchmarkExecutor for Test {
    fn benchmark_group(&self) -> String {
        bench_header(&self.script_asset_path).0
    }

    fn benchmark_name(&self) -> String {
        let (_, name) = bench_header(&self.script_asset_path);
        let language = match self.kind {
            TestKind::Lua => LUA,
            TestKind::Rune => "Rune",
        };
        format!("{name} {language}")
    }

    fn execute<M: Measurement>(&self, criterion: &mut BenchmarkGroup<M>) {
        match self.kind {
            #[cfg(feature = "lua")]
            test_utils::TestKind::Lua => run_lua_benchmark(
                &self.script_asset_path.to_string_lossy(),
                &self.benchmark_name(),
                criterion,
            )
            .expect("Benchmark failed"),
            #[cfg(not(feature = "lua"))]
            test_utils::TestKind::Lua => unimplemented!("Lua benchmarks need a Lua feature"),
            #[cfg(feature = "rune")]
            test_utils::TestKind::Rune => run_rune_benchmark(
                &self.script_asset_path.to_string_lossy(),
                &self.benchmark_name(),
                criterion,
            )
            .expect("Benchmark failed"),
            #[cfg(not(feature = "rune"))]
            test_utils::TestKind::Rune => unimplemented!("Rune benchmarks need the `rune` feature"),
        }
    }
}

pub fn script_benchmarks(criterion: &mut Criterion, filter: Option<Regex>) {
    // find manifest dir
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tests = discover_all_tests(manifest_dir, |p| {
        p.script_asset_path.starts_with("benchmarks")
            && !p.script_asset_path.starts_with(PLUGIN_SCRIPTS)
            // Rune scripts are only discovered with the `rune` feature, Lua scripts always are
            && (cfg!(feature = "lua") || matches!(p.kind, TestKind::Rune))
            && if let Some(filter) = &filter {
                let matching = filter.is_match(&p.benchmark_name());
                if !matching {
                    println!(
                        "Skipping benchmark: '{}'. due to filter: '{filter}'",
                        p.benchmark_name()
                    );
                };
                matching
            } else {
                true
            }
    });

    // group by benchmark group
    let mut grouped: HashMap<String, Vec<Test>> =
        tests.into_iter().fold(HashMap::default(), |mut acc, t| {
            acc.entry(t.benchmark_group()).or_default().push(t);
            acc
        });

    // sort within groups by benchmark name
    for (_, tests) in grouped.iter_mut() {
        tests.sort_by_key(|a| a.benchmark_name());
    }

    // debug
    println!(
        "{}",
        grouped
            .iter()
            .map(|(k, v)| {
                format!(
                    "Group: {k}, Tests: {}",
                    v.iter()
                        .map(|t| t.benchmark_name())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );

    for (group, tests) in grouped {
        println!("Running benchmarks for group: {group}");
        let mut benchmark_group = criterion.benchmark_group(group);

        for t in tests {
            println!("Running benchmark: {}", t.benchmark_name());
            span!(
                Level::INFO,
                "Benchmark harness for test",
                test_name = &t.benchmark_name()
            );
            t.execute(&mut benchmark_group);
        }

        benchmark_group.finish();
    }
}
