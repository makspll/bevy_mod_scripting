
local comp

function on_update()
    if comp == nil then
        comp = world:get_component(entity,"MyComponent")

        print(string.format("%s",comp))

        print(string.format("%s",comp.vec2:min_element()))
        comp.vec2 = vec2(6,10)
        comp.vec2.x = 69
        comp.uvec2 = comp.uvec2 + comp.uvec2
        comp.usize = comp.vec2:min_element()
        comp.f32 = comp.f32 + comp.f32 + comp.vec2:min_element()
        comp.mat3:col(0).x = -5

        print(string.format("%s",comp))

    end
end