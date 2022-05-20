local current_scripts

function on_update()

    if current_scripts == nil then
        local comp = world:get_component(entity,"ScriptCollection<LuaFile>")

        print(string.format("hello from %s", comp))


        new_entity = world:new_script_entity("scripts/basic_example.lua")
        -- local comp_new = world:get_component(new_entity,"ScriptCollection<LuaFile>")

        -- comp_new.scripts[1].name = string.format("%d.lua",comp_new.scripts[1].id)

        -- print(string.format("%s",comp_new.scripts[1].name))
    end
end