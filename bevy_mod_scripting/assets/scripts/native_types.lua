
local comp

function on_update()
    if comp == nil then
        comp = world:get_component(entity,"MyComponent")

        print(string.format("%s",comp))


        comp.vec2 = -comp.vec2 + comp.vec2 + comp.vec2:min(comp.vec2);
        comp.uvec2 = comp.uvec2 + comp.uvec2;
        comp.usize = comp.vec2:min_element();
        comp.f32 = comp.f32 + comp.f32 + comp.vec2:min_element()
        comp.vec2.x = 5.4

        print(string.format("%s",comp))

    end
end