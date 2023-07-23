use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*};
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

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(0)),
        ..Default::default()
    }))
    .add_plugins(ScriptingPlugin)
    .add_systems(Startup, load_our_scripts)
    // randomly fire events for either all scripts,
    // the script with id 0
    // or the script with id 1
    .add_systems(Update, do_update)
    .add_script_handler::<LuaScriptHost<()>, 0, 0>(PostUpdate)
    .add_script_host::<LuaScriptHost<()>>(PostUpdate);
    app.run();

    Ok(())
}
