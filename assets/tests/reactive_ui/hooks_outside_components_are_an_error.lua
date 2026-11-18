-- hooks in a function the ui runs again would be made again on every run, with nothing owning them
local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

local ok, err = pcall(function()
    t.render(ui.list(ui.signal({ "a" }), function(item)
        ui.effect(function() end)
        return ui.text(item)
    end))
end)
assert(not ok and string.find(tostring(err), "`effect` can only be called at the top of a component", 1, true),
    "effect in a list row: " .. tostring(err))

ok, err = pcall(function()
    t.render(ui.show(function()
        ui.signal(0)
        return ui.text("x")
    end))
end)
assert(not ok and string.find(tostring(err), "`signal` can only be called at the top of a component", 1, true),
    "signal in a show: " .. tostring(err))

-- outside of the ui a signal is plain script state
assert(ui.signal(1):peek() == 1)
