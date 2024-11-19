use asset::ScriptAsset;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_lua::bindings::providers::LuaBevyScriptingPlugin;
use script::ScriptComponent;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MyComponent {
    quat: Quat,
    vec2: Vec2,
    usize: usize,
    f32: f32,
    mat3: Mat3,
    option_vec3: Option<Vec3>,
    vec_of_option_bools: Vec<Option<bool>>,
    option_vec_of_bools: Option<Vec<bool>>,
}

fn load_script(
    server: Res<AssetServer>,
    mut commands: Commands,
    mut handle: Local<Handle<ScriptAsset>>,
) {
    let path = "scripts/bevy_api.lua";
    let handle_ = server.load::<ScriptAsset>(path);
    *handle = handle_;

    commands.spawn(ScriptComponent::new(vec![path.into()]));
}

fn init_data(mut commands: Commands) {
    commands.spawn(MyComponent {
        usize: 5,
        vec2: Vec2::new(1.0, 2.0),
        f32: 6.7,
        mat3: Mat3::from_cols(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0),
            Vec3::new(7.0, 8.0, 9.0),
        ),
        quat: Quat::from_xyzw(1.0, 2.0, 3.0, 4.0),
        option_vec3: None,
        vec_of_option_bools: vec![Some(true), None, Some(false)],
        option_vec_of_bools: Some(vec![true, true, true]),
    });
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(LuaScriptingPlugin::<()>::default())
        .add_plugins(LuaBevyScriptingPlugin)
        .register_type::<MyComponent>()
        // note the implementation for Option is there, but we must register `LuaProxyable` for it
        .add_systems(Startup, (init_data, load_script));

    app.run();

    Ok(())
}
