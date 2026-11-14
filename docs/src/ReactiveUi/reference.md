# Reactive UI Reference

Everything is a member of the `reactive_ui` namespace: `local ui = import_namespace("reactive_ui")`.

## Elements

| | |
|---|---|
| `ui.node(props, children)` | A container. `children` is a list of elements |
| `ui.text(props)` | Text. `ui.text("hello")` is `ui.text({ text = "hello" })` |
| `ui.image(props)` | A node showing the image asset at the path `image` |
| `ui.element(Component, props)` | An instance of a component, `props` is given to its function. `props.key` identifies it among its siblings |
| `ui.show(f, deps)` | A region showing what `f` returns (an element, a list of them, or nothing). Re-runs when what `f` read changes |
| `ui.list(items, f, deps)` | One child per item of the list in the signal `items`, made by `f(item)`. Children are kept by item |
| `ui.list_template(items, f, deps)` | As `ui.list`, but `f` runs once, see [row templates](#row-templates) |
| `ui.provide(context, value, children)` | A node providing a context value to what is below it |

Elements are only valid during the update which built them: build them where they are returned, don't store them.

`ui.show`, `ui.list` and `ui.list_template` are nodes holding their children. Their last argument is the properties of that node, which is where the layout of a list goes:

```lua
ui.list(items, function(item) .. end, nil, { flex_direction = "row", flex_wrap = "wrap", column_gap = 6 })
```

## Properties

A property is a plain value, a function (re-run when what it read changes), or a [value computed in Rust](#values-computed-in-rust). `nil` resets it.

| property | values |
|---|---|
| `width`, `height`, `min_width`, `max_width`, `min_height`, `max_height`, `left`, `right`, `top`, `bottom`, `flex_basis`, `row_gap`, `column_gap` | a number of pixels, `"50%"`, `"10px"`, `"5vw"`, `"5vh"`, `"5vmin"`, `"5vmax"`, `"auto"` |
| `margin`, `padding`, `border` | one size, `{ vertical, horizontal }`, `{ top, right, bottom, left }`, or `{ left = .., top = .. }` |
| `border_radius` | one size, or `{ top_left, top_right, bottom_right, bottom_left }` |
| `display` | `"flex"`, `"grid"`, `"block"`, `"none"`, or `false` to hide |
| `position_type` | `"relative"`, `"absolute"` |
| `flex_direction`, `flex_wrap`, `flex_grow`, `flex_shrink` | as in CSS: `"row"`, `"column_reverse"`, `"wrap"`, numbers |
| `align_items`, `align_self`, `align_content`, `justify_items`, `justify_self`, `justify_content` | the variant's name: `"center"`, `"flex_start"`, `"space_between"`, .. |
| `overflow` | `"visible"`, `"clip"`, `"hidden"`, `"scroll"`, or `{ x = .., y = .. }` |
| `aspect_ratio`, `box_sizing`, `direction`, `grid_auto_flow` | as the `Node` field |
| `background_color`, `border_color` | `"#rrggbb"`, `"#rrggbbaa"`, or `{ red, green, blue, alpha }` from 0 to 1 |
| `z_index` | a whole number |
| `text` | the text of a `ui.text` |
| `font_size`, `color` | text size in pixels and colour |
| `image` | the path of an image asset |
| `name` | the entity's `Name`, to find it with `ui.find` or from Rust |
| `key` | identifies the element among its siblings when a region re-runs |

A wrong value is an error naming the property and listing what it accepts.

## Events

| property | called with |
|---|---|
| `on_click` | `{ x, y, button, count }` |
| `on_press`, `on_release` | `{ x, y, button }` |
| `on_pointer_enter`, `on_pointer_leave` | `{ x, y }` |
| `on_hover` | `true` when the pointer enters, `false` when it leaves |
| `on_drag` | `{ x, y, button, delta_x, delta_y }` |
| `on_scroll` | `{ x, y, delta_x, delta_y }` |

An element whose `overflow` is `"scroll"` scrolls with the mouse wheel, without a handler: give it a size and let its children overflow.

These are `bevy_picking`'s pointer events. The innermost element with a handler for an event gets it. Handlers run during the next UI update, where they can set signals and change the world.

## State

| | |
|---|---|
| `ui.signal(value)` | State. `s:get()` reads it and records the read, `s:peek()` reads without recording, `s:set(value)` changes it |
| `ui.computed(f, deps)` | A value derived from others, recomputed when what `f` read changed. Its readers only update when the value did |
| `ui.effect(f, deps)` | Runs `f` now and whenever what it read changes. `f` can return a function to clean up before the next run |
| `ui.watch(entity, Component)` | A signal over a component of an entity: `:get()` returns the component, or `nil` when it is gone |
| `ui.selector(signal)` | For "which one is selected": `sel:selected(key)` only wakes the readers of the key which was and the key which is the signal's value |
| `ui.create_context(default)`, `ui.provide`, `ui.use_context(context)` | A value provided to a subtree. Read when a component renders, provide a signal for a value which changes |

Inside a component these belong to the instance: call them at the top, in the same order every time.

## Values computed in Rust

Used as properties, these update without running script code. They take signals' fields, other computed values and plain values.

| | |
|---|---|
| `ui.path(source, "a.b")` | A field of the value of a signal, computed or watch. `""` is the value itself. Struct fields by name, list items from 0 |
| `ui.format("{} / {}", { a, b })` | Text, each `{}` replaced by the next value |
| `ui.percent(part, whole)` | A size in percent |
| `ui.lt(a, b)`, `ui.mul(a, b)` | Less than, times |
| `ui.when(condition, if_true, if_false)` | One of two values |
| `ui.selected(selector, key)` | Whether `key` is selected |

## Row templates

`ui.list_template(items, f, deps)` calls `f(item)` once, with `item` standing for the item of every row, and builds each row in Rust from the elements `f` returned.

```lua
ui.list_template(entities, function(item)
    local data = ui.watch_item(item, Item)                 -- a component of each row's entity
    local name = ui.path(data, "data.name")
    return ui.node({
        name = ui.format("slot_{}", { name }),
        background_color = ui.when(ui.selected(is_selected, name), "#ffcc00", "#333333"),
        on_click = function(entity, event) select(entity) end,   -- shared by all rows, given the row's item
    }, {
        ui.text({ text = ui.format("x{}", { ui.path(data, "data.count") }) }),
    })
end)
```

- rows are made of `ui.node`, `ui.text` and `ui.image`
- properties are plain values or computed in Rust, `item` can be used wherever a value is taken
- `ui.watch_item` needs the items to be entities
- functions get the row's item as an argument, after their dependencies

## Dependencies

Wherever a function is taken, a list of dependencies can follow: the function is called with those values first.

```lua
local double = ui.computed(function(n) return n:get() * 2 end, { n })
ui.node({ on_click = { function(selected, name, event) selected:set(name) end, selected, name } })
ui.element(Slot, slot, { name, entity })      -- renders the component with slot(name, entity)
```

A handler or bound property with dependencies is a list starting with the function. In Lua dependencies are optional, functions can capture what they need (which is [cheaper](./performance.md#captures-and-dependencies)). In Rune they are required for signals and entities, see below.

## Mounting and testing

| | |
|---|---|
| `ui.component(name, f)` | Defines a component. Defining the name again (a reload) swaps the code and re-renders its instances |
| `ui.render(key, Component, props)` | Shows a component under a new root entity filling the window, returned. Calling it again with the key updates it. Call it from `on_script_loaded` |
| `ui.find(root, name)` | The entity with the `Name` below `root` |
| `ui.describe(entity)` | What an element shows: its text, colours, layout |
| `ui.interact(entity, "click")` | What a pointer event on the element would do |

The UI of a script is removed when the script is unloaded.

## Rune

The API is the same, with the differences Rune scripts have everywhere (`pub fn main()`, `state()` instead of globals, 0 based indexes) and these:

- Closures handed to BMS can only capture numbers and strings. Give functions the signals and entities they need as [dependencies](#dependencies).
- Keep component definitions in `state()` and define them once in `main`: `ui.component` re-renders the instances of a component it defines again.
- Arguments are counted: a handler declares the event it is given, `|selected, name, _event| selected.set(name)`.
- A closure ending in a bare local returns nothing, write `return out;`. `if` without `else` is `()`. A string can't be compared with `()`, use `match`.

```rust,ignore
fn counter(props) {
    let ui = import_namespace("reactive_ui");
    let count = ui.signal(props.start);
    ui.node(#{ flex_direction: "row" }, [
        ui.node(#{ on_click: [|count, _event| count.set(count.peek() + 1), count] }, [ui.text("+1")]),
        ui.text(#{ text: ui.format("count: {}", [ui.path(count, "")]) }),
    ])
}
```
