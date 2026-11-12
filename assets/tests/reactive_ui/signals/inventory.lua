-- signals: components run once, properties are bound to functions, `dynamic` regions produce changing children
local bms = import_namespace("bms")
local world = bms.world
local DynamicComponent = bms.DynamicComponent
local ui = import_namespace("reactive_ui")

local Health = world.register_new_component("script.reactive_ui", "Health", true)
local Item = world.register_new_component("script.reactive_ui", "Item", true)

-- globals survive reloads, the context is reused
Reloads = (Reloads or 0) + 1
if State == nil then
    State = { step = 0, entities = {} }
    State.player = world.spawn()
    world.insert_component(State.player, Health, DynamicComponent.new({ data = { current = 100, max = 100 } }))
    for _, name in ipairs({ "Sword", "Shield", "Potion" }) do
        local e = world.spawn()
        world.insert_component(e, Item, DynamicComponent.new({ data = { name = name, count = 1 } }))
        State.entities[name] = e
    end
    State.items = ui.signal({ "Sword", "Shield", "Potion" })
    State.search = ui.signal("")
end

local function contains(list, value)
    for _, v in ipairs(list) do
        if v == value then return true end
    end
    return false
end

local Slot = ui.define_signals("Slot", function(p)
    local item = ui.watch(p.entity, Item)
    return ui.node({
        name = "slot_" .. p.name,
        bg = function() return p.selected:get() == p.name and "#ffcc00" or "#333333" end,
        on_click = function() p.on_select(p.name) end,
    }, {
        ui.text({ name = "count_" .. p.name, text = function() return "x" .. item:get().data.count end }),
    })
end)

local Details = ui.define_signals("Details", function(p)
    return ui.text({ name = "details", text = "details: " .. p.name })
end)

local Inventory = ui.define_signals("Inventory", function(p)
    local hp = ui.watch(p.player, Health)
    local selected = ui.signal(nil)
    local clicks = ui.signal(0)
    local visible = ui.computed(function()
        local query = State.search:get()
        local out = {}
        for _, name in ipairs(State.items:get()) do
            if string.find(string.lower(name), query, 1, true) then out[#out + 1] = name end
        end
        return out
    end)
    ui.effect(function()
        local s = selected:get()
        if s ~= nil and not contains(State.items:get(), s) then selected:set(nil) end
    end)
    local function select(name)
        selected:set(name)
        clicks:set(clicks:peek() + 1)
    end

    return ui.node({ direction = "column" }, {
        ui.text({ name = "version", text = "v" .. Reloads }),
        ui.text({ name = "title", text = function() return "Inventory (" .. #visible:get() .. ")" end }),
        ui.text({ name = "hp", text = function()
            local h = hp:get().data
            return h.current .. "/" .. h.max
        end }),
        ui.dynamic(function()
            local h = hp:get().data
            if h.current < h.max * 0.25 then return ui.text({ key = "low", name = "low", text = "LOW HP" }) end
        end),
        ui.text({ name = "clicks", text = function() return "clicks: " .. clicks:get() end }),
        ui.dynamic(function()
            local out = {}
            for _, name in ipairs(visible:get()) do
                out[#out + 1] = ui.el(Slot, {
                    key = name, name = name, entity = State.entities[name], selected = selected, on_select = select,
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

function on_script_loaded()
    State.root = ui.mount("inventory", Inventory, { player = State.player })
end

-- the test steps, the same for both models

local function find(name) return ui.find(State.root, name) end
local function text(name)
    local e = find(name)
    assert(e ~= nil, "no element named " .. name)
    return ui.describe(e).text
end
local function expect(name, expected)
    local actual = text(name)
    assert(actual == expected, name .. ": expected `" .. expected .. "`, got `" .. tostring(actual) .. "`")
end

local actions = {
    [1] = function()
        world.get_component(State.player, Health).data = { current = 20, max = 100 }
    end,
    [2] = function() ui.interact(find("slot_Sword"), "press") end,
    [3] = function() State.search:set("sh") end,
    [4] = function()
        world.get_component(State.entities.Shield, Item).data = { name = "Shield", count = 5 }
    end,
    [5] = function()
        world.despawn(State.entities.Sword)
        State.items:set({ "Shield", "Potion" })
    end,
}

local checks = {
    [0] = function()
        expect("version", "v1")
        expect("title", "Inventory (3)")
        expect("hp", "100/100")
        assert(find("low") == nil, "low hp shown at full health")
        expect("clicks", "clicks: 0")
        expect("count_Sword", "x1")
        assert(find("details") == nil, "details shown without a selection")
    end,
    [1] = function()
        expect("hp", "20/100")
        expect("low", "LOW HP")
    end,
    [2] = function()
        expect("details", "details: Sword")
        expect("clicks", "clicks: 1")
        local sword, shield = ui.describe(find("slot_Sword")), ui.describe(find("slot_Shield"))
        assert(sword.bg ~= shield.bg, "the selected slot is not highlighted")
    end,
    [3] = function()
        expect("title", "Inventory (1)")
        assert(find("slot_Sword") == nil, "filtered out slot still shown")
        assert(find("slot_Shield") ~= nil, "matching slot not shown")
        expect("details", "details: Sword")
    end,
    [4] = function()
        expect("count_Shield", "x5")
    end,
    [5] = function()
        assert(find("details") == nil, "selection of a removed item was not cleared")
    end,
    [6] = function()
        expect("version", "v2")
        expect("title", "Inventory (1)")
        expect("clicks", "clicks: 1")
        expect("hp", "20/100")
        expect("count_Shield", "x5")
    end,
}

function on_test()
    local action = actions[State.step]
    if action then action() end
end

function on_test_last()
    checks[State.step]()
    local done = "step " .. State.step
    State.step = State.step + 1
    return done
end
