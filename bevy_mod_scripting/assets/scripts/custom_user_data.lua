
local comp

function on_update()
    if comp == nil then
        comp = world:get_component(entity,"MyComponent")


        comp.vec = comp.vec + comp.vec;
        print(string.format("%s",comp.vec))

    end
end