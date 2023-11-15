use bevy::prelude::*;
use bevy_mod_scripting_core::{
    hosts::{APIProvider, Recipients, ScriptHost},
    AddScriptApiProvider, AddScriptHost, AddScriptHostHandler, ScriptingPlugin,
};
use bevy_mod_scripting_rhai::{
    docs::RhaiDocFragment, rhai::Engine, RhaiContext, RhaiEvent, RhaiScriptHost,
};
use bevy_script_api::rhai::{
    bevy::RhaiBevyAPIProvider,
    std::{RegisterVecType, RhaiCopy},
    ReflectRhaiProxyable, RegisterForeignRhaiType,
};

// Step 1. Rust representation
// construct all our types and functionality
// Reflect is neccessary to allow access from scripts
// Clone allows receiving our wrapper as a function parameter (derives FromLua via UserData through mlua's traits)
// We can still use references to NonClone wrappers via AnyUserData in lua methods.
// Even though we are implementing Clone we are still able to reference the original data in script thanks to the script wrapper we are about to implement
// Debug is nice to have, we can forward that implementation to Lua's ToString via our macro
#[derive(Resource, Reflect, Default, Clone, Debug)]
#[reflect(Resource, RhaiProxyable)]
pub struct MyThing {
    usize: usize,
    string: String,
    array: Vec<usize>,
}

impl RhaiCopy for MyThing {}

impl MyThing {
    pub fn do_something_cool(&self) -> String {
        self.string.clone()
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(MyThing {
        array: vec![1, 2, 3],
        string: "Hello World!".to_owned(),
        usize: 42,
    });
}

fn run_one_shot(world: &mut World) {
    world.resource_scope(|world, mut host: Mut<RhaiScriptHost<()>>| {
        host.run_one_shot(
            r#"
                    fn once() {
                        print("hello hello");
                    }
                "#
            .as_bytes(),
            "script.rhai",
            Entity::from_raw(0),
            world,
            RhaiEvent {
                hook_name: "once".to_owned(),
                args: (),
                recipients: Recipients::All,
            },
        )
        .expect("Something went wrong in the script!");
    });
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, ScriptingPlugin))
        .add_script_host::<RhaiScriptHost<()>>(PostUpdate)
        .add_api_provider::<RhaiScriptHost<()>>(Box::new(RhaiBevyAPIProvider))
        .add_api_provider::<RhaiScriptHost<()>>(Box::new(WrapperApiProvider))
        .add_script_handler::<RhaiScriptHost<()>, 0, 2>(PostUpdate)
        .register_type::<MyThing>()
        .add_systems(Startup, init)
        .add_systems(Update, run_one_shot.run_if(run_once()));

    app.run();

    Ok(())
}

struct WrapperApiProvider;

impl APIProvider for WrapperApiProvider {
    type APITarget = Engine;

    type ScriptContext = RhaiContext;

    type DocTarget = RhaiDocFragment;

    fn register_with_app(&self, app: &mut App) {
        app.register_foreign_rhai_type::<Vec<usize>>();
    }

    fn attach_api(
        &mut self,
        api: &mut Self::APITarget,
    ) -> Result<(), bevy_mod_scripting_core::prelude::ScriptError> {
        api.register_vec_functions::<usize>();
        Ok(())
    }
}
