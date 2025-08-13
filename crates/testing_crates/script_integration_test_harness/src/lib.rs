pub mod parse;
pub mod scenario;
pub mod test_functions;

use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use bevy::{
    app::{App, Plugin, PostUpdate, Startup, Update},
    asset::{AssetPath, AssetServer, Handle, LoadState},
    ecs::{
        component::Component, resource::Resource, schedule::IntoScheduleConfigs, system::Command,
        world::FromWorld,
    },
    log::{
        tracing::{self, event},
        Level,
    },
    reflect::Reflect,
};
use bevy_mod_scripting_core::{
    bindings::{
        pretty_print::DisplayWithWorld, CoreScriptGlobalsPlugin, ReflectAccessId, WorldAccessGuard,
        WorldGuard,
    },
    commands::CreateOrUpdateScript,
    error::ScriptError,
    extractors::HandlerContext,
    script::{DisplayProxy, ScriptAttachment, ScriptComponent, ScriptId},
    BMSScriptingInfrastructurePlugin, IntoScriptPluginParams,
};
use bevy_mod_scripting_functions::ScriptFunctionsPlugin;
use criterion::{measurement::Measurement, BatchSize};
use rand::{Rng, SeedableRng};
use test_functions::{register_test_functions, RNG};
use test_utils::test_data::setup_integration_test;

use crate::scenario::Scenario;

fn dummy_update_system() {}
fn dummy_startup_system<T>() {}
fn dummy_before_post_update_system() {}
fn dummy_post_update_system() {}

pub fn install_test_plugin(app: &mut bevy::app::App, include_test_functions: bool) {
    app.add_plugins((
        ScriptFunctionsPlugin,
        CoreScriptGlobalsPlugin::default(),
        BMSScriptingInfrastructurePlugin,
    ));
    if include_test_functions {
        register_test_functions(app);
    }
    app.add_systems(Update, dummy_update_system);
    app.add_systems(Startup, dummy_startup_system::<String>);

    app.add_systems(
        PostUpdate,
        dummy_before_post_update_system.before(dummy_post_update_system),
    );
    app.add_systems(PostUpdate, dummy_post_update_system);
}

#[cfg(feature = "lua")]
pub fn make_test_lua_plugin() -> bevy_mod_scripting_lua::LuaScriptingPlugin {
    use bevy_mod_scripting_core::{bindings::WorldContainer, ConfigureScriptPlugin};
    use bevy_mod_scripting_lua::{mlua, LuaScriptingPlugin};

    LuaScriptingPlugin::default().add_context_initializer(
        |_, ctxt: &mut bevy_mod_scripting_lua::mlua::Lua| {
            let globals = ctxt.globals();
            globals.set(
                "assert_throws",
                ctxt.create_function(|_lua, (f, reg): (mlua::Function, String)| {
                    let world =
                        bevy_mod_scripting_core::bindings::ThreadWorldContainer.try_get_world()?;
                    let result = f.call::<()>(mlua::MultiValue::new());
                    let err = match result {
                        Ok(_) => {
                            return Err(mlua::Error::external(
                                "Expected function to throw error, but it did not.",
                            ))
                        }
                        Err(e) => ScriptError::from_mlua_error(e).display_with_world(world),
                    };

                    let regex = regex::Regex::new(&reg).unwrap();
                    if regex.is_match(&err) {
                        Ok(())
                    } else {
                        Err(mlua::Error::external(format!(
                            "Expected error message to match the regex: \n{}\n\nBut got:\n{}",
                            regex.as_str(),
                            err
                        )))
                    }
                })?,
            )?;
            Ok(())
        },
    )
}

