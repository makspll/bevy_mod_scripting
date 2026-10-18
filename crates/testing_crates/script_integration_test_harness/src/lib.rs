pub mod scenario;
pub mod test_functions;

use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use ::{
    bevy_app::{App, Plugin, PostUpdate, Startup, Update},
    bevy_asset::{AssetPath, AssetServer, LoadState},
    bevy_ecs::{
        component::Component,
        message::Messages,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Command, SystemId},
        world::FromWorld,
    },
    bevy_log::tracing::{self},
    bevy_reflect::Reflect,
};
use bevy_asset::Assets;
use bevy_mod_scripting_asset::ScriptAsset;
use bevy_mod_scripting_bindings::{CoreScriptGlobalsPlugin, WorldExtensions};
use bevy_mod_scripting_core::{
    BMSScriptingInfrastructurePlugin, IntoScriptPluginParams,
    commands::{AttachScript, RunScriptCallback},
    error::ScriptError,
    event::{
        IntoCallbackLabel, ScriptCallbackMessage, ScriptCallbackResponseMessage, ScriptErrorMessage,
    },
    handler::event_handler,
    pipeline::PipelineRun,
    script::{ScriptComponent, ScriptContexts},
};
use bevy_mod_scripting_display::DisplayProxy;
use bevy_mod_scripting_functions::{ConstructorsPlugin, ScriptFunctionsPlugin};
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_mod_scripting_world::{WorldAccessGuard, WorldGuard};
use criterion::{BatchSize, measurement::Measurement};
use rand::{Rng, SeedableRng};
use test_functions::{RNG, register_test_functions};
use test_utils::test_data::setup_integration_test;

use crate::scenario::Scenario;

fn dummy_update_system() {}
fn dummy_startup_system<T>() {}
fn dummy_before_post_update_system() {}
fn dummy_post_update_system() {}

