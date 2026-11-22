use std::collections::VecDeque;

use bevy::reflect::Reflect;
use bevy_mod_scripting_bindings::{FromScript, IntoScript, M, R, ReflectReference, ScriptValue, V};
use bevy_platform::collections::HashMap;
use criterion::{BatchSize, Criterion};
use script_integration_test_harness::{
    perform_benchmark_with_generator, test_functions::rand::Rng,
};

use crate::LUA;

/// benchmarks measuring conversion time for script values and other things, named after the Lua dialect of the build
/// since `cargo xtask bench` runs them in every build
pub fn conversion_benchmarks(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("conversions");

    #[derive(Reflect)]
    struct ReflectyVal(pub u32);

    perform_benchmark_with_generator(
        &format!("ScriptValue::List {LUA}"),
        &|rng, _| {
            let mut array = VecDeque::new();
            for _ in 0..10 {
                array.push_back(ScriptValue::Integer(rng.random()));
            }
            ScriptValue::List(array)
        },
        &|w, i| {
            let i = i.into_script(w.clone()).unwrap();
            let _ = w.with_call_scope(|s| Vec::<ScriptValue>::from_script(i, s).unwrap());
        },
        &mut group,
        BatchSize::SmallInput,
    );

    perform_benchmark_with_generator(
        &format!("ScriptValue::Map {LUA}"),
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
            let _ =
                w.with_call_scope(|s| HashMap::<String, ScriptValue>::from_script(i, s).unwrap());
        },
        &mut group,
        BatchSize::SmallInput,
    );

    perform_benchmark_with_generator(
        &format!("ScriptValue::Reference::from_into {LUA}"),
        &|rng, _world| ReflectReference::new_allocated(ReflectyVal(rng.random())),
        &|w, i| {
            let i = i.into_script(w.clone()).unwrap();
            let _ = w.with_call_scope(|s| ReflectReference::from_script(i, s).unwrap());
        },
        &mut group,
        BatchSize::SmallInput,
    );

    perform_benchmark_with_generator(
        &format!("Val<T>::from_into {LUA}"),
        &|rng, _| V::new(ReflectyVal(rng.random::<u32>())),
        &|w, i| {
            let v = i.into_script(w.clone()).unwrap();
            w.with_call_scope(|s| V::<ReflectyVal>::from_script(v, s).unwrap());
        },
        &mut group,
        BatchSize::SmallInput,
    );

    perform_benchmark_with_generator(
        &format!("Ref<T>::from {LUA}"),
        &|rng, w| {
            V::new(ReflectyVal(rng.random::<u32>()))
                .into_script(w)
                .unwrap()
        },
        &|w, i| {
            w.with_call_scope(|s| {
                R::<ReflectyVal>::from_script(i, s).unwrap();
            });
        },
        &mut group,
        BatchSize::SmallInput,
    );

    perform_benchmark_with_generator(
        &format!("Mut<T>::from {LUA}"),
        &|rng, w| {
            V::new(ReflectyVal(rng.random::<u32>()))
                .into_script(w)
                .unwrap()
        },
        &|w, i| {
            w.with_call_scope(|s| {
                M::<ReflectyVal>::from_script(i, s).unwrap();
            });
        },
        &mut group,
        BatchSize::SmallInput,
    );
}
