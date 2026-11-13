-- The ui of the `reactive_ui` example: an inventory of items which live in the ECS as `Item` components.
-- Rust restocks items and animates them, the ui shows it. Clicks change the components, Rust sees it.
local bms = import_namespace("bms")
local world = bms.world
local ui = import_namespace("reactive_ui")

local Item = world.get_type_by_name("inventory::Item")
local UiStats = world.get_type_by_name("inventory::UiStats")

-- script state, kept across reloads in globals
Kind = Kind or ui.signal("all")
Selected = Selected or ui.signal(nil)
Hovered = Hovered or ui.signal(nil)

local KINDS = { "all", "weapon", "armour", "potion", "scroll" }
-- the stock a full bar stands for
local FULL = 20

local theme = {
    page = "#101218", panel = "#171a22", card = "#1e222c", card_hover = "#272c38", empty = "#3a1f24",
    edge = "#2b303c", edge_hover = "#56607a", selected = "#f2c14e", flash = "#7be0a0",
    track = "#2b303c", fill = "#4fae78", fill_flash = "#b6f5cc",
    text = "#e8ebf2", muted = "#7d8597", accent = "#5b7cfa", accent_hover = "#7893ff",
    button = "#2b3142", button_hover = "#3a4259",
}

-- the items are found once, what they hold is read through the ui
local items, by_name = {}, {}
for _, result in ipairs(world.query():component(Item):build()) do
    -- the entity, then the components asked for
    local entity, item = result[1], result[2]
    items[#items + 1] = { entity = entity, kind = item.kind }
    by_name[item.name] = entity
end
local stats_entity = world.query():component(UiStats):build()[1][1]

-- A button which lights up under the pointer. Call it at the top of a component: its hover state is a signal
-- of the component's instance. `active` is an optional function, an active button keeps the accent colour.
local function button(label, on_click, active, colour)
    local hovered = ui.signal(false)
    return ui.node({
        padding = { 6, 14 }, border_radius = 6, on_click = on_click,
        on_hover = function(over) hovered:set(over) end,
        background_color = function()
            if active and active() then return hovered:get() and theme.accent_hover or theme.accent end
            if hovered:get() then return theme.button_hover end
            return colour or theme.button
        end,
    }, { ui.text({ text = label, font_size = 14, color = theme.text }) })
end

-- A bar filled to `value` out of `FULL`, both computed in Rust so the bar follows the eased `shown`.
local function stock_bar(value, fill, height)
    return ui.node({
        flex_grow = 1, height = height, border_radius = height / 2, background_color = theme.track,
        overflow = "clip",
    }, {
        ui.node({ width = ui.percent(value, FULL), height = "100%", border_radius = height / 2, background_color = fill }),
    })
end

-- The slots are a row template: this function runs once for all 500 rows. `item` stands for the entity of
-- each row, and every property is a value computed in Rust, so no script code runs when Rust restocks or
-- animates an item.
local function slots(visible, is_selected, is_hovered)
    return ui.list_template(visible, function(item)
        local data = ui.watch_item(item, Item)
        local name, count = ui.path(data, "name"), ui.path(data, "count")
        local flashing = ui.lt(0, ui.path(data, "flash"))
        local hovered = ui.selected(is_hovered, name)
        return ui.node({
            -- grow to fill the width of the panel
            flex_basis = 150, flex_grow = 1, max_width = 190, height = 68, padding = { 8, 10 }, border = 2, border_radius = 8,
            flex_direction = "column", justify_content = "space_between",
            border_color = ui.when(ui.selected(is_selected, name), theme.selected,
                ui.when(flashing, theme.flash, ui.when(hovered, theme.edge_hover, theme.edge))),
            background_color = ui.when(ui.lt(count, 1), theme.empty, ui.when(hovered, theme.card_hover, theme.card)),
            -- one handler for every row, called with the row's entity
            on_click = function(entity) Selected:set(world.get_component(entity, Item).name) end,
            on_hover = function(entity, over)
                local hovered_name = world.get_component(entity, Item).name
                if over then
                    Hovered:set(hovered_name)
                elseif Hovered:peek() == hovered_name then
                    Hovered:set(nil)
                end
            end,
        }, {
            ui.node({ flex_direction = "row", justify_content = "space_between", align_items = "center" }, {
                ui.text({ text = name, font_size = 14, color = theme.text }),
                ui.text({ text = ui.format("x{}", { count }), font_size = 13, color = ui.when(flashing, theme.flash, theme.muted) }),
            }),
            ui.text({ text = ui.path(data, "kind"), font_size = 11, color = theme.muted }),
            stock_bar(ui.path(data, "shown"), ui.when(flashing, theme.fill_flash, theme.fill), 6),
        })
    end, nil, {
        -- the layout of the list itself
        flex_direction = "row", flex_wrap = "wrap", row_gap = 8, column_gap = 8,
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
    local use_one = button("use one", add(-1), nil, "#5a2e36")
    local restock = button("+10", add(10), nil, "#2e5a40")
    local close = button("close", function() Selected:set(nil) end)
    return ui.node({ flex_direction = "row", column_gap = 16, align_items = "center", width = "100%" }, {
        ui.node({ flex_direction = "column", row_gap = 6, width = 260 }, {
            ui.text({ text = ui.path(item, "name"), font_size = 20, color = theme.selected }),
            ui.node({ flex_direction = "row", column_gap = 10, align_items = "center" }, {
                stock_bar(ui.path(item, "shown"), theme.fill, 10),
                ui.text({ text = ui.format("{} in stock", { ui.path(item, "count") }), font_size = 13, color = theme.muted }),
            }),
        }),
        ui.node({ flex_grow = 1 }, {}),
        use_one, restock, close,
    })
end)

local Inventory = ui.component("Inventory", function()
    local stats = ui.watch(stats_entity, UiStats)
    local is_selected = ui.selector(Selected)
    local is_hovered = ui.selector(Hovered)
    -- the entities of the items of the chosen kind
    local visible = ui.computed(function()
        local kind = Kind:get()
        local out = {}
        for _, item in ipairs(items) do
            if kind == "all" or item.kind == kind then out[#out + 1] = item.entity end
        end
        return out
    end)

    local tabs = {}
    for i, kind in ipairs(KINDS) do
        tabs[i] = button(kind, function() Kind:set(kind) end, function() return Kind:get() == kind end)
    end

    return ui.node({
        width = "100%", height = "100%", flex_direction = "column", padding = 16, row_gap = 12,
        background_color = theme.page,
    }, {
        -- header: title and stats, then the kinds
        ui.node({ flex_direction = "row", align_items = "center", column_gap = 8 }, {
            ui.node({ flex_direction = "column", row_gap = 2, flex_grow = 1 }, {
                ui.text({ text = "Inventory", font_size = 26, color = theme.text }),
                ui.text({
                    text = ui.format("{} items  |  ui update {} us  |  restocked by Rust {}", {
                        #items, ui.path(stats, "update_micros"), ui.path(stats, "restocked"),
                    }),
                    font_size = 12, color = theme.muted,
                }),
            }),
            ui.node({ flex_direction = "row", column_gap = 6 }, tabs),
        }),
        -- the selected item
        ui.show(function()
            local name = Selected:get()
            if name ~= nil then return ui.element(Details, { key = name, entity = by_name[name] }) end
            return ui.text({ text = "Click an item to inspect it", font_size = 16, color = theme.muted })
        end, nil, {
            background_color = theme.panel, border_radius = 10, padding = { 12, 16 }, height = 72, flex_shrink = 0,
            align_items = "center",
        }),
        -- scrolls with the mouse wheel
        ui.node({
            overflow = { y = "scroll" }, flex_direction = "column", flex_grow = 1, min_height = 0,
            background_color = theme.panel, border_radius = 10, padding = 14,
        }, {
            slots(visible, is_selected, is_hovered),
        }),
        ui.text({
            text = "scroll for more  -  green flashes are Rust restocking  -  edit this script to hot reload",
            font_size = 11, color = theme.muted,
        }),
    })
end)

function on_script_loaded()
    ui.render("inventory", Inventory, {})
end
