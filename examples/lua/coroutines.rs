use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;

/// fire on_update
fn do_update(mut w: PriorityEventWriter<LuaEvent<()>>, time: Res<Time>) {
    let event = LuaEvent {
        hook_name: "on_update".to_owned(),
        args: (),
        recipients: Recipients::All,
    };
    info!(
        "\t - event: {}, time: {:?}",
        event.hook_name,
        (time.startup() + time.elapsed())
    );
    w.send(event, 0);
}

fn load_our_scripts(server: Res<AssetServer>, mut commands: Commands) {
    let path = "scripts/coroutines.lua";
    let handle = server.load::<LuaFile, &str>(path);
    let script = Script::<LuaFile>::new(path.to_string(), handle);

    commands.spawn(()).insert(ScriptCollection::<LuaFile> {
        scripts: vec![script],
    });
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_startup_system(load_our_scripts)
        // randomly fire events for either all scripts,
        // the script with id 0
        // or the script with id 1
        .add_systems(Update, do_update)
        .add_script_handler_to_base_set::<LuaScriptHost<()>, _, 0, 0>(CoreSet::PostUpdate)
        .add_script_host_to_base_set::<LuaScriptHost<()>, _>(CoreSet::PostUpdate);
    app.run();

    Ok(())
}
