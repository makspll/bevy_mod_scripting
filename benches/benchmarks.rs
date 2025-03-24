use std::{path::PathBuf, time::Duration};

use bevy::utils::HashMap;
use criterion::{criterion_main, measurement::Measurement, BenchmarkGroup, Criterion};
use script_integration_test_harness::{run_lua_benchmark, run_rhai_benchmark};
use test_utils::{discover_all_tests, Test};

extern crate bevy_mod_scripting;
extern crate script_integration_test_harness;
extern crate test_utils;

pub trait BenchmarkExecutor {
    fn benchmark_group(&self) -> String;
    fn benchmark_name(&self) -> String;
    fn execute<M: Measurement>(&self, criterion: &mut BenchmarkGroup<M>);
}

impl BenchmarkExecutor for Test {
    fn benchmark_group(&self) -> String {
        // we want to use OS agnostic paths
        // use the file path from `benchmarks` onwards using folders as groupings
        // replace file separators with `/`
        // replace _ with spaces
        let path = self.path.to_string_lossy();
        let path = path.split("benchmarks").collect::<Vec<&str>>()[1]
            .replace(std::path::MAIN_SEPARATOR, "/");
        let first_folder = path.split("/").collect::<Vec<&str>>()[1];
        first_folder.replace("_", " ")
    }

    fn benchmark_name(&self) -> String {
        // use just the file stem
        let name = self
            .path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .replace("_", " ");

        let language = self.kind.to_string();

        format!("{name} {language}")
    }

    fn execute<M: Measurement>(&self, criterion: &mut BenchmarkGroup<M>) {
        match self.kind {
            test_utils::TestKind::Lua => run_lua_benchmark(
                &self.path.to_string_lossy(),
                &self.benchmark_name(),
                criterion,
            )
            .expect("Benchmark failed"),
            test_utils::TestKind::Rhai => run_rhai_benchmark(
                &self.path.to_string_lossy(),
                &self.benchmark_name(),
                criterion,
            )
            .expect("benchmark failed"),
        }
    }
}

fn script_benchmarks(criterion: &mut Criterion) {
    // find manifest dir
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tests = discover_all_tests(manifest_dir, |p| p.starts_with("benchmarks"));

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

    for (group, tests) in grouped {
        let mut benchmark_group = criterion.benchmark_group(group);

        for t in tests {
            t.execute(&mut benchmark_group);
        }

        benchmark_group.finish();
    }
}

pub fn benches() {
    let mut criterion: criterion::Criterion<_> = (criterion::Criterion::default())
        .configure_from_args()
        .measurement_time(Duration::from_secs(10));
    script_benchmarks(&mut criterion);
}
criterion_main!(benches);