#[cfg(feature = "rhai")]
pub fn make_test_rhai_plugin() -> bevy_mod_scripting_rhai::RhaiScriptingPlugin {
    use bevy_mod_scripting_core::{
        bindings::{ThreadWorldContainer, WorldContainer},
        ConfigureScriptPlugin,
    };
    use bevy_mod_scripting_rhai::{
        rhai::{Dynamic, EvalAltResult, FnPtr, NativeCallContext},
        RhaiScriptingPlugin,
    };

    RhaiScriptingPlugin::default().add_runtime_initializer(|runtime| {
        let mut runtime = runtime.write();
        runtime.set_max_call_levels(1000);
        runtime.register_fn("assert", |a: Dynamic, b: &str| {
            if !a.is::<bool>() {
                panic!("Expected a boolean value, but got {a:?}");
            }
            if !a.as_bool().unwrap() {
                panic!("Assertion failed. {b}");
            }
        });

        runtime.register_fn("assert", |a: Dynamic| {
            if !a.is::<bool>() {
                panic!("Expected a boolean value, but got {a:?}");
            }
            if !a.as_bool().unwrap() {
                panic!("Assertion failed");
            }
        });
        runtime.register_fn(
            "assert_throws",
            |ctxt: NativeCallContext, fn_: FnPtr, regex: String| {
                let world = ThreadWorldContainer.try_get_world()?;
                let args: [Dynamic; 0] = [];
                let result = fn_.call_within_context::<()>(&ctxt, args);
                match result {
                    Ok(_) => panic!("Expected function to throw error, but it did not."),
                    Err(e) => {
                        let e = ScriptError::from_rhai_error(*e);
                        let err = e.display_with_world(world);
                        let regex = regex::Regex::new(&regex).unwrap();
                        if regex.is_match(&err) {
                            Ok::<(), Box<EvalAltResult>>(())
                        } else {
                            panic!(
                                "Expected error message to match the regex: \n{}\n\nBut got:\n{}",
                                regex.as_str(),
                                err
                            )
                        }
                    }
                }
            },
        );
        Ok(())
    })
}

pub fn execute_integration_test(scenario: Scenario) -> Result<(), String> {
    // set "BEVY_ASSET_ROOT" to the global assets folder, i.e. CARGO_MANIFEST_DIR/../../../assets
    let mut manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // traverse upwards to find bevy_mod_scripting directory
    loop {
        if manifest_dir.ends_with("bevy_mod_scripting") {
            break;
        }
        manifest_dir.pop();
    }

    std::env::set_var("BEVY_ASSET_ROOT", manifest_dir.clone());

    match scenario.execute(App::default()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{e:?}")),
    }
}

#[cfg(feature = "lua")]
pub fn run_lua_benchmark<M: criterion::measurement::Measurement>(
    script_id: &str,
    label: &str,
    criterion: &mut criterion::BenchmarkGroup<M>,
) -> Result<(), String> {
    use bevy_mod_scripting_lua::mlua::Function;

    let plugin = make_test_lua_plugin();
    run_plugin_benchmark(
        plugin,
        script_id,
        label,
        criterion,
        |ctxt, _runtime, label, criterion| {
            let bencher: Function = ctxt.globals().get("bench").map_err(|e| e.to_string())?;
            let pre_bencher: Option<Function> = ctxt.globals().get("pre_bench").ok();
            criterion.bench_function(label, |c| {
                if let Some(pre_bencher) = &pre_bencher {
                    pre_bencher.call::<()>(()).unwrap();
                }
                c.iter(|| {
                    use bevy::log::{tracing, Level};

                    tracing::event!(Level::TRACE, "profiling_iter {}", label);
                    bencher.call::<()>(()).unwrap();
                })
            });
            Ok(())
        },
    )
}

