
local comp

function on_update()
    if comp == nil then
        comp = world:get_component(entity,"MyComponent")

        print(string.format("%s",comp))
        comp.vec2 = comp.vec2 - comp.vec2
        comp.uvec2 = comp.uvec2 + comp.uvec2
        comp.usize = comp.vec2:min_element()
        comp.f32 = comp.f32 + comp.f32 + comp.vec2:min_element()
        comp.vec2 = vec2(2,1)
        comp.vec4 = vec4(3,2,1,4)
        comp.quat = quat(3,2,1,4)
        comp.dquat = comp.dquat * 2

        a = mat3(vec3(1,0,0),vec3(0,1,0),vec3(0,0,-1))

        print(string.format("%s",test(a[1],a[1]) ))

        comp.mat3[1][1] = 69 -- this is safe!

        print(string.format("%s", comp))
    end

end