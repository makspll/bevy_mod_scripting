local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

local shown = ui.signal(true)
local tick = ui.signal(0)
local runs, cleanups = 0, 0

local Widget = ui.component("Widget", function()
    ui.effect(function() return function() error("cleanup failed") end end)
    ui.effect(function()
        tick:get()
        runs = runs + 1
        return function() cleanups = cleanups + 1 end
    end)
    return ui.text("widget")
end)

local s = t.render(ui.show(function() if shown:get() then return Widget() end end))
local ok, err = pcall(function() s:act(function() shown:set(false) end) end)
assert(not ok and string.find(tostring(err), "cleanup failed", 1, true), "the cleanup error was not reported: " .. tostring(err))
assert(cleanups == 1, "the second effect was not cleaned up")

runs = 0
s:act(function() tick:set(1) end)
assert(runs == 0, "an effect of the removed component still runs")