#[cfg(feature = "rhai")]
pub fn run_rhai_benchmark<M: criterion::measurement::Measurement>(
    script_id: &str,
    label: &str,
    criterion: &mut criterion::BenchmarkGroup<M>,
) -> Result<(), String> {
    use bevy_mod_scripting_rhai::rhai::Dynamic;

    let plugin = make_test_rhai_plugin();
    run_plugin_benchmark(
        plugin,
        script_id,
        label,
        criterion,
        |ctxt, runtime, label, criterion| {
            let runtime = runtime.read();
            const ARGS: [usize; 0] = [];
            let has_pre_bench = ctxt.ast.iter_functions().any(|f| f.name == "pre_bench");
            criterion.bench_function(label, |c| {
                // call "pre_bench" if any
                if has_pre_bench {
                    let _ = runtime
                        .call_fn::<Dynamic>(&mut ctxt.scope, &ctxt.ast, "pre_bench", ARGS)
                        .unwrap();
                }

                c.iter(|| {
                    use bevy::log::{tracing, Level};

                    tracing::event!(Level::TRACE, "profiling_iter {}", label);
                    let _ = runtime
                        .call_fn::<Dynamic>(&mut ctxt.scope, &ctxt.ast, "bench", ARGS)
                        .unwrap();
                })
            });
            Ok(())
        },
    )
}

pub fn run_plugin_benchmark<'a, P, F, M: criterion::measurement::Measurement>(
    plugin: P,
    script_path: impl Into<AssetPath<'a>>,
    label: &str,
    criterion: &mut criterion::BenchmarkGroup<M>,
    bench_fn: F,
) -> Result<(), String>
where
    P: IntoScriptPluginParams + Plugin,
    F: Fn(&mut P::C, &P::R, &str, &mut criterion::BenchmarkGroup<M>) -> Result<(), String>,
{
    use bevy_mod_scripting_core::bindings::{
        ThreadWorldContainer, WorldAccessGuard, WorldContainer,
    };

    let mut app = setup_integration_test(|_, _| {});

    install_test_plugin(&mut app, true);
    app.add_plugins(plugin);
    let script_path = script_path.into();
    let script_handle = app.world().resource::<AssetServer>().load(script_path);
    let script_id = script_handle.id();
    let entity = app
        .world_mut()
        .spawn(ScriptComponent(vec![script_handle.clone()]))
        .id();

    // finalize
    app.cleanup();
    app.finish();

    let timer = Instant::now();

    // Wait until script is loaded.
    loop {
        if timer.elapsed() > Duration::from_secs(30) {
            return Err("Timeout after 30 seconds, could not load script".into());
        }
        app.update();
        match app.world().resource::<AssetServer>().load_state(script_id) {
            LoadState::Loaded => break,
            LoadState::Failed(e) => {
                return Err(format!(
                    "Failed to load script {}: {e}",
                    script_handle.display()
                ));
            }
            _ => continue,
        }
    }

    app.update();

    let mut context = HandlerContext::<P>::yoink(app.world_mut());
    let guard = WorldGuard::new_exclusive(app.world_mut());

    let context_key = ScriptAttachment::EntityScript(entity, Handle::Weak(script_id));

    let ctxt_arc = context.script_context().get(&context_key).unwrap();
    let mut ctxt_locked = ctxt_arc.lock();

    let runtime = &context.runtime_container().runtime;

    let _ = WorldAccessGuard::with_existing_static_guard(guard, |guard| {
        // Ensure the world is available via ThreadWorldContainer
        ThreadWorldContainer
            .set_world(guard.clone())
            .map_err(|e| e.display_with_world(guard))?;
        // Pass the locked context to the closure for benchmarking its Lua (or generic) part
        bench_fn(&mut ctxt_locked, runtime, label, criterion)
    });
    context.release(app.world_mut());
    Ok(())
}

pub fn run_plugin_script_load_benchmark<
    P: IntoScriptPluginParams + Plugin + FromWorld,
    M: Measurement,
