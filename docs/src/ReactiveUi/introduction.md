# Reactive UI

> [!WARNING]
> The reactive UI is an experimental feature, its API can still change.

`bevy_mod_scripting_reactive_ui` lets scripts describe `bevy_ui` trees which keep themselves up to date. You say what the UI looks like for the current state, and when the state changes only the parts which read it are updated. It works the same from every scripting language.

```rust,ignore
app.add_plugins(ReactiveUiPlugin);
```

```lua
local ui = import_namespace("reactive_ui")

local Counter = ui.component("Counter", function(props)
    local count = ui.signal(props.start)
    return ui.node({ flex_direction = "row", column_gap = 8 }, {
        ui.node({ padding = 4, background_color = "#444444", on_click = function() count:set(count:get() + 1) end }, {
            ui.text("+1"),
        }),
        ui.text({ text = function() return "count: " .. count:get() end }),
    })
end)

function on_script_loaded()
    ui.render("counter", Counter, { start = 0 })
end
```

Coming from another UI framework? [Start here](./coming-from.md).

## The model in five lines

- A **component** is a function returning elements. It runs **once** per instance (and again when its script reloads), not on every change.
- A **signal** holds state: `s:get()` reads it, `s:set(v)` changes it. `ui.computed` derives a value, `ui.effect` reacts to changes, `ui.watch(entity, Component)` is a signal over an ECS component.
- A property given a **function** is *bound*: the function re-runs when a signal it read changes, and only that property is written.
- **Regions** are the parts of the tree whose shape changes: `ui.show` for conditional content, `ui.list` for one child per item.
- Everything is flushed once per frame, before `bevy_ui` lays out.

## Elements

```lua
ui.node(props, children)   -- a container, one bevy_ui entity
ui.text(props)             -- text, `ui.text("hello")` for short
ui.image(props)            -- a node showing the image asset at `image`
ui.element(Component, props)
```

Properties are the layout fields of `bevy_ui`'s `Node` under their own names, taking plain values, plus colours, text style and events. See the [reference](./reference.md).

```lua
ui.node({
    flex_direction = "column", align_items = "center", justify_content = "space_between",
    width = "50%", max_height = 300, padding = { 4, 8 }, margin = { left = 2 },
    border = 1, border_color = "#ffffff", border_radius = 6, background_color = "#222222",
})
```

## Talking to the ECS, both ways

The UI reads the world through `ui.watch`, and writes to it like any script does, from handlers and effects.

```lua
local Health = world.register_new_component("script.game", "Health", true)

local HealthBar = ui.component("HealthBar", function(props)
    local health = ui.watch(props.player, Health)   -- updates when the component changes, whoever changed it
    return ui.node({ width = 200, height = 16, background_color = "#330000" }, {
        ui.node({
            width = ui.percent(ui.path(health, "data.current"), ui.path(health, "data.max")),
            height = "100%", background_color = "#ff3333",
            -- and back: a click changes the component, which updates the bar
            on_click = function() world.get_component(props.player, Health).data = { current = 100, max = 100 } end,
        }),
    })
end)
```

Watched components are checked by their change tick once per frame, so a system written in Rust changing `Health` updates the bar without any event or glue code.

## State which outlives a component

Signals made outside of a component are plain script state: keep them in a global (Lua) or the script's `state()` (Rune) and any component can read them. To hand a value to a whole subtree without passing it through every component's props, provide it as a context:

```lua
local Theme = ui.create_context({ accent = "#ffcc00" })

ui.provide(Theme, { accent = "#00ccff" }, { ui.element(Inventory) })
-- in any component below
local theme = ui.use_context(Theme)
```

## Reloading

Reloading a script keeps its UI and its state. Components are identified by the name given to `ui.component`: defining one again swaps its code and re-renders its instances, signals keep their values. State is matched by the order `ui.signal`, `ui.computed`, `ui.effect` and `ui.watch` are called in a component, so call them unconditionally at the top, like hooks in React.

## Performance comes from the shape of your code

Updates are cheap by construction, they touch only what changed. Building many elements is where scripts cost, because every call into the engine crosses the language boundary. The API gives you three levels to choose from, the [performance page](./performance.md) has the numbers and the rules of thumb.
