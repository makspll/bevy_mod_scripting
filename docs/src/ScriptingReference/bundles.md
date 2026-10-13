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

`type:components()` returns the components making up a bundle, nested bundles are flattened.

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
local EnemyBundle = world.get_type_by_name("EnemyBundle")
world.spawn_bundle(EnemyBundle, construct(EnemyBundle, { health = construct(Health, { hp = 10 }) }))
```

## Script bundles

Scripts can register their own bundles, made up of any components, including script registered components (`world.register_new_component`), and other bundles, which are flattened into their components:

```lua
local Mana = world.register_new_component("Mana")
local Caster = world.register_new_bundle("Caster", { Mana, world.get_type_by_name("EnemyBundle") })
```

Values of script bundles are `DynamicBundle`s, whose `data` maps member names to values. Native components are named by their short type name, script components by their registered name. Members without a value are default constructed:

```lua
local entity = world.spawn_bundle(Caster, DynamicBundle.new({
    data = {
        Mana = 100,
        Health = construct(Health, { hp = 5 }),
    }
}))
```

Script components accept their data directly, as above, the same value you would set as `DynamicComponent`'s `data`.

The members of script bundles are inserted one by one, so hooks and observers run once per member rather than once for the whole bundle, unlike for Rust bundles. If any member value cannot be converted, nothing is inserted.
