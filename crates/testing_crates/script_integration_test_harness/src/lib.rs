pub mod test_functions;

use std::{
    marker::PhantomData,
    path::PathBuf,
    time::{Duration, Instant},
};

use bevy::{
    app::{Last, Plugin, PostUpdate, Startup, Update},
    asset::{AssetServer, Handle},
    ecs::{
        event::{Event, Events},
        schedule::{IntoSystemConfigs, SystemConfigs},
        system::{IntoSystem, Local, Res, SystemState},
        world::{FromWorld, Mut},
    },
    prelude::{Entity, World},
    reflect::TypeRegistry,
};
use bevy_mod_scripting_core::{
    asset::ScriptAsset,
    bindings::{pretty_print::DisplayWithWorld, script_value::ScriptValue, WorldGuard},
    callback_labels,
    error::{InteropError, ScriptError},
    event::{IntoCallbackLabel, ScriptErrorEvent},
    extractors::{HandlerContext, WithWorldGuard},
    handler::handle_script_errors,
    script::ScriptId,
    IntoScriptPluginParams, ScriptingPlugin,
};
use bevy_mod_scripting_functions::ScriptFunctionsPlugin;
use test_functions::register_test_functions;
use test_utils::test_data::setup_integration_test;

fn dummy_update_system() {}
fn dummy_startup_system<T>() {}
fn dummy_before_post_update_system() {}
fn dummy_post_update_system() {}

pub trait Benchmarker: 'static + Send + Sync {
    fn bench(
        &self,
        label: &str,
        f: &dyn Fn() -> Result<ScriptValue, ScriptError>,
    ) -> Result<ScriptValue, ScriptError>;

    fn clone_box(&self) -> Box<dyn Benchmarker>;
}

#[derive(Clone)]
pub struct NoOpBenchmarker;

impl Benchmarker for NoOpBenchmarker {
    fn bench(
        &self,
        _label: &str,
        f: &dyn Fn() -> Result<ScriptValue, ScriptError>,
    ) -> Result<ScriptValue, ScriptError> {
        f()
    }

    fn clone_box(&self) -> Box<dyn Benchmarker> {
        Box::new(self.clone())
    }
}

#[derive(Event)]
struct TestEventFinished;

struct TestCallbackBuilder<P: IntoScriptPluginParams, L: IntoCallbackLabel> {
    _ph: PhantomData<(P, L)>,
}

impl<L: IntoCallbackLabel, P: IntoScriptPluginParams> TestCallbackBuilder<P, L> {
    fn build(script_id: impl Into<ScriptId>, expect_response: bool) -> SystemConfigs {
        let script_id = script_id.into();
        IntoSystem::into_system(
            move |world: &mut World,
                  system_state: &mut SystemState<WithWorldGuard<HandlerContext<P>>>| {
                let with_guard = system_state.get_mut(world);
                let _ = run_test_callback::<P, L>(&script_id.clone(), with_guard, expect_response);

                system_state.apply(world);
            },
        )
        .with_name(L::into_callback_label().to_string())
        .into_configs()
    }
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

        runtime.register_fn("assert", |a: Dynamic, b: &str| {
            if !a.is::<bool>() {
                panic!("Expected a boolean value, but got {:?}", a);
            }
            if !a.as_bool().unwrap() {
                panic!("Assertion failed. {}", b);
            }
        });

