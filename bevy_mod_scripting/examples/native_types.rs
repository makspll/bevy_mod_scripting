use bevy::math::DQuat;
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

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MyComponent {
    dquat: DQuat,
    quat: Quat,
    vec2: Vec2,
    uvec2: UVec2,
    usize: usize,
    f32: f32,
    mat3: Mat3,
    vec4: Vec4,
}

fn load_our_script(server: Res<AssetServer>, mut commands: Commands) {
    let path = "scripts/native_types.lua";
    let handle = server.load::<LuaFile, &str>(path);

    commands
        .spawn()
        .insert(ScriptCollection::<LuaFile> {
            scripts: vec![Script::<LuaFile>::new(path.to_string(), handle)],
        })
        .insert(MyComponent {
            vec2: Vec2::new(1.0,2.0),
            vec4: Vec4::new(1.0,2.0,3.0,4.0),
            uvec2: UVec2::new(3,4),
            usize: 5,
            f32: 6.7,
            mat3: Mat3::from_cols(Vec3::new(1.0,2.0,3.0),Vec3::new(4.0,5.0,6.0),Vec3::new(7.0,8.0,9.0)),
            quat: Quat::from_xyzw(1.0,2.0,3.0,4.0),
            dquat: DQuat::from_xyzw(1.0,2.0,3.0,4.0)
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
        .register_type::<MyComponent>()
        // this stage handles addition and removal of script contexts, we can safely use `CoreStage::PostUpdate`
        .add_script_host::<RLuaScriptHost<MyLuaArg, LuaAPIProvider>, _>(CoreStage::PostUpdate);
    app.run();

    Ok(())
}
