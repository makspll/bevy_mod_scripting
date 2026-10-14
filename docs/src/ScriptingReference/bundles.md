# Bundles

Scripts can insert [bundles](https://docs.rs/bevy/latest/bevy/ecs/bundle/trait.Bundle.html) defined in Rust, all components at once:

| Function | Behaviour |
|---|---|
| `world.insert_bundle(entity, type, value)` | Inserts the bundle's components into the entity. |
| `world.spawn_bundle(type, value)` | Spawns an entity with the bundle and returns it. |

Bundles need `ReflectBundle` type data, and are constructed like any other type:

```rust,ignore
#[derive(Bundle, Reflect, Default)]
#[reflect(Bundle, Default)]
struct EnemyBundle {
    health: Health,
    transform: Transform,
}

app.register_type::<EnemyBundle>();
// tuples and foreign bundles: app.register_type_data::<(Health, Transform), ReflectBundle>()
```

```lua
local EnemyBundle = world.get_type_by_name("EnemyBundle")
world.spawn_bundle(EnemyBundle, EnemyBundle.new({ health = Health.new({ hp = 10 }) }))
```
