use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;
use bevy_mod_scripting::{LuaScriptHost, ScriptHost};
use bevy_mod_scripting::{api::lua::bevy::LuaWorld, ScriptRef};
use bevy_mod_scripting::{
    langs::mlu::mlua, AddScriptHost, LuaEvent, Recipients, ScriptingPlugin,
};
use bevy_mod_scripting_derive::impl_script_newtype;

// Step 1. Rust representation
// construct all our types and functionality
// Reflect is neccessary to allow access from scripts
// Clone makes deriving everything we need much easier, mainly ToLua and FromLua are missing for UserData/TealData with no Clone
// Even though we are implementing Clone we are still able to reference the original data in script thanks to the script wrapper we are about to implement
// Debug is nice to have, we can forward that implementation to Lua's ToString via our macro
#[derive(Reflect, Default, Clone, Debug)]
#[reflect(Resource)]
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
// The bevy API, however doing this means your provide static typing for your scripts in languages which support it,

impl_script_newtype!(
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

    impl {
        // we can also directly add methods to the underlying script traits using their specific syntax
        // note that in this case you need to provide an implementation for every language you want to support,
        // the flags above automatically do this for you.

        // below is a custom lua function
        // the fn here means this is a function and not a method (no self argument)
        // normally you'd make these available globally via mlua::create_proxy, but I digress.
        fn "make_ref_to_my_resource" => |ctx,()| {
            let globals = ctx.globals();
            let lua_world : LuaWorld = globals.get("world")?;
            let world = lua_world.upgrade().expect("World is gone! we're doomed!");
            let mut world = world.write();

            let reflect_resource_data = world.resource_scope(|world, type_registry: Mut<TypeRegistryArc>| {
                let type_registry = type_registry.read();
                let data = type_registry.get_type_data::<ReflectResource>(std::any::TypeId::of::<MyThing>()).expect("Type not registered properly");
                data.clone()
            });

            // this is absolutely safe!
            Ok(LuaMyThing::new_ref(ScriptRef::new_resource_ref(reflect_resource_data, lua_world.as_ref().clone())))
        };
    }

);

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_script_host::<LuaScriptHost<LuaMyThing>, _>(CoreStage::PostUpdate)
        .register_type::<MyThing>()
        .init_resource::<MyThing>()
        .add_system(
            (|world: &mut World| {
                world.insert_resource(MyThing {
                    usize: 420,
                    string: "I live in the bevy world, you can't touch me!".to_owned(),
                });

                // run script
                world.resource_scope(|world, mut host: Mut<LuaScriptHost<LuaMyThing>>| {
                    host.run_one_shot(
                        r#"
                    function once(my_thing)
                        local my_thing2 = my_thing.make_ref_to_my_resource()
                        print(my_thing2)
                        print(my_thing2.usize)
                        print(my_thing2.string)
                        print(my_thing2:do_something_cool())

                        my_thing2.usize = my_thing.usize
                        my_thing2.string = my_thing.string

                        print(my_thing2:do_something_cool())
                    end
                "#
                        .as_bytes(),
                        "script.lua",
                        world,
                        LuaEvent {
                            hook_name: "once".to_owned(),
                            args: vec![LuaMyThing::new(MyThing {
                                usize: 42,
                                string: "Haha! Yes I can!!!!".to_owned(),
                            })],
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
            })
            .exclusive_system(),
        );

    app.run();

    Ok(())
}
