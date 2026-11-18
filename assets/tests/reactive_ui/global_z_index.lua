-- `global_z_index` raises an element above the rest of the ui, `z_index` only orders siblings
local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

local raised = ui.signal(true)
local s = t.render(ui.node({}, {
    ui.node({ name = "modal", global_z_index = function() return raised:get() and 100 or nil end }),
}))
s:expect_style(s:get_by_name("modal"), { global_z_index = 100 })

s:act(function() raised:set(false) end)
assert(s:style(s:get_by_name("modal")).global_z_index == nil, "global_z_index was not removed")
