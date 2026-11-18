local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

local trigger = ui.signal(0)
local menu_runs, menu_cleanups = 0, 0

local Menu = ui.component("Menu", function()
    local state = ui.signal("menu state")
    ui.effect(function()
        trigger:get()
        menu_runs = menu_runs + 1
        return function() menu_cleanups = menu_cleanups + 1 end
    end)
    return ui.text({ name = "screen", text = function() return state:get() end })
end)

local Game = ui.component("Game", function()
    local score = ui.signal(0)
    return ui.text({ name = "screen", text = function() return "score " .. score:get() end })
end)

local root = ui.render("screens", Menu())
t.screen(root)
ui.render("screens", Game())
local s = t.screen(root)
local text = s:text(s:get_by_name("screen"))
assert(text == "score 0", "the game got the menu's state: " .. text)
assert(menu_cleanups == 1, "the menu's effect was not cleaned up")

menu_runs = 0
s:act(function() trigger:set(1) end)
assert(menu_runs == 0, "the menu's effect still runs")