>(
    plugin: P,
    benchmark_id: &str,
    content: &str,
    criterion: &mut criterion::BenchmarkGroup<M>,
    reload_probability: f32,
) {
    let mut app = setup_integration_test(|_, _| {});
    install_test_plugin(&mut app, false);
    app.add_plugins(plugin);
    let mut rng_guard = RNG.lock().unwrap();
    *rng_guard = rand_chacha::ChaCha12Rng::from_seed([42u8; 32]);
    drop(rng_guard);
    criterion.bench_function(benchmark_id, |c| {
        c.iter_batched(
            || {
                let mut rng = RNG.lock().unwrap();
                let is_reload = rng.random_range(0f32..=1f32) < reload_probability;
                let random_id = if is_reload { 0 } else { rng.random::<u128>() };
                let random_script_id: ScriptId = ScriptId::from(
                    uuid::Builder::from_random_bytes(random_id.to_le_bytes()).into_uuid(),
                );
                // We manually load the script inside a command.
                (
                    CreateOrUpdateScript::<P>::new(ScriptAttachment::StaticScript(Handle::Weak(
                        random_script_id,
                    )))
                    .with_content(content),
                    is_reload,
                )
            },
            |(command, _is_reload)| {
                tracing::event!(
                    Level::TRACE,
                    "profiling_iter {} is reload?: {}",
                    benchmark_id,
                    _is_reload
                );
                command.apply(app.world_mut());
            },
            BatchSize::LargeInput,
        );
    });
}

pub fn perform_benchmark_with_generator<
    M: Measurement,
    I,
    G: Fn(&mut rand_chacha::ChaCha12Rng, WorldAccessGuard) -> I,
    B: Fn(WorldAccessGuard, I),
>(
    label: &str,
    generator: &G,
    bench_fn: &B,
    group: &mut criterion::BenchmarkGroup<M>,
    batch_size: BatchSize,
) {
    #[derive(Reflect, Component, Resource)]
    struct Fake1;
    #[derive(Reflect, Component, Resource)]
    struct Fake2;
    #[derive(Reflect, Resource)]
    struct Fake3;
    #[derive(Reflect, Resource)]
    struct Fake4;
    #[derive(Reflect, Resource)]
    struct Fake5;

    let mut world = std::mem::take(setup_integration_test(|_, _| {}).world_mut());
    let f1 = world.register_component::<Fake1>();
    let f2 = world.register_component::<Fake2>();
    let f3 = world.register_resource::<Fake3>();
    let f4 = world.register_resource::<Fake4>();
    let f5 = world.register_resource::<Fake5>();

    let world_guard = WorldAccessGuard::new_exclusive(&mut world);
    let mut rng_guard = RNG.lock().unwrap();
    *rng_guard = rand_chacha::ChaCha12Rng::from_seed([42u8; 32]);
    drop(rng_guard);
    group.bench_function(label, |c| {
        c.iter_batched(
            || {
                let mut rng_guard = RNG.lock().unwrap();
                unsafe { world_guard.release_all_accesses() };
                {
                    let allocator = world_guard.allocator();
                    let mut allocator = allocator.write();
                    allocator.clean_garbage_allocations();
                }

                // lock a random amount of fake components/resources, to make benchmarks more natural
                for _ in 0..rng_guard.random_range(0..=5) {
                    // pick random component
                    match rng_guard.random_range(0..=4) {
                        0 => world_guard.claim_write_access(ReflectAccessId::for_component_id(f1)),
                        1 => world_guard.claim_write_access(ReflectAccessId::for_component_id(f2)),
                        2 => world_guard.claim_write_access(ReflectAccessId::for_component_id(f3)),
                        3 => world_guard.claim_write_access(ReflectAccessId::for_component_id(f4)),
                        4 => world_guard.claim_write_access(ReflectAccessId::for_component_id(f5)),
                        _ => false,
                    };
                }

                (
                    generator(&mut rng_guard, world_guard.clone()),
                    world_guard.clone(),
                )
            },
            |(i, w)| {
                event!(bevy::log::Level::TRACE, "profiling_iter {}", label);
                bench_fn(w, i)
            },
            batch_size,
        );
    });
}
