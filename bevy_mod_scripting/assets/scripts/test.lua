local a

function on_update()

    if a == nil then
        a = world:get_component(entity,"ScriptCollection<LuaFile>")
    end


    print(a.scripts)

end