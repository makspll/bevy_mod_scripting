-- reactive ui benchmarks, the closures.lua ui with properties computed in Rust wherever the helpers can express them.
-- `BENCH` is prepended by the harness and selects the scenario, `bench_step` makes the change whose ui update is measured
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
local removed = ui.signal(nil)

local HealthBar = ui.component("HealthBar", function(p)
    local hp = ui.watch(p.player, Health)
    local current, max = ui.path(hp, "data.current"), ui.path(hp, "data.max")
    -- the low hp warning is structure, it stays a script region
    local low = ui.computed(function()
        local h = hp:get().data
        return h.current < h.max * 0.25
    end)
    return ui.node({ flex_direction = "column" }, {
        ui.node({
            width = ui.percent(current, max),
            background_color = ui.when(ui.lt(current, ui.mul(max, 0.25)), "#ff0000", "#00ff00"),
        }),
        ui.text({ text = ui.format("{} / {}", { current, max }) }),
        ui.show(function()
            if low:get() then return ui.text({ key = "low", text = "LOW HP" }) end
        end),
    })
end)

local function slot(p)
    local item = ui.watch(p.entity, Item)
    return ui.node({
        name = "slot_" .. p.name,
        background_color = ui.when(ui.selected(p.selected, p.name), "#ffcc00", "#333333"),
        on_click = function() p.on_select(p.name) end,
    }, {
        ui.text({ text = ui.format("x{}", { ui.path(item, "data.count") }) }),
    })
end
local Slot = ui.component("Slot", slot)

local Details = ui.component("Details", function(p)
    return ui.text({ text = "details: " .. p.name })
end)

local Inventory = ui.component("Inventory", function(p)
    local selected = ui.signal(nil)
    local is_selected = ui.selector(selected)
    local visible = ui.computed(function()
        local query, without = search:get(), removed:get()
        local out = {}
        for _, name in ipairs(items) do
            if name ~= without and string.find(name, query, 1, true) then out[#out + 1] = name end
        end
        return out
    end)
    local function select(name) selected:set(name) end
    return ui.node({ flex_direction = "column" }, {
        ui.text({ text = function() return "Inventory (" .. #visible:get() .. ")" end }),
        ui.list(visible, function(name)
            -- the props stay in the script
            return ui.element(Slot, function()
                return slot({ name = name, entity = entities[name], selected = is_selected, on_select = select })
            end)
        end),
        ui.show(function()
            local s = selected:get()
            if s ~= nil then return ui.element(Details, { key = s, name = s }) end
        end),
    })
end)

local Root = ui.component("Root", function()
    return ui.show(function()
        if shown:get() then return ui.element(Inventory, { key = "inventory" }) end
    end)
end)

local root
function on_script_loaded()
    if BENCH == "health" then
        root = ui.render("bench", HealthBar, { player = player })
    else
        root = ui.render("bench", Root, {})
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
    elseif BENCH == "one" then
        removed:set(n % 2 == 1 and "item 100" or nil)
    elseif BENCH == "count" then
        world.get_component(entities[items[n % 200 + 1]], Item).data = { name = "x", count = n }
    end
end
