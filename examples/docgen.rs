use bevy::prelude::PluginGroup;
use bevy::render::{RenderPlugin, settings::WgpuSettings};
use bevy::winit::WinitPlugin;
use bevy::{DefaultPlugins, app::App, ecs::reflect::AppTypeRegistry};
use bevy_mod_scripting::BMSPlugin;
use bevy_mod_scripting_bindings::AppScriptNamespaceMembers;
use bevy_mod_scripting_bindings::function::generics::AppGenericTypes;
use bevy_mod_scripting_bindings::function::script_function::AppScriptFunctionRegistry;
use ladfile_builder::plugin::{
    LadFileSettingsArc, ScriptingFilesGenerationPlugin, generate_lad_file,
};
use script_integration_test_harness::test_functions::register_test_functions;
use std::path::PathBuf;
use test_utils::test_data::register_test_types;

const TEST_HARNESS_GLOBALS: &str = "---@meta

---Calls `f`, which must throw an error matching the regex `pattern`.
---@param f fun()
---@param pattern string
function assert_throws(f, pattern) end
";

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    // headless bevy, kinda, I want to include as many plugins as I can which actually
    // provide reflected type definitions, but exclude anything that runs rendering stuff.
    // no window and no gpu backend, so this runs on CI and in containers
    app.add_plugins(
        DefaultPlugins
            .build()
            .disable::<WinitPlugin>()
            .set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: None,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            }),
    );

    // this example is used to drive the generated docs on the official BMS book
    app.add_plugins(BMSPlugin.set::<ScriptingFilesGenerationPlugin>(
        ScriptingFilesGenerationPlugin::new(
            true, // enabled, you can use a compilation feature to disable this here
            PathBuf::from("assets").join("definitions"),
            Some(PathBuf::from("bindings.lad.json")), // do also save the ladfile itself
            "Core BMS framework bindings",
            true,
            true,
        ),
    ));

    // `cargo run --example docgen --features lua_language_server_files -- testlib`
    // also includes the functions available to the script integration tests, i.e. to get completions while writing tests
    if std::env::args().skip(1).any(|arg| arg == "testlib") {
        register_test_functions(&mut app);
        register_test_types(&mut app);
        // `assert_throws` is a raw Lua global of the test harness, so it is not part of the ladfile
        std::fs::write(
            PathBuf::from("assets")
                .join("definitions")
                .join("test_harness.lua"),
            TEST_HARNESS_GLOBALS,
        )?;
    }

    // there are two ways to generate the ladfile

    // 1. add the docgen plugin and run your app as normal
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
    let namespace_registry = app
        .world()
        .get_resource::<AppScriptNamespaceMembers>()
        .unwrap()
        .clone();

    let settings = app
        .world()
        .get_resource::<LadFileSettingsArc>()
        .unwrap()
        .clone();

    let generic_types = app
        .world()
        .get_resource::<AppGenericTypes>()
        .cloned()
        .unwrap_or_default();

    generate_lad_file(
        &type_registry,
        &function_registry,
        &namespace_registry,
        &generic_types,
        &settings.0,
    );

    // bah bye, the generated file will be found in assets/
    // this can then be passed to various backends to generate docs, and other declaration files
    Ok(())
}
