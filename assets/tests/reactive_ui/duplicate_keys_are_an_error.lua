local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

local ok, err = pcall(function()
    t.render(ui.list(ui.signal({ "potion", "potion" }), function(item) return ui.text(item) end))
end)
assert(not ok and string.find(tostring(err), "the key `potion` twice", 1, true), "duplicate list items: " .. tostring(err))

ok, err = pcall(function()
    t.render(ui.node({}, { ui.keyed("a", ui.text("x")), ui.keyed("a", ui.text("y")) }))
end)
assert(not ok and string.find(tostring(err), "the key `a` twice", 1, true), "duplicate keyed siblings: " .. tostring(err))
