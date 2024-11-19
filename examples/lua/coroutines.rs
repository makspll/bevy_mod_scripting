use asset::ScriptAsset;
use bevy::prelude::*;
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use bevy_mod_scripting::prelude::*;
use script::ScriptComponent;

struct OnEventCallback;
impl IntoCallbackLabel for OnEventCallback {
    fn into_callback_label() -> CallbackLabel {
        "on_update".into()
    }
}

fn load_script(
    server: Res<AssetServer>,
    mut commands: Commands,
    mut handle: Local<Handle<ScriptAsset>>,
) {
    let path = "scripts/coroutines.lua";
    let handle_ = server.load::<ScriptAsset>(path);
    *handle = handle_;

    commands.spawn(ScriptComponent::new(vec![path.into()]));
}

fn send_event(mut writer: EventWriter<ScriptCallbackEvent<()>>) {
    writer.send(ScriptCallbackEvent::new_for_all(
        OnEventCallback::into_callback_label(),
        (),
    ));
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(LuaScriptingPlugin::<()>::default())
        .add_systems(Startup, load_script)
        .add_systems(
            Update,
            (
                send_event,
                event_handler::<OnEventCallback, (), Lua, ()>.after(send_event),
            ),
        );

    app.run();

    Ok(())
}
