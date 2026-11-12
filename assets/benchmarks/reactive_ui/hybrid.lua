-- reactive ui benchmarks, hybrid model. `BENCH` is prepended by the harness and selects the scenario,
-- `bench_step` makes the change whose ui update is measured
local bms = import_namespace("bms")
local world = bms.world
local DynamicComponent = bms.DynamicComponent
local ui = import_namespace("reactive_ui")

local Health = world.register_new_component("script.bench", "Health", true)
local Item = world.register_new_component("script.bench", "Item", true)

local player = world.spawn()
world.insert_component(player, Health, DynamicComponent.new({ data = { current = 100, max = 100 } }))
local items, entities = {}, {}
for i = 1, 200 do
    local name = "item " .. i
    local e = world.spawn()
    world.insert_component(e, Item, DynamicComponent.new({ data = { name = name, count = 1 } }))
    items[i] = name
    entities[name] = e
end
local search = ui.signal("")
local shown = ui.signal(true)

local HealthBar = ui.define_hybrid("HealthBar", function(p)
    local h = ui.watch(p.player, Health):get().data
    local low = h.current < h.max * 0.25
    local children = {
        ui.node({ key = "fill", width = (h.current / h.max * 100) .. "%", bg = low and "#ff0000" or "#00ff00" }),
        ui.text({ key = "label", text = h.current .. " / " .. h.max }),
    }
    if low then children[#children + 1] = ui.text({ key = "low", text = "LOW HP" }) end
    return ui.node({ direction = "column" }, children)
end)

local Slot = ui.define_hybrid("Slot", function(p)
    local item = ui.watch(p.entity, Item):get()
    return ui.node({
        name = "slot_" .. p.name,
        bg = p.selected and "#ffcc00" or "#333333",
        on_click = function() p.on_select(p.name) end,
    }, {
        ui.text({ text = "x" .. item.data.count }),
    })
end)

local Details = ui.define_hybrid("Details", function(p)
    return ui.text({ text = "details: " .. p.name })
end)

local Inventory = ui.define_hybrid("Inventory", function(p)
    local selected = ui.signal(nil)
    local query = search:get()
    local visible = {}
    for _, name in ipairs(items) do
        if string.find(name, query, 1, true) then visible[#visible + 1] = name end
    end
    local function select(name) selected:set(name) end
    local sel = selected:get()
    local slots = {}
    for _, name in ipairs(visible) do
        slots[#slots + 1] = ui.el(Slot, {
            key = name, name = name, entity = entities[name], selected = sel == name, on_select = select,
        })
    end
    local children = {
        ui.text({ key = "title", text = "Inventory (" .. #visible .. ")" }),
        ui.node({ key = "slots" }, slots),
    }
    if sel ~= nil then children[#children + 1] = ui.el(Details, { key = "details_" .. sel, name = sel }) end
    return ui.node({ direction = "column" }, children)
end)

local Root = ui.define_hybrid("Root", function()
    if shown:get() then
        return ui.node({}, { ui.el(Inventory, { key = "inventory" }) })
    end
    return ui.node({}, {})
end)

local root
function on_script_loaded()
    if BENCH == "health" then
        root = ui.mount("bench", HealthBar, { player = player })
    else
        root = ui.mount("bench", Root, {})
    end
end

local n = 0
function bench_step()
    n = n + 1
    if BENCH == "health" then
        world.get_component(player, Health).data = { current = 50 + n % 2, max = 100 }
    elseif BENCH == "mount" then
        shown:set(not shown:peek())
    elseif BENCH == "select" then
        ui.interact(ui.find(root, "slot_" .. items[n % 200 + 1]), "press")
    elseif BENCH == "search" then
        search:set(n % 2 == 0 and "" or "5")
    elseif BENCH == "count" then
        world.get_component(entities[items[n % 200 + 1]], Item).data = { name = "x", count = n }
    end
end
