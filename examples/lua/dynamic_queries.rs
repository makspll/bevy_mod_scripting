use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ComponentA;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ComponentB;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ComponentC;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, ScriptingPlugin))
        .register_type::<ComponentA>()
        .register_type::<ComponentB>()
        .register_type::<ComponentC>()
        .add_script_host::<LuaScriptHost<()>>(PostUpdate)
        .add_script_handler::<LuaScriptHost<()>, 0, 0>(PostUpdate)
        .add_api_provider::<LuaScriptHost<()>>(Box::new(LuaBevyAPIProvider))
        .add_api_provider::<LuaScriptHost<()>>(Box::new(LuaCoreBevyAPIProvider))
        .add_systems(Startup, (setup, apply_deferred, run).chain())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((ComponentA,));
    commands.spawn((ComponentA, ComponentB));
    commands.spawn((ComponentA, ComponentC));
    commands.spawn((ComponentA, ComponentB, ComponentC));

    commands.spawn((ComponentB,));
    commands.spawn((ComponentB, ComponentC));
    commands.spawn((ComponentB, ComponentA));
    commands.spawn((ComponentB, ComponentA, ComponentC));

    commands.spawn((ComponentC,));
    commands.spawn((ComponentC, ComponentA));
    commands.spawn((ComponentC, ComponentB));
    commands.spawn((ComponentC, ComponentA, ComponentB));

    let path = "scripts/dynamic_queries.lua";
    let handle = asset_server.load(path);

    commands.spawn(ScriptCollection::<LuaFile> {
        scripts: vec![Script::new(path.into(), handle)],
    });
}

fn run(mut events: PriorityEventWriter<LuaEvent<()>>) {
    events.send(
        LuaEvent {
            hook_name: "on_event".into(),
            args: (),
            recipients: Recipients::All,
        },
        0,
    );
}
