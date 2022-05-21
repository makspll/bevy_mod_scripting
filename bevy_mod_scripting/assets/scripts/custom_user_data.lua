
local current_scripts

function on_update()

    if current_scripts == nil then
        local comp = world:get_component(entity,"MyComponent")

        print(string.format("%s",comp.vec))

        comp.vec = comp.vec + comp.vec;
    end
end