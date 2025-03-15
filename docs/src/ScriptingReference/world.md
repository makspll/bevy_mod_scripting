## World

The `World` is the entry point for interacting with `Bevy`. It is provided to scripts under either the `world` or `World` static variable. 

### get_type_by_name

Returns either a `ScriptComponentRegistration` or `ScriptResourceRegistration` depending on the type of the type requested. If the type is neither returns a `ScriptTypeRegistration`.

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `type_name` | `String` | The name of the type to get, this can be either the short type name, i.e. `my_type` or the long name i.e. `my_crate::my_module::my_type` |

Returns:

| Return | Description |
| ---  | --- |
| `Option<ScriptTypeRegistration OR scriptComponentRegistration OR scriptResourceRegistration>` | The registration for the type if it exists, otherwise `None` |

```lua
MyType = world.get_type_by_name("MyType")
if MyType == nil then
    print("MyType not found")
end

-- OR
MyType = types.MyType
```

### get_component

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to get the component from |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the component |

Returns:

| Return | Description |
| ---  | --- |
| `Option<ReflectReference>` | The reference to the component if it exists, otherwise `None` |

```lua
local component = world.get_component(entity, MyType)
if component ~= nil then
    print("found component:" .. component)
end
```

### has_component

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to check the component for |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the component |

Returns:

| Return | Description |
| ---  | --- |
| `bool` | `true` if the entity has the component, otherwise `false` |

```lua
if world.has_component(entity, MyType) then
    print("Entity has MyType")
end
```

### remove_component

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to remove the component from |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the component |

```lua
world.remove_component(entity, MyType)
```

### get_resource

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the resource |

Returns:

| Return | Description |
| ---  | --- |
| `Option<ReflectReference>` | The resource if it exists, otherwise `None` |

```lua
local resource = world.get_resource(MyType)
if resource ~= nil then
    print("found resource:" .. resource)
end
```

### has_resource

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the resource |

Returns:

| Return | Description |
| ---  | --- |
| `bool` | `true` if the resource exists, otherwise `false` |

```lua
local hasResource = world.has_resource(MyType)
```

### remove_resource

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the resource |

```lua
world.remove_resource(MyType)
```

### add_default_component

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to add the component to |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the component |

```lua
world.add_default_component(entity, MyType)
```

### insert_component

Inserts or applies the given value to the component of the entity. If the component does not exist it will be added.

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to add the component to |
| `registration` | `ScriptTypeRegistration` | The type registration as returned by `get_type_by_name` of the component |
| `component` | `ReflectReference` | A reference to an existing component value to be inserted |

```lua
local existingComponent = world.get_component(otherEntity, MyType)
world.insert_component(entity, MyType, existingComponent)
```


### spawn

Returns:

| Return | Description |
| ---  | --- |
| `Entity` | The spawned entity |

```lua
local entity = world.spawn()
```

### insert_children

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The parent entity |
| `index` | `usize` | The index to insert the children at |
| `children` | `Vec<Entity>` | The children entities to insert |

```lua
world.insert_children(parent, 1, {child1, child2})
```

### push_children

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The parent entity |
| `children` | `Vec<Entity>` | The children entities to push |


```lua
world.push_children(parent, {child1, child2})
```

### get_children

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The parent entity |

Returns:

| Return | Description |
| ---  | --- |
| `Vec<Entity>` | The children entities |

```lua
local children = world.get_children(parent)
for _, child in pairs(children) do
    print("child: " .. child)
end
```

### get_parent

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The child entity |

Returns:

| Return | Description |
| ---  | --- |
| `Option<Entity>` | The parent entity if it exists, otherwise `None` |

```lua
local parent = world.get_parent(child)
if parent ~= nil then
    print("parent: " .. parent)
end
```

### despawn

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to despawn |

```lua
world.despawn(entity)
```

### despawn_descendants

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to despawn descendants of |

```lua
world.despawn_descendants(entity)
```

### despawn_recursive

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to despawn recursively |

```lua
world.despawn_recursive(entity)
```

### has_entity

Arguments:

| Argument | Type | Description |
| --- | --- | --- |
| `entity` | `Entity` | The entity to check |

Returns:

| Return | Description |
| ---  | --- |
| `bool` | `true` if the entity exists, otherwise `false` |

```lua
local exists = world.has_entity(entity)
if exists then
    print("entity exists")
end
```

### query

Returns:

| Return | Description |
| ---  | --- |
| `ScriptQueryBuilder` | The query builder |

```lua
local queryBuilder = world.query()
```

### exit
Send the exit signal to the application, will gracefully shutdown the application.

```lua
world.exit()
```
