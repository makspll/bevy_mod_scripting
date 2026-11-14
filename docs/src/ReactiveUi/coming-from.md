# Coming From Another Framework

The reactive UI borrows its ideas from frameworks you may know. Find yours below for what carries over and what will surprise you.

## React

Components, props, keys, context and effects are what you expect. The one big difference: **a component function runs once, not on every state change**. There is no re-render to optimise away.

| React | here |
|---|---|
| `function Counter(props) { .. }` | `ui.component("Counter", function(props) .. end)` |
| `<Counter start={0} />` | `ui.element(Counter, { start = 0 })` |
| `createRoot(el).render(<App />)` | `ui.render("app", App, props)` |
| `const [n, setN] = useState(0)` | `local n = ui.signal(0)`, `n:get()`, `n:set(1)` |
| `useMemo(() => .., [a, b])` | `ui.computed(function() .. end)` |
| `useEffect(() => { ..; return cleanup }, [a])` | `ui.effect(function() ..; return cleanup end)` |
| `{cond && <Details />}` | `ui.show(function() if cond:get() then return ui.element(Details) end end)` |
| `items.map(item => <Row key={item} />)` | `ui.list(items, function(item) return ui.element(Row, { item = item }) end)` |
| `createContext`, `<Ctx.Provider value>`, `useContext` | `ui.create_context`, `ui.provide`, `ui.use_context` |
| `onClick={e => ..}` | `on_click = function(event) .. end` |
| `style={{ flexDirection: "row" }}` | `flex_direction = "row"`, directly in the props |

```lua
local Counter = ui.component("Counter", function(props)
    local n = ui.signal(props.start)
    -- this line runs once. To show a changing value, bind the property to a function:
    return ui.text({ text = function() return "n = " .. n:get() end })
end)
```

- **No dependency arrays.** What a function reads while it runs is what it depends on. (The optional lists some functions take are for [passing values in](./reference.md#dependencies), not for tracking.)
- **Reading state is a call**, `n:get()`. A value read outside of a bound function, computed or effect is read once.
- **The rules of hooks still apply** to `ui.signal`, `ui.computed`, `ui.effect` and `ui.watch`: call them at the top of the component, unconditionally. This is how state survives a script reload.
- **Props don't flow.** A component keeps the props it was created with: pass a signal down when the child has to follow a value.

## Solid, Preact Signals, Vue, Svelte

This is your model: fine grained signals, components which run once, bindings instead of a virtual DOM.

| | here |
|---|---|
| `createSignal` / `signal` / `ref` | `ui.signal` |
| `createMemo` / `computed` | `ui.computed` |
| `createEffect` / `effect` / `watchEffect` | `ui.effect` |
| `<Show when>` / `v-if` / `{#if}` | `ui.show` |
| `<For each>` / `v-for` / `{#each}` | `ui.list` |
| `createSelector` | `ui.selector`, `selector:selected(key)` |
| `untrack` | `s:peek()` |
| context | `ui.create_context`, `ui.provide`, `ui.use_context` |

- There are no templates or JSX to compile, so reactivity is explicit: a property is reactive when you give it a function (or a value computed in Rust), not when it merely mentions a signal.
- `ui.watch(entity, Component)` is a signal whose source is the ECS. It is the equivalent of a store fed by the server.
- Two additions exist for the sake of the language boundary, see [performance](./performance.md): values computed in Rust (`ui.path`, `ui.format`, `ui.when`) and row templates (`ui.list_template`).

## Immediate mode (egui, Dear ImGui)

Here the UI is **retained**: you describe it once and it stays, instead of being rebuilt every frame.

```lua
-- immediate mode: runs every frame, reads the state as it is
-- if ui.button("+1") then count = count + 1 end
-- ui.label("count: " .. count)

-- retained and reactive: runs once, the parts depending on state say so
ui.node({ on_click = function() count:set(count:get() + 1) end }, { ui.text("+1") }),
ui.text({ text = function() return "count: " .. count:get() end }),
```

- State lives in signals, not in plain variables: changing a plain variable changes nothing on screen.
- Events are handlers on elements, not return values of the call which drew them.
- Nothing runs on frames where nothing changed, which is what makes a scripted UI affordable.

## Bevy UI in Rust

It is the same `bevy_ui` underneath: every element is an entity with a `Node`, and properties are the fields of `Node` with the names you know.

| Rust | here |
|---|---|
| `Node { flex_direction: FlexDirection::Row, .. }` | `flex_direction = "row"` |
| `Val::Px(10.)`, `Val::Percent(50.)`, `Val::Auto` | `10`, `"50%"`, `"auto"` |
| `UiRect::all(Val::Px(4.))` | `4`, or `{ 4, 8 }`, `{ left = 4 }` |
| `BackgroundColor`, `BorderColor`, `TextColor`, `TextFont` | `background_color`, `border_color`, `color`, `font_size` |
| `children![..]` | the second argument of `ui.node` |
| observers on `Pointer` events | `on_click`, `on_drag`, .. |
| a system with `Changed<Health>` updating a `Text` | `ui.watch(entity, Health)` and a bound property |

- You don't spawn, despawn or mutate UI entities, the tree is derived from state. The entities are ordinary ones though: Rust systems can query them, and `Name`s set with `name = ..` make them easy to find.
- `ui.watch` is how the UI observes components, including ones only Rust code writes to.
