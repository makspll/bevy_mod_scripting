use asset::ScriptAsset;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
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
    vec_of_usize: Vec<usize>,
    vec_of_usize2: Vec<usize>,
    map_of_strings: HashMap<String, String>,
    option_usize: Option<usize>,
    option_vec3: Option<Vec3>,
    vec_of_option_bools: Vec<Option<bool>>,
    option_vec_of_bools: Option<Vec<bool>>,
}

const SCRIPT_NAME: &str = "scripts/bevy_api.lua";

fn load_script(server: Res<AssetServer>, mut handle: Local<Handle<ScriptAsset>>) {
    let handle_ = server.load::<ScriptAsset>(SCRIPT_NAME);
    *handle = handle_;
}

fn init_data(mut commands: Commands) {
    commands.spawn((
        MyComponent {
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
            vec_of_usize: vec![1, 2, 3],
            vec_of_usize2: vec![4, 5, 6],
            map_of_strings: HashMap::from_iter(vec![("key".into(), "value".into())]),
            option_usize: None,
        },
        ScriptComponent::new(vec![SCRIPT_NAME.into()]),
    ));
}

struct EventCallbackLabel;
impl IntoCallbackLabel for EventCallbackLabel {
    fn into_callback_label() -> CallbackLabel {
        "on_event".into()
    }
}

pub fn send_event(mut writer: EventWriter<ScriptCallbackEvent<()>>) {
    writer.send(ScriptCallbackEvent::new_for_all(
        EventCallbackLabel::into_callback_label(),
        (),
    ));
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(LuaScriptingPlugin::<()>::default())
        .add_plugins(LuaBevyScriptingPlugin)
        .register_type::<MyComponent>()
        .add_systems(Startup, (init_data, load_script))
        .add_systems(
            Update,
            (
                send_event,
                event_handler::<EventCallbackLabel, (), Lua, ()>.after(send_event),
            ),
        );

    app.run();

    Ok(())
}
