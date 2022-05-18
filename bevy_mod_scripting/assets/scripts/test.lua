local a

function on_update()

    if current_scripts == nil then
        print(string.format("hello from %d", script))
        current_scripts = world:get_component(entity,"ScriptCollection<LuaFile>")

        new_scripts = world:new_component("ScriptCollection<LuaFile>")

        new_scripts.scripts = current_scripts.scripts
        new_scripts.scripts[1].id = current_scripts.scripts[1].id + 1
        print(current_scripts.scripts[1].id + 2)
        local components = {new_scripts}
        world:spawn(components)
    end




end