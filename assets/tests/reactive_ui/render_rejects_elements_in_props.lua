-- a root renders in a later update, when the elements built now are gone
local ui = import_namespace("reactive_ui")

---@param p { header: integer }
local Layout = ui.component("Layout", function(p) return ui.node({}, { p.header }) end)

local ok, err = pcall(function() ui.render("layout", Layout({ header = ui.text("hi") })) end)
assert(not ok and string.find(tostring(err), "`render` can't be given elements", 1, true), tostring(err))
