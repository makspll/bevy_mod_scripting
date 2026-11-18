-- signals: components run once, properties are bound to functions, `show` regions produce changing children
local bms = import_namespace("bms")
local world = bms.world
local DynamicComponent = bms.DynamicComponent
local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

-- script components hold a `DynamicComponent`, which watching them gives
---@type bms.ScriptComponentRegistration<bms.DynamicComponent>
local Health = world.register_new_component("script.reactive_ui", "Health", true)
---@type bms.ScriptComponentRegistration<bms.DynamicComponent>
local Item = world.register_new_component("script.reactive_ui", "Item", true)

---@class InventoryState
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

-- the test, with `reactive_ui_test`: events and `act` run what they change before returning

---@param s reactive_ui.Screen
---@param name string
---@param expected string
local function expect_text(s, name, expected)
    s:expect_style(s:get_by_name(name), { text = expected })
end

---@param s reactive_ui.Screen
local function first_load(s)
    -- properties are checked as the script sets them
    s:expect_style(s:get_by_name("panel"), {
        flex_direction = "Column", justify_content = "SpaceBetween", max_width = "50%",
        padding = { 4, 8 }, border = 1, border_color = "#ffffff", border_radius = 3,
    })
    s:expect_style(s:get_by_name("version"), { text = "v1", font_size = 20, color = "#ff0000" })
    s:expect_style(s:get_by_name("icon"), { image = "icons/sword.png", width = 16, height = 16 })
    -- laid out in a 1280x720 window
    local icon, panel = s:rect(s:get_by_name("icon")), s:rect(s:get_by_name("panel"))
    assert(icon.width == 16 and icon.height == 16, "the icon is " .. icon.width .. "x" .. icon.height)
    assert(panel.x == 0 and panel.y == 0 and panel.width <= 640, "the panel is at " .. panel.x .. ", " .. panel.y)
    expect_text(s, "title", "Inventory (3)")
    expect_text(s, "hp", "100/100")
    assert(s:query_by_name("low") == nil, "low hp shown at full health")
    expect_text(s, "clicks", "clicks: 0")
    expect_text(s, "count_Sword", "x1")
    assert(s:get_by_text("Sword x1") == s:get_by_name("row_text_Sword"), "rows are found by their text")
    assert(s:query_by_name("details") == nil, "details shown without a selection")
    s:updates()

    -- a watched component changes
    s:act(function()
        local hp = assert(world.get_component(State.player, Health)) --[[@as bms.DynamicComponent]]
        hp.data = { current = 20, max = 100 }
    end)
    expect_text(s, "hp", "20/100")
    expect_text(s, "low", "LOW HP")
    local updates = s:updates()
    assert(updates.renders.Inventory == nil, "the component rendered again")
    assert(updates.bindings == 1 and updates.regions == 1, "only the hp text and the low hp region run again")

    -- a click handler sets component state, a click on the text inside a row reaches the row's handler
    s:click(s:get_by_name("slot_Sword"))
    s:click(s:get_by_name("row_text_Shield"))
    expect_text(s, "details", "selected: Sword")
    expect_text(s, "clicks", "clicks: 1")
    s:expect_style(s:get_by_name("slot_Sword"), { background_color = "#ffcc00" })
    s:expect_style(s:get_by_name("slot_Shield"), { background_color = "#333333" })
    s:expect_style(s:get_by_name("row_Sword"), { background_color = "#ffcc00" })
    assert(State.row_clicked == "Shield", "the row handler did not get its entity")
    local row = s:within(s:get_by_name("row_Shield"))
    assert(row:query_by_text("Sword x1") == nil and #row:all_by_text("Shield x1") == 1, "within finds inside the row only")

    -- a global signal filters a keyed list
    s:act(function() State.search:set("sh") end)
    expect_text(s, "title", "Inventory (1)")
    assert(s:query_by_name("row_Sword") == nil, "filtered out row still shown")
    assert(s:query_by_name("slot_Sword") == nil, "filtered out slot still shown")
    assert(s:query_by_name("slot_Shield") ~= nil, "matching slot not shown")
    expect_text(s, "details", "selected: Sword")

    -- a list item's watched component changes
    s:act(function()
        local shield = assert(world.get_component(State.entities.Shield, Item)) --[[@as bms.DynamicComponent]]
        shield.data = { name = "Shield", count = 5 }
    end)
    expect_text(s, "count_Shield", "x5")
    expect_text(s, "row_text_Shield", "Shield x5")

    -- an effect clears the selection once the item is gone
    s:act(function()
        world.despawn(State.entities.Sword)
        State.items:set({ "Shield", "Potion" })
    end)
    assert(s:query_by_name("details") == nil, "selection of a removed item was not cleared")

    -- a component rendered on its own, with a context provided
    local d = t.render(Details({ name = "Potion" }), { t.context(Labels, { details = "given: " }) })
    expect_text(d, "details", "given: Potion")
    -- the whole tree as text, the same for the same ui
    local snapshot = d:snapshot()
    assert(snapshot == 'text name="details" text="given: Potion"\n', snapshot)
    expect_text(t.render(Details({ name = "Potion" })), "details", "details: Potion")
end

-- after a reload the new code renders and the state survives
---@param s reactive_ui.Screen
local function after_reload(s)
    expect_text(s, "version", "v2")
    expect_text(s, "title", "Inventory (1)")
    expect_text(s, "clicks", "clicks: 1")
    expect_text(s, "hp", "20/100")
    expect_text(s, "count_Shield", "x5")
    expect_text(s, "row_text_Shield", "Shield x5")
end

function on_test()
    local s = t.screen(State.root)
    if Reloads == 1 then first_load(s) else after_reload(s) end
    return "load " .. Reloads
end
