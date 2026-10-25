use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use ::{
    bevy_app::App,
    bevy_asset::Assets,
    bevy_ecs::{
        change_detection::Mut, component::ComponentId, entity::Entity, schedule::Schedules,
        world::World,
    },
    bevy_reflect::{Reflect, TypeRegistration},
};
use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::{
    DynamicScriptFunction, NamespaceMembers, ReflectReference, ScriptComponentRegistration,
    ScriptResourceRegistration, ScriptTypeRegistration, ScriptValue, WorldExtensions,
    error::InteropError,
    function::{
        from::{R, V},
        namespace::NamespaceBuilder,
        script_function::{DynamicScriptFunctionMut, FunctionCallContext},
    },
};
use bevy_system_reflection::{ReflectSchedule, ReflectSystemGraph};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;
use test_utils::test_data::{EnumerateTestComponents, TestAsset};

// lazy lock rng state
pub static RNG: std::sync::LazyLock<Mutex<ChaCha12Rng>> = std::sync::LazyLock::new(|| {
    let seed = [42u8; 32];
    Mutex::new(ChaCha12Rng::from_seed(seed))
});

pub use rand;

pub fn register_test_functions(world: &mut App) {
    let world = world.world_mut();
    NamespaceBuilder::<World>::new_unregistered(world)
        .register("_get_mock_type", |_s: FunctionCallContext| {
            #[derive(Reflect)]
            struct Dummy;
            let reg = ScriptTypeRegistration::new(Arc::new(TypeRegistration::of::<Dummy>()));
            ReflectReference::new_allocated(reg)
        })
        .register("_get_mock_component_type", |_s: FunctionCallContext| {
            #[derive(Reflect)]
            struct Dummy;
            let reg = ScriptTypeRegistration::new(Arc::new(TypeRegistration::of::<Dummy>()));
            let comp = ScriptComponentRegistration::new(reg, ComponentId::new(999999999999999));
            ReflectReference::new_allocated(comp)
        })
        .register("_get_mock_resource_type", |_s: FunctionCallContext| {
            #[derive(Reflect)]
            struct Dummy;
            let reg = ScriptTypeRegistration::new(Arc::new(TypeRegistration::of::<Dummy>()));
            let comp = ScriptResourceRegistration::new(reg, ComponentId::new(999999999999999));
            ReflectReference::new_allocated(comp)
        })
        .register("_sleep", |time: f64| {
            std::thread::sleep(std::time::Duration::from_secs_f64(time));
            Ok(())
        })
        .register(
            "_get_entity_with_test_component",
            |_s: FunctionCallContext, name: String| {
                World::enumerate_test_components()
                    .iter()
                    .find(|(n, _, _)| n.contains(&name))
                    .map(|(_, _, c)| {
                        ReflectReference::new_allocated(
                            c.unwrap_or(Entity::from_raw_u32(9999).unwrap()),
                        )
                    })
            },
        )
        .register(
            "_archetype_of",
            |s: FunctionCallContext, entity: V<Entity>| {
                let world = s.world()?;
                world
                    .with_world_access(|world| {
                        world
                            .get_entity(*entity)
                            .map(|entity| entity.archetype().id().index())
                            .ok()
                    })?
                    .ok_or_else(|| InteropError::string(format!("missing entity: {}", *entity)))
            },
        )
        .register(
            "_assert_system_runs_before",
            |s: FunctionCallContext,
             schedule: R<ReflectSchedule>,
             before: String,
             after: String| {
                assert_on_schedule_graph(
                    &s,
                    &schedule,
                    format!("Expected `{before}` to run before `{after}`"),
                    |graph| graph.system_runs_before(&before, &after),
                )
            },
        )
        .register(
            "_assert_systems_run_in_parallel",
            |s: FunctionCallContext, schedule: R<ReflectSchedule>, a: String, b: String| {
                assert_on_schedule_graph(
                    &s,
                    &schedule,
                    format!("Expected `{a}` and `{b}` to have no ordering between them"),
                    |graph| graph.systems_run_in_parallel(&a, &b),
                )
            },
        )
        .register(
            "_assert_throws",
            |_s: FunctionCallContext, f: DynamicScriptFunctionMut, reg: String| {
                let result = f.call(vec![], FunctionCallContext::new(Language::Unknown));
                let err = match result {
                    Ok(_) => {
                        return Err(InteropError::str(
                            "Expected function to throw error, but it did not.",
                        ));
                    }
                    Err(e) => format!("{e:#?}"),
                };

                let regex = regex::Regex::new(&reg).unwrap();
                if regex.is_match(&err) {
                    Ok(())
                } else {
                    Err(InteropError::string(format!(
                        "Expected error message to match the regex: \n{}\n\nBut got:\n{}",
                        regex.as_str(),
                        err
                    )))
                }
            },
        );

    // helpers of the script tests, `import_namespace("test_utils").noop()`
    NamespaceBuilder::<NamespaceMembers>::in_namespace(world, "test_utils")
        .register("_make_invalid_entity", |_s: FunctionCallContext| {
            let entity = Entity::from_raw_u32(u32::MAX - 2).unwrap();
            ReflectReference::new_allocated(entity)
        })
        .register("_make_placeholder_entity", |_s: FunctionCallContext| {
            let entity = Entity::PLACEHOLDER;
            ReflectReference::new_allocated(entity)
        })
        .register(
            "_entity_from_index",
            |_s: FunctionCallContext, index: u32| {
                let entity = Entity::from_raw_u32(index).unwrap();
                ReflectReference::new_allocated(entity)
            },
        )
        .register("global_hello_world", || Ok("hi!"))
        .register("random", |start: Option<u32>, end: Option<u32>| {
            let start = start.unwrap_or(0);
            let end = end.unwrap_or(1);
            let mut rng = RNG.lock().unwrap();
            rng.random_range::<u32, _>(start..=end)
        })
        .register("random_int", |start: Option<i32>, end: Option<i32>| {
            let start = start.unwrap_or(0);
            let end = end.unwrap_or(1);
            let mut rng = RNG.lock().unwrap();
            rng.random_range::<i32, _>(start..=end)
        })
        .register("reseed", || {
            let seed = [42u8; 32];
            let mut rng = RNG.lock().unwrap();
            *rng = ChaCha12Rng::from_seed(seed);
        })
        .register("make_hashmap", |map: HashMap<String, usize>| map)
        // identity functions over lists, used to check list order survives the script -> rust -> script round trip
        .register("make_vec_i64", |vec: Vec<i64>| vec)
        .register("make_vec_string", |vec: Vec<String>| vec)
        .register("make_vec_entity", |vec: Vec<V<Entity>>| vec)
        .register("noop", || {})
        .register(
            "noop_4_args",
            |_a: ScriptValue, _b: ScriptValue, _c: ScriptValue, _d: ScriptValue| {},
        )
        .register("into_script_function", |f: DynamicScriptFunction| f)
        .register(
            "assert_str_eq",
            |s1: String, s2: String, reason: Option<String>| {
                pretty_assertions::assert_eq!(
                    s1.trim(),
                    s2.trim(),
                    "Reason Provided: {}",
                    reason.unwrap_or_default()
                )
            },
        )
        .register(
            "create_test_asset",
            |s: FunctionCallContext, value: i32, name: String| {
                let world = s.world()?;
                let test_asset = TestAsset::new(value, name);
                let handle = world.with_resource_mut(|mut assets: Mut<Assets<TestAsset>>| {
                    assets.add(test_asset)
                })?;
                Ok(ReflectReference::new_allocated(handle))
            },
        );
}

/// Builds the system graph of the given schedule, and asserts the given check holds on it
fn assert_on_schedule_graph(
    ctx: &FunctionCallContext,
    schedule: &ReflectSchedule,
    failure_message: String,
    check: impl FnOnce(&ReflectSystemGraph) -> Result<bool, String>,
) -> Result<(), InteropError> {
    let world = ctx.world()?;
    let holds = world.with_resource(|schedules: &Schedules| {
        let sched = schedules
            .get(*schedule.label())
            .ok_or_else(|| InteropError::missing_schedule(schedule.identifier()))?;
        check(&bevy_system_reflection::schedule_to_reflect_graph(sched))
            .map_err(InteropError::string)
    })??;
    if holds {
        Ok(())
    } else {
        Err(InteropError::string(format!(
            "{failure_message} in schedule `{}`",
            schedule.identifier()
        )))
    }
}
