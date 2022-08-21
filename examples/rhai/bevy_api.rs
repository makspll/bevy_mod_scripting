use bevy::app::AppExit;
use bevy::math::DQuat;
use bevy::prelude::*;
use bevy_mod_scripting::{api::rhai::bevy::RhaiBevyAPIProvider, prelude::*};
use bevy_mod_scripting_rhai::rhai::Engine;
use bevy_script_api::rhai::{std::RegisterVecType, RegisterForeignRhaiType};

/// Let's define a resource, we want it to be "assignable" via lua so we derive `ReflectLuaProxyable`
/// This allows us to reach this value when it's a field under any other Reflectable type

#[derive(Default, Clone, Reflect)]
#[reflect(Resource)]
pub struct MyResource {
    pub thing: f64,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MyComponent {
    dquat: DQuat,
    quat: Quat,
    vec2: Vec2,
    vec3: Vec3,
    uvec2: UVec2,
    usize: usize,
    f32: f32,
    mat3: Mat3,
    vec4: Vec4,
    bool: bool,
    string: String,
    u8: u8,
    bool_option: Option<bool>,
    option: Option<Vec3>,
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
        .add_plugin(ScriptingPlugin)
        .register_type::<MyComponent>()
        .register_type::<MyResource>()
        .register_foreign_rhai_type::<Option<bool>>()
        .register_foreign_rhai_type::<Vec<Option<bool>>>()
        .register_foreign_rhai_type::<Option<Vec<bool>>>()
        // note the implementation for Option is there, but we must register `LuaProxyable` for it
        .init_resource::<MyResource>()
        // this stage handles addition and removal of script contexts, we can safely use `CoreStage::PostUpdate`
        .add_script_host::<RhaiScriptHost<()>, _>(CoreStage::PostUpdate)
        .add_api_provider::<RhaiScriptHost<()>>(Box::new(RhaiBevyAPIProvider))
        .add_api_provider::<RhaiScriptHost<()>>(Box::new(MyAPIProvider))
        .add_system(
            (|world: &mut World| {
                let entity = world
                    .spawn()
                    .insert(MyComponent {
                        vec2: Vec2::new(1.0, 2.0),
                        vec3: Vec3::new(1.0, 2.0, 3.0),
                        vec4: Vec4::new(1.0, 2.0, 3.0, 4.0),
                        uvec2: UVec2::new(1, 2),
                        usize: 5,
                        f32: 6.7,
                        mat3: Mat3::from_cols(
                            Vec3::new(1.0, 2.0, 3.0),
                            Vec3::new(4.0, 5.0, 6.0),
                            Vec3::new(7.0, 8.0, 9.0),
                        ),
                        quat: Quat::from_xyzw(1.0, 2.0, 3.0, 4.0),
                        dquat: DQuat::from_xyzw(1.0, 2.0, 3.0, 4.0),
                        bool: true,
                        string: "hello".to_owned(),
                        u8: 240,
                        bool_option: Some(true),
                        option: None,
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
                            print(world.get_children(entity));

                            let my_component_type = world.get_type_by_name("MyComponent");
                            let my_component = world.get_component(entity,my_component_type);

                            debug(my_component_type);
                            debug(my_component);
                            print(my_component.u8);
                            my_component.u8 = 257;
                            my_component.bool = false;
                            my_component.string = "bye";
                            my_component.bool_option = ();
                            print(my_component.u8);
                            print(my_component.bool);
                            print(my_component.string);
                            print(my_component.bool_option);
                            my_component.bool_option = true;
                            print(my_component.bool_option);
                            
                            for i in 0..my_component.vec_of_option_bools.len() {
                                print(`${i}: ${my_component.vec_of_option_bools[i]}`)
                            }

                            my_component.vec_of_option_bools = [true,false,true];
                            my_component.vec_of_option_bools[0] = false;
                            my_component.vec_of_option_bools.insert(1,());
                            my_component.vec_of_option_bools.push(false);

                            for i in 0..my_component.vec_of_option_bools.len() {
                                print(`${i}: ${my_component.vec_of_option_bools[i]}`)
                            }
                        }
                        "#
                        .as_bytes(),
                        "script.lua",
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
            })
            .exclusive_system(),
        );

    app.run();

    Ok(())
}
