use bevy::{DefaultPlugins, app::App, ecs::reflect::AppTypeRegistry};
use bevy_mod_scripting::ScriptFunctionsPlugin;
use bevy_mod_scripting_bindings::{
    DummyScriptFunctionRegistry,
    function::script_function::AppScriptFunctionRegistry,
    globals::{AppScriptGlobalsRegistry, core::CoreScriptGlobalsPlugin},
};
use bevy_mod_scripting_core::BMSScriptingInfrastructurePlugin;
use ladfile_builder::plugin::{LadFileSettings, ScriptingDocgenPlugin, generate_lad_file};

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    // headless bevy, kinda, I want to include as many plugins as I can which actually
    // provide reflected type definitions, but exclude anything that runs rendering stuff.
    app.add_plugins(DefaultPlugins);

    // docgen + scripting
    app.add_plugins((
        // normally the global plugin is included as part of each scripting plugin, here we just take
        // the definitions by themselves
        CoreScriptGlobalsPlugin::default(),
        ScriptFunctionsPlugin,
        BMSScriptingInfrastructurePlugin::default(),
    ));

    // there are two ways to generate the ladfile

    // 1. add the docgen plugin and run your app as normal
    app.add_plugins(ScriptingDocgenPlugin::default());
    app.finish();
    app.cleanup();
    // running update once will do the trick
    // app.update();

    // or 2. manually trigger the system
    // this is what we do here as we're running this example in GHA

    let type_registry = app
        .world()
        .get_resource::<AppTypeRegistry>()
        .unwrap()
        .clone();
    let function_registry = app
        .world()
        .get_resource::<AppScriptFunctionRegistry>()
        .unwrap()
        .clone();
    let dummy_function_registry = app
        .world()
        .get_resource::<DummyScriptFunctionRegistry>()
        .unwrap()
        .clone();
    let global_registry = app
        .world()
        .get_resource::<AppScriptGlobalsRegistry>()
        .unwrap()
        .clone();

    let settings = LadFileSettings {
        description: "Core BMS framework bindings",
        ..Default::default()
    };

    generate_lad_file(
        &type_registry,
        &function_registry,
        &dummy_function_registry,
        &global_registry,
        &settings,
    );

    // bah bye, the generated file will be found in assets/
    // this can then be passed to various backends to generate docs, and other declaration files
    Ok(())
}
