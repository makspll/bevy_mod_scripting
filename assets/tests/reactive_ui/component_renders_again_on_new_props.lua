local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

---@param p { user: string, on_pick: fun()? }
local Profile = ui.component("Profile", function(p)
    return ui.text({ name = "profile", text = "user " .. p.user })
end)

-- a region running again with other props renders the component again
local user = ui.signal("ada")
local s = t.render(ui.show(function() return Profile({ user = user:get() }) end))
s:act(function() user:set("bob") end)
local text = s:text(s:get_by_name("profile"))
assert(text == "user bob", "new props not shown: " .. text)

-- a closure made again is not a change
local tick = ui.signal(0)
local b = t.render(ui.show(function()
    tick:get()
    return Profile({ user = "cy", on_pick = function() end })
end))
b:updates()
b:act(function() tick:set(1) end)
assert(b:updates().renders.Profile == nil, "a new closure rendered the component again")
