# Performance

Two things cost time in a scripted UI, and they are very different:

- **Updating** what is on screen is proportional to what changed. Changing one signal re-runs the few functions which read it and writes the properties they produce.
- **Building** elements is proportional to how many you build, and each one crosses from the script into the engine several times: creating it, converting its properties, running its component.

So the question to ask of a piece of UI is not "is this fast" but "how often is it built, and how many are there".

The numbers below are for an inventory of 200 slots (a node with a background, a name, a click handler and a text each), measured on one machine. They include what the UI does up to handing the tree to `bevy_ui`, not layout or rendering. Use them to compare the options with each other.

## Three ways to write a property

```lua
-- 1. a function: flexible, runs in the script every time it updates
text = function() return "x" .. item:get().data.count end

-- 2. a value computed in Rust: no script code runs when it updates
text = ui.format("x{}", { ui.path(item, "data.count") })

-- 3. a plain value: never updates
text = "x1"
```

| one slot's count changes | Lua |
|---|---|
| function | 7.7 µs |
| computed in Rust | about 4 µs |

Functions are fine for the handful of values which change now and then. For what updates every frame (health bars, timers, cooldowns) or exists hundreds of times, reach for `ui.path`, `ui.format`, `ui.percent`, `ui.when` and `ui.selected`.

## Three ways to build a list

```lua
-- 1. a region: the function runs again in full whenever the list changes
ui.show(function()
    local out = {}
    for _, item in ipairs(items:get()) do out[#out + 1] = ui.element(Slot, { key = item, item = item }) end
    return out
end)

-- 2. a list: the function only runs for items which were not in the list before
ui.list(items, function(item) return ui.element(Slot, { item = item }) end)

-- 3. a row template: the function runs once for the whole list
ui.list_template(items, function(item)
    local data = ui.watch_item(item, Item)
    return ui.node({ background_color = "#333333" }, {
        ui.text({ text = ui.format("x{}", { ui.path(data, "data.count") }) }),
    })
end)
```

| 200 slots | region | list | row template |
|---|---|---|---|
| one item added or removed (Lua) | 686 µs | 109 µs | 173 µs |
| all 200 mounted or unmounted (Lua) | 2.0 ms | 2.0 ms | 0.90 ms |
| all 200 mounted or unmounted (Rune) | | 4.4 ms | 0.83 ms |

- Use **`ui.list`** for anything keyed. It costs nothing extra when everything changes and is six times cheaper when little does.
- Use **`ui.list_template`** for long, uniform lists: inventories, leaderboards, logs. No script code runs per row, so it costs the same in every language and about half of the alternatives in Lua. The price is that the row is described with values computed in Rust: no branching on the item in script code, no components inside the row.
- `ui.show` is for content which appears and disappears, not for lists.

For scale: a frame at 60 fps is 16.7 ms, so mounting 200 slots takes between a twentieth and a quarter of one frame depending on the option, once, when the inventory opens.

## Captures and dependencies

A function can capture what it needs, or be given it as [dependencies](./reference.md#dependencies). In Lua captures are free, while every dependency is converted on every call:

| 200 slots (Lua) | captures | dependencies |
|---|---|---|
| all 200 mounted or unmounted | 1.8 ms | 2.4 ms |
| a click selecting a slot | 23 µs | 24 µs |

Capture where the language lets you. In Rune, where closures can't capture signals or entities, dependencies are the way to write components, and row templates are worth it sooner.

## Rules of thumb

1. Give components the state they own, and nothing a parent has to re-create them for. Components run once: what changes belongs in signals, not in props.
2. Bind the property which changes, not the element. A region replacing an element to change its colour does the work of building it again.
3. Many instances of the same thing are a list. Hundreds are a row template.
4. Read ECS data through `ui.watch` and `ui.path`. A handler reading a component with `world.get_component` is fine, a function doing it for every row is not.
5. Signals only wake what read them. `ui.selector` makes "is this row the selected one" wake two rows instead of all of them.
6. Don't keep whole lists in signals which change often: the list crosses the boundary each time it is read. Keep the keys in the list and the data in components.
