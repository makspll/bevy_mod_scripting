-- signals: components run once, properties are bound to functions, `show` regions produce changing children
local bms = import_namespace("bms")
local world = bms.world
local DynamicComponent = bms.DynamicComponent
local ui = import_namespace("reactive_ui")

-- script components hold a `DynamicComponent`, which watching them gives
---@type bms.ScriptComponentRegistration<bms.DynamicComponent>
local Health = world.register_new_component("script.reactive_ui", "Health", true)
---@type bms.ScriptComponentRegistration<bms.DynamicComponent>
local Item = world.register_new_component("script.reactive_ui", "Item", true)

---@class InventoryState
---@field step integer
---@field player ecs.Entity
---@field entities table<string, ecs.Entity>
---@field items reactive_ui.Signal<string[]>
---@field search reactive_ui.Signal<string>
---@field root ecs.Entity
---@field row_clicked string?

-- globals survive reloads, the context is reused
---@type integer
Reloads = (Reloads or 0) + 1
---@type InventoryState
State = State
if State == nil then
    local player = world.spawn()
    world.insert_component(player, Health, DynamicComponent.new({ data = { current = 100, max = 100 } }))
    ---@type table<string, ecs.Entity>
    local entities = {}
    for _, name in ipairs({ "Sword", "Shield", "Potion" }) do
        local e = world.spawn()
        world.insert_component(e, Item, DynamicComponent.new({ data = { name = name, count = 1 } }))
        entities[name] = e
    end
    State = {
        step = 0,
        player = player,
        entities = entities,
        items = ui.signal({ "Sword", "Shield", "Potion" }),
        search = ui.signal(""),
    }
end

---@param list string[]
---@param value string
local function contains(list, value)
    for _, v in ipairs(list) do
        if v == value then return true end
    end
    return false
end

-- called with its arguments instead of a props table, see `Slot(..)` below
---@param name string
---@param entity ecs.Entity
---@param selected reactive_ui.Selector<string?>
---@param on_select fun(name: string)
local Slot = ui.component("Slot", function(name, entity, selected, on_select)
    local item = ui.watch(entity, Item)
    return ui.node({
        name = "slot_" .. name,
        -- computed in Rust, no script code runs when these change
        background_color = ui.when(ui.selected(selected, name), "#ffcc00", "#333333"),
        on_click = function() on_select(name) end,
    }, {
        ui.text({ name = "count_" .. name, text = ui.format("x{}", { ui.path(item, "data.count") }) }),
    })
end)

---@class Labels
---@field details string

-- read by `Details` wherever it is, without going through the props of what is in between
---@type reactive_ui.UiContext<Labels>
Labels = Labels or ui.create_context({ details = "details: " })

---@param p { name: string }
local Details = ui.component("Details", function(p)
    return ui.text({ name = "details", text = ui.use_context(Labels).details .. p.name })
end)

---@param p { player: ecs.Entity }
local Inventory = ui.component("Inventory", function(p)
    local hp = ui.watch(p.player, Health)
    ---@type reactive_ui.Signal<string|nil>
    local selected = ui.signal(nil)
    local is_selected = ui.selector(selected)
    local clicks = ui.signal(0)
    -- the signals it reads are tracked, wherever they come from
    local visible = ui.computed(function()
        local query = State.search:get()
        ---@type string[]
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
    local visible_entities = ui.computed(function()
        ---@type ecs.Entity[]
        local out = {}
        for _, name in ipairs(visible:get()) do out[#out + 1] = State.entities[name] end
        return out
    end)

    return ui.node({
        name = "panel", flex_direction = "Column", justify_content = "SpaceBetween", max_width = "50%",
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
            -- `Slot` renders with these values, `Details` below gets a props table through Rust
            return Slot(name, State.entities[name], is_selected, select)
        end),
        -- the region below is mounted later, and still sees what is provided here
        ui.provide(Labels, { details = "selected: " }, {
            ui.show(function()
                local s = selected:get()
                if s ~= nil then return ui.keyed(s, Details({ name = s })) end
            end),
        }),
        -- a row template: runs once, `item` stands for the entity of each row, functions get the row's entity
        ui.list_template(visible_entities, function(item)
            local data = ui.watch_item(item, Item)
            local name = ui.path(data, "data.name")
            return ui.node({
                name = ui.format("row_{}", { name }),
                background_color = ui.when(ui.selected(is_selected, name), "#ffcc00", "#333333"),
                -- in a row template handlers get the row's item first
                ---@param entity ecs.Entity
                on_click = function(entity)
                    local clicked = assert(world.get_component(entity, Item)) --[[@as bms.DynamicComponent]]
                    State.row_clicked = clicked.data.name --[[@as string]]
                end,
            }, {
                ui.text({ name = ui.format("row_text_{}", { name }), text = ui.format("{} x{}", { name, ui.path(data, "data.count") }) }),
            })
        end),
    })
end)

function on_script_loaded()
    State.root = ui.render("inventory", Inventory({ player = State.player }))
end

-- the test steps, the same for both models

local function find(name) return ui.find(State.root, name) end
local function get(name) return assert(find(name), "no element named " .. name) end
local function text(name) return ui.describe(get(name)).text end
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
        ui.interact(get("slot_Sword"), "click")
        -- on the text inside the row, which has no handler: the row's handler gets the event
        ui.interact(get("row_text_Shield"), "click")
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
        local panel = ui.describe(get("panel"))
        assert(string.find(panel.layout, "justify_content: SpaceBetween", 1, true), panel.layout)
        assert(string.find(panel.layout, "max_width: Percent(50.0)", 1, true), panel.layout)
        assert(string.lower(panel.border_color) == "#ffffff", panel.border_color)
        local version = ui.describe(get("version"))
        assert(version.font_size == "Px(20.0)", version.font_size)
        assert(string.lower(version.color) == "#ff0000", version.color)
        assert(ui.describe(get("icon")).image, "the image element has no image")
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
        expect("details", "selected: Sword")
        expect("clicks", "clicks: 1")
        local sword, shield = ui.describe(get("slot_Sword")), ui.describe(get("slot_Shield"))
        assert(sword.background_color ~= shield.background_color, "the selected slot is not highlighted")
        local sword_row, shield_row = ui.describe(get("row_Sword")), ui.describe(get("row_Shield"))
        assert(sword_row.background_color == sword.background_color and shield_row.background_color == shield.background_color, "the selected row is not highlighted")
        assert(State.row_clicked == "Shield", "the row handler did not get its entity")
    end,
    [3] = function()
        expect("title", "Inventory (1)")
        assert(find("row_Sword") == nil, "filtered out row still shown")
        assert(find("slot_Sword") == nil, "filtered out slot still shown")
        assert(find("slot_Shield") ~= nil, "matching slot not shown")
        expect("details", "selected: Sword")
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
