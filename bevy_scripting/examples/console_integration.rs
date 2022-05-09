use bevy::{ecs::event::Events, prelude::*};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin, PrintConsoleLine};
use bevy_scripting::{
    APIProvider, AddScriptHost, LuaEvent, LuaFile, RLuaScriptHost, Script, ScriptCollection,
    ScriptHost, ScriptingPlugin,
};
use rlua::{prelude::LuaLightUserData, Lua};
use std::sync::Mutex;

#[derive(Default)]
pub struct LuaAPIProvider {}

/// the custom Lua api, world is provided via a global pointer,
/// and callbacks are defined only once at script creation
impl APIProvider for LuaAPIProvider {
    type Ctx = Mutex<Lua>;
    fn attach_api(ctx: &Self::Ctx) {
        // callbacks can receive any `ToLuaMulti` arguments, here '()' and
        // return any `FromLuaMulti` arguments, here a `usize`
        // check the Rlua documentation for more details
        RLuaScriptHost::<Self>::register_api_callback(
            "print_to_console",
            |ctx, msg: String| {
                // retrieve the world pointer
                let world_data: LuaLightUserData = ctx.globals().get("world").unwrap();
                let world = unsafe { &mut *(world_data.0 as *mut World) };

                // do stuff with it
                // ...

                let mut events: Mut<Events<PrintConsoleLine>> = world.get_resource_mut().unwrap();
                events.send(PrintConsoleLine { line: msg });

                // return something
                Ok(())
            },
            ctx,
        )
    }
}

/// sends updates to script host which are then handled by the scripts
/// in the designated stage
pub fn trigger_on_update_script_callback(mut w: EventWriter<LuaEvent>) {
    let event = LuaEvent {
        hook_name: "on_update".to_string(),
        args: Vec::default(),
    };

    w.send(event);
}

/// optional, convenience for loading our script assets provided by bevy_asset_loader
/// keeps all of them loaded
#[derive(AssetCollection)]
struct LuaAssets {
    #[asset(path = "scripts", folder(typed))]
    folder: Vec<Handle<LuaFile>>,
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_plugin(ConsolePlugin)
        .add_startup_system(watch_assets)
        .add_state(GameState::AssetLoading)
        // register bevy_console commands
        .add_console_command::<RunScriptCmd, _, _>(run_script_cmd)
        .add_console_command::<DeleteScriptCmd, _, _>(
            delete_script_cmd::<RLuaScriptHost<LuaAPIProvider>>,
        )
        .add_system(trigger_on_update_script_callback)
        // choose and register script host
        .add_script_host::<RLuaScriptHost<LuaAPIProvider>, CoreStage>(CoreStage::PostUpdate);

    // bevy_asset_loader for loading and keeping script assets around easilly
    AssetLoader::new(GameState::AssetLoading)
        .continue_to_state(GameState::MainMenu)
        .with_collection::<LuaAssets>()
        .build(&mut app);

    // at runtime press '~' for console then type in help for command formats
    app.run();

    Ok(())
}

// we use bevy-debug-console to demonstrate how this can fit in in the runtime of a game
// note that using just the entity id instead of the full Entity has issues,
// but since we aren't despawning/spawning entities this works in our case
#[derive(ConsoleCommand)]
#[console_command(name = "run_script")]
///Runs a Lua script from the `assets/scripts` directory
pub struct RunScriptCmd {
    /// the relative path to the script, e.g.: `/hello.lua` for a script located in `assets/scripts/hello.lua`
    pub path: String,

    /// the entity id to attach this script to
    pub entity: Option<u32>,
}

pub fn run_script_cmd(
    mut log: ConsoleCommand<RunScriptCmd>,
    server: Res<AssetServer>,
    mut commands: Commands,
    mut existing_scripts: Query<
        &mut ScriptCollection<<RLuaScriptHost<LuaAPIProvider> as ScriptHost>::ScriptAsset>,
    >,
) {
    if let Some(RunScriptCmd { path, entity }) = log.take() {
        let handle = server.load::<LuaFile, &str>(&format!("scripts/{}", &path));

        match entity {
            Some(e) => {
                if let Ok(mut scripts) = existing_scripts.get_mut(Entity::from_raw(e)) {
                    info!("Creating script: scripts/{} {:?}", &path, &entity);

                    scripts.scripts.push(Script::<
                        <RLuaScriptHost<LuaAPIProvider> as ScriptHost>::ScriptAsset,
                    >::new::<RLuaScriptHost<LuaAPIProvider>>(
                        path, handle
                    ));
                } else {
                    log.reply_failed(format!("Something went wrong"));
                };
            }
            None => {
                info!("Creating script: scripts/{}", &path);

                commands.spawn().insert(ScriptCollection::<
                    <RLuaScriptHost<LuaAPIProvider> as ScriptHost>::ScriptAsset,
                > {
                    scripts: vec![Script::<
                        <RLuaScriptHost<LuaAPIProvider> as ScriptHost>::ScriptAsset,
                    >::new::<RLuaScriptHost<LuaAPIProvider>>(
                        path, handle
                    )],
                });
            }
        };
    }
}

/// optional, hot reloading
fn watch_assets(server: Res<AssetServer>) {
    server.watch_for_changes().unwrap();
}

pub fn delete_script_cmd<H: ScriptHost>(
    mut log: ConsoleCommand<DeleteScriptCmd>,
    mut scripts: Query<(Entity, &mut ScriptCollection<H::ScriptAsset>)>,
) {
    if let Some(DeleteScriptCmd { name, entity_id }) = log.take() {
        for (e, mut s) in scripts.iter_mut() {
            if e.id() == entity_id {
                let old_len = s.scripts.len();
                s.scripts.retain(|s| s.name() != name);

                if old_len > s.scripts.len() {
                    log.reply_ok(format!("Deleted script {}, on entity: {}", name, entity_id));
                } else {
                    log.reply_failed(format!(
                        "Entity {} did own a script named: {}",
                        entity_id, name
                    ))
                };
                return;
            }
        }

        log.reply_failed("Could not find given entity ID with a script")
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    MainMenu,
}

#[derive(ConsoleCommand)]
#[console_command(name = "delete_script")]
///Runs a Lua script from the `assets/scripts` directory
pub struct DeleteScriptCmd {
    /// the name of the script
    pub name: String,

    /// the entity the script is attached to
    pub entity_id: u32,
}
