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
use bevy_mod_scripting_core::bindings::globals::core::CoreScriptGlobalsPlugin;
use ladfile_builder::plugin::ScriptingDocgenPlugin;

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
        ScriptingDocgenPlugin::default(),
    ));

    // run once
    app.cleanup();
    app.finish();
    app.update();

    // bah bye
    Ok(())
}
