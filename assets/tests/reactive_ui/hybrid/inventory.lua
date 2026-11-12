-- hybrid: components re-render when the state they read changes, their output is diffed against the last render
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

local Slot = ui.define_hybrid("Slot", function(p)
    local item = ui.watch(p.entity, Item):get()
    return ui.node({
        name = "slot_" .. p.name,
        bg = p.selected and "#ffcc00" or "#333333",
        on_click = function() p.on_select(p.name) end,
    }, {
        ui.text({ name = "count_" .. p.name, text = "x" .. item.data.count }),
    })
end)

local Details = ui.define_hybrid("Details", function(p)
    return ui.text({ name = "details", text = "details: " .. p.name })
end)

local Inventory = ui.define_hybrid("Inventory", function(p)
    local h = ui.watch(p.player, Health):get().data
    local selected = ui.signal(nil)
    local clicks = ui.signal(0)
    local query = State.search:get()
    local visible = {}
    for _, name in ipairs(State.items:get()) do
        if string.find(string.lower(name), query, 1, true) then visible[#visible + 1] = name end
    end
    ui.effect(function()
        local s = selected:get()
        if s ~= nil and not contains(State.items:get(), s) then selected:set(nil) end
    end)
    local function select(name)
        selected:set(name)
        clicks:set(clicks:peek() + 1)
    end

    local sel = selected:get()
    local slots = {}
    for _, name in ipairs(visible) do
        slots[#slots + 1] = ui.el(Slot, {
            key = name, name = name, entity = State.entities[name], selected = sel == name, on_select = select,
        })
    end
    local children = {
        ui.text({ key = "version", name = "version", text = "v" .. Reloads }),
        ui.text({ key = "title", name = "title", text = "Inventory (" .. #visible .. ")" }),
        ui.text({ key = "hp", name = "hp", text = h.current .. "/" .. h.max }),
    }
    if h.current < h.max * 0.25 then
        children[#children + 1] = ui.text({ key = "low", name = "low", text = "LOW HP" })
    end
    children[#children + 1] = ui.text({ key = "clicks", name = "clicks", text = "clicks: " .. clicks:get() })
    children[#children + 1] = ui.node({ key = "slots" }, slots)
    if sel ~= nil then
        children[#children + 1] = ui.el(Details, { key = "details_" .. sel, name = sel })
    end
    return ui.node({ direction = "column" }, children)
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
