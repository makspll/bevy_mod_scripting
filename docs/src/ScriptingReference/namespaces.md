# Namespaces

Every type known to scripts lives in a namespace, by default the name of the crate defining it:

| Type | Namespace |
|---|---|
| `Transform` | `bevy_transform` |
| `Entity` | `bevy_ecs` |
| `Vec3` | `glam` |
| `Option` / `Vec` | `core` / `alloc` |
| primitives like `i32` | the root namespace, `""` |
| BMS' own types (`ScriptAttachment`, `ReflectReference`, ...) | `bms` |

`Type:namespace()` returns a type's namespace. Script registered components live in `script.*` namespaces, see below.

> **Deprecated:** the flat `types` global. It can't tell apart types of the same name in different namespaces, and it is a snapshot taken when the script's context is created. Use `import_namespace` instead.

## Importing a namespace

`import_namespace(namespace)` returns a table of the namespace's types (and, for script namespaces, exported functions) by short name:

```lua
local bevy_transform = import_namespace("bevy_transform")
local t = bevy_transform.Transform.new { translation = Vec3.new(0, 1, 0) }
```

* If two types in a namespace share a short name, the first one registered wins.
* Each call reads the shared type catalogue, so the result is always up to date. A table imported earlier does not pick up later registrations: import again.
* A namespace which does not exist is an error, with suggestions: `namespace "bevy_transfrom" does not exist. ... Did you mean: bevy_transform?`

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

Scripts register components into namespaces they pass explicitly. These start with `script.`, followed by dot-separated segments of letters, digits and underscores:

```lua
local Enemy = world.register_new_component("script.game.ai", "Enemy")

-- anywhere, including other scripts
local ai = import_namespace("script.game.ai")
world.add_default_component(entity, ai.Enemy)
```

* Names are unique within a namespace, across components and exports: `script.game.ai.Enemy` and `script.game.ui.Enemy` are different components.
* Registering a taken name is an error. `allow_existing` returns the existing registration instead, which is useful for scripts which run again.
* A script namespace exists once something is registered or exported into it.
* Components stay registered when their script unloads, as Bevy cannot unregister components.

Values of script components are `DynamicComponent`s, holding any value in `data`. Tables read from `data` are copies: assign a new table to change them.

```lua
world.insert_component(entity, Enemy, DynamicComponent.new({ data = { hp = 5 } }))
local enemy = world.get_component(entity, Enemy)
enemy.data = { hp = enemy.data.hp - 1 } -- `enemy.data.hp = 4` would not write through
```

## Exporting functions

`export_namespace(namespace, name, callback)` makes a callback of the calling script callable by other scripts, in any language:

```lua
-- script A
register_callback("spawn_enemy_impl", function(health)
    return world.spawn()
end)
export_namespace("script.game", "spawn_enemy", "spawn_enemy_impl")

-- script B, once script A has run
local enemy = import_namespace("script.game").spawn_enemy(5)
```

* `callback` is a function registered with `register_callback`, or a top level function of that name. Only functions can be exported.
* Calls run in the exporting script's context, like `run_script_callback` (see [Calling Other Scripts](./calling-other-scripts.md)), and return the callback's result.
* A script cannot call its own exports, or those of scripts sharing its context, from one of its callbacks: its context is busy. Call the function directly instead.
* Exports are looked up on every call, so replaced exports are picked up and removed ones fail with an error.
* A path can only be exported by one script. Exporting it again from the same script replaces it. Exports can't share a path with a component.
* `export_namespace` only works inside a script callback.

When a script unloads or is about to reload, its exports are removed right after its `on_script_unloaded` callback. Export from a callback which runs on load (e.g. `on_script_loaded`), so a reloaded script exports again.

## Editor support

LAD files record each type's namespace. The Lua language server definitions declare a `namespace.<name>` class per native namespace, and `import_namespace("bevy_transform")` returns the matching class, so completion works on imported tables. Script namespaces only exist at runtime and are untyped.
