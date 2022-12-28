use bevy::{ecs::event::Events, prelude::*};
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin, PrintConsoleLine};
use bevy_mod_scripting::prelude::*;

use std::sync::Mutex;

#[derive(Default)]
pub struct LuaAPIProvider;

/// the custom Lua api, world is provided via a global pointer,
/// and callbacks are defined only once at script creation
impl APIProvider for LuaAPIProvider {
    type APITarget = Mutex<Lua>;
    type DocTarget = LuaDocFragment;
    type ScriptContext = Mutex<Lua>;

    fn attach_api(&mut self, ctx: &mut Self::APITarget) -> Result<(), ScriptError> {
        // callbacks can receive any `ToLuaMulti` arguments, here '()' and
        // return any `FromLuaMulti` arguments, here a `usize`
        // check the Rlua documentation for more details

        let ctx = ctx.get_mut().unwrap();

        ctx.globals()
            .set(
                "print_to_console",
                ctx.create_function(|ctx, msg: String| {
                    // retrieve the world pointer
                    let world = ctx.get_world()?;
                    let mut world = world.write();

                    let mut events: Mut<Events<PrintConsoleLine>> =
                        world.get_resource_mut().unwrap();
                    events.send(PrintConsoleLine { line: msg });

                    // return something
                    Ok(())
                })
                .map_err(ScriptError::new_other)?,
            )
            .map_err(ScriptError::new_other)?;

        Ok(())
    }

    fn setup_script(
        &mut self,
        _: &ScriptData,
        _: &mut Self::ScriptContext,
    ) -> Result<(), ScriptError> {
        Ok(())
    }
}

/// sends updates to script host which are then handled by the scripts
/// in the designated stage
pub fn trigger_on_update_lua(mut w: PriorityEventWriter<LuaEvent<()>>) {
    let event = LuaEvent {
        hook_name: "on_update".to_string(),
        args: (),
        recipients: Recipients::All,
    };

    w.send(event, 0);
}

pub fn forward_script_err_to_console(
    mut r: EventReader<ScriptErrorEvent>,
    mut w: EventWriter<PrintConsoleLine>,
) {
    for e in r.iter() {
        w.send(PrintConsoleLine {
            line: format!("ERROR:{}", e.error),
        });
    }
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
    mut existing_scripts: Query<&mut ScriptCollection<LuaFile>>,
) {
    if let Some(Ok(RunScriptCmd { path, entity })) = log.take() {
        let handle = server.load::<LuaFile, &str>(&format!("scripts/{}", &path));

        match entity {
            Some(e) => {
                if let Ok(mut scripts) = existing_scripts.get_mut(Entity::from_raw(e)) {
                    info!("Creating script: scripts/{} {:?}", &path, e);
                    scripts.scripts.push(Script::<LuaFile>::new(path, handle));
                } else {
                    log.reply_failed("Something went wrong".to_string());
                };
            }
            None => {
                info!("Creating script: scripts/{}", &path);

                commands.spawn(()).insert(ScriptCollection::<LuaFile> {
                    scripts: vec![Script::<LuaFile>::new(path, handle)],
                });
            }
        };
    }
}

/// optional, hot reloading
fn watch_assets(server: Res<AssetServer>) {
    server.asset_io().watch_for_changes().unwrap();
}

pub fn delete_script_cmd(
    mut log: ConsoleCommand<DeleteScriptCmd>,
    mut scripts: Query<(Entity, &mut ScriptCollection<LuaFile>)>,
) {
    if let Some(Ok(DeleteScriptCmd { name, entity_id })) = log.take() {
        for (e, mut s) in scripts.iter_mut() {
            if e.index() == entity_id {
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

#[derive(ConsoleCommand)]
#[console_command(name = "delete_script")]
///Runs a Lua script from the `assets/scripts` directory
pub struct DeleteScriptCmd {
    /// the name of the script
    pub name: String,

    /// the entity the script is attached to
    pub entity_id: u32,
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_plugin(ConsolePlugin)
        .add_startup_system(watch_assets)
        // register bevy_console commands
        .add_console_command::<RunScriptCmd, _>(run_script_cmd)
        .add_console_command::<DeleteScriptCmd, _>(delete_script_cmd)
        // choose and register the script hosts you want to use
        .add_script_host::<LuaScriptHost<()>, _>(CoreStage::PostUpdate)
        .add_api_provider::<LuaScriptHost<()>>(Box::new(LuaAPIProvider))
        .add_api_provider::<LuaScriptHost<()>>(Box::new(LuaBevyAPIProvider))
        .add_script_handler_stage::<LuaScriptHost<()>, _, 0, 0>(CoreStage::PostUpdate)
        // add your systems
        .add_system(trigger_on_update_lua)
        .add_system(forward_script_err_to_console);

    info!("press '~' to open the console. Type in `run_script \"console_integration.lua\"` to run example script!");
    app.run();

    Ok(())
}
