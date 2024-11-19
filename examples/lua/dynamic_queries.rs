use asset::ScriptAsset;
use bevy::{core::FrameCount, prelude::*};
use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_lua::{bindings::providers::LuaBevyScriptingPlugin, LuaScriptingPlugin};
use script::ScriptComponent;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ComponentA;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ComponentB;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ComponentC;

fn load_script(
    server: Res<AssetServer>,
    mut commands: Commands,
    mut handle: Local<Handle<ScriptAsset>>,
) {
    let path = "scripts/dynamic_queries.lua";
    let handle_ = server.load::<ScriptAsset>(path);
    *handle = handle_;

    commands.spawn(ScriptComponent::new(vec![path.into()]));
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(LuaScriptingPlugin::<()>::default());
    app.add_plugins(LuaBevyScriptingPlugin)
        .register_type::<ComponentA>()
        .register_type::<ComponentB>()
        .register_type::<ComponentC>()
        .add_systems(Startup, (setup, load_script))
        .run();
}

fn setup(mut commands: Commands) {
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
}
