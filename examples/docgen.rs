use bevy::ecs::reflect::AppTypeRegistry;
use bevy::prelude::PluginGroup;
use bevy::{
    app::App,
    render::{
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    DefaultPlugins,
};
use bevy_mod_scripting::ScriptFunctionsPlugin;
use bevy_mod_scripting_core::bindings::function::script_function::AppScriptFunctionRegistry;
use bevy_mod_scripting_core::bindings::globals::core::CoreScriptGlobalsPlugin;
use bevy_mod_scripting_core::bindings::globals::AppScriptGlobalsRegistry;
use ladfile_builder::plugin::{generate_lad_file, LadFileSettings, ScriptingDocgenPlugin};

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    // headless bevy, kinda, I want to include as many plugins as I can which actually
    // provide reflected type definitions, but exclude anything that runs rendering stuff.
    app.add_plugins((DefaultPlugins.set(RenderPlugin {
        synchronous_pipeline_compilation: true,
        render_creation: RenderCreation::Automatic(WgpuSettings {
            backends: None,
            ..Default::default()
        }),
    }),));

    // docgen + scripting
    app.add_plugins((
        // normally the global plugin is included as part of each scripting plugin, here we just take
        // the definitions by themselves
        CoreScriptGlobalsPlugin,
        ScriptFunctionsPlugin,
    ));

    // there are two ways to generate the ladfile

    // 1. add the docgen plugin and run your app as normal
    app.add_plugins(ScriptingDocgenPlugin::default());
    // running the app once like below would do the trick
    // app.cleanup();
    // app.finish();
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
        &global_registry,
        &settings,
    );

    // bah bye, the generated file will be found in assets/
    // this can then be passed to various backends to generate docs, and other declaration files
    Ok(())
}
