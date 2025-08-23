extern crate bevy_mod_scripting;
extern crate script_integration_test_harness;
extern crate test_utils;
use bevy_platform::collections::HashMap;
use std::{path::PathBuf, sync::LazyLock, time::Duration};

use bevy::{
    log::{
        Level, tracing, tracing::span, tracing_subscriber, tracing_subscriber::layer::SubscriberExt,
    },
    reflect::Reflect,
};
use bevy_mod_scripting_core::bindings::{
    FromScript, IntoScript, Mut, Ref, ReflectReference, ScriptValue, Val,
};
use criterion::{
    BatchSize, BenchmarkFilter, BenchmarkGroup, Criterion, criterion_main, measurement::Measurement,
};
use regex::Regex;
use script_integration_test_harness::{
    make_test_lua_plugin, make_test_rhai_plugin, perform_benchmark_with_generator,
    run_lua_benchmark, run_plugin_script_load_benchmark, run_rhai_benchmark,
    test_functions::rand::Rng,
};
use test_utils::{Test, discover_all_tests};

static ENABLE_PROFILING: LazyLock<bool> =
    LazyLock::new(|| std::env::var("ENABLE_PROFILING").is_ok());

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
        let path = self.script_asset_path.to_string_lossy();
        let path = path.split("benchmarks").collect::<Vec<&str>>()[1]
            .replace(std::path::MAIN_SEPARATOR, "/");
        let first_folder = path.split("/").collect::<Vec<&str>>()[1];
        first_folder.replace("_", " ")
    }

    fn benchmark_name(&self) -> String {
        // use just the file stem
        let name = self
            .script_asset_path
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
                &self.script_asset_path.to_string_lossy(),
                &self.benchmark_name(),
                criterion,
            )
            .expect("Benchmark failed"),
            test_utils::TestKind::Rhai => run_rhai_benchmark(
                &self.script_asset_path.to_string_lossy(),
                &self.benchmark_name(),
                criterion,
            )
            .expect("benchmark failed"),
        }
    }
}

fn script_benchmarks(criterion: &mut Criterion, filter: Option<Regex>) {
    // find manifest dir
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tests = discover_all_tests(manifest_dir, |p| {
        p.script_asset_path.starts_with("benchmarks")
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

/// benchmarks measuring conversion time for script values and other things
fn conversion_benchmarks(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("conversions");

    #[derive(Reflect)]
    struct ReflectyVal(pub u32);

    perform_benchmark_with_generator(
        "ScriptValue::List",
        &|rng, _| {
            let mut array = Vec::new();
            for _ in 0..10 {
                array.push(ScriptValue::Integer(rng.random()));
            }
            ScriptValue::List(array)
        },
        &|w, i| {
            let i = i.into_script(w.clone()).unwrap();
            let _ = Vec::<ScriptValue>::from_script(i, w).unwrap();
        },
        &mut group,
        BatchSize::SmallInput,
    );

    perform_benchmark_with_generator(
        "ScriptValue::Map",
        &|rng, _| {
            let mut map = HashMap::default();
            for _ in 0..10 {
                map.insert(
                    rng.random::<u32>().to_string(),
                    ScriptValue::Integer(rng.random()),
                );
            }
            ScriptValue::Map(map)
        },
        &|w, i| {
            let i = i.into_script(w.clone()).unwrap();
            let _ = HashMap::<String, ScriptValue>::from_script(i, w).unwrap();
        },
        &mut group,
        BatchSize::SmallInput,
    );

    perform_benchmark_with_generator(
        "ScriptValue::Reference::from_into",
        &|rng, world| {
            let allocator = world.allocator();
            let mut allocator = allocator.write();
            ReflectReference::new_allocated(ReflectyVal(rng.random()), &mut allocator)
        },
        &|w, i| {
            let i = i.into_script(w.clone()).unwrap();
            let _ = ReflectReference::from_script(i, w).unwrap();
        },
        &mut group,
        BatchSize::SmallInput,
    );

    perform_benchmark_with_generator(
        "Val<T>::from_into",
        &|rng, _| Val::new(ReflectyVal(rng.random::<u32>())),
        &|w, i| {
            let v = i.into_script(w.clone()).unwrap();
            Val::<ReflectyVal>::from_script(v, w).unwrap();
        },
        &mut group,
        BatchSize::SmallInput,
    );

    perform_benchmark_with_generator(
        "Ref<T>::from",
        &|rng, w| {
            Val::new(ReflectyVal(rng.random::<u32>()))
                .into_script(w)
                .unwrap()
        },
        &|w, i| {
            Ref::<ReflectyVal>::from_script(i, w).unwrap();
        },
        &mut group,
        BatchSize::SmallInput,
    );

    perform_benchmark_with_generator(
        "Mut<T>::from",
        &|rng, w| {
            Val::new(ReflectyVal(rng.random::<u32>()))
                .into_script(w)
                .unwrap()
        },
        &|w, i| {
            Mut::<ReflectyVal>::from_script(i, w).unwrap();
        },
        &mut group,
        BatchSize::SmallInput,
    );
}

fn script_load_benchmarks(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("loading");
    let reload_probability = 0.5;
    // lua
    let plugin = make_test_lua_plugin();
    let content = include_str!("../assets/macro_benchmarks/loading/empty.lua");
    run_plugin_script_load_benchmark(plugin, "empty Lua", content, &mut group, reload_probability);

    // rhai
    let plugin = make_test_rhai_plugin();
    let content = include_str!("../assets/macro_benchmarks/loading/empty.rhai");
    run_plugin_script_load_benchmark(
        plugin,
        "empty Rhai",
        content,
        &mut group,
        reload_probability,
    );
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

        script_benchmarks(&mut criterion, filter);
        conversion_benchmarks(&mut criterion);
        script_load_benchmarks(&mut criterion);
    });
}
criterion_main!(benches);
