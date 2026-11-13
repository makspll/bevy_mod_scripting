-- The ui of the `reactive_ui` example: an inventory of items which live in the ECS as `Item` components.
-- Rust restocks items, the ui shows it. Clicks change the components, Rust sees it.
local bms = import_namespace("bms")
local world = bms.world
local ui = import_namespace("reactive_ui")

local Item = world.get_type_by_name("inventory::Item")
local UiStats = world.get_type_by_name("inventory::UiStats")

-- script state, kept across reloads in globals
Kind = Kind or ui.signal("all")
Selected = Selected or ui.signal(nil)

local KINDS = { "all", "weapon", "armour", "potion", "scroll" }

-- the items are found once, what they hold is read through the ui
local items, by_name = {}, {}
for _, result in ipairs(world.query():component(Item):build()) do
    -- the entity, then the components asked for
    local entity, item = result[1], result[2]
    items[#items + 1] = { entity = entity, kind = item.kind }
    by_name[item.name] = entity
end
local stats_entity = world.query():component(UiStats):build()[1][1]

local function button(label, on_click, background)
    return ui.node({
        padding = { 4, 10 }, border_radius = 4, background_color = background or "#3b4a6b", on_click = on_click,
    }, { ui.text({ text = label, font_size = 14 }) })
end

-- The slots are a row template: this function runs once for all 500 rows. `item` stands for the entity of
-- each row, and every property is a value computed in Rust, so no script code runs when Rust restocks an item.
local function slots(visible, is_selected)
    return ui.list_template(visible, function(item)
        local data = ui.watch_item(item, Item)
        local name, count = ui.path(data, "name"), ui.path(data, "count")
        return ui.node({
            width = 118, height = 46, padding = 5, border = 2, border_radius = 5,
            flex_direction = "column", justify_content = "space_between",
            border_color = ui.when(ui.selected(is_selected, name), "#ffcc00", "#3a3a3a"),
            background_color = ui.when(ui.lt(count, 1), "#3a2020", "#262626"),
            -- one handler for every row, called with the row's entity
            on_click = function(entity) Selected:set(world.get_component(entity, Item).name) end,
        }, {
            ui.text({ text = name, font_size = 13 }),
            ui.text({ text = ui.format("x{}", { count }), font_size = 12, color = "#9fd49f" }),
        })
    end, nil, {
        -- the layout of the list itself
        flex_direction = "row", flex_wrap = "wrap", row_gap = 6, column_gap = 6,
    })
end

-- The selected item is a component with closures: there is one of it, so being flexible costs nothing.
-- Its buttons write to the `Item` component, like any script would.
local Details = ui.component("Details", function(props)
    local item = ui.watch(props.entity, Item)
    local function add(amount)
        return function()
            local data = world.get_component(props.entity, Item)
            data.count = math.max(0, data.count + amount)
        end
    end
    return ui.node({ flex_direction = "row", column_gap = 8, align_items = "center" }, {
        ui.text({ text = ui.format("{}: {}", { ui.path(item, "name"), ui.path(item, "count") }), font_size = 18 }),
        button("use one", add(-1), "#6b3b3b"),
        button("+10", add(10), "#3b6b45"),
        button("close", function() Selected:set(nil) end),
    })
end)

local Inventory = ui.component("Inventory", function()
    local stats = ui.watch(stats_entity, UiStats)
    local is_selected = ui.selector(Selected)
    -- the entities of the items of the chosen kind
    local visible = ui.computed(function()
        local kind = Kind:get()
        local out = {}
        for _, item in ipairs(items) do
            if kind == "all" or item.kind == kind then out[#out + 1] = item.entity end
        end
        return out
    end)

    local kinds = {}
    for i, kind in ipairs(KINDS) do
        kinds[i] = ui.node({
            padding = { 4, 10 }, border_radius = 4,
            background_color = function() return Kind:get() == kind and "#8a6d1f" or "#3b4a6b" end,
            on_click = function() Kind:set(kind) end,
        }, { ui.text({ text = kind, font_size = 14 }) })
    end

    return ui.node({ width = "100%", height = "100%", flex_direction = "column", padding = 12, row_gap = 10 }, {
        ui.node({ flex_direction = "row", column_gap = 8, align_items = "center" }, kinds),
        ui.text({
            text = ui.format("ui update: {} µs    restocked by Rust: {}", {
                ui.path(stats, "update_micros"), ui.path(stats, "restocked"),
            }),
            font_size = 14, color = "#aaaaaa",
        }),
        ui.show(function()
            local name = Selected:get()
            if name ~= nil then return ui.element(Details, { key = name, entity = by_name[name] }) end
            return ui.text({ text = "click an item", font_size = 18, color = "#777777" })
        end),
        -- scrolls with the mouse wheel
        ui.node({ overflow = { y = "scroll" }, flex_direction = "column", flex_grow = 1, min_height = 0 }, {
            slots(visible, is_selected),
        }),
    })
end)

function on_script_loaded()
    ui.render("inventory", Inventory, {})
end
