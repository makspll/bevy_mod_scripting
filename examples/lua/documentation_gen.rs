use bevy::prelude::*;

use bevy_mod_scripting::{
    api::{impl_tealr_type, lua::bevy::LuaBevyAPIProvider},
    prelude::*,
};

use std::sync::Mutex;

#[derive(Clone)]
pub struct MyLuaArg;

impl<'lua> ToLua<'lua> for MyLuaArg {
    fn to_lua(self, _lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(Value::Nil)
    }
}

#[derive(Clone)]
/// This is acts as a documentation and function holder
/// We can add some general documentation about what it holds
/// but also specific function level documenation
pub struct APIModule;

impl_tealr_type!(APIModule);

impl TealData for APIModule {
    fn add_methods<'lua, T: tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods
            .document_type("This is type level documentation for our api, it will be shown first");
        methods.document_type("");

        methods.document("Here we document the next function");
        methods.document("## Markdown!:");
        methods.document(
            "```lua
                local hello = \"string\"  
            \n```",
        );
        methods.add_function("my_function", |_, ()| Ok("hello world!"));
        methods.generate_help();
    }
}

/// This is tealr's way to export global items
/// Here `my_api` will be available globally in the lua script

#[derive(Default)]
struct Export;
impl tealr::mlu::ExportInstances for Export {
    fn add_instances<'lua, T: tealr::mlu::InstanceCollector<'lua>>(
        self,
        instance_collector: &mut T,
    ) -> mlua::Result<()> {
        instance_collector.document_instance("Documentation for the exposed global variable");
        instance_collector.add_instance("my_api", |_| Ok(APIModule))?;

        Ok(())
    }
}

#[derive(Default)]
pub struct LuaAPIProvider;

/// the custom Lua api, world is provided via a global pointer,
/// and callbacks are defined only once at script creation
impl APIProvider for LuaAPIProvider {
    type APITarget = Mutex<Lua>;
    type DocTarget = LuaDocFragment;
    type ScriptContext = Mutex<Lua>;

    fn attach_api(&mut self, ctx: &mut Self::APITarget) -> Result<(), ScriptError> {
        // callbacks can receive any `ToLuaMulti` arguments, here '()' and
        // return any `FromLuaMulti` arguments, here a `usize`
        // check the Rlua documentation for more details

        let ctx = ctx.get_mut().unwrap();

        // equivalent to ctx.globals().set() but for multiple items
        tealr::mlu::set_global_env(Export, ctx).map_err(ScriptError::new_other)?;

        Ok(())
    }

    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        Some(LuaDocFragment::new("MyAPI", |tw|
                    // we must select items we want included in the documentation
                    tw.process_type::<APIModule>()
                    .document_global_instance::<Export>().unwrap()))
    }

    fn register_with_app(&self, _app: &mut App) {}
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        // add the providers and script host
        .add_script_host::<LuaScriptHost<MyLuaArg>, _>(CoreStage::PostUpdate)
        .add_api_provider::<LuaScriptHost<MyLuaArg>>(Box::new(LuaAPIProvider))
        .add_api_provider::<LuaScriptHost<MyLuaArg>>(Box::new(LuaBevyAPIProvider))
        // this needs to be placed after any `add_api_provider` and `add_script_host` calls
        // it will generate `doc` and `types` folders under `assets/scripts` containing the documentation and teal declaration files
        // respectively. See example asset folder to see how they look like. The `teal_file.tl` script in example assets shows the usage of one of those
        // declaration files, use the teal vscode extension to explore the type hints!
        // Note: This is a noop in optimized builds unless the `doc_always` feature is enabled!
        .update_documentation::<LuaScriptHost<MyLuaArg>>()
        .add_script_handler_stage::<LuaScriptHost<MyLuaArg>, _, 0, 0>(CoreStage::PostUpdate);

    // app.run(); no need, documentation gets generated before the app even starts

    Ok(())
}
