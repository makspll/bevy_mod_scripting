
local comp

function on_update()
    if comp == nil then

        my_component_type = world:get_type_by_name("MyComponent")
        my_resource_type = world:get_type_by_name("MyResource")
        comp = world:get_component(entity,my_component_type)
        res = world:get_resource(my_resource_type)

        print(string.format("%s",comp))
        print(string.format("%s",res))
        print(string.format("%s",res:custom_resource_method(42)))

        comp.vec2 = comp.vec2 + comp.vec2
        comp.uvec2 = comp.uvec2 + comp.uvec2
        comp.usize = comp.vec2:min_element()
        comp.f32 = comp.f32 + comp.f32 + comp.vec2:min_element()
        comp.vec2 = Vec2.new(2,1)
        comp.vec3 = Vec3.new(0,1,0):any_orthonormal_vector() + comp.mat3.x_axis
        comp.vec4 = Vec4.splat(3)
        comp.quat = Quat.from_xyzw(3,2,1,4)
        comp.dquat = comp.dquat * 2
        comp.my_reflect_thing.hello = "bye world!"
        a = Mat3.from_cols(Vec3.new(1,0,0),Vec3.new(0,1,0),Vec3.new(0,0,-1))

        -- comp.mat3[1][1] = 42
        comp.mat3.x_axis = Vec3.new(69,69,69)

        print(string.format("%s", comp))
        print(string.format("%s", res))
    end
end