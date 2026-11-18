# Testing UIs

`ReactiveUiTestPlugin` (the `testing` feature of `bevy_mod_scripting_reactive_ui`) adds the `reactive_ui_test` namespace: tests render a UI, find elements the way a player sees them, fire events at them and check what they show, like React Testing Library.

```rust,ignore
app.add_plugins((ReactiveUiPlugin, ReactiveUiTestPlugin));
```

Events and `act` run everything they change before returning (handlers, bindings, regions, effects, other scripts' work), so a test is a list of steps, without waiting for frames:

```multi-lang-example,lua
local ui = import_namespace("reactive_ui")
local t = import_namespace("reactive_ui_test")

---@param props { start: integer }
local Counter = ui.component("Counter", function(props)
    local count = ui.signal(props.start)
    return ui.node({ flex_direction = "Column" }, {
        ui.node({ name = "inc", on_click = function() count:set(count:get() + 1) end }, { ui.text("+1") }),
        ui.text({ name = "count", text = function() return "count: " .. count:get() end }),
    })
end)

local s = t.render(Counter({ start = 1 }))
s:updates()                              -- forget the first render
s:click(s:get_by_text("+1"))             -- reaches the handler of the node around the text
s:expect_style(s:get_by_name("count"), { text = "count: 2" })

local updates = s:updates()
assert(updates.renders.Counter == nil, "the component ran once")
assert(updates.bindings == 1, "only the text was computed again")

assert(s:snapshot() == [[
node flex_direction="column"
  node name="inc"
    text text="+1"
  text name="count" text="count: 2"
]], s:snapshot())
```

```multi-lang-example,rune
fn counter(props) {
    let ui = import_namespace("reactive_ui");
    let count = ui.signal(props.start);
    ui.node(#{ flex_direction: "Column" }, [
        ui.node(#{ name: "inc", on_click: [|count, _event| count.set(count.peek() + 1), count] }, [ui.text("+1")]),
        ui.text(#{ name: "count", text: ui.format("count: {}", [ui.path(count, "")]) }),
    ])
}

pub fn main() {
    let ui = import_namespace("reactive_ui");
    let t = import_namespace("reactive_ui_test");
    let Counter = ui.component("Counter", counter);

    let s = t.render(Counter.call(#{ start: 1 }));
    s.updates();                              // forget the first render
    s.click(s.get_by_text("+1"));             // reaches the handler of the node around the text
    s.expect_style(s.get_by_name("count"), #{ text: "count: 2" });

    let updates = s.updates();
    assert!(updates.renders.get("Counter") == None, "the component ran once");
    assert!(updates.bindings == 1, "only the text was computed again");

    let snapshot = s.snapshot();
    assert!(snapshot == "node flex_direction=\"column\"\n  node name=\"inc\"\n    text text=\"+1\"\n  text name=\"count\" text=\"count: 2\"\n", "{}", snapshot);
}
```

## Rendering

| | |
|---|---|
| `t.render(element, contexts)` | Mounts any element under a root of its own filling the window, and returns its `Screen`. `contexts` is a list of `t.context(Context, value)`, provided as `ui.provide` does |
| `t.screen(root)` | The `Screen` of a UI mounted otherwise, i.e. the root `ui.render` returned, once the work queued so far ran |

## Finding elements

Queries search the elements below the screen's root, breadth first. Give elements a `name` to find them by it.

| | none | one | more |
|---|---|---|---|
| `s:get_by_name(name)`, `s:get_by_text(text)` | error | the element | error |
| `s:query_by_name(name)`, `s:query_by_text(text)` | `nil` | the element | error |
| `s:all_by_name(name)`, `s:all_by_text(text)` | `{}` | `{ element }` | all of them |

`s:within(element)` is the screen of what is below an element, `s:children(element)` its children in order, `s:text(element)` the text of a text element.

## Events and changes

| | |
|---|---|
| `s:click(element)` | Clicks the element: its handler, or its nearest ancestor's, runs, then what it changed |
| `s:interact(element, event)` | Fires `"press"`, `"release"`, `"pointer_enter"`, `"pointer_leave"`, `"hover"`, `"drag"` or `"scroll"` the same way |
| `s:act(f)` | Calls `f`, then runs what it changed: for changes other than events, i.e. setting a signal or a watched component |
| `s:dispatch(element, event)` | Queues the event for the next UI update without running it: for what happens across updates, or benchmarks timing the update |

An error in a handler, a binding or an effect fails the call which ran it.

## Checking what is shown

| | |
|---|---|
| `s:expect_style(element, props)` | Checks the properties given, written as when setting them: `{ width = "50%", background_color = "#ffcc00" }`. Fails with every property which differs: ``` `panel`: max_width: expected "50%", got "40%" ``` |
| `s:style(element)` | The properties an element has, as a table in the same form |
| `s:rect(element)` | Where the element is once laid out: `{ x, y, width, height }` in logical pixels from the top left. Lays the UI out first, in a 1280x720 window when the app has no camera. Text isn't measured |
| `s:snapshot(element)` | The tree below the element (all of the screen if `nil`) as text: one line per element, indented, with its kind and the properties which aren't the defaults, sorted. The same UI always gives the same text, so it can be compared with a string kept in the test |
| `s:updates()` | What ran since the screen was made or this was last called: `renders` of each component by name, and how many `bindings`, `regions` and `effects` ran again. For checking that a change only updates what it should |

Properties are compared in one form whichever way they were written: `"#FFCC00"` and `{ 1, 0.8, 0 }` are the same colour, `{ 4, 8 }` and `{ top = 4, bottom = 4, left = 8, right = 8 }` the same padding.

## In the test harness

BMS' own script tests (`assets/tests/reactive_ui/`) have the plugin, see `inventory/inventory.lua` for a longer test, including what survives a reload.
