use bevy::{app::AppExit, prelude::*};

use bevy_mod_scripting::{
    api::{impl_lua_newtype, impl_script_newtype},
    prelude::*,
};

// Step 1. Rust representation
// construct all our types and functionality
// Reflect is neccessary to allow access from scripts
// Clone allows receiving our wrapper as a function parameter (derives FromLua via UserData through mlua's traits)
// We can still use references to NonClone wrappers via AnyUserData in lua methods.
// Even though we are implementing Clone we are still able to reference the original data in script thanks to the script wrapper we are about to implement
// Debug is nice to have, we can forward that implementation to Lua's ToString via our macro
#[derive(Resource, Reflect, Default, Clone, Debug)]
#[reflect(Resource, LuaProxyable)]
pub struct MyThing {
    usize: usize,
    string: String,
}

impl MyThing {
    pub fn do_something_cool(&self) -> String {
        self.string.clone()
    }
}

// Step 2. Script representation
// this macro does some magic and provides you with a `LuaMyThing` (and possibly more for other enabled languages) type with which you can create:
// - owned values of your type via ::new()
// - references to something in the world (or script) via ::new_ref() and the ScriptRef API
//   (however this is only really accessible given the world provided to the script via the script host)
// Script references can also be made to subfields (even non reflectable ones) of types via sub reflection
//
// Note: this step is not fully necessary, if your value is reflectable, you'll be able to reach it via
// The bevy API, however doing this means your type will have `pass-by-reference` semantics and if the language supports it, static typing.
// To see what else this macro can do see `src/api/generated.rs`
impl_script_newtype!(
    #[languages(lua)]
    MyThing:
        Debug + Clone
        + Fields(
            /// My usize field
            usize: Raw(usize),
            /// My string field
            string: Raw(String)

        ) + Methods(
            /// Does something realy cool!
            /// this documentation gets forwarded to any utilities provided by the script host wooo
            do_something_cool(&self:)
        )

    lua impl {
        // we can also directly add methods to the underlying script traits using their specific syntax
        // note that in this case you need to provide an implementation for every language you want to support,
        // the flags above automatically do this for you.

        // below is a custom lua function
        // the fn here means this is a function and not a method (no self argument)
        // normally you'd make these available globally via mlua::create_proxy, but I digress.
        fn "my_raw_function" => |ctx,()| {
            Ok("Hello from my_raw_function!".to_owned())
        };
    }

);

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(ScriptingPlugin)
        .register_type::<MyThing>()
        .init_resource::<MyThing>()
        .add_script_host::<LuaScriptHost<LuaMyThing>>(PostUpdate)
        .add_api_provider::<LuaScriptHost<LuaMyThing>>(Box::new(LuaBevyAPIProvider))
        .add_systems(Update, |world: &mut World| {
            world.insert_resource(MyThing {
                usize: 420,
                string: "I live in the bevy world, you can't touch me!".to_owned(),
            });

            // run script
            world.resource_scope(|world, mut host: Mut<LuaScriptHost<LuaMyThing>>| {
                host.run_one_shot(
                    r#"
                    function once(new_my_thing)
                        local my_thing_type = world:get_type_by_name("MyThing");
                        print(my_thing_type)
                        local my_thing = world:get_resource(my_thing_type);

                        print("my_thing: ", my_thing)
                        print("my_thing.usize: ", my_thing.usize)
                        print("my_thing.string: ", my_thing.string)
                        print("my_thing:do_something_cool(): ", my_thing:do_something_cool())
                        print("my_thing.my_raw_function(): ", my_thing.my_raw_function())
                        print("changing my thing through the world reference")

                        my_thing.usize = new_my_thing.usize
                        my_thing.string = new_my_thing.string

                        print("my_thing.do_something_cool(): ", my_thing:do_something_cool())
                    end
                "#
                    .as_bytes(),
                    "script.lua",
                    Entity::from_raw(0),
                    world,
                    LuaEvent {
                        hook_name: "once".to_owned(),
                        args: LuaMyThing::new(MyThing {
                            usize: 42,
                            string: "Haha! Yes I can!!!!".to_owned(),
                        }),
                        recipients: Recipients::All,
                    },
                )
                .expect("Something went wrong in the script!");
            });

            // print current state of MyThing
            let my_thing = world
                .get_resource::<MyThing>()
                .expect("Could not find MyThing Resource");
            println!("After script run: {my_thing:#?}");
            // exit app
            world.send_event(AppExit)
        });

    app.run();

    Ok(())
}
