# Bundles

Scripts can insert, remove and take [bundles](https://docs.rs/bevy/latest/bevy/ecs/bundle/trait.Bundle.html), i.e. groups of components, in one call. Bundles can be defined in Rust or registered by scripts, both are used through the same functions.

| Function | Behaviour |
|---|---|
| `world.insert_bundle(entity, type, value)` | Inserts the bundle's components into the entity. |
| `world.add_default_bundle(entity, type)` | Inserts a default constructed bundle. |
| `world.spawn_bundle(type, value)` | Spawns a new entity with the bundle and returns it. |
| `world.has_bundle(entity, type)` | Whether the entity has all of the bundle's components. |
| `world.remove_bundle(entity, type)` | Removes the bundle's components, components which are not present are ignored. |
| `world.take_bundle(entity, type)` | Removes the bundle's components and returns them as a bundle value, or `nil` if the entity does not have all of them. |

Bundle types are found with `world.get_type_by_name` or in the `types` table, like components. `type:components()` returns the components making up a bundle, nested bundles are flattened.

## Rust bundles

Bundles defined in Rust need to be registered with the `ReflectBundle` type data:

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

Values of Rust bundles are constructed like any other type, and are inserted all at once:

```lua
world.spawn_bundle(types.EnemyBundle, EnemyBundle.new({ health = Health.new({ hp = 10 }) }))
```

## Script bundles

Scripts can register their own bundles, made up of any components, including script registered components (`world.register_new_component`), and other bundles, which are flattened into their components:

```lua
local Mana = world.register_new_component("script.my_game", "Mana")
local Caster = world.register_new_bundle("script.my_game", "Caster", { Mana, types.EnemyBundle })
```

Registering a component or bundle under a name that is already taken is an error. Both `register_new_component` and `register_new_bundle` take an optional trailing `allow_existing` argument (default `false`); when `true`, the existing registration is returned instead, which is useful when a script may be run more than once (e.g. on reload):

```lua
local Mana = world.register_new_component("script.my_game", "Mana", true)
```

## Using script components like native ones

Values of script components are `DynamicComponent`s, holding their value in `data`:

```lua
local Health = world.register_new_component("script.my_game", "Health")
local entity = world.spawn()
world.insert_component(entity, Health, DynamicComponent.new({ data = { current = 5 } }))

local health = world.get_component(entity, Health)
health.data = { current = health.data.current - 1 }
```

Tables read from `data` are copies: assign a new table to `data` (or to the nested value's parent) to change it, `health.data.current = 1` does not write through.

Script registered components and bundles are also added to the `types` global (e.g. `types.Mana`). Contexts which were created before the registration pick them up before their next callback runs.

Values of script bundles are `DynamicBundle`s, whose `data` maps member names to values. Native components are named by their short type name, script components by their registered name. Members without a value are default constructed:

```lua
local entity = world.spawn_bundle(Caster, DynamicBundle.new({
    data = {
        Mana = 100,
        Health = Health.new({ hp = 5 }),
    }
}))
```

Script components accept their data directly, as above, the same value you would set as `DynamicComponent`'s `data`.

The members of script bundles are inserted one by one, so hooks and observers run once per member rather than once for the whole bundle, unlike for Rust bundles. If any member value cannot be converted, nothing is inserted.
