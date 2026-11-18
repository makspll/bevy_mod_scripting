-- a region running again doesn't build the rows of a list in it again, its items didn't change
local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

local other = ui.signal(0)
local items = ui.signal({ "a", "b", "c" })
local builds = 0

local s = t.render(ui.show(function()
    other:get()
    return ui.list(items, function(item)
        builds = builds + 1
        return ui.text(item)
    end)
end))
builds = 0
s:act(function() other:set(1) end)
assert(builds == 0, builds .. " unchanged rows were built again")
