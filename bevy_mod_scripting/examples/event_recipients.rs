use bevy::prelude::*;
use bevy_console::ConsolePlugin;
use bevy_event_priority::PriorityEventWriter;
use bevy_mod_scripting::{
    APIProvider, AddScriptHost, AddScriptHostHandler, LuaEvent, LuaFile, RLuaScriptHost,
    Recipients, Script, ScriptCollection, ScriptingPlugin, ScriptError,
};
use rand::prelude::SliceRandom;
use tealr::mlu::mlua::{Lua, ToLua};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{atomic::AtomicU32, Mutex};

#[derive(Clone)]
pub struct MyLuaArg(usize);

impl<'lua> ToLua<'lua> for MyLuaArg {
    fn to_lua(self, lua: &'lua Lua) -> tealr::mlu::mlua::Result<tealr::mlu::mlua::Value<'lua>> {
        self.0.to_lua(lua)
    }
}

#[derive(Default)]
pub struct LuaAPIProvider {}

/// the custom Lua api, world is provided via a global pointer,
/// and callbacks are defined only once at script creation
impl APIProvider for LuaAPIProvider {
    type Ctx = Mutex<Lua>;
    fn attach_api(ctx: &mut Self::Ctx) -> Result<(),ScriptError>  {
        // callbacks can receive any `ToLuaMulti` arguments, here '()' and
        // return any `FromLuaMulti` arguments, here a `usize`
        // check the Rlua documentation for more details

        let ctx = ctx.lock().unwrap();

        ctx.globals().set("print",ctx.create_function(|_ctx, msg: String| {
            info!("{}", msg);
            Ok(())
        })?)?;

        Ok(())
    }
}

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// utility for generating random events from a list
fn fire_random_event(w: &mut PriorityEventWriter<LuaEvent<MyLuaArg>>, events: &[ScriptEventData]) {
    let mut rng = rand::thread_rng();
    let id = COUNTER.fetch_add(1, Relaxed);
    let arg = MyLuaArg(id as usize);
    let event = events
        .choose(&mut rng)
        .map(|v| LuaEvent {
            hook_name: v.0.to_string(),
            args: vec![arg],
            recipients: v.1.clone(),
        })
        .unwrap();

    info!(
        "\t - event: {},\t recipients: {:?},\t id: {}",
        event.hook_name, event.recipients, id
    );
    w.send(event, 0);
}

fn do_update(mut w: PriorityEventWriter<LuaEvent<MyLuaArg>>) {
    info!("Update, firing:");

    let all_events = [
        ScriptEventData("on_event", Recipients::All),
        ScriptEventData("on_event", Recipients::ScriptID(0)),
        ScriptEventData("on_event", Recipients::ScriptID(1)),
        ScriptEventData(
            "on_event",
            Recipients::ScriptName("scripts/event_recipients.lua".to_owned()),
        ),
    ];

    // fire random event, for any stages
    fire_random_event(&mut w, &all_events);
}

#[derive(Clone)]
pub struct ScriptEventData(&'static str, Recipients);

fn load_our_scripts(server: Res<AssetServer>, mut commands: Commands) {
    // spawn two identical scripts
    // id's will be 0 and 1
    let path = "scripts/event_recipients.lua";
    let handle = server.load::<LuaFile, &str>(path);
    let scripts = (0..2)
        .into_iter()
        .map(|_| {
            Script::<LuaFile>::new::<RLuaScriptHost<MyLuaArg, LuaAPIProvider>>(
                path.to_string(),
                handle.clone(),
            )
        })
        .collect();

    commands
        .spawn()
        .insert(ScriptCollection::<LuaFile> { scripts });
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_plugin(ConsolePlugin)
        .add_startup_system(load_our_scripts)
        // randomly fire events for either all scripts,
        // the script with id 0
        // or the script with id 1
        .add_system(do_update)
        .add_script_handler_stage::<RLuaScriptHost<MyLuaArg, LuaAPIProvider>, _, 0, 0>(
            CoreStage::PostUpdate,
        )
        .add_script_host::<RLuaScriptHost<MyLuaArg, LuaAPIProvider>, _>(CoreStage::PostUpdate);

    app.run();

    Ok(())
}
