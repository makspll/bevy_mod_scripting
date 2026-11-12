-- reactive ui benchmarks, signals model. `BENCH` is prepended by the harness and selects the scenario,
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

local HealthBar = ui.define_signals("HealthBar", function(p)
    local hp = ui.watch(p.player, Health)
    local low = ui.computed(function()
        local h = hp:get().data
        return h.current < h.max * 0.25
    end)
    return ui.node({ direction = "column" }, {
        ui.node({
            width = function()
                local h = hp:get().data
                return (h.current / h.max * 100) .. "%"
            end,
            bg = function() return low:get() and "#ff0000" or "#00ff00" end,
        }),
        ui.text({ text = function()
            local h = hp:get().data
            return h.current .. " / " .. h.max
        end }),
        ui.dynamic(function()
            if low:get() then return ui.text({ key = "low", text = "LOW HP" }) end
        end),
    })
end)

local Slot = ui.define_signals("Slot", function(p)
    local item = ui.watch(p.entity, Item)
    return ui.node({
        name = "slot_" .. p.name,
        bg = function() return p.selected:get() == p.name and "#ffcc00" or "#333333" end,
        on_click = function() p.on_select(p.name) end,
    }, {
        ui.text({ text = function() return "x" .. item:get().data.count end }),
    })
end)

local Details = ui.define_signals("Details", function(p)
    return ui.text({ text = "details: " .. p.name })
end)

local Inventory = ui.define_signals("Inventory", function(p)
    local selected = ui.signal(nil)
    local visible = ui.computed(function()
        local query = search:get()
        local out = {}
        for _, name in ipairs(items) do
            if string.find(name, query, 1, true) then out[#out + 1] = name end
        end
        return out
    end)
    local function select(name) selected:set(name) end
    return ui.node({ direction = "column" }, {
        ui.text({ text = function() return "Inventory (" .. #visible:get() .. ")" end }),
        ui.dynamic(function()
            local out = {}
            for _, name in ipairs(visible:get()) do
                out[#out + 1] = ui.el(Slot, {
                    key = name, name = name, entity = entities[name], selected = selected, on_select = select,
                })
            end
            return out
        end),
        ui.dynamic(function()
            local s = selected:get()
            if s ~= nil then return ui.el(Details, { key = s, name = s }) end
        end),
    })
end)

local Root = ui.define_signals("Root", function()
    return ui.dynamic(function()
        if shown:get() then return ui.el(Inventory, { key = "inventory" }) end
    end)
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
