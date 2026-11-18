-- `cond and X()` and `cond and X() or nil` leave nothing in a list of children
local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

local cond = false

local s = t.render(ui.node({ name = "falsy" }, { cond and ui.text("hidden"), ui.text("shown") }))
assert(#s:children(s:get_by_name("falsy")) == 1, "`false` was not skipped")

s = t.render(ui.node({ name = "holes" }, { cond and ui.text("hidden") or nil, ui.text("shown") }))
assert(#s:children(s:get_by_name("holes")) == 1, "a hole was not skipped")
