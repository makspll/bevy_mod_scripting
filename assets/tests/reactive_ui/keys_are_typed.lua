-- `1` and "1" are different keys, entities are keys in lists and selectors alike
local world = import_namespace("bms").world
local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

local label = ui.signal("")
local mixed = ui.signal({ 1, "1" })
local s = t.render(ui.list(mixed, function(item)
    return ui.text({ name = "mixed", text = function() return tostring(item) .. label:get() end })
end))
s:act(function() mixed:set({ 1, "1", 2 }) end)
s:updates()
s:act(function() label:set("!") end)
assert(#s:all_by_name("mixed") == 3, "expected 3 rows")
assert(s:updates().bindings == 3, "a row was left behind")

assert(not ui.selector(ui.signal("1")):selected(1), "the number 1 is selected by the string \"1\"")
local entity = world.spawn()
assert(ui.selector(ui.signal(entity)):selected(entity), "an entity is not selected by itself")
