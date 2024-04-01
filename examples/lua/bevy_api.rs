use bevy::app::AppExit;

use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;

use bevy_script_api::{lua::RegisterForeignLuaType, prelude::*};

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

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(ScriptingPlugin)
        .register_type::<MyComponent>()
        // note the implementation for Option is there, but we must register `LuaProxyable` for it
        .register_foreign_lua_type::<Option<Vec3>>()
        .register_foreign_lua_type::<Vec<Option<bool>>>()
        .register_foreign_lua_type::<Option<bool>>()
        .register_foreign_lua_type::<Option<Vec<bool>>>()
        .add_script_host::<LuaScriptHost<()>>(PostUpdate)
        .add_api_provider::<LuaScriptHost<()>>(Box::new(BevyAPIProvider))
        .add_api_provider::<LuaScriptHost<()>>(Box::new(CoreBevyAPIProvider))
        .add_systems(Startup,
            |world: &mut World| {

                let entity = world.spawn(())
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
                        option_vec3: None,
                        vec_of_option_bools: vec![Some(true), None, Some(false)],
                        option_vec_of_bools: Some(vec![true, true, true]),
                    }).id();

                // run script
                world.resource_scope(|world, mut host: Mut<LuaScriptHost<()>>| {
                    host.run_one_shot(
                        r#"
                        function table_to_string(t)
                            local result = "["
                            for k,v in pairs(t) do
                                result = result .. string.format("%s:%s,",k,v)
                            end
                            return result .. "]"
                        end

                        function once()
                            
                            -- the api provides us with 3 globals
                            print(entity)
                            print(script)
                            print(world)

                            -- we first retrieve ID's for our component and resource by their short name (long name/full path also work)
                            local my_component_type = world:get_type_by_name("MyComponent")

                            -- then ask the world to give us a reference to `MyComponent` on the entity we just spawned
                            -- resources work the same way, but we use `get_resource` instead of `get_component`
                            -- the comp object is resolved to a `bevy_script_api::script_ref::ReflectValue` which implements UserData.
                            -- we can use a custom proxy instead (by implementing LuaProxyable), but this is the simplest way to get started.
                            local comp = world:get_component(entity, my_component_type)
                            print("Before script: ", comp)

                            print("============")

                            -- the index metamethod on ReflectValue's uses bevy's reflection mechanism on top of some custom sub-reflection logic to
                            -- allow reflecting inside Options, Vectors etc. 
                            -- when we index into ReflectValue's we either get back a custom proxy or another ReflectValue

                            -- the LuaBevyAPIProvider provides us custom proxies for many bevy types as well as std types.
                            -- all of these implementations can be overridden via the bevy TypeRegistry
                            comp.usize = 2
                            print("comp.usize after assigning to 2: ", comp.usize)

                            -- vec's and matrices have custom __index and __newindex overrides
                            print("comp.vec2 before: ", comp.vec2)
                            comp.vec2[1] = 69
                            print("comp.vec2 after: ", comp.vec2)

                            -- Option's get converted to nil or the value inside
                            print("comp.option_vec3 before: ", comp.option_vec3)
                            comp.option_vec3 = Vec3.new(2,1,3)
                            print("comp.option_vec3 after: ", comp.option_vec3)

                            -- reflection via index is indexed starting at 1, unlike in Rust to match Lua's indexing
                            print("comp.option_vec3[1] before: ", comp.option_vec3[1])
                            comp.option_vec3[1] = 5
                            print("comp.option_vec3[1] after: ", comp.option_vec3[1])

                            print("============")

                            -- Vec<T> references get converted to a custom proxy `LuaVec<T>` which is 
                            -- also assignable via lua tables 

                            print("comp.vec_of_option_bools before: ", table_to_string(comp.vec_of_option_bools))
                            comp.vec_of_option_bools = {true,false,true}
                            print("comp.vec_of_option_bools after assignment: ", table_to_string(comp.vec_of_option_bools))

                            print("comp.vec_of_option_bools[1] before: ", comp.vec_of_option_bools[1])
                            comp.vec_of_option_bools[1] = false
                            print("comp.vec_of_option_bools[1] after: ", comp.vec_of_option_bools[1])

                            -- there are some additional methods available on LuaVec proxies imitating the Vec<T> api
                            print("comp.vec_of_option_bools before insert: ", table_to_string(comp.vec_of_option_bools))
                            comp.vec_of_option_bools:insert(1,nil)
                            print("comp.vec_of_option_bools after insert: ", table_to_string(comp.vec_of_option_bools))

                            print("comp.vec_of_option_bools before push: ", table_to_string(comp.vec_of_option_bools))
                            comp.vec_of_option_bools:push(false)
                            print("comp.vec_of_option_bools after push: ", table_to_string(comp.vec_of_option_bools))

                            print("comp.vec_of_option_bools len after push: ", #comp.vec_of_option_bools)

                            print("comp.vec_of_option_bools before pop: ", table_to_string(comp.vec_of_option_bools))
                            print(comp.vec_of_option_bools:pop())
                            print("comp.vec_of_option_bools after pop: ", table_to_string(comp.vec_of_option_bools))
                            
                            print("the pairs inside comp.vec_of_option_bools: ")             
                            for k,v in pairs(comp.vec_of_option_bools) do
                                print(string.format(" - %s:%s",k,v))
                            end
                            
                            comp.vec_of_option_bools:clear()
                            print("comp.vec_of_option_bools after clear: ", table_to_string(comp.vec_of_option_bools))

                            print("comp.vec_of_option_bools len after clear: ", #comp.vec_of_option_bools)
                            print("============")

                            print("comp.option_vec_of_bools before: ", table_to_string(comp.option_vec_of_bools))
                            print(comp.option_vec_of_bools:pop())
                            print("comp.option_vec_of_bools after pop: ", table_to_string(comp.option_vec_of_bools))
  

                            print("comp.option_vec_of_bools len after pop: ", #comp.option_vec_of_bools)

                            print("the pairs inside comp.option_vec_of_bools: ")
                            for k,v in pairs(comp.option_vec_of_bools) do
                                print(string.format(" - %s:%s",k,v))
                            end

                            print("============")

                            local complex_vec_op = Vec3.new(0,1,0):any_orthonormal_vector() + comp.mat3.x_axis 
                            print("(0,1,0).any_orthonormal_vector() + mat3.x_axis is: ", complex_vec_op) 
                            
                            local new_mat3 = Mat3.from_cols(Vec3.new(1,0,0),Vec3.new(0,1,0),Vec3.new(0,0,-1))
                            print("new_mat3 is:", new_mat3)

                            comp.vec2 = comp.vec2 + comp.vec2
                            comp.usize = comp.vec2:min_element()
                            comp.f32 = comp.f32 + comp.f32 + comp.vec2:min_element()
                            comp.vec2 = Vec2.new(2,1)
                            comp.quat = Quat.from_xyzw(3,2,1,4)
                            comp.mat3.x_axis = Vec3.new(69,69,69)

                            print("============")

                            -- this is an example of something impossible to achieve with plain bevy reflection under the hood
                            comp.mat3[1][1] = 42

                            -- now let's retrieve these again to see if we actually changed their values permanently
                            comp = world:get_component(entity,my_component_type)
                            
                            print("After script:")
                            print(comp)
                        end
                        "#
                        .as_bytes(),
                        "script.lua",
                        entity,
                        world,
                        LuaEvent {
                            hook_name: "once".to_owned(),
                            args: (),
                            recipients: Recipients::All,
                        },
                    )
                    .expect("Something went wrong in the script!");
                });

                world.send_event(AppExit);
            },
        );

    app.run();

    Ok(())
}
