# Namespaces

Every type known to scripts lives in a namespace, and scripts reach types through their namespace: there are no globals named after types.

| Types | Namespace |
|---|---|
| bevy types covered by the bevy bindings, grouped by category | `math`, `ecs`, `time`, `transform`, `color`, `input`, `window`, `asset`, `camera`, `render`, `sprite`, `text`, `ui`, `gizmos`, `animation`, `audio`, `state` |
| BMS' own types (`ScriptAttachment`, `ReflectReference`, ...) | `bms` |
| any other type | the name of the crate defining it, i.e. `my_game` |
| primitives like `i32` | the root namespace, `""` |

A category namespace holds every type of the bevy crates it covers, i.e. `math` holds the types of `glam` and `bevy_math`, `render` those of `bevy_render`, `bevy_pbr`, `bevy_light`, `bevy_mesh` and `bevy_image`.

`Type:namespace()` returns a type's namespace. Script registered components and events live in `script.*` namespaces, see below.

## Importing a namespace

`import_namespace(namespace)` returns a table of the namespace's types (and, for script namespaces, events and exported functions) by short name. Each type can be used to call its static functions, to access its enum variants, and wherever a type is expected:

```lua
local bms = import_namespace("bms")
local world = bms.world
local Vec3 = import_namespace("math").Vec3
local transform = import_namespace("transform")

local t = transform.Transform.from_translation(Vec3.new(0, 1, 0))
world.add_default_component(bms.current_entity(), transform.Transform)
```

Import once at the top of a script, and keep the types you use in locals. In Lua, don't name the table of the `math` namespace `math`: that would hide Lua's own `math` library.

Besides their own static functions, types have the functions of their kind, unless they have a function of the same name:

| Kind | Functions |
|---|---|
| any type | `Type.new{...}`, see [Constructing Arbitrary Types](./constructing-arbitrary-types.md) |
| component | `Type.get_on(entity)`, `Type.add_default_on(entity)`, `Type.is_on(entity)`, `Type.remove_from(entity)` |
| resource | `Type.fetch()`, `Type.exists()`, `Type.remove_resource()` |
| bundle or component | `Type.insert_on(entity, value)`, `Type.spawn_with(value)` |
| asset | `Type.load(path)`, `Type.add_asset(value)`, `Type.get_asset(handle)` |

```lua
local Transform = import_namespace("transform").Transform
local transform = Transform.get_on(entity) -- a `Transform` or nil
```

* If two types in a namespace share a short name, the first one registered wins.
* Each call reads the shared type catalogue, so the result is always up to date. A table imported earlier does not pick up later registrations: import again.
* A namespace which does not exist is an error, with suggestions: `namespace "transfrom" does not exist. ... Did you mean: transform?`
* To also expose every type as a global named after it (i.e. `Vec3`), as before 0.24, set `CoreScriptGlobalsPlugin::register_static_references`. Types sharing a name then shadow each other.

## Overriding the namespace of a type

```rust,ignore
// on types with script bindings: keeps a namespace set before (with a warning)
#[script_bindings(namespace = "my_namespace")]
impl MyType { /* ... */ }

// on any type: always wins, before or after `script_bindings`
app.override_script_namespace::<MyType>("my_namespace"); // bevy_mod_scripting::bindings::ScriptNamespaceAppExt
// or directly
registry.register_type_data_with::<MyType, ScriptNamespace, _>("my_namespace");
```

## Namespaces defined by scripts

Scripts register components and events into namespaces they pass explicitly. These start with `script.`, followed by dot-separated segments of letters, digits and underscores:

```lua
local bms = import_namespace("bms")
local world = bms.world
local Enemy = world.register_new_component("script.game.ai", "Enemy")
local Hit = world.register_new_event("script.game.ai", "Hit")

-- anywhere, including other scripts
local ai = import_namespace("script.game.ai")
world.add_default_component(bms.current_entity(), ai.Enemy)
world.trigger_event(ai.Hit, { damage = 3 })
```

* Names are unique within a namespace, across components, events and exports: `script.game.ai.Enemy` and `script.game.ui.Enemy` are different components.
* Registering a taken name is an error. `allow_existing` returns the existing registration instead, which is useful for scripts which run again.
* A script namespace exists once something is registered or exported into it.
* Components and events stay registered when their script unloads, as Bevy cannot unregister components.

Values of script components are `DynamicComponent`s, holding any value in `data`. Tables read from `data` are copies: assign a new table to change them.

```lua
local bms = import_namespace("bms")
local world = bms.world
local DynamicComponent = bms.DynamicComponent
world.insert_component(bms.current_entity(), Enemy, DynamicComponent.new({ data = { hp = 5 } }))
local enemy = world.get_component(bms.current_entity(), Enemy)
enemy.data = { hp = enemy.data.hp - 1 } -- `enemy.data.hp = 4` would not write through
```

## Exporting functions

`export_namespace(namespace, name, callback)` makes a callback of the calling script callable by other scripts, in any language:

```lua
-- script A
local world = import_namespace("bms").world
register_callback("spawn_enemy_impl", function(health)
    return world.spawn()
end)
export_namespace("script.game", "spawn_enemy", "spawn_enemy_impl")

-- script B, once script A has run
local enemy = import_namespace("script.game").spawn_enemy(5)
```

* `callback` is a function registered with `register_callback`, or a top level function of that name. Only functions can be exported.
* Calls run in the exporting script's context, like `bms.run_script_callback` (see [Calling Other Scripts](./calling-other-scripts.md)), and return the callback's result.
* A script cannot call its own exports, or those of scripts sharing its context, from one of its callbacks: its context is busy. Call the function directly instead.
* Exports are looked up on every call, so replaced exports are picked up and removed ones fail with an error.
* A path can only be exported by one script. Exporting it again from the same script replaces it. Exports can't share a path with a component.
* `export_namespace` only works inside a script callback.

When a script unloads or is about to reload, its exports and observers are removed right after its `on_script_unloaded` callback. Export from a callback which runs on load (e.g. `on_script_loaded`), so a reloaded script exports again.

## Editor support

LAD files record each type's namespace. The Lua language server definitions declare a `namespace.<name>` class per native namespace, and `import_namespace("transform")` returns the matching class, so completion works on imported tables. Script namespaces only exist at runtime and are untyped.

The functions of a type's kind are typed for that type: `Transform.get_on(entity)` is a `Transform` or nil. The `world` functions taking registrations, i.e. `world.get_component(entity, Transform)`, give untyped values.
