## Entity spawning

Assuming you created a component and registered it using the following rust snippets.

```rust,ignore
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MyType {
    value: f32,
}
```

```rust,ignore
app.register_type::<MyType>();
```

In lua you can construct a MyType instance and attach it to an entity.

```lua
local MyType = types.MyType

function on_script_loaded()
    local instance = construct(MyType, { value = 50.0 })
    local entity = world.spawn()
    world.insert_component(entity, MyType, instance)
    print("Entity with component value: " .. world.get_component(entity, MyType).value)
end
```
