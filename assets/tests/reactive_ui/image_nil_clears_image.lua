local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

---@type reactive_ui.Signal<string?>
local path = ui.signal("icons/sword.png")
local s = t.render(ui.image({ name = "icon", width = 16, image = function() return path:get() end }))
s:expect_style(s:get_by_name("icon"), { image = "icons/sword.png" })

s:act(function() path:set(nil) end)
assert(s:style(s:get_by_name("icon")).image == nil, "the image is still shown")
