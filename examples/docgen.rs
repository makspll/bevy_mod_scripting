use bevy::{app::App, DefaultPlugins};
use bevy_mod_scripting::ScriptFunctionsPlugin;
use bevy_mod_scripting_core::bindings::globals::core::CoreScriptGlobalsPlugin;
use ladfile_builder::plugin::ScriptingDocgenPlugin;

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    // headless bevy, kinda, I want to include as many plugins as I can which actually
    // provide reflected type definitions, but exclude anything that runs rendering stuff.
    app.add_plugins(DefaultPlugins);

    // docgen + scripting
    app.add_plugins((
        // normally the global plugin is included as part of each scripting plugin, here we just take
        // the definitions by themselves
        CoreScriptGlobalsPlugin,
        ScriptFunctionsPlugin,
    ));

    app.add_plugins(ScriptingDocgenPlugin::default());
    app.cleanup();
    app.finish();
    app.update();

    // bah bye, the generated file will be found in assets/
    // this can then be passed to various backends to generate docs, and other declaration files
    Ok(())
}
