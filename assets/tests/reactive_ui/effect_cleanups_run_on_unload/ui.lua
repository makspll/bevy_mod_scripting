local bms = import_namespace("bms")
local world = bms.world
local ui = import_namespace("reactive_ui")

---@type bms.ScriptComponentRegistration<bms.DynamicComponent>
local CleanupRan = world.register_new_component("script.repro", "CleanupRan", true)

local Widget = ui.component("R06bWidget", function()
    ui.effect(function()
        return function()
            world.insert_component(world.spawn(), CleanupRan, bms.DynamicComponent.new({ data = true }))
        end
    end)
    return ui.text("widget")
end)

function on_script_loaded()
    ui.render("widget", Widget())
end
