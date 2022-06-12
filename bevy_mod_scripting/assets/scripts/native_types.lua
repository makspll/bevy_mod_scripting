
local comp

function on_update()
    if comp == nil then
        comp = world:get_component(entity,"MyComponent")

        print(string.format("%s",comp))

        print(string.format("%s",comp.vec2));
        comp.vec2 = comp.vec2 - comp.vec2
        -- comp.uvec2 = comp.uvec2 + comp.uvec2
        -- comp.usize = comp.vec2:min_element()
        -- comp.f32 = comp.f32 + comp.f32 + comp.vec2:min_element()
        -- comp.vec2 = vec2(2,1)
        -- comp.dquat = dquat(3,2,1,4) -- comp.quat * 2
        -- a = mat3(vec3(1,2,3),vec3(4,5,6),vec3(7,8,9))

        -- comp.mat3 = comp.mat3 -- this is safe!
        -- print("hello")
        -- comp.vec4 = vec4(3,2,1,4) -- currently crashing due to rlua bug
        -- comp.quat = quat(3,2,1,4) -- currently crashing due to rlua bug

        print(string.format("%s", comp))
    end

end