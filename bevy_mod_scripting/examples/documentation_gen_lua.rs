use bevy::prelude::*;
use bevy_console::ConsolePlugin;
use bevy_event_priority::PriorityEventWriter;
use bevy_mod_scripting::{
    APIProvider, AddScriptHost, AddScriptHostHandler, LuaEvent, LuaFile, RLuaScriptHost,
    Recipients, Script, ScriptCollection, ScriptingPlugin, ScriptError, AddScriptApiProvider, LuaDocFragment, GenDocumentation,
};
use rand::prelude::SliceRandom;
use tealr::{TypeWalker, TypeName};
use tealr::mlu::TypedFunction;
use tealr::mlu::{mlua,mlua::{Lua, ToLua},TealData};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{atomic::AtomicU32, Mutex};

#[derive(Clone)]
pub struct MyLuaArg(usize);

impl<'lua> ToLua<'lua> for MyLuaArg {
    fn to_lua(self, lua: &'lua Lua) -> tealr::mlu::mlua::Result<tealr::mlu::mlua::Value<'lua>> {
        self.0.to_lua(lua)
    }
}





#[derive(Clone,tealr::MluaUserData, TypeName)]
/// This is acts as a documentation and function holder
/// We can add some general documentation about what it holds
/// but also specific function level documenation
pub struct APIModule;


impl TealData for APIModule {
    fn add_methods<'lua, T: tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {

        methods.document_type("This is module level documentation for our api, it will be shown first");
        methods.document_type("");

        methods.document("Here we document the next function");
        methods.document("## Markdown!:");
        methods.document(
            r#"```lua
            local hello = "string";
            ```\n"#);
        methods.add_function("my_function", |_,s: String| Ok("hello world!"));

        methods.generate_help();
    }

}


/// This is tealr's way to export global items
/// Here `my_api` will be available globally in the lua script
struct Export;
impl tealr::mlu::ExportInstances for Export {
    fn add_instances<'lua, T: tealr::mlu::InstanceCollector<'lua>>(instance_collector: &mut T) -> mlua::Result<()> {
        instance_collector.add_instance("my_api".into(), |_| {
            Ok(APIModule)
        })

    }
}


#[derive(Default)]
pub struct LuaAPIProvider;

/// the custom Lua api, world is provided via a global pointer,
/// and callbacks are defined only once at script creation
impl APIProvider for LuaAPIProvider {
    type Target = Mutex<Lua>;
    type DocTarget = LuaDocFragment;

    fn attach_api(&mut self,ctx: &mut Self::Target) -> Result<(),ScriptError> {
       // callbacks can receive any `ToLuaMulti` arguments, here '()' and
        // return any `FromLuaMulti` arguments, here a `usize`
        // check the Rlua documentation for more details

        let ctx = ctx.lock().unwrap();

        // equivalent to ctx.globals().set() but for multiple items
        tealr::mlu::set_global_env::<Export>(&ctx).unwrap();

        Ok(())   
    }

    fn get_doc_fragment(&self) -> Option<Self::DocTarget>{
        Some(
            LuaDocFragment::new(
                // we must select items we want included in the documentation
                TypeWalker::new()
                    .process_type::<APIModule>()
                    .document_global_instance::<Export>().unwrap()
                    ,
                // you probably want to keep this false in most cases
                false,
                // this is the name of the exported .d.tl file top level export 
                "my_lib".to_string()
            )
        )
    }

    
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_plugin(ConsolePlugin)
        .add_script_host::<RLuaScriptHost<MyLuaArg>, _>(CoreStage::PostUpdate)
        .add_api_provider::<RLuaScriptHost<MyLuaArg>>(Box::new(LuaAPIProvider))
        // this is a no-op unless the `GEN_SCRIPT_DOC` env variable is set
        .gen_documentation::<RLuaScriptHost<MyLuaArg>>();

    app.run();

    Ok(())
}
