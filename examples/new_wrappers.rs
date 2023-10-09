use bevy::{app::AppExit, prelude::*};
use bevy_mod_scripting::{api::ScriptProxy, prelude::*};

use std::error::Error;
use std::fmt::Display;
#[derive(Debug)]
pub struct MyError;

impl Display for MyError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for MyError {}

#[derive(ScriptProxy, Reflect, Resource, Default, Debug, Clone)]
#[reflect(Resource, LuaProxyable)]
#[proxy(languages("on_feature(lua)"), derive(Clone))]
#[functions[
    // #[lua(MutatingMethod)]
    // fn set_my_string(&mut self, another_string: Option<String>);

    // #[lua(Method)]
    // fn get_my_string(&self) -> String;

    #[lua(Method,raw)]
    fn raw_method(&self, ctx : &Lua) -> Result<String, _> {
        let a = ctx.globals().get::<_,String>("world").unwrap();
        let a = self.inner()?;
        Ok("".to_owned())
    }


    // #[lua(MetaMethod)]
    // fn ToString(&self) -> String {
    //     format!("{:#?}", _self)
    // }
]]
pub struct MyProxiedStruct {
    my_string: String,
}

impl MyProxiedStruct {
    fn set_my_string(&mut self, another_string: Option<String>) {
        // if let Some(s) = another_string {
        //     self.my_string = s;
        // } else {
        //     self.my_string = "".to_owned();
        // }
    }

    fn get_my_string(&self) -> String {
        // self.my_string.clone()
        "".to_owned()
    }
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(ScriptingPlugin)
        .add_script_host::<LuaScriptHost<()>>(PostUpdate)
        .register_type::<MyProxiedStruct>()
        .init_resource::<MyProxiedStruct>()
        .add_api_provider::<LuaScriptHost<()>>(Box::new(LuaBevyAPIProvider))
        .add_systems(Startup, |world: &mut World| {
            world.insert_resource(MyProxiedStruct {
                // my_string: "I was retrieved from the world".to_owned(),
            });

            // run script
            world.resource_scope(|world, mut host: Mut<LuaScriptHost<()>>| {
                host.run_one_shot(
                    r#"
                    function once()
                        local type = world:get_type_by_name("MyProxiedStruct")
                        local resource = world:get_resource(type)

                        print("The initial value is:", resource)
                        print("The string value is:", resource:get_my_string())
                        
                        resource:set_my_string(nil)
                        print("The string value after calling method with nil is:", resource:get_my_string())
                        
                        resource:set_my_string("I was changed by the script")
                        print("The string value after calling method with string is:", resource:get_my_string())

                    end
                "#
                    .as_bytes(),
                    "script.lua",
                    Entity::from_raw(0),
                    world,
                    LuaEvent {
                        hook_name: "once".to_owned(),
                        args: (),
                        recipients: Recipients::All,
                    },
                )
                .expect("Something went wrong in the script!");
            });

            // print current state of MyThing
            let my_thing = world
                .get_resource::<MyProxiedStruct>()
                .expect("Could not find MyProxiedStruct Resource");
            println!("After the script MyProxiedStruct resource is now: {my_thing:#?}");
            // exit app
            world.send_event(AppExit)
        });

    app.run();

    Ok(())
}
