
local comp

function on_update()
    print(string.format("%s",comp))
    if comp == nil then
        comp = world:get_component(entity,"MyComponent")


        comp.vec = comp.vec + comp.vec;
        print(string.format("%s",comp.vec))

    end
end