# Bundles

Scripts can insert [bundles](https://docs.rs/bevy/latest/bevy/ecs/bundle/trait.Bundle.html) defined in Rust, i.e. groups of components, all at once:

| Function | Behaviour |
|---|---|
| `world.insert_bundle(entity, type, value)` | Inserts the bundle's components into the entity. |
| `world.spawn_bundle(type, value)` | Spawns a new entity with the bundle and returns it. |

Bundle types are found with `world.get_type_by_name` or `import`, like components.

Bundles need to be registered with the `ReflectBundle` type data:

```rust,ignore
#[derive(Bundle, Reflect, Default)]
#[reflect(Bundle, Default)]
struct EnemyBundle {
    health: Health,
    transform: Transform,
}

app.register_type::<EnemyBundle>();
```

Tuples and bundles from other crates can be registered with `app.register_type_data::<(Health, Transform), ReflectBundle>()`.

Values of bundles are constructed like any other type:

```lua
world.spawn_bundle(types.EnemyBundle, EnemyBundle.new({ health = Health.new({ hp = 10 }) }))
```

## Script components

Registering a component under a name that is already taken is an error. `register_new_component` takes an optional trailing `allow_existing` argument (default `false`); when `true`, the existing registration is returned instead, which is useful when a script may be run more than once (e.g. on reload):

```lua
local Mana = world.register_new_component("script.my_game", "Mana", true)
```

Values of script components are `DynamicComponent`s, holding their value in `data`:

```lua
local Health = world.register_new_component("script.my_game", "Health")
local entity = world.spawn()
world.insert_component(entity, Health, DynamicComponent.new({ data = { current = 5 } }))

local health = world.get_component(entity, Health)
health.data = { current = health.data.current - 1 }
```

Tables read from `data` are copies: assign a new table to `data` to change it, `health.data.current = 1` does not write through.

Script registered components are also added to the `types` global (e.g. `types.Mana`). Contexts which were created before the registration pick them up before their next callback runs.
