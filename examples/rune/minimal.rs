use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScriptingPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .add_script_host::<RuneScriptHost<()>>(PostUpdate)
        .add_script_handler::<RuneScriptHost<()>, 0, 1>(PostUpdate)
        .add_api_provider::<RuneScriptHost<()>>(Box::new(MyAPIProvider))
        .run()
}

struct MyAPIProvider;

impl APIProvider for MyAPIProvider {
    type APITarget = Context;
    type ScriptContext = RuneScriptContext;
    type DocTarget = RuneDocFragment;

    fn attach_api(&mut self, api: &mut Self::APITarget) -> Result<(), ScriptError> {
        let mut module = rune::Module::new();

        module
            .function("print_fancy", |msg: &str| println!("✨ {msg} ✨"))
            .build()
            .map_err(ScriptError::new_other)?;

        api.install(module).map_err(ScriptError::new_other)?;

        Ok(())
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let script_path = "scripts/minimal.rune";

    commands.spawn(ScriptCollection::<RuneFile> {
        scripts: vec![Script::new(
            script_path.to_owned(),
            asset_server.load(script_path),
        )],
    });
}

fn update(mut events: PriorityEventWriter<RuneEvent<()>>) {
    events.send(
        RuneEvent {
            hook_name: "on_event".into(),
            args: (),
            recipients: Recipients::All,
        },
        0,
    );
}
