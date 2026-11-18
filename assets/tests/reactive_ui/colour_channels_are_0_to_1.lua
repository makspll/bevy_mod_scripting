local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

local s = t.render(ui.node({ name = "half", background_color = { 1, 0.5, 0 } }))
s:expect_style(s:get_by_name("half"), { background_color = "#ff8000" })

local ok, err = pcall(function() t.render(ui.node({ background_color = { 255, 0, 0 } })) end)
assert(not ok and string.find(tostring(err), "colour channels are from 0 to 1", 1, true), tostring(err))
