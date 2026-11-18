local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

---@type reactive_ui.UiContext<string>
local Theme = ui.create_context("light")
local n = ui.signal(0)
---@type string?
local clicked = nil

local Title = ui.component("Title", function()
    return ui.node({ name = "button", on_click = function() clicked = ui.use_context(Theme) end }, {
        ui.text({ name = "title", text = function() return ui.use_context(Theme) .. n:get() end }),
    })
end)

local s = t.render(ui.provide(Theme, "dark", { Title() }))
s:act(function() n:set(1) end)
local text = s:text(s:get_by_name("title"))
assert(text == "dark1", "a binding running again saw the default: " .. text)

s:click(s:get_by_name("button"))
assert(clicked == "dark", "a handler saw the default: " .. tostring(clicked))
