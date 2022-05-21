use bevy::prelude::*;
use bevy_event_priority::PriorityEventWriter;
use bevy_mod_scripting::ReflectCustomUserData;
use bevy_mod_scripting::{
    APIProvider, AddScriptHost, AddScriptHostHandler, LuaEvent, LuaFile, RLuaScriptHost,
    Recipients, Script, ScriptCollection, ScriptingPlugin,
};

use rlua::{Lua, MetaMethod, ToLua, UserData, Value};

use std::sync::Mutex;

#[derive(Clone)]
pub struct MyLuaArg;

impl<'lua> ToLua<'lua> for MyLuaArg {
    fn to_lua(self, _lua: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
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
        ).unwrap();
    }
}

fn fire_script_update(mut w: PriorityEventWriter<LuaEvent<MyLuaArg>>) {
    w.send(
        LuaEvent::<MyLuaArg> {
            hook_name: "on_update".to_owned(),
            args: Vec::default(),
            recipients: Recipients::All,
        },
        0,
    )
}

#[derive(Clone, Reflect, Default)]
#[reflect(CustomUserData)]
pub struct MyUserData {
    x: u32,
    y: u32,
}

impl UserData for MyUserData {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_, s, ()| {
            Ok(format!("({},{})", s.x, s.y))
        });

        methods.add_meta_method(MetaMethod::Add, |_, s, o: MyUserData| {
            Ok(MyUserData {
                x: s.x.wrapping_add(o.x),
                y: s.y.wrapping_add(o.y),
            })
        });
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MyComponent {
    vec: MyUserData,
}

fn load_our_script(server: Res<AssetServer>, mut commands: Commands) {
    let path = "scripts/custom_user_data.lua";
    let handle = server.load::<LuaFile, &str>(path);

    commands
        .spawn()
        .insert(ScriptCollection::<LuaFile> {
            scripts: vec![Script::<LuaFile>::new(path.to_string(), handle)],
        })
        .insert(MyComponent {
            vec: MyUserData { x: 2, y: 3 },
        });
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_startup_system(load_our_script)
        .add_system(fire_script_update)
        .add_stage_before(
            CoreStage::PostUpdate,
            "scripts",
            SystemStage::single_threaded(),
        )
        .add_script_handler_stage::<RLuaScriptHost<MyLuaArg, LuaAPIProvider>, _, 0, 0>("scripts")
        .register_type::<MyUserData>()
        .register_type::<MyComponent>()
        // this stage handles addition and removal of script contexts, we can safely use `CoreStage::PostUpdate`
        .add_script_host::<RLuaScriptHost<MyLuaArg, LuaAPIProvider>, _>(CoreStage::PostUpdate);
    app.run();

    Ok(())
}