        runtime.register_fn("assert", |a: Dynamic| {
            if !a.is::<bool>() {
                panic!("Expected a boolean value, but got {:?}", a);
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

#[cfg(feature = "lua")]
pub fn execute_lua_integration_test(script_id: &str) -> Result<(), String> {
    let plugin = make_test_lua_plugin();
    execute_integration_test(plugin, |_, _| {}, script_id)
}

#[cfg(feature = "rhai")]
pub fn execute_rhai_integration_test(script_id: &str) -> Result<(), String> {
    let plugin = make_test_rhai_plugin();
    execute_integration_test(plugin, |_, _| {}, script_id)
}

pub fn execute_integration_test<
    P: IntoScriptPluginParams + Plugin + AsMut<ScriptingPlugin<P>>,
    F: FnOnce(&mut World, &mut TypeRegistry),
>(
    plugin: P,
    init: F,
    script_id: &str,
) -> Result<(), String> {
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

    let mut app = setup_integration_test(init);

    app.add_plugins((ScriptFunctionsPlugin, plugin));

    register_test_functions(&mut app);

    app.add_event::<TestEventFinished>();

    callback_labels!(
        OnTest => "on_test",
        OnTestPostUpdate => "on_test_post_update",
        OnTestLast => "on_test_last",
    );

    let script_id = script_id.to_owned();
    let script_id: &'static str = Box::leak(script_id.into_boxed_str());

    let load_system = |server: Res<AssetServer>, mut handle: Local<Handle<ScriptAsset>>| {
        *handle = server.load(script_id.to_owned());
    };

    // tests can opt in to this via "__RETURN"
    let expect_callback_response = script_id.contains("__RETURN");

    app.add_systems(Startup, load_system);
    app.add_systems(
        Update,
        TestCallbackBuilder::<P, OnTest>::build(script_id, expect_callback_response),
    );
    app.add_systems(
        PostUpdate,
        TestCallbackBuilder::<P, OnTestPostUpdate>::build(script_id, expect_callback_response),
    );
    app.add_systems(
        Last,
        TestCallbackBuilder::<P, OnTestLast>::build(script_id, expect_callback_response),
    );
    app.add_systems(Update, dummy_update_system);
    app.add_systems(Startup, dummy_startup_system::<String>);

    app.add_systems(
        PostUpdate,
        dummy_before_post_update_system.before(dummy_post_update_system),
    );
    app.add_systems(PostUpdate, dummy_post_update_system);

    app.cleanup();
    app.finish();

    let start = Instant::now(); // start the timer

    loop {
        app.update();

        if start.elapsed() > Duration::from_secs(10) {
            return Err("Timeout after 10 seconds".into());
        }

        let error_events = app
            .world_mut()
            .resource_mut::<Events<ScriptErrorEvent>>()
            .drain()
            .collect::<Vec<_>>();

        if let Some(event) = error_events.into_iter().next() {
            return Err(event
                .error
                .display_with_world(WorldGuard::new_exclusive(app.world_mut())));
        }

        let events_completed = app.world_mut().resource_ref::<Events<TestEventFinished>>();
        if events_completed.len() > 0 {
            return Ok(());
        }
    }
}

fn run_test_callback<P: IntoScriptPluginParams, C: IntoCallbackLabel>(
    script_id: &str,
    mut with_guard: WithWorldGuard<'_, '_, HandlerContext<'_, P>>,
    expect_response: bool,
) -> Result<ScriptValue, ScriptError> {
    let (guard, handler_ctxt) = with_guard.get_mut();

    if !handler_ctxt.is_script_fully_loaded(script_id.to_string().into()) {
        return Ok(ScriptValue::Unit);
    }

    let res = handler_ctxt.call::<C>(
        &script_id.to_string().into(),
        Entity::from_raw(0),
        vec![],
        guard.clone(),
    );

    let e = match res {
        Ok(ScriptValue::Error(e)) => e.into(),
        Err(e) => e,
        Ok(v) => {
            if expect_response && !matches!(v, ScriptValue::Bool(true)) {
                InteropError::external_error(format!("Response from callback {} was either not received or wasn't correct. Expected true, got: {v:?}", C::into_callback_label()).into()).into()
            } else {
                match guard.with_resource_mut(|mut events: Mut<Events<TestEventFinished>>| {
                    events.send(TestEventFinished)
                }) {
                    Ok(_) => return Ok(v),
                    Err(e) => e.into(),
                }
            }
        }
    };

    handle_script_errors(guard, vec![e.clone()].into_iter());

    Err(e)
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
            criterion.bench_function(label, |c| {
                c.iter(|| {
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
            criterion.bench_function(label, |c| {
                c.iter(|| {
                    let _ = runtime
                        .call_fn::<Dynamic>(&mut ctxt.scope, &ctxt.ast, "bench", ARGS)
                        .unwrap();
                })
            });
            Ok(())
        },
    )
}

pub fn run_plugin_benchmark<P, F, M: criterion::measurement::Measurement>(
    plugin: P,
    script_id: &str,
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

    app.add_plugins((ScriptFunctionsPlugin, plugin));
    register_test_functions(&mut app);

    let script_id = script_id.to_owned();
    let script_id_clone = script_id.clone();
    app.add_systems(
        Startup,
        move |server: Res<AssetServer>, mut handle: Local<Handle<ScriptAsset>>| {
            *handle = server.load(script_id_clone.to_owned());
        },
    );

    // finalize
    app.cleanup();
    app.finish();

    let timer = Instant::now();

    let mut state = SystemState::<WithWorldGuard<HandlerContext<P>>>::from_world(app.world_mut());

    loop {
        app.update();

        let mut handler_ctxt = state.get_mut(app.world_mut());
        let (guard, context) = handler_ctxt.get_mut();

        if context.is_script_fully_loaded(script_id.clone().into()) {
            let script = context
                .scripts()
                .get_mut(script_id.to_owned())
                .ok_or_else(|| String::from("Could not find scripts resource"))?;
            let ctxt_arc = script.context.clone();
            let mut ctxt_locked = ctxt_arc.lock();

            let runtime = &context.runtime_container().runtime;

            return WorldAccessGuard::with_existing_static_guard(guard, |guard| {
                // Ensure the world is available via ThreadWorldContainer
                ThreadWorldContainer
                    .set_world(guard.clone())
                    .map_err(|e| e.display_with_world(guard))?;
                // Pass the locked context to the closure for benchmarking its Lua (or generic) part
                bench_fn(&mut ctxt_locked, runtime, label, criterion)
            });
        }
        state.apply(app.world_mut());
        if timer.elapsed() > Duration::from_secs(30) {
            return Err("Timeout after 30 seconds".into());
        }
    }
}
