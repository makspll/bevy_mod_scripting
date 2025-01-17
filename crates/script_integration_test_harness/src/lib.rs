pub mod test_functions;

use bevy::{
    app::App,
    prelude::{Entity, World},
    reflect::TypeRegistry,
};
use bevy_mod_scripting_core::{
    bindings::{
        pretty_print::DisplayWithWorld, script_value::ScriptValue, WorldAccessGuard, WorldGuard,
    },
    context::ContextLoadingSettings,
    event::{IntoCallbackLabel, OnScriptLoaded},
    handler::CallbackSettings,
    runtime::RuntimeSettings,
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
    code: &[u8],
) -> Result<(), String> {
    let mut app = setup_integration_test(init);

    app.add_plugins(ScriptFunctionsPlugin);
    register_test_functions(&mut app);

    init_app(&mut app);

    app.cleanup();
    app.finish();

    let context_settings: ContextLoadingSettings<P> = app
        .world_mut()
        .remove_resource()
        .ok_or("could not find context loading settings")
        .unwrap();

    let callback_settings: CallbackSettings<P> = app
        .world_mut()
        .remove_resource()
        .ok_or("could not find callback settings")
        .unwrap();

    let runtime_settings: RuntimeSettings<P> = app
        .world_mut()
        .remove_resource()
        .ok_or("could not find runtime settings")
        .unwrap();

    let mut runtime = P::build_runtime();
    runtime_settings
        .initializers
        .iter()
        .for_each(|initializer| {
            (initializer)(&mut runtime);
        });

    // load the context as normal
    let mut loaded_context = (context_settings.loader.load)(
        &(script_id.to_owned()).into(),
        code,
        &context_settings.context_initializers,
        &context_settings.context_pre_handling_initializers,
        app.world_mut(),
        &mut runtime,
    )
    .map_err(|e| {
        let world = app.world_mut();
        let world = WorldAccessGuard::new(world);
        e.display_with_world(WorldGuard::new(world))
    })?;

    // call on_script_loaded as normal
    let val = (callback_settings.callback_handler)(
        vec![],
        Entity::from_raw(0),
        &(script_id.to_owned()).into(),
        &OnScriptLoaded::into_callback_label(),
        &mut loaded_context,
        &context_settings.context_pre_handling_initializers,
        &mut runtime,
        app.world_mut(),
    )
    .map_err(|e| e.display_with_world(WorldGuard::new(WorldAccessGuard::new(app.world_mut()))))?;

    if let ScriptValue::Error(e) = val {
        return Err(e.display_with_world(WorldGuard::new(WorldAccessGuard::new(app.world_mut()))));
    }

    // call on_test callback
    let val = (callback_settings.callback_handler)(
        vec![],
        Entity::from_raw(0),
        &(script_id.to_owned()).into(),
        &"on_test".into(),
        &mut loaded_context,
        &context_settings.context_pre_handling_initializers,
        &mut runtime,
        app.world_mut(),
    )
    .map_err(|e| e.display_with_world(WorldGuard::new(WorldAccessGuard::new(app.world_mut()))))?;

    if let ScriptValue::Error(e) = val {
        return Err(e.display_with_world(WorldGuard::new(WorldAccessGuard::new(app.world_mut()))));
    }

    Ok(())
}
