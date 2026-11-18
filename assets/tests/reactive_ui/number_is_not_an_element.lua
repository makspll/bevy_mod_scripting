-- element handles are numbers to scripts, but no small number is one
local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

for i = 0, 5 do ui.text("decoy " .. i) end

local ok, err = pcall(function() t.render(ui.show(function() return 3 end)) end)
assert(not ok, "returning 3 was taken as an element")
assert(string.find(tostring(err), "got the number 3", 1, true), "unclear error: " .. tostring(err))
