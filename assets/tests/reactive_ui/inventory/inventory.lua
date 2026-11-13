-- signals: components run once, properties are bound to functions, `show` regions produce changing children
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

-- rendered with dependencies instead of a props table, see the `element` below
local function slot(name, entity, selected, on_select)
    local item = ui.watch(entity, Item)
    return ui.node({
        name = "slot_" .. name,
        -- computed in Rust, no script code runs when these change
        background_color = ui.when(ui.selected(selected, name), "#ffcc00", "#333333"),
        -- a function given what it needs, instead of capturing it
        on_click = { function(on_select, name) on_select(name) end, on_select, name },
    }, {
        ui.text({ name = "count_" .. name, text = ui.format("x{}", { ui.path(item, "data.count") }) }),
    })
end
local Slot = ui.component("Slot", slot)

local Details = ui.component("Details", function(p)
    return ui.text({ name = "details", text = "details: " .. p.name })
end)

local Inventory = ui.component("Inventory", function(p)
    local hp = ui.watch(p.player, Health)
    local selected = ui.signal(nil)
    local is_selected = ui.selector(selected)
    local clicks = ui.signal(0)
    -- the signals it reads are its dependencies, captures work as well in Lua (the effect below)
    local visible = ui.computed(function(search, items)
        local query = search:get()
        local out = {}
        for _, name in ipairs(items:get()) do
            if string.find(string.lower(name), query, 1, true) then out[#out + 1] = name end
        end
        return out
    end, { State.search, State.items })
    ui.effect(function()
        local s = selected:get()
        if s ~= nil and not contains(State.items:get(), s) then selected:set(nil) end
    end)
    local function select(name)
        selected:set(name)
        clicks:set(clicks:peek() + 1)
    end
    local visible_entities = ui.computed(function()
        local out = {}
        for _, name in ipairs(visible:get()) do out[#out + 1] = State.entities[name] end
        return out
    end)

    return ui.node({
        name = "panel", flex_direction = "column", justify_content = "space_between", max_width = "50%",
        padding = { 4, 8 }, border = 1, border_color = "#ffffff", border_radius = 3,
    }, {
        ui.text({ name = "version", text = "v" .. Reloads, font_size = 20, color = "#ff0000" }),
        ui.image({ name = "icon", image = "icons/sword.png", width = 16, height = 16 }),
        ui.text({ name = "title", text = function() return "Inventory (" .. #visible:get() .. ")" end }),
        ui.text({ name = "hp", text = ui.format("{}/{}", { ui.path(hp, "data.current"), ui.path(hp, "data.max") }) }),
        ui.show(function()
            local h = hp:get().data
            if h.current < h.max * 0.25 then return ui.text({ key = "low", name = "low", text = "LOW HP" }) end
        end),
        ui.text({ name = "clicks", text = function() return "clicks: " .. clicks:get() end }),
        -- one child per item, only built for new items
        ui.list(visible, function(name)
            -- `slot` is called with these values, `Details` below gets a props table through Rust
            return ui.element(Slot, slot, { name, State.entities[name], is_selected, select })
        end),
        ui.show(function()
            local s = selected:get()
            if s ~= nil then return ui.element(Details, { key = s, name = s }) end
        end),
        -- a row template: runs once, `item` stands for the entity of each row, functions get the row's entity
        ui.list_template(visible_entities, function(item)
            local data = ui.watch_item(item, Item)
            local name = ui.path(data, "data.name")
            return ui.node({
                name = ui.format("row_{}", { name }),
                background_color = ui.when(ui.selected(is_selected, name), "#ffcc00", "#333333"),
                on_click = function(entity)
                    local clicked = assert(world.get_component(entity, Item)) --[[@as bms.DynamicComponent]]
                    State.row_clicked = clicked.data.name
                end,
            }, {
                ui.text({ name = ui.format("row_text_{}", { name }), text = ui.format("{} x{}", { name, ui.path(data, "data.count") }) }),
            })
        end),
    })
end)

function on_script_loaded()
    State.root = ui.render("inventory", Inventory, { player = State.player })
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
        local hp = assert(world.get_component(State.player, Health)) --[[@as bms.DynamicComponent]]
        hp.data = { current = 20, max = 100 }
    end,
    [2] = function()
        ui.interact(find("slot_Sword"), "press")
        ui.interact(find("row_Shield"), "press")
    end,
    [3] = function() State.search:set("sh") end,
    [4] = function()
        local shield = assert(world.get_component(State.entities.Shield, Item)) --[[@as bms.DynamicComponent]]
        shield.data = { name = "Shield", count = 5 }
    end,
    [5] = function()
        world.despawn(State.entities.Sword)
        State.items:set({ "Shield", "Potion" })
    end,
}

local checks = {
    [0] = function()
        expect("version", "v1")
        -- layout, colours, text style and images are plain values
        local panel = ui.describe(find("panel"))
        assert(string.find(panel.layout, "justify_content: SpaceBetween", 1, true), panel.layout)
        assert(string.find(panel.layout, "max_width: Percent(50.0)", 1, true), panel.layout)
        assert(string.lower(panel.border_color) == "#ffffff", panel.border_color)
        local version = ui.describe(find("version"))
        assert(version.font_size == "Px(20.0)", version.font_size)
        assert(string.lower(version.color) == "#ff0000", version.color)
        assert(ui.describe(find("icon")).image, "the image element has no image")
        expect("title", "Inventory (3)")
        expect("hp", "100/100")
        assert(find("low") == nil, "low hp shown at full health")
        expect("clicks", "clicks: 0")
        expect("count_Sword", "x1")
        expect("row_text_Sword", "Sword x1")
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
        assert(sword.background_color ~= shield.background_color, "the selected slot is not highlighted")
        local sword_row, shield_row = ui.describe(find("row_Sword")), ui.describe(find("row_Shield"))
        assert(sword_row.background_color == sword.background_color and shield_row.background_color == shield.background_color, "the selected row is not highlighted")
        assert(State.row_clicked == "Shield", "the row handler did not get its entity")
    end,
    [3] = function()
        expect("title", "Inventory (1)")
        assert(find("row_Sword") == nil, "filtered out row still shown")
        assert(find("slot_Sword") == nil, "filtered out slot still shown")
        assert(find("slot_Shield") ~= nil, "matching slot not shown")
        expect("details", "details: Sword")
    end,
    [4] = function()
        expect("count_Shield", "x5")
        expect("row_text_Shield", "Shield x5")
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
        expect("row_text_Shield", "Shield x5")
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
