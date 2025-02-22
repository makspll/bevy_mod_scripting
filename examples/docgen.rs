use bevy::{app::App, asset::AssetPlugin, hierarchy::HierarchyPlugin, MinimalPlugins};
use bevy_mod_scripting::ScriptFunctionsPlugin;
use ladfile_builder::plugin::ScriptingDocgenPlugin;

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    // headless bevy
    app.add_plugins((MinimalPlugins, AssetPlugin::default(), HierarchyPlugin));

    // docgen + scripting
    app.add_plugins((ScriptFunctionsPlugin, ScriptingDocgenPlugin::default()));

    // run once
    app.cleanup();
    app.finish();
    app.update();

    // bah bye
    Ok(())
}
