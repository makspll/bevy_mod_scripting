use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(ScriptingPlugin)
        .add_plugins(AssetPlugin::default());

    static INVALID_ARGUMENT_WARNING: &str = "Expected one of: 'lua','rhai' as arguments";

    let lang = args.get(1).expect(INVALID_ARGUMENT_WARNING);

    match lang.as_str() {
        "lua" => {
            #[cfg(all(feature = "lua", feature = "lua_script_api"))]
            app.add_script_host::<LuaScriptHost<()>>(PostUpdate)
                .add_api_provider::<LuaScriptHost<()>>(Box::new(BevyAPIProvider))
                .update_documentation::<LuaScriptHost<()>>();

            #[cfg(any(not(feature = "lua"), not(feature = "lua_script_api")))]
            println!("Re-run with the following features enabled: `lua`,`lua_script_api`")
        }
        "rhai" => {
            println!("Rhai documentation generation is not supported yet");
        }
        _ => println!("{}", INVALID_ARGUMENT_WARNING),
    }
}
