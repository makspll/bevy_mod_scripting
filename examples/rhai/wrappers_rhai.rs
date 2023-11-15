use bevy::{app::AppExit, prelude::*};
use bevy_mod_scripting_core::ScriptingPlugin;
use bevy_script_api::rhai::{ReflectRhaiProxyable, std::RhaiCopy};

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

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(ScriptingPlugin)
        .register_type::<MyThing>()
        .init_resource::<MyThing>();

    app.run();

    Ok(())
}