pub fn install_test_plugin(app: &mut App, include_test_functions: bool) {
    app.add_plugins((
        ScriptFunctionsPlugin,
        CoreScriptGlobalsPlugin::default(),
        ConstructorsPlugin::default(),
        BMSScriptingInfrastructurePlugin::default(),
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
    use bevy_mod_scripting_core::ConfigureScriptPlugin;
    use bevy_mod_scripting_lua::{LuaScriptingPlugin, mlua};

    LuaScriptingPlugin::default().add_context_initializer(
        |_, ctxt: &mut bevy_mod_scripting_lua::LuaContext| {
            use bevy_mod_scripting_lua::IntoInteropError;

            let globals = ctxt.globals();
            globals
                .set(
                    "assert_throws",
                    ctxt.create_function(|_lua, (f, reg): (mlua::Function, String)| {
                        let result = f.call::<()>(mlua::MultiValue::new());
                        let err = match result {
                            Ok(_) => {
                                return Err(mlua::Error::external(
                                    "Expected function to throw error, but it did not.",
                                ));
                            }
                            Err(e) => format!(
                                "{}",
                                ScriptError::from(
                                    bevy_mod_scripting_lua::IntoInteropError::to_bms_error(e),
                                )
                            ),
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
                    })
                    .map_err(IntoInteropError::to_bms_error)?,
                )
                .map_err(IntoInteropError::to_bms_error)?;
            Ok(())
        },
    )
}

#[cfg(feature = "rune")]
pub fn make_test_rune_plugin() -> bevy_mod_scripting_rune::RuneScriptingPlugin {
    use bevy_mod_scripting_core::ConfigureScriptPlugin;
    use bevy_mod_scripting_rune::{
        RuneScriptingPlugin,
        rune::{
            ContextError, Module,
            runtime::{Function, Value, VmError, VmResult},
        },
    };

    fn test_module(module: &mut Module) -> Result<(), ContextError> {
        // like in Lua, runs the function and expects it to fail with a message matching the regex
        module
            .function("assert_throws", |function: &Function, pattern: &str| {
                let error = match function.call::<Value>(()).into_result() {
                    Ok(_) => {
                        return VmResult::err(VmError::panic(
                            "Expected function to throw error, but it did not.",
                        ));
                    }
                    Err(error) => error.to_string(),
                };
                #[allow(clippy::unwrap_used, reason = "tests")]
                let regex = regex::Regex::new(pattern).unwrap();
                if regex.is_match(&error) {
                    VmResult::Ok(())
                } else {
                    VmResult::err(VmError::panic(format!(
                        "Expected error message to match the regex: 
{}

But got:
{}",
                        regex.as_str(),
                        error
                    )))
                }
            })
            .build()?;
        Ok(())
    }

    RuneScriptingPlugin::default().add_runtime_initializer(|runtime| {
        runtime.add_module(test_module);
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

    unsafe { std::env::set_var("BEVY_ASSET_ROOT", manifest_dir.clone()) };

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
    let span = tracing::trace_span!("bench_fn");

    let plugin = make_test_lua_plugin();
    run_plugin_benchmark(
        plugin,
        script_id.to_string(),
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
                    span.in_scope(|| {
                        bencher.call::<()>(()).unwrap();
                    })
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
    let mut app = setup_integration_test(|_, _| {});
    let span = tracing::trace_span!("bench_fn");

    install_test_plugin(&mut app, true);
    app.add_plugins(plugin);

    // finalize
    app.cleanup();
    app.finish();

    let script_path = script_path.into();
    let script_handle = app.world().resource::<AssetServer>().load(script_path);
    let script_id = script_handle.id();
    let entity = app
        .world_mut()
        .spawn(ScriptComponent(vec![script_handle.clone()]))
        .id();

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

    app.update_until_all_scripts_processed::<P>();

    let script_contexts = app
        .world_mut()
        .get_resource_or_init::<ScriptContexts<P>>()
        .clone();

    let context_key = ScriptAttachment::EntityScript(entity, script_handle.clone());
    let cache = WorldAccessGuard::setup_cache(
        app.world(),
        bevy_mod_scripting_bindings::CurrentScriptAttachment(Some(context_key.clone())),
    );
    let guard = WorldGuard::new_exclusive(app.world_mut(), cache);

    let script_contexts = script_contexts.read();
    let ctxt_arc = script_contexts.get_context(&context_key).unwrap();
    let ctxt_arc = ctxt_arc.as_loaded().unwrap();
    drop(script_contexts);
    let mut ctxt_locked = ctxt_arc.lock();

    let runtime = P::readonly_configuration(guard.id()).runtime;

    let _ = WorldAccessGuard::with_existing_static_guard(guard, |guard| {
        guard.set_current_attachment(context_key.clone());
        // Pass the locked context to the closure for benchmarking its Lua (or generic) part
        span.in_scope(|| bench_fn(&mut ctxt_locked, runtime, label, criterion))
    });
    Ok(())
}

pub fn run_plugin_script_load_benchmark<
    P: IntoScriptPluginParams + Plugin + FromWorld,
    F: Fn() -> P,
    M: Measurement,
>(
    plugin_maker: F,
    benchmark_id: &str,
    content: &str,
    criterion: &mut criterion::BenchmarkGroup<M>,
) {
    let content_boxed = content.to_string().into_bytes().into_boxed_slice();
    let span = tracing::trace_span!("bench_fn");

    criterion.bench_function(benchmark_id, |c| {
        c.iter_batched(
            || {
                let mut app = setup_integration_test(|_, _| {});
                install_test_plugin(&mut app, false);
                app.add_plugins(plugin_maker());

                // Safety: we claimed a unique guard, only code accessing this will need to do the same
                let world = app.world_mut();
                let mut assets = world.get_resource_or_init::<Assets<ScriptAsset>>();

                let id = assets.add(ScriptAsset {
                    content: content_boxed.clone(),
                    language: P::LANGUAGE,
                });

                // We manually load the script inside a command.
                (
                    app,
                    AttachScript::<P>::new(ScriptAttachment::StaticScript(id)),
                )
            },
            |(mut app, command)| span.in_scope(|| command.apply(app.world_mut())),
            BatchSize::LargeInput,
        );
    });
}

bevy_mod_scripting_core::callback_labels!(
    OnBenchCallback => "on_bench_callback"
);

/// Sets up an app with `script_count` static scripts, each in its own context, all defining an `on_bench_callback` callback.
fn setup_callback_benchmark_app<P: IntoScriptPluginParams + Plugin, F: Fn() -> P>(
    plugin_maker: F,
    content: &str,
    script_count: usize,
) -> (App, Vec<ScriptAttachment>) {
    let mut app = setup_integration_test(|_, _| {});
    install_test_plugin(&mut app, false);
    app.add_plugins(plugin_maker());
    app.finish();
    app.cleanup();

    let world = app.world_mut();
    let attachments = (0..script_count)
        .map(|_| {
            let id = world
                .get_resource_or_init::<Assets<ScriptAsset>>()
                .add(ScriptAsset {
                    content: content.as_bytes().into(),
                    language: P::LANGUAGE,
                });
            let attachment = ScriptAttachment::StaticScript(id);
            AttachScript::<P>::new(attachment.clone()).apply(world);
            attachment
        })
        .collect();
    // loading finishes over the course of the next update
    app.update();
    (app, attachments)
}

/// Benchmarks dispatching one [`ScriptCallbackMessage`] through the [`event_handler`] system to `script_count` static scripts, each in its own context.
///
/// Measures the per-callback overhead of the handler path, the callback itself is expected to be empty.
pub fn run_plugin_callback_event_benchmark<
    P: IntoScriptPluginParams + Plugin,
    F: Fn() -> P,
    M: Measurement,
>(
    plugin_maker: F,
    benchmark_id: &str,
    content: &str,
    script_count: usize,
    criterion: &mut criterion::BenchmarkGroup<M>,
) {
    let (mut app, _) = setup_callback_benchmark_app(plugin_maker, content, script_count);
    let world = app.world_mut();
    let handler: SystemId<(), bevy_ecs::error::Result> =
        world.register_system(event_handler::<OnBenchCallback, P>);

    // make sure every script actually ran the callback, so we are not benchmarking the error path
    world.write_message(
        ScriptCallbackMessage::new_for_all_scripts(OnBenchCallback, vec![]).with_response(),
    );
    world
        .run_system(handler)
        .expect("handler system failed")
        .expect("handler system errored");
    let responses = world.resource::<Messages<ScriptCallbackResponseMessage>>();
    let ok_responses = responses
        .iter_current_update_messages()
        .filter(|r| r.response.is_ok())
        .count();
    assert_eq!(
        ok_responses,
        script_count,
        "expected every script to handle the benchmark callback, residents: {}, errors: {:?}",
        world
            .resource::<ScriptContexts<P>>()
            .read()
            .all_residents_len(),
        world
            .resource::<Messages<ScriptErrorMessage>>()
            .iter_current_update_messages()
            .collect::<Vec<_>>()
    );

    let span = tracing::trace_span!("bench_fn");
    criterion.bench_function(benchmark_id, |b| {
        b.iter(|| {
            span.in_scope(|| {
                world.write_message(ScriptCallbackMessage::new_for_all_scripts(
                    OnBenchCallback,
                    vec![],
                ));
                let _ = world.run_system(handler);
                // drop handled messages, so they don't accumulate between iterations
                world
                    .resource_mut::<Messages<ScriptCallbackMessage>>()
                    .update();
            })
        });
    });
}

/// Benchmarks running a single callback via the [`RunScriptCallback`] command, which sets up a fresh world guard on every call.
pub fn run_plugin_callback_command_benchmark<
    P: IntoScriptPluginParams + Plugin,
    F: Fn() -> P,
    M: Measurement,
>(
    plugin_maker: F,
    benchmark_id: &str,
    content: &str,
    criterion: &mut criterion::BenchmarkGroup<M>,
) {
    let (mut app, attachments) = setup_callback_benchmark_app(plugin_maker, content, 1);
    let world = app.world_mut();
    let attachment = attachments[0].clone();
    let label = OnBenchCallback::into_callback_label();

    let make_command =
        || RunScriptCallback::<P>::new(attachment.clone(), label.clone(), vec![], false);
    if let Err(err) = make_command().apply(world) {
        panic!("benchmark callback failed: {err}");
    }

    let span = tracing::trace_span!("bench_fn");
    criterion.bench_function(benchmark_id, |b| {
        b.iter_batched(
            make_command,
            |command| span.in_scope(|| command.apply(world)),
            BatchSize::SmallInput,
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
    #[derive(Reflect, Component)]
    struct Fake1;
    #[derive(Reflect, Component)]
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
    let f3 = world.register_component::<Fake3>();
    let f4 = world.register_component::<Fake4>();
    let f5 = world.register_component::<Fake5>();
    let cache = WorldAccessGuard::setup_cache(
        &world,
        bevy_mod_scripting_bindings::CurrentScriptAttachment::empty(),
    );
    let world_guard = WorldAccessGuard::new_exclusive(&mut world, cache);
    let mut rng_guard = RNG.lock().unwrap();
    *rng_guard = rand_chacha::ChaCha12Rng::from_seed([42u8; 32]);
    drop(rng_guard);
    let span = tracing::trace_span!("bench_fn");
    group.bench_function(label, |c| {
        c.iter_batched(
            || {
                let mut rng_guard = RNG.lock().unwrap();
                unsafe { world_guard.release_all_accesses() };

                // lock a random amount of fake components/resources, to make benchmarks more natural
                for _ in 0..rng_guard.random_range(0..=5) {
                    // pick random component
                    match rng_guard.random_range(0..=4) {
                        0 => world_guard.claim_write_access(f1),
                        1 => world_guard.claim_write_access(f2),
                        2 => world_guard.claim_write_access(f3),
                        3 => world_guard.claim_write_access(f4),
                        4 => world_guard.claim_write_access(f5),
                        _ => false,
                    };
                }

                (
                    generator(&mut rng_guard, world_guard.clone()),
                    world_guard.clone(),
                )
            },
            |(i, w)| span.in_scope(|| bench_fn(w, i)),
            batch_size,
        );
    });
}
