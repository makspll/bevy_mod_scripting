use bevy::{core::FixedTimestep, prelude::*};
use bevy_event_priority::PriorityEventWriter;
use bevy_mod_scripting::{
    APIProvider, AddScriptHost, AddScriptHostHandler, LuaEvent, LuaFile, RLuaScriptHost,
    Recipients, Script, ScriptCollection, ScriptingPlugin, LuaWorld,
};
use rand::prelude::SliceRandom;
use rlua::{Lua, ToLua, Value};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{atomic::AtomicU32, Mutex};

#[derive(Clone)]
pub struct MyLuaArg;

impl<'lua> ToLua<'lua> for MyLuaArg {
    fn to_lua(self, lua: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        Ok(Value::Nil)
    }
}

#[derive(Default)]
pub struct LuaAPIProvider {}

/// the custom Lua api, world is provided via a global pointer,
/// and callbacks are defined only once at script creation
impl APIProvider for LuaAPIProvider {
    type Ctx = Mutex<Lua>;
    fn attach_api(ctx: &mut Self::Ctx) {
        // callbacks can receive any `ToLuaMulti` arguments, here '()' and
        // return any `FromLuaMulti` arguments, here a `usize`
        // check the Rlua documentation for more details
        RLuaScriptHost::<MyLuaArg, Self>::register_api_callback(
            "print",
            |_ctx, msg: String| {
                info!("{}", msg);
                Ok(())
            },
            ctx,
        );

    }
}



fn fire_script_update(mut w: PriorityEventWriter<LuaEvent<MyLuaArg>>) {

    w.send(LuaEvent::<MyLuaArg>{
        hook_name: "on_update".to_owned(),
        args: Vec::default(),
        recipients: Recipients::All,
    }, 0)
}


fn load_our_script(server: Res<AssetServer>, mut commands: Commands) {
    let path = "scripts/basic_example.lua";
    let handle = server.load::<LuaFile, &str>(path);

    commands.spawn().insert(ScriptCollection::<LuaFile> {
        scripts: vec![Script::<LuaFile>::new(path.to_string(), handle)],
    });
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_startup_system(load_our_script)
        .add_system(fire_script_update)
        .add_stage_before(CoreStage::PostUpdate,"scripts", SystemStage::single_threaded())
        .add_script_handler_stage::<RLuaScriptHost<MyLuaArg,LuaAPIProvider>, _, 0, 0>("scripts")
        // this stage handles addition and removal of script contexts, we can safely use `CoreStage::PostUpdate`
        .add_script_host::<RLuaScriptHost<MyLuaArg,LuaAPIProvider>, _>(CoreStage::PostUpdate);
    app.run();

    Ok(())
}
