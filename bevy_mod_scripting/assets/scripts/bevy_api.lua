
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
        print(string.format("%s",comp.u8))

        -- local ok = pcall(function () comp.vec2 = comp.vec2 end) 
        -- assert(not ok)

        comp.u8 = 2

        if comp.option == nil then
            print(string.format("option was %s", comp.option))
            comp.option = Vec3.new(2,1,3)
            print(string.format("option[1] is now %s", comp.option[1]))
            comp.option[1] = 5
            print(string.format("and now option[1] is %s", comp.option[1]))
        end

        comp.vec_of_option_bools = {true,false,true}
        comp.vec_of_option_bools[1] = true

        comp.option_vec_of_bools = {true,false,true}
        comp.option_vec_of_bools[1] = true
        
        comp.vec2 = comp.vec2 + comp.vec2

        comp.uvec2 = comp.uvec2 + comp.uvec2
        comp.usize = comp.vec2:min_element()
        comp.f32 = comp.f32 + comp.f32 + comp.vec2:min_element()
        comp.vec2 = Vec2.new(2,1)
        comp.vec3 = Vec3.new(0,1,0):any_orthonormal_vector() + comp.mat3.x_axis + comp.option
        comp.vec4 = Vec4.splat(3)
        comp.quat = Quat.from_xyzw(3,2,1,4)
        comp.dquat = comp.dquat * 2
        comp.my_reflect_thing.hello = "bye world!"
        a = Mat3.from_cols(Vec3.new(1,0,0),Vec3.new(0,1,0),Vec3.new(0,0,-1))

        comp.mat3[1][1] = 42
        comp.mat3.x_axis = Vec3.new(69,69,69)

        comp = world:get_component(entity,my_component_type)
        res = world:get_resource(my_resource_type)

        print(string.format("%s", comp))
        print(string.format("%s", res))
        print(string.format("%s %s %s", comp.vec_of_option_bools[0],comp.vec_of_option_bools[1],comp.vec_of_option_bools[2]))
        print(string.format("%s %s %s", comp.option_vec_of_bools[0],comp.option_vec_of_bools[1],comp.option_vec_of_bools[2]))

    end
end