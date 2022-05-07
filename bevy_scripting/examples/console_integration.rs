use bevy::{prelude::*, ecs::event::Events};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin, PrintConsoleLine};
use bevy_scripting::{
    APIProvider, LuaEvent, LuaFile, LuaPlugin, RLuaScriptHost, Script, ScriptHost,
};
use rlua::{prelude::LuaLightUserData, Lua};
use std::sync::Mutex;

/// optional, convenience for loading our script assets provided by bevy_asset_loader
/// keeps all of them loaded
#[derive(AssetCollection)]
struct LuaAssets {
    #[asset(path = "scripts", folder(typed))]
    folder: Vec<Handle<LuaFile>>,
}

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
            |ctx, msg : String| {
                // retrieve the world pointer
                let world_data: LuaLightUserData = ctx.globals().get("world").unwrap();
                let world = unsafe { &mut *(world_data.0 as *mut World) };
                
                // do stuff with it
                // ...

                let mut events : Mut<Events<PrintConsoleLine>> = world.get_resource_mut().unwrap();
                events.send(PrintConsoleLine{ line: msg });


                // return something
                Ok(())
            },
            ctx,
        )
    }
}

// we use bevy-debug-console to demonstrate how this can fit in in the runtime of a game
#[derive(ConsoleCommand)]
#[console_command(name = "run_script")]
///Runs a Lua script from the `assets/scripts` directory
pub struct RunScriptCmd {
    ///the relative path to the script, e.g.: `/hello.lua` for a script located in `assets/scripts/hello.lua`
    pub path: String,
}


pub fn run_script_cmd(
    mut log: ConsoleCommand<RunScriptCmd>,
    server: Res<AssetServer>,
    mut commands: Commands,
) {
    if let Some(RunScriptCmd { path }) = log.take() {
        info!("Running script: scripts/{}", path);

        let handle = server.load::<LuaFile, &str>(&format!("scripts/{}", &path));

        let entity = commands.spawn().insert(Script::<
            <RLuaScriptHost<LuaAPIProvider> as ScriptHost>::ScriptAssetType,
        > {
            handle,
            name: path,
        }).id().id();

        log.reply_ok(format!("Running script on entity with ID: {entity}"))

    }
}

#[derive(ConsoleCommand)]
#[console_command(name = "delete_script")]
///Runs a Lua script from the `assets/scripts` directory
pub struct DeleteScriptCmd {
    ///the entity the script is attached to (only one script can be attached to an entitty as of now)
    pub entity_id: u32,
}


/// optional, hot reloading
fn watch_assets(server: Res<AssetServer>) {
    server.watch_for_changes().unwrap();
}

pub fn delete_script_cmd<H : ScriptHost>(
    mut log: ConsoleCommand<DeleteScriptCmd>,
    mut commands: Commands,
    scripts : Query<(Entity,&Script<H::ScriptAssetType>)>
) {
    if let Some(DeleteScriptCmd { entity_id }) = log.take() {

        for (e,s) in scripts.iter() {
            if e.id() == entity_id {
                commands.entity(e)
                    .remove::<Script<H::ScriptAssetType>>();
                log.reply_ok(format!("Deleted script {}",s.name));
                return;
            }
        };
        
        log.reply_failed("Could not find given entity ID with a script")

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

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    MainMenu,
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(LuaPlugin)
        .add_plugin(ConsolePlugin)
        .add_startup_system(watch_assets)
        .add_state(GameState::AssetLoading)
        .add_console_command::<RunScriptCmd, _, _>(run_script_cmd)
        .add_console_command::<DeleteScriptCmd, _, _>(delete_script_cmd::<RLuaScriptHost<LuaAPIProvider>>)
        .add_system(trigger_on_update_script_callback);

    // bevy_asset_loader
    AssetLoader::new(GameState::AssetLoading)
        .continue_to_state(GameState::MainMenu)
        .with_collection::<LuaAssets>()
        .build(&mut app);

    // bevy_scripting setup
    RLuaScriptHost::<LuaAPIProvider>::register_with_app(&mut app, CoreStage::PostUpdate);

    // at runtime press '~' for console then type in help
    app.run();

    Ok(())
}
