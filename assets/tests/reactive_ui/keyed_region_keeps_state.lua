local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

local Counter = ui.component("Counter", function()
    local count = ui.signal(0)
    return ui.node({ name = "inc", on_click = function() count:set(count:peek() + 1) end }, {
        ui.text({ name = "count", text = function() return "count " .. count:get() end }),
    })
end)

local tick = ui.signal(0)
local s = t.render(ui.show(function()
    tick:get()
    return { ui.keyed("k", ui.show(function() return Counter() end)) }
end))
s:click(s:get_by_name("inc"))
s:act(function() tick:set(1) end)
local text = s:text(s:get_by_name("count"))
assert(text == "count 1", "the keyed region was mounted again: " .. text)
