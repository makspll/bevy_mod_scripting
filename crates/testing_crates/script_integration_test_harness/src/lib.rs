pub mod test_functions;

use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use bevy::{
    app::{App, Update},
    asset::{AssetServer, Handle},
    ecs::{
        event::{Event, Events},
        system::{Local, Res},
        world::Mut,
    },
    prelude::{Entity, World},
    reflect::TypeRegistry,
};
use bevy_mod_scripting_core::{
    asset::ScriptAsset,
    bindings::{pretty_print::DisplayWithWorld, script_value::ScriptValue, WorldGuard},
    callback_labels,
    event::ScriptErrorEvent,
    extractors::{HandlerContext, WithWorldGuard},
    handler::handle_script_errors,
    IntoScriptPluginParams,
};
use bevy_mod_scripting_functions::ScriptFunctionsPlugin;
use test_functions::register_test_functions;
use test_utils::test_data::setup_integration_test;

pub fn execute_integration_test<
    P: IntoScriptPluginParams,
    F: FnOnce(&mut World, &mut TypeRegistry),
    G: FnOnce(&mut App),
>(
    init: F,
    init_app: G,
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

    app.add_plugins(ScriptFunctionsPlugin);

    register_test_functions(&mut app);

    init_app(&mut app);

    #[derive(Event)]
    struct TestEventFinished;
    app.add_event::<TestEventFinished>();

    callback_labels!(
        OnTest => "on_test"
    );

    let script_id = script_id.to_owned();
    let script_id: &'static str = Box::leak(script_id.into_boxed_str());

    let load_system = |server: Res<AssetServer>, mut handle: Local<Handle<ScriptAsset>>| {
        *handle = server.load(script_id.to_owned());
    };
    let run_on_test_callback = |mut with_guard: WithWorldGuard<HandlerContext<P>>| {
        let (guard, handler_ctxt) = with_guard.get_mut();

        if !handler_ctxt.is_script_fully_loaded(script_id.into()) {
            return;
        }

        let res = handler_ctxt.call::<OnTest>(
            script_id.into(),
            Entity::from_raw(0),
            vec![],
            guard.clone(),
        );
        let e = match res {
            Ok(ScriptValue::Error(e)) => e.into(),
            Err(e) => e,
            _ => {
                match guard.with_resource_mut(|mut events: Mut<Events<TestEventFinished>>| {
                    events.send(TestEventFinished)
                }) {
                    Ok(_) => return,
                    Err(e) => e.into(),
                }
            }
        };
        handle_script_errors(guard, vec![e].into_iter())
    };

    app.add_systems(Update, (load_system, run_on_test_callback));

    app.cleanup();
    app.finish();

    let start = Instant::now(); // start the timer

    loop {
        app.update();

        if start.elapsed() > Duration::from_secs(10) {
            return Err("Timeout after 10 seconds".into());
        }

        let events_completed = app.world_mut().resource_ref::<Events<TestEventFinished>>();
        if events_completed.len() > 0 {
            return Ok(());
        }

        let error_events = app
            .world_mut()
            .resource_mut::<Events<ScriptErrorEvent>>()
            .drain()
            .collect::<Vec<_>>();

        if let Some(event) = error_events.into_iter().next() {
            return Err(event
                .error
                .display_with_world(WorldGuard::new(app.world_mut())));
        }
    }
}
