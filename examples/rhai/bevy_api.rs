use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_rhai::{RhaiEvent, RhaiScriptHost};
/// Let's define a resource, we want it to be "assignable" via lua so we derive `ReflectLuaProxyable`
/// This allows us to reach this value when it's a field under any other Reflectable type

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_script_host::<RhaiScriptHost<()>, _>(CoreStage::PostUpdate)
        .add_system(|world: &mut World| {
            // run script
            world.resource_scope(|world, mut host: Mut<RhaiScriptHost<()>>| {
                host.run_one_shot(
                    r#"
                        fn once(){
                            print("hello")   
                        }
                        "#
                    .as_bytes(),
                    "script.rhai",
                    world,
                    RhaiEvent {
                        hook_name: "once".to_owned(),
                        args: (),
                        recipients: Recipients::All,
                    },
                )
                .expect("Something went wrong in the script!");
            });

            world.send_event(AppExit)
        });

    app.run();

    Ok(())
}
