use bevy::{app::AppExit, core::FrameCount, prelude::*};
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_core::{
    asset::ScriptAsset, script::ScriptComponent, AddContextPreHandlingInitializer,
};

fn load_script(
    server: Res<AssetServer>,
    mut commands: Commands,
    mut handle: Local<Handle<ScriptAsset>>,
) {
    let path = "scripts/event_recipients.lua";
    let handle_ = server.load::<ScriptAsset>(path);
    *handle = handle_;

    commands.spawn(ScriptComponent::new(vec![
        "scripts/event_recipients.lua".into()
    ]));
    commands.spawn(ScriptComponent::new(vec![
        "scripts/event_recipients.lua".into()
    ]));
}

fn trigger_script_callback(mut writer: EventWriter<ScriptCallbackEvent<usize>>) {
    writer.send(ScriptCallbackEvent::new_for_all(
        OnEventCallback::into_callback_label(),
        1,
    ));

    writer.send(ScriptCallbackEvent::new(
        OnEventCallback::into_callback_label(),
        2,
        Recipients::Entity(Entity::from_raw(6)),
    ));
}

fn quit_after_few_frames(mut exit: EventWriter<AppExit>, count: Res<FrameCount>) {
    if count.0 > 5 {
        exit.send(AppExit::Success);
    }
}

pub struct OnEventCallback;
impl IntoCallbackLabel for OnEventCallback {
    fn into_callback_label() -> CallbackLabel {
        "on_event".into()
    }
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(LuaScriptingPlugin::<usize>::default())
        .add_context_pre_handling_initializer::<()>(|_, e, ctx: &mut Lua| {
            ctx.globals().set("entity", format!("{e:?}")).unwrap();
            Ok(())
        })
        .add_systems(Startup, load_script)
        .add_systems(
            Update,
            (
                trigger_script_callback,
                event_handler::<OnEventCallback, usize, Lua, ()>.after(trigger_script_callback),
                quit_after_few_frames,
            ),
        )
        .run();

    Ok(())
}
