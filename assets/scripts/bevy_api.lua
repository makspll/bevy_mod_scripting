function table_to_string(t)
    local result = "["
    for k,v in pairs(t) do
        result = result .. string.format("%s:%s,",k,v)
    end
    return result .. "]"
end

    
function on_event()

    print(entity) 
    print(script)
    print(world)


    local my_component_type = world:get_type_by_name("MyComponent")

    local comp = world:get_component(entity, my_component_type)
    print("Before script: ", comp)


    print(comp.option_usize)
    comp.option_usize = 69
    print(comp.option_usize)
    comp.option_usize = nil
    print(comp.option_usize)

    print("vec")
    print(comp.vec_of_usize)
    print(comp.vec_of_usize[2])
    comp.vec_of_usize[2] = 69
    print(comp.vec_of_usize[2])
    world:exit()

    print("============")

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

    world:exit()
end