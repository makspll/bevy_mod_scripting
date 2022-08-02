use bevy::app::AppExit;
use bevy::math::DQuat;
use bevy::prelude::*;
use bevy_mod_scripting::{
    langs::mlu::mlua::{self,UserData}, lua::bevy::LuaBevyAPIProvider, AddScriptHost, LuaEvent,
    LuaScriptHost, Recipients, ScriptingPlugin,
    AddScriptApiProvider, ReflectLuaProxyable, RegisterForeignLuaType, ValueLuaType, ScriptHost,
};



/// Let's define a resource, we want it to be "assignable" via lua so we derive `ReflectLuaProxyable`
/// This allows us to reach this value when it's a field under any other Reflectable type

#[derive(Default, Clone, Reflect)]
#[reflect(Resource, LuaProxyable)]
pub struct MyResource {
    pub thing: f64,
}

/// NOTE: this is a marker enabling an automatic implementation of LuaProxyable
/// By default, because this type implements Clone as well,
/// It will be passed BY VALUE
/// meaning that calling these methods will result in changes to the cloned value on lua side only
/// untill the resource is assigned back to the original component to make the changes on the original type.
/// To have "pass by reference" semantics use a  [`bevy_mod_scripting::api::lua::LuaWrapper`] and implement LuaProxyable yourself (see wrappers.rs example)
impl ValueLuaType for MyResource {}

impl UserData for MyResource {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("custom_resource_method", |_, s, v: f64| {
            s.thing = v;

            Ok("hello?")
        });

        methods.add_meta_method(mlua::MetaMethod::ToString, |_, s, ()| {
            Ok(format!(
                "I'm a resource with a custom metatable!: {}",
                s.thing
            ))
        });
    }
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
    u8: u8,
    option: Option<Vec3>,
    vec_of_option_bools: Vec<Option<bool>>,
    option_vec_of_bools: Option<Vec<bool>>,
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .register_type::<MyComponent>()
        .register_type::<MyResource>()
        // note the implementation for Option is there, but we must register `LuaProxyable` for it
        .register_foreign_lua_type::<Option<Vec3>>()
        .register_foreign_lua_type::<Vec<Option<bool>>>()
        .register_foreign_lua_type::<Option<bool>>()
        .register_foreign_lua_type::<Option<Vec<bool>>>()
        .init_resource::<MyResource>()
        // this stage handles addition and removal of script contexts, we can safely use `CoreStage::PostUpdate`
        .add_script_host::<LuaScriptHost<()>, _>(CoreStage::PostUpdate)
        .add_api_provider::<LuaScriptHost<()>>(Box::new(LuaBevyAPIProvider))
        .add_system(
            (|world: &mut World| {

                world.spawn()
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
                        u8: 240,
                        option: None,
                        vec_of_option_bools: vec![Some(true), None, Some(false)],
                        option_vec_of_bools: Some(vec![true, true, true]),
                    });

                // run script
                world.resource_scope(|world, mut host: Mut<LuaScriptHost<()>>| {
                    host.run_one_shot(
                        r#"
                        function once()
                            -- we first retrieve ID's for our component and resource by their short name (long name/full path also work)
                            
                            local my_component_type = world:get_type_by_name("MyComponent")
                            local my_resource_type = world:get_type_by_name("MyResource")

                            -- then ask the world to give us a reference to `MyComponent` on the entity we just spawned
                            -- normally we could use `entity` but we are running in a one shot environment

                            local comp = world:get_component(Entity.from_raw(0), my_component_type)
                            -- and our resource

                            local res = world:get_resource(my_resource_type)
                            -- we can now arbitrarily affect these 
                            
                            -- we can even call our custom methods!
                            
                            print(string.format(res:custom_resource_method(42)))
                            
                            -- we can set any of the fields
                            -- if the field is `LuaProxyable` like our resource
                            -- that trait's implementation controls this assignment
                            comp.u8 = 2
                            
                            -- Option's get converted to nil | Value

                            if comp.option == nil then
                                print(string.format("option was %s", comp.option))
                                comp.option = Vec3.new(2,1,3)
                                print(string.format("option[1] is now %s", comp.option[1]))
                                comp.option[1] = 5
                                print(string.format("and now option[1] is %s", comp.option[1]))
                            end
                            
                            -- Vec<T> references get converted to a special proxy `LuaVec<T>` which is 
                            -- assignable via the Lua Tables

                            comp.vec_of_option_bools = {true,false,true}
                            
                            -- everything on the bevy side which uses Lua constructs as "Owned" variants
                            -- indexes from 1, other types map one to one with the bevy implementations and index from zero

                            comp.vec_of_option_bools[1] = false
                            -- there are some additional methods available on LuaVec

                            comp.vec_of_option_bools:insert(1,nil)
                            comp.vec_of_option_bools:push(false)
                            
                            -- Note, that Option's are reflected as Value types in Bevy, we are using
                            -- a custom magical SubReflection system allowing us to do this!

                            comp.option_vec_of_bools = {false,true,false}
                            comp.option_vec_of_bools[1] = true
                            comp.option_vec_of_bools:insert(1,false)
                            comp.option_vec_of_bools:push(true)
                    
                            print(#comp.vec_of_option_bools)
                            print(comp.vec_of_option_bools:pop())
                            print(comp.option_vec_of_bools:pop())

                            for k,v in pairs(comp.vec_of_option_bools) do
                                print(string.format("%s:%s",k,v))
                            end

                            print(#comp.option_vec_of_bools)

                            for k,v in pairs(comp.option_vec_of_bools) do
                                print(string.format("%s:%s",k,v))
                            end
                    
                            comp.vec_of_option_bools:clear()

                            print(#comp.vec_of_option_bools)

                            -- Every Bevy type implementing Reflect is available as a LuaProxyable wrapper
                            -- available types are under `api::lua::bevy::*` and std types in `api::lua::std::*`
                            -- not all functions are exposed but support for more complex functions will be rolled out
                            
                            comp.vec2 = comp.vec2 + comp.vec2
                            comp.uvec2 = comp.uvec2 + comp.uvec2
                            comp.usize = comp.vec2:min_element()
                            comp.f32 = comp.f32 + comp.f32 + comp.vec2:min_element()
                            comp.vec2 = Vec2.new(2,1)
                            comp.vec3 = Vec3.new(0,1,0):any_orthonormal_vector() + comp.mat3.x_axis + comp.option
                            comp.vec4 = Vec4.splat(3)
                            comp.quat = Quat.from_xyzw(3,2,1,4)
                            comp.dquat = comp.dquat * 2
                            local a = Mat3.from_cols(Vec3.new(1,0,0),Vec3.new(0,1,0),Vec3.new(0,0,-1))
                            
                            -- again an instance of sub reflection here, very cool!
                            
                            comp.mat3[0][0] = 42
                            comp.mat3.x_axis = Vec3.new(69,69,69)
                            
                            -- now let's retrieve these again to see if we actually changed their values
                            
                            comp = world:get_component(Entity.from_raw(0),my_component_type)
                            res = world:get_resource(my_resource_type)
                            
                            -- notet that our custom resource's value has not affected the original
                            -- this is because it is a by-value proxy, see wrappers.rs for an alternative
                            print("After script:")
                            print(string.format("%s", comp))
                            print(string.format("%s", res))
                        end
                        "#
                        .as_bytes(),
                        "script.lua",
                        world,
                        LuaEvent {
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
