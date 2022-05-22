
local comp

function on_update()
    if comp == nil then
        comp = world:get_component(entity,"MyComponent")

        print(string.format("%s",comp.bevy_vec))

        comp.vec = comp.vec + comp.vec;
        comp.bevy_vec = comp.bevy_vec + comp.bevy_vec;
        print(string.format("%s",comp.vec))
        print(string.format("%s",comp.bevy_vec))

    end
end