
local comp

function on_update()
    if comp == nil then
        comp = world:get_component(entity,"MyComponent")

        print(string.format("%s",comp))

        comp.vec2 = comp.vec2 * vec2(6,10)
        -- comp.vec2.x = 69
        comp.uvec2 = comp.uvec2 + comp.uvec2
        comp.usize = comp.vec2:min_element()
        comp.f32 = comp.f32 + comp.f32 + comp.vec2:min_element()
        -- comp.mat3:col(0).x = -5

        comp.vec4 = vec4(3,2,1,4)
        comp.dquat = dquat(3,2,1,4) -- comp.quat * 2
        comp.quat = quat(3,2,1,4) -- comp.quat * 2

        print(string.format("%s", comp))

    end
end