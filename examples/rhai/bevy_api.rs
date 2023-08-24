use bevy::app::AppExit;

use bevy::prelude::*;
use bevy_mod_scripting::{api::rhai::bevy::RhaiBevyAPIProvider, prelude::*};
use bevy_mod_scripting_rhai::rhai::Engine;
use bevy_script_api::rhai::{std::RegisterVecType, RegisterForeignRhaiType};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MyComponent {
    quat: Quat,
    vec2: Vec2,
    usize: usize,
    f32: f32,
    mat3: Mat3,
    option_vec3: Option<Vec3>,
    vec_of_option_bools: Vec<Option<bool>>,
    option_vec_of_bools: Option<Vec<bool>>,
}

pub struct MyAPIProvider;
// unlike mlua, rhai does not have the concept of generic types, all functionality is based around
// registering monomorphized functions, therefore we must register functions of generic types for every type we want
// to use them with, it's less convenient and is what the compiler would do anyway but hey it works!
impl APIProvider for MyAPIProvider {
    type APITarget = Engine;

    type ScriptContext = RhaiContext;

    type DocTarget = RhaiDocFragment;

    fn attach_api(&mut self, api: &mut Self::APITarget) -> Result<(), ScriptError> {
        api.set_max_expr_depths(999, 999);
        api.register_vec_functions::<Option<bool>>();
        api.register_vec_functions::<bool>();
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(ScriptingPlugin)
        .register_type::<MyComponent>()
        .register_foreign_rhai_type::<Option<bool>>()
        .register_foreign_rhai_type::<Vec<Option<bool>>>()
        .register_foreign_rhai_type::<Option<Vec<bool>>>()
        // note the implementation for Option is there, but we must register `LuaProxyable` for it
        // this system set handles addition and removal of script contexts, we can safely use `CoreSet::PostUpdate`
        .add_script_host::<RhaiScriptHost<()>>(PostUpdate)
        .add_api_provider::<RhaiScriptHost<()>>(Box::new(RhaiBevyAPIProvider))
        .add_api_provider::<RhaiScriptHost<()>>(Box::new(MyAPIProvider))
        .add_systems(Update, |world: &mut World| {
            let entity = world
                .spawn(())
                .insert(MyComponent {
                    usize: 5,
                    vec2: Vec2::new(1.0, 2.0),
                    f32: 6.7,
                    mat3: Mat3::from_cols(
                        Vec3::new(1.0, 2.0, 3.0),
                        Vec3::new(4.0, 5.0, 6.0),
                        Vec3::new(7.0, 8.0, 9.0),
                    ),
                    quat: Quat::from_xyzw(1.0, 2.0, 3.0, 4.0),
                    option_vec3: Some(Vec3::new(1.0, 2.0, 3.0)),
                    vec_of_option_bools: vec![Some(true), None, Some(false)],
                    option_vec_of_bools: Some(vec![true, true, true]),
                })
                .id();

            // run script
            world.resource_scope(|world, mut host: Mut<RhaiScriptHost<()>>| {
                host.run_one_shot(
                    r#"
                        fn once() {
                            print(world);
                            debug(world.get_children(entity));

                            // we first retrieve ID's for our component and resource by their short name (long name/full path also work)
                            let my_component_type = world.get_type_by_name("MyComponent");

                            // then ask the world to give us a reference to `MyComponent` on the entity we just spawned
                            // resources work the same way, but we use `get_resource` instead of `get_component`
                            // the comp object is resolved to a `bevy_script_api::script_ref::ReflectValue`.
                            // we can use a custom proxy instead (by implementing RhaiProxyable), but this is the simplest way to get started.
                            let comp = world.get_component(entity,my_component_type);

                            print("Before script: " + comp);

                            print("=============");

                            comp.usize = 2;
                            print("comp.usize: after assigning to 2: " + comp.usize);

                            // not supported yet (no Bevy proxies yet)
                            // print("comp.option_vec3 before: " + comp.option_vec3);
                            // comp.option_vec3 = Vec3::new(2,1,3);
                            // print("comp.option_vec3 after: " + comp.option_vec3);
                            
                            // print("comp.option_vec3[0] before: " + comp.option_vec3[0]);
                            // comp.option_vec3[1] = 5;
                            // print("comp.option_vec3[0] after: " + comp.option_vec3[0]);

                            print("=============");

                            print("comp.vec_of_option_bools before: " + comp.vec_of_option_bools);
                            comp.vec_of_option_bools = [true,false,true];
                            print("comp.vec_of_option_bools after: " + comp.vec_of_option_bools);

                            print("comp.vec_of_option_bools[0] before: " + comp.vec_of_option_bools[0]);
                            comp.vec_of_option_bools[0] = false;
                            print("comp.vec_of_option_bools[0] after: " + comp.vec_of_option_bools[0]);

                            print("comp.vec_of_option_bools before insert: " + comp.vec_of_option_bools);
                            comp.vec_of_option_bools.insert(1,());
                            print("comp.vec_of_option_bools after insert: " + comp.vec_of_option_bools);

                            print("comp.vec_of_option_bools before push: " + comp.vec_of_option_bools);
                            comp.vec_of_option_bools.push(false);
                            print("comp.vec_of_option_bools after push: " + comp.vec_of_option_bools);

                            print("comp.vec_of_option_bools len after push: " + comp.vec_of_option_bools.len());

                            print("comp.vec_of_option_bools before pop: " + comp.vec_of_option_bools);
                            print(comp.vec_of_option_bools.pop());
                            print("comp.vec_of_option_bools after pop: " + comp.vec_of_option_bools);

                            print("the elements inside comp.vec_of_option_bools: ");
                            for e in comp.vec_of_option_bools {
                                print(`elem: ${e}`);
                            }

                            comp.vec_of_option_bools.clear();
                            print("comp.vec_of_option_bools after clear: " + comp.vec_of_option_bools);
                            print("comp.vec_of_option_bools len after clear: " + comp.vec_of_option_bools.len());

                            print("=============");

                            print("comp.option_vec_of_bools before: " + comp.option_vec_of_bools);
                            print(comp.option_vec_of_bools.pop());
                            print("comp.option_vec_of_bools after pop: " + comp.option_vec_of_bools);
                            
                            print("comp.option_vec_of_bools len after pop: " + comp.option_vec_of_bools.len());

                            print("the elements inside comp.option_vec_of_bools: ");
                            for e in comp.option_vec_of_bools {
                                print(`elem: ${e}`);
                            }

                            print("=============");


                            let comp_after = world.get_component(entity, my_component_type);
                            print("after script:");
                            print(comp_after);

                        }
                        "#
                    .as_bytes(),
                    "script.rhai",
                    entity,
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
