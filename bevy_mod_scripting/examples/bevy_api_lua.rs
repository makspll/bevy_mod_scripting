use std::marker::PhantomData;

use bevy::math::DQuat;
use bevy::prelude::*;

use bevy::reflect::FromType;
use bevy_event_priority::PriorityEventWriter;
use bevy_mod_scripting::mlu::mlua::UserData;
use bevy_mod_scripting::{ReflectLuaProxyable, AddScriptApiProvider, ValueLuaType, RegisterForeignLuaType};
use bevy_mod_scripting::{
    AddScriptHost, AddScriptHostHandler, LuaEvent, LuaFile, RLuaScriptHost,
    Recipients, Script, ScriptCollection, ScriptingPlugin,
    langs::mlu::mlua,
    lua::LuaBevyAPIProvider
};




#[derive(Clone)]
pub struct MyLuaArg;

impl<'lua> mlua::ToLua<'lua> for MyLuaArg {
    fn to_lua(self, _lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::Nil)
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


#[derive(Default,Clone,Reflect)]
#[reflect(Resource,LuaProxyable)]
pub struct MyResource{
    pub thing: f64
}

impl ValueLuaType for MyResource {}

impl UserData for MyResource {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("custom_resource_method", |_,s,v : f64|{
            s.thing = v;

            Ok("hello?")
        });

        methods.add_meta_method(mlua::MetaMethod::ToString, |_,s,()|{
            Ok(format!("I'm a resource with a custom metatable!: {}",s.thing))
        });
    }
}


#[derive(Default,Reflect)]
pub struct MyReflectThing {
    pub hello : String,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MyComponent {
    dquat: DQuat,
    quat: Quat,
    vec2: Vec2,
    vec3: Vec3,
    uvec2: UVec2,
    usize: usize,
    f32: f32,
    mat3: Mat3,
    vec4: Vec4,
    u8: u8,
    option: Option<Vec3>,
    my_reflect_thing: MyReflectThing,
}

fn load_our_script(server: Res<AssetServer>, mut commands: Commands) {
    let path = "scripts/bevy_api.lua";
    let handle = server.load::<LuaFile, &str>(path);

    commands
        .spawn()
        .insert(ScriptCollection::<LuaFile> {
            scripts: vec![Script::<LuaFile>::new(path.to_string(), handle)],
        })
        .insert(MyComponent {
            vec2: Vec2::new(1.0,2.0),
            vec3: Vec3::new(1.0,2.0,3.0),
            vec4: Vec4::new(1.0,2.0,3.0,4.0),
            uvec2: UVec2::new(3,4),
            usize: 5,
            f32: 6.7,
            mat3: Mat3::from_cols(Vec3::new(1.0,2.0,3.0),Vec3::new(4.0,5.0,6.0),Vec3::new(7.0,8.0,9.0)),
            quat: Quat::from_xyzw(1.0,2.0,3.0,4.0),
            dquat: DQuat::from_xyzw(1.0,2.0,3.0,4.0),
            u8: 240,
            option: None,
            my_reflect_thing: MyReflectThing { hello: "hello world".to_owned() },            
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
        .add_script_handler_stage::<RLuaScriptHost<MyLuaArg>, _, 0, 0>("scripts")
        .register_type::<MyComponent>()
        .register_type::<MyReflectThing>()
        .register_type::<MyResource>()
        // note the implementation for Option is there, but we must register `LuaProxyable` for it, otherwise 
        .register_foreign_lua_type::<Option<Vec3>>()
        .init_resource::<MyResource>()
        // this stage handles addition and removal of script contexts, we can safely use `CoreStage::PostUpdate`
        .add_script_host::<RLuaScriptHost<MyLuaArg>, _>(CoreStage::PostUpdate)
        .add_api_provider::<RLuaScriptHost<MyLuaArg>>(Box::new(LuaBevyAPIProvider));
    
    app.run();

    Ok(())
}
